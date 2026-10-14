import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { paceOf } from "../../kit/js/count.js";
import { choreograph, initialPlacement, opening } from "../src/choreography.js";
import { countGame, countLines, thirds } from "../src/counting.js";
import { emptyModel, reduce } from "../src/model.js";
import { momentsOf } from "../src/moments.js";
import { t } from "../src/strings.js";
import { createTalk } from "../src/talk.js";
import { ZONES } from "../src/zones.js";
import { playSitting, taker } from "./sim.js";

const WORDS = JSON.parse(readFileSync(new URL("../words.json", import.meta.url), "utf8"));
const game = countGame({ category: t("hud.count"), goal: t("hud.thirds", { n: 36 }), names: { you: "You", opp: "Opp" }, group: (n) => t("hud.group", { n }), hud: {} });

test("the count in thirds reads as the rules say", () => {
  assert.equal(thirds(107), "35⅔");
  assert.equal(thirds(106), "35⅓");
  assert.equal(thirds(105), "35");
  assert.equal(thirds(1), "⅓");
  assert.equal(thirds(0), "0");
});

test("a take's groups of three add up exactly to the engine's thirds, and the two takes to 210", async () => {
  const { replies } = await playSitting({ seed: 7, touren: 3, human: 1, level: 1 }, { policy: taker(7), limit: 300 });
  let m = emptyModel(1);
  let checked = 0;
  for (const r of replies)
    for (const e of r.events) {
      m = reduce(m, e);
      if (e.type !== "count") continue;
      const lines = countLines(m.counted, 1);
      const sum = { you: 0, opp: 0 };
      for (const l of lines) sum[l.side] += l.thirds;
      assert.equal(sum.you, e.thirds[1]);
      assert.equal(sum.opp, e.thirds[0]);
      assert.equal(sum.you + sum.opp, 210);
      checked++;
    }
  assert.ok(checked > 0);
});

test("the talk is quiet through the count, and a result line follows it", async () => {
  const { replies } = await playSitting({ seed: 7, touren: 3, human: 1, level: 1 }, { policy: taker(7), limit: 300 });
  let model = emptyModel(1);
  const log = [];
  let prev = null;
  let placement = null;
  let done = false;
  for (const r of replies) {
    const since = log.length;
    for (const e of r.events) {
      model = reduce(model, e);
      log.push(e);
    }
    const next = { model, events: [...log] };
    if (!prev) {
      prev = opening(next);
      placement = initialPlacement(prev, { zones: ZONES });
    }
    const countEvent = r.events.find((e) => e.type === "count");
    const pace = countEvent ? paceOf(game, countLines(model.counted, 1)) : null;
    const result = choreograph(prev, next, placement, { zones: ZONES }, { pace });
    placement = result.placement;
    prev = next;
    if (!countEvent || done) continue;
    done = true;
    assert.ok(r.events.some((e) => e.type === "game_scored"));
    const spoken = [];
    let clock = 0;
    const talk = createTalk({ words: WORDS, getLang: () => "en", rng: () => 0.3, say: ({ key }) => spoken.push({ key, at: clock }) });
    talk.onEvent({ type: "sitting_start", touren: 3 }, { human: 1 });
    spoken.length = 0;
    const moments = momentsOf({ events: r.events, since, beats: result.beats, count: result.count });
    let start = null;
    let close = null;
    for (const m of moments) {
      clock = m.ms;
      if (m.event.type === "count_start") start = m.ms;
      if (m.event.type === "count_close") close = m.ms;
      talk.onEvent(m.event, { human: 1 });
    }
    assert.ok(start !== null && close !== null && start < close, "the count is bracketed");
    const between = spoken.filter((s) => s.at > start && s.at < close);
    assert.deepEqual(between, [], "nothing is said inside the count");
    const after = spoken.filter((s) => s.at >= close && /^[a-z-]+\.\d+$/.test(s.key) && WORDS.groups && Object.entries(WORDS.groups).some(([g, keys]) => /^(result-|draw)/.test(g) && keys.includes(s.key)));
    assert.ok(after.length > 0, `a result line follows the count (said: ${JSON.stringify(spoken.map((s) => s.key))})`);
    // The result is passed on after the talk is told the count has closed.
    const order = moments.map((m) => m.event.type);
    assert.ok(order.indexOf("count_close") < order.indexOf("game_scored"));
  }
  assert.ok(done, "a count was seen");
});
