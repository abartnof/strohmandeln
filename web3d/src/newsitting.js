// The new-sitting screen's body (docs/TABLE3D.md 3.5.1): the length, the
// three rule settings, the opponent's skill and the seat: the sitting's own
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
export const NEW_KEYS = Object.freeze(["touren", "both_pass", "values", "ultimo", "skill"]);

// The rows for these choices (the prefs' keys): { key, labelKey, options: [{ value, labelKey, selected }] }.
export function newSittingModel(prefs) {
  return NEW_KEYS.map((key) => {
    const row = ROWS.find((r) => r.key === key);
    const long = key === "touren"; // "3 Touren (6 deals)" here, "3 Touren" in the menu
    return {
      key,
      labelKey: long ? "ui.new.length" : row.labelKey,
      options: row.options.map((o) => ({ value: o.value, labelKey: long ? `ui.new.length.${o.value}` : o.labelKey, selected: o.value === prefs[key] })),
    };
  });
}

// Draws the body into a fresh element of `doc`; `values()` is what is chosen.
export function createNewSitting(doc, { t, prefs, sitting = null }) {
  const model = newSittingModel({ ...prefs, ...sitting });
  const chosen = Object.fromEntries(model.map((r) => [r.key, r.options.find((o) => o.selected)?.value ?? r.options[0].value]));
  const buttons = [];

  const draw = (row) => {
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

  return { node, values: () => ({ ...chosen }) };
}
