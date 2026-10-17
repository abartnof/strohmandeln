// The new-sitting screen's body (docs/TABLE3D.md 3.5.1): the length, the
// three rule settings, the opponent's skill, its style and the seat: the sitting's own
// choices, offered here and nowhere else (the settings menu only reads the
// sitting back, 7.1). It starts from the sitting on the table, else from the
// kept prefs. The page saves what was chosen when Deal is pressed (3.5.2).
import { h } from "./dom.js";
import { BOTH_PASS, VALUES, ULTIMO, TOUREN, SKILLS } from "./prefs.js";

// "2/3/4" -> "v234" (a key's tail without slashes).
const valuesKey = (v) => `v${v.replaceAll("/", "")}`;
const row = (key, list, label = (v) => String(v), noteKey) => ({
  key,
  labelKey: `settings.${key}`,
  options: list.map((value) => ({ value, labelKey: `settings.${key}.${label(value)}` })),
  noteKey,
});
const ROWS = [
  row("touren", TOUREN),
  row("both_pass", BOTH_PASS, (v) => v, "settings.both_pass.note"),
  row("values", VALUES, valuesKey, "settings.values.note"),
  row("ultimo", ULTIMO, (v) => v, "settings.ultimo.note"),
  row("skill", SKILLS),
];

// In the order of 3.5.1.
export const NEW_KEYS = Object.freeze(["touren", "both_pass", "values", "ultimo", "skill", "style"]);

// The Style row: a native select (Normal, Random, the offered styles: engine.styles()), a line under it for
// the choice (docs/STYLES.md 5), and at skills 3.5 and 4 a note (STYLES 3).
const styleRow = (offered) => ({ key: "style", kind: "select", labelKey: "settings.style", options: ["normal", "random", ...offered].map((value) => ({ value, labelKey: `style.${value}.name`, lineKey: `style.${value}.line` })) });
const BIDDING_SKILLS = [3.5, 4];
const showsInBidding = (chosen) => BIDDING_SKILLS.includes(chosen.skill) && chosen.style !== "normal";

// The rows for these choices (the prefs' keys): { key, labelKey, options: [{ value, labelKey, selected }] }.
export function newSittingModel(prefs, offered) {
  return NEW_KEYS.map((key) => {
    const row = key === "style" ? styleRow(offered) : ROWS.find((r) => r.key === key);
    // a kept style that is not (or no longer) offered reads as Normal
    const current = key === "style" && !row.options.some((o) => o.value === prefs.style) ? "normal" : prefs[key];
    const long = key === "touren"; // "3 Touren (6 deals)" here, "3 Touren" in the menu
    return {
      key,
      ...(row.kind ? { kind: row.kind } : {}),
      labelKey: long ? "ui.new.length" : row.labelKey,
      options: row.options.map((o) => ({ value: o.value, labelKey: long ? `ui.new.length.${o.value}` : o.labelKey, ...(o.lineKey ? { lineKey: o.lineKey } : {}), selected: o.value === current })),
    };
  });
}

// Draws the body into a fresh element of `doc`; `values()` is what is chosen.
export function createNewSitting(doc, { t, prefs, sitting = null, styles }) {
  const model = newSittingModel({ ...prefs, ...sitting }, styles);
  const chosen = Object.fromEntries(model.map((r) => [r.key, r.options.find((o) => o.selected)?.value ?? r.options[0].value]));
  const styleOptions = model.find((r) => r.key === "style").options;
  const buttons = [];

  const styleLine = h(doc, "p", { class: "new-note style-line", "data-role": "style-line" });
  const styleNote = h(doc, "p", { class: "new-note style-note", "data-role": "style-note" });
  const showStyle = () => {
    const o = styleOptions.find((x) => x.value === chosen.style);
    styleLine.textContent = o ? t(o.lineKey) : "";
    styleNote.textContent = showsInBidding(chosen) ? t("style.note.bidding") : "";
  };

  const drawSelect = (row) => {
    const id = `new-${row.key}`;
    return h(doc, "div", { class: "new-row", "data-row": row.key },
      h(doc, "span", { class: "new-label", id }, t(row.labelKey)),
      h(doc, "select", {
        class: "settings-select",
        "data-key": row.key,
        "aria-labelledby": id,
        onchange: (ev) => {
          chosen[row.key] = ev.target.value;
          showStyle();
        },
      }, row.options.map((o) => h(doc, "option", { value: String(o.value), selected: o.value === chosen[row.key] ? true : null }, t(o.labelKey)))),
      styleLine,
      styleNote);
  };

  const draw = (row) => {
    if (row.kind === "select") return drawSelect(row);
    const id = `new-${row.key}`;
    return h(doc, "div", { class: "new-row", "data-row": row.key },
      h(doc, "span", { class: "new-label", id }, t(row.labelKey)),
      h(doc, "div", { class: "choice", role: "radiogroup", "aria-labelledby": id },
        row.options.map((o) => {
          const on = o.value === chosen[row.key];
          const b = h(doc, "button", {
            class: `big${on ? " on" : ""}`,
            type: "button",
            role: "radio",
            "aria-checked": String(on),
            "data-key": row.key,
            "data-value": String(o.value),
            "data-length": row.key === "touren" ? String(o.value) : null,
          }, t(o.labelKey));
          b.addEventListener("click", () => {
            chosen[row.key] = o.value;
            for (const x of buttons.filter((y) => y.getAttribute("data-key") === row.key)) {
              const mine = x === b;
              x.classList.toggle("on", mine);
              x.setAttribute("aria-checked", String(mine));
            }
            showStyle();
          });
          buttons.push(b);
          return b;
        })));
  };

  const node = h(doc, "div", { class: "new" },
    h(doc, "p", {}, t("ui.new.helper")),
    model.map(draw),
    h(doc, "div", { class: "new-row", "data-row": "hand" },
      h(doc, "span", { class: "new-label" }, t("ui.new.hand")),
      h(doc, "span", { class: "new-fixed" }, t("ui.new.hand.lot"))),
    h(doc, "p", { class: "new-note" }, t("ui.new.kept")));

  showStyle();
  return { node, values: () => ({ ...chosen }) };
}
