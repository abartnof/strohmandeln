import test from "node:test";
import assert from "node:assert/strict";
import { tutorApi, briefHead } from "../src/tutorwire.js";
import { createBriefing } from "../src/briefing.js";
import { fakeStore } from "./support/fakedom.js";

const sent = () => {
  const calls = [];
  return { calls, api: tutorApi((name, text) => (calls.push([name, text]), {}), () => 0) };
};

test("the brief's header is `level n`, then the style lines the engine reads (PROTOCOL, the tutor)", () => {
  assert.equal(briefHead(3), "level 3");
  assert.equal(briefHead(3, {}), "level 3");
  assert.equal(briefHead(4, { style: "normal" }), "level 4\nstyle normal");
  assert.equal(briefHead(3, { style: "random" }), "level 3\nstyle random");
  assert.equal(briefHead(3, { style: "random", drawn: "ms", reveal: true }), "level 3\nstyle random\ndrawn ms\nreveal");
  assert.equal(briefHead(3, { style: "bold", reveal: true }), "level 3\nstyle bold\nreveal");
});

test("both brief calls carry the header", () => {
  const { calls, api } = sent();
  api.briefOfRecord(3, "{}", [1], [], { style: "bermann" });
  api.briefSitting(3, [{ n: 1, detail: {} }], [], { style: "random", drawn: "hunter", reveal: true });
  assert.match(calls[0][1], /^level 3\nstyle bermann\n\{\}\n==\n1\n--\n/);
  assert.match(calls[1][1], /^level 3\nstyle random\ndrawn hunter\nreveal\ndeal 1\n/);
  api.briefOfRecord(3, "{}", [], []);
  assert.match(calls[2][1], /^level 3\n\{\}/, "no style: the engine's default, Normal");
});

test("the briefing hands the engine the sitting's style: the after-deal brief without a reveal", () => {
  const seen = [];
  const engine = { briefOfRecord: (...a) => (seen.push(a), { brief: { bullets: [], summary: null, detail: null } }), learner: () => ({ version: 1 }) };
  const b = createBriefing({ store: fakeStore(), engine, level: () => 3, head: () => ({ style: "random" }) });
  b.briefOf({ record: "rec", assisted: [] });
  assert.deepEqual(seen[0][4], { style: "random" });
});
