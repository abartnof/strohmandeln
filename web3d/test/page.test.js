import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { bonusLines, countGame, edgeOf, thirds, TARGET } from "../src/counting.js";
import { countEvents, handTotal, blank } from "../../kit/js/scoring.js";
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

// 2.5.5: honours, ultimo and valat are Points in a category beside the count in thirds.
const words = { category: "Count", bonus: "Honours and ultimo", goal: "36 of 70", names: { you: "You", opp: "Opp" }, group: (n) => `Group ${n}`, hud: {} };

test("bonuses are an apart category: Points, never added to the thirds", () => {
  const game = countGame(words);
  assert.deepEqual(game.categories.map(([k]) => k), ["count", "bonus"]);
  assert.deepEqual(game.apart, ["bonus"]);
  assert.equal(game.format(2, "bonus"), "2");
  assert.equal(game.format(107, "count"), "35⅔");
  const h = blank(game);
  h.count.you = 107;
  h.bonus.you = 3;
  assert.equal(handTotal(game, h, "you"), 107);
});

test("only the credits that are bonuses become lines, on the credited player's side, in the language's words", () => {
  const credits = [
    { seat: 1, cause: "game", points: 3 },
    { seat: 1, cause: "trull", points: 2 },
    { seat: 0, cause: "ultimo_won", points: 2 },
    { seat: 1, cause: "valat", points: 4 },
  ];
  const lines = bonusLines(credits, 0, (cause) => `<${cause}>`);
  assert.deepEqual(lines.map((l) => [l.side, l.points, l.label]), [["opp", 2, "<trull>"], ["you", 2, "<ultimo_won>"], ["opp", 4, "<valat>"]]);
  const game = countGame(words);
  const told = countEvents(game, lines, { at: 0, hand: 1 }).filter((e) => !e.end);
  assert.deepEqual(told.map((e) => [e.side, e.cat, e.pts, e.label]), [["opp", "bonus", 2, "<trull>"], ["you", "bonus", 2, "<ultimo_won>"], ["opp", "bonus", 4, "<valat>"]]);
  assert.deepEqual(bonusLines([{ seat: 0, cause: "game", points: 3 }], 0, String), []);
});

// 2.5.3, 2.5.4: the edge. 106 is "35 1/3", one short; 105:105 is undecided and lights no tick.
test("the edge: a tick is drawn once a side has 107; a side on 106 stands one short", () => {
  assert.deepEqual(edgeOf({ you: 107, opp: 103 }), { tick: true, short: [] });
  assert.deepEqual(edgeOf({ you: 104, opp: 106 }), { tick: false, short: ["opp"] });
  assert.deepEqual(edgeOf({ you: 105, opp: 105 }), { tick: false, short: [] }, "35 : 35 is no edge");
  assert.deepEqual(edgeOf({ you: 210, opp: 0 }), { tick: true, short: [] });
});

// A language switch rebuilds the HUD: the tick (2.5.4) and a dropped drag must survive it.
test("the main page re-applies the edge to a rebuilt HUD and ends a drag the browser cancelled", () => {
  const main = src("main.js");
  const lang = main.slice(main.indexOf("onLang((lang)"));
  assert.match(lang.slice(0, lang.indexOf("stage.render()")), /createHud[\s\S]*classList\.toggle\("edge", edgeOf\(totals\)\.tick\)/);
  assert.match(main, /addEventListener\("pointercancel"/);
});
