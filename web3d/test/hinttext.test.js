// The hint as words: parameters named and formatted, every key of
// docs/MESSAGES.md voiced in both languages with exactly its parameters.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings, thirds } from "../src/strings.js";
import { hintLines, hintParams, hintSpoken } from "../src/hinttext.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const doc = readFileSync(new URL("../../docs/MESSAGES.md", import.meta.url), "utf8");
const section = (from, to) => doc.split(from)[1].split(to)[0];
const slots = (s) => [...new Set([...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]))].sort();

const words = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, cardName: s.cardName, romanName: s.romanName, thirds, count: s.count };
};

test("a card parameter is named, a _thirds parameter is told in points, a count is told by the language", () => {
  const p = hintParams({ card: "T19", led: "Jh", trick_thirds: 14, given_thirds: 1, tarocks: 8 }, words("en"));
  assert.deepEqual(p, { card: "tarock 19", led: "jack of hearts", trick_thirds: "4⅔ points", given_thirds: "⅓ point", tarocks: "8" });
  assert.equal(hintParams({ n_thirds: 3 }, words("en")).n_thirds, "1 point");
  assert.equal(hintParams({ n_thirds: 6 }, words("de")).n_thirds, "2 Augen");
});

test("a hint reads: the card, why, the rule, the odds", () => {
  const r = {
    hint: { action: "play", card: "T19", source: "hand" },
    reason: "win-cheapest",
    params: { card: "T19", led: "T15", alt: "T21", trick_thirds: 2 },
    rule: "cheapest",
    odds: { alt: "T21", better: 7, worlds: 10, gain_thirds: 4 },
  };
  const en = hintLines(r, words("en"));
  assert.equal(en.lead, "Play tarock 19.");
  assert.equal(en.reason, "Tarock 19 beats tarock 15 and takes ⅔ point; it is the cheapest winner, and Mond also wins but costs more to spend.");
  assert.equal(en.more.length, 2);
  assert.match(en.more[0], /^Rule: do not spend a king/);
  assert.equal(en.more[1], "Against Mond: ahead in 7 of 10 sampled deals, by 1⅓ points on average.");
  assert.match(hintSpoken(en), /^Play tarock 19\. Tarock 19 beats/);
  const de = hintLines(r, words("de"));
  assert.equal(de.lead, "Tarock 19 spielen.");
  assert.match(de.reason, /^Tarock 19 sticht Tarock 15 und holt ⅔ Auge;/);
});

test("a hint without a card to play has no lead, without rule or odds no more", () => {
  const r = { hint: { action: "take" }, reason: "bid-weak-hand", params: { sure_thirds: 40, tarocks: 4 }, rule: null, odds: null };
  const en = hintLines(r, words("en"));
  assert.equal(en.lead, "");
  assert.equal(en.reason, "Cards sure to take a trick, worth only 13⅓ points (4 tarocks): too few to risk taking it up.");
  assert.deepEqual(en.more, []);
});

test("every hint key of MESSAGES.md has a string with exactly its parameters, in both languages", () => {
  const rows = [...section("## Hint reasons", "### Hint rules").matchAll(/^\| `([a-z-]+)` \| ([^|]*) \|/gm)];
  assert.ok(rows.length >= 20);
  for (const [, key, params] of rows) {
    const e = TABLE[`hint.reason.${key}`];
    assert.ok(e, key);
    const want = [...params.matchAll(/`([a-z_]+)`/g)].map((m) => m[1]).sort();
    assert.deepEqual(slots(e.en), want, key);
    assert.deepEqual(slots(e.de), want, key);
  }
});

test("the hint strings say what MESSAGES.md says, word for word", () => {
  const reasons = [...section("## Hint reasons", "### Hint rules").matchAll(/^\| `([a-z-]+)` \| [^|]*\| ([^|]*) \| ([^|]*) \|$/gm)];
  const rules = [...section("### Hint rules", "### Hint odds").matchAll(/^\| `([a-z-]+)` \| ([^|]*) \| ([^|]*) \| [^|]* \|$/gm)];
  assert.ok(reasons.length >= 20 && rules.length >= 8);
  for (const [, key, en, de] of reasons) assert.deepEqual(TABLE[`hint.reason.${key}`], { en: en.trim(), de: de.trim() }, key);
  for (const [, key, en, de] of rules) assert.deepEqual(TABLE[`hint.rule.${key}`], { en: en.trim(), de: de.trim() }, key);
  assert.ok(TABLE["hint.reason.decline-rich"]);
});

