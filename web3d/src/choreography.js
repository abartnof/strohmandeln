// The choreography: how the cards get from one state to the next
// (docs/TABLE3D.md 2).
//
// A "state" here is the page's own: { model, events } -- the model of the
// table (model.js) and every event of the sitting so far, in order. The events
// since the last state are replayed into intermediate models, each laid out by
// `layout`, so every stage runs between two true layouts; a final settle lands
// everything on layout(next.model) whatever happened. A new sitting or a
// restore is one direct transition.
//
// choreograph(prev, next, placement, view, options) -> { motions, placement, duration, beats, count, marks }
//
//   placement  where each of the 54 meshes is: [{ id, key, zone, index, code, pose }]
//   motions    [{ id, path, delay, duration, reveal }]: `reveal` is the face a
//              mesh shows from the start of its motion, or { code: null,
//              atEnd: true }, the face it stops showing when it lands
//   beats      { eventIndex: ms }: when each new event is seen to happen
//   view       { chosen, scroll, left, zones }: how the person sees the state
//   options    { waits, pace, reduced, game }
//   count      { at, lines: [ms], end, turns } the count's moments
//
// Structure and the plan/match machinery are cassino's choreography.js
// (@ 73f0cd7); the stages are this game's.

import { Vector3 } from "three";
import { countSchedule } from "../../kit/js/count.js";
import { bob, carryBlock, lying, peelOff, pull, rise, slide, still, toss, transfer } from "../../kit/js/kinematics.js";
import { CARD } from "../../kit/js/units.js";
import { COUNT_RULES, countLines } from "./counting.js";
import { jitter, layout } from "./layout.js";
import { emptyModel, reduce, unknownLeaving } from "./model.js";
import { ZONES } from "./zones.js";

// Starting points at normal speed, tuned by eye; the speed setting scales them all.
export const TIMING = Object.freeze({
  think: 380, // the opponent's pause before a move that follows another
  play: 560, // the tug out of the hand, then the toss
  resort: 260, // a hand closing up
  turn: 520, // a packet's top card lifted, turned and raised
  hold: 800, // a turned card held up to be read
  holdLong: 1400, // one the opponent speaks to (a Sküs, Mond, Pagat or king)
  pull: 460, // the held card flown to its hand
  chain: 200, // the next card's lift begins as the last one arrives
  cover: 420, // a turned low card lowered as its packet's cover
  rest: 1000, // a trick seen before it is gathered
  ultimo: 1400, // the Pagat held up where it lies
  carry: 620, // the gathered trick turned over into its winner's pile
  gatherStep: 220,
  beat: 260, // a chip that moves no card
  say: 700, // a call said before the next thing
  cutOut: 420,
  cutShow: 1600,
  cutBack: 360,
  dealCard: 320,
  dealStep: 70, // between cards of a hand
  dealGroup: 160, // between hands and packets
  countCard: 520,
  countStagger: 110,
  countLead: 2600, // `count-start` is said and its box gone before the first card turns
  countTail: 1000, // the last popup done before the count is closed
  collect: 480,
  collectStagger: 12,
  void: 1500,
  banner: 3000, // the result held for a tap
  end: 900,
  direct: 420,
});

const REST = 0.02;
const STEP = CARD.thickness + 0.02;
const TOWARD = { you: new Vector3(0, 0, 1), them: new Vector3(0, 0, -1) };
const PLAYED = { clearance: 3 };

// Cards the opponent speaks to when they are turned up and pulled (5.1).
const SPOKEN = new Set(["SK", "T21", "T1", "Kh", "Kd", "Kc", "Ks"]);

const ride = (to) => 0.3 + 4 * Math.max(0, to.position.y);
const onTable = (pose) => {
  const n = new Vector3(0, 0, 1).applyQuaternion(pose.quaternion);
  return Math.abs(n.y) > 0.9 && pose.position.y < 3;
};
const samePlace = (a, b) => Math.abs(a.position.x - b.position.x) < 1e-9 && Math.abs(a.position.z - b.position.z) < 1e-9;
const samePose = (a, b) => a.position.equals(b.position) && a.quaternion.equals(b.quaternion);

