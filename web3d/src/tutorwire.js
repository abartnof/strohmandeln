// The tutor's calls as the page makes them (docs/PROTOCOL.md "The tutor"):
// the text each takes, built from the table's own records. `call(name, text)`
// is the engine's (text in the alloc buffer, JSON out), `callNumber` the one
// call that answers a number. Stateless calls (evidence, learner, brief) take
// everything they need, so any instance of the module can answer them.

const numbers = (assisted) => (assisted ?? []).join(" ");
// Summaries oldest first, a line `--` between; a missing one is an empty entry
// (the engine lists it as stale).
const summaries = (list) => (list ?? []).map((s) => s ?? "").join("\n--\n");

export function tutorApi(call, callNumber) {
  return {
    // The record text of the deal that last ended: { record } or { record: null }.
    dealRecord: () => call("deal_record"),
    evidenceOf: (record, assisted) => call("evidence_of", `${record}\n==\n${numbers(assisted)}`),
    learner: (history) => call("learner", summaries(history)),
    briefOfRecord: (level, record, assisted, history) => call("brief_of_record", `level ${level}\n${record}\n==\n${numbers(assisted)}\n--\n${summaries(history)}`),
    // deals: [{ record, assisted, n }], the sitting's, oldest first.
    briefSitting: (level, deals, history) => {
      const body = deals.map((d) => `${Number.isInteger(d.n) && d.n > 0 ? `deal ${d.n}\n` : ""}${d.record}\n==\n${numbers(d.assisted)}`).join("\n++\n");
      return call("brief_sitting", `level ${level}\n${body}\n--\n${summaries(history)}`);
    },
    setFocus: (slug) => callNumber("set_focus", slug ?? ""),
    nudge: () => call("nudge"),
  };
}
