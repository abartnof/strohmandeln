// The brief as the page gets it (docs/TUTOR.md 6, 8, 10): when a deal is
// scored its record is taken from the engine and kept with the actions a hint
// or a nudge was shown for; its evidence is worked out once, with the brief
// (one advisor pass serves both), and stored; the focus is read from what is
// kept. The page holds no judgement: everything here is the engine's text and
// keys. Evidence runs on the main thread, after the deal has settled (the
// page schedules it): a deal's is some tens of milliseconds, in the module
// about a tenth of a second.
//
// `engine` is the page's engine (src/engine.js) and `store` an injected
// storage; with none, the brief rests on the deals of this page alone.
import { addDeal, historyOf, loadDeals, setSummary, validSummary } from "./tutorstore.js";

export function createBriefing({ store, engine, level = () => 4, now = () => Math.floor(Date.now() / 1000) }) {
  const memory = []; // the deals of this page, whatever the store keeps
  const cache = new Map(); // record -> brief

  // The deals the tutor reads: the store's, else this page's.
  const kept = () => {
    const stored = loadDeals(store);
    return stored.length ? stored : memory.slice(-60);
  };

  return {
    // The deal just scored (call right after its `game_scored` event, before
    // another deal can end): { record, assisted, summary: null, at, n, sit }
    // or null if the engine has no record.
    recordDeal({ assisted = [], n, sit }) {
      const record = engine.dealRecord()?.record;
      if (typeof record !== "string") return null;
      const entry = { record, assisted: [...new Set(assisted)].sort((a, b) => a - b), summary: null, at: now() };
      if (Number.isInteger(n) && n > 0) entry.n = n;
      if (sit) entry.sit = String(sit);
      const at = memory.findIndex((d) => d.record === record);
      if (at >= 0) memory.splice(at, 1);
      memory.push(entry);
      addDeal(store, entry);
      return entry;
    },

    // The brief of a recorded deal: { bullets, method, summary, focus }, or
    // null if the engine cannot make one. Worked out once; the deal's summary
    // is stored. The history is the summaries of the deals kept before it.
    briefOf(entry) {
      if (!entry) return null;
      if (cache.has(entry.record)) return cache.get(entry.record);
      const all = kept();
      const at = all.findIndex((d) => d.record === entry.record);
      const history = historyOf(at >= 0 ? all.slice(0, at) : all);
      const r = engine.briefOfRecord(level(), entry.record, entry.assisted, history);
      const brief = r?.brief ?? null;
      if (brief && validSummary(brief.summary)) {
        setSummary(store, entry.record, brief.summary);
        const m = memory.find((d) => d.record === entry.record);
        if (m) m.summary = brief.summary;
      }
      if (brief) cache.set(entry.record, brief);
      return brief;
    },

    // The brief of the sitting `sit`: its kept deals as one game, against what
    // was kept before them. null if it has none, or the engine cannot.
    sittingBrief(sit) {
      const all = kept();
      const mine = all.filter((d) => d.sit === String(sit));
      if (!mine.length) return null;
      const first = all.indexOf(mine[0]);
      const r = engine.briefSitting(level(), mine, historyOf(all.slice(0, first)));
      return r?.brief ?? null;
    },

    // The skill to work on, from every summary kept (a slug or null).
    focus() {
      const r = engine.learner(historyOf(kept()));
      return r?.focus ?? null;
    },

    // Sets the skill the nudge shows (null: none).
    setFocus(slug) {
      engine.setFocus(slug ?? "");
    },

    // The nudge for the position now: { skill, key, deal_action } or null.
    nudge() {
      const r = engine.nudge();
      return r?.nudge ? { ...r.nudge, deal_action: r.deal_action } : null;
    },
  };
}
