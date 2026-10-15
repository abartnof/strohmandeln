// What the tutor keeps between sittings (docs/TUTOR.md 8): the last 60 scored
// deals, each as its record and its summary, in a versioned key; every access
// survives missing, full or blocked storage; the store is injected, so these
// tests never touch the real key.
import test from "node:test";
import assert from "node:assert/strict";
import {
  KEY,
  MAX_DEALS,
  MAX_IMPORT,
  MAX_RECORD,
  addDeal,
  clearTutor,
  dropDeal,
  exportText,
  historyOf,
  importText,
  isNewerStore,
  loadDeals,
  refreshOne,
  setEvidence,
  setSummary,
  summaryVersion,
  validSummary,
} from "../src/tutorstore.js";

const memory = () => {
  const m = new Map();
  return { getItem: (k) => (m.has(k) ? m.get(k) : null), setItem: (k, v) => m.set(k, v), removeItem: (k) => m.delete(k), m };
};
const broken = {
  getItem: () => {
    throw new Error("blocked");
  },
  setItem: () => {
    throw new Error("full");
  },
  removeItem: () => {
    throw new Error("blocked");
  },
};
const SLUGS = ["worth", "cheap", "off", "run", "pagat", "void", "behind", "endgame", "bid", "announce"];
const summary = (v = 2, worth = "7 5 9") => ["strohmandeln evidence v" + v, ...SLUGS.map((s) => (s === "worth" ? `worth ${worth}` : `${s} 0 0 0`))].join("\n");
const detail = (s = summary()) => ({ summary: s, decisions: 12, facts: { ultimo_won: false, mond_caught: null, announced: [] }, misses: {}, settings: {}, variety: 3 });
const deal = (n, over = {}) => ({ record: `{"v":1,"n":${n}}`, assisted: [], summary: null, at: 1_760_000_000 + n, ...over });

test("the key and the caps are those of the spec", () => {
  assert.equal(KEY, "strohmandeln.tutor");
  assert.equal(MAX_DEALS, 60);
  assert.equal(MAX_RECORD, 20_000);
  assert.equal(MAX_IMPORT, 1_000_000);
});

test("a summary is the engine's text: a header and a line a skill", () => {
  assert.equal(validSummary(summary()), true);
  assert.equal(summaryVersion(summary(7)), 7);
  for (const bad of [null, 3, "", "worth 1 1 1", summary().replace("v2", "vX"), summary() + "\nextra 1 1 1", summary().split("\n").slice(0, 5).join("\n"), summary() + "\n--", "x".repeat(3000)]) {
    assert.equal(validSummary(bad), false, String(bad).slice(0, 30));
  }
  assert.equal(summaryVersion("junk"), null);
});

test("deals are kept oldest first, once each, and the oldest go past sixty", () => {
  const store = memory();
  for (let n = 0; n < 63; n++) assert.equal(addDeal(store, deal(n)), true);
  assert.equal(addDeal(store, deal(62)), true, "offered again: kept once");
  const kept = loadDeals(store);
  assert.equal(kept.length, 60);
  assert.equal(JSON.parse(kept[0].record).n, 3);
  assert.equal(JSON.parse(kept.at(-1).record).n, 62);
  assert.equal(JSON.parse(store.getItem(KEY)).v, 1);
});

test("a deal keeps its assisted numbers, its sitting and its number", () => {
  const store = memory();
  addDeal(store, deal(1, { assisted: [3, 4, 4], sit: "77", n: 5 }));
  const [d] = loadDeals(store);
  assert.deepEqual(d.assisted, [3, 4, 4]);
  assert.equal(d.sit, "77");
  assert.equal(d.n, 5);
  assert.equal(d.summary, null);
});

test("a summary is set once worked out, and only if it reads as the engine's", () => {
  const store = memory();
  addDeal(store, deal(1));
  assert.equal(setSummary(store, deal(1).record, "junk"), false);
  assert.equal(setSummary(store, deal(1).record, summary()), true);
  assert.equal(loadDeals(store)[0].summary, summary());
  assert.equal(setSummary(store, "no such record", summary()), false);
});

test("a record is one line of JSON with a version: a wire separator in it is refused", () => {
  const store = memory();
  for (const record of ["not json", "[1]", '{"v":2}', '{"v":1}\n==\n{"v":1}', '{"v":1}\r\n', "--", "==", "++", ""]) {
    assert.equal(addDeal(store, deal(1, { record })), false, JSON.stringify(record));
  }
  assert.equal(addDeal(store, deal(1)), true);
});

test("a bad entry is dropped, the rest kept; a bad store reads as empty", () => {
  const store = memory();
  const big = { record: "x".repeat(MAX_RECORD + 1) };
  store.setItem(KEY, JSON.stringify({ v: 1, deals: [deal(1), { record: 3 }, null, big, deal(2, { summary: "junk", assisted: ["x"] }), deal(3, { summary: summary() })] }));
  const kept = loadDeals(store);
  assert.deepEqual(kept.map((d) => JSON.parse(d.record).n), [1, 2, 3]);
  assert.equal(kept[1].summary, null, "a summary that is not the engine's is treated as missing");
  assert.deepEqual(kept[1].assisted, [], "so are assisted numbers that are not numbers");
  assert.equal(kept[2].summary, summary());
  store.setItem(KEY, "not json");
  assert.deepEqual(loadDeals(store), []);
  assert.deepEqual(loadDeals(null), []);
});

