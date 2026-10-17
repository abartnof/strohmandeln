// The cards-out tracker's model and panel (docs/TABLE3D.md 6.6): what is shown
// for the engine's payload, in both languages; the fold; hidden when Hints is
// off; and the panel's DOM. Over hand-made payloads for the exact words, and
// over the real engine's for the shape.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings, thirds } from "../src/strings.js";
import { ALL_CODES } from "../src/tarockfaces.js";
import { cardLabel, trackerFits, trackerModel, trackerOpen, trackerVisible } from "../src/tracker.js";
import { trackerPanel } from "../src/trackerview.js";
import { fakeRoot } from "./support/fakedom.js";
import { playSitting, rng } from "./sim.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const words = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, lang, cardName: s.cardName, romanName: s.romanName, thirds };
};

const KINDS = ["h", "d", "c", "s", "trump"];
const group = (kind, o = {}) => ({ kind, mine: [], out: [], unseen: 0, void: false, best: null, top_out: null, master: false, ...o });

// A payload: every card unseen but those placed; groups as given (the engine's).
function payload({ human = 0, trick = 5, places = {}, groups = {}, out_thirds = 94 } = {}) {
  const cards = Object.fromEntries(ALL_CODES.map((c) => [c, { place: "unseen" }]));
  for (const [c, p] of Object.entries(places)) cards[c] = typeof p === "string" ? { place: p } : { place: "played", by: p[0], trick: p[1] };
  return {
    human,
    trick,
    cards,
    groups: KINDS.map((k) => group(k, groups[k])),
    out_count: 30,
    out_thirds,
    unseen_count: 20,
    unseen_thirds: 60,
  };
}

const lines = (p, lang = "en") => trackerModel(p, words(lang)).lines.map((l) => l.text);

test("it is shown with Hints on, its own setting on, and the Kibitzer not muted", () => {
  const on = { hints: true, tracker: true, tutor: true, muted: false };
  assert.equal(trackerVisible(on), true);
  assert.equal(trackerVisible({ ...on, hints: false }), false);
  assert.equal(trackerVisible({ ...on, tracker: false }), false);
  assert.equal(trackerVisible({ ...on, muted: true }), false, "a muted Kibitzer keeps no slate");
  assert.equal(trackerVisible({ ...on, tutor: false, muted: true }), true, "the mute is the tutor's: with the tutor off it does not apply");
});

test("the panel is folded unless the person unfolded it; a stale saved fold is ignored", () => {
  assert.equal(trackerOpen({}), false);
  assert.equal(trackerOpen({ trackerFolded: false }), false, "an old saved preference does not open it");
  assert.equal(trackerOpen({ trackerFolded: true }), false);
  assert.equal(trackerOpen({ unfolded: true }), true);
});

test("the panel is for a window that is wide and not upright; otherwise it is not asked for", () => {
  assert.equal(trackerFits({ width: 1280, height: 720 }), true);
  assert.equal(trackerFits({ width: 1024, height: 768 }), true);
  assert.equal(trackerFits({ width: 390, height: 844 }), false);
  assert.equal(trackerFits({ width: 844, height: 390 }), true);
  assert.equal(trackerFits({ width: 700, height: 400 }), false, "700 px or less is out");
  assert.equal(trackerFits({ width: 800, height: 1000 }), false, "upright");
});

test("no payload, no panel", () => {
  assert.equal(trackerModel(null, words("en")), null);
});

test("the grid: four suits from the king down, then the tarocks in rows of 8, 8 and 6", () => {
  const m = trackerModel(payload(), words("en"));
  assert.deepEqual(m.rows.map((r) => r.id), ["h", "d", "c", "s", "t1", "t2", "t3"]);
  assert.deepEqual(m.rows.map((r) => r.cells.length), [8, 8, 8, 8, 8, 8, 6]);
  const codes = m.rows.flatMap((r) => r.cells.map((c) => c.code));
  assert.deepEqual([...codes].sort(), [...ALL_CODES].sort(), "all 54, once each");
  assert.deepEqual(m.rows[0].cells.map((c) => c.label), ["K", "Q", "C", "J", "1", "2", "3", "4"]);
  assert.deepEqual(m.rows[3].cells.map((c) => c.label), ["K", "Q", "C", "J", "10", "9", "8", "7"]);
  assert.deepEqual(m.rows[4].cells.map((c) => c.label), ["Sküs", "XXI", "XX", "XIX", "XVIII", "XVII", "XVI", "XV"]);
  assert.deepEqual(m.rows[6].cells.map((c) => c.label), ["VI", "V", "IIII", "III", "II", "I"]);
});

