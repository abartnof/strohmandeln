// The brief as the page gets it (web3d/src/briefing.js): the deal's record
// taken when it is scored, its evidence worked out once, stored, and the
// focus read from what is kept. Run against the real module (bin/gate builds
// it), with an injected store, so the real key is never written.
import test from "node:test";
import assert from "node:assert/strict";
import { createBriefing } from "../src/briefing.js";
import { KEY, loadDeals, validSummary } from "../src/tutorstore.js";
import { playSitting, random } from "./sim.js";

const memory = () => {
  const m = new Map();
  return { getItem: (k) => (m.has(k) ? m.get(k) : null), setItem: (k, v) => m.set(k, v), removeItem: (k) => m.delete(k), m };
};

// A sitting played by random legal actions; `onScored(engine, n)` after each scored deal.
async function sittingWith(seed, onScored, { limit = 4000 } = {}) {
  let scored = 0;
  await playSitting({ seed, touren: 3, human: 0 }, {
    policy: random(seed),
    limit,
    onReply: (reply, engine) => {
      if (reply.events?.some((e) => e.type === "game_scored")) onScored(engine, ++scored);
    },
  });
  return scored;
}

test("a scored deal is kept with its record, then briefed once, and its summary is stored", async () => {
  const store = memory();
  let briefing;
  const briefs = [];
  const scored = await sittingWith(11, (engine, n) => {
    briefing ??= createBriefing({ store, engine, level: () => 3, now: () => 1_760_000_000 + n });
    const entry = briefing.recordDeal({ assisted: [], n, sit: "11" });
    assert.equal(JSON.parse(entry.record).v, 1);
    const b = briefing.briefOf(entry);
    briefs.push(b);
    assert.ok(b.bullets.length >= 1 && b.bullets.length <= 3, JSON.stringify(b));
    assert.equal(briefing.briefOf(entry), b, "worked out once");
  }, { limit: 1500 });
  assert.ok(scored >= 2, `${scored} deals`);
  const kept = loadDeals(store);
  assert.equal(kept.length, scored);
  for (const d of kept) {
    assert.ok(validSummary(d.summary), "the engine's own text passes the page's check");
    assert.equal(d.sit, "11");
    assert.ok(d.detail && d.detail.summary === d.summary, "the detail the sitting's brief is made of is kept with it");
  }
  assert.equal(JSON.parse(store.getItem(KEY)).v, 1);
  // the stored summary is the brief's
  assert.equal(kept.at(-1).summary, briefs.at(-1).summary);
});

test("the focus is read from what is kept; with no storage the brief rests on the deal alone", async () => {
  const store = memory();
  let briefing;
  let bare;
  await sittingWith(12, (engine, n) => {
    briefing ??= createBriefing({ store, engine, level: () => 3 });
    bare ??= createBriefing({ store: null, engine, level: () => 3 });
    briefing.briefOf(briefing.recordDeal({ assisted: [], n, sit: "12" }));
    const e2 = bare.recordDeal({ assisted: [], n, sit: "12" });
    const b = bare.briefOf(e2);
    assert.ok(b.bullets.length >= 1, "no storage still briefs");
    assert.ok([null, ...["worth", "cheap", "off", "run", "pagat", "void", "behind", "endgame", "bid", "announce"]].includes(briefing.focus()));
  }, { limit: 1500 });
  assert.deepEqual(loadDeals(null), []);
});

