// Escape (docs/TABLE3D.md 3.10.5): which layer one press closes. Pure; main.js
// reads the page's state, asks, and does the closing.
//
// Dialogs that need an answer are never dismissed by Escape and hold back
// whatever lies beneath them: the extension prompt (One more Tour / Enough),
// the sitting's end, the welcome (Continue / New), and the new-sitting screen
// when there is no sitting to go back to.

const NEEDS_ANSWER = new Set(["extend", "over", "welcome"]);

// state: { dialogs: ids, bottom to top; settings, slate, hud, tracker, chosen: booleans;
// canCancelNew: the new-sitting screen has a Cancel }.
// Returns { kind: "dialog", id } | { kind: "settings" | "slate" | "hud" | "tracker" | "selection" } | null.
export function escapeTarget(state) {
  const top = state.dialogs[state.dialogs.length - 1];
  if (top !== undefined) {
    if (NEEDS_ANSWER.has(top) || (top === "new" && !state.canCancelNew)) return null;
    return { kind: "dialog", id: top };
  }
  for (const kind of ["settings", "slate", "hud", "tracker"]) if (state[kind]) return { kind };
  return state.chosen ? { kind: "selection" } : null;
}
