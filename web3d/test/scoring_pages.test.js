// The scoring pages (`thirds`, `scoring`, `scoring-full`, and the values on
// `bidding`): every number they print is the engine's, or the table's counting
// code's, per RULES 1.7, 8, 9, 11, 12, 14 and 15. The worked deal and the
// worked evening are also pinned in the core tests
// (the_pages_worked_deal_pays_game_trull_and_a_lost_ultimo,
// the_pages_worked_evening_settles_3_points_as_6_kreuzer).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { PAGE_KEYS } from "../src/tutor.js";
import { valueOf, thirdsOf } from "../src/cardinfo.js";
import { COURT_LABELS } from "../src/tarockfaces.js";
import { countLines, thirds } from "../src/counting.js";
import { playSitting, passer, taker } from "./sim.js";
import { COMBOS, STRINGS, VALUES, KEYS, page, pages, spansOf, text } from "./support/pages.js";

// The 54 cards by code (RULES 1.2).
const PACK = [
  ...["h", "d"].flatMap((su) => ["K", "Q", "C", "J", "1", "2", "3", "4"].map((r) => r + su)),
  ...["c", "s"].flatMap((su) => ["K", "Q", "C", "J", "10", "9", "8", "7"].map((r) => r + su)),
  ...Array.from({ length: 21 }, (_, i) => `T${i + 1}`),
  "SK",
];
const LANGS = ["en", "de"];
// Every card's value as the Rust engine has it: the core test rules_1_7_every_card_value_is_the_tables
// holds this file to Card::value() for all 54 codes, so the pages are held to the engine through it.
const ENGINE_VALUES = JSON.parse(readFileSync(new URL("./fixtures/card-values.json", import.meta.url), "utf8"));

test("the table's card values (cardinfo.js) are the engine's for all 54 cards", () => {
  assert.deepEqual(Object.keys(ENGINE_VALUES).sort(), [...PACK].sort());
  for (const c of PACK) {
    assert.equal(valueOf(c), ENGINE_VALUES[c], c);
    assert.equal(thirdsOf(c), 3 * ENGINE_VALUES[c] - 2, c);
  }
});

test("the pack the tests count is 54 cards, worth 106 before the count in threes and 70 after it", () => {
  assert.equal(PACK.length, 54);
  assert.equal(new Set(PACK).size, 54);
  assert.equal(PACK.reduce((n, c) => n + valueOf(c), 0), 106);
  assert.equal(PACK.reduce((n, c) => n + thirdsOf(c), 0), 210);
  assert.equal(106 - (54 / 3) * 2, 70);
  for (const lang of LANGS) assert.match(text(lang, "scoring-full"), /\b106\b/);
});

// The value table is the list whose lines begin with a bold value.
const valueRows = (lang, key) => {
  const ul = page(lang, key).blocks.find((b) => b.items && b.items[0][0]?.bold && /^\d+$/.test(b.items[0][0].text));
  assert.ok(ul, `${lang} ${key}: no list of values`);
  return ul.items.map((spans) => ({ value: Number(spans[0].text), line: spans.map((x) => x.text).join("") }));
};
const NAMES = { SK: "Sküs", T21: "Mond", T1: "Pagat" };
const mentions = (line, label) => new RegExp(`(^|[^A-Za-zü])${label}([^A-Za-zü]|$)`).test(line);

