// The tutor's DOM builders over the tests' tiny DOM.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { helpBody, pageBody, reviewBody } from "../src/tutorview.js";
import { tutorialPages, PAGE_KEYS } from "../src/tutor.js";
import { fakeDocument } from "./support/fakedom.js";

const doc = fakeDocument();
const pages = tutorialPages(readFileSync(new URL("../tutorial.en.md", import.meta.url), "utf8"), PAGE_KEYS, { both_pass: "play" });

test("a page is drawn as its headings, paragraphs and lists, with bold kept", () => {
  const body = pageBody(doc, pages.find((p) => p.key === "bidding"));
  assert.equal(body.getAttribute("data-page"), "bidding");
  assert.equal(body.findAll((e) => e.tagName === "H3").length, 1);
  assert.ok(body.findAll((e) => e.tagName === "LI").length >= 5);
  assert.deepEqual(body.findAll((e) => e.tagName === "B").map((e) => e.textContent).slice(0, 2), ["I take it up", "Pass"]);
});

test("a link to another page is a button that reports the page it names, in the text's own place", () => {
  const body = pageBody(doc, { key: "x", title: "X", blocks: [{ type: "p", spans: [{ text: "See " }, { text: "more", page: "scoring-full" }, { text: "." }] }, { type: "ul", items: [[{ text: "Also ", bold: true }, { text: "this", page: "thirds" }]] }] });
  const links = body.findAll((e) => e.tagName === "BUTTON");
  assert.deepEqual(links.map((b) => [b.textContent, b.getAttribute("data-act"), b.getAttribute("data-page"), b.getAttribute("type")]), [["more", "page", "scoring-full", "button"], ["this", "page", "thirds", "button"]]);
  assert.ok(links.every((b) => b.classList.contains("tutor-link")));
  assert.equal(body.findAll((e) => e.tagName === "P")[0].textContent, "See more.");
});

test("the help list has a button for each page, the seen ones marked", () => {
  const body = helpBody(doc, pages, { intro: "Pages.", seen: ["thirds"] });
  const buttons = body.findAll((e) => e.tagName === "BUTTON");
  assert.equal(buttons.length, PAGE_KEYS.length);
  assert.deepEqual(buttons.map((b) => b.getAttribute("data-page")), PAGE_KEYS);
  assert.match(buttons.find((b) => b.getAttribute("data-page") === "thirds").textContent, /✓$/);
  assert.doesNotMatch(buttons[0].textContent, /✓/);
});

test("a brief is the count, bullets with a bold lead, and a folded method", () => {
  const body = reviewBody(doc, { header: "You counted 40.", bullets: [{ lead: "Went well:", text: "All met." }, { lead: "To try:", text: "Look." }], how: "How?", method: "Thirds." });
  const items = body.findAll((e) => e.tagName === "LI");
  assert.equal(items.length, 2);
  assert.equal(items[0].textContent, "Went well: All met.");
  assert.equal(items[0].findAll((e) => e.tagName === "B")[0].textContent, "Went well:");
  assert.equal(body.findAll((e) => e.tagName === "SUMMARY")[0].textContent, "How?");
  const head = body.findAll((e) => e.tagName === "P" && e.className === "tutor-count");
  assert.equal(head.length, 1);
  assert.equal(head[0].textContent, "You counted 40.");
});

test("a brief without a bullet or a header draws neither a list nor a count", () => {
  const body = reviewBody(doc, { header: null, bullets: [], how: "How?", method: "Thirds." });
  assert.equal(body.findAll((e) => e.tagName === "UL").length, 0);
  assert.equal(body.findAll((e) => e.className === "tutor-count").length, 0);
  assert.equal(body.findAll((e) => e.tagName === "SUMMARY").length, 1);
});
