// The table talk: protocol events to phrase groups (docs/TABLE3D.md section 5).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createTalk, CALLS } from "../src/talk.js";

const words = JSON.parse(readFileSync(new URL("../words.json", import.meta.url), "utf8"));
const H = 1; // the human's seat
const O = 0; // the opponent's

// A small seeded rng (mulberry32).
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

function make({ bank = words, lang = "en", seed = 1, mode } = {}) {
  const said = [];
  const talk = createTalk({ words: bank, getLang: () => lang, rng: seeded(seed), say: (l) => said.push(l) });
  if (mode) talk.setMode(mode);
  const feed = (...events) => events.forEach((e) => talk.onEvent(e, { human: H }));
  const groups = () => said.map((l) => l.key.replace(/\.\d+$/, ""));
  return { talk, said, feed, groups };
}

const deal = (n, dealer = O) => ({ type: "deal_start", deal_number: n, tour: Math.ceil(n / 2), dealer, forehand: 1 - dealer, multiplier: 1 });
const pulled = (seat, card, packet = 1, last = false) => ({ type: "card_pulled", seat, packet, card, last });
const cover = (seat, packet) => ({ type: "cover_set", seat, packet, card: "5h" });
const play = (seat, card, trick = 1) => ({ type: "card_played", seat, card, source: "hand", trick });
const scored = (seat, result = "declarer_win") => ({ type: "game_scored", result, credits: [{ seat, cause: "game", points: 1 }], multiplier: 1, contos: [0, 0] });

test("every call group is in the bank", () => {
  for (const g of CALLS) assert.ok(words.groups[g], g);
});

test("a line is {key, text, who}, in the language asked for", () => {
  const en = make({ lang: "en" });
  en.feed({ type: "bid", seat: O });
  const de = make({ lang: "de" });
  de.feed({ type: "bid", seat: O });
  assert.match(en.said[0].key, /^bid-take-mine\.\d+$/);
  assert.equal(en.said[0].text, words.texts[en.said[0].key].en);
  assert.equal(de.said[0].text, words.texts[de.said[0].key].de);
  assert.equal(en.said[0].who, "them");
});

test("the sitting begins: greeting, Touren, the cut", () => {
  const t = make();
  t.feed({ type: "sitting_start", first_dealer: 0, touren: 3 });
  assert.deepEqual(t.groups(), ["greet", "touren-3", "cut"]);
  const u = make();
  u.feed({ type: "sitting_start", first_dealer: 1, touren: 6 });
  assert.deepEqual(u.groups(), ["greet", "touren-6", "cut"]);
});

test("deals, bids, passes, void, doubling, both-pass, throw-in, reveal", () => {
  const t = make();
  t.feed(deal(1, O), deal(2, H), { type: "void_deal", seats: [0] }, { type: "doubling_applied", multiplier: 2 },
    { type: "bid", seat: O }, { type: "bid", seat: H }, { type: "pass", seat: O }, { type: "pass", seat: H },
    { type: "both_pass", mode: "play" }, { type: "thrown_in" }, { type: "reveal_start", seat: O });
  assert.deepEqual(t.groups(), ["deal-mine", "deal-yours", "void-deal", "doubling-applied", "bid-take-mine", "bid-take-yours",
    "bid-pass-mine", "bid-pass-yours", "both-pass", "thrown-in", "reveal-begin"]);
});

test("pulls: king, tarock, and the three named cards, by side", () => {
  const t = make();
  t.feed({ type: "reveal_start", seat: O });
  t.feed(pulled(O, "Kh"), pulled(H, "Kd", 2), pulled(O, "T5", 3), pulled(H, "T7", 1), pulled(O, "SK", 2), pulled(H, "T21", 3), pulled(O, "T1", 1));
  assert.deepEqual(t.groups().slice(1), ["pull-king-mine", "pull-king-yours", "pull-tarock-mine", "pull-tarock-yours", "pull-sku", "pull-mond", "pull-pagat"]);
});

