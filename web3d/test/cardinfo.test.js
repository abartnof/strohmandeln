import { test } from "node:test";
import assert from "node:assert/strict";
import { sortCodes } from "../src/cardinfo.js";

test("a hand lies high to low throughout: each suit from its king, the tarocks from the Sküs", () => {
  const hand = sortCodes(["T1", "SK", "T21", "3h", "Kh", "T10", "Qh"]);
  assert.deepEqual(hand, ["Kh", "Qh", "3h", "SK", "T21", "T10", "T1"]);
});
