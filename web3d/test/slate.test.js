import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createLedger, slateModel, createSlate } from "../src/slate.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

const SETTINGS = { both_pass: "play", ultimo: 1, values: "2/3/4" };
const start = { type: "sitting_start", first_dealer: 0, touren: 3, settings: SETTINGS };
const deal = (n, tour, dealer, multiplier = 1) => ({ type: "deal_start", deal_number: n, tour, dealer, forehand: 1 - dealer, multiplier });
const scored = (result, credits, multiplier, contos) => ({ type: "game_scored", result, credits, multiplier, contos });

// A sitting of one Tour with the human at seat 1.
const tour1 = [
  start,
  deal(1, 1, 0),
  scored("declarer_win", [{ seat: 1, cause: "game", points: 3 }, { seat: 1, cause: "trull", points: 1 }], 1, [0, 4]),
  deal(2, 1, 1),
  scored("declarer_loss", [{ seat: 0, cause: "game", points: 4 }, { seat: 1, cause: "ultimo_won", points: 1 }], 1, [4, 5]),
  { type: "tour_end", tour: 1, contos: [4, 5], balance: -1 },
];

test("a row per deal, with the credits by seat and the Conto after it", () => {
  const l = createLedger();
  l.ingest(tour1);
  const m = slateModel(l.snapshot(), 1);
  assert.equal(m.header.touren, 3);
  assert.deepEqual(m.header.settings, SETTINGS);
  const rows = m.blocks.filter((b) => b.type === "deal");
  assert.equal(rows.length, 2);
  assert.equal(rows[0].number, 1);
  assert.equal(rows[0].status, "played");
  assert.equal(rows[0].dealerIsYou, false);
  assert.deepEqual(rows[0].you.credits, [{ cause: "game", points: 3 }, { cause: "trull", points: 1 }]);
  assert.equal(rows[0].you.total, 4);
  assert.equal(rows[0].opp.total, 0);
  assert.equal(rows[0].youConto, 4);
  assert.equal(rows[0].oppConto, 0);
  assert.equal(rows[1].dealerIsYou, true);
  assert.equal(rows[1].opp.total, 4);
  assert.equal(rows[1].youConto, 5);
  assert.equal(rows[1].oppConto, 4);
});

test("a rule closes the Tour with the Contos", () => {
  const l = createLedger();
  l.ingest(tour1);
  const m = slateModel(l.snapshot(), 1);
  assert.deepEqual(m.blocks.map((b) => b.type), ["deal", "deal", "tour"]);
  const rule = m.blocks[2];
  assert.equal(rule.tour, 1);
  assert.equal(rule.youConto, 5);
  assert.equal(rule.oppConto, 4);
});

test("columns follow the human's seat", () => {
  const l = createLedger();
  l.ingest(tour1);
  const m0 = slateModel(l.snapshot(), 0);
  const rows = m0.blocks.filter((b) => b.type === "deal");
  assert.equal(rows[0].opp.total, 4);
  assert.equal(rows[0].dealerIsYou, true);
  assert.equal(rows[1].dealerIsYou, false);
});

test("the balance and the direction, in points and kreuzer", () => {
  const l = createLedger();
  l.ingest(tour1);
  const m = slateModel(l.snapshot(), 1);
  assert.equal(m.foot.balance, 1);
  assert.equal(m.foot.kreuzer, 2);
  assert.equal(m.foot.direction, "receive");
  const m0 = slateModel(l.snapshot(), 0);
  assert.equal(m0.foot.balance, -1);
  assert.equal(m0.foot.direction, "pay");
  const sq = createLedger();
  sq.ingest([start]);
  assert.equal(slateModel(sq.snapshot(), 1).foot.direction, "square");
});

test("a doubled deal, a draw, a throw-in; a void deal is no row", () => {
  const l = createLedger();
  l.ingest([
    start,
    deal(1, 1, 0),
    { type: "void_deal", seats: [0] },
    deal(1, 1, 0),
    scored("draw", [{ seat: 0, cause: "trull", points: 1 }], 1, [1, 0]),
    deal(2, 1, 1, 2),
    { type: "both_pass", mode: "throw_in" },
    { type: "thrown_in" },
  ]);
  const rows = slateModel(l.snapshot(), 1).blocks.filter((b) => b.type === "deal");
  assert.equal(rows.length, 2);
  assert.equal(rows[0].status, "draw");
  assert.equal(rows[1].status, "thrown_in");
  assert.equal(rows[1].multiplier, 2);
  assert.deepEqual(rows[1].you.credits, []);
  assert.equal(rows[1].youConto, 0, "a throw-in leaves the Contos as they were");
  assert.equal(rows[1].oppConto, 1);
});

test("a deal in progress is a live row until it is scored", () => {
  const l = createLedger();
  l.ingest([start, deal(1, 1, 0)]);
  const rows = slateModel(l.snapshot(), 1).blocks.filter((b) => b.type === "deal");
  assert.equal(rows[0].status, "live");
});

test("an accepted extension adds a double rule and lengthens the sitting", () => {
  const l = createLedger();
  l.ingest([...tour1, { type: "extension_offered" }, { type: "extension_decided", accepted: true, touren_added: 3 }, deal(3, 2, 0)]);
  const m = slateModel(l.snapshot(), 1);
  assert.deepEqual(m.blocks.map((b) => b.type), ["deal", "deal", "tour", "extended", "deal"]);
  assert.equal(m.header.touren, 6);
  const refused = createLedger();
  refused.ingest([...tour1, { type: "extension_decided", accepted: false, touren_added: 0 }]);
  assert.equal(slateModel(refused.snapshot(), 1).header.touren, 3);
});

