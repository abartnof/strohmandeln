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

// The pack as 27 tricks of two cards, won by `winnerOf(trickIndex)`.
const PACK = [...["h", "d"].flatMap((su) => ["K", "Q", "C", "J", "1", "2", "3", "4"].map((r) => r + su)), ...["c", "s"].flatMap((su) => ["K", "Q", "C", "J", "10", "9", "8", "7"].map((r) => r + su)), ...Array.from({ length: 21 }, (_, i) => `T${i + 1}`), "SK"];
const tricks = (winnerOf) => [{ type: "deal_start" }, ...Array.from({ length: 27 }, (_, i) => ({ type: "trick_won", winner: winnerOf(i), cards: [PACK[2 * i], PACK[2 * i + 1]] }))];

test("a Valat ends 210 to nothing, and any split of the pack into tricks makes 210 between the two lines", () => {
  assert.deepEqual(runningTally(tricks(() => 1), 1), { you: 210, opp: 0 });
  assert.deepEqual(runningTally(tricks(() => 1), 0), { you: 0, opp: 210 });
  // a split near even: give each trick to whoever is behind
  let a = 0;
  let b = 0;
  const drawn = tricks((i) => {
    const thirds = [PACK[2 * i], PACK[2 * i + 1]].reduce((n, c) => n + (c === "SK" || /^(K|T1$|T21$)/.test(c) ? 13 : /^Q/.test(c) ? 10 : /^C/.test(c) ? 7 : /^J/.test(c) ? 4 : 1), 0);
    const w = a <= b ? 0 : 1;
    if (w === 0) a += thirds;
    else b += thirds;
    return w;
  });
  const t = runningTally(drawn, 0);
  assert.equal(t.you + t.opp, 210);
  assert.deepEqual(t, { you: a, opp: b });
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
