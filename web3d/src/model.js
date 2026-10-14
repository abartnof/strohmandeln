// The table as the human may know it, and how an event changes it.
//
// The page draws a model, never the protocol's state: `fromState(state)` makes
// it for a state at rest (by replaying the deal's events, which the state
// carries, docs/PROTOCOL.md), and `reduce(model, event)` moves it one event on,
// so the choreography can lay out every stage between two states. `check`
// compares a model with the state it should equal (tests, and a guard in the
// page).
//
// Seats are 0 and 1. A model holds only cards: no scores, no text.
//
//   hand      the human's hand cards, sorted: [{ code, public }]
//   opp       the opponent's: [{ code | null, public }]; a code is known when
//             pulled or part of an announced honour; its card is still a back
//   packets   packets[seat][i] = { cover: code | null, hidden: n }
//   held      the card turned and held up: { seat, packet, code } | null
//   trick     the open trick: [{ seat, code, source }]
//   won       the cards of each seat's won pile, in the order they went in
//   counted   the cards turned up in the count, in order (top of the pile first)
//   pack      cards in the pack (54 before a deal and after a sitting)

import { handSortKey, sortCodes } from "./cardinfo.js";

const PACKET = () => ({ cover: null, hidden: 0 });

export function emptyModel(human = 0) {
  return {
    human,
    dealer: null,
    forehand: null,
    multiplier: 1,
    declarer: null,
    pack: 54,
    hand: [],
    opp: [],
    packets: [[PACKET(), PACKET(), PACKET()], [PACKET(), PACKET(), PACKET()]],
    held: null,
    trick: [],
    won: [[], []],
    counted: [[], []],
    revealing: null,
    announced: [],
    void: null,
    over: false,
  };
}

export function cloneModel(m) {
  return {
    ...m,
    hand: m.hand.map((c) => ({ ...c })),
    opp: m.opp.map((c) => ({ ...c })),
    packets: m.packets.map((row) => row.map((p) => ({ ...p }))),
    held: m.held && { ...m.held },
    trick: m.trick.map((c) => ({ ...c })),
    won: m.won.map((w) => [...w]),
    counted: m.counted.map((w) => [...w]),
    announced: m.announced.map((a) => ({ ...a })),
  };
}

// The opponent's hand: the cards the human knows (sorted) after the ones it
// does not; it is drawn in backs either way.
const sortOpp = (opp) => [...opp.filter((c) => !c.code), ...opp.filter((c) => c.code).sort((a, b) => handSortKey(a.code) - handSortKey(b.code))];

// Which of the opponent's unknown cards leaves when it plays a card the human
// did not know it held: the middle of them, as a card is drawn from a fan.
export const unknownLeaving = (opp) => {
  const unknown = opp.map((c, i) => (c.code ? -1 : i)).filter((i) => i >= 0);
  return unknown[Math.floor(unknown.length / 2)];
};

function addToHand(m, seat, code, isPublic = true) {
  if (seat === m.human) {
    m.hand = sortCodes([...m.hand.map((c) => c.code), code]).map((c) => m.hand.find((h) => h.code === c) ?? { code: c, public: isPublic });
  } else {
    m.opp = sortOpp([...m.opp, { code, public: isPublic }]);
  }
}

function removeFromHand(m, seat, code) {
  if (seat === m.human) {
    m.hand = m.hand.filter((c) => c.code !== code);
    return;
  }
  const known = m.opp.findIndex((c) => c.code === code);
  const at = known >= 0 ? known : unknownLeaving(m.opp);
  if (at !== undefined) m.opp.splice(at, 1);
}

