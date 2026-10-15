// What the tutor keeps between sittings (docs/TABLE3D.md 6.4.4): the last 30
// sittings' tallies, version 1, every access survives missing or full storage.
import test from "node:test";
import assert from "node:assert/strict";
import { KEY, MAX, addSitting, clearProgress, exportText, loadProgress, summaryOf } from "../src/progress.js";

const memory = () => {
  const m = new Map();
  return { getItem: (k) => (m.has(k) ? m.get(k) : null), setItem: (k, v) => m.set(k, v), removeItem: (k) => m.delete(k), m };
};
const broken = { getItem: () => { throw new Error("blocked"); }, setItem: () => { throw new Error("full"); }, removeItem: () => { throw new Error("blocked"); } };
const sitting = (n) => ({ won: n, lost: 1, drawn: 0, net: n - 1, tip: "try-count" });

test("the key and the cap are those of the spec", () => {
  assert.equal(KEY, "strohmandeln.progress");
  assert.equal(MAX, 30);
});

test("a sitting's review becomes a summary of its tally and its tip", () => {
  const review = { deal: 0, bullets: [{ lead: "sitting", key: "sitting-tally", params: { won: 2, lost: 1, drawn: 0, net: 3 } }, { lead: "decided", key: "level", params: { deal: 1 } }, { lead: "try", key: "try-pagat", params: {} }] };
  assert.deepEqual(summaryOf(review), { won: 2, lost: 1, drawn: 0, net: 3, tip: "try-pagat" });
  assert.equal(summaryOf(null), null);
  assert.equal(summaryOf({ bullets: [] }), null);
});

test("sittings are kept, oldest first, and the oldest go past thirty", () => {
  const store = memory();
  for (let n = 0; n < 33; n++) assert.equal(addSitting(store, sitting(n)), true);
  const kept = loadProgress(store);
  assert.equal(kept.length, 30);
  assert.equal(kept[0].won, 3);
  assert.equal(kept.at(-1).won, 32);
  assert.equal(JSON.parse(store.getItem(KEY)).v, 1);
});

test("a bad entry is dropped, the rest kept; a bad store reads as empty", () => {
  const store = memory();
  store.setItem(KEY, JSON.stringify({ v: 1, sittings: [sitting(1), { won: "x" }, null, sitting(2)] }));
  assert.deepEqual(loadProgress(store).map((s) => s.won), [1, 2]);
  store.setItem(KEY, "not json");
  assert.deepEqual(loadProgress(store), []);
  assert.deepEqual(loadProgress(null), []);
});

test("missing, blocked or full storage means a sitting alone, without a throw", () => {
  assert.equal(addSitting(null, sitting(1)), false);
  assert.equal(addSitting(broken, sitting(1)), false);
  assert.deepEqual(loadProgress(broken), []);
  assert.doesNotThrow(() => clearProgress(broken));
});

test("a store written by a newer page is read as empty and never written over", () => {
  const store = memory();
  store.setItem(KEY, JSON.stringify({ v: 2, sittings: [sitting(1)] }));
  assert.deepEqual(loadProgress(store), []);
  assert.equal(addSitting(store, sitting(9)), false);
  assert.equal(JSON.parse(store.getItem(KEY)).v, 2);
});

test("it can be exported and cleared", () => {
  const store = memory();
  addSitting(store, sitting(1));
  assert.deepEqual(JSON.parse(exportText(store)).sittings.length, 1);
  clearProgress(store);
  assert.deepEqual(loadProgress(store), []);
  assert.deepEqual(JSON.parse(exportText(store)).sittings, []);
});
