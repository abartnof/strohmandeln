// Times the tutor's hint through the wasm build, on the positions that
// examples/hintbench.rs times natively (the same sittings, the human taking
// each hint, the opponent at level 3): one line per hint, "sitting move kind
// ms", kind being lead, follow or other (a bid). Compare the two by line.
//
//   cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
//   node crates/strohmandeln-wasm/tests/hintbench.mjs [wasm file] [sittings]
//   cargo run --release -p strohmandeln-wasm --example hintbench -- [sittings]
import { readFileSync } from "node:fs";
import { loadWasm } from "../../../kit/js/engine.js";

const file = process.argv[2] ?? new URL("../../../target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm", import.meta.url);
const sittings = Number(process.argv[3] ?? 4);
const { exports: w, out, write } = await loadWasm(readFileSync(file), "strohmandeln");
const wire = (a) => (a.action === "play" ? { action: "play", card: a.card } : a);
const send = (a) => w.strohmandeln_send(write(JSON.stringify(a))) === 1;

for (let seed = 0; seed < sittings; seed++) {
  const config = { seed, touren: 3, human: seed % 2, level: 3 };
  if (w.strohmandeln_new(write(JSON.stringify(config))) !== 1) throw new Error(JSON.stringify(out()));
  let r = out();
  for (let move = 0; r.state.prompt !== "over"; move++) {
    if (r.state.prompt === "opponent") {
      w.strohmandeln_step();
      r = out();
    } else if (r.state.phase === "extend") {
      send({ action: "extend", accept: false });
      r = out();
    } else {
      const t = performance.now();
      w.strohmandeln_hint();
      const ms = performance.now() - t;
      const h = out();
      const kind = h.hint?.action !== "play" ? "other" : h.reason.startsWith("lead-") ? "lead" : "follow";
      if (r.state.legal.length > 1) console.log(`${seed} ${move} ${kind} ${ms.toFixed(3)}`);
      if (!send(wire(h.hint))) throw new Error(JSON.stringify(out()));
      r = out();
    }
  }
}