test("the sitting's brief covers the deals of its sitting only, and runs no advisor", async () => {
  const store = memory();
  let briefing;
  let engineSeen;
  await sittingWith(13, (engine, n) => {
    engineSeen = engine;
    briefing ??= createBriefing({ store, engine, level: () => 3 });
    briefing.briefOf(briefing.recordDeal({ assisted: [], n, sit: "13" }));
  }, { limit: 1500 });
  const other = createBriefing({ store, engine: engineSeen, level: () => 3 });
  assert.equal(other.sittingBrief("nobody"), null);
  // every call that works evidence out is counted: the sitting's brief makes none
  const counted = [];
  const spy = { ...engineSeen, evidenceOf: (...a) => (counted.push("evidenceOf"), engineSeen.evidenceOf(...a)), briefOfRecord: (...a) => (counted.push("briefOfRecord"), engineSeen.briefOfRecord(...a)) };
  const b = createBriefing({ store, engine: spy, level: () => 3 }).sittingBrief("13");
  assert.deepEqual(counted, [], "the advisor ran for the sitting's brief");
  assert.deepEqual(b, briefing.sittingBrief("13"));
  assert.ok(b.bullets.every((x) => x.lead !== "tutor.lead.deal" || false));
  const again = briefing.sittingBrief("13");
  assert.ok(b.bullets.length >= 1 && b.bullets.length <= 3);
  assert.ok(validSummary(b.summary));
  for (const x of b.bullets) assert.ok(x.key.startsWith("tutor."));
  assert.ok(again);
});

test("a deal kept without its detail has it worked out for the sitting's brief once", async () => {
  const store = memory();
  let engineSeen;
  let sitting;
  await sittingWith(16, (engine, n) => {
    engineSeen = engine;
    const briefing = createBriefing({ store, engine, level: () => 3 });
    const e = briefing.recordDeal({ assisted: [], n, sit: "16" });
    sitting = briefing;
    // an old entry: the summary only
    briefing.briefOf(e);
  }, { limit: 1500 });
  const deals = loadDeals(store).map((d) => ({ ...d, detail: null }));
  store.setItem(KEY, JSON.stringify({ v: 1, deals }));
  const counted = [];
  const spy = { ...engineSeen, evidenceOf: (...a) => (counted.push("evidenceOf"), engineSeen.evidenceOf(...a)) };
  const b = createBriefing({ store, engine: spy, level: () => 3 }).sittingBrief("16");
  assert.ok(b.bullets.length >= 1);
  assert.equal(counted.length, deals.length, "once for each, and then kept");
  assert.ok(loadDeals(store).every((d) => d.detail));
  assert.ok(sitting);
});

test("forgetting what is kept empties the page's own memory too, so the focus does not come back", async () => {
  const store = memory();
  let briefing;
  await sittingWith(17, (engine, n) => {
    briefing ??= createBriefing({ store, engine, level: () => 3 });
    briefing.briefOf(briefing.recordDeal({ assisted: [], n, sit: "17" }));
  }, { limit: 1500 });
  assert.ok(briefing.count() > 0);
  store.removeItem(KEY);
  assert.ok(briefing.count() > 0, "the memory outlives the store, which is what forget() is for");
  briefing.forget();
  assert.equal(briefing.count(), 0);
  assert.equal(briefing.focus(), null);
  assert.equal(briefing.sittingBrief("17"), null);
});

test("an assisted decision is not counted: the numbers go with the record", async () => {
  const store = memory();
  let seen = null;
  await sittingWith(14, (engine, n) => {
    if (n > 1) return;
    const briefing = createBriefing({ store, engine, level: () => 3 });
    const all = Array.from({ length: 60 }, (_, i) => i);
    const entry = briefing.recordDeal({ assisted: all, n, sit: "14" });
    seen = briefing.briefOf(entry);
  }, { limit: 1500 });
  const tallies = seen.summary.split("\n").slice(1).map((l) => Number(l.split(" ")[1]));
  assert.ok(tallies.every((c) => c === 0), seen.summary);
  assert.equal(seen.bullets[0].key, "tutor.short", "nothing counted: a short deal");
});

