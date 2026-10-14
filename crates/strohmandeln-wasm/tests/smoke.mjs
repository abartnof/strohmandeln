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
    for (const code of audit()) {
      assert.ok(!seen.includes(`"${code}"`), `hidden card ${code} reached the human`);
      looked++;
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
w.strohmandeln_hint();
assert.equal(out().hint, null);
console.log("export/import restores the state; the hint is a stub");
