// The person's settings, kept in the browser (docs/TABLE3D.md 7.1, 7.7).
// Every read and write survives storage that is missing, full or blocked (a
// private window, cleared site data): the page then starts from the defaults
// and works as a single sitting. The record carries `v`, the version of the
// defaults it was kept under, so a default changed later is not mistaken for
// the person's choice.
//
// Cassino's prefs.js is the pattern (its keys and defaults are its own).

export const KEY = "strohmandeln.prefs";
// Version 1: the first. Version 2: the style (docs/TABLE3D.md 7.3); the half levels 1.5 and 2.5
// are gone from the dial.
export const VERSION = 2;

export const BOTH_PASS = Object.freeze(["play", "throw_in"]);
export const VALUES = Object.freeze(["2/3/4", "1/2/3", "1/2/4"]);
export const ULTIMO = Object.freeze([1, 2]);
export const TOUREN = Object.freeze([3, 6]);
export const SKILLS = Object.freeze([1, 2, 3, 3.5, 4]);
// The styles the new-sitting screen offers after Normal and Random, in order (docs/STYLES.md 9:
// Werner and Ulmann fell out; Hunter is provisional). The engine's `styles()` is the source
// once it exists (offeredStyles); this list is the fallback until then, and is deleted with it.
export const OFFERED_STYLES = Object.freeze(["bermann", "ms", "bold", "cautious", "hunter"]);

const styleId = (x) => typeof x === "string" && /^[a-z]{2,16}$/.test(x);

// The offered styles: the engine's own list when it has one (a list of ids, or {styles: [...]}), else the fallback.
export function offeredStyles(engine) {
  let got = null;
  try {
    got = engine?.styles?.() ?? null;
  } catch {
    got = null;
  }
  if (got && !Array.isArray(got)) got = got.styles;
  return Array.isArray(got) && got.length && got.every(styleId) ? got : OFFERED_STYLES;
}
export const SPEEDS = Object.freeze(["slow", "normal", "fast", "instant"]);
export const TALKS = Object.freeze(["all", "calls", "none"]);
export const FACES = Object.freeze(["auto", "classic", "jumbo"]);
export const MOTIONS = Object.freeze(["system", "on", "off"]);
export const LANGS = Object.freeze(["en", "de"]);

export const DEFAULTS = Object.freeze({
  // The three rule settings and the length: the next sitting's (7.2, 3.5.2).
  both_pass: "play",
  values: "2/3/4",
  ultimo: 1,
  touren: 3,
  skill: 3,
  style: "normal",
  speed: "normal",
  talk: "all",
  tutor: true,
  // Tips from the Kibitzer (docs/TUTOR.md 6): off sets no focus and shows no
  // nudge; the brief keeps its first line.
  tips: true,
  hints: false,
  // The cards-out tracker (6.6): shown with Hints on; it starts folded each
  // deal and keeps no fold of its own.
  tracker: true,
  explain: false,
  announceAuto: false,
  slateAuto: true,
  faces: "auto",
  leftHanded: false,
  reducedMotion: "system",
  lang: "en",
  surface: "random",
  // The Kibitzer muted (6.1), and the tutorial pages seen (6.2.2).
  muted: false,
  seen: Object.freeze([]),
});

// How a stored field is checked: a predicate per field; a bad one takes its
// default.
const isBool = (x) => typeof x === "boolean";
const oneOf = (list) => (x) => list.includes(x);
const CHECKS = {
  both_pass: oneOf(BOTH_PASS),
  values: oneOf(VALUES),
  ultimo: oneOf(ULTIMO),
  touren: oneOf(TOUREN),
  skill: oneOf(SKILLS),
  style: (x) => styleId(x) || x === "normal", // the offered list decides what the screen shows (newsitting.js)
  speed: oneOf(SPEEDS),
  talk: oneOf(TALKS),
  tutor: isBool,
  tips: isBool,
  hints: isBool,
  tracker: isBool,
  explain: isBool,
  announceAuto: isBool,
  slateAuto: isBool,
  faces: oneOf(FACES),
  leftHanded: isBool,
  reducedMotion: oneOf(MOTIONS),
  lang: oneOf(LANGS),
  surface: (x) => typeof x === "string" && x.length > 0 && x.length < 40,
  muted: isBool,
};

