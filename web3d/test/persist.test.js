import test from "node:test";
import assert from "node:assert/strict";
import { KEY, MAX_SAVED, MAX_FILE, FILE_NAME, saveSitting, loadSitting, clearSitting, fileText, parseFile } from "../src/persist.js";
import { fakeStore } from "./support/fakedom.js";

const EXPORT = { protocol: 1, human: 1, level: 4, record: { version: 1, seed: 7, settings: { both_pass: "play", ultimo: 1, values: "2/3/4" }, touren: 3, moves: [{ seat: 1, action: { action: "take" } }] } };
const EXTRAS = { lang: "de", sig: { a: 1 }, seen: ["packets"], human: 1, slate: { ledger: { v: 1 }, human: 1 } };

test("a sitting is kept after an action and read back with the page's extras", () => {
  const store = fakeStore();
  assert.deepEqual(saveSitting(EXPORT, EXTRAS, {}, store), { ok: true });
  assert.deepEqual(loadSitting(store), { export: EXPORT, extras: EXTRAS });
});

test("nothing kept reads as null", () => {
  assert.equal(loadSitting(fakeStore()), null);
  assert.equal(loadSitting(fakeStore({}, { broken: true })), null);
  assert.equal(loadSitting(null), null);
});

test("a sitting that is over is not kept, and clears the old one (9.4)", () => {
  const store = fakeStore();
  saveSitting(EXPORT, EXTRAS, {}, store);
  assert.deepEqual(saveSitting(EXPORT, EXTRAS, { over: true }, store), { ok: true });
  assert.equal(loadSitting(store), null);
  assert.equal(store.map.has(KEY), false);
});

test("a record over 200 KB is refused and the old one is left", () => {
  const store = fakeStore();
  saveSitting(EXPORT, EXTRAS, {}, store);
  const big = { ...EXPORT, record: { ...EXPORT.record, moves: new Array(MAX_SAVED).fill({ seat: 0, action: { action: "pass" } }) } };
  assert.deepEqual(saveSitting(big, EXTRAS, {}, store), { ok: false, reason: "too-big" });
  assert.deepEqual(loadSitting(store).export, EXPORT);
});

test("blocked storage fails quietly with a reason", () => {
  assert.deepEqual(saveSitting(EXPORT, EXTRAS, {}, fakeStore({}, { broken: true })), { ok: false, reason: "unavailable" });
  assert.deepEqual(saveSitting(EXPORT, EXTRAS, {}, null), { ok: false, reason: "unavailable" });
});

test("a damaged or foreign entry reads as null", () => {
  for (const bad of ["{nope", "[]", JSON.stringify({ v: 9, export: EXPORT }), JSON.stringify({ v: 1, export: { protocol: 2 } }), JSON.stringify({ v: 1, export: { protocol: 1 } }), JSON.stringify({ v: 1, over: true, export: EXPORT })]) {
    assert.equal(loadSitting(fakeStore({ [KEY]: bad })), null, bad);
  }
});

test("clearSitting removes it", () => {
  const store = fakeStore();
  saveSitting(EXPORT, EXTRAS, {}, store);
  clearSitting(store);
  assert.equal(loadSitting(store), null);
  clearSitting(fakeStore({}, { broken: true }));
});

test("the file is {v, protocol, export, extras} and reads back", () => {
  assert.equal(FILE_NAME, "strohmandeln-sitting.json");
  const text = fileText(EXPORT, EXTRAS);
  const parsed = JSON.parse(text);
  assert.deepEqual(Object.keys(parsed).sort(), ["export", "extras", "protocol", "v"]);
  assert.equal(parsed.protocol, 1);
  assert.deepEqual(parseFile(text), { ok: true, export: EXPORT, extras: EXTRAS });
});

test("a bad file is refused in a sentence key and changes nothing", () => {
  assert.deepEqual(parseFile("not json"), { ok: false, error: "load-bad" });
  assert.deepEqual(parseFile("{}"), { ok: false, error: "load-bad" });
  assert.deepEqual(parseFile(JSON.stringify({ v: 1, protocol: 2, export: EXPORT, extras: {} })), { ok: false, error: "load-bad" });
  assert.deepEqual(parseFile(JSON.stringify({ v: 1, protocol: 1, export: { protocol: 1, record: {} }, extras: {} })), { ok: false, error: "load-bad" });
  assert.deepEqual(parseFile("x".repeat(MAX_FILE + 1)), { ok: false, error: "load-big" });
});

test("a file without extras still loads", () => {
  const r = parseFile(JSON.stringify({ v: 1, protocol: 1, export: EXPORT }));
  assert.equal(r.ok, true);
  assert.deepEqual(r.extras, {});
});