export function reduce(model, e) {
  const m = cloneModel(model);
  switch (e.type) {
    case "sitting_start":
      m.dealer = e.first_dealer;
      break;
    case "deal_start":
      return { ...emptyModel(m.human), dealer: e.dealer, forehand: e.forehand, multiplier: e.multiplier };
    case "dealt":
      if (e.seat === m.human) m.hand = sortCodes(e.cards).map((code) => ({ code, public: false }));
      m.opp = Array.from({ length: 15 }, () => ({ code: null, public: false }));
      m.packets = [0, 1].map(() => [0, 1, 2].map(() => ({ cover: null, hidden: 4 })));
      m.pack = 0;
      break;
    case "void_deal":
      m.void = [...e.seats];
      break;
    case "bid":
      m.declarer = e.seat;
      break;
    case "thrown_in":
      return { ...emptyModel(m.human), dealer: m.dealer, forehand: m.forehand, multiplier: m.multiplier, thrown: true };
    case "reveal_start":
      m.revealing = e.seat;
      break;
    case "packet_turned":
      m.held = { seat: e.seat, packet: e.packet, code: e.card };
      break;
    case "card_pulled": {
      m.held = null;
      m.packets[e.seat][e.packet - 1].hidden -= 1;
      addToHand(m, e.seat, e.card, true);
      break;
    }
    case "cover_set": {
      m.held = null;
      const p = m.packets[e.seat][e.packet - 1];
      p.hidden -= 1;
      p.cover = e.card;
      break;
    }
    case "announcement":
      m.announced.push({ seat: e.seat, honour: e.honour });
      for (const code of e.cards ?? []) {
        if (e.seat === m.human) {
          const c = m.hand.find((h) => h.code === code);
          if (c) c.public = true;
        } else if (!m.opp.some((c) => c.code === code)) {
          const at = unknownLeaving(m.opp);
          if (at !== undefined) m.opp[at] = { code, public: true };
          m.opp = sortOpp(m.opp);
        }
      }
      break;
    case "card_played": {
      const packet = /^packet (\d)$/.exec(e.source);
      if (packet) m.packets[e.seat][Number(packet[1]) - 1].cover = null;
      else removeFromHand(m, e.seat, e.card);
      m.trick.push({ seat: e.seat, code: e.card, source: e.source });
      break;
    }
    case "trick_won":
      m.won[e.winner].push(...m.trick.map((c) => c.code));
      m.trick = [];
      break;
    case "count":
      for (const s of [0, 1]) {
        m.counted[s] = [...m.won[s]].reverse();
        m.won[s] = [];
      }
      break;
    case "sitting_end":
      return { ...emptyModel(m.human), dealer: m.dealer, over: true };
    default:
      break; // the rest move no cards
  }
  return m;
}

// The model for a state at rest: the deal in progress replayed from its events.
export function fromState(state) {
  let m = emptyModel(state.human);
  if (state.over && !state.deal) return { ...m, over: true };
  for (const e of state.events ?? []) m = reduce(m, e);
  return m;
}

// Where a model and the state it should equal disagree, as sentences (none: equal).
export function check(model, state) {
  const bad = [];
  const d = state.deal;
  if (!d) return model.hand.length === 0 ? bad : ["a model with a hand for a state with no deal"];
  const same = (a, b) => JSON.stringify(a) === JSON.stringify(b);
  if (!same(model.hand.map((c) => c.code), sortCodes(d.own_hand.map((c) => c.card)))) bad.push(`hand ${model.hand.map((c) => c.code)} vs ${d.own_hand.map((c) => c.card)}`);
  if (model.opp.length !== d.opponent_hand_size) bad.push(`opponent hand ${model.opp.length} vs ${d.opponent_hand_size}`);
  if (!same(sortCodes(model.opp.filter((c) => c.code).map((c) => c.code)), sortCodes(d.opponent_known))) bad.push(`known ${model.opp.filter((c) => c.code).map((c) => c.code)} vs ${d.opponent_known}`);
  for (const seat of [0, 1]) {
    for (let i = 0; i < 3; i++) {
      const p = model.packets[seat][i];
      if (p.cover !== d.covers[seat][i]) bad.push(`cover ${seat}/${i} ${p.cover} vs ${d.covers[seat][i]}`);
      if (p.hidden !== d.hidden[seat][i]) bad.push(`hidden ${seat}/${i} ${p.hidden} vs ${d.hidden[seat][i]}`);
    }
  }
  const counted = model.counted[0].length + model.counted[1].length;
  const wonNow = model.won[0].length + model.won[1].length + counted;
  if (wonNow !== 2 * d.tricks.length) bad.push(`won ${wonNow} vs ${2 * d.tricks.length} cards`);
  if ((d.lead ? 1 : 0) !== model.trick.length && !(d.lead === null && model.trick.length === 0)) bad.push(`trick ${model.trick.length} vs lead ${d.lead?.card}`);
  if (d.lead && model.trick[0]?.code !== d.lead.card) bad.push(`lead ${model.trick[0]?.code} vs ${d.lead.card}`);
  return bad;
}