// A kept half level snaps as the engine does (Level::from_f64, ties down): 1.5 is 1, 2.5 is 2.
const snapSkill = (x) => (x === 1.5 ? 1 : x === 2.5 ? 2 : x);

const fresh = () => ({ ...DEFAULTS, seen: [] });

function defaultStore() {
  try {
    return globalThis.localStorage ?? null;
  } catch {
    return null;
  }
}

function read(store) {
  try {
    return store ? store.getItem(KEY) : undefined;
  } catch {
    return undefined; // blocked
  }
}

// What was kept, each field checked, with a note for the page to say once:
// "unavailable" (no storage), "newer" (written by a newer page: read as
// empty, and left as it is).
export function loadPrefsInfo(store = defaultStore()) {
  const raw = read(store);
  if (raw === undefined) return { prefs: fresh(), notice: "unavailable" };
  if (raw === null) return { prefs: fresh(), notice: null };
  let kept;
  try {
    kept = JSON.parse(raw);
  } catch {
    return { prefs: fresh(), notice: null };
  }
  if (!kept || typeof kept !== "object" || Array.isArray(kept)) return { prefs: fresh(), notice: null };
  if (Number.isInteger(kept.v) && kept.v > VERSION) return { prefs: fresh(), notice: "newer" };
  const prefs = fresh();
  kept.skill = snapSkill(kept.skill);
  for (const [field, ok] of Object.entries(CHECKS)) if (ok(kept[field])) prefs[field] = kept[field];
  if (Array.isArray(kept.seen)) prefs.seen = kept.seen.filter((k) => typeof k === "string");
  return { prefs, notice: null };
}

export const loadPrefs = (store) => loadPrefsInfo(store).prefs;

// Whether it was kept. A store written by a newer page is not overwritten.
export function savePrefs(prefs, store = defaultStore()) {
  try {
    if (!store) return false;
    const old = store.getItem(KEY);
    if (old) {
      try {
        if (JSON.parse(old)?.v > VERSION) return false;
      } catch {
        // unparsable: replaced
      }
    }
    store.setItem(KEY, JSON.stringify({ ...prefs, v: VERSION }));
    return true;
  } catch {
    return false;
  }
}

// The URL's say, over what was kept (7.7.4): for tests and shared links.
export function withUrl(prefs, params) {
  const out = { ...prefs };
  const take = (field, value) => {
    if (CHECKS[field](value)) out[field] = value;
  };
  if (params.has("lang")) take("lang", params.get("lang"));
  if (params.has("speed")) take("speed", params.get("speed"));
  if (params.has("talk")) take("talk", params.get("talk"));
  if (params.has("touren")) take("touren", Number(params.get("touren")));
  return out;
}

// The URL's other overrides, which are not preferences: the deal's seed, the
// human's seat, and the main-thread tutor.
export function urlOverrides(params) {
  const int = (k) => {
    if (!params.has(k)) return null;
    const n = Number(params.get(k));
    return Number.isInteger(n) && n >= 0 ? n : null;
  };
  const human = int("human");
  return { seed: int("seed"), human: human === 0 || human === 1 ? human : null, noworker: params.has("noworker") };
}

// The argument of strohmandeln_new (docs/PROTOCOL.md). `level` is the skill dial as it
// is (1, 2, 3, 3.5, 4) and `style` an id of docs/STYLES.md. The seed is drawn if none is given.
export function sittingConfig(prefs, { seed, human } = {}) {
  const cfg = {
    seed: Number.isInteger(seed) ? seed : Math.floor(Math.random() * 2 ** 32),
    touren: prefs.touren,
    ...(human === 0 || human === 1 ? { human } : {}),
    level: prefs.skill,
    style: prefs.style,
    settings: { both_pass: prefs.both_pass, values: prefs.values, ultimo: prefs.ultimo },
  };
  return cfg;
}

// How long the holds and flights of docs/TABLE3D.md 2 last, against Normal.
const SCALE = { slow: 1.6, normal: 1, fast: 0.6, instant: 0 };
export const speedScale = (speed) => SCALE[speed] ?? 1;