const layoutOf = (model, view = {}) => layout(model, { zones: view.zones ?? ZONES, chosen: view.chosen ?? null, scroll: view.scroll ?? 0, left: view.left ?? false });

// The meshes placed straight onto a layout, the first time a state is seen.
export function initialPlacement(state, view = {}) {
  return layoutOf(state.model, view).map((slot, id) => ({ id, ...slot }));
}

// Before the first deal of a sitting: the whole pack squared at the first
// dealer's right. Choreographed from this to a new sitting's first state, the
// opening is played out.
export function opening(state) {
  const first = state.events.find((e) => e.type === "sitting_start");
  return { model: { ...emptyModel(state.model.human), dealer: first?.first_dealer ?? null }, events: [] };
}

// ---- the reducer: events into stages ---------------------------------------

const KIND = {
  sitting_start: "cut",
  deal_start: "collect",
  dealt: "deal",
  void_deal: "void",
  thrown_in: "collect",
  packet_turned: "turn",
  card_pulled: "pull",
  cover_set: "cover",
  card_played: "play",
  trick_won: "gather",
  count: "count",
  game_scored: "banner",
  sitting_end: "finish",
};
// How long a stage that moves no card takes.
const BEAT = { bid: TIMING.say, pass: TIMING.say, both_pass: TIMING.say, reveal_start: TIMING.beat, packet_empty: TIMING.beat, announcement: 400, honour_forfeited: TIMING.beat, pagat_ultimo: 0, valat: 1200, doubling_set: TIMING.beat, doubling_applied: TIMING.beat, doubling_lapsed: TIMING.beat, tour_end: 600, extension_offered: TIMING.say, extension_decided: TIMING.say };

export function stagesBetween(prev, next) {
  if (!prev || prev.model.human !== next.model.human || next.events.length < prev.events.length) return null;
  let m = prev.model;
  const stages = [];
  const fresh = next.events.slice(prev.events.length);
  fresh.forEach((e, n) => {
    const at = prev.events.length + n;
    const after = reduce(m, e);
    stages.push({ kind: KIND[e.type] ?? "beat", event: e, before: m, model: after, at, ahead: fresh.slice(n + 1), fresh });
    m = after;
  });
  return stages;
}

// ---- matching meshes to slots ------------------------------------------------

const SOURCES = {
  trick: ["their-hand", "your-hand", "your-packets", "their-packets", "your-won", "their-won"],
  "your-won": ["your-won", "trick", "count", "your-hand"],
  "their-won": ["their-won", "trick", "count", "their-hand"],
  count: ["your-won", "their-won"],
  "your-hand": ["your-hand", "pack", "your-packets", "their-packets", "trick"],
  "their-hand": ["their-hand", "pack", "your-packets", "their-packets", "trick"],
  "your-packets": ["your-packets", "pack", "your-hand", "trick"],
  "their-packets": ["their-packets", "pack", "their-hand", "trick"],
  pack: ["pack", "your-won", "their-won", "count", "trick", "your-packets", "their-packets", "your-hand", "their-hand"],
};
const CLAIM = ["trick", "count", "your-won", "their-won", "your-hand", "their-hand", "your-packets", "their-packets", "pack"];
const KEYED = new Set(["pack", "your-packets", "their-packets", "your-won", "their-won"]);

function takeOrder(zone, meshes, destination) {
  const byIndex = [...meshes].sort((a, b) => a.index - b.index);
  if (zone === destination) return byIndex;
  if (zone === "pack" || zone.endsWith("won")) return byIndex.reverse(); // off the top
  if (zone === "their-hand") {
    const n = byIndex.length;
    const k = n ? Math.floor(((jitter(`${destination}${n}`, 1) + 1) / 2) * n) % n : 0;
    return [...byIndex.slice(k), ...byIndex.slice(0, k)];
  }
  return byIndex;
}