test("a plain suit card pulled is not remarked", () => {
  const t = make();
  t.feed(pulled(O, "5h"), pulled(O, "Qd"));
  assert.deepEqual(t.said, []);
});

test("a chain of three pulls from one packet: the run replaces the third pull's line", () => {
  const t = make();
  t.feed(pulled(H, "T3", 2), pulled(H, "T4", 2), pulled(H, "T5", 2), pulled(H, "T6", 2, true));
  assert.deepEqual(t.groups(), ["pull-tarock-yours", "pull-tarock-yours", "run-yours"]);
  const u = make();
  u.feed(pulled(O, "T3", 2), { type: "packet_turned", seat: O, packet: 3, card: "T2" }, pulled(O, "T2", 3), pulled(O, "T9", 3));
  assert.deepEqual(u.groups(), ["pull-tarock-mine", "pull-tarock-mine", "pull-tarock-mine"]);
});

test("nothing: three covers set with no pull, once per side per deal", () => {
  const t = make();
  t.feed(deal(1), { type: "reveal_start", seat: H });
  for (const p of [1, 2, 3]) t.feed({ type: "packet_turned", seat: H, packet: p, card: "5h" }, cover(H, p));
  assert.deepEqual(t.groups(), ["deal-mine", "reveal-begin", "nothing-yours"]);
  t.feed({ type: "reveal_start", seat: H });
  for (const p of [1, 2, 3]) t.feed(cover(H, p));
  assert.equal(t.groups().filter((g) => g === "nothing-yours").length, 1);
  t.feed(deal(2), { type: "reveal_start", seat: H });
  for (const p of [1, 2, 3]) t.feed(cover(H, p));
  assert.equal(t.groups().filter((g) => g === "nothing-yours").length, 2);
});

test("a reveal with a pull is not 'nothing'", () => {
  const t = make();
  t.feed({ type: "reveal_start", seat: O }, pulled(O, "5h"), cover(O, 1), cover(O, 2), cover(O, 3));
  assert.deepEqual(t.groups(), ["reveal-begin"]);
});

test("tricks: good and bad, the Sküs, the Mondfang", () => {
  const t = make();
  t.feed(play(O, "T9"), play(H, "4h"), { type: "trick_won", winner: O, cards: ["T9", "4h"], trick: 1 });
  assert.deepEqual(t.groups(), ["trick-good-mine"]);
  const u = make();
  u.feed(play(H, "Kd", 2), play(O, "T4", 2), { type: "trick_won", winner: O, cards: ["Kd", "T4"], trick: 2 });
  assert.match(u.groups()[0], /^trick-(bad-yours|good-mine)$/);
  const v = make();
  v.feed(play(O, "SK", 3), play(H, "T21", 3), { type: "trick_won", winner: O, cards: ["SK", "T21"], trick: 3 });
  assert.deepEqual(v.groups(), ["sku-played", "mond-caught-mine"]);
  const w = make();
  w.feed(play(O, "T21", 3), play(H, "SK", 3), { type: "trick_won", winner: H, cards: ["T21", "SK"], trick: 3 });
  assert.deepEqual(w.groups(), ["sku-played", "mond-caught-yours"]);
});

test("an unremarkable trick says nothing", () => {
  const t = make();
  t.feed(play(O, "5h"), play(H, "4h"), { type: "trick_won", winner: O, cards: ["5h", "4h"], trick: 1 });
  assert.deepEqual(t.said, []);
});

test("ultimo, announcements, valat, forfeit", () => {
  const t = make();
  t.feed({ type: "pagat_ultimo", seat: O, result: "won" }, { type: "pagat_ultimo", seat: H, result: "lost" },
    { type: "announcement", seat: O, honour: "trull", cards: [] }, { type: "announcement", seat: H, honour: "kings", cards: [] },
    { type: "valat", seat: H }, { type: "honour_forfeited", seat: H, honour: "kings" });
  assert.deepEqual(t.groups(), ["ultimo-won-mine", "ultimo-lost-yours", "trull-mine", "kings-yours", "valat-yours", "honour-forfeited"]);
});

