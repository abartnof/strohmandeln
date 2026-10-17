// The brief as the page gets it (docs/TUTOR.md 6, 8, 10): when a deal is
// scored its record is taken from the engine and kept with the actions a hint
// or a nudge was shown for; its evidence is worked out once, with the brief
// (one advisor pass serves both), and kept with it (the summary the learner
// reads, and the detail the sitting's brief is made of); the focus is read
// from what is kept. The page holds no judgement: everything here is the
// engine's text and keys. Evidence runs on the main thread, after the deal has
// settled (the page schedules it): about a tenth of a second in the module,
// more on a loaded or slower machine (docs/PROTOCOL.md "Timing"), which is why
// the page shows the chip as working until it is done.
//
// `engine` is the page's engine (src/engine.js) and `store` an injected
// storage; with none, the brief rests on the deals of this page alone.
import { MAX_NUMBER, addDeal, historyOf, loadDeals, setEvidence, setSummary, summaryVersion, validSummary } from "./tutorstore.js";

export function createBriefing({ store, engine, level = () => 4, head = () => ({}), now = () => Math.floor(Date.now() / 1000) }) {
  let memory = []; // the deals of this page, whatever the store keeps
  const cache = new Map(); // record -> brief

  // The sitting's style for the engine: head() is { style, drawn, over }. What Random drew is
  // handed over, with `reveal`, only once the sitting is over; the engine withholds it otherwise.
  const styleOf = () => {
    const { style, drawn, over } = head();
    if (!style) return undefined;
    return over ? { style, drawn: drawn ?? undefined, reveal: true } : { style };
  };

  // The deals the tutor reads: the store's, else this page's.
  const kept = () => {
    const stored = loadDeals(store);
    return stored.length ? stored : memory.slice(-60);
  };

  // Keeps what the engine worked out for a deal (the store's copy and the page's).
  function keepEvidence(record, summary, detail) {
    const saved = detail ? setEvidence(store, record, summary, detail) : setSummary(store, record, summary);
    const m = memory.find((d) => d.record === record);
    if (m) Object.assign(m, { summary, ...(detail ? { detail } : {}) });
    return saved;
  }

  return {
    // The deal just scored (call right after its `game_scored` event, before
    // another deal can end): { record, assisted, summary: null, detail: null,
    // at, n, sit } or null if the engine has no record.
    recordDeal({ assisted = [], n, sit }) {
      const record = engine.dealRecord()?.record;
      if (typeof record !== "string") return null;
      const entry = { record, assisted: [...new Set(assisted)].sort((a, b) => a - b), summary: null, detail: null, at: now() };
      if (Number.isInteger(n) && n > 0) entry.n = n;
      if (sit) entry.sit = String(sit);
      const at = memory.findIndex((d) => d.record === record);
      if (at >= 0) memory.splice(at, 1);
      memory.push(entry);
      addDeal(store, entry);
      return entry;
    },

    // The brief of a recorded deal: { bullets, method, summary, detail, focus },
    // or null if the engine cannot make one. Worked out once; the deal's summary
    // and detail are kept. The history is the summaries of the deals kept before it.
    briefOf(entry) {
      if (!entry) return null;
      if (cache.has(entry.record)) return cache.get(entry.record);
      const all = kept();
      const at = all.findIndex((d) => d.record === entry.record);
      const history = historyOf(at >= 0 ? all.slice(0, at) : all);
      const r = engine.briefOfRecord(level(), entry.record, entry.assisted, history, styleOf());
      const brief = r?.brief ?? null;
      if (brief && validSummary(brief.summary)) keepEvidence(entry.record, brief.summary, brief.detail);
      if (brief) cache.set(entry.record, brief);
      return brief;
    },

    // The brief of the sitting `sit`: its kept deals as one game, against what
    // was kept before them. It is made of what was kept when each deal was scored
    // and runs no advisor; a deal kept without its detail, or with one of an older
    // evidence, has it worked out once. If the engine still refuses a detail
    // (damaged in storage) every detail is worked out again, once, and the call
    // retried: one bad deal must not blank the sitting's brief. null if the
    // sitting has no deal, or the engine cannot.
    sittingBrief(sit) {
      const all = kept();
      const mine = all.filter((d) => d.sit === String(sit));
      if (!mine.length) return null;
      const version = engine.learner([])?.version;
      const compute = (d) => {
        const got = engine.evidenceOf(d.record, d.assisted);
        if (!got?.detail || !validSummary(got.summary)) return null;
        keepEvidence(d.record, got.summary, got.detail);
        return got.detail;
      };
      const current = (d) => d.detail && summaryVersion(d.detail.summary) === version;
      const deals = (again) => {
        const out = [];
        for (const [i, d] of mine.entries()) {
          const detail = !again && current(d) ? d.detail : compute(d);
          if (detail) out.push({ n: Number.isInteger(d.n) && d.n >= 1 && d.n <= MAX_NUMBER ? d.n : i + 1, detail });
        }
        return out;
      };
      const before = historyOf(all.slice(0, all.indexOf(mine[0])));
      let list = deals(false);
      if (!list.length) return null;
      let r = engine.briefSitting(level(), list, before, styleOf());
      if (r?.error === "bad-input") {
        list = deals(true);
        r = list.length ? engine.briefSitting(level(), list, before, styleOf()) : null;
      }
      return r?.brief ?? null;
    },

    // The skill to work on, from every summary kept (a slug or null).
    focus() {
      const r = engine.learner(historyOf(kept()));
      return r?.focus ?? null;
    },

    // How many deals are kept (the store's, else this page's).
    count: () => kept().length,

    // What was cleared from the store is cleared from the page's memory too.
    forget() {
      memory = [];
      cache.clear();
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
