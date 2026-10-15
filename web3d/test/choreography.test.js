import { test } from "node:test";
import assert from "node:assert/strict";
import { cardCorners } from "../../kit/js/kinematics.js";
import { choreograph, initialPlacement, LAST_TRICK, opening, SLOW_LAST, stagesBetween, TIMING } from "../src/choreography.js";
import { layout } from "../src/layout.js";
import { emptyModel, fromState, reduce } from "../src/model.js";
import { ZONES, ZONES_PORTRAIT } from "../src/zones.js";
import { passer, playSitting, taker } from "./sim.js";

// A sitting as the page sees it: after every reply the model, the events so
// far, and what the engine says is hidden from the human at that moment.
async function states(config, policy, extend = false) {
  const hiddenAt = [];
  const { replies } = await playSitting(config, { policy, extend, onReply: (_, engine) => hiddenAt.push(new Set(engine.audit().hidden)) });
  let model = emptyModel(config.human);
  const log = [];
  return replies.map((r, i) => {
    for (const e of r.events) {
      model = reduce(model, e);
      log.push(e);
    }
    return { state: { model, events: [...log] }, fresh: r.events, hidden: hiddenAt[i], wasm: r.state };
  });
}

const view = { zones: ZONES };

// What the director would do with a result: every mesh's pose at time t.
function poseAt(result, placement, id, t) {
  const here = placement[id].pose;
  let pose = here;
  for (const m of result.motions.filter((x) => x.id === id).sort((a, b) => a.delay - b.delay)) {
    if (t < m.delay) break;
    pose = m.path(Math.min(1, (t - m.delay) / m.duration));
  }
  return pose;
}

async function walk(config, policy, extend = false, zones = ZONES) {
  const seq = await states(config, policy, extend);
  const first = seq[0].state;
  let prev = opening(first);
  let placement = initialPlacement(prev, { zones });
  const seen = new Set();
  for (const { state, fresh, hidden } of seq) {
    const result = choreograph(prev, state, placement, { zones });
    for (const e of fresh) seen.add(e.type);
    // It ends exactly on the layout of the next state.
    const want = layout(state.model, { zones });
    assert.equal(result.placement.length, 54);
    const got = new Map(result.placement.map((m) => [m.zone + "|" + m.key + "|" + m.code, m]));
    for (const s of want) {
      const m = got.get(s.zone + "|" + s.key + "|" + s.code);
      assert.ok(m, `no mesh at ${s.zone} ${s.key} ${s.code} after ${fresh.map((e) => e.type)}`);
      assert.ok(m.pose.position.distanceTo(s.pose.position) < 1e-6, `${s.key} not on its place`);
    }
    // No face the human may not see, at rest or on the way.
    // A reply can run on into the next deal: cards of the deal that ended in it are no longer hidden
    // by the new deal's own audit, and codes recur from deal to deal.
    const lastDeal = fresh.map((e) => e.type).lastIndexOf("deal_start");
    const pm = prev.model;
    const older = new Set(lastDeal > 0 ? [...fresh.slice(0, lastDeal).flatMap((e) => [e.card, ...(e.cards ?? [])]), ...pm.won.flat(), ...pm.counted.flat(), ...pm.trick.map((c) => c.code)] : []);
    const secret = (code) => hidden.has(code) && !older.has(code);
    for (const m of result.placement) assert.ok(!m.code || !secret(m.code), `${m.code} shown but hidden`);
    for (const m of result.motions) assert.ok(!m.reveal?.code || !secret(m.reveal.code), `${m.reveal?.code} revealed but hidden in ${fresh.map((e) => e.type + (e.card ?? ""))}`);
    // No card through the table, sampled along every motion.
    for (const m of result.motions) {
      for (let k = 0; k <= 12; k++) {
        const pose = m.path(k / 12);
        const low = Math.min(...cardCorners(pose).map((c) => c.y));
        assert.ok(low > -0.05, `a card ${low.toFixed(2)} cm below the table in ${fresh.map((e) => e.type)}`);
      }
      assert.ok(m.duration > 0 && m.delay >= 0);
    }
    // Every new event has a moment, in order.
    const idx = Object.keys(result.beats).map(Number).sort((a, b) => a - b);
    assert.equal(idx.length, state.events.length - prev.events.length, "a beat for every event");
    // A trick won is heard as the trick is complete, and its cards land on the pile later (TABLE3D 2.1: the tally moves then).
    fresh.forEach((e, k) => {
      const at = prev.events.length + k;
      if (e.type === "trick_won") {
        assert.ok(result.landed[at] >= result.beats[at] + 1000, `trick ${e.trick}: lands ${result.landed[at]} after it is won ${result.beats[at]}`);
        assert.ok(result.landed[at] <= result.duration, "inside the stage");
      } else assert.equal(result.landed[at], undefined, `${e.type} has no landing`);
    });
    placement = result.placement;
    prev = state;
  }
  return seen;
}

