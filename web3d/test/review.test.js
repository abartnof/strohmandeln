// The brief's dialog: the module's bullets (docs/PROTOCOL.md "The tutor") and
// the count put into the words of the current language, with cards named and
// thirds written, in both languages.
import test from "node:test";
import assert from "node:assert/strict";
import { briefBlocks, explainText, tutorWords, words } from "../src/review.js";
import { createStrings, thirds, signed } from "../src/strings.js";
import TABLE from "../strings.json" with { type: "json" };

const fmt = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, cardName: s.romanName, thirds, signed, suitName: (c) => (lang === "en" ? s.suitName(c).toLowerCase() : s.suitName(c)) };
};

const count = { lead: "count", key: "count", params: { mine: 120, theirs: 90 } };
const review = { deal: 3, bullets: [count, { lead: "decided", key: "level", params: {} }] };
const brief = {
  method: "tutor.method",
  bullets: [
    { lead: "tutor.lead.well", key: "tutor.well.met", params: { chances: 3, skill: "off" } },
    { lead: "tutor.lead.next", key: "tutor.rule.cheap", params: { skill: "cheap" } },
    { lead: "tutor.lead.deal", key: "tutor.miss.worth.poor", params: { trick: 9, played: "Qh", better: "4h", thirds: 22 } },
  ],
};

test("the count is written in points and thirds", () => {
  assert.equal(words(count, fmt("en")), "You counted 40 to your opponent's 30; 35⅔ was the line.");
  assert.equal(words({ lead: "count", key: "count", params: { mine: 107, theirs: 103 } }, fmt("en")), "You counted 35⅔ to your opponent's 34⅓; 35⅔ was the line.");
});

test("the net of a sitting is signed", () => {
  const b = { lead: "sitting", key: "sitting-tally", params: { won: 3, lost: 2, drawn: 1, net: -4 } };
  assert.equal(words(b, fmt("en")), "Deals: 3 won, 2 lost, 1 drawn; −4 Points in all.");
});

test("a brief is a title, the count, up to three bullets with a lead each, and the method", () => {
  const r = briefBlocks(brief, review, fmt("en"));
  assert.equal(r.title, "How the deal went");
  assert.equal(r.header, "You counted 40 to your opponent's 30; 35⅔ was the line.");
  assert.deepEqual(r.bullets.map((b) => b.lead), ["Went well:", "Next: the cheapest card.", "This deal:"]);
  assert.deepEqual(r.bullets.map((b) => b.text), [
    "You met all 3 chances: playing the packets off.",
    "If I may: Win with the lowest card that wins; otherwise give the lowest.",
    "Trick 9: your queen of hearts won only 7⅓ points; 4 of hearts would have saved it for a bigger trick.",
  ]);
  assert.match(r.method, /compared afterwards/);
  assert.equal(r.how, "How is this worked out?");
});

test("the same brief in German: cards by their names, the rule after its prefix", () => {
  const r = briefBlocks(brief, review, fmt("de"), { whole: false });
  assert.equal(r.title, "Wie die Partie verlief");
  assert.deepEqual(r.bullets.map((b) => b.lead), ["Gut gelaufen:", "Als Nächstes: die billigste Karte.", "In dieser Partie:"]);
  assert.equal(r.bullets[0].text, "Sie nutzten alle 3 Gelegenheiten: das Abspielen der Pakete.");
  assert.equal(r.bullets[1].text, "Wenn ich mir erlauben darf: Den Stich mit der kleinsten Karte holen, die ihn gewinnt; sonst die kleinste zugeben.");
  assert.match(r.bullets[2].text, /^Im 9\. Stich holte Herz-Dame nur 7⅓ Augen; /);
  assert.equal(r.how, "Wie kommt das zustande?");
});