for (const lang of LANGS) {
  for (const key of ["thirds", "scoring-full"]) {
    test(`${lang} ${key}: the table of card values is the engine's, for all 54 cards (RULES 1.7)`, () => {
      const rows = valueRows(lang, key);
      assert.deepEqual(rows.map((r) => r.value), [5, 4, 3, 2, 1]);
      const line = (v) => rows.find((r) => r.value === v).line;
      const labels = COURT_LABELS[lang];
      const named = { 5: ["Kh", "Kd", "Kc", "Ks", "SK", "T21", "T1"], 4: ["Qh", "Qd", "Qc", "Qs"], 3: ["Ch", "Cd", "Cc", "Cs"], 2: ["Jh", "Jd", "Jc", "Js"] };
      for (const [v, codes] of Object.entries(named)) for (const c of codes) assert.equal(ENGINE_VALUES[c], Number(v), c);
      // no other card is worth more than 1: the five rows are all the table there is
      assert.deepEqual(PACK.filter((c) => ENGINE_VALUES[c] > 1).sort(), Object.values(named).flat().sort());
      // the letters the faces show, on the row of the value their cards have
      for (const [v, label] of [[5, labels.K], [4, labels.Q], [3, labels.C], [2, labels.J]]) assert.ok(mentions(line(v), label), `${v}: ${label}`);
      for (const c of ["SK", "T21", "T1"]) assert.ok(line(5).includes(NAMES[c]), `${c}`);
      assert.ok(mentions(line(5), "XXI") && mentions(line(5), "I"));
      // every other card is worth 1, and the row says tarocks II to XX
      const rest = PACK.filter((c) => !Object.values(named).flat().includes(c));
      assert.equal(rest.length, 54 - 7 - 12);
      assert.ok(rest.every((c) => ENGINE_VALUES[c] === 1));
      assert.ok(mentions(line(1), "II") && mentions(line(1), "XX"));
      for (const n of [2, 20]) assert.equal(valueOf(`T${n}`), 1);
    });
  }
}

test("the bidding page gives the same values with the court letters as the faces show them, and points to the table", () => {
  for (const lang of LANGS) {
    const t = text(lang, "bidding");
    const labels = COURT_LABELS[lang];
    for (const [letter, v] of [[labels.Q, 4], [labels.C, 3], [labels.J, 2]]) assert.match(t, new RegExp(`\\(${letter}\\) ${v}`), `${lang} ${letter}`);
    assert.match(t, new RegExp(`\\(${labels.K}\\)[^\\n]*5`));
    assert.match(t, /\b70\b/);
    assert.match(t, /\b36\b/);
    assert.deepEqual(spansOf(page(lang, "bidding")).filter((x) => x.page).map((x) => x.page), ["thirds"]);
  }
});

test("the thirds and scoring pop-ups link to the long page, and the long page to none; every link names a page", () => {
  for (const lang of LANGS) {
    const links = (key) => spansOf(page(lang, key)).filter((x) => x.page).map((x) => x.page);
    assert.deepEqual(links("thirds"), ["scoring-full"]);
    assert.deepEqual(links("scoring"), ["scoring-full"]);
    assert.deepEqual(links("scoring-full"), []);
    for (const p of pages(lang)) for (const k of links(p.key)) assert.ok(PAGE_KEYS.includes(k), `${p.key} -> ${k}`);
  }
});

test("the count: the worked groups, each card's own count, and the tally that runs while the tricks are played", () => {
  for (const lang of LANGS) {
    // K, J and 7: 5 + 2 + 1 = 8, less 2 = 6 -- three cards of the table's counting code
    assert.equal(["Kh", "Js", "7c"].reduce((n, c) => n + thirdsOf(c), 0), 18);
    assert.equal(thirds(18), "6");
    assert.match(text(lang, "thirds"), /5 \+ 2 \+ 1 = 8, [^\n]*\b2 = 6/);
    assert.match(text(lang, "scoring-full"), /5 \+ 2 \+ 1 = 8, [^\n]*\b2 = 6/);
    // three low cards: 3 less 2 is 1; a last pair K, J: 7 less 1⅓ is 5⅔
    assert.equal(thirds(3 * thirdsOf("4h")), "1");
    assert.equal(thirds(thirdsOf("Kh") + thirdsOf("Js")), "5⅔");
    assert.match(text(lang, "scoring-full"), /5⅔/);
    for (const [c, shown] of [["Kh", "4⅓"], ["Qh", "3⅓"], ["Ch", "2⅓"], ["Jh", "1⅓"], ["7c", "⅓"]]) {
      assert.equal(thirds(thirdsOf(c)), shown);
      assert.ok(text(lang, "scoring-full").includes(shown), `${lang} ${shown}`);
    }
    assert.match(text(lang, "thirds"), /runs as you take tricks|läuft mit, während Sie Stiche machen/);
  }
});

test("36 of 70: 107 thirds is 35⅔, which is nearer 36 than 35; 106 is not; the other take has 34⅓", () => {
  assert.equal(thirds(107), "35⅔");
  assert.equal(thirds(106), "35⅓");
  assert.ok(Math.round(107 / 3) === 36 && Math.round(106 / 3) === 35);
  assert.equal(thirds(210 - 107), "34⅓");
  for (const lang of LANGS) {
    const t = text(lang, "scoring-full");
    for (const x of ["35⅔", "35⅓", "34⅓", "35 : 35"]) assert.ok(t.includes(x), `${lang} ${x}`);
  }
});

