// Escape (docs/TABLE3D.md 3.10.5): which layer one press closes. Pure; main.js
// reads the page's state, asks, and does the closing.
//
// Dialogs that need an answer are never dismissed by Escape and hold back
// whatever lies beneath them: the extension prompt (One more Tour / Enough),
// the sitting's end, and the new-sitting screen when there is no sitting to go
// back to. Escape then only moves focus to the dialog's default button. The
// welcome acts as its default (Continue): nothing is lost.

const NEEDS_ANSWER = new Set(["extend", "over"]);

// state: { dialogs: ids, bottom to top; why (the hint's open "Why?"), settings, slate, hud, tracker, chosen: booleans;
// canCancelNew: the new-sitting screen has a Cancel }.
// Returns { kind: "dialog", id } | { kind: "focus", id } (a dialog that needs an answer)
// | { kind: "why" | "settings" | "slate" | "hud" | "tracker" | "selection" } | null.
export function escapeTarget(state) {
  const top = state.dialogs[state.dialogs.length - 1];
  if (top !== undefined) {
    if (NEEDS_ANSWER.has(top) || (top === "new" && !state.canCancelNew)) return { kind: "focus", id: top };
    return { kind: "dialog", id: top };
  }
  for (const kind of ["why", "settings", "slate", "hud", "tracker"]) if (state[kind]) return { kind };
  return state.chosen ? { kind: "selection" } : null;
}

// A native <select> that is open takes the Escape that closes it; the page's
// handler leaves that press alone. observe() sees mousedown, keydown, change and
// focusout; swallows() answers for an Escape keydown and consumes the open state.
export function selectGuard() {
  let open = false;
  const isSelect = (e) => e.target?.tagName === "SELECT";
  return {
    observe(e) {
      if (e.type === "mousedown") open = isSelect(e) ? !open : false;
      else if (e.type === "keydown" && isSelect(e) && ((e.altKey && (e.key === "ArrowDown" || e.key === "ArrowUp")) || e.key === " " || e.key === "F4")) open = true;
      else if (e.type === "change" || e.type === "focusout") open = false;
    },
    swallows(e) {
      if (!(open && isSelect(e))) return false;
      open = false;
      return true;
    },
  };
}