export function match(current, target) {
  const byCode = new Map(current.filter((m) => m.code).map((m) => [m.code, m]));
  const taken = new Set();
  const owner = new Array(target.length).fill(null);
  const own = (k, mesh) => {
    owner[k] = mesh.id;
    taken.add(mesh.id);
  };
  target.forEach((slot, k) => {
    const mesh = slot.code && byCode.get(slot.code);
    if (mesh) own(k, mesh);
  });
  const byKey = new Map(current.filter((m) => KEYED.has(m.zone) && !taken.has(m.id)).map((m) => [`${m.zone}|${m.key}`, m]));
  target.forEach((slot, k) => {
    if (owner[k] !== null || !KEYED.has(slot.zone)) return;
    const mesh = byKey.get(`${slot.zone}|${slot.key}`);
    if (mesh && !taken.has(mesh.id)) own(k, mesh);
  });
  for (const zone of CLAIM) {
    const open = target.map((slot, k) => k).filter((k) => target[k].zone === zone && owner[k] === null);
    for (const source of SOURCES[zone]) {
      if (!open.length) break;
      const free = takeOrder(source, current.filter((m) => m.zone === source && !taken.has(m.id)), zone);
      while (open.length && free.length) own(open.shift(), free.shift());
    }
  }
  const spare = current.filter((m) => !taken.has(m.id));
  owner.forEach((id, k) => {
    if (id === null) own(k, spare.shift());
  });
  return owner;
}

// ---- the plan: stages into motions -------------------------------------------

class Plan {
  constructor(placement, view, options) {
    this.moments = {};
    this.pace = options.pace ?? null;
    this.reduced = Boolean(options.reduced ?? view.reduced);
    this.now = placement.map((m) => ({ ...m }));
    this.view = view;
    this.clock = 0;
    this.end = 0;
    this.motions = [];
    this.free = new Map();
    this.marks = [];
    this.counted = null;
  }

  layout(model) {
    return layoutOf(model, { ...this.view, chosen: null });
  }

  add(id, path, start, duration, reveal) {
    const begin = Math.max(start, this.free.get(id) ?? 0);
    const fixed = this.reduced ? 1 : duration;
    this.motions.push({ id, path, delay: begin, duration: fixed, reveal });
    this.free.set(id, begin + fixed);
    this.end = Math.max(this.end, begin + fixed);
    return begin + duration;
  }

  move(id, slot, path, start, duration) {
    const was = this.now[id];
    const reveal = was.code === slot.code ? undefined : slot.code ? { code: slot.code } : { code: null, atEnd: true };
    const end = this.add(id, this.reduced ? still(slot.pose) : path, start, duration, reveal);
    this.now[id] = { id, key: slot.key ?? was.key, zone: slot.zone ?? was.zone, index: slot.index ?? was.index, code: slot.code, pose: slot.pose };
    return end;
  }

  via(id, pose) {
    const m = this.now[id];
    return { key: m.key, zone: m.zone, index: m.index, code: m.code, pose };
  }

  stage(target, choose, start = this.clock) {
    const owner = match(this.now, target);
    let end = start;
    owner.forEach((id, k) => {
      const mesh = this.now[id];
      const slot = target[k];
      if (samePose(mesh.pose, slot.pose) && mesh.code === slot.code) {
        this.now[id] = { id, ...slot };
        return;
      }
      const m = choose?.(mesh, slot) ?? this.plain(mesh, slot);
      end = Math.max(end, this.move(id, slot, m.path, start + m.delay, m.duration));
    });
    this.clock = Math.max(this.clock, end);
    return end;
  }

  plain(mesh, slot) {
    const from = mesh.pose;
    const to = slot.pose;
    if (mesh.zone === slot.zone && !onTable(to)) return { path: transfer(from, to, { clearance: 1.2 }), delay: 0, duration: TIMING.resort };
    if (onTable(from) && onTable(to)) {
      if (samePlace(from, to)) return { path: slide(from, to), delay: 0, duration: TIMING.resort };
      return { path: slide(from, to, { lift: ride(to) }), delay: 0, duration: TIMING.resort * 1.4 };
    }
    if (onTable(to)) return { path: toss(from, to), delay: 0, duration: TIMING.direct };
    return { path: transfer(from, to, { clearance: 1.5 }), delay: 0, duration: TIMING.direct };
  }

  think(seat, model) {
    if (seat !== model.human && this.motions.length) this.clock = Math.max(this.clock, this.end) + TIMING.think;
  }

