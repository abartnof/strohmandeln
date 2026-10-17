import test from "node:test";
import assert from "node:assert/strict";
import { escapeTarget } from "../src/overlays.js";

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

test("the topmost dialog is the last opened", () => {
  assert.deepEqual(at({ dialogs: ["page", "help"] }), { kind: "dialog", id: "help" });
  assert.deepEqual(at({ dialogs: ["clear"], settings: true }), { kind: "dialog", id: "clear" });
});

test("a dialog that needs an answer is never dismissed, and holds back what is beneath it", () => {
  for (const id of ["extend", "over", "welcome"]) {
    assert.equal(at({ dialogs: [id], slate: true, chosen: true }), null, id);
  }
  assert.equal(at({ dialogs: ["new"] }), null);
  assert.deepEqual(at({ dialogs: ["new"], canCancelNew: true }), { kind: "dialog", id: "new" });
});

test("dialogs with a way back or a single Close are dismissed", () => {
  for (const id of ["page", "help", "review", "keys", "announce", "clear", "error"]) {
    assert.deepEqual(at({ dialogs: [id] }), { kind: "dialog", id });
  }
});