test("nothing is said between the first count card and the last popup", () => {
  const t = make();
  t.feed({ type: "count_start" });
  assert.deepEqual(t.groups(), ["count-start"]);
  t.feed({ type: "bid", seat: O }, { type: "pagat_ultimo", seat: O, result: "won" }, { type: "slow", n: 1 });
  assert.equal(t.said.length, 1, "quiet while counting");
  t.feed({ type: "count_close" });
  t.feed(scored(O));
  assert.match(t.groups().at(-1), /^result-/);
});

test("count-close is said for a close count only", () => {
  const close = make();
  close.feed({ type: "count", thirds: [107, 103], declarer: 0, threshold: 107 }, { type: "count_start" }, { type: "count_close" });
  assert.deepEqual(close.groups(), ["count-start", "count-close"]);
  const wide = make();
  wide.feed({ type: "count", thirds: [150, 60], declarer: 0, threshold: 107 }, { type: "count_start" }, { type: "count_close" });
  assert.deepEqual(wide.groups(), ["count-start"]);
});

test("results: winner and loser, the draw, wide wins bring luck talk", () => {
  const opp = make();
  opp.feed({ type: "count", thirds: [110, 100], declarer: O, threshold: 107 }, scored(O));
  assert.match(opp.groups()[0], /^result-(won-mine|lost-yours)$/);
  assert.equal(opp.groups().length, 1);
  const me = make();
  me.feed({ type: "count", thirds: [100, 110], declarer: H, threshold: 107 }, scored(H));
  assert.match(me.groups()[0], /^result-(lost-mine|won-yours)$/);
  const draw = make();
  draw.feed(scored(O, "draw"));
  assert.deepEqual(draw.groups(), ["draw"]);
  const wide = make();
  wide.feed({ type: "count", thirds: [60, 150], declarer: H, threshold: 107 }, scored(H));
  assert.equal(wide.groups().length, 2);
  assert.match(wide.groups()[1], /^luck-(yours|bad-mine)$/);
  const wideOpp = make();
  wideOpp.feed({ type: "count", thirds: [150, 60], declarer: O, threshold: 107 }, scored(O));
  assert.equal(wideOpp.groups()[1], "luck-mine");
});

test("luck from a lopsided first reveal: six for one side, none for the other", () => {
  const t = make();
  t.feed(deal(1), { type: "reveal_start", seat: H });
  for (let i = 0; i < 6; i++) t.feed(pulled(H, `T${i + 2}`, (i % 3) + 1));
  t.feed({ type: "trick_won", winner: H, cards: ["5h", "4h"], trick: 1 });
  t.feed(scored(H));
  assert.match(t.groups().at(-1), /^luck-(yours|bad-mine)$/);
});

test("tours, extension, settlement, farewell", () => {
  const t = make();
  t.feed({ type: "sitting_start", first_dealer: 0, touren: 3 });
  t.said.length = 0;
  t.feed({ type: "tour_end", tour: 1, contos: [0, 0], balance: 0 }, { type: "tour_end", tour: 3, contos: [0, 0], balance: 0 },
    { type: "extension_offered" }, { type: "extension_decided", accepted: true, touren_added: 1 }, { type: "extension_decided", accepted: false, touren_added: 0 },
    { type: "sitting_end", contos: [1, 2], balance: 1, payer: O, kreuzer: 2 }, { type: "sitting_end", contos: [1, 2], balance: -1, payer: H, kreuzer: 2 },
    { type: "sitting_end", contos: [1, 1], balance: 0, payer: null, kreuzer: 0 });
  assert.deepEqual(t.groups(), ["tour-end", "tour-last", "extend-ask", "extend-yes", "extend-no", "settle-opp-pays", "farewell",
    "settle-player-pays", "farewell", "settle-even", "farewell"]);
});

