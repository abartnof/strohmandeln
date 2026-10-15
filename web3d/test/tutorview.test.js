// The tutor's DOM builders over the tests' tiny DOM.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { helpBody, pageBody, reviewBody, trendBody } from "../src/tutorview.js";
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

test("the help list has a button for each page, the seen ones marked", () => {
  const body = helpBody(doc, pages, { intro: "Pages.", seen: ["thirds"] });
  const buttons = body.findAll((e) => e.tagName === "BUTTON");
  assert.equal(buttons.length, 11);
  assert.deepEqual(buttons.map((b) => b.getAttribute("data-page")), PAGE_KEYS);
  assert.match(buttons.find((b) => b.getAttribute("data-page") === "thirds").textContent, /✓$/);
  assert.doesNotMatch(buttons[0].textContent, /✓/);
});

test("the earlier sittings are a titled list of their tallies", () => {
  const body = trendBody(doc, "Your sittings", ["Deals: 2 won.", "Deals: 1 won."]);
  assert.equal(body.findAll((e) => e.tagName === "LI").length, 2);
  assert.equal(body.textContent.startsWith("Your sittings"), true);
});

test("a review is bullets with a bold lead, and a folded method", () => {
  const body = reviewBody(doc, { bullets: [{ lead: "The count.", text: "You counted 40." }, { lead: "One thing to try.", text: "If I may." }], how: "How?", method: "Thirds." });
  const items = body.findAll((e) => e.tagName === "LI");
  assert.equal(items.length, 2);
  assert.equal(items[0].textContent, "The count. You counted 40.");
  assert.equal(items[0].findAll((e) => e.tagName === "B")[0].textContent, "The count.");
  assert.equal(body.findAll((e) => e.tagName === "SUMMARY")[0].textContent, "How?");
});