  // The mesh of a hand card: the human's by its code; the opponent's, the
  // fan's card at `index`.
  handMesh(model, seat, code, beforeModel) {
    if (seat === model.human) return this.now.find((m) => m.zone === "your-hand" && m.code === code);
    const opp = beforeModel.opp;
    const known = opp.findIndex((c) => c.code === code);
    const at = known >= 0 ? known : unknownLeaving(opp);
    const fan = this.now.filter((m) => m.zone === "their-hand").sort((a, b) => a.index - b.index);
    return fan[at] ?? fan[0];
  }

  // ---- the stages ---------------------------------------------------------

  beat({ event }) {
    const ms = BEAT[event.type] ?? 0;
    this.clock = Math.max(this.clock, this.end) + 0;
    this.moments.start = this.clock;
    this.clock += ms;
  }

  direct(model) {
    this.stage(layoutOf(model, this.view), null, Math.max(this.clock, this.end));
  }

  // Every card back to the pack at the new dealer's right: the cards lying
  // highest leave first, each landing on those before it; a card face up
  // turns over on its way.
  collect({ model }) {
    const target = this.layout(model);
    const pack = target.filter((s) => s.zone === "pack");
    const kept = new Set(this.now.filter((m) => m.zone === "pack").map((m) => m.key));
    const open = pack.filter((s) => !kept.has(s.key));
    const leaving = this.now.filter((m) => m.zone !== "pack").sort((a, b) => b.pose.position.y - a.pose.position.y || a.id - b.id);
    const start = Math.max(this.clock, this.end);
    leaving.forEach((mesh, k) => {
      const slot = open[k];
      if (!slot) return;
      const faceUp = new Vector3(0, 0, 1).applyQuaternion(mesh.pose.quaternion).y > 0.5;
      const toward = slot.pose.position.clone().sub(mesh.pose.position).setY(0);
      const path =
        onTable(mesh.pose) && !faceUp
          ? slide(mesh.pose, slot.pose, { lift: ride(slot.pose) })
          : onTable(mesh.pose)
            ? peelOff(mesh.pose, (lifted) => toss(lifted, slot.pose), { toward })
            : toss(mesh.pose, slot.pose);
      this.move(mesh.id, slot, path, start + k * TIMING.collectStagger, TIMING.collect);
    });
    this.moments.start = start;
    this.clock = Math.max(this.clock, this.end);
  }

  // The cut for the deal: each player's card peeled off the pack and laid
  // face up beside it, yours nearer you, both seen a moment, then back on the
  // pack face down. The faces are two cards of the human's own hand: the
  // pack is shuffled after a cut, so they say nothing of the deal.
  cut({ model, ahead, fresh }) {
    const dealt = (ahead.length ? ahead : fresh).find((e) => e.type === "dealt");
    const codes = dealt ? [dealt.cards[13], dealt.cards[14]] : null;
    const pack = this.now.filter((m) => m.zone === "pack").sort((a, b) => b.index - a.index);
    if (!codes || pack.length < 2) return;
    const start = Math.max(this.clock, this.end);
    const base = pack[0].pose.position;
    const inward = base.x > 0 ? -1 : 1;
    const shownAt = (side) => lying({ x: base.x + inward * (CARD.width + 2.5), z: base.z + side * (CARD.height / 2 + 0.8), height: REST });
    let end = start;
    [[pack[0], "you", 1, codes[0]], [pack[1], "them", -1, codes[1]]].forEach(([mesh, who, side, code]) => {
      const home = { ...mesh };
      const pose = shownAt(side);
      const out = peelOff(mesh.pose, (lifted) => toss(lifted, pose, { clearance: 3 }), { toward: TOWARD[who] });
      const shown = this.move(mesh.id, { ...this.via(mesh.id, pose), code }, out, start + (who === "them" ? 90 : 0), TIMING.cutOut);
      this.moments.shown = Math.max(this.moments.shown ?? 0, shown);
      const back = this.move(mesh.id, home, peelOff(pose, (lifted) => toss(lifted, home.pose, { clearance: 3 }), { toward: TOWARD[who] }), shown + TIMING.cutShow, TIMING.cutBack);
      end = Math.max(end, back);
    });
    this.moments.start = start;
    this.clock = end;
    void model;
  }