test("the settlement: the Contos, the difference, the sum owed, the payer", () => {
  const l = createLedger();
  l.ingest([...tour1, { type: "sitting_end", contos: [9, 15], balance: -6, payer: 0, kreuzer: 12 }]);
  const m = slateModel(l.snapshot(), 1);
  assert.deepEqual(m.settlement, { youConto: 15, oppConto: 9, difference: 6, kreuzer: 12, payer: "opp" });
  const mine = createLedger();
  mine.ingest([...tour1, { type: "sitting_end", contos: [15, 9], balance: 6, payer: 1, kreuzer: 12 }]);
  assert.equal(slateModel(mine.snapshot(), 1).settlement.payer, "you");
  const sq = createLedger();
  sq.ingest([start, { type: "sitting_end", contos: [3, 3], balance: 0, payer: null, kreuzer: 0 }]);
  assert.equal(slateModel(sq.snapshot(), 1).settlement.payer, null);
});

test("a snapshot restores the same ledger, and survives JSON", () => {
  const l = createLedger();
  l.ingest(tour1);
  const copy = createLedger(JSON.parse(JSON.stringify(l.snapshot())));
  assert.deepEqual(copy.snapshot(), l.snapshot());
  copy.ingest([deal(3, 2, 0)]);
  assert.notDeepEqual(copy.snapshot(), l.snapshot());
});

test("a new sitting_start clears the ledger", () => {
  const l = createLedger();
  l.ingest(tour1);
  l.ingest([start]);
  assert.equal(slateModel(l.snapshot(), 1).blocks.length, 0);
});

// ---- the DOM ---------------------------------------------------------------

function made(lang = "en") {
  const s = createStrings(TABLE, lang);
  const root = fakeRoot();
  const slate = createSlate(root, { t: s.t, tn: s.tn, onLang: s.onLang });
  return { s, root, slate };
}
const stateOf = (over = {}) => ({ human: 1, contos: [4, 5], balance: -1, touren_total: 3, over: false, ...over });

test("open and close toggle the sheet; the escape key closes it", () => {
  const { root, slate } = made();
  assert.equal(root.hidden, true);
  slate.open();
  assert.equal(root.hidden, false);
  assert.equal(root.getAttribute("aria-hidden"), "false");
  root.dispatch("keydown", { key: "Escape" });
  assert.equal(root.hidden, true);
  slate.open();
  slate.close();
  assert.equal(root.hidden, true);
});

test("update writes a row per deal in the current language", () => {
  const { root, slate } = made();
  slate.update(stateOf(), tour1);
  const text = root.textContent;
  assert.match(text, /3 Touren/);
  assert.match(text, /Your opponent/);
  assert.match(text, /Game 3/);
  assert.match(text, /Trull 1/);
  assert.match(text, /Ultimo won 1/);
  assert.match(text, /Tour 1/);
  assert.match(text, /Balance \+1 Pt: 2 kr\./);
  assert.match(text, /You receive/);
});

test("the slate speaks German, and follows a switch of language", () => {
  const { s, root, slate } = made();
  slate.update(stateOf(), tour1);
  s.setLang("de");
  const text = root.textContent;
  assert.match(text, /Ihr Gegner/);
  assert.match(text, /Spiel 3/);
  assert.match(text, /Troule 1/);
  assert.match(text, /Saldo \+1 Pt: 2 kr\./);
  assert.match(text, /Sie bekommen/);
});

test("settle writes the settlement: the payer and the sum, or Quitt", () => {
  const { s, root, slate } = made();
  slate.update(stateOf(), tour1);
  slate.settle({ type: "sitting_end", contos: [9, 15], balance: -6, payer: 0, kreuzer: 12 });
  assert.equal(root.hidden, false, "the slate opens large at the end");
  assert.ok(root.classList.contains("slate-large"));
  assert.match(root.textContent, /Settlement/);
  assert.match(root.textContent, /Owed: 12 kr\./);
  assert.match(root.textContent, /Your opponent pays/);
  const steps = root.byClass("slate-step");
  assert.ok(steps.length >= 4, "figures are written one at a time");
  slate.settle({ type: "sitting_end", contos: [3, 3], balance: 0, payer: null, kreuzer: 0 });
  assert.match(root.textContent, /Square\./);
  s.setLang("de");
  assert.match(root.textContent, /Quitt\./);
});

test("the tooltip carries the reconstruction note", () => {
  const { root } = made();
  assert.match(root.findAll((e) => e.getAttribute("title")?.includes("borrowed"))[0]?.getAttribute("title") ?? "", /borrowed/);
});

test("open can fold itself after a moment, unless it is touched", () => {
  const timers = [];
  const cancelled = new Set();
  const s = createStrings(TABLE);
  const root = fakeRoot();
  const slate = createSlate(root, { t: s.t, tn: s.tn, onLang: s.onLang, later: (ms, fn) => { timers.push({ ms, fn }); return timers.length; }, cancel: (id) => cancelled.add(id) });
  slate.open({ ms: 3000 });
  assert.equal(timers[0].ms, 3000);
  timers[0].fn();
  assert.equal(root.hidden, true);
  slate.open({ ms: 3000 });
  root.click();
  assert.ok(cancelled.has(2), "a tap cancels the fold");
  assert.equal(root.hidden, false, "a tap keeps it open");
});

test("snapshot and restore carry the ledger across a reload", () => {
  const a = made();
  a.slate.update(stateOf(), tour1);
  const snap = JSON.parse(JSON.stringify(a.slate.snapshot()));
  const b = made();
  b.slate.restore(snap);
  assert.equal(b.root.textContent, a.root.textContent);
});
