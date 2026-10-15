// The tutor: which page falls due, when it opens, "Not now" and "Remind me",
// the mute (docs/TABLE3D.md 6.1, 6.2).
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createTutor, pagesDue, extendPromptDue, PAGE_KEYS } from "../src/tutor.js";

const spec = readFileSync(new URL("../../docs/TABLE3D.md", import.meta.url), "utf8");

const state = (over = {}) => ({ prompt: "opponent", phase: "play", legal: [], deal: { lead: null, trick_number: 1, own_hand: [] }, ...over });
const ev = (type, more = {}) => ({ type, ...more });

test("the page keys are those of the table in TABLE3D 6.2.2, in order", () => {
  const keys = [...spec.split("### 6.2 Tutorial pages")[1].split("### 6.3")[0].matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
  assert.deepEqual(PAGE_KEYS, keys);
});

test("each page is due at its own first moment", () => {
  const due = (events, s = state(), more = {}) => pagesDue({ events, state: s, human: 1, ...more });
  assert.deepEqual(due([ev("dealt", { seat: 1 })]), ["the-cards", "packets"], "the cards first, then the packets");
  assert.deepEqual(due([ev("bid", { seat: 0 })]), ["bidding"]);
  assert.deepEqual(due([], state({ prompt: "you", phase: "bid" })), ["bidding"]);
  assert.deepEqual(due([ev("reveal_start", { seat: 1 })]), ["the-pull"]);
  assert.deepEqual(due([], state({ prompt: "you", deal: { lead: "Kh", trick_number: 3, own_hand: [] } })), ["playing", "following"]);
  assert.deepEqual(due([], state({ prompt: "you", deal: { lead: null, trick_number: 3, own_hand: [] } })), ["playing"], "a lead is not a follow");
  assert.deepEqual(due([ev("card_played", { seat: 1, card: "Qd", source: "packet 2" })]), ["from-a-packet"], "still due after the fact");
  assert.deepEqual(due([ev("card_played", { seat: 0, card: "Qd", source: "packet 2" })]), [], "the opponent's cover is not yours");
  assert.deepEqual(due([ev("card_played", { seat: 1, card: "Qd", source: "hand" })]), []);
  assert.deepEqual(due([ev("announcement", { seat: 0, honour: "kings" })]), ["honours"]);
  assert.deepEqual(due([], state({ prompt: "you", legal: [{ action: "announce", honour: "trull" }] })), ["playing", "honours"]);
  assert.deepEqual(due([ev("pagat_ultimo", { seat: 0, result: "won" })]), ["pagat-ultimo"]);
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 23, own_hand: [{ card: "T1", public: false }] } })), ["pagat-ultimo", "scoring"]);
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 22, own_hand: [{ card: "T1", public: false }] } })), ["scoring"], "the last five tricks only");
  assert.deepEqual(due([ev("count", { thirds: [100, 110] })]), ["thirds", "scoring"]);
  assert.deepEqual(due([ev("game_scored", { result: "draw" })]), ["scoring", "draw"]);
  assert.deepEqual(due([ev("game_scored", { result: "declarer_win" })]), ["scoring"]);
  assert.deepEqual(due([ev("tour_end", { tour: 1 })]), ["touren-slate"]);
  assert.deepEqual(due([], state(), { slate: true }), ["touren-slate"]);
  assert.deepEqual(due([ev("extension_offered")]), ["extension"]);
});

test("pages due together come in the order of the table", () => {
  const got = pagesDue({ events: [ev("extension_offered"), ev("dealt", { seat: 0 }), ev("count")], state: state(), human: 0 });
  assert.deepEqual(got, ["the-cards", "packets", "thirds", "scoring", "extension"]);
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
  assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), "the-cards");
  assert.deepEqual(opened, [["the-cards", false]]);
  assert.deepEqual(said, ["kibitz"]);
  tutor.close("the-cards");
  tutor.close(tutor.rest(state(), [], { human: 1 }));
  assert.equal(tutor.rest(state(), [], { human: 1 }), null, "nothing owed");
  tutor.rest(state(), [ev("reveal_start", { seat: 1 })], { human: 1 });
  assert.deepEqual(said, ["kibitz"], "the intro once");
});

