// The settings menu (docs/TABLE3D.md 7.1 to 7.6). The menu is described as
// data (settingsModel), then drawn (createSettings), so the arrangement is
// testable without a DOM. It holds the person's choices and reports each one
// to the page through onChange(key, value); the page saves them (prefs.js)
// and acts on them. It adds no variants: the rules offered are the three of
// docs/RULES.md and nothing else (CLAUDE.md).
import { h } from "./dom.js";
import { BOTH_PASS, VALUES, ULTIMO, TOUREN, SKILLS, SPEEDS, TALKS, FACES, MOTIONS, LANGS } from "./prefs.js";
import { onLang as sharedOnLang } from "./strings.js";

// "2/3/4" -> "v234" (a key's tail without slashes).
const valuesKey = (v) => `v${v.replaceAll("/", "")}`;

const choice = (key, applies, list, label = (v) => String(v), extra = {}) => ({
  key,
  kind: "choice",
  applies,
  labelKey: `settings.${key}`,
  options: list.map((value) => ({ value, labelKey: `settings.${key}.${label(value)}` })),
  ...extra,
});
const toggle = (key, extra = {}) => ({ key, kind: "toggle", applies: "now", labelKey: `settings.${key}`, ...extra });

// Every setting of 7.1, in the sections it is shown in.
export const SECTIONS = [
  {
    id: "rules",
    titleKey: "settings.section.rules",
    rows: [
      choice("both_pass", "next", BOTH_PASS, (v) => v, { noteKey: "settings.both_pass.note" }),
      choice("values", "next", VALUES, valuesKey, { noteKey: "settings.values.note" }),
      choice("ultimo", "next", ULTIMO, (v) => v, { noteKey: "settings.ultimo.note" }),
      choice("touren", "next", TOUREN),
    ],
  },
  { id: "opponent", titleKey: "settings.section.opponent", rows: [choice("skill", "next", SKILLS)] },
  {
    id: "table",
    titleKey: "settings.section.table",
    rows: [
      choice("speed", "now", SPEEDS),
      choice("faces", "now", FACES),
      toggle("leftHanded"),
      choice("reducedMotion", "now", MOTIONS),
      // The kit's table tops: "random" and whatever the page lists.
      { key: "surface", kind: "choice", applies: "now", labelKey: "settings.surface", options: [{ value: "random", labelKey: "settings.surface.random" }], surfaces: true },
    ],
  },
  {
    id: "aids",
    titleKey: "settings.section.aids",
    rows: [
      choice("talk", "now", TALKS),
      toggle("tutor", { noteKey: "settings.tutor.note" }),
      toggle("tips", { noteKey: "settings.tips.note" }),
      toggle("hints"),
      toggle("tracker", { noteKey: "settings.tracker.note" }),
      toggle("explain"),
      toggle("announceAuto", { noteKey: "settings.announceAuto.note" }),
      toggle("slateAuto"),
    ],
  },
  { id: "page", titleKey: "settings.section.page", rows: [choice("lang", "now", LANGS)] },
  {
    id: "sitting",
    titleKey: "settings.section.sitting",
    rows: [],
    actions: [
      { name: "save", labelKey: "settings.save" },
      { name: "load", labelKey: "settings.load" },
      { name: "progress-export", labelKey: "settings.progress.export" },
      { name: "progress-import", labelKey: "settings.progress.import" },
      { name: "progress-clear", labelKey: "settings.progress.clear" },
    ],
  },
];

export const ROWS = SECTIONS.flatMap((s) => s.rows);

// The menu for these preferences: `inSitting` locks the rows that apply from
// the next sitting (7.1); `surfaces` is [{value, label}] from the kit.
export function settingsModel(prefs, { inSitting = false, surfaces = [] } = {}) {
  return {
    titleKey: "settings.title",
    sections: SECTIONS.map((s) => ({
      ...s,
      rows: s.rows.map((r) => {
        const options = (r.surfaces ? [...r.options, ...surfaces.map((x) => ({ value: x.value, label: x.label }))] : r.options ?? []).map((o) => ({ ...o, selected: o.value === prefs[r.key] }));
        return { ...r, options, value: prefs[r.key], locked: inSitting && r.applies === "next" };
      }),
    })),
  };
}

