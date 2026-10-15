import test from "node:test";
import assert from "node:assert/strict";
import { dragKind, sameSpot, shake, shakeOffset, SHAKE } from "../src/gesture.js";

test("a drag up toward the trick plays; a sideways drag scrolls; a touch is a tap", () => {
  assert.equal(dragKind(2, -3), "tap");
  assert.equal(dragKind(5, -90), "play");
  assert.equal(dragKind(40, -90), "play", "steep enough");
  assert.equal(dragKind(90, -90), "none", "a diagonal is neither");
  assert.equal(dragKind(-80, 10), "scroll");
  assert.equal(dragKind(0, -20), "none", "too short to be a play");
  assert.equal(dragKind(0, 90), "none", "down is nothing");
});

test("the shake dies away and ends at rest", () => {
  assert.equal(shakeOffset(0), 0);
  assert.equal(shakeOffset(SHAKE.ms), 0);
  assert.equal(shakeOffset(SHAKE.ms + 50), 0);
  const samples = Array.from({ length: 36 }, (_, i) => shakeOffset((i / 36) * SHAKE.ms));
  assert.ok(samples.every((v) => Math.abs(v) <= SHAKE.amplitude));
  assert.ok(samples.some((v) => v > 0.05) && samples.some((v) => v < -0.05), "it swings both ways");
  const early = Math.max(...samples.slice(0, 12).map(Math.abs));
  const late = Math.max(...samples.slice(24).map(Math.abs));
  assert.ok(late < early, "it dies away");
});

test("shake moves the mesh and puts it back; reduced motion does not move it", () => {
  let clock = 0;
  const frames = [];
  const mesh = { position: { x: 2 } };
  const seen = [];
  const env = { now: () => clock, frame: (fn) => frames.push(fn), render: () => seen.push(mesh.position.x) };
  assert.equal(shake(mesh, env), true);
  while (frames.length) {
    const fn = frames.shift();
    clock += 20;
    fn();
  }
  assert.ok(seen.some((x) => x !== 2));
  assert.equal(mesh.position.x, 2);
  assert.equal(shake(mesh, { ...env, reduced: true }), false);
});

test("a second shake begun mid-shake does not move the card's resting place", () => {
  let clock = 0;
  const frames = [];
  const mesh = { position: { x: 2 } };
  const env = { now: () => clock, frame: (fn) => frames.push(fn), render: () => {} };
  shake(mesh, env);
  clock += 100;
  frames.shift()(); // mid-shake: the card is off its place
  assert.notEqual(mesh.position.x, 2);
  shake(mesh, env); // pressed again
  let guard = 0;
  while (frames.length && guard++ < 200) {
    clock += 20;
    frames.shift()();
  }
  assert.equal(mesh.position.x, 2);
});

test("a second click at the first one's spot is the same card again, though the card has lifted from under it", () => {
  assert.equal(sameSpot({ x: 100, y: 200 }, { x: 100, y: 200 }), true);
  assert.equal(sameSpot({ x: 100, y: 200 }, { x: 108, y: 205 }), true, "a hand drifts");
  assert.equal(sameSpot({ x: 100, y: 200 }, { x: 140, y: 200 }), false, "another card");
  assert.equal(sameSpot(null, { x: 1, y: 1 }), false, "no first click (a key chose it)");
});
