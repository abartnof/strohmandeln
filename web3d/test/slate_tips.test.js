import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createSlate, TIP_KINDS } from "../src/slate.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

// The slate's tooltips (docs/TABLE3D.md 4.7): every figure, heading and mark has
// a title and the same text as its description, from the record.
const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

const SETTINGS = { both_pass: "play", ultimo: 1, values: "2/3/4" };
const start = (settings = SETTINGS, first = 0) => ({ type: "sitting_start", first_dealer: first, touren: 3, settings });
const deal = (n, tour, dealer, multiplier = 1) => ({ type: "deal_start", deal_number: n, tour, dealer, forehand: 1 - dealer, multiplier });
const scored = (result, credits, multiplier, contos) => ({ type: "game_scored", result, credits, multiplier, contos });

// Human at seat 1. Seat 0 is the opponent. Every cause, both ways round.
const SITTING = [
  start(),
  deal(1, 1, 0), scored("declarer_win", [{ seat: 1, cause: "game", points: 3 }, { seat: 1, cause: "trull", points: 1 }], 1, [0, 4]),
  deal(2, 1, 1), scored("declarer_loss", [{ seat: 0, cause: "game", points: 4 }, { seat: 1, cause: "ultimo_won", points: 1 }, { seat: 0, cause: "kings", points: 1 }], 1, [5, 5]),
  { type: "tour_end", tour: 1, contos: [5, 5], balance: 0 },
  deal(3, 2, 0), scored("draw", [{ seat: 1, cause: "trull", points: 1 }], 1, [5, 6]),
  deal(4, 2, 1, 2), scored("simple_win", [{ seat: 0, cause: "game", points: 4 }, { seat: 0, cause: "ultimo_lost", points: 2 }, { seat: 1, cause: "kings", points: 2 }], 2, [11, 8]),
  { type: "tour_end", tour: 2, contos: [11, 8], balance: -3 },
  { type: "extension_decided", accepted: true, touren_added: 3 },
  deal(5, 3, 0), scored("simple_win", [{ seat: 1, cause: "game", points: 2 }], 1, [11, 10]),
  deal(6, 3, 1), scored("declarer_win", [{ seat: 0, cause: "valat", points: 12 }], 1, [23, 10]),
  { type: "tour_end", tour: 3, contos: [23, 10], balance: -13 },
  deal(7, 4, 0),
];
const thrown = [start({ ...SETTINGS, both_pass: "throw_in" }, 1), deal(1, 1, 1, 2), { type: "thrown_in" }];
const ended = (payer) => ({ type: "sitting_end", contos: payer === null ? [4, 4] : [10, 4], balance: payer === null ? 0 : 6, payer, kreuzer: payer === null ? 0 : 12 });

function made(lang, human, events, state) {
  const s = createStrings(TABLE, lang);
  const root = fakeRoot();
  const slate = createSlate(root, { t: s.t, tn: s.tn, onLang: s.onLang });
  slate.update({ human, contos: state, balance: 0, over: false }, events);
  return { root, slate, s };
}
const tips = (root) => root.findAll((e) => e.classList.contains("slate-tip"));
const sets = [
  (lang, human) => made(lang, human, [...SITTING], [23, 10]),
  (lang, human) => made(lang, human, [...SITTING, ended(1)], [10, 4]),
  (lang, human) => made(lang, human, [...SITTING, ended(null)], [4, 4]),
  (lang, human) => made(lang, human, thrown, [0, 0]),
];

test("every kind has a template in both languages, of at most 25 words in English", () => {
  for (const kind of TIP_KINDS) {
    const e = TABLE[`slate.tip.${kind}`];
    assert.ok(e && e.en && e.de, `slate.tip.${kind}`);
    assert.ok(e.en.replace(/\{x2\}/g, " Counted double: 2 × 2.").replace(/\{(pts|total|kr)\}/g, "12 Points").replace(/\{\w+\}/g, "x").split(/\s+/).length <= 25, `${kind}: too long`);
  }
  for (const k of ["slate.tip.pt.one", "slate.tip.pt.other", "slate.tip.doubled"]) assert.ok(TABLE[k]?.en && TABLE[k]?.de, k);
});

test("every figure, heading and mark of a sample sitting carries a filled title and description; all kinds are covered", () => {
  const seen = new Set();
  for (const lang of ["en", "de"]) {
    for (const human of [0, 1]) {
      for (const make of sets) {
        const { root } = make(lang, human);
        for (const el of tips(root)) {
          const title = el.getAttribute("title");
          assert.ok(title && title.length > 10, `empty tip on ${el.className}`);
          assert.ok(!/[{}]/.test(title), `unfilled: ${title}`);
          assert.ok(!/undefined|NaN/.test(title), title);
          assert.equal(el.getAttribute("aria-description"), title);
          const kind = el.getAttribute("data-tip");
          assert.ok(TIP_KINDS.includes(kind), kind);
          seen.add(kind);
        }
      }
    }
  }
  assert.deepEqual(TIP_KINDS.filter((k) => !seen.has(k)), [], "kinds no sample reached");
});

test("every figure on the sheet is a tip element", () => {
  const { root } = sets[1]("en", 1);
  for (const cls of ["slate-credit", "slate-total", "slate-conto", "slate-no", "slate-dealer", "slate-double", "slate-note", "slate-balance", "slate-direction", "slate-col", "slate-tour-no", "slate-extended"]) {
    for (const el of root.byClass(cls)) assert.ok(el.classList.contains("slate-tip"), `${cls} has no tip`);
  }
});

