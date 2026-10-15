// What the tutor keeps between sittings (docs/TUTOR.md 8): the last sixty
// scored deals, each as its record (the engine's text of the deal), the
// actions a hint or a nudge was shown for, and its summary (the engine's
// text of what the deal showed of each skill), in the browser's storage.
// The page holds no judgement: the summaries are the engine's, and the
// records let it recompute any that a newer evidence calls stale, so a change
// to the evidence never drops a deal. Every access survives storage that is
// missing, full or blocked; the tutor then works from the deal just played.
//
// The store is injected (anything with getItem, setItem and removeItem), so a
// test never writes the real key. The old key `strohmandeln.progress` (a
// tally a sitting) is left in place and unread.
//
// Cassino's progress.js is the pattern.

export const KEY = "strohmandeln.tutor";
export const VERSION = 1;
export const MAX_DEALS = 60;
export const MAX_RECORD = 20_000; // a record's length in characters (a deal is about 2,000)
export const MAX_IMPORT = 1_000_000; // an import file's size
const MAX_ASSISTED = 200;
const FILE = "tutor";
// The engine's evidence text: a header and a line a skill (10 of them); the
// engine test checks that the engine's own output passes.
const SUMMARY_LINES = 11;

// Whether `text` reads as the engine's evidence summary, so what is kept (and
// later handed to the engine) is no more than that.
export function validSummary(text) {
  if (typeof text !== "string" || text.length > 2000 || !/^strohmandeln evidence v\d+\n/.test(text)) return false;
  const lines = text.replace(/\n$/, "").split("\n");
  return lines.length === SUMMARY_LINES && !lines.some((l) => ["--", "==", "++"].includes(l.trim()));
}

// The evidence version a valid summary was made by.
export const summaryVersion = (text) => (validSummary(text) ? Number(/^strohmandeln evidence v(\d+)/.exec(text)[1]) : null);

function read(store) {
  try {
    return store?.getItem(KEY) ?? null;
  } catch {
    return null;
  }
}

function parsed(store) {
  try {
    return JSON.parse(read(store) ?? "null");
  } catch {
    return null;
  }
}

const recordOk = (d) => Boolean(d) && typeof d.record === "string" && d.record.length > 0 && d.record.length <= MAX_RECORD;
const numbers = (a) => Array.isArray(a) && a.length <= MAX_ASSISTED && a.every((n) => Number.isInteger(n) && n >= 0 && n < 1000);

// Entries checked one by one: a record is text, anything else about it that
// is not as expected is treated as missing.
function sound(deals) {
  if (!Array.isArray(deals)) return [];
  return deals.filter(recordOk).map((d) => {
    const out = {
      record: d.record,
      assisted: numbers(d.assisted) ? d.assisted : [],
      summary: validSummary(d.summary) ? d.summary : null,
      at: Number.isFinite(d.at) ? d.at : 0,
    };
    if (Number.isInteger(d.n) && d.n >= 0) out.n = d.n;
    if (typeof d.sit === "string" && d.sit.length <= 40) out.sit = d.sit;
    return out;
  });
}

// Whether what is kept was written by a newer page: read as empty, and never
// written over (an older tab must not undo a newer one's progress).
export function isNewerStore(store) {
  const kept = parsed(store);
  return Boolean(kept) && typeof kept.v === "number" && kept.v > VERSION;
}

// The kept deals, oldest first; none when the store is missing, unreadable or
// written by a newer page.
export function loadDeals(store) {
  const kept = parsed(store);
  return kept && kept.v === VERSION ? sound(kept.deals).slice(-MAX_DEALS) : [];
}

// Whether it was kept.
function save(store, deals) {
  if (!store || isNewerStore(store)) return false;
  try {
    store.setItem(KEY, JSON.stringify({ v: VERSION, deals: deals.slice(-MAX_DEALS) }));
    return true;
  } catch {
    return false;
  }
}

// A scored deal: its record, the actions assisted, its summary if worked out
// (else null), when, and optionally its number and its sitting. Kept once,
// however often it is offered; the oldest go past sixty.
export function addDeal(store, deal) {
  const [d] = sound([deal]);
  if (!d) return false;
  const deals = loadDeals(store).filter((x) => x.record !== d.record);
  deals.push(d);
  return save(store, deals);
}

// The summary of a kept deal, once worked out.
export function setSummary(store, record, summary) {
  const deals = loadDeals(store);
  const d = deals.find((x) => x.record === record);
  if (!d || !validSummary(summary)) return false;
  d.summary = summary;
  return save(store, deals);
}

export function dropDeal(store, record) {
  return save(store, loadDeals(store).filter((d) => d.record !== record));
}

export function clearTutor(store) {
  try {
    store?.removeItem(KEY);
  } catch {
    // Not cleared: it was not readable either.
  }
}

// The summaries, oldest first, as the engine's learner and brief take them
// (null where one is missing).
export const historyOf = (deals) => deals.map((d) => d.summary ?? null);

// Recomputes one stale summary from its record, which the engine's learner
// lists (engine.learner(history).stale) after a change to the evidence. `tried`
// is the records already attempted this sitting. A summary made by a newer
// evidence than this engine's is not stale: another tab of a newer page made
// it, and recomputing would only trade the two versions back and forth.
//
// A record the engine says it cannot restore is dropped after that one try,
// not asked for again every sitting; a call that fails some other way (the
// engine has stopped) leaves it for the next sitting. Returns whether it tried
// one; the page calls it again, when idle, until it says no.
export async function refreshOne(store, engine, tried = new Set()) {
  const deals = loadDeals(store);
  if (!deals.length) return false;
  let learnt;
  try {
    learnt = await engine.learner(historyOf(deals));
  } catch (error) {
    console.error(error);
    return false;
  }
  if (!learnt) return false; // the engine has stopped
  const newer = (i) => learnt.version != null && (summaryVersion(deals[i].summary) ?? 0) > learnt.version;
  const at = (learnt.stale ?? []).find((i) => deals[i] && !tried.has(deals[i].record) && !newer(i));
  if (at === undefined) return false;
  const { record, assisted } = deals[at];
  tried.add(record);
  try {
    const summary = await engine.evidenceOf(record, assisted);
    if (validSummary(summary)) setSummary(store, record, summary);
    else if (!engine.stopped?.()) dropDeal(store, record);
  } catch (error) {
    console.error(error);
  }
  return true;
}

// The file the settings offer: what is kept, as text.
export function exportText(store) {
  return JSON.stringify({ strohmandeln: FILE, v: VERSION, deals: loadDeals(store) }, null, 1);
}

// A file chosen to import, replacing what is kept. { ok: true, count } or
// { ok: false, error } with a key of strings.json (`error.<error>`); a file
// that is refused changes nothing.
export function importText(store, text) {
  if (typeof text !== "string" || text.length > MAX_IMPORT) return { ok: false, error: "progress-big" };
  if (isNewerStore(store)) return { ok: false, error: "progress-newer" };
  let file;
  try {
    file = JSON.parse(text);
  } catch {
    return { ok: false, error: "progress-bad" };
  }
  if (!file || file.strohmandeln !== FILE || !Array.isArray(file.deals)) return { ok: false, error: "progress-bad" };
  if (file.v !== VERSION) return { ok: false, error: "progress-version" };
  if (!file.deals.every(recordOk)) return { ok: false, error: "progress-bad" };
  const deals = sound(file.deals).slice(-MAX_DEALS);
  if (!deals.length) return { ok: false, error: "progress-empty" };
  if (!save(store, deals)) return { ok: false, error: "progress-blocked" };
  return { ok: true, count: deals.length };
}