test("every rule of MESSAGES.md has a string, and every row cites its basis", () => {
  const rows = [...section("### Hint rules", "### Hint odds").matchAll(/^\| `([a-z-]+)` \| [^|]* \| [^|]* \| ([^|]*) \|/gm)];
  assert.ok(rows.length >= 8);
  for (const [, key, basis] of rows) {
    assert.ok(TABLE[`hint.rule.${key}`], key);
    assert.match(basis, /research\/|RULES/, key);
  }
  assert.deepEqual(slots(TABLE["hint.odds"].en), ["alt", "better", "gain_thirds", "worlds"]);
});

test("no hint key is left in the strings that MESSAGES.md does not give", () => {
  const given = new Set([...section("## Hint reasons", "### Hint rules").matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]));
  for (const k of Object.keys(TABLE)) {
    const m = /^hint\.reason\.(.+)$/.exec(k);
    if (m) assert.ok(given.has(m[1]), k);
  }
});

test("shown tarocks use the faces' Roman numerals; Pagat, Mond, Sküs keep their names; zero is plural", () => {
  const w = words("en");
  const name = w.romanName;
  assert.deepEqual(["T12", "T4", "T1", "T21", "SK", "Jh"].map(name), ["tarock XII", "tarock IIII", "Pagat", "Mond", "Sküs", "jack of hearts"]);
  assert.equal(hintParams({ n_thirds: 0 }, w).n_thirds, "0 points");
  assert.equal(hintLines({ hint: { action: "play", card: "T12" }, reason: "only-card", params: { card: "T12" } }, { ...w, cardName: name }).lead, "Play tarock XII.");
});

test("a count of a thousand or more is grouped as the language writes it", () => {
  const rate = { hint: { action: "play", card: "Kh" }, reason: "lead-rate", params: { card: "Kh", wins: 1000, worlds: 1000 }, rule: null, odds: null };
  assert.equal(hintLines(rate, words("en")).reason, "King of hearts takes the trick in 1,000 of 1,000 sampled deals.");
  assert.equal(hintLines(rate, words("de")).reason, "Herz-König gewinnt den Stich in 1.000 von 1.000 durchgespielten Austeilungen.");
  const part = { ...rate, params: { card: "Kh", wins: 640, worlds: 1000 } };
  assert.match(hintLines(part, words("en")).reason, /in 640 of 1,000 sampled deals\.$/);
  assert.match(hintLines(part, words("de")).reason, /in 640 von 1\.000 durchgespielten/);
  const odds = hintLines({ ...part, odds: { alt: "T21", better: 31, worlds: 50, gain_thirds: 4 } }, words("en"));
  assert.equal(odds.more[0], "Against Mond: ahead in 31 of 50 sampled deals, by 1⅓ points on average.");
  assert.equal(hintParams({ out: 999, wins: 10000 }, words("en")).wins, "10,000");
  assert.equal(hintParams({ out: 999 }, words("en")).out, "999");
  // A caller that passes no count gets the plain number.
  const { count, ...plain } = words("en");
  assert.equal(hintParams({ wins: 1000 }, plain).wins, 1000);
});

test("the spoken hint says the counts in the language's grouping", () => {
  const rate = { hint: { action: "play", card: "Kh" }, reason: "lead-rate", params: { card: "Kh", wins: 848, worlds: 1000 }, rule: null, odds: null };
  assert.equal(hintSpoken(hintLines(rate, words("en"))), "Play king of hearts. King of hearts takes the trick in 848 of 1,000 sampled deals.");
  assert.equal(hintSpoken(hintLines(rate, words("de"))), "Herz-König spielen. Herz-König gewinnt den Stich in 848 von 1.000 durchgespielten Austeilungen.");
});
