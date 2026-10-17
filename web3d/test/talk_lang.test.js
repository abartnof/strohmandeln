// The talk's language: English lines in English, German in German, and a switch
// takes effect on the next line said (docs/TABLE3D.md 5, 7.5).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createTalk } from "../src/talk.js";

const words = JSON.parse(readFileSync(new URL("../words.json", import.meta.url), "utf8"));
const H = 1;
const O = 0;

function seeded(seed) {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

// A bank whose English texts begin "EN:" and German "DE:": the language a line was said in is then plain.
const marked = {
  groups: words.groups,
  texts: Object.fromEntries(Object.entries(words.texts).map(([k, v]) => [k, { en: `EN:${k}`, de: `DE:${k}`, sig: v.sig }])),
};

function sitting(lang, bank = marked) {
  let current = lang;
  const said = [];
  const talk = createTalk({ words: bank, getLang: () => current, rng: seeded(3), say: (l) => said.push(l) });
  const feed = (...events) => events.forEach((e) => talk.onEvent(e, { human: H }));
  return { said, feed, setLang: (l) => (current = l) };
}

const deal = (n) => ({ type: "deal_start", deal_number: n, tour: Math.ceil(n / 2), dealer: O, forehand: H, multiplier: 1 });
const pulled = (seat, card) => ({ type: "card_pulled", seat, packet: 1, card, last: false });
const play = (seat, card) => ({ type: "card_played", seat, card, source: "hand", trick: 1 });
const scored = (seat) => ({ type: "game_scored", result: "declarer_win", credits: [{ seat, cause: "game", points: 1 }], multiplier: 1, contos: [0, 0] });
const moments = (n) => [
  { type: "sitting_start", touren: 3 }, deal(n), pulled(O, "Kh"), pulled(H, "T1"), pulled(O, "T21"), pulled(O, "SK"), pulled(H, "T5"),
  play(O, "T5"), play(H, "T3"), { type: "slow", n: 1 }, { type: "refused" }, { type: "opponent_error" }, scored(O), scored(H),
  { type: "count_start" }, { type: "count_close" },
];

test("in English every line said is the English text", () => {
  const s = sitting("en");
  for (let n = 1; n <= 6; n++) s.feed(...moments(n));
  assert.ok(s.said.length > 20);
  assert.deepEqual(s.said.filter((l) => !l.text.startsWith("EN:")), []);
});

test("a switch to German and back changes the language of the next line only", () => {
  const s = sitting("en");
  s.feed(...moments(1));
  const n1 = s.said.length;
  s.setLang("de");
  s.feed(...moments(2));
  const n2 = s.said.length;
  s.setLang("en");
  s.feed(...moments(3));
  assert.ok(s.said.slice(0, n1).every((l) => l.text.startsWith("EN:")));
  assert.ok(s.said.slice(n1, n2).length && s.said.slice(n1, n2).every((l) => l.text.startsWith("DE:")));
  assert.ok(s.said.slice(n2).length && s.said.slice(n2).every((l) => l.text.startsWith("EN:")));
});

test("each line says its key, so the table can say it in the language of the moment it is heard", () => {
  const s = sitting("en");
  s.feed(...moments(1));
  assert.ok(s.said.every((l) => l.key && marked.texts[l.key]));
});

test("every line has an English text of its own: none empty, none the German", () => {
  for (const [k, v] of Object.entries(words.texts)) {
    assert.ok(v.en && v.en.trim(), `${k}: no English`);
    assert.notEqual(v.en, v.de, `${k}: the English is the German`);
  }
});
