// Who speaks when (docs/TABLE3D.md 5.2.2): the opponent has one voice, so
// lines wait their turn in the order of their moments, and chatter yields to
// the calls by the kit's rules (dialogue.js): a chatter line is left out if
// it would come more than LATE after its own moment, and is cut or dropped
// when a call arrives that it would make more than SLACK late. A chatty
// table plays at the pace of a quiet one.
//
// The talk decides what is said and at which moment (talk.js); this decides
// when it is heard. A call that comes later than the chatter ahead of it
// cannot be known beforehand, so chatter yields when the call arrives: a
// chatter line not yet begun is dropped, one being said is cut if the call
// would be more than SLACK late, and one nearly done is let finish.
import { GAP, LATE, saying, SLACK } from "../../kit/js/dialogue.js";

// clock() -> ms; later(ms, fn, kind) runs fn after ms on the table's clock
// (kind "talk": a line still to be said, which a skip drops; "linger": a box
// coming down, which a skip leaves);
// show(line) puts the line's box up; hide(line) takes it down.
export function createSpeaker({ clock, later, show, hide }) {
  let slots = []; // { line, start, end, state: "queued" | "showing" | "done" }
  let epoch = 0;

  // A queued line whose time has passed without being shown was dropped by a skip.
  const live = () => (slots = slots.filter((s) => (s.state === "queued" && s.start >= clock()) || s.state === "showing"));

  function cut(slot, now) {
    if (slot.state === "showing") hide(slot.line);
    slot.state = "done";
    slot.end = Math.min(slot.end, now);
  }

  // Offer a line at its moment: { text, call }. Returns whether it will be said.
  function offer(line) {
    const now = clock();
    live();
    if (line.call) {
      for (const slot of slots) {
        if (slot.line.call) continue;
        if (slot.state === "queued" || slot.end + GAP > now + SLACK) cut(slot, now);
      }
      live();
    }
    const tail = slots.length ? Math.max(...slots.map((s) => s.end)) + GAP : now;
    const start = Math.max(now, tail);
    if (!line.call && start - now > LATE) return false;
    const slot = { line, start, end: start + saying(line.text), state: "queued" };
    slots.push(slot);
    const mine = epoch;
    later(start - now, () => {
      if (mine !== epoch || slot.state !== "queued") return;
      slot.state = "showing";
      show(line);
      later(slot.end - slot.start, () => {
        if (mine !== epoch || slot.state !== "showing") return;
        slot.state = "done";
        hide(line);
      }, "linger");
    }, "talk");
    return true;
  }

  // A new sitting, or a skip that dropped what was still to come.
  function stop() {
    epoch++;
    slots = [];
  }

  return { offer, stop };
}
