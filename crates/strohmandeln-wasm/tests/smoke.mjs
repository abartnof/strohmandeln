// The WebAssembly module in Node, as the page would use it. Three checks:
//
// 1. The golden sittings (tests/golden.txt): the fixed policy must produce
//    replies that hash exactly as the native build's did.
// 2. Whole sittings by random legal actions: each ends with a settlement
//    (a sitting_end event), and no hidden card ever appears in the human's
//    state or events (the module's audit export lists them).
// 3. Export then import restores the same state.
//
//   cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
//   node crates/strohmandeln-wasm/tests/smoke.mjs [random sittings]
import { readFileSync } from "node:fs";
import assert from "node:assert/strict";
import { loadWasm } from "../../../kit/js/engine.js";

const path = new URL("../../../target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm", import.meta.url);
const { exports: w, out, write } = await loadWasm(readFileSync(path), "strohmandeln");
const raw = () => new Uint8Array(w.memory.buffer, w.strohmandeln_out(), w.strohmandeln_out_len()).slice();

const create = (config) => {
  assert.equal(w.strohmandeln_new(write(JSON.stringify(config))), 1, JSON.stringify(out()));
  return out();
};
const send = (action) => w.strohmandeln_send(write(JSON.stringify(action))) === 1;
const audit = () => (w.strohmandeln_audit(), out().hidden);
w.strohmandeln_styles(), assert.deepEqual(out().styles, ["bermann", "ms", "bold", "cautious"]); // the dialog's list, which Random draws from

// FNV-1a, 64 bits, as strohmandeln_wasm::fnv, on 32-bit halves (BigInt is far
// too slow for tens of megabytes). The prime is 2^40 + 0x1b3.
const fnv = (chunks) => {
  let hi = 0xcbf29ce4;
  let lo = 0x84222325;
  for (const c of chunks) {
    for (let i = 0; i < c.length; i++) {
      lo = (lo ^ c[i]) >>> 0;
      const m = lo * 0x1b3;
      const carry = Math.floor(m / 4294967296);
      const nlo = m >>> 0;
      const nhi = (hi * 0x1b3 + carry + ((lo << 8) >>> 0)) >>> 0;
      // hi * 0x1b3 can exceed 2^53 only if hi > 2^44: it cannot (hi < 2^32).
      hi = nhi;
      lo = nlo;
    }
  }
  return (BigInt(hi) << 32n) | BigInt(lo);
};

// The opponent's `dealt` (their 15 cards) never reaches the human, in the
// events of a reply or in the state's own list of the deal's events.
let dealtSeen = 0;
const onlyOwnDealt = (r) => {
  for (const e of [...r.events, ...(r.state?.events ?? [])]) {
    if (e.type !== "dealt") continue;
    dealtSeen++;
    assert.equal(e.seat, r.state.human, "the opponent's dealt reached the human");
  }
};

// The wire form of a play is the card alone; the source is for display.
const wire = (a) => (a.action === "play" ? { action: "play", card: a.card } : a);

// 1. The golden sittings.
const golden = readFileSync(new URL("./golden.txt", import.meta.url), "utf8")
  .split("\n")
  .filter((l) => l.trim() && !l.startsWith("#"));
let bad = 0;
for (const line of golden) {
  const [seed, touren, human, want, wantEvents, contos, balance] = line.trim().split(/\s+/);
  let r = create({ seed: Number(seed), touren: Number(touren), human: Number(human), level: 1 });
  onlyOwnDealt(r);
  const chunks = [raw()];
  let events = r.events.length;
  while (r.state.prompt !== "over") {
    if (r.state.prompt === "opponent") assert.equal(w.strohmandeln_step(), 1);
    else {
      const a = r.state.phase === "extend" ? { action: "extend", accept: false } : wire(r.state.legal[0]);
      assert.ok(send(a), JSON.stringify(out()));
    }
    chunks.push(raw());
    r = out();
    onlyOwnDealt(r);
    events += r.events.length;
  }
  const ok = String(fnv(chunks)) === want && String(events) === wantEvents &&
    JSON.stringify(r.state.contos) === contos && String(r.state.balance) === balance;
  if (!ok) {
    bad++;
    console.log(`MISMATCH ${line}: the module's sitting hashes ${fnv(chunks)}, ${events} events`);
  }
}
assert.equal(bad, 0, "the module must play the golden sittings as the native build does");
assert.ok(dealtSeen > 6, "the human's own dealt events were seen");
console.log(`golden sittings: ${golden.length} of ${golden.length} play exactly as the native build; ${dealtSeen} dealt events, all the human's`);

