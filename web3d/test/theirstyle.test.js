// "Their style", the after-deal brief's fourth line (docs/STYLES.md 5, docs/TUTOR.md 6): how the
// engine is asked (the sitting's earlier records in the header, for a styled opponent only) and
// how the line is told (a fact in the Kibitzer's voice, in both languages, naming no style).
import test from "node:test";
import assert from "node:assert/strict";
import { briefHead } from "../src/tutorwire.js";
import { createBriefing } from "../src/briefing.js";
import { briefBlocks, tutorWords } from "../src/review.js";
import { createStrings, thirds, signed } from "../src/strings.js";
import { fakeStore } from "./support/fakedom.js";
import TABLE from "../strings.json" with { type: "json" };

const fmt = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, lang, cardName: s.romanName, thirds, signed, suitName: s.suitName };
};

test("the earlier deals ride in the header, a line each, oldest first", () => {
  assert.equal(briefHead(3, { style: "bold", earlier: ["{a}", "{b}"] }), "level 3\nstyle bold\nearlier {a}\nearlier {b}");
  assert.equal(briefHead(3, { style: "random", drawn: "ms", earlier: ["{a}"] }), "level 3\nstyle random\ndrawn ms\nearlier {a}");
  assert.equal(briefHead(3, { style: "bold", earlier: [] }), "level 3\nstyle bold");
});

test("the briefing sends this sitting's earlier records, before the deal, for a styled opponent only", () => {
  const seen = [];
  let current = "r1";
  const engine = {
    briefOfRecord: (...a) => (seen.push(a), { brief: { bullets: [], summary: null, detail: null } }),
    learner: () => ({ version: 1 }),
    dealRecord: () => ({ record: current }),
  };
  const make = (style) => createBriefing({ store: fakeStore(), engine, level: () => 3, head: () => ({ style }) });
  const bold = make("bold");
  const entries = ["r1", "r2", "r3"].map((r, i) => {
    current = r;
    return bold.recordDeal({ assisted: [], n: i + 1, sit: i === 0 ? "7" : "8" });
  });
  bold.briefOf(entries[2]);
  assert.deepEqual(seen.at(-1)[4], { style: "bold", earlier: ["r2"] });
  const normal = make("normal");
  current = "q1";
  normal.briefOf(normal.recordDeal({ assisted: [], n: 1, sit: "9" }));
  assert.deepEqual(seen.at(-1)[4], { style: "normal" }, "Normal needs no records");
});

const review = { deal: 3, bullets: [{ lead: "count", key: "count", params: { mine: 120, theirs: 90 } }] };
const base = [{ lead: "tutor.lead.well", key: "tutor.well.met", params: { chances: 3, skill: "off" } }];

test("their style is one more line, a fact; no style is named", () => {
  const line = { lead: "tutor.lead.style", key: "tutor.style.took", params: { n: 4, of: 6 } };
  const brief = { method: "tutor.method", bullets: [...base, line] };
  const e = briefBlocks(brief, review, fmt("en"));
  assert.deepEqual(e.bullets[1], { lead: "Their style:", text: "your opponent has taken up 4 of 6 bids so far." });
  const d = briefBlocks(brief, review, fmt("de"));
  assert.deepEqual(d.bullets[1], { lead: "Spielweise des Gegners:", text: "Ihr Gegner hat bisher 4 von 6 Geboten aufgenommen." });
  const none = { lead: "tutor.lead.style", key: "tutor.style.took-none", params: { of: 5 } };
  assert.equal(tutorWords(none, fmt("en")).text, "your opponent has not taken up in 5 bids so far.");
  const once = { lead: "tutor.lead.style", key: "tutor.style.took-once", params: { of: 14 } };
  assert.equal(tutorWords(once, fmt("de")).text, "Ihr Gegner hat bisher in 14 Geboten nur einmal aufgenommen.");
});

test("four lines are shown, and with the tips off the style line goes with the advice", () => {
  const four = [
    ...base,
    { lead: "tutor.lead.next", key: "tutor.rule.cheap", params: { skill: "cheap" } },
    { lead: "tutor.lead.deal", key: "tutor.miss.worth.poor", params: { trick: 9, played: "Qh", better: "4h", thirds: 22 } },
    { lead: "tutor.lead.style", key: "tutor.style.took", params: { n: 4, of: 6 } },
  ];
  const brief = { method: "tutor.method", bullets: four };
  assert.equal(briefBlocks(brief, review, fmt("en")).bullets.length, 4);
  assert.deepEqual(briefBlocks(brief, review, fmt("en"), { tips: false }).bullets.map((b) => b.lead), ["Went well:"]);
});
