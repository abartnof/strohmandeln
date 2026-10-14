import { test } from "node:test";
import assert from "node:assert/strict";
import { cardCorners } from "../../kit/js/kinematics.js";
import { layout } from "../src/layout.js";
import { cloneModel, emptyModel, reduce } from "../src/model.js";
import { ZONES, ZONES_PORTRAIT, ZONES_TOUCH } from "../src/zones.js";
import { playSitting, taker } from "./sim.js";

const ARRANGEMENTS = { landscape: ZONES, touch: ZONES_TOUCH, portrait: ZONES_PORTRAIT };

// The model at every reply of a sitting.
async function models(config, policy) {
  const { replies } = await playSitting(config, { policy });
  let m = emptyModel(config.human);
  const out = [];
  for (const r of replies) {
    for (const e of r.events) {
      m = reduce(m, e);
      out.push(cloneModel(m));
    }
  }
  return out;
}

test("every model lays out 54 cards, each key once, none through the table, in every arrangement", async () => {
  const all = await models({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  for (const [name, zones] of Object.entries(ARRANGEMENTS)) {
    for (const m of all.filter((_, i) => i % 7 === 0 || i === all.length - 1)) {
      const slots = layout(m, { zones });
      assert.equal(slots.length, 54, `${name}: ${slots.length} cards`);
      assert.equal(new Set(slots.map((s) => s.zone + s.key)).size, 54, `${name}: keys are unique`);
      for (const s of slots) {
        const low = Math.min(...cardCorners(s.pose).map((c) => c.y));
        assert.ok(low >= -0.001, `${name}: ${s.zone} ${s.key} ${low.toFixed(2)} cm below the table`);
      }
    }
  }
});

test("faces only where the view has them: backs for the opponent's hand, hidden packet cards, the pack and the piles", async () => {
  const all = await models({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  for (const m of all.filter((_, i) => i % 5 === 0)) {
    for (const s of layout(m)) {
      if (["their-hand", "pack", "your-won", "their-won"].includes(s.zone)) assert.equal(s.code, null, `${s.zone} shows a face`);
      if (s.zone.endsWith("packets") && !s.cover && !s.held) assert.equal(s.code, null, "a hidden packet card shows a face");
      if (s.code) assert.equal(s.faceUp, true);
    }
  }
});
