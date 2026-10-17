import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createNewSitting, NEW_KEYS, newSittingModel } from "../src/newsitting.js";
import { DEFAULTS, OFFERED_STYLES } from "../src/prefs.js";
import { createStrings } from "../src/strings.js";
import { fakeRoot } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

// 3.5.1: length, the three rule settings, the opponent's skill, the seat.
test("the screen offers the length, the three rules and the skill, in that order, with the saved choice selected", () => {
  assert.deepEqual(NEW_KEYS, ["touren", "both_pass", "values", "ultimo", "skill", "style"]);
  const m = newSittingModel({ ...DEFAULTS, skill: 3.5, ultimo: 2 });
  assert.deepEqual(m.map((r) => r.key), NEW_KEYS);
  const selected = Object.fromEntries(m.map((r) => [r.key, r.options.filter((o) => o.selected).map((o) => o.value)]));
  assert.deepEqual(selected, { touren: [3], both_pass: ["play"], values: ["2/3/4"], ultimo: [2], skill: [3.5], style: ["normal"] });
  assert.equal(m[0].options[0].labelKey, "ui.new.length.3");
});

test("every key the screen uses is in both languages", () => {
  const keys = ["ui.new.helper", "ui.new.hand", "ui.new.hand.lot", "ui.new.kept", "style.note.bidding"];
  for (const r of newSittingModel(DEFAULTS)) keys.push(r.labelKey, ...r.options.flatMap((o) => [o.labelKey, o.lineKey ?? o.labelKey]));
  for (const k of keys) {
    assert.ok(TABLE[k], k);
    assert.ok(TABLE[k].en && TABLE[k].de, k);
  }
});

test("a press chooses within its row only; values() is what was chosen", () => {
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
  assert.deepEqual(ns.values(), { touren: 3, both_pass: "play", values: "2/3/4", ultimo: 1, skill: 3, style: "normal" });
  const button = (key, value) => ns.node.findAll((e) => e.getAttribute("data-key") === key && e.getAttribute("data-value") === String(value))[0];
  button("touren", 6).click();
  button("both_pass", "throw_in").click();
  button("skill", 2).click();
  assert.deepEqual(ns.values(), { touren: 6, both_pass: "throw_in", values: "2/3/4", ultimo: 1, skill: 2, style: "normal" });
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
  const sitting = { touren: 6, both_pass: "throw_in", values: "1/2/3", ultimo: 2, skill: 2, style: "bold" };
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

test("the skill row offers 1, 2, 3, 3½, 4 and no other half step", () => {
  const skill = newSittingModel(DEFAULTS).find((r) => r.key === "skill");
  assert.deepEqual(skill.options.map((o) => o.value), [1, 2, 3, 3.5, 4]);
});

test("the Style row is a native select: Normal, Random, then the offered styles, from one list", () => {
  const style = newSittingModel(DEFAULTS).find((r) => r.key === "style");
  assert.equal(style.kind, "select");
  assert.deepEqual(style.options.map((o) => o.value), ["normal", "random", ...OFFERED_STYLES]);
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
  const sel = ns.node.findAll((e) => e.tagName === "SELECT" && e.getAttribute("data-key") === "style");
  assert.equal(sel.length, 1);
  assert.deepEqual(sel[0].findAll((e) => e.tagName === "OPTION").map((o) => o.textContent),
    ["Normal", "Random", "After Bermann", "After Mayr and Sedlaczek", "Bold", "Cautious", "Pagat hunter"]);
});

test("each style has a one-line description under the select, which follows the choice, in both languages", () => {
  for (const lang of ["en", "de"]) {
    const s = createStrings(TABLE, lang);
    const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS } });
    const line = () => ns.node.findAll((e) => e.getAttribute("data-role") === "style-line")[0].textContent;
    assert.equal(line(), TABLE["style.normal.line"][lang]);
    const sel = ns.node.findAll((e) => e.tagName === "SELECT")[0];
    for (const id of ["normal", "random", ...OFFERED_STYLES]) {
      sel.dispatch("change", { target: { value: id } });
      assert.equal(ns.values().style, id);
      assert.equal(line(), TABLE[`style.${id}.line`][lang], id);
      assert.ok(line().length > 10 && !/\n/.test(line()));
    }
  }
});

test("at skill 3½ and 4 a note says the style shows mostly in the bidding (not under Normal)", () => {
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS, style: "bermann" } });
  const note = () => ns.node.findAll((e) => e.getAttribute("data-role") === "style-note")[0].textContent;
  const button = (value) => ns.node.findAll((e) => e.getAttribute("data-key") === "skill" && e.getAttribute("data-value") === String(value))[0];
  assert.equal(note(), "");
  button(3.5).click();
  assert.match(note(), /mostly in the bidding/);
  button(4).click();
  assert.match(note(), /mostly in the bidding/);
  button(2).click();
  assert.equal(note(), "");
  button(4).click();
  ns.node.findAll((e) => e.tagName === "SELECT")[0].dispatch("change", { target: { value: "normal" } });
  assert.equal(note(), "", "Normal has no style to show");
});

test("the style starts from the sitting on the table, else from the prefs", () => {
  const s = createStrings(TABLE, "en");
  const doc = fakeRoot().ownerDocument;
  const sel = (ns) => ns.node.findAll((e) => e.tagName === "OPTION" && e.getAttribute("selected") !== null).map((o) => o.getAttribute("value"));
  const sitting = { touren: 3, both_pass: "play", values: "2/3/4", ultimo: 1, skill: 3, style: "random" };
  assert.deepEqual(sel(createNewSitting(doc, { t: s.t, prefs: { ...DEFAULTS, style: "bold" }, sitting })), ["random"]);
  assert.deepEqual(sel(createNewSitting(doc, { t: s.t, prefs: { ...DEFAULTS, style: "bold" } })), ["bold"]);
});

test("the Style row follows the list it is given; a kept style that is not in it is Normal", () => {
  const model = newSittingModel({ ...DEFAULTS, style: "hunter" }, ["bold", "ms"]);
  const style = model.find((r) => r.key === "style");
  assert.deepEqual(style.options.map((o) => o.value), ["normal", "random", "bold", "ms"]);
  assert.deepEqual(style.options.filter((o) => o.selected).map((o) => o.value), ["normal"]);
  const s = createStrings(TABLE, "en");
  const ns = createNewSitting(fakeRoot().ownerDocument, { t: s.t, prefs: { ...DEFAULTS, style: "hunter" }, styles: ["bold", "ms"] });
  assert.equal(ns.values().style, "normal");
});
