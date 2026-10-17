// The English terms (docs/TABLE3D.md, the terms note): the count's unit is
// "card point(s)" (German Augen); "Point(s)" with a capital is the payment on
// the slate (German Points; RULES 9, 15.8). A bare lowercase "point(s)" is the
// count's unit without its qualifier, and is not allowed in an English string
// or page. The hint.* strings are held to the same rule elsewhere.
import test from "node:test";
import assert from "node:assert/strict";
import { STRINGS, allPages, lines } from "./support/pages.js";

// Named exceptions: [text pattern, why]. None is needed at present.
const EXCEPTIONS = [];
const allowed = (s) => EXCEPTIONS.some(([re]) => re.test(s));
// A lowercase point(s) not directly after "card ": the count without its qualifier.
const BARE = { test: (s) => /(?<!\bcard )\bpoints?\b/.test(s.replace(/\{[^}]*\}/g, "")) }; // placeholders are not words
const strings = () => Object.entries(STRINGS).filter(([k]) => !k.startsWith("hint."));

test("no English string names the count's unit as a bare \"point(s)\"", () => {
  const bad = strings().filter(([, v]) => BARE.test(v.en) && !allowed(v.en)).map(([k, v]) => `${k}: ${v.en}`);
  assert.deepEqual(bad, []);
});

test("no English page names the count's unit as a bare \"point(s)\", under every setting", () => {
  const bad = new Set(allPages("en").flatMap((p) => lines(p).filter((l) => BARE.test(l) && !allowed(l)).map((l) => `${p.key}: ${l}`)));
  assert.deepEqual([...bad], []);
});

test("the payment is a capital Point and the count's unit is \"card point\": the two never swap", () => {
  const all = [...strings().map(([, v]) => v.en), ...allPages("en").flatMap(lines)];
  assert.ok(all.every((s) => !/\bcard Points?\b/.test(s)), "\"card Point\" with a capital");
  assert.ok(all.some((s) => /\bcard points\b/.test(s)));
  assert.ok(all.some((s) => /\bPoints\b/.test(s)));
});

test("the scoring page says the count wins and the slate pays, in the two units", () => {
  const sc = lines(allPages("en").find((p) => p.key === "scoring")).join("\n");
  assert.match(sc, /36 card points of the 70/);
  assert.match(sc, /paid in Points/);
});
