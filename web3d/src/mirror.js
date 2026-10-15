// The accessible mirror of the cards (docs/TABLE3D.md 3.10.1): the hand and
// the human's covers as two labelled lists of buttons the keyboard and a
// screen reader can reach, over the canvas that cannot be. Each list is one
// Tab stop (the card in focus, else the first lit card, else none: focus
// never lands on a dimmed card by default, 3.10.3); arrows, Home and End
// move within it. It draws what it is given and reports what is pressed; it
// holds no rule.
import { h } from "./dom.js";

// state: { hand: [{ code, lit }], packets: [{ packet, code, hidden, lit }], focus: { row, index } | null }
export function createMirror(doc, { t, tn, cardName, onFocus, onMove, onPick, onPacket }) {
  const lists = {
    hand: h(doc, "ul", { class: "mirror-list", "data-row": "hand", role: "list" }),
    packets: h(doc, "ul", { class: "mirror-list", "data-row": "packets", role: "list" }),
  };
  const buttons = { hand: [], packets: [] };
  const node = h(doc, "div", { class: "mirror" }, lists.hand, lists.packets);
  let inside = false; // the keyboard's focus is within the mirror
  let signature = "";

  const labelOf = (row, item) =>
    row === "hand" ? t("sr.hand.item", { card: cardName(item.code) }) : tn("sr.packet.item", item.hidden, { card: cardName(item.code), packet: item.packet });

  for (const [row, ul] of Object.entries(lists)) {
    ul.addEventListener("focusin", (e) => {
      inside = true;
      const i = e.target?.getAttribute?.("data-i");
      if (i !== null && i !== undefined) onFocus?.(row, Number(i));
    });
    ul.addEventListener("focusout", (e) => {
      if (!e.relatedTarget || !buttons[row].includes(e.relatedTarget)) inside = false;
    });
    ul.addEventListener("keydown", (e) => {
      const i = e.target?.getAttribute?.("data-i");
      const at = i === null || i === undefined ? 0 : Number(i);
      const key = e.key;
      let used = true;
      if (key === "ArrowLeft" || key === "ArrowUp") onMove?.(row, -1);
      else if (key === "ArrowRight" || key === "ArrowDown") onMove?.(row, 1);
      else if (key === "Home") onMove?.(row, 0, 0);
      else if (key === "End") onMove?.(row, 0, 999);
      else if (key === "Enter" || key === " ") onPick?.(row, at);
      else if (row === "packets" && ["1", "2", "3"].includes(key)) onPacket?.(Number(key));
      else used = false;
      if (used) e.preventDefault?.();
    });
    // A space presses a button on its key-up: the key-down has done it already.
    ul.addEventListener("keyup", (e) => {
      if (e.key === " ") e.preventDefault?.();
    });
  }

  // The Tab stop of each list: the card in focus, else the first lit one.
  function setStops(rows, focus) {
    for (const row of Object.keys(lists)) {
      const mine = focus && focus.row === row ? focus.index : -1;
      const stop = mine >= 0 ? mine : rows[row].findIndex((x) => x.lit);
      buttons[row].forEach((b, i) => b.setAttribute("tabindex", i === stop ? "0" : "-1"));
    }
  }

  function set({ hand = [], packets = [], focus = null }) {
    const rows = { hand, packets };
    const sig = JSON.stringify(rows);
    if (sig === signature) return setStops(rows, focus); // only the focus moved: no redraw, so the keyboard keeps its place
    signature = sig;
    let target = null;
    for (const [row, ul] of Object.entries(lists)) {
      const items = rows[row];
      ul.setAttribute("aria-label", t(row === "hand" ? "sr.hand.list" : "sr.packets.list"));
      ul.hidden = items.length === 0;
      buttons[row] = items.map((item, i) => {
        const b = h(doc, "button", { type: "button", "data-i": String(i), "data-code": item.code, "aria-disabled": item.lit ? null : "true", "aria-pressed": item.chosen ? "true" : null }, labelOf(row, item));
        b.addEventListener("click", () => onPick?.(row, i));
        if (inside && focus && focus.row === row && focus.index === i) target = b;
        return b;
      });
      ul.replaceChildren(...buttons[row].map((b) => h(doc, "li", {}, b)));
    }
    setStops(rows, focus);
    target?.focus();
  }

  // Put the keyboard's focus on an item (an arrow pressed before any Tab).
  const focusItem = (row, index) => buttons[row][index]?.focus();

  // The language changed: the labels are drawn again on the next set().
  const relabel = () => (signature = "");

  return { node, set, focusItem, relabel, hasFocus: () => inside };
}
