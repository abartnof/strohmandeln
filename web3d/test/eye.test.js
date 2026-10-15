// The eye over the table: what the camera must keep in view (the `reach` of
// each eye in zones.js is measured here against whole sittings), and the
// legibility of a cover against a hand card (docs/TABLE3D.md 1.5.2).
import { test } from "node:test";
import assert from "node:assert/strict";
import { Vector3 } from "three";
import { cameraFor, DESKTOP_FOOT } from "../../kit/js/framing.js";
import { cardCorners } from "../../kit/js/kinematics.js";
import { layout } from "../src/layout.js";
import { cloneModel, emptyModel, reduce } from "../src/model.js";
import { EYE, EYE_PORTRAIT, EYE_TOUCH, ZONES, ZONES_PORTRAIT, ZONES_TOUCH } from "../src/zones.js";
import { playSitting, taker } from "./sim.js";

async function models() {
  const { replies } = await playSitting({ seed: 7, touren: 3, human: 1, level: 1 }, { policy: taker(7), limit: 300 });
  let m = emptyModel(1);
  const out = [];
  for (const r of replies)
    for (const e of r.events) {
      m = reduce(m, e);
      out.push(cloneModel(m));
    }
  return out;
}

const ARRANGEMENTS = [
  ["landscape", ZONES, EYE, [[1280, 720], [1440, 900], [1366, 650], [1920, 960]]],
  ["touch", ZONES_TOUCH, EYE_TOUCH, [[1024, 768], [1180, 820]]],
  ["portrait", ZONES_PORTRAIT, EYE_PORTRAIT, [[390, 844]]],
];

// The reach of the table from the eye's axis, as tangents.
function reachOf(all, zones, eye, skip = () => false) {
  const pos = new Vector3(...eye.position);
  const f = new Vector3(...eye.target).sub(pos).normalize();
  const r = new Vector3().crossVectors(f, new Vector3(0, 1, 0)).normalize();
  const u = new Vector3().crossVectors(r, f).normalize();
  const out = { up: -9, down: 9, across: 0 };
  for (const m of all)
    for (const s of layout(m, { zones })) {
      if (skip(s)) continue;
      for (const c of cardCorners(s.pose)) {
        const v = c.clone().sub(pos);
        const d = v.dot(f);
        out.up = Math.max(out.up, v.dot(u) / d);
        out.down = Math.min(out.down, v.dot(u) / d);
        out.across = Math.max(out.across, Math.abs(v.dot(r)) / d);
      }
    }
  return out;
}

test("each eye keeps the whole table in view: its reach is at least what the layouts need, and not far more", async () => {
  const all = (await models()).filter((_, i) => i % 3 === 0);
  const landscape = reachOf(all, ZONES, EYE);
  assert.ok(EYE.reach.up >= landscape.up - 0.01 && EYE.reach.up <= landscape.up + 0.05, `up ${EYE.reach.up} vs ${landscape.up.toFixed(3)}`);
  assert.ok(EYE.reach.foot >= -landscape.down - 0.01 && EYE.reach.foot <= -landscape.down + 0.05, `foot ${EYE.reach.foot} vs ${(-landscape.down).toFixed(3)}`);
  assert.ok(EYE.widthTan >= landscape.across - 0.01, `width ${EYE.widthTan} vs ${landscape.across.toFixed(3)}`);
  const touch = reachOf(all, ZONES_TOUCH, EYE_TOUCH);
  assert.ok(EYE_TOUCH.reach.up >= touch.up - 0.01 && EYE_TOUCH.reach.foot >= -touch.down - 0.01);
  // The portrait row scrolls, so the hand's width is not asked of the eye.
  const portrait = reachOf(all, ZONES_PORTRAIT, EYE_PORTRAIT, (s) => s.zone === "your-hand");
  assert.ok(EYE_PORTRAIT.reach.up >= portrait.up - 0.01 && -EYE_PORTRAIT.reach.down >= -portrait.down - 0.01 + 0, `portrait ${JSON.stringify(portrait)}`);
  assert.ok(EYE_PORTRAIT.reach.across >= portrait.across - 0.02, `portrait across ${EYE_PORTRAIT.reach.across} vs ${portrait.across.toFixed(3)}`);
});

test("the portrait hand's reach across, as framing.js assumes it, covers every hand length and fill", () => {
  const pos = new Vector3(...EYE_PORTRAIT.position);
  const f = new Vector3(...EYE_PORTRAIT.target).sub(pos).normalize();
  const r = new Vector3().crossVectors(f, new Vector3(0, 1, 0)).normalize();
  const across = (fill) => {
    let out = 0;
    for (let n = 1; n <= 18; n++) {
      const m = emptyModel(1);
      m.hand = Array.from({ length: n }, (_, i) => ({ code: `c${i}` }));
      for (const s of layout(m, { zones: ZONES_PORTRAIT, fill }).filter((x) => x.zone === "your-hand"))
        for (const c of cardCorners(s.pose)) {
          const v = c.clone().sub(pos);
          out = Math.max(out, Math.abs(v.dot(r)) / v.dot(f));
        }
    }
    return out;
  };
  const { still, perFill } = EYE_PORTRAIT.hand;
  assert.ok(across(1) <= still, `${across(1).toFixed(3)} at fill 1`);
  for (const fill of [1.5, 2, 2.4]) assert.ok(across(fill) <= still + (fill - 1) * perFill, `${across(fill).toFixed(3)} at fill ${fill}`);
});

const screen = (cam, w, h) => (p) => {
  const v = p.clone().project(cam);
  return { x: ((v.x + 1) / 2) * w, y: ((1 - v.y) / 2) * h };
};
const heightOf = (slot, at) => {
  const ys = cardCorners(slot.pose).map(at).map((p) => p.y);
  return Math.max(...ys) - Math.min(...ys);
};

test("a cover is drawn as large as a hand card, to within a tenth, in every arrangement and window", async () => {
  const all = await models();
  const m = all.find((x) => x.packets[1].some((p) => p.cover) && x.hand.length >= 15 && !x.held);
  assert.ok(m, "a model with covers");
  for (const [name, zones, eye, windows] of ARRANGEMENTS) {
    for (const [w, h] of windows) {
      const strips = { top: 150, foot: h - (h - 100), raised: 0 };
      const cam = cameraFor(w / h, 0, h, name === "portrait" ? { top: 190, foot: 110, raised: 0 } : strips, null, eye, EYE_PORTRAIT);
      void DESKTOP_FOOT;
      const at = screen(cam, w, h);
      const slots = layout(m, { zones });
      const own = slots.filter((s) => s.zone === "your-hand");
      // The middle card of the hand (the ends of a fan are turned, which tilts their extent).
      const hand = own.filter((s) => s.index === own.length >> 1).map((s) => heightOf(s, at));
      const covers = slots.filter((s) => s.cover).map((s) => heightOf(s, at)).sort((a, b) => a - b);
      const hm = hand[0];
      const cm = covers[covers.length >> 1];
      assert.ok(Math.abs(cm - hm) <= 0.10 * hm, `${name} ${w}x${h}: a cover ${cm.toFixed(0)} px, a hand card ${hm.toFixed(0)} px`);
    }
  }
});
