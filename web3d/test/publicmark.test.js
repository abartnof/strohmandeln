// The public mark is a memory aid, so a hint (TABLE3D 1.5.7, 6.3.4; the maintainer's decision, 17 Oct 2026).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { hintsOn, publicSlots, publicNoteDue, takePublicNote, PUBLIC_NOTE } from "../src/tutor.js";
import { loadPrefs, savePrefs } from "../src/prefs.js";

test("hints are on when Hints is set and the Kibitzer is not muted (muting is the tutor's)", () => {
  assert.equal(hintsOn({ hints: true, tutor: true, muted: false }), true);
  assert.equal(hintsOn({ hints: false, tutor: true, muted: false }), false);
  assert.equal(hintsOn({ hints: true, tutor: true, muted: true }), false);
  assert.equal(hintsOn({ hints: true, tutor: false, muted: true }), true, "with the tutor off the mute does not apply");
});

test("the public marks are drawn only with hints on, on both hands", () => {
  const slots = [
    { zone: "your-hand", index: 0, public: true },
    { zone: "their-hand", index: 2, public: true },
    { zone: "your-hand", index: 1, public: false },
    { zone: "table", index: 0, public: true },
  ];
  assert.deepEqual(publicSlots(slots, { hints: false, tutor: true, muted: false }), []);
  assert.deepEqual(publicSlots(slots, { hints: true, tutor: true, muted: true }), []);
  assert.deepEqual(publicSlots(slots, { hints: true, tutor: true, muted: false }), [slots[0], slots[1]]);
});

test("the note on the gold dot is due once, when hints are first on", () => {
  assert.equal(publicNoteDue({ hints: false, tutor: true, muted: false, seen: [] }), false);
  assert.equal(publicNoteDue({ hints: true, tutor: true, muted: true, seen: [] }), false);
  assert.equal(publicNoteDue({ hints: true, tutor: true, muted: false, seen: [] }), true);
  assert.equal(publicNoteDue({ hints: true, tutor: true, muted: false, seen: [PUBLIC_NOTE] }), false, "never again once seen");
});

test("the note is kept as seen (persisted) and is not due again after a reload", () => {
  const data = new Map();
  const store = { getItem: (k) => data.get(k) ?? null, setItem: (k, v) => data.set(k, v), removeItem: (k) => data.delete(k) };
  let prefs = { ...loadPrefs(store), hints: true };
  const patch = (p) => {
    prefs = { ...prefs, ...p };
    savePrefs(prefs, store);
  };
  assert.equal(takePublicNote(() => prefs, patch), true);
  assert.equal(takePublicNote(() => prefs, patch), false);
  const reloaded = loadPrefs(store);
  assert.deepEqual(reloaded.seen, [PUBLIC_NOTE]);
  assert.equal(publicNoteDue({ ...reloaded, hints: true }), false);
});

test("the note and the tip are in both languages", () => {
  const s = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
  for (const k of ["ui.public.tip", "ui.public.note.title", "ui.public.note"]) assert.ok(s[k]?.en && s[k]?.de, k);
});
