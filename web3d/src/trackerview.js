// The cards-out card's DOM (docs/TABLE3D.md 6.6) as a pure builder over a
// document: the HUD's card shape (hud.css `.hud-card`) with its chevron, closed
// to the head alone and open to the model of tracker.js. A press of the head or
// its chevron is reported by `data-act="tracker"`, which the page's click
// handler reads. The words arrive translated.
import { CHEVRON_SVG } from "../../kit/js/hud.js";
import { h } from "./dom.js";

// `model`: trackerModel's, or null while closed. `open`: the card's state.
// `title`, `showLabel`, `hideLabel`: the head's words and the chevron's name.
export function trackerPanel(doc, model, { title, showLabel, hideLabel, open }) {
  const name = open ? hideLabel : showLabel;
  const chevron = h(doc, "button", { class: "hud-chev", type: "button", "data-act": "tracker", "aria-expanded": String(open), "aria-label": name, title: name });
  chevron.innerHTML = CHEVRON_SVG;
  // Closed, the head is the title and the chevron; open, the Kibitzer's byline sits under the title.
  const titles = h(doc, "div", { class: "tracker-titles" }, h(doc, "h2", { class: "tracker-title" }, title), open && model ? h(doc, "p", { class: "tracker-byline" }, model.byline) : null);
  const head = h(doc, "div", { class: "tracker-head", "data-act": "tracker" }, titles, chevron);
  const card = h(doc, "section", { class: open ? "hud-card tracker-card open" : "hud-card tracker-card", role: "region", "aria-label": title, "data-trick": model ? String(model.trick) : null }, head);
  if (open && model) card.append(trackerBody(doc, model));
  return card;
}

function trackerBody(doc, model) {
  return h(
    doc,
    "div",
    { class: "tracker" },
    h(doc, "ul", { class: "tracker-lines" }, model.lines.map((l) => h(doc, "li", { "data-line": l.id }, l.text))),
    // The grid is for the eye; the screen reader reads the sentences instead.
    h(
      doc,
      "div",
      { class: "tracker-grid", "aria-hidden": "true" },
      // The counts' heads sit over the counts' columns (tracker.css places both).
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
