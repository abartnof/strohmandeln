// The running tally (docs/TABLE3D.md 2.5.3): the take so far, in exact thirds, after each
// trick, counted as the count will count it. It ends on the engine's own numbers, so the
// table's card values (cardinfo.js thirdsOf) are the engine's for every card that is won.
import test from "node:test";
import assert from "node:assert/strict";
import { runningTally } from "../src/counting.js";
import { playSitting, random, taker } from "./sim.js";

test("before a trick is won the tally is nothing; a trick adds its cards' thirds to its winner only", () => {
  assert.deepEqual(runningTally([{ type: "deal_start" }], 0), { you: 0, opp: 0 });
  const events = [{ type: "deal_start" }, { type: "trick_won", winner: 1, cards: ["Kh", "4d"] }, { type: "trick_won", winner: 0, cards: ["SK", "T2"] }];
  assert.deepEqual(runningTally(events.slice(0, 2), 1), { you: 13 + 1, opp: 0 });
  assert.deepEqual(runningTally(events, 1), { you: 14, opp: 13 + 1 });
  assert.deepEqual(runningTally(events, 0), { you: 14, opp: 14 }, "the human's seat says which line is 'you'");
});

test("a new deal begins the tally again", () => {
  const events = [{ type: "deal_start" }, { type: "trick_won", winner: 0, cards: ["Kh", "4d"] }, { type: "deal_start" }];
  assert.deepEqual(runningTally(events, 0), { you: 0, opp: 0 });
});

for (const [name, policy] of [["a declarer", taker], ["random play", random]]) {
  test(`over played sittings (${name}) the running value after trick 27 is the count's total for both sides, and it never falls`, async () => {
    let counts = 0;
    for (const [seed, human] of [[3, 0], [7, 1], [11, 1], [19, 0]]) {
      const { replies } = await playSitting({ seed, touren: 3, human, level: 1 }, { policy: policy(seed), limit: 400 });
      const log = [];
      let before = { you: 0, opp: 0 };
      let tricks = 0;
      for (const r of replies)
        for (const e of r.events) {
          log.push(e);
          if (e.type === "deal_start") {
            before = { you: 0, opp: 0 };
            tricks = 0;
          }
          if (e.type === "trick_won") {
            tricks++;
            const now = runningTally(log, human);
            assert.ok(now.you >= before.you && now.opp >= before.opp, "a tally never falls");
            assert.equal(now.you + now.opp > before.you + before.opp, true, "each trick adds something");
            before = now;
          }
          if (e.type === "count") {
            assert.equal(tricks, 27);
            assert.deepEqual(runningTally(log, human), { you: e.thirds[human], opp: e.thirds[1 - human] });
            assert.equal(before.you + before.opp, 210);
            counts++;
          }
        }
    }
    assert.ok(counts >= 4, `${counts} counts checked`);
  });
}
