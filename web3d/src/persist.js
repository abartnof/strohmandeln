// Saving and restoring a sitting (docs/TABLE3D.md 9). The sitting is the
// module's own export (docs/PROTOCOL.md: seed, settings, Touren, every move),
// kept with the page's extras (language, the talk's record, tutorial pages
// seen, the human's seat, the slate's snapshot). Every access is in
// try/catch: a missing, full or blocked store means a sitting played
// unsaved, which the page says once.
//
// The page calls saveSitting after every accepted action and opponent step,
// loadSitting on load (to offer Continue), and hands the result to
// strohmandeln_import; clearSitting at the farewell.

export const KEY = "strohmandeln.sitting";
export const VERSION = 1;
export const PROTOCOL = 1;
export const MAX_SAVED = 200 * 1024; // bytes (9.1)
export const MAX_FILE = 1024 * 1024; // bytes (9.3)
export const FILE_NAME = "strohmandeln-sitting.json";

// Size in UTF-8 bytes: the caps are bytes, and "ä" is two of them.
const bytes = (text) => new TextEncoder().encode(text).length;

function defaultStore() {
  try {
    return globalThis.localStorage ?? null;
  } catch {
    return null;
  }
}

// The export the module gives is {protocol, human, level, record}.
function validExport(x) {
  return Boolean(x) && typeof x === "object" && x.protocol === PROTOCOL && Boolean(x.record) && typeof x.record === "object" && x.record.version === 1 && Array.isArray(x.record.moves);
}

// Keep the sitting. { over: true } clears instead: a finished sitting is not
// kept (9.4). Returns {ok: true} or {ok: false, reason: "too-big" |
// "unavailable"}.
export function saveSitting(exported, extras, { over = false } = {}, store = defaultStore()) {
  try {
    if (!store) return { ok: false, reason: "unavailable" };
    if (over) {
      store.removeItem(KEY);
      return { ok: true };
    }
    const text = JSON.stringify({ v: VERSION, export: exported, extras: extras ?? {} });
    if (bytes(text) > MAX_SAVED) return { ok: false, reason: "too-big" };
    store.setItem(KEY, text);
    return { ok: true };
  } catch {
    return { ok: false, reason: "unavailable" };
  }
}

// The sitting kept, or null: none, damaged, from another version, or over.
export function loadSitting(store = defaultStore()) {
  try {
    const raw = store?.getItem(KEY);
    if (!raw) return null;
    const kept = JSON.parse(raw);
    if (!kept || kept.v !== VERSION || kept.over || !validExport(kept.export)) return null;
    return { export: kept.export, extras: kept.extras && typeof kept.extras === "object" ? kept.extras : {} };
  } catch {
    return null;
  }
}

export function clearSitting(store = defaultStore()) {
  try {
    store?.removeItem(KEY);
  } catch {
    // Not cleared, this time.
  }
}

// The file of "Save the sitting" (9.3): {v, protocol, export, extras}.
export function fileText(exported, extras) {
  return JSON.stringify({ v: VERSION, protocol: PROTOCOL, export: exported, extras: extras ?? {} });
}

// The file of "Load a sitting": {ok: true, export, extras}, or {ok: false,
// error} with a key of strings.json (`error.<error>`); nothing is changed
// either way.
export function parseFile(text) {
  if (typeof text !== "string" || bytes(text) > MAX_FILE) return { ok: false, error: "load-big" };
  try {
    const f = JSON.parse(text);
    if (!f || f.v !== VERSION || f.protocol !== PROTOCOL || !validExport(f.export)) return { ok: false, error: "load-bad" };
    return { ok: true, export: f.export, extras: f.extras && typeof f.extras === "object" ? f.extras : {} };
  } catch {
    return { ok: false, error: "load-bad" };
  }
}