test("the worked deal: 28 cards worth 55 count 36⅓, the other 26 worth 51 count 33⅔, and the takes make 70", () => {
  // a real split of the pack: a take of 28 cards whose values add up to 55
  const take = [];
  let sum = 0;
  for (const c of [...PACK].sort((a, b) => valueOf(b) - valueOf(a))) {
    if (take.length < 28 && sum + valueOf(c) + (27 - take.length) <= 55) {
      take.push(c);
      sum += valueOf(c);
    }
  }
  assert.equal(take.length, 28);
  assert.equal(sum, 55);
  const rest = PACK.filter((c) => !take.includes(c));
  assert.equal(rest.length, 26);
  const groups = countLines([take, rest], 0);
  const total = (side) => groups.filter((l) => l.side === side).reduce((n, l) => n + l.thirds, 0);
  assert.equal(total("you"), 109);
  assert.equal(total("opp"), 101);
  assert.equal(total("you") + total("opp"), 210);
  assert.equal(rest.reduce((n, c) => n + valueOf(c), 0), 51);
  assert.equal(thirds(109), "36⅓");
  assert.equal(thirds(101), "33⅔");
  assert.equal(thirds(2 * 28), "18⅔"); // two-thirds off each of 28 cards
  assert.equal(thirds(2 * 26), "17⅓");
  for (const lang of LANGS) {
    const t = text(lang, "scoring-full");
    for (const x of ["28", "55", "18⅔", "36⅓", "26", "51", "17⅓", "33⅔"]) assert.ok(t.includes(x), `${lang} ${x}`);
  }
});

for (const lang of LANGS) {
  test(`${lang}: the game's value on the pages is what the engine pays, for each values setting (RULES 9.1, 9.2)`, async () => {
    const WINS = { en: /wins gets (\d+) Points?(?:, however many card points they took)?; one who loses pays (\d+)/, de: /gewinnt, erhält (\d+) Points?; wer verliert, zahlt (\d+)/ }[lang];
    const PLAIN = { en: /pays its winner (\d+)/, de: /seinem Sieger (\d+)/ }[lang];
    for (const [values, [wins, loses, simple]] of Object.entries(VALUES)) {
      // what the engine paid for the game, in sittings played by a declarer and by two passers
      const paid = {};
      for (const [policy, seeds] of [[taker, [1, 2, 3, 4, 5, 6]], [passer, [1, 2]]]) {
        for (const seed of seeds) {
          const { replies } = await playSitting({ seed, touren: 3, human: 1, level: 1, settings: { values } }, { policy: policy(seed), limit: 400 });
          for (const r of replies)
            for (const e of r.events) {
              const g = e.type === "game_scored" && e.multiplier === 1 ? e.credits.find((c) => c.cause === "game") : null;
              if (g) paid[e.result] = g.points;
            }
        }
      }
      assert.deepEqual([paid.declarer_win, paid.declarer_loss, paid.simple_win], [wins, loses, simple], `engine ${values}`);
      for (const key of ["bidding", "scoring", "scoring-full"]) {
        const t = text(lang, key, { values });
        const m = WINS.exec(t);
        assert.ok(m, `${lang} ${key} ${values}: no value line`);
        assert.deepEqual([Number(m[1]), Number(m[2])], [wins, loses], `${key} ${values}`);
        if (key !== "bidding") assert.equal(Number(PLAIN.exec(t)?.[1]), simple, `${key} ${values} plain game`);
        assert.doesNotMatch(text(lang, key, { values, both_pass: "throw_in" }), PLAIN, `${key}: no plain game when both pass is thrown in`);
      }
    }
  });
}

test("the Pagat ultimo is worth the setting (RULES 12.3), on the scoring pages", () => {
  const IS = { en: /ultimo, won or lost, is worth (\d+) Points?/, de: /ultimo ist, gewonnen wie verloren, (\d+) Points?/ };
  for (const lang of LANGS)
    for (const ultimo of [1, 2])
      for (const key of ["scoring", "scoring-full"]) assert.equal(Number(IS[lang].exec(text(lang, key, { ultimo }))?.[1]), ultimo, `${lang} ${key} ${ultimo}`);
});

