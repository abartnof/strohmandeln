// The engine in Node, for the table's tests: sittings played by a policy,
// every reply kept. The module is the one bin/gate builds
// (target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm).
import { readFileSync } from "node:fs";
import { loadWasm } from "../../kit/js/engine.js";

const WASM = new URL("../../target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm", import.meta.url);
let loaded = null;

export async function engineInNode() {
  loaded ??= await loadWasm(readFileSync(WASM), "strohmandeln");
  const { exports: w, out, write } = loaded;
  const call = (name, text) => (text === undefined ? w[`strohmandeln_${name}`]() : w[`strohmandeln_${name}`](write(text)), out());
  return {
    new: (config) => call("new", JSON.stringify(config)),
    send: (action) => call("send", JSON.stringify(action)),
    step: () => call("step"),
    render: () => call("render"),
    export: () => call("export"),
    import: (record) => call("import", JSON.stringify(record)),
    audit: () => call("audit"),
    hint: () => call("hint"),
  };
}

// The action a play is sent as: the card alone (PROTOCOL.md).
export const wire = (a) => (a.action === "play" ? { action: "play", card: a.card } : a);

// A sitting played to its end (or `limit` replies) by `policy(state, rng)`,
// which returns one of state.legal. The opponent is stepped. Returns every
// reply, the opening first: [{ reply, state, events }].
export async function playSitting(config, { policy = (s) => s.legal[0], limit = 4000, extend = false, onReply } = {}) {
  const engine = await engineInNode();
  let reply = engine.new(config);
  const replies = [reply];
  onReply?.(reply, engine);
  for (let n = 0; n < limit && reply.state.prompt !== "over"; n++) {
    if (reply.state.prompt === "opponent") reply = engine.step();
    else if (reply.state.phase === "extend") reply = engine.send({ action: "extend", accept: extend });
    else reply = engine.send(wire(policy(reply.state)));
    if (!reply.ok && !reply.stepped) throw new Error(`refused: ${JSON.stringify(reply.refusal ?? reply)}`);
    replies.push(reply);
    onReply?.(reply, engine);
  }
  return { replies, engine };
}

// A small deterministic generator, for policies.
export function rng(seed) {
  let s = seed >>> 0 || 1;
  return () => {
    s ^= s << 13;
    s >>>= 0;
    s ^= s >>> 17;
    s ^= s << 5;
    s >>>= 0;
    return s / 4294967296;
  };
}

// Policies. `random` takes any legal action, `passer` passes the bidding and
// otherwise plays at random, `taker` takes it up.
export const random = (seed) => {
  const r = rng(seed);
  return (s) => s.legal[Math.floor(r() * s.legal.length)];
};
export const passer = (seed) => {
  const r = random(seed);
  return (s) => s.legal.find((a) => a.action === "pass") ?? r(s);
};
export const taker = (seed) => {
  const r = random(seed);
  return (s) => s.legal.find((a) => a.action === "take") ?? r(s);
};
