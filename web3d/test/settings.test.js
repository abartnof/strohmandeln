import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { settingsModel, createSettings, sittingFrom, ROWS } from "../src/settings.js";
import { PATTERNS } from "../../kit/js/surfaces.js";
import { DEFAULTS } from "../src/prefs.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

const RULE_ROWS = ["both_pass", "values", "ultimo", "touren", "skill"];
const SPEC_ROWS = ["speed", "talk", "tutor", "tips", "hints", "tracker", "explain", "announceAuto", "slateAuto", "faces", "leftHanded", "reducedMotion", "lang", "surface"];

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

test("the menu holds none of the five sitting rows (3.5.2, 7.1)", () => {
  const keys = ROWS.map((r) => r.key);
  for (const k of RULE_ROWS) assert.ok(!keys.includes(k), k);
  assert.ok(!ROWS.some((r) => "applies" in r), "no applies column");
  for (const k of ["settings.mid-sitting", "settings.applies.next", "settings.applies.now"]) assert.ok(!TABLE[k], `${k} is gone`);
  const { root } = made();
  for (const k of RULE_ROWS) assert.equal(root.findAll((e) => e.getAttribute("data-row") === k).length, 0, k);
  assert.doesNotMatch(root.textContent, /from the next sitting/);
});

test("the sitting is read from the record's sitting_start and the engine's level", () => {
  assert.equal(sittingFrom([], 3), null);
  const events = [{ type: "sitting_start", first_dealer: 0, touren: 6, settings: { both_pass: "throw_in", ultimo: 2, values: "1/2/3" } }];
  assert.deepEqual(sittingFrom(events, 2.5), { touren: 6, both_pass: "throw_in", values: "1/2/3", ultimo: 2, skill: 2.5 });
  assert.deepEqual(sittingFrom({ touren: 6, settings: { both_pass: "throw_in", ultimo: 2, values: "1/2/3" } }, 2.5), sittingFrom(events, 2.5), "from an export's record too");
  assert.deepEqual(sittingFrom([{ type: "sitting_start", touren: 3 }], 1), { touren: 3, both_pass: "play", values: "2/3/4", ultimo: 1, skill: 1 });
});

test("the This-sitting line: hidden with no sitting; read from the sitting, in both languages", () => {
  const { s, root, menu } = made();
  const line = () => root.findAll((e) => e.getAttribute("data-row") === "this-sitting")[0];
  assert.equal(line(), undefined);
  menu.setSitting({ touren: 3, both_pass: "play", values: "2/3/4", ultimo: 1, skill: 3 });
  assert.match(line().textContent, /This sitting: 3 Touren, 2\/3\/4, ultimo 1, simple game, opponent level 3/);
  menu.setSitting({ touren: 6, both_pass: "throw_in", values: "1/2/4", ultimo: 2, skill: 1.5 });
  assert.match(line().textContent, /This sitting: 6 Touren, 1\/2\/4, ultimo 2, throw-in, opponent level 1½/);
  s.setLang("de");
  assert.match(line().textContent, /Dieser Abend: 6 Touren, 1\/2\/4, Ultimo 2, Einwerfen, Gegner Stufe 1½/);
  menu.setSitting(null);
  assert.equal(line(), undefined);
});

test("the surface row is a select: Random first, then the kit's 18, named in both languages", () => {
  const m = settingsModel({ ...DEFAULTS }, { surfaces: PATTERNS.map((p) => ({ value: p.id, label: p.name })) });
  const row = m.sections.flatMap((s) => s.rows).find((r) => r.key === "surface");
  assert.equal(row.kind, "select");
  assert.equal(row.options.length, 19);
  assert.deepEqual(row.options.map((o) => o.value), ["random", ...PATTERNS.map((p) => p.id)]);
  for (const o of row.options) assert.ok(TABLE[o.labelKey]?.en && TABLE[o.labelKey]?.de, o.labelKey);
  assert.equal(row.options.filter((o) => o.selected).length, 1);
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

test("the table top is a labelled native select; a change reports and shows", () => {
  const surfaces = PATTERNS.map((p) => ({ value: p.id, label: p.name }));
  const { root, calls, menu } = made({}, { surfaces });
  const sel = root.findAll((e) => e.tagName === "SELECT")[0];
  assert.equal(sel.getAttribute("data-key"), "surface");
  assert.ok(sel.getAttribute("aria-labelledby"));
  assert.equal(sel.findAll((e) => e.tagName === "OPTION").length, 19);
  assert.equal(root.findAll((e) => e.getAttribute("role") === "radio" && e.getAttribute("data-key") === "surface").length, 0);
  sel.dispatch("change", { target: { value: "dot-grid" } });
  assert.deepEqual(calls, [["surface", "dot-grid"]]);
  const chosen = root.findAll((e) => e.tagName === "OPTION" && e.getAttribute("selected") !== null);
  assert.deepEqual(chosen.map((o) => o.getAttribute("value")), ["dot-grid"]);
  menu.set("surface", "random");
  assert.equal(root.findAll((e) => e.tagName === "OPTION" && e.getAttribute("selected") !== null)[0].getAttribute("value"), "random");
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

test("open and close (Escape is the page's, overlays.js)", () => {
  const { root, menu } = made();
  assert.equal(root.hidden, true);
  menu.open();
  assert.equal(root.hidden, false);
  root.dispatch("keydown", { key: "Escape" });
  assert.equal(root.hidden, false);
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
