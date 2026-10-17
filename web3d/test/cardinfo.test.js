import { test } from "node:test";
import assert from "node:assert/strict";
import { sortCodes } from "../src/cardinfo.js";

test("a hand lies high to low throughout: each suit from its king, the tarocks from the Sküs", () => {
  const hand = sortCodes(["T1", "SK", "T21", "3h", "Kh", "T10", "Qh"]);
  assert.deepEqual(hand, ["Kh", "Qh", "3h", "SK", "T21", "T10", "T1"]);
});

import { redPipBeat } from "../src/cardinfo.js";

// RULES 1.5: in hearts and diamonds the pips run 1 high to 4 low. A trick is the
// note's moment when a red pip took a higher-numbered pip of its suit.
test("redPipBeat: the lower-numbered red pip beats the higher-numbered one of its suit", () => {
  assert.deepEqual(redPipBeat(["4h", "1h"]), { win: "1h", lose: "4h" });
  assert.deepEqual(redPipBeat(["2d", "3d"]), { win: "2d", lose: "3d" });
  assert.deepEqual(redPipBeat(["1d", "4d"]), { win: "1d", lose: "4d" });
});

test("redPipBeat: nothing for black pips, courts, tarocks, other suits or a pip beaten by a higher rank", () => {
  assert.equal(redPipBeat(["10c", "7c"]), null);
  assert.equal(redPipBeat(["8s", "9s"]), null);
  assert.equal(redPipBeat(["1h", "Jh"]), null);
  assert.equal(redPipBeat(["Kd", "4d"]), null);
  assert.equal(redPipBeat(["3h", "T2"]), null);
  assert.equal(redPipBeat(["3h", "1d"]), null);
  assert.equal(redPipBeat(["3h"]), null);
});
