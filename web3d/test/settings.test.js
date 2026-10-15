import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { settingsModel, createSettings, ROWS } from "../src/settings.js";
import { DEFAULTS } from "../src/prefs.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

const SPEC_ROWS = ["both_pass", "values", "ultimo", "touren", "skill", "speed", "talk", "tutor", "hints", "tracker", "explain", "announceAuto", "slateAuto", "faces", "leftHanded", "reducedMotion", "lang", "surface"];

test("every setting of spec 7.1 has a row, and every default is an option", () => {
  assert.deepEqual(ROWS.map((r) => r.key).sort(), [...SPEC_ROWS].sort());
  const m = settingsModel({ ...DEFAULTS }, {});
  const rows = m.sections.flatMap((s) => s.rows);
  for (const r of rows) {
    if (r.kind === "choice") assert.ok(r.options.some((o) => o.selected), `${r.key} has a selected option`);
  }
});

test("every label and note key exists in both languages", () => {
  const m = settingsModel({ ...DEFAULTS }, {});
  const keys = [m.titleKey];
  for (const s of m.sections) {
    keys.push(s.titleKey);
    for (const r of s.rows) {
      keys.push(r.labelKey);
      if (r.noteKey) keys.push(r.noteKey);
      for (const o of r.options ?? []) if (o.labelKey) keys.push(o.labelKey);
    }
    for (const a of s.actions ?? []) keys.push(a.labelKey);
  }
  for (const k of keys) assert.ok(TABLE[k], k);
});

test("the three rules and the length apply from the next sitting; the rest now", () => {
  const rows = Object.fromEntries(ROWS.map((r) => [r.key, r.applies]));
  for (const k of ["both_pass", "values", "ultimo", "touren", "skill"]) assert.equal(rows[k], "next", k);
  for (const k of ["speed", "talk", "tutor", "hints", "tracker", "explain", "announceAuto", "slateAuto", "faces", "leftHanded", "reducedMotion", "lang", "surface"]) assert.equal(rows[k], "now", k);
});

test("in a sitting the next-sitting rows are locked", () => {
  const m = settingsModel({ ...DEFAULTS }, { inSitting: true });
  const rows = Object.fromEntries(m.sections.flatMap((s) => s.rows).map((r) => [r.key, r]));
  assert.equal(rows.values.locked, true);
  assert.equal(rows.speed.locked, false);
  assert.equal(settingsModel({ ...DEFAULTS }, {}).sections[0].rows[0].locked, false);
});

test("the surface row offers random and the kit's surfaces", () => {
  const m = settingsModel({ ...DEFAULTS }, { surfaces: [{ value: "green", label: "Green baize" }] });
  const row = m.sections.flatMap((s) => s.rows).find((r) => r.key === "surface");
  assert.deepEqual(row.options.map((o) => o.value), ["random", "green"]);
  assert.equal(row.options[1].label, "Green baize");
});

function made(prefs = {}, opts = {}) {
  const s = createStrings(TABLE);
  const root = fakeRoot();
  const calls = [];
  const actions = [];
  const menu = createSettings(root, { t: s.t, prefs: { ...DEFAULTS, ...prefs }, onChange: (k, v) => calls.push([k, v]), onAction: (a) => actions.push(a), onLang: s.onLang, ...opts });
  return { s, root, menu, calls, actions };
}
const button = (root, key, value) => root.findAll((e) => e.tagName === "BUTTON" && e.getAttribute("data-key") === key && e.getAttribute("data-value") === String(value))[0];

test("choosing an option reports it and selects it", () => {
  const { root, calls } = made();
  const fast = button(root, "speed", "fast");
  assert.equal(fast.getAttribute("aria-checked"), "false");
  fast.click();
  assert.deepEqual(calls, [["speed", "fast"]]);
  assert.equal(button(root, "speed", "fast").getAttribute("aria-checked"), "true");
  assert.equal(button(root, "speed", "normal").getAttribute("aria-checked"), "false");
});

test("values keep their type: numbers stay numbers", () => {
  const { root, calls } = made();
  button(root, "skill", "2.5").click();
  button(root, "touren", "6").click();
  button(root, "ultimo", "2").click();
  button(root, "values", "1/2/4").click();
  assert.deepEqual(calls, [["skill", 2.5], ["touren", 6], ["ultimo", 2], ["values", "1/2/4"]]);
});

test("a toggle flips", () => {
  const { root, calls } = made();
  const sw = button(root, "hints", "toggle");
  assert.equal(sw.getAttribute("aria-checked"), "false");
  sw.click();
  assert.deepEqual(calls, [["hints", true]]);
  assert.equal(button(root, "hints", "toggle").getAttribute("aria-checked"), "true");
  button(root, "hints", "toggle").click();
  assert.deepEqual(calls[1], ["hints", false]);
});

test("a locked row ignores a click and says when it applies", () => {
  const { root, menu, calls } = made();
  menu.setSitting(true);
  button(root, "values", "1/2/3").click();
  assert.deepEqual(calls, []);
  assert.match(root.textContent, /from the next sitting/);
  menu.setSitting(false);
  button(root, "values", "1/2/3").click();
  assert.deepEqual(calls, [["values", "1/2/3"]]);
});

test("set updates the view and does not report", () => {
  const { root, menu, calls } = made();
  menu.set("lang", "de");
  assert.deepEqual(calls, []);
  assert.equal(button(root, "lang", "de").getAttribute("aria-checked"), "true");
});

test("a switch of language redraws the menu", () => {
  const { s, root } = made();
  assert.match(root.textContent, /Speed/);
  s.setLang("de");
  assert.match(root.textContent, /Tempo/);
  assert.match(root.textContent, /Einstellungen/);
});

test("open and close, with Escape", () => {
  const { root, menu } = made();
  assert.equal(root.hidden, true);
  menu.open();
  assert.equal(root.hidden, false);
  root.dispatch("keydown", { key: "Escape" });
  assert.equal(root.hidden, true);
  menu.open();
  menu.close();
  assert.equal(root.hidden, true);
});

test("the sitting actions are reported", () => {
  const { root, actions } = made();
  const names = ["save", "load", "progress-export", "progress-clear"];
  for (const n of names) root.findAll((e) => e.getAttribute("data-action") === n)[0].click();
  assert.deepEqual(actions, names);
});

test("each choice is a radio in a labelled group", () => {
  const { root } = made();
  const group = root.findAll((e) => e.getAttribute("role") === "radiogroup" && e.getAttribute("data-key") === "speed")[0];
  assert.ok(group.getAttribute("aria-labelledby") || group.getAttribute("aria-label"));
  assert.equal(group.findAll((e) => e.getAttribute("role") === "radio").length, 4);
});
