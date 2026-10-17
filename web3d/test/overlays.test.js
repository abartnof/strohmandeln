import test from "node:test";
import assert from "node:assert/strict";
import { escapeTarget, selectGuard } from "../src/overlays.js";

const none = { dialogs: [], settings: false, slate: false, hud: false, tracker: false, chosen: false, canCancelNew: false };
const at = (over) => escapeTarget({ ...none, ...over });

test("nothing open: nothing to do (the slate is not folded, nothing is opened)", () => {
  assert.equal(at({}), null);
});

test("one layer per press, topmost first: dialogs, settings, slate, score list, tracker, selection", () => {
  const closes = { dialog: { dialogs: [] }, settings: { settings: false }, slate: { slate: false }, hud: { hud: false }, tracker: { tracker: false }, selection: { chosen: false } };
  let s = { ...none, dialogs: ["help"], settings: true, slate: true, hud: true, tracker: true, chosen: true };
  const order = [];
  for (let i = 0; i < 6; i++) {
    const got = escapeTarget(s);
    order.push(got.kind);
    s = { ...s, ...closes[got.kind] };
  }
  assert.deepEqual(order, ["dialog", "settings", "slate", "hud", "tracker", "selection"]);
  assert.equal(escapeTarget(s), null);
});

test("an open Why? closes first, then the layers beneath it; a dialog still comes before it", () => {
  assert.deepEqual(at({ why: true }), { kind: "why" });
  assert.deepEqual(at({ why: true, settings: true, slate: true, tracker: true, chosen: true }), { kind: "why" });
  assert.deepEqual(at({ why: true, dialogs: ["help"] }), { kind: "dialog", id: "help" });
  assert.deepEqual(at({ settings: true }), { kind: "settings" });
});

test("the topmost dialog is the last opened", () => {
  assert.deepEqual(at({ dialogs: ["page", "help"] }), { kind: "dialog", id: "help" });
  assert.deepEqual(at({ dialogs: ["clear"], settings: true }), { kind: "dialog", id: "clear" });
});

test("a dialog that needs an answer is never dismissed: Escape moves focus to its default, and holds back what is beneath", () => {
  for (const id of ["extend", "over"]) {
    assert.deepEqual(at({ dialogs: [id], slate: true, chosen: true }), { kind: "focus", id }, id);
  }
  assert.deepEqual(at({ dialogs: ["new"] }), { kind: "focus", id: "new" });
  assert.deepEqual(at({ dialogs: ["new"], canCancelNew: true }), { kind: "dialog", id: "new" });
});

test("the welcome acts as its default (Continue): nothing is lost", () => {
  assert.deepEqual(at({ dialogs: ["welcome"] }), { kind: "dialog", id: "welcome" });
});

test("the table's note is a layer", () => {
  assert.deepEqual(at({ dialogs: ["note"] }), { kind: "dialog", id: "note" });
});

test("an open native select takes its own Escape; a closed one does not", () => {
  const g = selectGuard();
  const sel = { tagName: "SELECT" };
  const other = { tagName: "BUTTON" };
  assert.equal(g.swallows({ type: "keydown", key: "Escape", target: sel }), false); // closed
  g.observe({ type: "mousedown", target: sel }); // opened
  assert.equal(g.swallows({ type: "keydown", key: "Escape", target: sel }), true); // closes the select only
  assert.equal(g.swallows({ type: "keydown", key: "Escape", target: sel }), false); // the next one is the page's
  g.observe({ type: "keydown", key: "ArrowDown", altKey: true, target: sel });
  g.observe({ type: "change", target: sel });
  assert.equal(g.swallows({ type: "keydown", key: "Escape", target: sel }), false);
  g.observe({ type: "mousedown", target: other });
  assert.equal(g.swallows({ type: "keydown", key: "Escape", target: other }), false);
});

test("dialogs with a way back or a single Close are dismissed", () => {
  for (const id of ["page", "help", "review", "keys", "announce", "clear", "error"]) {
    assert.deepEqual(at({ dialogs: [id] }), { kind: "dialog", id });
  }
});