test("the worked deal's slate lines are the ones the engine pays, for each setting (core test the_pages_worked_deal...)", () => {
  for (const lang of LANGS)
    for (const c of COMBOS) {
      const [wins] = VALUES[c.values];
      const entry = (cause, n) => STRINGS["slate.entry"][lang].replace("{cause}", STRINGS[`slate.cause.${cause}`][lang]).replace("{n}", n);
      const t = text(lang, "scoring-full", c);
      for (const x of [entry("game", wins), entry("trull", 1), entry("ultimo_lost", c.ultimo)]) assert.ok(t.includes(x), `${lang} ${JSON.stringify(c)}: ${x}`);
    }
});

test("the worked evening: 12 and 9 is a difference of 3 Points, 6 kreuzer at 2 a Point (core test the_pages_worked_evening...)", () => {
  assert.equal((12 - 9) * 2, 6);
  for (const lang of LANGS) {
    const t = text(lang, "scoring-full");
    // the sentences, with the numbers the core test settles: accounts [12, 9], balance 3, 6 kreuzer, the smaller account pays
    const example = {
      en: /your account reads 12 and your opponent's 9\. The difference is 3 Points, so 6 kreuzer, and your opponent pays you\./,
      de: /Ihr Conto auf 12, das Ihres Gegners auf 9\. Der Unterschied ist 3 Points, also 6 Kreuzer, und Ihr Gegner zahlt Ihnen\./,
    }[lang];
    assert.match(t, example);
    assert.match(t, lang === "en" ? /settled at 2 kreuzer a Point/ : /zu 2 Kreuzer der Point beglichen/);
  }
});

test("a draw and the double (RULES 14) are said under play and absent under throw-in; the Valat pays 12 (RULES 10)", () => {
  assert.match(text("en", "scoring-full", { both_pass: "play" }), /counts double/);
  assert.doesNotMatch(text("en", "scoring-full", { both_pass: "throw_in" }), /counts double/);
  assert.match(text("de", "scoring-full", { both_pass: "play" }), /zählt doppelt/);
  assert.doesNotMatch(text("de", "scoring-full", { both_pass: "throw_in" }), /zählt doppelt/);
  // 12 Points from the other player, in place of the game's value (core test rules_10_1_and_10_2_valat_is_12_and_replaces_the_game_value_only)
  assert.match(text("en", "scoring-full"), /Valat\. The other player pays 12 Points in place of the game's value, and honours and ultimo are paid as well\./);
  assert.match(text("de", "scoring-full"), /Valat\. Der andere Spieler zahlt 12 Points anstelle des Wertes des Spiels, und Honneurs und Ultimo werden zusätzlich bezahlt\./);
});

test("the long page is titled by its string and listed in the spec's table", () => {
  assert.equal(STRINGS["tutor.page.scoring-full"].en, "Scoring in full");
  assert.equal(STRINGS["tutor.page.scoring-full"].de, "Die Abrechnung im Ganzen");
  assert.ok(KEYS.includes("scoring-full"));
});

test("the long page names the HUD's row as the HUD does, and the count's unit is never a Point (payment only)", () => {
  for (const lang of LANGS) {
    const label = STRINGS["hud.bonus"][lang];
    const t = text(lang, "scoring-full");
    assert.ok(t.includes(`"${label}"`) || t.includes(`„${label}“`), `${lang}: the row label ${label}`);
  }
  for (const key of ["bidding", "thirds", "scoring", "scoring-full"])
    for (const lang of LANGS) assert.doesNotMatch(text(lang, key), /\b(36|35⅔|70|106) Points\b/, `${lang} ${key}`);
});

test("the long page's dialog keeps its Close button in view (a sticky action bar), and the link buttons have a hit area", () => {
  const css = readFileSync(new URL("../src/tutor.css", import.meta.url), "utf8");
  assert.match(css, /\.tutor-page\[data-page="scoring-full"\]\) \.actions \{[^}]*position: sticky/);
  assert.match(css, /\.tutor-link \{[^}]*padding: [1-9]\d*px/);
});