// 2. Whole sittings by random legal actions.
const mulberry = (a) => () => {
  a = (a + 0x6d2b79f5) | 0;
  let t = Math.imul(a ^ (a >>> 15), 1 | a);
  t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
  return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
};
const sittings = Number(process.argv[2] ?? 4);
let moves = 0;
let looked = 0;
for (let i = 0; i < sittings; i++) {
  const rnd = mulberry(1000 + i);
  let r = create({ seed: 500 + i, touren: i % 2 ? 6 : 3, human: i % 2, level: 1 });
  let settled = false;
  const check = () => {
    onlyOwnDealt(r);
    settled ||= r.events.some((e) => e.type === "sitting_end");
    const from = Math.max(0, r.events.map((e) => e.type).lastIndexOf("deal_start"));
    const seen = JSON.stringify(r.state) + JSON.stringify(r.events.slice(from));
    const hidden = audit();
    for (const code of hidden) {
      assert.ok(!seen.includes(`"${code}"`), `hidden card ${code} reached the human`);
      looked++;
    }
    // The tracker names every card, but a hidden one only as unseen.
    w.strohmandeln_tracker();
    const tracker = out().tracker;
    assert.equal(tracker === null, r.state.deal === null);
    if (tracker) {
      assert.equal(Object.keys(tracker.cards).length, 54);
      for (const code of hidden) assert.equal(tracker.cards[code].place, "unseen", `the tracker placed hidden ${code}`);
    }
  };
  check();
  while (r.state.prompt !== "over") {
    if (r.state.prompt === "opponent") assert.equal(w.strohmandeln_step(), 1);
    else {
      const legal = r.state.legal;
      // Refusals are legal answers to an illegal try: poke one in now and then.
      if (moves % 50 === 0) {
        const refused = send({ action: "play", card: "ZZ" });
        assert.ok(!refused);
        assert.ok(out().error === "bad-action" || out().refusal?.key);
      }
      assert.ok(send(wire(legal[Math.floor(rnd() * legal.length)])), JSON.stringify(out()));
    }
    r = out();
    moves++;
    check();
  }
  assert.ok(settled, "the sitting ends with a settlement");
  const end = r.events.find((e) => e.type === "sitting_end");
  assert.ok(end && Number.isInteger(end.kreuzer));
}
assert.ok(looked > 500, "the audit looked at something");
console.log(`${sittings} random sittings: all settled, ${moves} moves, ${looked} hidden-card checks, none leaked`);

// 3. Export then import.
create({ seed: 77, touren: 3, human: 0, level: 1 });
for (let n = 0; n < 40; n++) {
  w.strohmandeln_render();
  const s = out().state;
  if (s.prompt === "opponent") w.strohmandeln_step();
  else if (s.prompt === "you") send(wire(s.legal[0]));
}
w.strohmandeln_render();
const before = out().state;
w.strohmandeln_export();
const saved = out();
assert.equal(w.strohmandeln_import(write(JSON.stringify(saved))), 1);
assert.deepEqual(out().state, before);
console.log("export/import restores the state");

// 4. The hint (level-4 search): a legal move with a reason key, on the human's turn only.
create({ seed: 78, touren: 3, human: 0, level: 3.5 });
let hinted = 0;
let worst = 0;
for (let n = 0; n < 60; n++) {
  w.strohmandeln_render();
  const s = out().state;
  if (s.prompt === "opponent") {
    w.strohmandeln_hint();
    assert.equal(out().hint, null);
    assert.equal(out().reason, "not-your-turn");
    assert.equal(w.strohmandeln_step(), 1);
  } else if (s.prompt === "you") {
    const t0 = performance.now();
    w.strohmandeln_hint();
    worst = Math.max(worst, performance.now() - t0);
    const h = out();
    assert.ok(s.legal.some((a) => JSON.stringify(a) === JSON.stringify(h.hint)), JSON.stringify(h));
    assert.match(h.reason, /^[a-z]+(-[a-z]+)*$/);
    assert.equal(typeof h.params, "object");
    if (h.hint.action === "play") assert.equal(h.params.card, h.hint.card, "the reason names the card");
    assert.ok(h.rule === null || typeof h.rule === "string");
    assert.ok(h.odds === null || (h.odds.worlds >= h.odds.better && h.odds.gain_thirds >= 1), JSON.stringify(h.odds));
    hinted++;
    assert.ok(send(wire(h.hint)), "the hint is playable");
  } else break;
}
assert.ok(hinted > 10, "several hints were taken");
console.log(`${hinted} hints, each legal and playable (slowest ${worst.toFixed(0)} ms)`);