test("an extension moves the last Tour", () => {
  const t = make();
  t.feed({ type: "sitting_start", first_dealer: 0, touren: 3 });
  t.feed({ type: "extension_decided", accepted: true, touren_added: 1 });
  t.said.length = 0;
  t.feed({ type: "tour_end", tour: 3, contos: [0, 0], balance: 0 }, { type: "tour_end", tour: 4, contos: [0, 0], balance: 0 });
  assert.deepEqual(t.groups(), ["tour-end", "tour-last"]);
});

test("table-level moments: slow, a refused card, the opponent's own error, doubling lapsed", () => {
  const t = make();
  t.feed({ type: "slow", n: 1 }, { type: "refused" }, { type: "opponent_error" }, { type: "doubling_lapsed" });
  assert.deepEqual(t.groups(), ["slow", "slip-yours", "blunder-own", "doubling-lapsed"]);
});

test("modes: all, calls only, none", () => {
  const events = [{ type: "bid", seat: O }, { type: "slow", n: 1 }, pulled(O, "Kh"), { type: "pagat_ultimo", seat: O, result: "won" }];
  const all = make();
  all.feed(...events);
  assert.equal(all.said.length, 4);
  const calls = make({ mode: "calls" });
  calls.feed(...events);
  assert.deepEqual(calls.groups(), ["bid-take-mine", "ultimo-won-mine"]);
  const none = make({ mode: "none" });
  none.feed(...events);
  assert.equal(none.said.length, 0);
  calls.talk.setMode("all");
  calls.feed({ type: "slow", n: 2 });
  assert.equal(calls.groups().at(-1), "slow");
  assert.throws(() => calls.talk.setMode("loud"));
});

test("deterministic under a seeded rng; different seeds differ somewhere", () => {
  const run = (seed) => {
    const t = make({ seed });
    for (let d = 1; d <= 6; d++) t.feed(deal(d, d % 2), { type: "bid", seat: O }, { type: "pagat_ultimo", seat: O, result: "won" });
    return t.said.map((l) => l.key).join();
  };
  assert.equal(run(7), run(7));
  assert.notEqual(run(7), run(8));
});

const sigBank = (texts) => ({
  groups: Object.fromEntries(Object.entries(texts).map(([g, list]) => [g, list.map((_, i) => `${g}.${i}`)])),
  texts: Object.fromEntries(Object.entries(texts).flatMap(([g, list]) => list.map(([en, sig], i) => [`${g}.${i}`, { en, de: en, sig }]))),
});

test("cross-group repeat control: a saying heard in one group is passed over in another", () => {
  const bank = sigBank({
    "pull-king-mine": [["A", "S"], ["A2", "x1"]],
    "nothing-mine": [["B", "S"], ["B2", "x2"]],
  });
  for (let seed = 1; seed <= 20; seed++) {
    const t = make({ bank, seed });
    t.feed(deal(1), pulled(O, "Kh"), { type: "reveal_start", seat: O }, cover(O, 1), cover(O, 2), cover(O, 3));
    const sigs = t.said.map((l) => bank.texts[l.key].sig);
    assert.equal(sigs.length, 2);
    assert.ok(sigs.filter((s) => s === "S").length <= 1, `seed ${seed}: ${t.said.map((l) => l.key)}`);
  }
});

test("a calls-only line is not subject to the sig record", () => {
  const bank = sigBank({ "bid-take-mine": [["C", "S"]], "bid-pass-mine": [["C", "S"]] });
  const t = make({ bank });
  t.feed({ type: "bid", seat: O }, { type: "pass", seat: O }, { type: "bid", seat: O });
  assert.equal(t.said.length, 3);
});

test("when every line of a group is spent the oldest is allowed again", () => {
  const bank = sigBank({ "pull-king-mine": [["A", "s0"], ["B", "s1"]] });
  const t = make({ bank });
  t.feed(deal(1), pulled(O, "Kh", 1), pulled(O, "Kd", 2), pulled(O, "Ks", 3), pulled(O, "Kc", 1));
  assert.equal(t.said.length, 4);
  assert.notEqual(t.said[2].key, t.said[3].key);
});

