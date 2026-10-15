import { test } from "node:test";
import assert from "node:assert/strict";
import { Plane, Ray, Vector3 } from "three";
import { cameraFor, framing } from "../../kit/js/framing.js";
import { cardCorners } from "../../kit/js/kinematics.js";
import { CARD } from "../../kit/js/units.js";
import { layout } from "../src/layout.js";
import { cloneModel, emptyModel, reduce } from "../src/model.js";
import { EYE_PORTRAIT, EYE_TOUCH, ZONES, ZONES_PORTRAIT, ZONES_TOUCH } from "../src/zones.js";
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

// ---- a phone's hand fits the view (docs/TABLE3D.md 1.3) ----------------------

// Seen through the camera the page uses: every card of the hand inside the
// window, and each card's index (a strip 2.9 card units wide at its left, 5.6
// tall: kit/js/faces.js JUMBO) not under another card.
const INDEX = { w: (2.9 / 7) * CARD.width, h: (5.6 / 9.8) * CARD.height };

function hidden(slots, i, cam) {
  const eye = cam.position.clone();
  const s = slots[i].pose;
  let covered = 0;
  for (const fx of [0, 0.5, 1]) {
    for (const fy of [0, 0.5, 1]) {
      const local = new Vector3(-CARD.width / 2 + fx * INDEX.w, CARD.height / 2 - fy * INDEX.h, 0);
      const p = local.applyQuaternion(s.quaternion).add(s.position);
      const ray = new Ray(eye, p.clone().sub(eye).normalize());
      const dist = p.distanceTo(eye);
      slots.forEach((o, j) => {
        if (j === i) return;
        const n = new Vector3(0, 0, 1).applyQuaternion(o.pose.quaternion);
        const q = ray.intersectPlane(new Plane().setFromNormalAndCoplanarPoint(n, o.pose.position), new Vector3());
        if (!q || q.distanceTo(eye) >= dist - 1e-4) return;
        const l = q.sub(o.pose.position).applyQuaternion(o.pose.quaternion.clone().invert());
        if (Math.abs(l.x) <= CARD.width / 2 && Math.abs(l.y) <= CARD.height / 2) covered++;
      });
    }
  }
  return covered;
}

const PHONES = [
  ["portrait 390x844", 390, 844, { top: 200, foot: 160, raised: 0 }, null],
  ["portrait 360x740", 360, 740, { top: 190, foot: 156, raised: 0 }, null],
  ["upright 768x1024", 768, 1024, { top: 200, foot: 160, raised: 0 }, null],
];

test("a phone's whole hand, up to 18 cards, is in the window with every index clear", async () => {
  const all = await models({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  const biggest = Math.max(...all.map((m) => m.hand.length));
  assert.ok(biggest >= 15, `a hand of ${biggest}`);
  for (const n of [18, 15, 12, 9, 5, 1]) {
    const m = cloneModel(all.find((x) => x.hand.length === biggest));
    m.hand = Array.from({ length: n }, (_, i) => m.hand[i % m.hand.length]).map((c, i) => ({ ...c, code: `${c.code}#${i}` }));
    for (const [name, w, h, strips, sides] of PHONES) {
      const cam = cameraFor(w / h, 0, h, strips, sides, EYE_PORTRAIT, EYE_PORTRAIT);
      const f = framing(w / h, 0, h, strips, sides, EYE_PORTRAIT, EYE_PORTRAIT);
      const slots = layout(m, { zones: ZONES_PORTRAIT, fill: f.fill }).filter((s) => s.zone === "your-hand");
      assert.equal(slots.length, n);
      slots.forEach((s, i) => {
        for (const c of cardCorners(s.pose)) {
          const v = c.clone().project(cam);
          const x = ((v.x + 1) / 2) * w;
          const y = ((1 - v.y) / 2) * h;
          assert.ok(x >= 2 && x <= w - 2 && y >= 0 && y <= h, `${name}, ${n} cards: card ${i} corner at ${x.toFixed(0)},${y.toFixed(0)}`);
        }
        assert.equal(hidden(slots, i, cam), 0, `${name}, ${n} cards: the index of card ${i} is under another card`);
      });
    }
  }
});

test("a phone held sideways (the touch arrangement): the whole hand is in the window, every index clear", async () => {
  const all = await models({ seed: 7, touren: 3, human: 1, level: 1 }, taker(7));
  const m = all.find((x) => x.hand.length >= 15);
  assert.ok(m);
  const [w, h] = [844, 390];
  const strips = { top: 150, foot: 106, raised: 0 };
  const cam = cameraFor(w / h, 0, h, strips, null, EYE_TOUCH, EYE_PORTRAIT);
  const slots = layout(m, { zones: ZONES_TOUCH }).filter((s) => s.zone === "your-hand");
  slots.forEach((s, i) => {
    for (const c of cardCorners(s.pose)) {
      const v = c.clone().project(cam);
      assert.ok(Math.abs(v.x) <= 1 && Math.abs(v.y) <= 1, `card ${i} corner at ${v.x.toFixed(2)},${v.y.toFixed(2)}`);
    }
    assert.equal(hidden(slots, i, cam), 0, `the index of card ${i} is under another card`);
  });
});