// 5. The review: null before a deal is scored, then two bullets of keys with
//    parameters, and two for the sitting at its end.
create({ seed: 91, touren: 3, human: 1 });
w.strohmandeln_review();
assert.equal(out().review, null);
let reviewed = 0;
for (let n = 0; n < 4000; n++) {
  w.strohmandeln_render();
  const s = out().state;
  if (s.prompt === "over") break;
  if (s.prompt === "opponent") assert.equal(w.strohmandeln_step(), 1);
  else {
    const a = s.legal[0];
    assert.ok(send(a.action === "extend" ? { action: "extend", accept: false } : wire(a)));
  }
  if (out().events?.some((e) => e.type === "game_scored")) {
    w.strohmandeln_review();
    const r = out().review;
    assert.equal(r.bullets.length, 2, JSON.stringify(r));
    assert.equal(r.bullets[0].lead, "count");
    for (const b of r.bullets) assert.match(b.key, /^[a-z]+(-[a-z]+)*$/);
    reviewed++;
  }
}
w.strohmandeln_review_sitting();
assert.equal(out().review.bullets.length, 2);
assert.equal(out().review.bullets[0].key, "sitting-tally");
assert.ok(reviewed >= 3, "every scored deal had a review");
console.log(`${reviewed} deal reviews and a sitting review`);

// 6. The tutor: the record of each scored deal, its evidence, the learner
//    over the summaries, the brief of the deal and of the sitting, the
//    nudge. Stateless calls take text; none moves the game.
create({ seed: 91, touren: 3, human: 1 });
assert.equal(w.strohmandeln_set_focus(write("nonsense")), 0);
assert.equal(w.strohmandeln_set_focus(write("worth")), 1);
w.strohmandeln_deal_record();
assert.equal(out().record, null);
const records = [];
const summaries = [];
const details = [];
let nudged = 0;
for (let n = 0; n < 4000; n++) {
  w.strohmandeln_render();
  const s = out().state;
  if (s.prompt === "over") break;
  if (s.prompt === "opponent") assert.equal(w.strohmandeln_step(), 1);
  else {
    if (s.phase === "play" && n % 3 === 0) {
      w.strohmandeln_nudge();
      const r = out();
      assert.equal(r.deal_action, s.deal_actions);
      if (r.nudge) {
        nudged++;
        assert.deepEqual(r.nudge, { key: "tutor.nudge.worth", skill: "worth" });
      }
      w.strohmandeln_render();
      assert.deepEqual(out().state, s, "a nudge moves nothing");
    }
    const a = s.legal[0];
    assert.ok(send(a.action === "extend" ? { action: "extend", accept: false } : wire(a)));
  }
  if (out().events?.some((e) => e.type === "game_scored")) {
    w.strohmandeln_deal_record();
    const record = out().record;
    assert.equal(typeof record, "string");
    records.push(record);
    w.strohmandeln_evidence_of(write(`${record}\n==\n`));
    const e = out();
    assert.match(e.summary, /^strohmandeln evidence v\d+\nworth \d+ \d+ [\d.]+\n/);
    summaries.push(e.summary);
    details.push(JSON.stringify(e.detail));
    w.strohmandeln_learner(write(summaries.join("\n--\n")));
    const learnt = out();
    assert.equal(learnt.deals, summaries.length);
    assert.deepEqual(learnt.stale, []);
    const history = summaries.slice(0, -1).join("\n--\n");
    w.strohmandeln_brief_of_record(write(`level 3\n${record}\n==\n--\n${history}`));
    const b = out().brief;
    assert.ok(b.bullets.length >= 1 && b.bullets.length <= 3, JSON.stringify(b));
    for (const x of b.bullets) assert.match(x.key, /^tutor\.[a-z.1-9-]+$/);
    assert.equal(b.summary, e.summary, "one advisor pass serves both");
    assert.equal(b.method, "tutor.method");
  }
}
assert.ok(records.length >= 3, "deals were scored");
console.log(`${records.length} deals through the tutor's calls, ${nudged} nudges`);
const sittingDeals = details.map((d, i) => `deal ${i + 1}\n${d}`).join("\n++\n");
w.strohmandeln_brief_sitting(write(`level 3\n${sittingDeals}\n--\n`));
assert.ok(out().brief.bullets.length <= 3);
w.strohmandeln_brief_sitting(write(`level 3\n${details[0]}\n--\n`));
assert.equal(out().error, "bad-input", "a sitting's deal has its number");
w.strohmandeln_brief_of_record(write("level 3\nnot a record\n==\n"));
assert.equal(out().error, "bad-input");

