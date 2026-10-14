// The tutorial pages: both files parse with the kit's parser, are parallel,
// each page has its title key, and the pages are short and plain.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { parseTutorial } from "../../kit/js/tutorial.js";

const read = (p) => readFileSync(new URL(p, import.meta.url), "utf8");
const STRINGS = JSON.parse(read("../strings.json"));
// The page keys are the table of docs/TABLE3D.md 6.2.2, in its order.
const KEYS = [...read("../../docs/TABLE3D.md").split("### 6.2 Tutorial pages")[1].split("### 6.3")[0].matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
const FILES = { en: "../tutorial.en.md", de: "../tutorial.md" };
const pages = (lang) => parseTutorial(read(FILES[lang]), KEYS);
const lines = (page) => page.blocks.flatMap((b) => (b.spans ? [b.spans] : b.items)).map((spans) => spans.map((x) => x.text).join(""));
const words = (s) => s.split(/\s+/).filter(Boolean).length;

// A newcomer reads about 200 words a minute; a page is read in under a minute
// and a page is looked at between moves, so the cap is 150 words, headings
// included. A line is at most 25 words (TABLE3D 6.2.1).
const PAGE_WORDS = 150;
const LINE_WORDS = 25;

test("the spec lists eleven page keys", () => {
  assert.equal(KEYS.length, 11);
  assert.equal(KEYS[0], "packets");
  assert.equal(KEYS[10], "extension");
});

for (const lang of ["en", "de"]) {
  test(`${lang}: every key has a page, in order, titled by its tutor.page.* string`, () => {
    const ps = pages(lang);
    assert.deepEqual(ps.map((p) => p.key), KEYS);
    for (const p of ps) {
      assert.equal(p.title, STRINGS[`tutor.page.${p.key}`][lang], p.key);
      assert.ok(p.blocks.length > 1, p.key);
    }
  });

  test(`${lang}: no page exceeds ${PAGE_WORDS} words, no line ${LINE_WORDS}`, () => {
    for (const p of pages(lang)) {
      const ls = lines(p);
      const total = words(p.title) + ls.reduce((n, l) => n + words(l), 0);
      assert.ok(total <= PAGE_WORDS, `${p.key}: ${total} words`);
      for (const l of ls) assert.ok(words(l) <= LINE_WORDS, `${p.key}: ${words(l)} words: ${l}`);
    }
  });

  test(`${lang}: no jargon a newcomer would not know`, () => {
    const JARGON = ["Stichzwang", "Bummerl", "Valat", "Mondfang", "Stecher", "Impassiren", "finesse", "ruff", "void", "Kontra", "Hinterhand", "Vorhand"];
    const text = pages(lang).flatMap((p) => [p.title, ...lines(p)]).join("\n");
    assert.deepEqual(JARGON.filter((j) => new RegExp(`\\b${j}`, "i").test(text)), []);
  });
}

test("the German is the formal Sie, and no du", () => {
  const text = pages("de").flatMap(lines).join("\n");
  assert.ok(!/\b(du|dein|deine|dir|dich)\b/i.test(text));
  assert.ok(/\bSie\b/.test(text));
});

test("the two languages are parallel: same pages, same shape of blocks", () => {
  const shape = (p) => p.blocks.map((b) => (b.items ? `${b.type}${b.items.length}` : b.type)).join(",");
  const en = pages("en");
  const de = pages("de");
  assert.equal(en.length, de.length);
  en.forEach((p, i) => {
    assert.equal(de[i].key, p.key);
    assert.equal(shape(de[i]), shape(p), p.key);
  });
});

test("the pages name the buttons as the strings do", () => {
  const bold = (lang) => new Set(pages(lang).flatMap((p) => p.blocks.flatMap((b) => b.spans ?? b.items.flat())).filter((x) => x.bold).map((x) => x.text));
  const names = { en: ["I take it up", "Pass", "Play", "Announce Trull", "Announce four kings", "Slate", "One more Tour", "Enough"], de: ["Ich nehme auf", "Weiter", "Ausspielen", "Troule ansagen", "Vier Könige ansagen", "Tafel", "Noch eine Tour", "Genug"] };
  for (const lang of ["en", "de"]) for (const n of names[lang]) assert.ok(bold(lang).has(n), `${lang}: ${n}`);
  const label = { "I take it up": "ui.take", Pass: "ui.pass", Play: "ui.play", "Announce Trull": "ui.announce.trull", "Announce four kings": "ui.announce.kings", Slate: "ui.menu.slate", "One more Tour": "ui.ext.yes", Enough: "ui.ext.no" };
  for (const [en, key] of Object.entries(label)) assert.equal(STRINGS[key].en, en);
});
