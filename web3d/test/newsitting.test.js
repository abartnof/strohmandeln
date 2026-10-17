import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createNewSitting, NEW_KEYS, newSittingModel } from "../src/newsitting.js";
import { DEFAULTS } from "../src/prefs.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

// 3.5.1: length, the three rule settings, the opponent's skill, the seat.
test("the screen offers the length, the three rules and the skill, in that order, with the saved choice selected", () => {
  assert.deepEqual(NEW_KEYS, ["touren", "both_pass", "values", "ultimo", "skill"]);
  const m = newSittingModel({ ...DEFAULTS, skill: 2.5, ultimo: 2 });
  assert.deepEqual(m.map((r) => r.key), NEW_KEYS);
  const selected = Object.fromEntries(m.map((r) => [r.key, r.options.filter((o) => o.selected).map((o) => o.value)]));
  assert.deepEqual(selected, { touren: [3], both_pass: ["play"], values: ["2/3/4"], ultimo: [2], skill: [2.5] });
  assert.equal(m[0].options[0].labelKey, "ui.new.length.3");
});

test("every key the screen uses is in both languages", () => {
  const keys = ["ui.new.helper", "ui.new.hand", "ui.new.hand.lot", "ui.new.kept"];
  for (const r of newSittingModel(DEFAULTS)) keys.push(r.labelKey, ...r.options.map((o) => o.labelKey));
  for (const k of keys) {
    assert.ok(TABLE[k], k);
    assert.ok(TABLE[k].en && TABLE[k].de, k);
  }
});

test("a press chooses within its row only; values() is what was chosen", () => {
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
  assert.deepEqual(ns.values(), { touren: 3, both_pass: "play", values: "2/3/4", ultimo: 1, skill: 3 });
  const button = (key, value) => ns.node.findAll((e) => e.getAttribute("data-key") === key && e.getAttribute("data-value") === String(value))[0];
  button("touren", 6).click();
  button("both_pass", "throw_in").click();
  button("skill", 1.5).click();
  assert.deepEqual(ns.values(), { touren: 6, both_pass: "throw_in", values: "2/3/4", ultimo: 1, skill: 1.5 });
  assert.equal(button("touren", 6).getAttribute("aria-checked"), "true");
  assert.equal(button("touren", 3).getAttribute("aria-checked"), "false");
  assert.equal(button("values", "2/3/4").getAttribute("aria-checked"), "true", "other rows are untouched");
  assert.equal(button("touren", 6).getAttribute("data-length"), "6", "the length keeps its hook");
});

test("the seat is by lot, and shown as such", () => {
  const s = createStrings(TABLE, "de");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
  const text = ns.node.textContent;
  assert.ok(text.includes(s.t("ui.new.hand")) && text.includes(s.t("ui.new.hand.lot")));
  assert.ok(text.includes(s.t("settings.skill")), "the skill row is labelled in German");
});

test("the screen starts from the sitting on the table, else from the prefs", () => {
  const s = createStrings(TABLE, "en");
  const doc = fakeRoot().ownerDocument;
  const sitting = { touren: 6, both_pass: "throw_in", values: "1/2/3", ultimo: 2, skill: 1.5 };
  const ns = createNewSitting(doc, { t: s.t, prefs: { ...DEFAULTS }, sitting });
  assert.deepEqual(ns.values(), sitting);
  const from = createNewSitting(doc, { t: s.t, prefs: { ...DEFAULTS, skill: 2 }, sitting: null });
  assert.equal(from.values().skill, 2);
});

test("the rules rows are on the screen, not behind a disclosure; the note no longer points at Settings", () => {
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
  assert.equal(ns.node.findAll((e) => e.tagName === "DETAILS").length, 0);
  for (const k of NEW_KEYS) assert.equal(ns.node.findAll((e) => e.getAttribute("data-row") === k).length, 1, k);
  assert.doesNotMatch(TABLE["ui.new.kept"].en + TABLE["ui.new.kept"].de, /Settings|Einstellungen/);
});
