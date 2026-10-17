// The sitting's brief names what a Random style drew, once the engine gives it (docs/STYLES.md 2.3).
import test from "node:test";
import assert from "node:assert/strict";
import { briefBlocks } from "../src/review.js";
import { createStrings, thirds, signed } from "../src/strings.js";
import TABLE from "../strings.json" with { type: "json" };

const fmt = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, lang, cardName: s.romanName, thirds, signed, suitName: (c) => s.suitName(c) };
};
const review = { deal: 3, bullets: [{ lead: "count", key: "count", params: { mine: 120, theirs: 90 } }] };
const brief = { method: "tutor.method", bullets: [] };

test("the end-of-sitting brief reveals the drawn style; the deal's brief and a chosen style never do", () => {
  const drawn = { ...brief, style: { id: "random", drawn: "ulmann" } };
  assert.equal(briefBlocks(drawn, review, fmt("en"), { whole: true }).reveal, "Your opponent's style was drawn at random: \"After Ulmann\".");
  assert.equal(briefBlocks(drawn, review, fmt("de"), { whole: true }).reveal, "Der Stil Ihres Gegners wurde gelost: „Nach Ulmann“.");
  assert.equal(briefBlocks({ ...brief, style: { id: "random", drawn: null } }, review, fmt("en"), { whole: true }).reveal, null, "hidden: the engine withholds it");
  assert.equal(briefBlocks({ ...brief, style: { id: "bold", drawn: null } }, review, fmt("en"), { whole: true }).reveal, null);
  assert.equal(briefBlocks(drawn, review, fmt("en"), { whole: false }).reveal, null);
});