test("the labels follow the language: K D C B in German", () => {
  const m = trackerModel(payload(), words("de"));
  assert.deepEqual(m.rows[0].cells.slice(0, 4).map((c) => c.label), ["K", "D", "C", "B"]);
  assert.equal(cardLabel("Jh", words("de")), "♥︎B");
  assert.equal(cardLabel("Jh", words("en")), "♥︎J");
  assert.equal(cardLabel("T4", words("en")), "IIII");
  assert.equal(cardLabel("SK", words("de")), "Sküs");
  assert.equal(cardLabel("10s", words("en")), "♠︎10");
});

test("a cell's state is yours, played or still out; who played it and when are in its title", () => {
  const p = payload({
    human: 1,
    places: { Kh: "hand", T21: "cover", Qh: [1, 2], T5: [0, 3], Ks: "opp_cover", T9: "opp_hand" },
  });
  const m = trackerModel(p, words("en"));
  const cell = (code) => m.rows.flatMap((r) => r.cells).find((c) => c.code === code);
  assert.equal(cell("Kh").state, "mine");
  assert.equal(cell("T21").state, "mine");
  assert.equal(cell("Qh").state, "played");
  assert.equal(cell("T5").state, "played");
  assert.equal(cell("Ks").state, "out");
  assert.equal(cell("T9").state, "out");
  assert.equal(cell("Jd").state, "out", "an unseen card");
  assert.match(cell("Qh").title, /queen of hearts: played by you, trick 2/);
  assert.equal(cell("T5").title, "tarock V: played by your opponent, trick 3", "Roman, as the face shows it");
  assert.match(cell("Ks").title, /face up on your opponent's packet/);
  assert.match(trackerModel(p, words("de")).rows[0].cells[1].title, /Herz-Dame: von Ihnen gespielt, Stich 2/);
});

test("the legend names the three states, and says still out holds your face-down packet cards", () => {
  const m = trackerModel(payload(), words("en"));
  assert.deepEqual(m.legend.map((l) => l.state), ["mine", "played", "out"]);
  assert.deepEqual(m.legend.map((l) => l.text), ["Yours", "Played", "Still out, with your face-down packet cards"]);
  assert.deepEqual(trackerModel(payload(), words("de")).legend.map((l) => l.text), ["Ihre", "Gespielt", "Noch draußen, auch Ihre verdeckten Paketkarten"]);
});

test("tarocks out: the count and the highest four, then an ellipsis", () => {
  const out = ["SK", "T21", "T19", "T18", "T10", "T7", "T3"];
  const p = payload({ groups: { trump: { out, unseen: 7, top_out: "SK" } } });
  assert.equal(lines(p)[0], "Tarocks still out: 7 — Sküs, XXI, XIX, XVIII…");
  assert.equal(lines(p, "de")[0], "Tarock noch draußen: 7 — Sküs, XXI, XIX, XVIII…");
  const few = payload({ groups: { trump: { out: ["T21", "T2"], unseen: 2, top_out: "T21" } } });
  assert.equal(lines(few)[0], "Tarocks still out: 2 — XXI, II");
  const none = payload();
  assert.equal(lines(none)[0], "No tarock is out: every one is yours or played.");
  assert.equal(lines(none, "de")[0], "Kein Tarock ist mehr draußen: alle sind in Ihrer Hand oder gespielt.");
});

test("a master: one is a sentence, several are one line, the tarock first", () => {
  const one = payload({ groups: { s: { mine: ["Ks"], best: "Ks", master: true, out: ["Qs"], top_out: "Qs" } } });
  assert.ok(lines(one).includes("Your ♠︎K is the highest spade left."), lines(one).join("|"));
  assert.ok(lines(one, "de").includes("Höchstes Pik im Spiel: ♠︎K, es gehört Ihnen."));
  const t = payload({ groups: { trump: { mine: ["T21"], best: "T21", master: true, out: ["T5"], top_out: "T5" } } });
  assert.ok(lines(t).includes("Your XXI is the highest tarock left."));
  const several = payload({
    groups: {
      s: { mine: ["Ks"], best: "Ks", master: true },
      h: { mine: ["Qh"], best: "Qh", master: true, out: ["2h"], top_out: "2h" },
      trump: { mine: ["T21"], best: "T21", master: true },
    },
  });
  assert.ok(lines(several).includes("Your highest cards, nothing higher out: XXI, ♥︎Q, ♠︎K"), lines(several).join("|"));
  const notMaster = payload({ groups: { s: { mine: ["Qs"], best: "Qs", master: false, out: ["Ks"], top_out: "Ks" } } });
  assert.ok(!lines(notMaster).some((l) => /highest/.test(l)));
});

test("a suit master that he can trump says so: he has shown the suit void and tarocks are out", () => {
  const spade = { mine: ["Ks"], best: "Ks", master: true, out: ["Qs"], top_out: "Qs" };
  const trumpOut = { out: ["T5"], top_out: "T5" };
  const ruff = payload({ groups: { s: { ...spade, void: true }, trump: trumpOut } });
  assert.ok(lines(ruff).includes("Your ♠︎K is the highest spade left, but he may trump it."), lines(ruff).join("|"));
  assert.ok(lines(ruff, "de").includes("Höchstes Pik im Spiel: ♠︎K, aber er kann stechen."), lines(ruff, "de").join("|"));
  // no void, or no tarock out: nothing to warn of
  assert.ok(lines(payload({ groups: { s: spade, trump: trumpOut } })).includes("Your ♠︎K is the highest spade left."));
  assert.ok(lines(payload({ groups: { s: { ...spade, void: true } } })).includes("Your ♠︎K is the highest spade left."));
  // a tarock master is never trumped
  const t = payload({ groups: { trump: { mine: ["T21"], best: "T21", master: true, out: ["T5"], top_out: "T5", void: true } } });
  assert.ok(lines(t).includes("Your XXI is the highest tarock left."));
  // in a list, a mark beside the card
  const several = payload({
    groups: {
      s: { ...spade, void: true },
      h: { mine: ["Qh"], best: "Qh", master: true, out: ["2h"], top_out: "2h" },
      trump: { mine: ["T21"], best: "T21", master: true, out: ["T5"], top_out: "T5" },
    },
  });
  assert.ok(lines(several).includes("Your highest cards, nothing higher out: XXI, ♥︎Q, ♠︎K (he may trump)"), lines(several).join("|"));
  assert.ok(lines(several, "de").includes("Ihre höchsten Karten, nichts Höheres draußen: XXI, ♥︎D, ♠︎K (er kann stechen)"));
});

test("shown voids: suits and tarocks, joined", () => {
  const h = payload({ groups: { h: { void: true } } });
  assert.ok(lines(h).includes("Your opponent has shown no hearts."));
  assert.ok(lines(h, "de").includes("Ihr Gegner hat kein Herz mehr gezeigt."));
  const two = payload({ groups: { h: { void: true }, c: { void: true } } });
  assert.ok(lines(two).includes("Your opponent has shown no hearts or clubs."));
  assert.ok(lines(two, "de").includes("Ihr Gegner hat kein Herz oder Kreuz mehr gezeigt."));
  const t = payload({ groups: { trump: { void: true } } });
  assert.ok(lines(t).includes("Your opponent has shown no tarocks."));
  assert.ok(lines(t, "de").includes("Ihr Gegner hat kein Tarock mehr gezeigt."));
  assert.ok(!lines(payload()).some((l) => /opponent has shown/.test(l)));
});

test("the points still out are told in thirds, of the 70 (a point is what the count says, not what is paid)", () => {
  assert.ok(lines(payload({ out_thirds: 94 })).includes("Still out: 31⅓ of 70 points"));
  assert.ok(lines(payload({ out_thirds: 210 })).includes("Still out: 70 of 70 points"));
  assert.ok(lines(payload({ out_thirds: 94 }), "de").includes("Noch draußen: 31⅓ von 70 Augen"));
});

test("two to four lines, in order: tarocks, masters, voids, points", () => {
  const all = payload({
    groups: {
      trump: { mine: ["T21"], best: "T21", master: true, out: ["T5"], top_out: "T5" },
      h: { void: true },
    },
  });
  const m = trackerModel(all, words("en"));
  assert.deepEqual(m.lines.map((l) => l.id), ["trumps", "masters", "voids", "points"]);
  assert.deepEqual(trackerModel(payload(), words("en")).lines.map((l) => l.id), ["trumps", "points"]);
});

test("the screen reader gets a sentence per kind in place of the grid", () => {
  const p = payload({ groups: { h: { mine: ["Qh"], out: ["Kh", "Ch"] }, d: {} } });
  const m = trackerModel(p, words("en"));
  assert.equal(m.sr.length, 5);
  assert.equal(m.sr[0], "Hearts: yours queen of hearts; still out king of hearts, cavall of hearts.");
  assert.equal(m.sr[1], "Diamonds: yours none; still out none.");
  assert.match(trackerModel(p, words("de")).sr[0], /^Herz: Ihre Herz-Dame; noch draußen Herz-König, Herz-Cavall\.$/);
});

test("the panel: a labelled region, the lines a list, the grid hidden from the screen reader, a Fold button", () => {
  const w = words("en");
  const m = trackerModel(payload({ groups: { h: { void: true } } }), w);
  const node = trackerPanel(fakeRoot().ownerDocument, m, { foldLabel: w.t("tracker.fold"), foldTitle: w.t("tracker.fold.title") });
  assert.equal(node.getAttribute("role"), "region");
  assert.equal(node.getAttribute("aria-label"), "Cards out");
  assert.equal(node.byClass("tracker-lines")[0].tagName, "UL");
  assert.equal(node.byClass("tracker-lines")[0].children.length, m.lines.length);
  assert.equal(node.byClass("tracker-grid")[0].getAttribute("aria-hidden"), "true");
  assert.equal(node.byClass("tracker-grid")[0].byClass("tracker-cell").length, 54);
  assert.equal(node.byClass("tracker-sr")[0].children.length, 5);
  const fold = node.byAttr("data-act", "tracker-fold")[0];
  assert.equal(fold.getAttribute("title"), "Fold the count away");
  assert.equal(fold.textContent, "Fold");
  const mark = node.byClass("tracker-cell").find((c) => c.getAttribute("data-code") === "Kh");
  assert.equal(mark.getAttribute("data-state"), "out");
  assert.equal(node.byClass("tracker-legend")[0].children.length, 3);
});

test("over a real sitting every payload makes a panel of two to four lines in both languages, nothing unspoken", async () => {
  const random = rng(5);
  let looked = 0;
  const { engine } = await playSitting(
    { seed: 31, touren: 3, human: 0, level: 1 },
    {
      policy: (s) => s.legal[Math.floor(random() * s.legal.length)],
      limit: 400,
      onReply: (reply, e) => {
        const p = e.tracker().tracker;
        if (!p) return;
        for (const lang of ["en", "de"]) {
          const m = trackerModel(p, words(lang));
          assert.ok(m.lines.length >= 2 && m.lines.length <= 4, m.lines.map((l) => l.text).join("|"));
          for (const text of [...m.lines.map((l) => l.text), ...m.sr, ...m.rows.flatMap((r) => r.cells.map((c) => c.title))]) {
            assert.ok(!/[{}]|tracker\./.test(text), `unfilled: ${text}`);
          }
          const cells = m.rows.flatMap((r) => r.cells);
          assert.equal(cells.length, 54);
          const states = (s) => cells.filter((c) => c.state === s).length;
          const places = Object.values(p.cards);
          assert.equal(states("mine"), places.filter((x) => x.place === "hand" || x.place === "cover").length);
          assert.equal(states("played"), places.filter((x) => x.place === "played").length);
          assert.equal(states("out"), p.out_count);
        }
        looked++;
      },
    },
  );
  assert.ok(engine);
  assert.ok(looked > 100, `${looked}`);
});