// rootEl: where the menu is drawn (hidden until opened). t: the strings.
// onChange(key, value): a choice made. onAction(name): "save", "load",
// "progress-export", "progress-import" or "progress-clear". onLang(fn): redraw on a switch.
export function createSettings(rootEl, { t, prefs, onChange = () => {}, onAction = () => {}, onLang = sharedOnLang, surfaces = [] }) {
  const doc = rootEl.ownerDocument;
  let current = { ...prefs };
  let inSitting = false;
  let surfaceList = surfaces;

  rootEl.classList.add("settings");
  rootEl.hidden = true;
  rootEl.setAttribute("aria-hidden", "true");
  rootEl.setAttribute("role", "dialog");

  function choose(row, value) {
    if (row.locked || current[row.key] === value) return;
    current = { ...current, [row.key]: value };
    onChange(row.key, value);
    render();
  }

  function drawRow(row) {
    const id = `settings-${row.key}`;
    const labelText = t(row.labelKey);
    const when = row.locked ? t("settings.applies.next") : null;
    let control;
    if (row.kind === "toggle") {
      control = h(doc, "button", {
        class: "settings-switch",
        type: "button",
        role: "switch",
        "data-key": row.key,
        "data-value": "toggle",
        "aria-checked": row.value ? "true" : "false",
        "aria-labelledby": id,
        onclick: () => choose(row, !row.value),
      }, row.value ? t("settings.on") : t("settings.off"));
    } else {
      control = h(doc, "div", { class: "settings-options", role: "radiogroup", "data-key": row.key, "aria-labelledby": id },
        row.options.map((o) =>
          h(doc, "button", {
            class: `settings-option${o.selected ? " settings-selected" : ""}`,
            type: "button",
            role: "radio",
            "data-key": row.key,
            "data-value": String(o.value),
            "aria-checked": o.selected ? "true" : "false",
            disabled: row.locked ? true : null,
            onclick: () => choose(row, o.value),
          }, o.label ?? t(o.labelKey))));
    }
    return h(doc, "div", { class: `settings-row${row.locked ? " settings-locked" : ""}`, "data-row": row.key },
      h(doc, "span", { class: "settings-label", id }, labelText),
      control,
      row.noteKey ? h(doc, "span", { class: "settings-note" }, t(row.noteKey)) : null,
      when ? h(doc, "span", { class: "settings-when" }, when) : null);
  }

  function render() {
    const m = settingsModel(current, { inSitting, surfaces: surfaceList });
    rootEl.replaceChildren(
      h(doc, "div", { class: "settings-sheet" },
        h(doc, "div", { class: "settings-head" },
          h(doc, "h2", {}, t(m.titleKey)),
          h(doc, "button", { class: "settings-close", type: "button", "aria-label": t("ui.close"), onclick: () => close() }, "×")),
        m.sections.map((s) =>
          h(doc, "section", { class: "settings-section", "data-section": s.id },
            h(doc, "h3", {}, t(s.titleKey)),
            s.rows.map(drawRow),
            (s.actions ?? []).map((a) => h(doc, "button", { class: "settings-action", type: "button", "data-action": a.name, onclick: () => onAction(a.name) }, t(a.labelKey)))))));
  }

  function open() {
    rootEl.hidden = false;
    rootEl.setAttribute("aria-hidden", "false");
    const first = rootEl.findAll ? rootEl.findAll((e) => e.getAttribute("role") === "radio" && e.getAttribute("aria-checked") === "true")[0] : rootEl.querySelector?.('[role="radio"][aria-checked="true"]');
    first?.focus?.();
  }
  function close() {
    rootEl.hidden = true;
    rootEl.setAttribute("aria-hidden", "true");
  }

  rootEl.addEventListener("keydown", (ev) => {
    if (ev.key === "Escape") close();
  });
  onLang(render);
  render();

  return {
    open,
    close,
    // Set a value from outside (the page changed it, or it was loaded): shown,
    // not reported.
    set(key, value) {
      current = { ...current, [key]: value };
      render();
    },
    // Whether a sitting is under way: the rules and the length are then shown
    // greyed, "from the next sitting" (3.5.2, 7.1).
    setSitting(on) {
      inSitting = Boolean(on);
      render();
    },
    setSurfaces(list) {
      surfaceList = list;
      render();
    },
    render,
  };
}
