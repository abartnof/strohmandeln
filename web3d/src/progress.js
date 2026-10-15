// What the tutor keeps between sittings (docs/TABLE3D.md 6.4.4): the tally and
// the tip of each of the last thirty sittings, in the browser's storage, for
// trends. Every access survives storage that is missing, full or blocked; the
// tutor then works from the sitting alone. The store is injected (anything
// with getItem, setItem and removeItem).
//
// Cassino's progress.js is the pattern; it keeps records, this keeps tallies.

export const KEY = "strohmandeln.progress";
export const VERSION = 1;
export const MAX = 30;

const int = (x) => Number.isInteger(x) && Math.abs(x) < 1e6;

// A sitting's review (docs/PROTOCOL.md "The review") as what is kept, or null.
export function summaryOf(review) {
  const tally = review?.bullets?.find((b) => b.key === "sitting-tally")?.params;
  if (!tally || !["won", "lost", "drawn", "net"].every((k) => int(tally[k]))) return null;
  const tip = review.bullets.find((b) => b.lead === "try")?.key;
  return { won: tally.won, lost: tally.lost, drawn: tally.drawn, net: tally.net, tip: typeof tip === "string" ? tip : null };
}

const sound = (s) => s && typeof s === "object" && ["won", "lost", "drawn", "net"].every((k) => int(s[k])) && (s.tip === null || typeof s.tip === "string");

function parsed(store) {
  try {
    return JSON.parse(store?.getItem(KEY) ?? "null");
  } catch {
    return null;
  }
}

const newer = (kept) => Boolean(kept) && typeof kept.v === "number" && kept.v > VERSION;

// The kept sittings, oldest first; none when the store is missing, unreadable
// or written by a newer page.
export function loadProgress(store) {
  const kept = parsed(store);
  if (!kept || kept.v !== VERSION || !Array.isArray(kept.sittings)) return [];
  return kept.sittings.filter(sound).map((s) => ({ won: s.won, lost: s.lost, drawn: s.drawn, net: s.net, tip: s.tip ?? null })).slice(-MAX);
}

// Whether it was kept. A newer page's store is not overwritten.
export function addSitting(store, summary) {
  if (!store || !sound({ tip: null, ...summary })) return false;
  if (newer(parsed(store))) return false;
  try {
    store.setItem(KEY, JSON.stringify({ v: VERSION, sittings: [...loadProgress(store), summary].slice(-MAX) }));
    return true;
  } catch {
    return false;
  }
}

export function clearProgress(store) {
  try {
    store?.removeItem(KEY);
  } catch {
    // Not cleared: it was not readable either.
  }
}

// The file the settings offer: what is kept, as text.
export function exportText(store) {
  return JSON.stringify({ v: VERSION, sittings: loadProgress(store) }, null, 2);
}