  // 3 x 5 hand cards, forehand first, then six packets of four, each card
  // straight from the top of the pack to where it will rest.
  deal({ model, event }) {
    const target = this.layout(model);
    const top = this.now.filter((m) => m.zone === "pack").sort((a, b) => b.index - a.index);
    const hands = { you: target.filter((s) => s.zone === "your-hand"), them: target.filter((s) => s.zone === "their-hand") };
    const dealer = model.dealer;
    const fore = model.forehand;
    const order = [];
    for (let r = 0; r < 3; r++) for (const seat of [fore, dealer]) order.push({ hand: seat });
    // The human's cards in the order they were dealt, each to its sorted place.
    const mineByDeal = event.seat === model.human ? [...event.cards] : [];
    const ownCards = [mineByDeal.splice(0, 5), mineByDeal.splice(0, 5), mineByDeal.splice(0, 5)];
    // forehand's three rounds and the dealer's three, interleaved: if the human
    // is forehand their rounds are 0, 2, 4 of `order`; else 1, 3, 5.
    const rounds = order.map((o, n) => ({ ...o, n }));
    const humanRounds = rounds.filter((o) => o.hand === model.human);
    humanRounds.forEach((o, k) => (o.cards = ownCards[k]));
    const slotCode = new Map(hands.you.map((s) => [s.code, s]));
    const start = Math.max(this.clock, this.end);
    let t = 0;
    const toHand = (mesh, slot, at) => this.move(mesh.id, slot, rise(mesh.pose, slot.pose, { clearance: 4 }), start + at, TIMING.dealCard);
    let theirNext = 0;
    for (const round of rounds) {
      for (let c = 0; c < 5; c++) {
        const mesh = top.shift();
        if (!mesh) continue;
        const slot = round.hand === model.human ? slotCode.get(round.cards?.[c]) : hands.them[theirNext++];
        if (slot) toHand(mesh, slot, t);
        t += TIMING.dealStep;
      }
      t += TIMING.dealGroup;
    }
    // Six packets of four: forehand's first, the dealer's first, forehand's second ...
    for (let p = 0; p < 6; p++) {
      const seat = p % 2 === 0 ? fore : dealer;
      const i = Math.floor(p / 2);
      for (let k = 0; k < 4; k++) {
        const mesh = top.shift();
        const slot = target.find((s) => s.packet?.seat === seat && s.packet.i === i && s.index === k);
        if (mesh && slot) this.move(mesh.id, slot, toss(mesh.pose, slot.pose, { clearance: 3 }), start + t, TIMING.dealCard);
        t += TIMING.dealStep;
      }
      t += TIMING.dealGroup;
    }
    this.moments.start = start;
    this.clock = Math.max(this.clock, start + t);
    this.stage(target, null, Math.max(this.clock, this.end));
  }

  // A hand without a tarock: the cards held up a moment, the tag over them.
  void({ model, event }) {
    const start = Math.max(this.clock, this.end);
    for (const seat of event.seats) {
      const zone = seat === model.human ? "your-hand" : "their-hand";
      for (const m of this.now.filter((x) => x.zone === zone)) this.add(m.id, bob(m.pose, 3.5), start, TIMING.void);
    }
    this.moments.start = start;
    this.clock = start + TIMING.void;
  }

  // The packet's top card lifted off its stack, turned over and held up facing
  // the human long enough to read.
  turn({ before, model, event }) {
    const i = event.packet - 1;
    const h = before.packets[event.seat][i].hidden;
    const key = `pk:${event.seat}:${i}:${h - 1}`;
    const mesh = this.now.find((m) => m.key === key && m.zone.endsWith("packets"));
    const target = this.layout(model);
    const slot = target.find((s) => s.held);
    const start = Math.max(this.clock, this.end);
    if (!mesh || !slot) return;
    const path = peelOff(mesh.pose, (lifted) => transfer(lifted, slot.pose, { clearance: 2 }), { toward: TOWARD.you });
    const end = this.move(mesh.id, { ...slot, index: 99 }, path, start, TIMING.turn);
    this.moments.start = start;
    this.moments.raised = start + TIMING.turn * 0.55;
    this.clock = end + (SPOKEN.has(event.card) ? TIMING.holdLong : TIMING.hold);
  }

