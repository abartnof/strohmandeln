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
  return { t: s.t, cardName: s.cardName, romanName: s.romanName, thirds, count: s.count, lang };
};

test("a card parameter is named, a _thirds parameter is told in points, a count is told by the language", () => {
  const p = hintParams({ card: "T19", led: "Jh", trick_thirds: 14, given_thirds: 1, tarocks: 8 }, words("en"));
  assert.deepEqual(p, { card: "tarock 19", led: "jack of hearts", trick_thirds: "4⅔ card points", given_thirds: "⅓ card point", tarocks: "8" });
  assert.equal(hintParams({ n_thirds: 3 }, words("en")).n_thirds, "1 card point");
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
  assert.equal(en.reason, "Tarock 19 beats tarock 15 and takes ⅔ card point; it is the lowest-value card that wins, and Mond also wins but is worth more.");
  assert.equal(en.more.length, 2);
  assert.match(en.more[0], /^Rule: do not spend a king/);
  assert.equal(en.more[1], "Against Mond: ahead in 7 of 10 sampled deals, by 1⅓ card points on average.");
  assert.match(hintSpoken(en), /^Play tarock 19\. Tarock 19 beats/);
  const de = hintLines(r, words("de"));
  assert.equal(de.lead, "Tarock 19 spielen.");
  assert.match(de.reason, /^Tarock 19 sticht Tarock 15 und holt ⅔ Auge;/);
});