test("one page at a time: the next waits until the first is closed", () => {
  const { tutor, opened } = make();
  tutor.rest(state(), [ev("dealt", { seat: 1 }), ev("reveal_start", { seat: 1 })], { human: 1 });
  assert.deepEqual(opened.map((o) => o[0]), ["the-cards"]);
  assert.equal(tutor.rest(state(), [], { human: 1 }), null, "still open");
  tutor.close("the-cards");
  assert.equal(tutor.rest(state(), [], { human: 1 }), "packets", "the pages at one rest follow in the table's order");
  tutor.close("packets");
  assert.equal(tutor.rest(state(), [], { human: 1 }), "the-pull", "owed, now shown");
});

test("a page is not opened while a dialog is up, nor lost", () => {
  const { tutor } = make();
  assert.equal(tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1, blocked: true }), null);
  assert.equal(tutor.rest(state(), [], { human: 1 }), "the-cards");
});

test("'Not now' marks a page seen; 'Remind me' leaves it for the next deal", () => {
  const { tutor, prefs } = make();
  tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 });
  tutor.close("the-cards", false);
  assert.deepEqual(prefs.seen, ["the-cards"]);
  const again = make();
  again.tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 });
  again.tutor.close("the-cards", true);
  assert.deepEqual(again.prefs.seen, []);
  assert.equal(again.tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), "packets", "the next page owed, not the same twice in one deal");
  again.tutor.close("packets", true);
  assert.equal(again.tutor.rest(state(), [ev("dealt", { seat: 1 })], { human: 1 }), null, "not twice in one deal");
  assert.equal(again.tutor.rest(state(), [ev("deal_start", { deal_number: 2 }), ev("dealt", { seat: 1 })], { human: 1 }), "the-cards", "at the next occurrence");
});

test("a seen page does not open again", () => {
  const { tutor } = make({ seen: ["the-cards", "packets"] });
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

test("the long scoring page never falls due by itself, at any moment, and Help opens it (TABLE3D 6.2.2)", () => {
  const everything = [ev("dealt"), ev("bid"), ev("reveal_start"), ev("announcement"), ev("pagat_ultimo"), ev("trick_won"), ev("count"), ev("game_scored", { result: "draw" }), ev("tour_end"), ev("extension_offered")];
  const late = state({ prompt: "you", phase: "play", legal: [{ action: "announce", honour: "trull" }], deal: { lead: "Kh", trick_number: 27, own_hand: [{ card: "T1" }] } });
  assert.ok(!pagesDue({ events: everything, state: late, human: 1, slate: true }).includes("scoring-full"));
  const { tutor, opened } = make();
  assert.equal(tutor.help("scoring-full"), true);
  assert.deepEqual(opened, [["scoring-full", true]]);
});

test("a link from a page that opened by itself marks it seen and opens the other as a Help page", () => {
  const { tutor, prefs, opened } = make();
  assert.equal(tutor.rest(state(), [ev("count")], { human: 1 }), "thirds");
  assert.equal(tutor.follow("scoring-full"), true);
  assert.deepEqual(opened, [["thirds", false], ["scoring-full", true]]);
  assert.deepEqual(prefs.seen, ["thirds", "scoring-full"]);
  assert.equal(tutor.current(), "scoring-full");
  tutor.helpClosed("scoring-full");
  assert.equal(tutor.current(), null);
  // the page that was owed with it still opens at the next rest
  assert.equal(tutor.rest(state(), [], { human: 1 }), "scoring");
});

test("a link from a Help page opens the other and leaves the first as it was", () => {
  const { tutor, prefs, opened } = make();
  tutor.help("thirds");
  assert.equal(tutor.follow("scoring-full"), true);
  assert.deepEqual(opened, [["thirds", true], ["scoring-full", true]]);
  assert.deepEqual(prefs.seen, ["thirds", "scoring-full"]);
  assert.equal(tutor.follow("no-such-page"), false);
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

// The pacing (docs/review/tutorial-pacing.md): each page is due no later than
// the first decision it explains is asked, with the events and state of that moment.
const ASKED = [
  ["bidding", ["the-cards", "packets", "bidding"], [ev("dealt", { seat: 1 })], state({ prompt: "you", phase: "bid", legal: [{ action: "take" }, { action: "pass" }] })],
  ["playing", ["playing"], [], state({ prompt: "you", legal: [{ action: "play", card: "T5", source: "hand" }], deal: { lead: null, trick_number: 1, own_hand: [] } })],
  ["following", ["playing", "following"], [], state({ prompt: "you", legal: [{ action: "play", card: "T5", source: "hand" }], deal: { lead: "Kh", trick_number: 1, own_hand: [] } })],
  ["from-a-packet", ["playing", "from-a-packet"], [], state({ prompt: "you", legal: [{ action: "play", card: "Qs", source: "packet 2" }], deal: { lead: null, trick_number: 1, own_hand: [] } })],
  ["honours", ["playing", "honours"], [], state({ prompt: "you", legal: [{ action: "announce", honour: "kings" }, { action: "play", card: "T5", source: "hand" }], deal: { lead: null, trick_number: 1, own_hand: [] } })],
  ["extension", ["extension"], [], state({ prompt: "you", phase: "extend", legal: [{ action: "extend", accept: true }] })],
];
for (const [name, want, events, s] of ASKED) {
  test(`pacing: the ${name} page is due when its decision is asked`, () => {
    const got = pagesDue({ events, state: s, human: 1 });
    for (const k of want) assert.ok(got.includes(k), `${k} missing from ${got}`);
  });
}

test("pacing: the count and the scoring are explained before the first count", () => {
  const due = (events, s = state()) => pagesDue({ events, state: s, human: 1 });
  assert.deepEqual(due([ev("trick_won", { winner: 0 })]), ["thirds"], "the tally starts to run");
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 21, own_hand: [] } })), ["scoring"], "before the last tricks");
  assert.deepEqual(due([], state({ deal: { lead: null, trick_number: 20, own_hand: [] } })), []);
  assert.ok(due([ev("count")]).includes("scoring"), "never later than the count");
  assert.ok(due([ev("count")]).includes("thirds"));
});

