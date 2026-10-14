// The count in thirds (docs/TABLE3D.md 2.5), for the kit's HUD and popups.
//
// A take is told in groups of three cards: a line a group, `thirds` the sum of
// 3 x value - 2 over its cards (RULES 8.1), a last group of fewer than three
// the same way. The two takes' lines alternate, so both sides count at once
// and the count is as long as the longer take, not the two together.
import { defineGame } from "../../kit/js/scoring.js";
import { thirdsOf } from "./cardinfo.js";

// 107 thirds is 35 2/3, and reaches 36: the finish lights at 107.
export const TARGET = 107;
export const SEGMENTS = 36;

// A value in thirds as points and a vulgar fraction: 107 -> "35⅔", 106 -> "35⅓", 105 -> "35".
export function thirds(units) {
  const whole = Math.floor(units / 3);
  const rest = units % 3;
  const fraction = rest === 1 ? "⅓" : rest === 2 ? "⅔" : "";
  return whole === 0 && fraction ? fraction : `${whole}${fraction}`;
}

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

// What the choreography needs of the game: which side a line is, which cards it turns up.
export const COUNT_RULES = Object.freeze({
  line: (l) => ({ side: l.side, cat: "count", pts: l.thirds }),
  cards: (l) => l.codes,
});

// The HUD's game, for a language: `words` are the strings (hud.*).
export function countGame(words) {
  return defineGame({
    categories: [["count", words.category]],
    target: TARGET,
    segments: SEGMENTS,
    scale: 3,
    total: 210,
    lit: (units) => Math.min(SEGMENTS, Math.floor((units + 1) / 3)),
    format: (units) => thirds(units),
    goal: words.goal,
    names: words.names,
    words: words.hud,
    ...COUNT_RULES,
    popup: (l) => ({ pts: l.thirds, label: words.group(l.group + 1) }),
  });
}
