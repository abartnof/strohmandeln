// The tutor: which page falls due, when it opens, "Not now" and "Remind me",
// the mute (docs/TABLE3D.md 6.1, 6.2).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createTutor, pagesDue, PAGE_KEYS } from "../src/tutor.js";

const spec = readFileSync(new URL("../../docs/TABLE3D.md", import.meta.url), "utf8");

const state = (over = {}) => ({ prompt: "opponent", phase: "play", legal: [], deal: { lead: null, trick_number: 1, own_hand: [] }, ...over });
const ev = (type, more = {}) => ({ type, ...more });

test("the page keys are those of the table in TABLE3D 6.2.2, in order", () => {
  const keys = [...spec.split("### 6.2 Tutorial pages")[1].split("### 6.3")[0].matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
  assert.deepEqual(PAGE_KEYS, keys);
});

test("each page is due at its own first moment", () => {
  const due = (events, s = state(), more = {}) => pagesDue({ events, state: s, human: 1, ...more });
  assert.deepEqual(due([ev("dealt", { seat: 1 })]), ["packets"]);
  assert.deepEqual(due([ev("bid", { seat: 0 })]), ["bidding"]);
  assert.deepEqual(due([], state({ prompt: "you", phase: "bid" })), ["bidding"]);
  assert.deepEqual(due([ev("reveal_start", { seat: 1 })]), ["the-pull"]);
  assert.deepEqual(due([], state({ prompt: "you", deal: { lead: "Kh", trick_number: 3, own_hand: [] } })), ["following"]);
  assert.deepEqual(due([], state({ prompt: "you", deal: { lead: null, trick_number: 3, own_hand: [] } })), [], "a lead is not a follow");
  assert.deepEqual(due([ev("card_played", { seat: 1, card: "Qd", source: "packet 2" })]), ["from-a-packet"]);
  assert.deepEqual(due([ev("card_played", { seat: 0, card: "Qd", source: "packet 2" })]), [], "the opponent's cover is not yours");
  assert.deepEqual(due([ev("card_played", { seat: 1, card: "Qd", source: "hand" })]), []);
  assert.deepEqual(due([ev("announcement", { seat: 0, honour: "kings" })]), ["honours"]);
  assert.deepEqual(due([], state({ prompt: "you", legal: [{ action: "announce", honour: "trull" }] })), ["honours"]);
  assert.deepEqual(due([ev("pagat_ultimo", { seat: 0, result: "won" })]), ["pagat-ultimo"]);
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 23, own_hand: [{ card: "T1", public: false }] } })), ["pagat-ultimo"]);
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 22, own_hand: [{ card: "T1", public: false }] } })), [], "the last five tricks only");
  assert.deepEqual(due([ev("count", { thirds: [100, 110] })]), ["thirds"]);
  assert.deepEqual(due([ev("game_scored", { result: "draw" })]), ["draw"]);
  assert.deepEqual(due([ev("game_scored", { result: "declarer_win" })]), []);
  assert.deepEqual(due([ev("tour_end", { tour: 1 })]), ["touren-slate"]);
  assert.deepEqual(due([], state(), { slate: true }), ["touren-slate"]);
  assert.deepEqual(due([ev("extension_offered")]), ["extension"]);
});

test("pages due together come in the order of the table", () => {
  const got = pagesDue({ events: [ev("extension_offered"), ev("dealt", { seat: 0 }), ev("count")], state: state(), human: 0 });
  assert.deepEqual(got, ["packets", "thirds", "extension"]);
});

// A tutor over a plain object of preferences.
function make(start = {}) {
  const prefs = { tutor: true, muted: false, seen: [], ...start };
  const opened = [];
  const said = [];
  const tutor = createTutor({
    prefs: () => prefs,
    patch: (p) => Object.assign(prefs, p),
    open: (key, how) => opened.push([key, how.help]),
    speak: (group) => said.push(group),
  });
  return { tutor, prefs, opened, said };
}

test("a page opens at rest, once, and the intro line is said with the first", () => {
  const { tutor, opened, said } = make();
  assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), "packets");
  assert.deepEqual(opened, [["packets", false]]);
  assert.deepEqual(said, ["kibitz"]);
  tutor.close("packets");
  assert.equal(tutor.rest(state(), [], { human: 1 }), null, "nothing owed");
  tutor.rest(state(), [ev("reveal_start", { seat: 1 })], { human: 1 });
  assert.deepEqual(said, ["kibitz"], "the intro once");
});

test("one page at a time: the next waits until the first is closed", () => {
  const { tutor, opened } = make();
  tutor.rest(state(), [ev("dealt", { seat: 1 }), ev("reveal_start", { seat: 1 })], { human: 1 });
  assert.deepEqual(opened.map((o) => o[0]), ["packets"]);
  assert.equal(tutor.rest(state(), [], { human: 1 }), null, "still open");
  tutor.close("packets");
  assert.equal(tutor.rest(state(), [], { human: 1 }), "the-pull", "owed, now shown");
});

test("a page is not opened while a dialog is up, nor lost", () => {
  const { tutor } = make();
  assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1, blocked: true }), null);
  assert.equal(tutor.rest(state(), [], { human: 1 }), "packets");
});

test("'Not now' marks a page seen; 'Remind me' leaves it for the next deal", () => {
  const { tutor, prefs } = make();
  tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 });
  tutor.close("packets", false);
  assert.deepEqual(prefs.seen, ["packets"]);
  const again = make();
  again.tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 });
  again.tutor.close("packets", true);
  assert.deepEqual(again.prefs.seen, []);
  assert.equal(again.tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), null, "not twice in one deal");
  assert.equal(again.tutor.rest(state(), [ev("deal_start", { deal_number: 2 }), ev("dealt", { seat: 1 })], { human: 1 }), "packets", "at the next occurrence");
});

test("a seen page does not open again", () => {
  const { tutor } = make({ seen: ["packets"] });
  assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), null);
});

test("muted, or the tutor off, no page opens by itself; Help still works", () => {
  for (const start of [{ muted: true }, { tutor: false }]) {
    const { tutor, opened } = make(start);
    assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), null);
    assert.equal(tutor.help("bidding"), true);
    assert.deepEqual(opened, [["bidding", true]]);
  }
  assert.equal(make().tutor.help("no-such-page"), false);
});

test("Help marks the page seen", () => {
  const { tutor, prefs } = make();
  tutor.help("thirds");
  assert.deepEqual(prefs.seen, ["thirds"]);
});

test("the mute: one line, then silence; a second tap un-mutes without a word", () => {
  const { tutor, prefs, said } = make();
  tutor.toggleMute();
  assert.equal(prefs.muted, true);
  assert.deepEqual(said, ["kibitz-mute"]);
  tutor.mute();
  assert.deepEqual(said, ["kibitz-mute"], "once");
  assert.equal(tutor.active(), false);
  tutor.toggleMute();
  assert.equal(prefs.muted, false);
  assert.deepEqual(said, ["kibitz-mute"]);
  assert.equal(tutor.active(), true);
});

test("the intro line is said when the tab is first opened, once a sitting", () => {
  const { tutor, said } = make();
  tutor.intro();
  tutor.intro();
  assert.deepEqual(said, ["kibitz"]);
  tutor.reset();
  tutor.intro();
  assert.deepEqual(said, ["kibitz", "kibitz"]);
  const off = make({ tutor: false });
  off.tutor.intro();
  assert.deepEqual(off.said, [], "off: silent");
});
