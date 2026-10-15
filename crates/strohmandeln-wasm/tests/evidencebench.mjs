// Times the tutor's evidence of a deal through the wasm build (M7,
// docs/MEASUREMENTS.md): the human takes each hint against the opponent at
// level 3 (the browser's default), and after each scored deal the record the
// table would store goes to `strohmandeln_evidence_of`. One line per deal,
// "sitting deal ms", and the records are written to a file, one a line, for
// examples/evidencebench.rs to time natively on the same deals.
//
//   cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
//   node crates/strohmandeln-wasm/tests/evidencebench.mjs [records file] [sittings] [first seed] [wasm file]
//   cargo run --release -p strohmandeln-wasm --example evidencebench -- [records file]
import { readFileSync, writeFileSync } from "node:fs";
import { loadWasm } from "../../../kit/js/engine.js";

const records = process.argv[2] ?? "evidence-records.txt";
const sittings = Number(process.argv[3] ?? 4);
const first = Number(process.argv[4] ?? 0);
const file = process.argv[5] ?? new URL("../../../target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm", import.meta.url);
const { exports: w, out, write } = await loadWasm(readFileSync(file), "strohmandeln");
const wire = (a) => (a.action === "play" ? { action: "play", card: a.card } : a);
const send = (a) => w.strohmandeln_send(write(JSON.stringify(a))) === 1;

const lines = [];
const ms = [];
for (let seed = first; seed < first + sittings; seed++) {
  const config = { seed, touren: 3, human: seed % 2, level: 3 };
  if (w.strohmandeln_new(write(JSON.stringify(config))) !== 1) throw new Error(JSON.stringify(out()));
  let r = out();
  let last = null;
  let deal = 0;
  while (r.state.prompt !== "over") {
    if (r.state.prompt === "opponent") {
      w.strohmandeln_step();
    } else if (r.state.phase === "extend") {
      send({ action: "extend", accept: false });
    } else {
      w.strohmandeln_hint();
      if (!send(wire(out().hint))) throw new Error(JSON.stringify(out()));
    }
    r = out();
    // a deal just scored: its record is new
    w.strohmandeln_deal_record();
    const rec = out().record;
    if (rec && rec !== last) {
      last = rec;
      const t = performance.now();
      w.strohmandeln_evidence_of(write(`${rec}\n==\n`));
      const dt = performance.now() - t;
      const e = out();
      if (!e.summary) throw new Error(JSON.stringify(e));
      lines.push(rec);
      ms.push(dt);
      console.log(`${seed} ${deal++} ${dt.toFixed(3)}`);
    }
  }
}
writeFileSync(records, lines.join("\n") + "\n");
ms.sort((a, b) => a - b);
const q = (p) => ms[Math.round((ms.length - 1) * p)];
console.log(`deals ${ms.length} median ${q(0.5).toFixed(1)} ms p95 ${q(0.95).toFixed(1)} ms max ${ms[ms.length - 1].toFixed(1)} ms`);
