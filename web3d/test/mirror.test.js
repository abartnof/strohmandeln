import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createMirror } from "../src/mirror.js";
import { createStrings } from "../src/strings.js";
import { fakeDocument } from "./support/fakedom.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));

function build(lang = "en") {
  const s = createStrings(TABLE, lang);
  const calls = [];
  const doc = fakeDocument();
  const m = createMirror(doc, {
    ...s,
    onFocus: (...a) => calls.push(["focus", ...a]),
    onMove: (...a) => calls.push(["move", ...a]),
    onPick: (...a) => calls.push(["pick", ...a]),
    onPacket: (...a) => calls.push(["packet", ...a]),
  });
  return { m, calls, s };
}

const HAND = [
  { code: "7c", lit: false },
  { code: "Qs", lit: true },
  { code: "T5", lit: true },
];
const PACKETS = [
  { packet: 2, code: "Kh", hidden: 3, lit: true },
  { packet: 3, code: "T1", hidden: 1, lit: false },
];
const buttonsOf = (m, row) => m.node.byAttr("data-row", row)[0].findAll((e) => e.tagName === "BUTTON");

test("the hand and the covers are two labelled lists of labelled items, in both languages", () => {
  for (const lang of ["en", "de"]) {
    const { m, s } = build(lang);
    m.set({ hand: HAND, packets: PACKETS });
    const lists = m.node.findAll((e) => e.tagName === "UL");
    assert.equal(lists.length, 2);
    assert.equal(lists[0].getAttribute("role"), "list");
    assert.equal(lists[0].getAttribute("aria-label"), s.t("sr.hand.list"));
    assert.equal(lists[1].getAttribute("aria-label"), s.t("sr.packets.list"));
    assert.notEqual(s.t("sr.hand.list"), "sr.hand.list");
    const hand = buttonsOf(m, "hand");
    assert.equal(hand.length, 3);
    assert.equal(hand[0].textContent, s.t("sr.hand.item", { card: s.cardName("7c") }));
    const packets = buttonsOf(m, "packets");
    assert.equal(packets[0].textContent, s.tn("sr.packet.item", 3, { card: s.cardName("Kh"), packet: 2 }));
    assert.equal(packets[1].textContent, s.tn("sr.packet.item", 1, { card: s.cardName("T1"), packet: 3 }));
  }
});

test("each list is one Tab stop, on the first lit card; a dimmed card is marked and not a stop", () => {
  const { m } = build();
  m.set({ hand: HAND, packets: PACKETS });
  const hand = buttonsOf(m, "hand");
  assert.deepEqual(hand.map((b) => b.getAttribute("tabindex")), ["-1", "0", "-1"]);
  assert.deepEqual(hand.map((b) => b.getAttribute("aria-disabled")), ["true", null, null]);
  m.set({ hand: HAND.map((c, i) => ({ ...c, chosen: i === 1 })), packets: PACKETS });
  assert.deepEqual(buttonsOf(m, "hand").map((b) => b.getAttribute("aria-pressed")), [null, "true", null]);
  assert.deepEqual(buttonsOf(m, "packets").map((b) => b.getAttribute("tabindex")), ["0", "-1"]);
});

test("with nothing lit no card is a Tab stop; an empty list is hidden", () => {
  const { m } = build();
  m.set({ hand: HAND.map((c) => ({ ...c, lit: false })), packets: [] });
  assert.ok(buttonsOf(m, "hand").every((b) => b.getAttribute("tabindex") === "-1"));
  assert.equal(m.node.byAttr("data-row", "packets")[0].hidden, true);
});

test("the card in focus is the Tab stop, lit or not", () => {
  const { m } = build();
  m.set({ hand: HAND, packets: PACKETS, focus: { row: "hand", index: 0 } });
  assert.deepEqual(buttonsOf(m, "hand").map((b) => b.getAttribute("tabindex")), ["0", "-1", "-1"]);
});

test("keys: arrows, Home and End move; Enter, Space and a click pick; 1 2 3 choose a packet", () => {
  const { m, calls } = build();
  m.set({ hand: HAND, packets: PACKETS });
  const hand = m.node.byAttr("data-row", "hand")[0];
  const stop = buttonsOf(m, "hand")[1];
  const press = (ul, key, target) => ul.dispatch("keydown", { key, target });
  press(hand, "ArrowRight", stop);
  press(hand, "ArrowLeft", stop);
  press(hand, "Home", stop);
  press(hand, "End", stop);
  const ev = press(hand, "Enter", stop);
  assert.ok(ev.defaultPrevented, "the key is taken, so the page does not act on it twice");
  press(hand, " ", stop);
  stop.click();
  const packets = m.node.byAttr("data-row", "packets")[0];
  press(packets, "2", buttonsOf(m, "packets")[0]);
  press(hand, "2", stop); // a digit means a packet only in the packets list
  assert.deepEqual(calls, [
    ["move", "hand", 1],
    ["move", "hand", -1],
    ["move", "hand", 0, 0],
    ["move", "hand", 0, 999],
    ["pick", "hand", 1],
    ["pick", "hand", 1],
    ["pick", "hand", 1],
    ["packet", 2],
  ]);
});

test("focus entering an item is reported with its row and place", () => {
  const { m, calls } = build();
  m.set({ hand: HAND, packets: PACKETS });
  const hand = m.node.byAttr("data-row", "hand")[0];
  hand.dispatch("focusin", { target: buttonsOf(m, "hand")[2] });
  assert.deepEqual(calls, [["focus", "hand", 2]]);
  assert.equal(m.hasFocus(), true);
});

test("a redraw keeps the keyboard on the card it was on, and only redraws on a change", () => {
  const { m } = build();
  m.set({ hand: HAND, packets: PACKETS });
  const hand = m.node.byAttr("data-row", "hand")[0];
  hand.dispatch("focusin", { target: buttonsOf(m, "hand")[1] });
  const before = buttonsOf(m, "hand");
  m.set({ hand: HAND, packets: PACKETS }); // nothing changed
  assert.equal(buttonsOf(m, "hand")[0], before[0]);
  m.set({ hand: HAND, packets: PACKETS, focus: { row: "hand", index: 2 } }); // only the focus moved
  assert.equal(buttonsOf(m, "hand")[0], before[0], "no redraw: the keyboard keeps its place");
  assert.equal(buttonsOf(m, "hand")[2].getAttribute("tabindex"), "0");
  m.set({ hand: HAND.slice(1), packets: PACKETS, focus: { row: "hand", index: 0 } });
  const after = buttonsOf(m, "hand");
  assert.equal(after.length, 2);
  assert.equal(after[0].focused, true);
});

test("focusItem puts the focus on an item", () => {
  const { m } = build();
  m.set({ hand: HAND, packets: PACKETS });
  m.focusItem("packets", 1);
  assert.equal(buttonsOf(m, "packets")[1].focused, true);
});