  // The held card flown to its owner's hand, the hand making room; the next
  // card's lift begins as this one arrives.
  pull({ model, event }) {
    const target = this.layout(model);
    const mesh = this.now.find((m) => m.key === "held");
    const start = Math.max(this.clock, this.end - 0);
    if (!mesh) return this.stage(target, null, start);
    const mine = event.seat === model.human;
    const slot = mine ? target.find((s) => s.zone === "your-hand" && s.code === event.card) : target.find((s) => s.zone === "their-hand" && s.known === event.card);
    const end = this.move(mesh.id, slot, pull(mesh.pose, slot.pose, { tug: 2.2, clearance: 2.2 }), start, TIMING.pull);
    this.moments.start = start;
    this.stage(target, null, start);
    this.clock = Math.max(start, end - TIMING.chain);
  }

  // The held low card lowered onto its packet as the cover.
  cover({ model, event }) {
    const target = this.layout(model);
    const mesh = this.now.find((m) => m.key === "held");
    const start = Math.max(this.clock, this.end);
    if (!mesh) return this.stage(target, null, start);
    const slot = target.find((s) => s.cover && s.code === event.card);
    const end = this.move(mesh.id, slot, transfer(mesh.pose, slot.pose, { clearance: 1.6 }), start, TIMING.cover);
    this.moments.start = start;
    this.clock = Math.max(start, end - TIMING.chain);
  }

  // A card played: out of the hand (or off its stack) and tossed into the
  // trick; the hand closes up behind it.
  play({ before, model, event }) {
    this.think(event.seat, model);
    const target = this.layout(model);
    const slot = target.find((s) => s.zone === "trick" && s.code === event.card);
    const fromPacket = event.source !== "hand";
    const mesh = fromPacket ? this.now.find((m) => m.code === event.card && m.zone.endsWith("packets")) : this.handMesh(model, event.seat, event.card, before);
    const start = Math.max(this.clock, this.end);
    if (!mesh || !slot) return this.stage(target, null, start);
    const path = fromPacket ? peelOff(mesh.pose, (lifted) => toss(lifted, slot.pose, PLAYED), { toward: new Vector3(0, 0, event.seat === model.human ? -1 : 1) }) : pull(mesh.pose, slot.pose, PLAYED);
    const end = this.move(mesh.id, { ...slot, index: slot.index }, path, start, TIMING.play);
    this.moments.start = start;
    this.stage(target, null, start);
    this.clock = Math.max(this.clock, end);
  }

  // The gather: the trick rests a moment, is squared into one heap, and
  // carried face down onto its winner's pile.
  gather({ model, event, ahead }) {
    const target = this.layout(model);
    const ultimo = ahead[0]?.type === "pagat_ultimo";
    const cards = event.cards.map((c) => this.now.find((m) => m.code === c)).filter(Boolean);
    const start = Math.max(this.clock, this.end) + TIMING.rest + (ultimo ? TIMING.ultimo : 0);
    this.moments.start = start - TIMING.rest;
    this.moments.ultimo = start - TIMING.rest - (ultimo ? TIMING.ultimo : 0) + 200;
    if (!cards.length) return this.stage(target, null, start);
    const base = cards[0].pose.position;
    const heapPose = (k) => lying({ x: base.x, z: base.z, height: REST + k * STEP });
    let t = start;
    cards.forEach((mesh, k) => {
      const pose = heapPose(k);
      if (k > 0) t = Math.max(t, this.move(mesh.id, this.via(mesh.id, pose), slide(mesh.pose, pose, { lift: ride(pose) }), start, TIMING.gatherStep));
    });
    const zone = model.won[event.winner].length ? (event.winner === model.human ? "your-won" : "their-won") : null;
    const pile = target.filter((s) => s.zone === zone);
    const slots = pile.slice(pile.length - cards.length).reverse();
    const paths = carryBlock(cards.map((m, k) => (k === 0 ? m.pose : heapPose(k))), slots.map((s) => s.pose), { peel: TOWARD[event.winner === model.human ? "you" : "them"] });
    cards.forEach((mesh, k) => this.move(mesh.id, slots[k], paths[k], t, TIMING.carry));
    this.moments.landed = t + TIMING.carry;
    this.clock = t + TIMING.carry;
    this.stage(target, null, this.clock);
  }

