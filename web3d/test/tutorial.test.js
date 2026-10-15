// The tutorial pages: both files parse with the kit's parser, are parallel,
// each page has its title key, and the pages are short and plain.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { tutorialPages } from "../src/tutor.js";
import { KEYS, STRINGS, COMBOS, allPages, lines, page, pages, text, words } from "./support/pages.js";

const read = (p) => readFileSync(new URL(p, import.meta.url), "utf8");

// A newcomer reads about 200 words a minute; a page is read in under a minute
// and a page is looked at between moves, so the cap is 150 words, headings
// included. A line is at most 25 words (TABLE3D 6.2.1).
const PAGE_WORDS = 150;
const LINE_WORDS = 25;
// One page is deliberately over: `scoring-full`, the long page, which never
// opens by itself and is read at leisure (TABLE3D 6.2.4). The pop-up that holds
// the table of what every card is worth, `thirds`, fits the ordinary cap.
const OVER_WORDS = { "scoring-full": 1500 };
const OVER_LINE = { "scoring-full": 45 };

test("the spec lists fourteen page keys: thirteen that fall due, and the long scoring page that does not", () => {
  assert.equal(KEYS.length, 14);
  assert.equal(KEYS[0], "packets");
  assert.equal(KEYS[12], "extension");
  assert.equal(KEYS[13], "scoring-full");
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

  test(`${lang}: no page exceeds ${PAGE_WORDS} words (the named exceptions their own cap), no line ${LINE_WORDS} (the long page ${OVER_LINE["scoring-full"]}), under every setting`, () => {
    for (const p of allPages(lang)) {
      const ls = lines(p);
      const total = words(p.title) + ls.reduce((n, l) => n + words(l), 0);
      assert.ok(total <= (OVER_WORDS[p.key] ?? PAGE_WORDS), `${p.key}: ${total} words`);
      for (const l of ls) assert.ok(words(l) <= (OVER_LINE[p.key] ?? LINE_WORDS), `${p.key}: ${words(l)} words: ${l}`);
    }
  });

  test(`${lang}: the exceptions are used (a cap nothing needs is removed)`, () => {
    for (const key of Object.keys(OVER_WORDS)) {
      const longest = Math.max(...allPages(lang).filter((p) => p.key === key).map((p) => words(p.title) + lines(p).reduce((n, l) => n + words(l), 0)));
      assert.ok(longest > PAGE_WORDS, `${key} fits the ordinary cap: ${longest}`);
    }
    assert.ok(Math.max(...lines(page(lang, "scoring-full")).map(words)) > LINE_WORDS);
  });

  test(`${lang}: no jargon a newcomer would not know`, () => {
    const JARGON = ["Stichzwang", "Bummerl", "Valat", "Mondfang", "Stecher", "Impassiren", "finesse", "ruff", "void", "Kontra", "Hinterhand", "Vorhand"];
    const said = allPages(lang).filter((p) => p.key !== "scoring-full").flatMap((p) => [p.title, ...lines(p)]).join("\n");
    assert.deepEqual(JARGON.filter((j) => new RegExp(`\\b${j}`, "i").test(said)), []);
    // the long page names the Valat (RULES 10), once, and nothing else of the list
    const long = allPages(lang).filter((p) => p.key === "scoring-full").flatMap((p) => lines(p)).join("\n");
    assert.deepEqual(JARGON.filter((j) => j !== "Valat").filter((j) => new RegExp(`\\b${j}`, "i").test(long)), []);
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
  const names = { en: ["I take it up", "Pass", "Announce Trull", "Announce four kings", "Slate", "One more Tour", "Enough"], de: ["Ich nehme auf", "Weiter", "Troule ansagen", "Vier Könige ansagen", "Tafel", "Noch eine Tour", "Genug"] };
  for (const lang of ["en", "de"]) for (const n of names[lang]) assert.ok(bold(lang).has(n), `${lang}: ${n}`);
  const label = { "I take it up": "ui.take", Pass: "ui.pass", "Announce Trull": "ui.announce.trull", "Announce four kings": "ui.announce.kings", Slate: "ui.menu.slate", "One more Tour": "ui.ext.yes", Enough: "ui.ext.no" };
  for (const [en, key] of Object.entries(label)) assert.equal(STRINGS[key].en, en);
});

test("the spec and the pages state the count as RULES 8.1 does: value less two-thirds, a group of three less 2", () => {
  const row = read("../../docs/TABLE3D.md").split("| `thirds` |")[1].split("\n")[0];
  assert.match(row, /less two-thirds/);
  assert.doesNotMatch(row, /one-third/);
  const text = (lang) => pages(lang).find((p) => p.key === "thirds").blocks.flatMap((b) => (b.spans ? [b.spans] : b.items)).map((s) => s.map((x) => x.text).join("")).join("\n");
  assert.match(text("en"), /two-thirds[^\n]*sum less 2/);
  assert.match(text("de"), /zwei Drittel[^\n]*Summe weniger 2/);
});

test("the pages hold to RULES 5.4, 6.1, 7.1 and 12.1: trump beats suit, the last card goes to hand, the Pagat is simply paid", () => {
  const page = (lang, key) => pages(lang).find((p) => p.key === key).blocks.flatMap((b) => (b.spans ? [b.spans] : b.items)).map((s) => s.map((x) => x.text).join("")).join("\n");
  assert.match(page("en", "playing"), /tarock beats any suit card/);
  assert.match(page("de", "playing"), /Tarock sticht jede Farbkarte/);
  assert.match(page("en", "the-pull"), /last card of a packet/);
  assert.match(page("de", "the-pull"), /letzte Karte eines Pakets/);
  for (const l of ["en", "de"]) assert.doesNotMatch(page(l, "from-a-packet"), /before it is shown|ehe sie gezeigt/);
  assert.doesNotMatch(page("en", "pagat-ultimo"), /turn up higher/);
});

test("the bidding page tells what happens when both pass, by the setting (RULES 3.4)", () => {
  const page = (lang, setting) => lines(pages(lang, setting).find((p) => p.key === "bidding")).join("\n");
  assert.match(page("en", "play"), /plain game is played/);
  assert.doesNotMatch(page("en", "play"), /thrown in/);
  assert.match(page("en", "throw_in"), /thrown in: nothing is scored/);
  assert.doesNotMatch(page("en", "throw_in"), /plain game/);
  assert.match(page("de", "play"), /einfaches Spiel/);
  assert.match(page("de", "throw_in"), /eingeworfen: nichts wird gezählt/);
  assert.doesNotMatch(page("de", "throw_in"), /einfaches Spiel/);
});

test("every combination of the settings has the same shape in both languages", () => {
  const shape = (p) => p.blocks.map((b) => (b.items ? b.type + b.items.length : b.type)).join(",");
  for (const c of COMBOS) {
    const en = pages("en", c.both_pass, c);
    const de = pages("de", c.both_pass, c);
    en.forEach((p, i) => assert.equal(shape(de[i]), shape(p), `${JSON.stringify(c)} ${p.key}`));
  }
});

test("a variant block is dropped, whole, when its setting is not the game's", () => {
  const md = "# P\n\n## H\n\n- one\n<!-- if both_pass=play -->\n- plain\n<!-- end -->\n<!-- if both_pass=throw_in -->\n- thrown\n<!-- end -->\n- last\n";
  const items = (setting) => tutorialPages(md, ["p"], { both_pass: setting })[0].blocks[1].items.map((s) => s[0].text);
  assert.deepEqual(items("play"), ["one", "plain", "last"]);
  assert.deepEqual(items("throw_in"), ["one", "thrown", "last"]);
});

test("a block may name several settings, and is kept only when all of them hold", () => {
  const md = "# P\n\n## H\n\n<!-- if values=2/3/4 both_pass=play -->\n- both\n<!-- end -->\n<!-- if values=2/3/4 -->\n- values only\n<!-- end -->\n- last\n";
  const items = (settings) => tutorialPages(md, ["p"], settings)[0].blocks[1].items.map((x) => x[0].text);
  assert.deepEqual(items({ values: "2/3/4", both_pass: "play" }), ["both", "values only", "last"]);
  assert.deepEqual(items({ values: "2/3/4", both_pass: "throw_in" }), ["values only", "last"]);
  assert.deepEqual(items({ values: "1/2/3", both_pass: "play" }), ["last"]);
});
