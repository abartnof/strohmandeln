// The tutor's calls as the page makes them (docs/PROTOCOL.md "The tutor"):
// the text each takes, built from the table's own records. `call(name, text)`
// is the engine's (text in the alloc buffer, JSON out), `callNumber` the one
// call that answers a number. Stateless calls (evidence, learner, brief) take
// everything they need, so any instance of the module can answer them.

const numbers = (assisted) => (assisted ?? []).join(" ");
// Summaries oldest first, a line `--` between; a missing one is an empty entry
// (the engine lists it as stale).
const summaries = (list) => (list ?? []).map((s) => s ?? "").join("\n--\n");

// The brief calls' header: `level n`, then the optional lines the engine reads
// (docs/PROTOCOL.md, "The tutor"): the sitting's style, what Random drew and
// `reveal` (the sitting is over; the engine withholds `drawn` without it).
export function briefHead(level, { style, drawn, reveal } = {}) {
  return [`level ${level}`, style ? `style ${style}` : null, drawn ? `drawn ${drawn}` : null, reveal ? "reveal" : null].filter(Boolean).join("\n");
}

export function tutorApi(call, callNumber) {
  return {
    // The record text of the deal that last ended: { record } or { record: null }.
    dealRecord: () => call("deal_record"),
    evidenceOf: (record, assisted) => call("evidence_of", `${record}\n==\n${numbers(assisted)}`),
    learner: (history) => call("learner", summaries(history)),
    briefOfRecord: (level, record, assisted, history, style) => call("brief_of_record", `${briefHead(level, style)}\n${record}\n==\n${numbers(assisted)}\n--\n${summaries(history)}`),
    // deals: [{ n, detail }], the sitting's, oldest first: what was kept of each deal when it was
    // scored (docs/PROTOCOL.md "The tutor"); the advisor does not run.
    briefSitting: (level, deals, history, style) => {
      const body = deals.map((d) => `deal ${d.n}\n${JSON.stringify(d.detail)}`).join("\n++\n");
      return call("brief_sitting", `${briefHead(level, style)}\n${body}\n--\n${summaries(history)}`);
    },
    setFocus: (slug) => callNumber("set_focus", slug ?? ""),
    nudge: () => call("nudge"),
  };
}
