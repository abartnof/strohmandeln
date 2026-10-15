// The tutor, a period Kibitzer (docs/TABLE3D.md 6): which tutorial page falls
// due and when, the mute, and the pages' bookkeeping. It holds no rules of
// play: it reads the events and the module's state (docs/PROTOCOL.md) and the
// words it says are keys of the strings and of the phrase bank. The view
// (the tab, the dialogs) is the page's; this is what is decided.
//
// Cassino's tutorial.js (pageDue, the seen list) is the pattern.

import { parseTutorial } from "../../kit/js/tutorial.js";

// The page keys, in the order they are due when more than one could be
// (the table of TABLE3D 6.2.2; web3d/test/tutorial.test.js holds them to it).
export const PAGE_KEYS = ["packets", "bidding", "the-pull", "following", "from-a-packet", "honours", "pagat-ultimo", "thirds", "draw", "touren-slate", "extension"];

// The pages of a tutorial file for the sitting's rule settings. A block of
// lines between `<!-- if name=value -->` and `<!-- end -->` is kept when the
// setting `name` is `value`, and dropped, markers and all, when it is not
// (the bidding page says what happens when both pass, by `both_pass`).
export function tutorialPages(markdown, keys, settings = {}) {
  let keep = true;
  const lines = [];
  for (const line of markdown.split("\n")) {
    const open = /^<!-- if (\w+)=(\S+) -->$/.exec(line.trim());
    if (open) keep = String(settings[open[1]]) === open[2];
    else if (line.trim() === "<!-- end -->") keep = true;
    else if (keep) lines.push(line);
  }
  return parseTutorial(lines.join("\n"), keys);
}

const has = (events, type, ok = () => true) => events.some((e) => e.type === type && ok(e));

// The last tricks of the deal, in which the Pagat may be saved for the last (6.2.2).
const LAST_TRICKS_FROM = 23;

// The pages whose moment is now, in order, from the events not yet reckoned
// with and the module's state. `human` is the human's seat; `slate` is
// whether the slate was opened by hand since.
export function pagesDue({ events = [], state = null, human = 0, slate = false }) {
  const due = new Set();
  const mine = state?.prompt === "you";
  if (has(events, "dealt")) due.add("packets");
  if (has(events, "bid") || (mine && state.phase === "bid")) due.add("bidding");
  if (has(events, "reveal_start")) due.add("the-pull");
  if (mine && state.phase === "play" && state.deal?.lead) due.add("following");
  if (has(events, "card_played", (e) => e.seat === human && String(e.source).startsWith("packet"))) due.add("from-a-packet");
  if (has(events, "announcement") || (mine && (state.legal ?? []).some((a) => a.action === "announce"))) due.add("honours");
  if (has(events, "pagat_ultimo") || (state?.deal?.trick_number >= LAST_TRICKS_FROM && (state.deal.own_hand ?? []).some((c) => c.card === "T1"))) due.add("pagat-ultimo");
  if (has(events, "count")) due.add("thirds");
  if (has(events, "game_scored", (e) => e.result === "draw")) due.add("draw");
  if (has(events, "tour_end") || slate) due.add("touren-slate");
  if (has(events, "extension_offered")) due.add("extension");
  return PAGE_KEYS.filter((k) => due.has(k));
}

// The tutor's bookkeeping. `prefs()` is the current preferences, `patch(p)`
// keeps a change of them, `open(key)` shows a page and `speak(group)` says a
// line of the phrase bank's kibitzer groups ("kibitz", "kibitz-mute").
export function createTutor({ prefs, patch, open = () => {}, speak = () => {} }) {
  let owed = new Set(); // pages whose moment came and which are not yet shown
  let shown = new Set(); // opened since the last deal began (a reminder waits for the next)
  let introduced = false;
  let current = null; // the page open now
  let slate = false;
  let seen = [];

  const on = () => Boolean(prefs().tutor) && !prefs().muted;

  const api = {
    // The tutor is on and not muted: pages, tips, the review chip (6.1).
    active: on,
    muted: () => Boolean(prefs().muted),
    current: () => current,

    // The sitting begins again: nothing is owed.
    reset() {
      owed = new Set();
      shown = new Set();
      current = null;
      slate = false;
      introduced = false;
    },

    // The slate was opened by hand (the touren-slate page's other moment).
    slateOpened() {
      slate = true;
    },

    // At rest (the cards still): the page due, opened if there is one. `events`
    // are those since the last call; `blocked` says a dialog is already open
    // (the events are then kept for the next rest). Returns the key opened.
    rest(state, events, { human = 0, blocked = false } = {}) {
      if (events.some((e) => e.type === "deal_start")) shown = new Set();
      if (!on()) {
        owed = new Set();
        slate = false;
        return null;
      }
      seen = prefs().seen ?? [];
      for (const k of pagesDue({ events, state, human, slate })) owed.add(k);
      slate = false;
      if (blocked || current) return null;
      const key = PAGE_KEYS.find((k) => owed.has(k) && !seen.includes(k) && !shown.has(k));
      if (!key) return null;
      owed.delete(key);
      shown.add(key);
      current = key;
      if (!introduced) {
        introduced = true;
        speak("kibitz");
      }
      open(key, { help: false });
      return key;
    },

    // A page is closed. "Not now" marks it seen; "Remind me" leaves it for the
    // next time its moment comes (6.2.2).
    close(key, remind = false) {
      if (current === key) current = null;
      if (!remind && !(prefs().seen ?? []).includes(key)) patch({ seen: [...(prefs().seen ?? []), key] });
    },

    // A page from the Help list (6.2.2): always allowed, marks it seen.
    help(key) {
      if (!PAGE_KEYS.includes(key)) return false;
      current = key;
      if (!(prefs().seen ?? []).includes(key)) patch({ seen: [...(prefs().seen ?? []), key] });
      open(key, { help: true });
      return true;
    },

    // The Help list closed.
    helpClosed(key) {
      if (current === key) current = null;
    },

    // "§ 13, Kiebitz, halt's Maul!": mute for the sitting, the one line (6.1).
    mute() {
      if (prefs().muted) return;
      patch({ muted: true });
      speak("kibitz-mute");
    },
    unmute() {
      if (prefs().muted) patch({ muted: false });
    },
    toggleMute() {
      if (prefs().muted) api.unmute();
      else api.mute();
    },
    // The first time the tab is opened in a sitting: the intro line (6.1).
    intro() {
      if (introduced || !on()) return;
      introduced = true;
      speak("kibitz");
    },
  };
  return api;
}
