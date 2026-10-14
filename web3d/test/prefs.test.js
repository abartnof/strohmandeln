import test from "node:test";
import assert from "node:assert/strict";
import { DEFAULTS, VERSION, KEY, loadPrefs, loadPrefsInfo, savePrefs, withUrl, urlOverrides, sittingConfig, speedScale, SKILLS } from "../src/prefs.js";
import { fakeStore } from "./support/fakedom.js";

test("an empty store gives the defaults of the spec", () => {
  const p = loadPrefs(fakeStore());
  assert.deepEqual(p, { ...DEFAULTS, seen: [] });
  assert.equal(p.both_pass, "play");
  assert.equal(p.values, "2/3/4");
  assert.equal(p.ultimo, 1);
  assert.equal(p.touren, 3);
  assert.equal(p.skill, 3);
  assert.equal(p.speed, "normal");
  assert.equal(p.talk, "all");
  assert.equal(p.tutor, true);
  assert.equal(p.hints, false);
  assert.equal(p.slateAuto, true);
  assert.equal(p.faces, "auto");
  assert.equal(p.lang, "en");
});

test("a broken or missing store gives defaults and a notice, and saving does not throw", () => {
  const store = fakeStore({}, { broken: true });
  const info = loadPrefsInfo(store);
  assert.deepEqual(info.prefs, { ...DEFAULTS, seen: [] });
  assert.equal(info.notice, "unavailable");
  assert.equal(savePrefs({ ...DEFAULTS }, store), false);
  assert.deepEqual(loadPrefs(null), { ...DEFAULTS, seen: [] });
});

test("what is saved is read back, with the version", () => {
  const store = fakeStore();
  assert.equal(savePrefs({ ...DEFAULTS, speed: "fast", lang: "de", values: "1/2/4", seen: ["packets"] }, store), true);
  assert.equal(JSON.parse(store.map.get(KEY)).v, VERSION);
  const p = loadPrefs(store);
  assert.equal(p.speed, "fast");
  assert.equal(p.lang, "de");
  assert.equal(p.values, "1/2/4");
  assert.deepEqual(p.seen, ["packets"]);
});

test("fields of the wrong type or value are read field by field", () => {
  const store = fakeStore({ [KEY]: JSON.stringify({ v: 1, speed: 7, lang: "fr", values: "5/5/5", ultimo: 3, touren: 4, skill: 9, tutor: "yes", hints: true, seen: ["a", 1, "b"], faces: "jumbo" }) });
  const p = loadPrefs(store);
  assert.equal(p.speed, "normal");
  assert.equal(p.lang, "en");
  assert.equal(p.values, "2/3/4");
  assert.equal(p.ultimo, 1);
  assert.equal(p.touren, 3);
  assert.equal(p.skill, 3);
  assert.equal(p.tutor, true);
  assert.equal(p.hints, true);
  assert.equal(p.faces, "jumbo");
  assert.deepEqual(p.seen, ["a", "b"]);
});

test("unparsable storage is read as empty", () => {
  assert.deepEqual(loadPrefs(fakeStore({ [KEY]: "{nope" })), { ...DEFAULTS, seen: [] });
  assert.deepEqual(loadPrefs(fakeStore({ [KEY]: "[1]" })), { ...DEFAULTS, seen: [] });
});

test("a record from a newer page is read as empty, left as it is, and said", () => {
  const newer = JSON.stringify({ v: VERSION + 1, speed: "fast" });
  const store = fakeStore({ [KEY]: newer });
  const info = loadPrefsInfo(store);
  assert.equal(info.notice, "newer");
  assert.equal(info.prefs.speed, "normal");
  assert.equal(savePrefs({ ...DEFAULTS }, store), false);
  assert.equal(store.map.get(KEY), newer);
});

test("skill is one of the dial's halves", () => {
  assert.deepEqual(SKILLS, [1, 1.5, 2, 2.5, 3, 3.5, 4]);
  assert.equal(loadPrefs(fakeStore({ [KEY]: JSON.stringify({ v: 1, skill: 2.5 }) })).skill, 2.5);
});

test("URL overrides are for tests", () => {
  const q = (s) => new URLSearchParams(s);
  const p = withUrl({ ...DEFAULTS }, q("lang=de&speed=instant&talk=none&touren=6"));
  assert.equal(p.lang, "de");
  assert.equal(p.speed, "instant");
  assert.equal(p.talk, "none");
  assert.equal(p.touren, 6);
  assert.equal(withUrl({ ...DEFAULTS }, q("lang=xx&speed=warp")).lang, "en");
  assert.deepEqual(urlOverrides(q("seed=7&human=1&noworker")), { seed: 7, human: 1, noworker: true });
  assert.deepEqual(urlOverrides(q("")), { seed: null, human: null, noworker: false });
});

test("the config for strohmandeln_new follows the protocol", () => {
  const cfg = sittingConfig({ ...DEFAULTS, both_pass: "throw_in", values: "1/2/3", ultimo: 2, touren: 6, skill: 4 }, { seed: 7, human: 1 });
  assert.deepEqual(cfg, { seed: 7, touren: 6, human: 1, level: 6, settings: { both_pass: "throw_in", values: "1/2/3", ultimo: 2 } });
  const no = sittingConfig({ ...DEFAULTS }, {});
  assert.ok(!("human" in no));
  assert.equal(no.level, 4);
  assert.ok(Number.isInteger(no.seed));
});

test("speed scales: slow longer, instant nothing", () => {
  assert.ok(speedScale("slow") > speedScale("normal"));
  assert.equal(speedScale("normal"), 1);
  assert.ok(speedScale("fast") < 1);
  assert.equal(speedScale("instant"), 0);
});