// 7. No input traps the module. A panic in the module is an `unreachable`
//    that kills every later call, the live game's included, so each of these
//    runs on an instance of its own and must answer a JSON object.
{
  const bytes = readFileSync(path);
  const stored = JSON.parse(records[0]);
  const changed = (f) => {
    const v = JSON.parse(records[0]);
    f(v);
    return JSON.stringify(v);
  };
  const doubled = changed((v) => (v.deal.actions = [...v.deal.actions, ...v.deal.actions]));
  const trailing = (action, seat) => changed((v) => v.deal.actions.push({ seat, action: { action } }));
  const recordsOdd = [
    "", "\n\n\n", "nonsense", records[0],
    changed((v) => (v.deal.deck = [])),
    changed((v) => (v.deal.deck = ["T1", "T1"])),
    changed((v) => (v.deal.deck = v.deal.deck.slice(0, 40))),
    changed((v) => (v.deal.deck[1] = v.deal.deck[0])),
    changed((v) => (v.deal.actions = [])),
    changed((v) => (v.human = 1 - stored.human)),
    changed((v) => (v.human = 5)),
    changed((v) => (v.deal.settings = {})),
    changed((v) => (v.deal.dealer = 7)),
    changed((v) => (v.deal.multiplier = 0)),
    changed((v) => (v.deal.multiplier = 4294967295)),
    changed((v) => (v.deal.actions[0].seat = 5)),
    doubled,
    trailing("take", 0), trailing("take", 1), trailing("pass", 0), trailing("pass", 1),
  ];
  const tails = ["", "1 2 3", "-1 x", Array(50000).fill("99999999999999999999").join(" "), Array(200000).fill("1").join(" ")];
  const oddText = ["", "\n", "==", "--", "++", "level", "level 3", "level 3\n==", "level 3\n++\n++", "level 3\n--\n--", "deal 1\n{}", "level 1e999\n", "level NaN\n"];
  const calls = ["evidence_of", "learner", "brief_of_record", "brief_sitting"];
  let tried = 0;
  const attempt = async (call, text) => {
    const { exports: x, out: o, write: wr } = await loadWasm(bytes, "strohmandeln");
    x[`strohmandeln_${call}`](wr(text));
    assert.equal(typeof o(), "object", `${call}: ${text.slice(0, 60)}`);
    // the module still answers afterwards
    x.strohmandeln_new(wr(JSON.stringify({ seed: 1, touren: 3 })));
    assert.equal(o().ok, true);
    tried++;
  };
  for (const r of recordsOdd) {
    for (const call of calls) {
      const body = call === "learner" || call === "brief_sitting" ? r : `${r}\n==\n${tails[tried % tails.length]}`;
      await attempt(call, call.startsWith("brief") ? `level 3\n${body}` : body);
    }
  }
  for (const t of oddText) for (const call of calls) await attempt(call, t);
  // a detail comes from storage too: numbers at the top of their range, damaged misses,
  // odd deal numbers, an old summary
  const max = 4294967295;
  const extreme = JSON.parse(details[0]);
  const slugs = ["worth", "cheap", "off", "run", "pagat", "void", "behind", "endgame", "bid", "announce"];
  extreme.summary = `${extreme.summary.split("\n")[0]}\n${slugs.map((k) => `${k} ${max} ${max} 1e308`).join("\n")}`;
  extreme.decisions = max;
  extreme.misses = { worth: { action: 1e30, trick: max, played: "Kh", better: "4h", cost: 1e308, direction: "greedy", led: "Jh", trick_thirds: max, tarocks: max, sure_thirds: max, consequence: { by: "Kh", trick: max } } };
  const old = { ...JSON.parse(details[0]) };
  old.summary = old.summary.replace(/evidence v\d+/, "evidence v1");
  const damaged = { ...JSON.parse(details[0]), misses: { off: { cost: 1 } } };
  const sittings = [
    [1, 2, 3].map((n) => `deal ${n}\n${JSON.stringify(extreme)}`).join("\n++\n"),
    ...[0, 10000, max, 4294967296, -1].map((n) => `deal ${n}\n${JSON.stringify(extreme)}`),
    `deal 1\n${JSON.stringify(old)}`,
    `deal 1\n${JSON.stringify(damaged)}`,
  ];
  for (const body of sittings) await attempt("brief_sitting", `level 3\n${body}\n--\n`);
  const x = await loadWasm(bytes, "strohmandeln");
  for (const slug of ["", " ", "worth ", "WORTH", "w".repeat(100000)]) x.exports.strohmandeln_set_focus(x.write(slug));
  x.exports.strohmandeln_nudge();
  assert.equal(x.out().error, "no-sitting");
  console.log(`${tried} odd inputs answered, none trapped`);
}