test("missing, blocked or full storage means one deal alone, without a throw", () => {
  assert.equal(addDeal(null, deal(1)), false);
  assert.equal(addDeal(broken, deal(1)), false);
  assert.deepEqual(loadDeals(broken), []);
  assert.equal(setSummary(broken, "x", summary()), false);
  assert.doesNotThrow(() => clearTutor(broken));
  assert.equal(isNewerStore(broken), false);
});

test("a store written by a newer page is read as empty and never written over", () => {
  const store = memory();
  store.setItem(KEY, JSON.stringify({ v: 2, deals: [deal(1)] }));
  assert.deepEqual(loadDeals(store), []);
  assert.equal(isNewerStore(store), true);
  assert.equal(addDeal(store, deal(9)), false);
  assert.equal(setSummary(store, deal(1).record, summary()), false);
  assert.equal(importText(store, exportText(memory())).ok, false);
  assert.equal(JSON.parse(store.getItem(KEY)).v, 2);
});

test("the old progress key is left in place and unread", () => {
  const store = memory();
  store.setItem("strohmandeln.progress", JSON.stringify({ v: 1, sittings: [{ won: 1, lost: 0, drawn: 0, net: 3, tip: null }] }));
  addDeal(store, deal(1));
  clearTutor(store);
  assert.ok(store.getItem("strohmandeln.progress"));
  assert.equal(store.getItem(KEY), null);
});

test("the history is the summaries oldest first, missing ones null", () => {
  const store = memory();
  addDeal(store, deal(1, { summary: summary() }));
  addDeal(store, deal(2));
  assert.deepEqual(historyOf(loadDeals(store)), [summary(), null]);
});

test("what the sitting's brief is made of is kept with the deal: its detail", () => {
  const store = memory();
  addDeal(store, deal(1));
  assert.equal(loadDeals(store)[0].detail, null);
  assert.equal(setEvidence(store, deal(1).record, summary(), detail()), true);
  assert.deepEqual(loadDeals(store)[0].detail, detail());
  assert.equal(loadDeals(store)[0].summary, summary());
  // a detail that is not an object, or is too large, or lacks the engine's summary, is not kept
  for (const bad of [null, "x", [1], { ...detail(), summary: "junk" }, { ...detail(), pad: "x".repeat(5000) }]) {
    assert.equal(setEvidence(store, deal(1).record, summary(), bad), false);
  }
  assert.deepEqual(loadDeals(store)[0].detail, detail(), "unchanged");
  store.setItem(KEY, JSON.stringify({ v: 1, deals: [deal(2, { summary: summary(), detail: "junk" })] }));
  assert.equal(loadDeals(store)[0].detail, null, "a damaged detail reads as missing");
  assert.equal(setEvidence(null, "x", summary(), detail()), false);
});

test("clearing says what it did, and leaves a newer page's store alone", () => {
  const store = memory();
  store.setItem(KEY, JSON.stringify({ v: 2, deals: [deal(1)] }));
  assert.equal(clearTutor(store), "newer");
  assert.ok(store.getItem(KEY), "a newer page's store is not ours to clear");
  const mine = memory();
  addDeal(mine, deal(1));
  assert.equal(clearTutor(mine), "cleared");
  assert.equal(mine.getItem(KEY), null);
  assert.equal(clearTutor(broken), "blocked");
  assert.equal(clearTutor(null), "blocked");
});

test("the number a deal had is 1 to 9999: anything else is read as missing", () => {
  const store = memory();
  for (const n of [0, -3, 10000, 2 ** 33, 1.5, "7"]) {
    store.setItem(KEY, JSON.stringify({ v: 1, deals: [deal(1, { n })] }));
    assert.equal("n" in loadDeals(store)[0], false, String(n));
  }
  for (const n of [1, 9999]) {
    store.setItem(KEY, JSON.stringify({ v: 1, deals: [deal(1, { n })] }));
    assert.equal(loadDeals(store)[0].n, n);
  }
  assert.equal(addDeal(memory(), deal(1, { n: 0 })), true, "the deal is kept, without its number");
});

test("it can be exported, cleared and imported; a bad file is refused in a key and changes nothing", () => {
  const store = memory();
  addDeal(store, deal(1, { summary: summary() }));
  const text = exportText(store);
  assert.equal(JSON.parse(text).strohmandeln, "tutor");
  clearTutor(store);
  assert.deepEqual(loadDeals(store), []);
  assert.deepEqual(importText(store, text), { ok: true, count: 1 });
  assert.equal(loadDeals(store)[0].summary, summary());
  const before = store.getItem(KEY);
  const refuse = (t, error) => {
    assert.deepEqual(importText(store, t), { ok: false, error });
    assert.equal(store.getItem(KEY), before, error);
  };
  refuse("x".repeat(MAX_IMPORT + 1), "progress-big");
  refuse("not json", "progress-bad");
  refuse(JSON.stringify({ strohmandeln: "other", v: 1, deals: [] }), "progress-bad");
  refuse(JSON.stringify({ strohmandeln: "tutor", v: 1, deals: [{ record: 3 }] }), "progress-bad");
  refuse(JSON.stringify({ strohmandeln: "tutor", v: 2, deals: [] }), "progress-version");
  refuse(JSON.stringify({ strohmandeln: "tutor", v: 1, deals: [] }), "progress-empty");
  assert.deepEqual(importText(broken, text), { ok: false, error: "progress-blocked" });
});