test("pacing: the pages of one moment come in the order they are needed", () => {
  const i = (k) => PAGE_KEYS.indexOf(k);
  const order = ["the-cards", "packets", "bidding", "the-pull", "playing", "following", "from-a-packet", "honours", "thirds", "pagat-ultimo", "scoring", "draw", "touren-slate", "extension", "scoring-full"];
  assert.deepEqual(PAGE_KEYS, order);
  assert.ok(i("bidding") < i("playing"));
});

test("pacing: the cards are the first page of the first deal, ahead of the packets (TABLE3D 6.2.2)", () => {
  assert.equal(PAGE_KEYS[0], "the-cards");
  const { tutor, opened } = make();
  tutor.rest(state(), [ev("deal_start", { deal_number: 1 }), ev("dealt", { seat: 1 })], { human: 1 });
  for (let n = 0; tutor.current() && n < 5; n++) {
    tutor.close(tutor.current());
    tutor.rest(state(), [], { human: 1 });
  }
  assert.deepEqual(opened.map((o) => o[0]), ["the-cards", "packets"]);
  assert.deepEqual(pagesDue({ events: [ev("bid", { seat: 0 })], state: state(), human: 1 }), ["bidding"], "only at the deal");
});

test("pacing: the extension prompt waits for every due page, then is asked", () => {
  const { tutor, opened } = make();
  const extend = state({ prompt: "you", phase: "extend", legal: [{ action: "extend", accept: true }] });
  const asked = () => extendPromptDue({ mine: true, busy: false, phase: "extend", extendOpen: false, pageOpen: Boolean(tutor.current()) });
  // At rest: the page first (as rested() does), then the prompt.
  tutor.rest(extend, [ev("extension_offered"), ev("count")], { human: 1 });
  assert.equal(asked(), false, "a page is open: the prompt waits");
  let guard = 0;
  while (tutor.current() && guard++ < 10) {
    tutor.close(tutor.current());
    tutor.rest(extend, [], { human: 1 });
  }
  assert.deepEqual(opened.map((o) => o[0]), ["thirds", "scoring", "extension"]);
  assert.equal(asked(), true, "all closed: the prompt is asked");
  const off = make({ tutor: false });
  off.tutor.rest(extend, [ev("extension_offered")], { human: 1 });
  assert.equal(extendPromptDue({ mine: true, busy: false, phase: "extend", extendOpen: false, pageOpen: Boolean(off.tutor.current()) }), true, "tutor off: never held back");
  assert.equal(extendPromptDue({ mine: true, busy: true, phase: "extend", extendOpen: false, pageOpen: false }), false, "not while cards move");
});