test("a hint without a card to play has no lead, without rule or odds no more", () => {
  const r = { hint: { action: "take" }, reason: "bid-weak-hand", params: { sure_thirds: 40, tarocks: 4 }, rule: null, odds: null };
  const en = hintLines(r, words("en"));
  assert.equal(en.lead, "");
  assert.equal(en.reason, "Cards sure to take a trick, worth only 13⅓ card points (4 tarocks): too few to risk taking it up.");
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
  assert.equal(hintParams({ n_thirds: 0 }, w).n_thirds, "0 card points");
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
  assert.equal(odds.more[0], "Against Mond: ahead in 31 of 50 sampled deals, by 1⅓ card points on average.");
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

// ---- the default hint: headline, question, one sentence (docs/HINTS.md 3.1) ----

const sentenceRows = () => [...section("### Hint questions, sentences", "## Review keys").matchAll(/^\| `([a-z.-]+)` \| ([^|]*) \| ([^|]*) \| ([^|]*) \|$/gm)];

test("every question and sentence of MESSAGES.md is a string with exactly its parameters, word for word", () => {
  const rows = sentenceRows();
  assert.equal(rows.length, 8 + 28);
  for (const [, key, params, en, de] of rows) {
    assert.deepEqual(TABLE[`hint.${key}`], { en: en.trim(), de: de.trim() }, key);
    const want = [...params.matchAll(/`([a-z_]+)`/g)].map((m) => m[1]).sort();
    assert.deepEqual(slots(en), want, key);
    assert.deepEqual(slots(de), want, key);
  }
});

const play = (card) => ({ action: "play", card, source: "hand" });
const mondCase = {
  hint: play("T21"),
  reason: "win-over-cheaper",
  params: { card: "T21", led: "T2", alt: "T3", trick_thirds: 14 },
  rule: null,
  odds: null,
  question: "ask.keep",
  keep: "T21",
  so: null,
  ans: { key: "ans.keep-mond-play", params: {} },
  why: { alt: "T3", answers: [], contrast: null, trumped: null, facts: [{ key: "fact.skus-out", params: {}, anchor: "row:trump" }, { key: "fact.tarocks-out", params: { out: 8, mine: 13 }, anchor: "line1" }], rule: null, odds: null },
};
const coverCase = {
  hint: play("Jd"),
  reason: "lead-cost",
  params: { card: "Jd", cost_thirds: 4 },
  rule: null,
  odds: null,
  question: "ask.trumps",
  keep: null,
  so: null,
  ans: { key: "ans.trumps-no", params: { mine: 6, total: 22 } },
  why: { alt: "T19", answers: [], contrast: { key: "vs.lead", params: { card: "Jd", alt: "T19", worlds: 1000, wins: 130, alt_wins: 940, cost_thirds: 4, alt_cost_thirds: 1 } }, trumped: null, facts: [], rule: null, odds: null },
};

test("the Mond over III: the card once, the question with its article, the fact; the Pagat is not mentioned", () => {
  const en = hintLines(mondCase, words("en"));
  assert.deepEqual([en.lead, en.question, en.text], ["Play Mond.", "Is the Mond safe to keep?", "Not while the Sküs is out: play it on his tarock, where it wins."]);
  assert.equal(hintSpoken(en), "Play Mond. Is the Mond safe to keep? Not while the Sküs is out: play it on his tarock, where it wins.");
  assert.doesNotMatch(JSON.stringify(en), /Pagat/);
  const de = hintLines(mondCase, words("de"));
  assert.deepEqual([de.lead, de.question, de.text], ["Mond spielen.", "Ist der Mond sicher aufgehoben?", "Nicht, solange der Sküs draußen ist: ihn auf sein Tarock spielen, wo er sticht."]);
});

test("the cover lead: the question about tarocks and the counts, no part repeats the headline", () => {
  const en = hintLines(coverCase, words("en"));
  assert.deepEqual([en.lead, en.question, en.text], ["Play jack of diamonds.", "Lead a tarock?", "Not yet: you hold 6 of the 22 still out."]);
  for (const part of [en.question, en.text]) assert.doesNotMatch(part, /^Play\b/);
  assert.equal(en.lead.split("Play").length, 2, "the imperative once");
  assert.equal(hintLines(coverCase, words("de")).question, "Tarock anspielen?");
});

test("a rule (so) replaces the fact; without a question or sentence the plain reason stands", () => {
  const so = { ...mondCase, question: "ask.worth", keep: null, so: { key: "so.worth-rich", params: {} } };
  assert.equal(hintLines(so, words("en")).text, "When he leads a card worth 2⅓ card points or more, take the trick.");
  const plain = { hint: play("Kh"), reason: "lead-cost", params: { card: "Kh", cost_thirds: 13 }, rule: null, odds: null };
  const en = hintLines(plain, words("en"));
  assert.equal(en.question, "");
  assert.equal(en.text, en.reason);
  const asReason = { ...plain, ans: { key: "reason.lead-cost", params: plain.params } };
  assert.equal(hintLines(asReason, words("en")).text, "If he takes the trick, you lose only 4⅓ card points with king of hearts.");
  assert.equal(hintSpoken(hintLines(asReason, words("en"))), "Play king of hearts. If he takes the trick, you lose only 4⅓ card points with king of hearts.");
});

test("a suit is named by the table, a kept packet by its card", () => {
  const out = { ...coverCase, question: "ask.out", ans: { key: "ans.out-void", params: { suit: "h", out: 7 } } };
  assert.equal(hintLines(out, words("en")).text, "He showed no hearts; 7 tarocks are out.");
  assert.equal(hintLines(out, words("de")).text, "Er zeigte kein Herz; 7 Tarock sind draußen.");
  const suit = { ...coverCase, question: "ask.suit", ans: { key: "ans.suit", params: { suit: "d", mine_thirds: 17, out_thirds: 12 } } };
  assert.equal(hintLines(suit, words("en")).text, "Diamonds: yours 5⅔ card points, out 4 card points.");
  const packet = { ...mondCase, keep: "Jd", ans: { key: "reason.keep-packet", params: {} } };
  assert.equal(hintLines(packet, words("en")).question, "Is the packet showing jack of diamonds safe to keep?");
  assert.equal(hintLines(packet, words("de")).question, "Ist das Paket mit Karo-Bube sicher aufgehoben?");
});

test("the Why? parts are told: the contrast, the facts with their anchors, null parts dropped", () => {
  const w = hintLines(coverCase, words("en")).why;
  assert.equal(w.contrast, "Instead of tarock 19: it took the trick in 940 of 1,000 deals and lost ⅓ card point when it did not; jack of diamonds took it in 130 and lost 1⅓ card points.");
  assert.equal(w.trumped, null);
  assert.deepEqual(w.facts, []);
  const f = hintLines(mondCase, words("en")).why.facts;
  assert.deepEqual(f, [
    { text: "The Sküs has not been played.", anchor: "row:trump" },
    { text: "8 tarocks are still out; you hold 13.", anchor: "line1" },
  ]);
  assert.equal(hintLines({ hint: { action: "take" }, reason: "bid-weak-hand", params: { sure_thirds: 40, tarocks: 4 } }, words("en")).why, null);
});

// ---- the word budgets (docs/HINTS.md 3.1, 9): headline + question + sentence, a placeholder one word ----

const wordCount = (s) => s.trim().split(/\s+/).filter(Boolean).length;
const budgetPairs = {
  "ask.keep": ["ans.keep-mond-play", "ans.keep-mond-hold", "ans.keep-pagat-hold", "ans.keep-packet"],
  "ask.beneath": ["ans.beneath"],
  "ask.worth": ["ans.worth-poor", "ans.worth-rich", "so.worth-rich"],
  "ask.cheap": ["ans.cheap", "ans.cheap-lose", "ans.ask-differ"],
  "ask.out": ["ans.out-void", "ans.out-master", "ans.out-master-tarock"],
  "ask.suit": ["ans.suit"],
  "ask.trumps": ["ans.trumps-no", "ans.trumps-yes"],
  "": ["ans.ask-none", "ans.ask-none-even"],
};

test("headline, question and sentence stay within 25 words in English and 30 in German", () => {
  for (const [lang, limit] of [["en", 25], ["de", 30]]) {
    const head = wordCount(TABLE["hint.play"][lang]);
    for (const [q, sentences] of Object.entries(budgetPairs)) {
      const qn = q ? wordCount(TABLE[`hint.${q}`][lang]) : 0;
      for (const s of sentences) {
        const n = head + qn + wordCount(TABLE[`hint.${s}`][lang]);
        assert.ok(n <= limit, `${lang} ${q} ${s}: ${n} words`);
      }
    }
  }
});

test("every sentence the engine can give is in a budget pair, and each contrast line is at most 30 words", () => {
  const paired = new Set(Object.values(budgetPairs).flat());
  for (const [, key] of sentenceRows()) {
    if (/^(ans|so)\./.test(key)) assert.ok(paired.has(key), key);
    if (/^vs\./.test(key)) for (const lang of ["en", "de"]) assert.ok(wordCount(TABLE[`hint.${key}`][lang]) <= 30, `${lang} ${key}`);
  }
});

test("no reason starts with the play imperative: the headline says it once", () => {
  const rows = [...section("## Hint reasons", "### Hint rules").matchAll(/^\| `([a-z-]+)` \| ([^|]*) \| ([^|]*) \| ([^|]*) \|$/gm)];
  assert.ok(rows.length >= 20);
  for (const [, key, , en, de] of rows) {
    assert.doesNotMatch(en.trim(), /^play\b/i, key);
    assert.doesNotMatch(de.trim(), /^\{card\} spielen/, key);
    assert.doesNotMatch(TABLE[`hint.reason.${key}`].en, /^play\b/i, key);
    // nor an imperative of the headline's card anywhere in the reason
    for (const text of [en, de, TABLE[`hint.reason.${key}`].en, TABLE[`hint.reason.${key}`].de]) assert.doesNotMatch(text, /\b(lead|play) \{card\}|\{card\} (anspielen|spielen)/i, key);
  }
});

test("a reason that can stand as the sentence fits the budget with the headline", () => {
  for (const [lang, limit] of [["en", 25], ["de", 30]]) {
    const head = wordCount(TABLE["hint.play"][lang]);
    for (const key of Object.keys(TABLE)) {
      if (!key.startsWith("hint.reason.") || key.includes(".bid-") || /announce|extension|not-your/.test(key)) continue;
      const n = head + wordCount(TABLE[key][lang]);
      assert.ok(n <= limit, `${lang} ${key}: ${n} words`);
    }
  }
});

test("the sentence for a yes/no question opens with its verdict", () => {
  const verdict = { en: /^(Yes|No|Not)\b/, de: /^(Ja|Nein|Noch nicht|Nicht)\b/ };
  for (const q of ["ask.keep", "ask.trumps", "ask.worth"]) {
    for (const s of budgetPairs[q]) {
      if (s.startsWith("so.")) continue;
      for (const lang of ["en", "de"]) assert.match(TABLE[`hint.${s}`][lang], verdict[lang], `${lang} ${s}`);
    }
  }
});

// ---- the Why? (docs/HINTS.md 3.1): the intro, the contrast, the facts, the rule, the odds; at most 90 words ----

test("the Why? lines: the intro with the number of deals, then the contrast, the facts, the rule and the odds", () => {
  const r = { ...mondCase, rule: "cheapest", odds: { alt: "T3", better: 41, worlds: 50, gain_thirds: 4 }, why: { ...mondCase.why, contrast: { key: "vs.follow", params: { card: "T21", alt: "T3", w: 50, b: 41, a: 7 } }, odds: { alt: "T3", better: 41, worlds: 50, gain_thirds: 4 } } };
  const w = hintLines(r, words("en")).why;
  assert.deepEqual(w.lines.map((l) => l.text), [
    "I deal his hidden cards at random 50 times and play each out.",
    "Instead of tarock 3: over the rest of the deal Mond was better in 41 of 50 deals, tarock 3 in 7, the rest equal.",
    "The Sküs has not been played.",
    "8 tarocks are still out; you hold 13.",
    "Rule: do not spend a king where a small card serves.",
    "Against tarock 3: ahead in 41 of 50 sampled deals, by 1⅓ card points on average.",
  ]);
  assert.deepEqual(w.lines.map((l) => l.anchor ?? null), [null, null, "row:trump", "line1", null, null]);
  assert.match(hintLines(r, words("de")).why.lines[0].text, /^Ich teile seine verdeckten Karten 50-mal/);
  // no sample, no intro
  assert.equal(hintLines(mondCase, words("en")).why.lines[0].text, "The Sküs has not been played.");
});

test("the Why? never runs over 90 words: the last lines are left out first", () => {
  const long = { ...coverCase, rule: "finesse", odds: { alt: "T19", better: 41, worlds: 50, gain_thirds: 4 }, why: { ...coverCase.why, trumped: { key: "vs.lead-trumped", params: { card: "Jd", alt: "T19", worlds: 1000, trumped: 300, alt_trumped: 0 } }, facts: [{ key: "fact.tarocks-out", params: { out: 16, mine: 6 }, anchor: "line1" }, { key: "fact.master", params: { card: "T19" }, anchor: "line2" }], odds: { alt: "T19", better: 41, worlds: 50, gain_thirds: 4 } } };
  for (const lang of ["en", "de"]) {
    const w = hintLines(long, words(lang)).why;
    const total = w.lines.reduce((n, l) => n + wordCount(l.text), 0);
    assert.ok(total <= 90, `${lang} ${total}`);
    assert.ok(w.lines.length >= 4, lang);
    assert.match(w.lines[1].text, /^(Instead of|Statt)/);
  }
  // the whole budget is cut, not a line: a tiny budget keeps the first lines only
  const tiny = hintLines(long, { ...words("en"), whyWords: 30 }).why;
  assert.ok(tiny.lines.reduce((n, l) => n + wordCount(l.text), 0) <= 30);
});

// ---- plain words (HINTS 7): no jargon of price in a hint ----

test("no hint string says cheap, dear, costs or billig, teuer, kostet, unless HINTS 7 lists the term", () => {
  const terms = readFileSync(new URL("../../docs/HINTS.md", import.meta.url), "utf8").split("## 7.")[1].split("## 8.")[0];
  const listed = (w) => new RegExp(`^\\| [^|]*\\b${w}\\b[^|]* \\| [^|]+ \\|$`, "im").test(terms.split("Not used:")[0]);
  const bad = { en: /\b(cheap|cheaper|cheapest|dear|dearer|costs?|costing)\b/i, de: /billig|teuer|kostet|kostete|kosten/i };
  for (const [key, v] of Object.entries(TABLE)) {
    if (!key.startsWith("hint.")) continue;
    for (const lang of ["en", "de"]) {
      const m = v[lang].match(bad[lang]);
      if (m) assert.ok(listed(m[0].toLowerCase()), `${lang} ${key}: "${m[0]}" is not a listed term`);
    }
  }
});

test("the count's unit in English is the card point", () => {
  assert.deepEqual(TABLE["hint.unit.one"], { en: "card point", de: "Auge" });
  assert.deepEqual(TABLE["hint.unit.other"], { en: "card points", de: "Augen" });
  for (const [key, v] of Object.entries(TABLE)) if (key.startsWith("hint.")) assert.doesNotMatch(v.en, /(?<!card )\bpoints?\b/, key);
});

// ---- a rule of M5r in the sentence slot, its measure in the Why? ----

const m5rCase = {
  ...coverCase,
  question: "ask.cheap",
  so: { key: "so.m5r", params: {}, text: { en: "With few tarocks out, play the card worth least.", de: "Mit wenig Tarock draußen: die wertniedrigste Karte spielen." } },
  why: { ...coverCase.why, rule: { id: "cheap/outcome:tu4", p: 85, n: 3586, late: true } },
};

test("an M5r sentence is the engine's own words in the language shown", () => {
  assert.equal(hintLines(m5rCase, words("en")).text, "With few tarocks out, play the card worth least.");
  assert.equal(hintLines(m5rCase, words("de")).text, "Mit wenig Tarock draußen: die wertniedrigste Karte spielen.");
});

test("the Why? tells the measure of the rule, and that it describes late play when it does", () => {
  const en = hintLines(m5rCase, words("en")).why.lines.map((l) => l.text);
  assert.ok(en.includes("It is a rule measured against the strongest player: it plays so in 85% of 3,586 positions like this."), en.join("|"));
  assert.ok(en.includes("The rule describes play late in the deal."));
  const de = hintLines(m5rCase, words("de")).why.lines.map((l) => l.text);
  assert.ok(de.includes("Eine Regel, gegen den stärksten Spieler gemessen: er spielt so in 85 % von 3.586 solchen Stellungen."), de.join("|"));
  const early = hintLines({ ...m5rCase, why: { ...m5rCase.why, rule: { id: "worth.poor/no-king:ml7", p: 84, n: 1156, late: false } } }, words("en")).why.lines.map((l) => l.text);
  assert.ok(early.some((l) => l.includes("84% of 1,156")));
  assert.ok(!early.includes("The rule describes play late in the deal."));
});
