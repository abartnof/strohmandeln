// The cards-out panel's DOM (docs/TABLE3D.md 6.6) as a pure builder over a
// document: it draws the model of tracker.js and reports a press of Fold by a
// `data-act` the page's click handler reads. The words arrive translated.
import { h } from "./dom.js";

// `model`: trackerModel's. `foldLabel`, `foldTitle`: the Fold button's words.
export function trackerPanel(doc, model, { foldLabel, foldTitle }) {
  return h(
    doc,
    "section",
    { class: "tracker", role: "region", "aria-label": model.title, "data-trick": String(model.trick) },
    h(
      doc,
      "div",
      { class: "tracker-head" },
      h(doc, "div", { class: "tracker-titles" }, h(doc, "h2", { class: "tracker-title" }, model.title), h(doc, "p", { class: "tracker-byline" }, model.byline)),
      h(doc, "button", { class: "tracker-fold", type: "button", "data-act": "tracker-fold", title: foldTitle }, foldLabel),
    ),
    h(doc, "ul", { class: "tracker-lines" }, model.lines.map((l) => h(doc, "li", { "data-line": l.id }, l.text))),
    // The grid is for the eye; the screen reader reads the sentences instead.
    h(
      doc,
      "div",
      { class: "tracker-grid", "aria-hidden": "true" },
      // The counts' heads sit over the last three columns of the rows below.
      h(
        doc,
        "div",
        { class: "tracker-row tracker-heads" },
        model.countHeads.map((c) => h(doc, "span", { class: "tracker-count-head", "data-count": c.id, title: c.title }, c.text)),
      ),
      model.rows.map((r) =>
        h(
          doc,
          "div",
          { class: "tracker-row", "data-row": r.id },
          h(doc, "span", { class: `tracker-mark tracker-mark-${r.id[0]}` }, r.mark),
          r.cells.map((c) => h(doc, "span", { class: "tracker-cell", "data-code": c.code, "data-state": c.state, title: c.title }, c.label)),
          // A short tarock row leaves its missing cells empty, so the counts line up.
          Array.from({ length: 8 - r.cells.length }, () => h(doc, "span", {}, "")),
          model.countHeads.map((c) => h(doc, "span", { class: "tracker-count", "data-count": c.id, title: c.title }, String(r.counts[c.id]))),
        ),
      ),
    ),
    h(
      doc,
      "ul",
      { class: "tracker-legend", "aria-hidden": "true" },
      model.legend.map((l) => h(doc, "li", {}, h(doc, "span", { class: "tracker-cell tracker-swatch", "data-state": l.state }, ""), l.text)),
    ),
    h(doc, "ul", { class: "tracker-sr" }, model.sr.map((s) => h(doc, "li", {}, s))),
  );
}