  // The count: each take turned up in threes out of its pile into its row,
  // as the kit's popups tell it.
  count({ model, before, event, at }) {
    const target = this.layout(model);
    const lines = countLines(model.counted, model.human);
    const start = Math.max(this.clock, this.end) + TIMING.beat + TIMING.countLead;
    const sched = countSchedule(COUNT_RULES, lines, { start, pace: this.pace });
    for (const turn of sched.turns) {
      turn.cards.forEach((code, w) => {
        const slot = target.find((s) => s.zone === "count" && s.code === code);
        const seat = turn.side === "you" ? model.human : 1 - model.human;
        const i = model.counted[seat].indexOf(code);
        const key = `won:${seat}:${before.won[seat].length - 1 - i}`;
        const mesh = this.now.find((m) => m.key === key && m.zone.endsWith("won"));
        if (!mesh || !slot) return;
        const toward = turn.side === "you" ? TOWARD.you : TOWARD.them;
        this.move(mesh.id, slot, peelOff(mesh.pose, (lifted) => toss(lifted, slot.pose, { clearance: 2 }), { toward }), turn.at + w * TIMING.countStagger, TIMING.countCard);
      });
    }
    const end = sched.end + TIMING.countTail;
    this.counted = { at, lines: sched.lines, end, turns: sched.turns, grouped: lines };
    this.moments.start = start - TIMING.beat - TIMING.countLead;
    this.clock = end;
    void event;
    this.stage(target, null, this.clock);
  }

  // The result is held for a tap.
  banner() {
    this.moments.start = Math.max(this.clock, this.end) + TIMING.beat;
    this.clock = this.moments.start + TIMING.banner;
  }

  // The sitting's end: every card back to the pack.
  finish({ model }) {
    this.collect({ model });
    this.clock += TIMING.end;
  }

  result() {
    return { motions: this.motions, placement: this.now, duration: Math.max(this.clock, this.end), count: this.counted ?? null, marks: this.marks };
  }
}

// Stages heard as they begin (a call spoken as it is made); the rest when
// they have landed, or at a moment of their own.
const HEARD_AT_START = new Set(["collect", "count", "deal", "cut", "void", "pull", "cover", "beat", "banner", "turn"]);
const HEARD_WITHIN = { packet_turned: "raised", pagat_ultimo: "ultimo", trick_won: "start" };

export function choreograph(prev, next, placement, view = {}, options = {}) {
  const plan = new Plan(placement, view, options);
  const stages = stagesBetween(prev, next);
  const waits = Object.entries(options.waits ?? {}).map(([at, ms]) => [Number(at), ms]);
  const waitFor = (k) => Math.max(0, ...waits.filter(([a]) => a <= k).map(([, ms]) => ms));
  const marks = [];
  if (stages) {
    for (const stage of stages) {
      plan.clock = Math.max(plan.clock, waitFor(stage.at));
      const start = Math.max(plan.clock, plan.end);
      plan.moments = {};
      if (typeof plan[stage.kind] !== "function") throw new Error(`no stage ${stage.kind}`);
      plan[stage.kind](stage);
      marks.push({ at: stage.at, type: stage.event.type, start: plan.moments.start ?? start, end: Math.max(plan.clock, plan.end), moments: plan.moments, early: HEARD_AT_START.has(stage.kind) });
    }
  }
  plan.direct(next.model); // settle: exactly layout(next), whatever came before
  const result = plan.result();
  const beats = {};
  for (const mark of marks) {
    const within = HEARD_WITHIN[mark.type] && mark.moments[HEARD_WITHIN[mark.type]];
    beats[mark.at] = within ?? (mark.early ? mark.start : mark.end);
  }
  return { ...result, beats };
}