test("a declared win reads from the record: who, what, the value", () => {
  const { root } = sets[0]("en", 1);
  const game = root.byClass("slate-credit")[0];
  assert.equal(game.getAttribute("title"), "Game: you declared and won, so your opponent pays you 3 Points (the declarer-wins value).");
  const trull = root.byClass("slate-credit")[1];
  assert.match(trull.getAttribute("title"), /^Trull: you announced Sküs, Mond and Pagat held together, so your opponent pays you 1 Point\.$/);
});

test("a loss, a simple game, ultimo, a valat and a doubled deal each say their cause", () => {
  const { root } = sets[0]("en", 1);
  const by = (kind) => root.findAll((e) => e.getAttribute("data-tip") === kind).map((e) => e.getAttribute("title"));
  assert.match(by("game.declarer_loss.opp")[0], /you declared and lost, so you pay your opponent 4 Points/);
  assert.match(by("ultimo_won.you")[0], /Pagat/);
  assert.match(by("kings.opp")[0], /four kings/);
  assert.match(by("game.simple_win.opp")[0], /both passed.*your opponent won the simple game.*4 Points.*Counted double: 2 × 2/);
  assert.match(by("ultimo_lost.opp")[0], /you lost the last trick with the Pagat/);
  assert.match(by("valat.opp")[0], /Valat: your opponent took all 27 tricks.*12 Points/);
  assert.match(by("double")[0], /counts double: the last deal played ended in a draw/);
  assert.match(by("note.draw")[0], /^Draw: neither of you reached 36 card points/);
});

test("subtotals, balance and settlement say what they hold", () => {
  const { root } = sets[1]("en", 1);
  const by = (kind) => root.findAll((e) => e.getAttribute("data-tip") === kind).map((e) => e.getAttribute("title"));
  assert.equal(by("tour.you")[0], "Your account at the end of Tour 1: 5 Points credited to you so far, 5 of them in this Tour.");
  assert.match(by("tour.you")[1], /8 Points credited to you so far, 3 of them in this Tour/);
  assert.equal(by("balance.behind")[0], "Difference of the two accounts: your opponent is 6 Points ahead, which is 12 kr. at 2 kreuzer a Point.");
  assert.equal(by("direction.pay")[0], "At 2 kreuzer a Point, you would pay your opponent 12 kr. if the evening ended now.");
  assert.match(by("settle.owed")[0], /6 Points at 2 kreuzer each is 12 kr\./);
  assert.match(by("settle.payer.you")[0], /smaller one, by 6 Points/);
  const live = sets[0]("en", 0).root;
  const ahead = live.findAll((e) => e.getAttribute("data-tip") === "balance.ahead")[0];
  assert.equal(ahead.getAttribute("title"), "Difference of the two accounts: you are 13 Points ahead, which is 26 kr. at 2 kreuzer a Point.");
});

test("a square sitting says nothing is owed", () => {
  const { root } = sets[2]("de", 1);
  assert.ok(root.findAll((e) => e.getAttribute("data-tip") === "settle.square").length === 1);
});

test("a thrown-in deal with a doubling pending says it waits", () => {
  const { root } = sets[3]("en", 1);
  const el = root.findAll((e) => e.getAttribute("data-tip") === "double.thrown")[0];
  assert.match(el.getAttribute("title"), /does not use it up/);
});

test("keyboard: one tab stop; the arrows move through the figures, up and down by line", () => {
  const { root } = sets[0]("en", 1);
  const all = tips(root);
  assert.equal(all.filter((e) => e.getAttribute("tabindex") === "0").length, 1);
  assert.ok(all.every((e) => ["0", "-1"].includes(e.getAttribute("tabindex"))));
  const board = root.byClass("slate-board")[0];
  const at = all[0];
  board.dispatch("focusin", { target: at });
  const ev = board.dispatch("keydown", { key: "ArrowRight", target: at });
  assert.equal(ev.defaultPrevented, true);
  const now = tips(root);
  assert.equal(now.filter((e) => e.getAttribute("tabindex") === "0").length, 1);
  assert.equal(now[1].getAttribute("tabindex"), "0");
  assert.equal(now[1].focused, true);
  // Down jumps to the next line, Up comes back.
  board.dispatch("focusin", { target: now[1] });
  board.dispatch("keydown", { key: "ArrowDown", target: now[1] });
  const down = tips(root).find((e) => e.getAttribute("tabindex") === "0");
  assert.ok(Number(down.getAttribute("data-line")) > Number(now[1].getAttribute("data-line")));
  board.dispatch("focusin", { target: down });
  board.dispatch("keydown", { key: "ArrowUp", target: down });
  assert.equal(tips(root).find((e) => e.getAttribute("tabindex") === "0").getAttribute("data-line"), now[1].getAttribute("data-line"));
});

test("focus and hover show the text in the caption, for touch too", () => {
  const { root } = sets[0]("en", 1);
  const board = root.byClass("slate-board")[0];
  const el = tips(root)[3];
  board.dispatch("focusin", { target: el });
  assert.equal(root.byClass("slate-tip-caption")[0].textContent, el.getAttribute("title"));
  const el2 = tips(root)[5];
  board.dispatch("mouseover", { target: el2 });
  assert.equal(root.byClass("slate-tip-caption")[0].textContent, el2.getAttribute("title"));
});
