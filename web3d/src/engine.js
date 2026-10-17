// The engine as the page calls it (docs/PROTOCOL.md): the kit's wasm loader,
// and one function a call. A fault in the module (a panic) is told once to
// `onTrap` and the engine is still after it (kit/js/engine.js `guard`).
import { guard, loadWasm } from "../../kit/js/engine.js";
import { tutorApi } from "./tutorwire.js";

export async function loadEngine(bytes, onTrap) {
  const { exports: ex, out, write } = await loadWasm(bytes, "strohmandeln");
  const call = (name, text) => {
    if (text === undefined) ex[`strohmandeln_${name}`]();
    else ex[`strohmandeln_${name}`](write(text));
    return out();
  };
  // A play is sent as the card alone, with its announcements (PROTOCOL.md).
  const callNumber = (name, text) => ex[`strohmandeln_${name}`](write(text));
  const wire = (a) => (a.action === "play" ? { action: "play", card: a.card, ...(a.announce?.length ? { announce: a.announce } : {}) } : a);
  return guard(
    {
      new: (config) => call("new", JSON.stringify(config)),
      send: (action) => call("send", JSON.stringify(wire(action))),
      step: () => call("step"),
      render: () => call("render"),
      legal: () => call("legal"),
      export: () => call("export"),
      import: (record) => call("import", JSON.stringify(record)),
      hint: () => call("hint"),
      review: () => call("review"),
      reviewSitting: () => call("review_sitting"),
      explain: () => call("explain"),
      tracker: () => call("tracker"),
      audit: () => call("audit"),
      // The ids the new-sitting screen offers besides Normal and Random (docs/PROTOCOL.md).
      styles: () => call("styles")?.styles ?? [],
      // The tutor (docs/PROTOCOL.md "The tutor"); stateless but for the focus.
      ...tutorApi(call, callNumber),
    },
    onTrap,
  );
}
