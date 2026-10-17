// Seeds for the browser checks whose preconditions come from the engine's play, found without a browser:
// the same sitting the page plays at ?level=1 with the engine's first legal action (or "weiter"), in Node.
//
//   node web3d/test/seed_scan.mjs FIRST LAST [first|weiter]
//
// Needs the wasm (python3 kit/build.py builds it). Rerun after any change to the opponent or the deal: the
// browser seeds (last_trick_ultimo, draw_and_doubling, slate_rows) depend on the opponent at level 1.
// Prints, per seed, the pagat_ultimo results, whether a draw was scored, and whether a doubling was applied.
// web3d/test/seed_probe.py checks the same in the page, and the screen-reader and deal-brief preconditions.
import { playSitting } from "./sim.js";
import { DEFAULTS, sittingConfig } from "../src/prefs.js";

const [first, last, policy = "first"] = [Number(process.argv[2]), Number(process.argv[3]), process.argv[4]];
const choose = (s) => (policy === "weiter" ? s.legal.find((a) => a.action === "pass") : undefined) ?? s.legal[0];

for (let seed = first; seed <= last; seed++) {
  const config = { ...sittingConfig(DEFAULTS, { seed, human: 1 }), level: 1 };
  const events = [];
  await playSitting(config, { policy: choose, onReply: (r) => events.push(...(r.events ?? [])) });
  const ultimo = [...new Set(events.filter((e) => e.type === "pagat_ultimo").map((e) => e.result))].sort();
  const draw = events.some((e) => e.type === "game_scored" && e.result === "draw");
  const doubled = events.some((e) => e.type === "doubling_applied");
  console.log(seed, "ultimo", JSON.stringify(ultimo), "draw", draw, "doubled", doubled);
}
