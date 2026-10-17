import test from "node:test";
import assert from "node:assert/strict";
import { DEFAULTS, VERSION, KEY, loadPrefs, loadPrefsInfo, savePrefs, withUrl, urlOverrides, sittingConfig, speedScale, SKILLS, STYLES, OFFERED_STYLES } from "../src/prefs.js";
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
  assert.equal(p.tracker, true, "the tracker is on, and shows once Hints is");
  assert.equal("trackerFolded" in p, false, "no fold is kept: the panel starts folded each deal");
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

test("the skill dial is 1, 2, 3, 3½, 4; a kept half level snaps as the engine does (1.5 to 1, 2.5 to 2)", () => {
  assert.deepEqual(SKILLS, [1, 2, 3, 3.5, 4]);
  const skill = (v) => loadPrefs(fakeStore({ [KEY]: JSON.stringify({ v: 1, skill: v }) })).skill;
  assert.equal(skill(3.5), 3.5);
  assert.equal(skill(1.5), 1);
  assert.equal(skill(2.5), 2);
  assert.equal(skill(9), 3, "anything else takes the default");
});

test("the style is kept; one that is not offered (any more) is Normal", () => {
  assert.equal(DEFAULTS.style, "normal");
  assert.deepEqual(STYLES.slice(0, 2), ["normal", "random"]);
  assert.deepEqual(OFFERED_STYLES, ["bermann", "ms", "bold", "cautious", "hunter"], "Werner and Ulmann are not offered (STYLES 9)");
  const style = (v) => loadPrefs(fakeStore({ [KEY]: JSON.stringify({ v: 1, style: v }) })).style;
  for (const id of STYLES) assert.equal(style(id), id);
  assert.equal(style("werner"), "normal");
  assert.equal(style(5), "normal");
  assert.equal(sittingConfig({ ...DEFAULTS, style: "random" }, {}).style, "random");
  assert.equal(sittingConfig({ ...DEFAULTS }, {}).style, "normal");
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
  assert.deepEqual(cfg, { seed: 7, touren: 6, human: 1, level: 4, style: "normal", settings: { both_pass: "throw_in", values: "1/2/3", ultimo: 2 } });
  const no = sittingConfig({ ...DEFAULTS }, {});
  assert.ok(!("human" in no));
  assert.equal(no.level, DEFAULTS.skill);
  for (const skill of SKILLS) assert.equal(sittingConfig({ ...DEFAULTS, skill }, {}).level, skill, "the 1 to 4 dial goes to the module as it is");
  assert.ok(Number.isInteger(no.seed));
});

test("speed scales: slow longer, instant nothing", () => {
  assert.ok(speedScale("slow") > speedScale("normal"));
  assert.equal(speedScale("normal"), 1);
  assert.ok(speedScale("fast") < 1);
  assert.equal(speedScale("instant"), 0);
});

test("the tracker's setting is read field by field", () => {
  const read = (o) => loadPrefs(fakeStore({ [KEY]: JSON.stringify({ v: 1, ...o }) }));
  assert.equal(read({ tracker: false }).tracker, false);
  assert.equal(read({ tracker: "no" }).tracker, true);
  assert.equal("trackerFolded" in read({ trackerFolded: false }), false, "an old saved fold is dropped");
});
