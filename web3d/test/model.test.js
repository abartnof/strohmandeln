import { test } from "node:test";
import assert from "node:assert/strict";
import { check, emptyModel, fromState, reduce } from "../src/model.js";
import { announcer, passer, playSitting, random, taker } from "./sim.js";

// The model by events, kept across a whole sitting, must equal the state's
// own account of the deal at every reply, and the model made from the state
// (by replaying the deal's events) must equal it.
async function walk(config, policy, extend = false) {
  const seen = new Set();
  const { replies } = await playSitting(config, { policy, extend });
  let m = emptyModel(config.human);
  for (const r of replies) {
    for (const e of r.events) {
      m = reduce(m, e);
      seen.add(e.type);
    }
    if (r.state.deal) {
      assert.deepEqual(check(m, r.state), [], `after ${r.events.map((e) => e.type)}`);
      const f = fromState(r.state);
      for (const k of ["hand", "opp", "packets", "held", "trick", "won", "counted", "pack", "announced", "dealer", "declarer", "multiplier", "forehand"]) assert.deepEqual(f[k], m[k], k);
    }
  }
  assert.ok(m.over, "the sitting ended");
  return { replies, seen };
}

test("events and states agree through whole sittings, taking and passing", async () => {
  const kinds = new Set();
  for (const [seed, policy, extend] of [
    [7, taker(7), false],
    [11, passer(11), true],
    [3, random(3), false],
    [1, announcer(1), false],
  ]) {
    const { seen } = await walk({ seed, touren: 3, human: seed % 2, level: 1 }, policy, extend);
    for (const k of seen) kinds.add(k);
  }
  for (const k of ["dealt", "card_pulled", "cover_set", "packet_empty", "card_played", "trick_won", "count", "sitting_end", "announcement"]) assert.ok(kinds.has(k), k);
});

test("a throw-in puts every card back in the pack", async () => {
  const { replies } = await playSitting({ seed: 1, touren: 3, human: 1, level: 1, settings: { both_pass: "throw_in" } }, { policy: passer(1) });
  let m = emptyModel(1);
  let thrown = 0;
  for (const r of replies)
    for (const e of r.events) {
      m = reduce(m, e);
      if (e.type === "thrown_in") {
        thrown++;
        assert.equal(m.pack, 54);
        assert.equal(m.hand.length, 0);
      }
    }
  assert.ok(thrown > 0);
});

test("a void deal is named, and the next deal starts from a full pack", () => {
  let m = emptyModel(0);
  m = reduce(m, { type: "deal_start", deal_number: 1, dealer: 0, forehand: 1, multiplier: 1, tour: 1 });
  m = reduce(m, { type: "dealt", seat: 0, cards: ["9s", "8c", "Qs", "3d", "Qh", "Kh", "7s", "10s", "9c", "8s", "Jc", "Jd", "2h", "Cd", "Kc"] });
  assert.equal(m.pack, 0);
  m = reduce(m, { type: "void_deal", seats: [1] });
  assert.deepEqual(m.void, [1]);
  m = reduce(m, { type: "deal_start", deal_number: 1, dealer: 0, forehand: 1, multiplier: 1, tour: 1 });
  assert.equal(m.pack, 54);
  assert.equal(m.void, null);
});