test("whole sittings choreograph: each stage ends on its layout, shows no hidden face, keeps every card above the table", async () => {
  const seen = new Set();
  for (const [config, policy, extend] of [
    [{ seed: 7, touren: 3, human: 1, level: 1 }, taker(7), false],
    [{ seed: 3, touren: 3, human: 0, level: 1 }, passer(3), true],
  ]) {
    for (const k of await walk(config, policy, extend)) seen.add(k);
  }
  for (const k of ["dealt", "packet_turned", "card_pulled", "cover_set", "card_played", "trick_won", "count", "game_scored", "sitting_end"]) assert.ok(seen.has(k), k);
});

test("the portrait arrangement lays out and choreographs the same", async () => {
  await walk({ seed: 5, touren: 3, human: 1, level: 1 }, taker(5), false, ZONES_PORTRAIT);
});

test("a void deal and a throw-in choreograph", async () => {
  // A void deal found by seed search (seed 520, human 0: the opponent had no tarock).
  const config = { seed: 520, touren: 3, human: 0, level: 1 };
  const { replies } = await playSitting(config, { policy: taker(1), limit: 3 });
  assert.ok(replies[0].events.some((e) => e.type === "void_deal"));
  let prev = opening({ model: emptyModel(0), events: [replies[0].events[0]] });
  let model = emptyModel(0);
  const log = [];
  for (const e of replies[0].events) {
    model = reduce(model, e);
    log.push(e);
  }
  const next = { model, events: log };
  const result = choreograph(prev, next, initialPlacement(prev, view), view);
  assert.equal(result.placement.length, 54);
  assert.ok(result.duration > 0);
  const stages = stagesBetween(prev, next).map((s) => s.kind);
  assert.ok(stages.includes("void"));
  void fromState;
});

// 2.3.1: trick 27 is slower, 1.2 times.
test("the plays of trick 27 take 1.2 times as long as any other", async () => {
  const seq = await states({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  let prev = opening(seq[0].state);
  let placement = initialPlacement(prev, view);
  const lasting = { last: new Set(), other: new Set() };
  for (const { state, fresh } of seq) {
    const result = choreograph(prev, state, placement, view);
    const only = fresh.length === 1 && fresh[0].type === "card_played";
    if (only) lasting[fresh[0].trick === LAST_TRICK ? "last" : "other"].add(Math.max(...result.motions.map((m) => m.duration)));
    placement = result.placement;
    prev = state;
  }
  assert.equal(LAST_TRICK, 27);
  assert.ok(lasting.last.size > 0, "the last trick's plays were seen");
  assert.deepEqual([...lasting.other].map((d) => Math.round(d)), [TIMING.play], "other plays last TIMING.play");
  assert.deepEqual([...lasting.last].map((d) => Math.round(d)), [Math.round(TIMING.play * SLOW_LAST)]);
  assert.equal(SLOW_LAST, 1.2);
});

// 2.4: on a Valat the winner's whole pile is fanned a moment, then laid back.
test("a Valat fans the winner's won pile and lays it back where it was", async () => {
  const seq = await states({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  const at = seq.find((s) => s.state.model.won[0].length >= 6 && s.state.model.trick.length === 0);
  assert.ok(at, "a state with a pile to fan");
  const prev = at.state;
  const next = { model: prev.model, events: [...prev.events, { type: "valat", seat: 0 }] };
  const placement = initialPlacement(prev, view);
  const result = choreograph(prev, next, placement, view);
  const zone = prev.model.human === 0 ? "your-won" : "their-won";
  const pile = placement.filter((m) => m.zone === zone);
  assert.equal(pile.length, prev.model.won[0].length);
  const moved = new Set(result.motions.map((m) => m.id));
  for (const m of pile) assert.ok(moved.has(m.id), "every card of the pile moves");
  const top = Math.max(...result.motions.filter((m) => pile.some((p) => p.id === m.id)).map((m) => m.path(0.5).position.y));
  const rest = Math.max(...pile.map((m) => m.pose.position.y));
  assert.ok(top > rest + 2, "the pile is lifted into a fan");
  for (const m of pile) {
    const end = result.placement[m.id].pose.position;
    assert.ok(end.distanceTo(m.pose.position) < 1e-6, "laid back in place");
  }
  for (const m of result.motions) {
    for (let k = 0; k <= 20; k++) {
      const low = Math.min(...cardCorners(m.path(k / 20)).map((c) => c.y));
      assert.ok(low > -0.05, `a fanned card ${low.toFixed(1)} cm below the table`);
    }
  }
  assert.ok(result.beats[prev.events.length] !== undefined, "the Valat has its moment");
  assert.ok(result.duration >= TIMING.valat);
});
