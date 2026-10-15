import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { countGame, thirds, TARGET } from "../src/counting.js";
import { t } from "../src/strings.js";

const src = (f) => readFileSync(new URL(`../src/${f}`, import.meta.url), "utf8");

// 3.1.1: the page holds no rule of play. What may be played is state.legal;
// why it may not is the engine's refusal key.
test("the page's code has no follow-suit or trump logic", () => {
  for (const f of ["main.js", "ui.js", "model.js", "layout.js", "choreography.js", "a11y.js", "cardinfo.js"]) {
    const text = src(f).replace(/\/\/.*$/gm, "");
    assert.ok(!/must-follow|must-trump|must_follow|followSuit|suitOf\b/.test(text), `${f} names a follow-suit rule`);
    assert.ok(!/\.suit\s*[!=]==|suit\s*[!=]==\s*["']/.test(text), `${f} compares suits`);
  }
});

test("the HUD's finish lights at 107 thirds and not before", () => {
  const game = countGame({ category: "Count", goal: "36 of 70", names: { you: "You", opp: "Opp" }, group: (n) => `Group ${n}`, hud: {} });
  assert.equal(game.lit(TARGET), 36);
  assert.equal(game.lit(TARGET - 1), 35, "35 1/3 stops short of the finish");
  assert.equal(game.lit(35), 11, "11 2/3 lights eleven segments, not twelve");
  assert.equal(thirds(TARGET), "35⅔");
  assert.equal(t("hud.thirds", { n: 36 }), "36 of 70");
});