// ---- stale summaries ------------------------------------------------------------

// An engine that says which entries are stale and gives a summary for a record.
const fakeEngine = ({ stale, version = 2, summaryFor = () => summary(), calls = [] }) => ({
  learner: (history) => {
    calls.push(["learner", history.length]);
    return { stale: stale(history), version };
  },
  evidenceOf: (record, assisted) => {
    calls.push(["evidenceOf", record, assisted]);
    const s = summaryFor(record);
    return typeof s === "string" ? { summary: s, detail: detail(s) } : s;
  },
});

test("a stale summary is recomputed from its record and kept; a deal is never dropped for it", async () => {
  const store = memory();
  addDeal(store, deal(1, { summary: summary(1), assisted: [2] }));
  addDeal(store, deal(2, { summary: summary(), detail: detail() }));
  const calls = [];
  const engine = fakeEngine({ stale: (h) => h.map((s, i) => (summaryVersion(s) === 1 ? i : -1)).filter((i) => i >= 0), calls });
  assert.equal(await refreshOne(store, engine), true);
  assert.deepEqual(calls.at(-1), ["evidenceOf", deal(1).record, [2]], "with its assisted numbers");
  assert.equal(summaryVersion(loadDeals(store)[0].summary), 2);
  assert.equal(await refreshOne(store, engine), false, "nothing left stale");
  assert.equal(loadDeals(store).length, 2);
});

test("a deal kept without its detail has it worked out while the table is idle", async () => {
  const store = memory();
  addDeal(store, deal(1, { summary: summary() }));
  const engine = fakeEngine({ stale: () => [] });
  assert.equal(await refreshOne(store, engine), true);
  assert.deepEqual(loadDeals(store)[0].detail, detail());
  assert.equal(await refreshOne(store, engine), false);
});

test("only a record the engine cannot restore is dropped; another error leaves it", async () => {
  const store = memory();
  addDeal(store, deal(1));
  const other = fakeEngine({ stale: () => [0], summaryFor: () => ({ error: "bad-constants" }) });
  assert.equal(await refreshOne(store, other), true);
  assert.equal(loadDeals(store).length, 1, "bad-constants does not take the deal");
  assert.equal(await refreshOne(store, fakeEngine({ stale: () => [0], summaryFor: () => null })), true);
  assert.equal(loadDeals(store).length, 1, "nothing at all (a stopped engine) does not either");
  // a record that does not even parse for the engine cannot be restored
  assert.equal(await refreshOne(store, fakeEngine({ stale: () => [0], summaryFor: () => ({ error: "bad-input", why: "record" }) })), true);
  assert.equal(loadDeals(store).length, 0);
});

test("a record the engine can no longer restore is dropped after one failed try", async () => {
  const store = memory();
  addDeal(store, deal(1));
  addDeal(store, deal(2, { summary: summary() }));
  const engine = fakeEngine({ stale: (h) => h.map((s, i) => (s === null ? i : -1)).filter((i) => i >= 0), summaryFor: () => ({ error: "replay" }) });
  const tried = new Set();
  assert.equal(await refreshOne(store, engine, tried), true);
  assert.deepEqual(loadDeals(store).map((d) => JSON.parse(d.record).n), [2]);
});

test("a summary of a newer version is not recomputed back, and a record is tried once", async () => {
  const store = memory();
  addDeal(store, deal(1, { summary: summary(5) }));
  const engine = fakeEngine({ stale: () => [0], version: 2 });
  assert.equal(await refreshOne(store, engine), false);
  assert.equal(summaryVersion(loadDeals(store)[0].summary), 5);
  const store2 = memory();
  addDeal(store2, deal(1));
  let asked = 0;
  const flaky = fakeEngine({ stale: () => [0], summaryFor: () => (asked++, undefined) });
  flaky.stopped = () => true;
  const tried = new Set();
  assert.equal(await refreshOne(store2, flaky, tried), true);
  assert.equal(await refreshOne(store2, flaky, tried), false, "not asked twice");
  assert.equal(asked, 1);
  assert.equal(loadDeals(store2).length, 1, "a stopped engine drops nothing");
});

test("an engine that throws leaves the deals alone", async () => {
  const store = memory();
  addDeal(store, deal(1));
  const engine = { learner: () => { throw new Error("trap"); } };
  const saved = console.error;
  console.error = () => {};
  try {
    assert.equal(await refreshOne(store, engine), false);
  } finally {
    console.error = saved;
  }
  assert.equal(loadDeals(store).length, 1);
});