test("the nudge follows the focus the page sets", async () => {
  const store = memory();
  let asked = 0;
  let shown = 0;
  await playSitting({ seed: 15, touren: 3, human: 0 }, {
    policy: random(15),
    limit: 300,
    onReply: (reply, engine) => {
      const briefing = createBriefing({ store, engine, level: () => 3 });
      briefing.setFocus("worth");
      if (reply.state.prompt === "you" && reply.state.phase === "play") {
        asked++;
        const n = briefing.nudge();
        if (n) {
          shown++;
          assert.equal(n.skill, "worth");
          assert.equal(n.key, "tutor.nudge.worth");
          assert.equal(n.deal_action, reply.state.deal_actions);
        }
      }
    },
  });
  assert.ok(asked > 5);
  assert.ok(shown > 0);
});

// ---- one bad stored deal must not blank the sitting's brief -----------------------------

// A played sitting kept in a store, then damaged by `damage(entry, index)` (raw JSON).
async function damaged(seed, damage) {
  const store = memory();
  let engineSeen;
  let scored = 0;
  await sittingWith(seed, (engine, n) => {
    engineSeen = engine;
    const briefing = createBriefing({ store, engine, level: () => 3 });
    briefing.briefOf(briefing.recordDeal({ assisted: [], n, sit: String(seed) }));
    scored = n;
  }, { limit: 1500 });
  assert.ok(scored >= 2, `${scored} deals`);
  const raw = JSON.parse(store.getItem(KEY));
  raw.deals.forEach(damage);
  store.setItem(KEY, JSON.stringify(raw));
  const counted = [];
  const spy = { ...engineSeen, evidenceOf: (...a) => (counted.push(a[0]), engineSeen.evidenceOf(...a)) };
  return { store, counted, briefing: createBriefing({ store, engine: spy, level: () => 3 }), sit: String(seed), deals: raw.deals.length };
}

test("a stored deal number of 0, or of 2^33, does not blank the sitting's brief", async () => {
  for (const [seed, n] of [[21, 0], [22, 2 ** 33], [23, -4]]) {
    const { briefing, sit, counted } = await damaged(seed, (d, i) => i === 0 && (d.n = n));
    const b = briefing.sittingBrief(sit);
    assert.ok(b && b.bullets.length >= 1, `n ${n}`);
    assert.deepEqual(counted, [], "no advisor for a number");
  }
});

test("a detail with a damaged miss is worked out again once, and the brief is made", async () => {
  const { briefing, store, sit, counted, deals } = await damaged(24, (d, i) => i === 0 && (d.detail.misses = { off: { cost: 1 } }));
  const b = briefing.sittingBrief(sit);
  assert.ok(b && b.bullets.length >= 1);
  assert.equal(counted.length, deals, "the engine does not say which: every detail of the sitting, once");
  assert.notDeepEqual(loadDeals(store)[0].detail.misses, { off: { cost: 1 } }, "and kept mended");
  assert.equal(briefing.sittingBrief(sit) !== null, true);
  assert.equal(counted.length, deals, "not again");
});

test("a detail whose summary is of an older evidence counts as missing and is worked out again", async () => {
  const { briefing, store, sit, counted } = await damaged(25, (d, i) => i === 0 && (d.detail.summary = d.detail.summary.replace(/evidence v\d+/, "evidence v1")));
  const b = briefing.sittingBrief(sit);
  assert.ok(b && b.bullets.length >= 1);
  assert.equal(counted.length, 1);
  assert.match(loadDeals(store)[0].detail.summary, /evidence v3|evidence v[2-9]/);
});

test("when the engine still refuses, the brief is null and nothing is looped", async () => {
  const { store, sit } = await damaged(26, () => {});
  let calls = 0;
  const engine = {
    briefSitting: () => (calls++, { error: "bad-input", brief: null }),
    evidenceOf: () => ({ error: "replay" }),
    learner: () => ({ version: 3 }),
  };
  const briefing = createBriefing({ store, engine, level: () => 3 });
  assert.equal(briefing.sittingBrief(sit), null);
  assert.ok(calls <= 2, `${calls} tries`);
});
