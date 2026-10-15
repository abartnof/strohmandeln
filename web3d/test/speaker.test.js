import test from "node:test";
import assert from "node:assert/strict";
import { createSpeaker } from "../src/speaker.js";
import { GAP, LATE, SLACK, saying } from "../../kit/js/dialogue.js";

// A clock moved by hand; `later(ms, fn)` runs at its time.
function rig() {
  let now = 0;
  const queue = [];
  const shown = [];
  const speaker = createSpeaker({
    clock: () => now,
    later: (ms, fn) => queue.push({ at: now + ms, fn }),
    show: (line) => shown.push({ text: line.text, at: now }),
    hide: (line) => shown.push({ text: line.text, hidden: now }),
  });
  const run = (to) => {
    for (;;) {
      queue.sort((a, b) => a.at - b.at);
      if (!queue.length || queue[0].at > to) break;
      const e = queue.shift();
      now = e.at;
      e.fn();
    }
    now = to;
  };
  return { speaker, run, shown, at: () => now };
}
const said = (r) => r.shown.filter((s) => !s.hidden).map((s) => s.text);

test("a line is said at its moment when nothing else is being said", () => {
  const r = rig();
  r.run(1000);
  r.speaker.offer({ text: "Ha.", call: false });
  r.run(1000);
  assert.deepEqual(r.shown[0], { text: "Ha.", at: 1000 });
});

test("a line waits for the one before it, in the order of their moments", () => {
  const r = rig();
  r.speaker.offer({ text: "First line here", call: true });
  r.run(100);
  r.speaker.offer({ text: "Second", call: true });
  r.run(10_000);
  const first = r.shown.find((s) => s.text === "First line here");
  const second = r.shown.find((s) => s.text === "Second" && !s.hidden);
  assert.equal(first.at, 0);
  assert.equal(second.at, saying("First line here") + GAP);
});

test("chatter more than LATE after its own moment is left out", () => {
  const r = rig();
  const long = "x".repeat(40); // saying() is 250 + 65 * 40 = 2850 ms
  r.speaker.offer({ text: long, call: true });
  r.run(100);
  r.speaker.offer({ text: "chatter", call: false });
  r.run(20_000);
  assert.ok(saying(long) + GAP - 100 > LATE);
  assert.deepEqual(said(r), [long]);
});

test("chatter that would make a call more than SLACK late is cut when the call comes", () => {
  const r = rig();
  const chatter = "x".repeat(20); // 1550 ms
  r.speaker.offer({ text: chatter, call: false });
  r.run(300);
  r.speaker.offer({ text: "Call!", call: true });
  r.run(10_000);
  const call = r.shown.find((s) => s.text === "Call!" && !s.hidden);
  assert.ok(call.at - 300 <= SLACK, "the call is not late");
  assert.ok(r.shown.some((s) => s.text === chatter && s.hidden !== undefined && s.hidden <= call.at), "the chatter came down");
});

test("chatter not yet begun is dropped when a call comes before its turn", () => {
  const r = rig();
  r.speaker.offer({ text: "x".repeat(10), call: true }); // ends at 900
  r.run(100);
  r.speaker.offer({ text: "chatter", call: false }); // would begin at 990
  r.run(200);
  r.speaker.offer({ text: "Call!", call: true });
  r.run(20_000);
  assert.ok(!said(r).includes("chatter"));
  assert.ok(said(r).includes("Call!"));
});

test("chatter that is nearly done is let finish; the call waits at most SLACK", () => {
  const r = rig();
  const text = "x".repeat(10); // 900 ms
  r.speaker.offer({ text, call: false });
  r.run(800);
  r.speaker.offer({ text: "Call!", call: true });
  r.run(10_000);
  const call = r.shown.find((s) => s.text === "Call!" && !s.hidden);
  assert.equal(call.at, saying(text) + GAP);
  assert.ok(call.at - 800 <= SLACK);
  assert.ok(said(r).includes(text), "the chatter was said");
});

test("stop() forgets what is queued and what is showing", () => {
  const r = rig();
  r.speaker.offer({ text: "a", call: true });
  r.speaker.offer({ text: "b", call: true });
  r.speaker.stop();
  r.run(10_000);
  assert.ok(said(r).length <= 1);
  r.speaker.offer({ text: "c", call: true });
  r.run(20_000);
  assert.ok(said(r).includes("c"));
});
