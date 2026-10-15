// The count in thirds (docs/TABLE3D.md 2.5), for the kit's HUD and popups.
//
// A take is told in groups of three cards: a line a group, `thirds` the sum of
// 3 x value - 2 over its cards (RULES 8.1), a last group of fewer than three
// the same way. The two takes' lines alternate, so both sides count at once
// and the count is as long as the longer take, not the two together.
import { defineGame } from "../../kit/js/scoring.js";
import { thirdsOf } from "./cardinfo.js";
import { thirds } from "./strings.js";

// 107 thirds is 35 2/3, and reaches 36: the finish lights at 107.
export const TARGET = 107;
export const SEGMENTS = 36;

export { thirds };

// A take's lines: { side, group, codes, thirds }. `counted` is each seat's
// cards in the order they are turned up.
export function countLines(counted, human) {
  const per = counted.map((cards, seat) => {
    const lines = [];
    for (let g = 0; g * 3 < cards.length; g++) {
      const codes = cards.slice(g * 3, g * 3 + 3);
      lines.push({ side: seat === human ? "you" : "opp", group: g, codes, thirds: codes.reduce((s, c) => s + thirdsOf(c), 0) });
    }
    return lines;
  });
  const [you, opp] = human === 0 ? per : [per[1], per[0]];
  const out = [];
  for (let i = 0; i < Math.max(you.length, opp.length); i++) {
    if (you[i]) out.push(you[i]);
    if (opp[i]) out.push(opp[i]);
  }
  return out;
}

// The running tally (2.5.3): the cards each side has won so far in the deal
// (the events' `trick_won`, since the last `deal_start`), counted in exact
// thirds as the count will count them. After trick 27 it is the `count`
// event's own numbers. `human` is the human's seat; "you" is that seat's line.
export function runningTally(events, human) {
  const tally = { you: 0, opp: 0 };
  for (const e of events) {
    if (e.type === "deal_start") tally.you = tally.opp = 0;
    else if (e.type === "trick_won") tally[e.winner === human ? "you" : "opp"] += e.cards.reduce((n, c) => n + thirdsOf(c), 0);
  }
  return tally;
}

// The edge of the count (2.5.3, 2.5.4): once a side has 107 thirds the target
// is joined by a tick on both lines; a side on 106 (35 1/3) is one short.
export const edgeOf = (totals) => ({
  tick: totals.you >= TARGET || totals.opp >= TARGET,
  short: ["you", "opp"].filter((side) => totals[side] === TARGET - 1),
});

// The bonuses (honours, ultimo, valat) a deal paid, as lines of the bonus
// category: Points, in the credit's own words (`label(cause)`), over the
// player credited. `credits` are the `game_scored` credits (the game's own
// Points are not bonuses).
const BONUS_CAUSES = new Set(["trull", "kings", "ultimo_won", "ultimo_lost", "valat"]);
export function bonusLines(credits, human, label) {
  return credits.filter((c) => BONUS_CAUSES.has(c.cause)).map((c) => ({ side: c.seat === human ? "you" : "opp", bonus: true, points: c.points, label: label(c.cause) }));
}

// What the choreography needs of the game: which side a line is, which cards it turns up.
export const COUNT_RULES = Object.freeze({
  line: (l) => (l.bonus ? { side: l.side, cat: "bonus", pts: l.points } : { side: l.side, cat: "count", pts: l.thirds }),
  cards: (l) => l.codes ?? [],
});

// The HUD's game, for a language: `words` are the strings (hud.*).
export function countGame(words) {
  return defineGame({
    categories: [["count", words.category], ["bonus", words.bonus]],
    apart: ["bonus"], // Points beside the count in thirds, never part of the line to 107
    target: TARGET,
    segments: SEGMENTS,
    scale: 3,
    total: 210,
    lit: (units) => (units >= TARGET ? SEGMENTS : Math.floor(units / 3)),
    format: (units, cat) => (cat === "bonus" ? String(units) : thirds(units)),
    goal: words.goal,
    names: words.names,
    words: words.hud,
    ...COUNT_RULES,
    popup: (l, i, lines, info) => (l.bonus ? { pts: info.pts, label: l.label } : { pts: l.thirds, label: words.group(l.group + 1) }),
  });
}