test("a sig spoken three deals ago may be said again", () => {
  const bank = sigBank({ "pull-king-mine": [["A", "s0"], ["B", "s1"]], "nothing-mine": [["C", "s0"]] });
  const t = make({ bank });
  t.feed(deal(1), pulled(O, "Kh"));
  const first = t.said[0].key;
  t.feed(deal(4), { type: "reveal_start", seat: O }, cover(O, 1), cover(O, 2), cover(O, 3));
  // deal 4 is three deals on: s0 is free again, so it is said whatever the group offers
  assert.equal(t.groups().at(-1), "nothing-mine");
  assert.ok(first);
});

test("5 sittings: no sig twice in a 3-deal window where an alternative existed", () => {
  for (let seed = 1; seed <= 5; seed++) {
    const t = make({ seed });
    const last = new Map(); // sig -> deal of last hearing
    const slack = [];
    t.feed({ type: "sitting_start", first_dealer: 0, touren: 6 });
    for (let d = 1; d <= 12; d++) {
      t.feed(deal(d, d % 2));
      const before = t.said.length;
      t.feed({ type: "reveal_start", seat: O });
      for (let i = 0; i < 4; i++) t.feed(pulled(i % 2 ? H : O, i % 2 ? "Kh" : "T5", (i % 3) + 1));
      t.feed({ type: "reveal_start", seat: H });
      for (const p of [1, 2, 3]) t.feed(cover(H, p));
      for (let k = 0; k < 3; k++) {
        t.feed(play(O, "T9", k), play(H, "4h", k), { type: "trick_won", winner: k % 2 ? O : H, cards: ["T9", "4h"], trick: k });
      }
      for (const l of t.said.slice(before)) {
        const sig = words.texts[l.key].sig;
        assert.ok(sig, `${l.key} has a sig`);
        if (!CALLS.has(l.key.replace(/\.\d+$/, "")) && last.has(sig) && d - last.get(sig) < 3) slack.push(`${sig} @${d}`);
        last.set(sig, d);
      }
    }
    assert.deepEqual(slack, [], `seed ${seed}`);
  }
});

test("snapshot and restore keep the record across a reload", () => {
  const bank = sigBank({ "pull-king-mine": [["A", "s0"], ["B", "s1"]] });
  const t = make({ bank });
  t.feed(deal(1), pulled(O, "Kh"));
  const snap = t.talk.snapshot();
  const u = make({ bank, seed: 99 });
  u.talk.restore(JSON.parse(JSON.stringify(snap)));
  u.feed(pulled(O, "Kh"));
  assert.notEqual(u.said[0].key, t.said[0].key);
});

test("count-close is only for the exact 35 and two thirds the lines name (107)", () => {
  for (const [thirds, closes] of [[[108, 102], false], [[106, 104], false], [[107, 103], true]]) {
    const t = make();
    t.feed({ type: "count", thirds, declarer: 0, threshold: 107 }, { type: "count_start" }, { type: "count_close" });
    assert.equal(t.groups().includes("count-close"), closes, String(thirds));
  }
});

test("a new deal ends a count that never closed, and forgets the last count", () => {
  const t = make();
  t.feed(deal(1), { type: "count", thirds: [150, 60], declarer: 0, threshold: 107 }, { type: "count_start" });
  t.feed(deal(2));
  assert.equal(t.groups().at(-1).startsWith("deal-"), true, "quiet lifted by the next deal");
  // no stale count: a deal scored without a count event of its own is not 'wide'
  const u = make();
  u.feed(deal(1), { type: "count", thirds: [150, 60], declarer: 0, threshold: 107 }, scored(O));
  u.feed(deal(2), scored(O));
  assert.equal(u.groups().filter((g) => g.startsWith("luck-")).length, 1);
});
