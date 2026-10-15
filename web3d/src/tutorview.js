// The tutor's DOM, as pure builders over a document (docs/TABLE3D.md 6): a
// tutorial page, the help list and the review dialog's body. They draw what
// they are given; the words are keys of the strings, given as already
// translated text, and what is pressed is reported by `data-` attributes the
// page's click handler reads. Tested over the tests' tiny DOM.
import { h } from "./dom.js";

const spans = (doc, list) => list.map((s) => (s.bold ? h(doc, "b", {}, s.text) : s.italic ? h(doc, "i", {}, s.text) : s.text));

// A parsed tutorial page (kit/js/tutorial.js parseTutorial): its headings,
// paragraphs and lists.
export function pageBody(doc, page) {
  return h(
    doc,
    "div",
    { class: "tutor-page", "data-page": page.key },
    page.blocks.map((b) => {
      if (b.type === "h") return h(doc, "h3", {}, spans(doc, b.spans));
      if (b.type === "p") return h(doc, "p", {}, spans(doc, b.spans));
      return h(doc, b.type, {}, b.items.map((item) => h(doc, "li", {}, spans(doc, item))));
    }),
  );
}

// The Help list: one button a page, the seen ones marked; pressing reports
// `data-page`.
export function helpBody(doc, pages, { intro, seen = [], seenMark = "✓" }) {
  return h(
    doc,
    "div",
    { class: "tutor-help" },
    h(doc, "p", {}, intro),
    h(
      doc,
      "ul",
      { class: "tutor-pages" },
      pages.map((p) =>
        h(doc, "li", {}, h(doc, "button", { class: "big", type: "button", "data-page": p.key, "data-act": "page" }, p.title, seen.includes(p.key) ? ` ${seenMark}` : "")),
      ),
    ),
  );
}

// The review dialog's body (web3d/src/review.js reviewBlocks): bullets with a
// bold lead, and the folded method.
export function reviewBody(doc, blocks) {
  return h(
    doc,
    "div",
    { class: "tutor-review-body" },
    h(doc, "ul", {}, blocks.bullets.map((b) => h(doc, "li", {}, h(doc, "b", {}, b.lead), " ", b.text))),
    h(doc, "details", {}, h(doc, "summary", {}, blocks.how), h(doc, "p", {}, blocks.method)),
  );
}