test("a card, a suit, an honour and two skills are named in the language", () => {
  const e = fmt("en");
  const d = fmt("de");
  const miss = { lead: "tutor.lead.deal-n", key: "tutor.miss.void", params: { deal: 4, trick: 12, played: "Kh", suit: "h" } };
  assert.deepEqual(tutorWords(miss, e), { lead: "Deal 4:", text: "Trick 12: you led king of hearts, though your opponent had already failed to follow hearts." });
  assert.equal(tutorWords(miss, d).lead, "Partie 4:");
  assert.match(tutorWords(miss, d).text, /^Im 12\. Stich spielten Sie Herz-König an, obwohl Ihr Gegner Herz schon nicht bedient hatte\.$/);
  const honour = { lead: "", key: "tutor.miss.announce", params: { card: "T21", honour: "trull" } };
  assert.equal(tutorWords(honour, e).text, "You played Mond without announcing the Trull: a Point lost.");
  assert.equal(tutorWords(honour, d).text, "Sie spielten Mond, ohne die Troule anzusagen: ein Point ging verloren.");
  assert.equal(tutorWords(honour, e).lead, "");
  const two = { lead: "tutor.lead.well", key: "tutor.well.mastered-two", params: { skill: "worth", skill2: "cheap" } };
  assert.equal(tutorWords(two, e).text, "Mastered: judging a trick's worth and the cheapest card.");
  const fell = { lead: "tutor.lead.deal", key: "tutor.miss.off.figure", params: { card: "Qh", king: "Kh", late: 14 } };
  assert.equal(tutorWords(fell, e).text, "Your queen of hearts fell under his king of hearts in trick 14; play it at once next time.");
  const tarock = { lead: "", key: "tutor.miss.run", params: { trick: 3, played: "T12", better: "T5" } };
  assert.match(tutorWords(tarock, e).text, /tarock XII.*tarock V/);
});

test("with the tips off the brief gives what went well, one bullet, and no advice", () => {
  const r = briefBlocks(brief, review, fmt("en"), { tips: false });
  assert.equal(r.bullets.length, 1);
  assert.equal(r.bullets[0].lead, "Went well:");
  const advice = briefBlocks({ bullets: brief.bullets.slice(1), method: "tutor.method" }, review, fmt("en"), { tips: false });
  assert.deepEqual(advice.bullets, []);
  assert.ok(advice.header, "the count is still the header");
});

test("at most three bullets, and the sitting's dialog has the tally for a header", () => {
  const r = briefBlocks({ ...brief, bullets: [...brief.bullets, ...brief.bullets] }, review, fmt("en"));
  assert.equal(r.bullets.length, 3);
  const tally = { bullets: [{ lead: "sitting", key: "sitting-tally", params: { won: 2, lost: 1, drawn: 0, net: 3 } }] };
  const s = briefBlocks(brief, tally, fmt("de"), { whole: true });
  assert.equal(s.title, "Wie der Abend verlief");
  assert.equal(s.header, "Partien: 2 gewonnen, 1 verloren, 0 unentschieden; +3 Points im Ganzen.");
});

test("a brief with nothing to say and no count is null; a count alone is a dialog", () => {
  assert.equal(briefBlocks(null, null, fmt("en")), null);
  assert.equal(briefBlocks({ bullets: [] }, { bullets: [] }, fmt("en")), null);
  const only = briefBlocks(null, review, fmt("en"));
  assert.deepEqual(only.bullets, []);
  assert.ok(only.header);
});

test("every sentence of a brief is a key the table has, in both languages", () => {
  for (const b of brief.bullets) for (const key of [b.lead, b.key]) for (const lang of ["en", "de"]) assert.ok(TABLE[key]?.[lang], key);
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

test("the review and the brief are gentle: no bullet names a fault or a person", () => {
  const bad = /\b(stupid|foolish|blunder|mistake|error|wrong|careless|bad player|fool)/i;
  for (const [key, v] of Object.entries(TABLE)) {
    if (!key.startsWith("review.say.") && !key.startsWith("explain.") && !key.startsWith("tutor.")) continue;
    assert.ok(!bad.test(v.en), key);
  }
});

test("explanation names tarocks by Roman numeral; the plain cardName keeps Arabic", () => {
  const x = createStrings(TABLE, "en");
  const f = { t: x.t, cardName: x.romanName, thirds, signed, suitName: x.suitName };
  assert.equal(words({ lead: "decided", key: "trick-lost-rich", params: { trick: 4, mine: "T12", theirs: "T19", thirds: 23 } }, f), "Trick 4 was the rich one you lost: tarock XII against tarock XIX, 7⅔ between them.");
  assert.equal(explainText({ lead: "explain", key: "explain-won-you", params: { winner: "T21", loser: "T4" } }, f), "Your trick: Mond beat tarock IIII.");
  assert.equal(x.cardName("T12"), "tarock 12");
});
