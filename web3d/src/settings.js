// The settings menu (docs/TABLE3D.md 7.1 to 7.6). The menu is described as
// data (settingsModel), then drawn (createSettings), so the arrangement is
// testable without a DOM. It holds the aids and the appearance, each taking
// effect at once, and reports each choice to the page through onChange(key,
// value); the page saves them (prefs.js) and acts on them. The sitting's own
// choices (the length, the three rules, the opponent's level) are the
// new-sitting screen's (newsitting.js); here a read-only line says what the
// sitting on the table is (3.5.2, 7.1).
import { h } from "./dom.js";
import { SPEEDS, TALKS, FACES, MOTIONS, LANGS, DEFAULTS } from "./prefs.js";
import { onLang as sharedOnLang } from "./strings.js";

const choice = (key, list, label = (v) => String(v), extra = {}) => ({
  key,
  kind: "choice",
  labelKey: `settings.${key}`,
  options: list.map((value) => ({ value, labelKey: `settings.${key}.${label(value)}` })),
  ...extra,
});
const toggle = (key, extra = {}) => ({ key, kind: "toggle", labelKey: `settings.${key}`, ...extra });

// Every setting of 7.1, in the sections it is shown in.
export const SECTIONS = [
  {
    id: "table",
    titleKey: "settings.section.table",
    rows: [
      choice("speed", SPEEDS),
      choice("faces", FACES),
      toggle("leftHanded"),
      choice("reducedMotion", MOTIONS),
      // The kit's table tops, a dropdown: "random" first, then whatever the page lists.
      { key: "surface", kind: "select", labelKey: "settings.surface", options: [{ value: "random", labelKey: "settings.surface.random" }], surfaces: true },
    ],
  },
  {
    id: "aids",
    titleKey: "settings.section.aids",
    rows: [
      choice("talk", TALKS),
      toggle("tutor", { noteKey: "settings.tutor.note" }),
      toggle("tips", { noteKey: "settings.tips.note" }),
      toggle("hints"),
      toggle("tracker", { noteKey: "settings.tracker.note" }),
      toggle("explain"),
      toggle("announceAuto", { noteKey: "settings.announceAuto.note" }),
      toggle("slateAuto"),
    ],
  },
  { id: "page", titleKey: "settings.section.page", rows: [choice("lang", LANGS)] },
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

// The sitting on the table, in the prefs' own keys, and the engine's level;
// null when there is none. `source` is the reply's events (the sitting_start
// among them, a new sitting) or the export's record ({touren, settings}, a
// restored one: the state's events hold only the deal in progress).
// `style` and `drawn` are the engine's (state or export): the style chosen, and what Random drew
// (null until the sitting is over).
export function sittingFrom(source, level, style, drawn) {
  const begun = Array.isArray(source) ? source.find((e) => e.type === "sitting_start") : source;
  if (!begun || !begun.touren) return null;
  const rule = begun.settings ?? {};
  return {
    touren: begun.touren,
    both_pass: rule.both_pass ?? DEFAULTS.both_pass,
    values: rule.values ?? DEFAULTS.values,
    ultimo: rule.ultimo ?? DEFAULTS.ultimo,
    skill: level ?? DEFAULTS.skill,
    style: style ?? DEFAULTS.style,
    ...(style === "random" ? { drawn: drawn ?? null } : {}),
  };
}

// The menu for these preferences; `surfaces` is [{value, label}] from the kit
// (the strings name them, the label is the fallback); `sitting` is sittingFrom's.
export function settingsModel(prefs, { sitting = null, surfaces = [] } = {}) {
  return {
    titleKey: "settings.title",
    sitting,
    sections: SECTIONS.map((s) => ({
      ...s,
      rows: s.rows.map((r) => {
        const list = r.surfaces ? [...r.options, ...surfaces.map((x) => ({ value: x.value, label: x.label, labelKey: `settings.surface.${x.value}` }))] : r.options ?? [];
        return { ...r, options: list.map((o) => ({ ...o, selected: o.value === prefs[r.key] })), value: prefs[r.key] };
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
  let sitting = null;
  let surfaceList = surfaces;

  rootEl.classList.add("settings");
  rootEl.hidden = true;
  rootEl.setAttribute("aria-hidden", "true");
  rootEl.setAttribute("role", "dialog");

  function choose(row, value) {
    if (current[row.key] === value) return;
    current = { ...current, [row.key]: value };
    onChange(row.key, value);
    render();
  }

  function drawRow(row) {
    const id = `settings-${row.key}`;
    const labelText = t(row.labelKey);
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
    } else if (row.kind === "select") {
      const named = (o) => (o.label && t(o.labelKey) === o.labelKey ? o.label : t(o.labelKey));
      control = h(doc, "select", { class: "settings-select", "data-key": row.key, "aria-labelledby": id, onchange: (ev) => choose(row, ev.target.value) },
        row.options.map((o) => h(doc, "option", { value: String(o.value), selected: o.selected ? true : null }, named(o))));
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
            onclick: () => choose(row, o.value),
          }, o.label ?? t(o.labelKey))));
    }
    return h(doc, "div", { class: "settings-row", "data-row": row.key },
      h(doc, "span", { class: "settings-label", id }, labelText),
      control,
      row.noteKey ? h(doc, "span", { class: "settings-note" }, t(row.noteKey)) : null);
  }

  // The style's name; Random is named only at the end (STYLES 2.3): "Random (shown at the end)", then "Random: After Ulmann".
  const styleName = (x) => {
    if (x.style !== "random") return t(`style.${x.style}.name`);
    return x.drawn ? t("style.random.drawn", { drawn: t(`style.${x.drawn}.name`) }) : t("style.random.hidden");
  };

  // "This sitting: 3 Touren, 2/3/4, ultimo 1, simple game, opponent level 3", from the record.
  function thisSitting(x) {
    return h(doc, "div", { class: "settings-this", "data-row": "this-sitting" },
      h(doc, "p", {}, t("settings.sitting", { touren: t(`settings.touren.${x.touren}`), values: x.values, ultimo: x.ultimo, both_pass: t(`settings.sitting.${x.both_pass}`), level: t(`settings.skill.${x.skill}`), style: styleName(x) })),
      h(doc, "p", { class: "settings-note" }, t("settings.sitting.next")));
  }

  function render() {
    const m = settingsModel(current, { sitting, surfaces: surfaceList });
    rootEl.replaceChildren(
      h(doc, "div", { class: "settings-sheet" },
        h(doc, "div", { class: "settings-head" },
          h(doc, "h2", {}, t(m.titleKey)),
          h(doc, "button", { class: "settings-close", type: "button", "aria-label": t("ui.close"), onclick: () => close() }, "×")),
        m.sitting ? thisSitting(m.sitting) : null,
        m.sections.map((s) =>
          h(doc, "section", { class: "settings-section", "data-section": s.id },
            h(doc, "h3", {}, t(s.titleKey)),
            s.rows.map(drawRow),
            (s.actions ?? []).map((a) => h(doc, "button", { class: "settings-action", type: "button", "data-action": a.name, onclick: () => onAction(a.name) }, t(a.labelKey)))))));
  }

  let opener = null;
  function open() {
    opener = doc.activeElement ?? null;
    rootEl.hidden = false;
    rootEl.setAttribute("aria-hidden", "false");
    const first = rootEl.findAll ? rootEl.findAll((e) => e.getAttribute("role") === "radio" && e.getAttribute("aria-checked") === "true")[0] : rootEl.querySelector?.('[role="radio"][aria-checked="true"]');
    first?.focus?.();
  }
  // Escape is the page's (overlays.js); focus goes back to what opened the menu.
  function close() {
    rootEl.hidden = true;
    rootEl.setAttribute("aria-hidden", "true");
    // Back to the opener; if it is gone, hidden or body (a click does not focus a button in Safari), to the cog.
    const usable = (n) => n && n !== doc.body && n.isConnected && !n.closest?.("[hidden]") && !rootEl.contains?.(n);
    const to = [opener, doc.querySelector?.("[data-act=settings]")].find(usable);
    to?.focus?.();
    opener = null;
  }
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
    // The sitting on the table (sittingFrom's), or null: the read-only line (3.5.2, 7.1).
    setSitting(info) {
      sitting = info ?? null;
      render();
    },
    setSurfaces(list) {
      surfaceList = list;
      render();
    },
    render,
  };
}
