// The review dialog's words: the module's bullets (docs/PROTOCOL.md "The
// review") put into the current language with cards named and thirds written.
import test from "node:test";
import assert from "node:assert/strict";
import { explainText, reviewBlocks, words } from "../src/review.js";
import { createStrings, thirds, signed } from "../src/strings.js";
import TABLE from "../strings.json" with { type: "json" };

const fmt = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, cardName: s.cardName, thirds, signed };
};

const bullets = [
  { lead: "count", key: "count", params: { mine: 120, theirs: 90 } },
  { lead: "decided", key: "trick-lost-rich", params: { trick: 4, mine: "Qs", theirs: "Ks", thirds: 23 } },
  { lead: "try", key: "try-announce", params: { honour: "trull" } },
];

test("the count is written in points and thirds", () => {
  assert.equal(words(bullets[0], fmt("en")), "You counted 40 to your opponent's 30; 35⅔ was the line.");
  assert.equal(words({ lead: "count", key: "count", params: { mine: 107, theirs: 103 } }, fmt("en")), "You counted 35⅔ to your opponent's 34⅓; 35⅔ was the line.");
});

test("cards are named in the language, the honour too", () => {
  assert.equal(words(bullets[1], fmt("en")), "Trick 4 was the rich one you lost: queen of spades against king of spades, 7⅔ between them.");
  assert.equal(words(bullets[1], fmt("de")), "Der 4. Stich war der reiche, den Sie abgaben: Pik-Dame gegen Pik-König, zusammen 7⅔.");
  assert.match(words(bullets[2], fmt("en")), /you held the Trull and did not announce it/);
  assert.match(words(bullets[2], fmt("de")), /Sie hielten die Troule und sagten nicht an/);
});

test("the sitting's net is signed", () => {
  const b = { lead: "sitting", key: "sitting-tally", params: { won: 3, lost: 2, drawn: 1, net: -4 } };
  assert.equal(words(b, fmt("en")), "Deals: 3 won, 2 lost, 1 drawn; −4 Points in all.");
});

test("a review is a title, up to three bullets with a lead each, and the method", () => {
  const r = reviewBlocks({ deal: 3, bullets }, fmt("en"));
  assert.equal(r.title, "How the deal went");
  assert.deepEqual(r.bullets.map((b) => b.lead), ["The count.", "Where it was decided.", "One thing to try."]);
  assert.match(r.method, /thirds/);
  assert.equal(r.how, "How is this worked out?");
  assert.equal(reviewBlocks({ bullets }, fmt("de"), { whole: true }).title, "Wie der Abend verlief");
  assert.equal(reviewBlocks({ bullets: [...bullets, ...bullets] }, fmt("en")).bullets.length, 3);
});

test("an explanation names the suit and the cards in the language", () => {
  const s = (lang) => {
    const x = createStrings(TABLE, lang);
    return { t: x.t, cardName: x.cardName, suitName: (c) => (lang === "en" ? x.suitName(c).toLowerCase() : x.suitName(c)) };
  };
  const trump = { lead: "explain", key: "explain-trump-opp", params: { suit: "h" } };
  assert.equal(explainText(trump, s("en")), "Trump: your opponent has no hearts in hand or showing.");
  assert.equal(explainText(trump, s("de")), "Tarock: Ihr Gegner hat kein Herz in der Hand oder offen.");
  const won = { lead: "explain", key: "explain-won-you", params: { winner: "T21", loser: "Qd" } };
  assert.equal(explainText(won, s("en")), "Your trick: Mond beat queen of diamonds.");
  assert.equal(explainText({ lead: "explain", key: "explain-notarock-you", params: {} }, s("en")), "You had no tarock in hand or showing, so any card went.");
  assert.equal(explainText(null, s("en")), "");
});

test("a review with nothing to say is null", () => {
  assert.equal(reviewBlocks(null, fmt("en")), null);
  assert.equal(reviewBlocks({ bullets: [] }, fmt("en")), null);
});

test("the review is gentle: no bullet names a fault or a person", () => {
  const bad = /\b(stupid|foolish|blunder|mistake|error|wrong|careless|bad player|fool)/i;
  for (const [key, v] of Object.entries(TABLE)) {
    if (!key.startsWith("review.say.") && !key.startsWith("explain.")) continue;
    assert.ok(!bad.test(v.en), key);
  }
});
