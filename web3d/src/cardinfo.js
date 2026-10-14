// What the table needs to know of a card code (docs/RULES.md 1.2) to lay a
// hand out and to tell the count: the order a hand is held in, a card's
// value in thirds (RULES 8.1: 3 x value - 2), and the pieces a name is made
// of. The page holds no rule of play: nothing here says what may be played.
import { parseCode } from "./tarockfaces.js";

const SUIT_ORDER = ["h", "c", "d", "s"]; // alternating colours, as a hand is sorted
const COURT_RANK = { K: 20, Q: 19, C: 18, J: 17, 10: 10, 9: 9, 8: 8, 7: 7, 4: 4, 3: 3, 2: 2, 1: 1 };

// The order a hand lies in: the suits first (each from its king down), then
// the tarocks from the Pagat up, the Sküs last.
export function handSortKey(code) {
  const c = parseCode(code);
  if (c.kind === "suit") return SUIT_ORDER.indexOf(c.suit) * 100 + (100 - COURT_RANK[c.rank]);
  if (c.kind === "tarock") return 1000 + c.n;
  return 1100;
}

export const sortCodes = (codes) => [...codes].sort((a, b) => handSortKey(a) - handSortKey(b));

// A card's value in Points before the count in threes: the Trull and the
// kings 5, queens 4, cavaliers 3, jacks 2, the rest 1 (RULES 8.1).
export function valueOf(code) {
  const c = parseCode(code);
  if (c.kind === "sku") return 5;
  if (c.kind === "tarock") return c.n === 1 || c.n === 21 ? 5 : 1;
  return { K: 5, Q: 4, C: 3, J: 2 }[c.rank] ?? 1;
}

// A card's count in thirds of a point: 3 x value - 2.
export const thirdsOf = (code) => 3 * valueOf(code) - 2;

// The honours the human may be asked about (RULES 11): the Trull's cards, the kings.
export const TRULL = Object.freeze(["SK", "T21", "T1"]);
export const KINGS = Object.freeze(["Kh", "Kd", "Kc", "Ks"]);
export const isTarock = (code) => parseCode(code).kind !== "suit";
export const suitOf = (code) => {
  const c = parseCode(code);
  return c.kind === "suit" ? c.suit : null;
};
export const rankOf = (code) => {
  const c = parseCode(code);
  return c.kind === "suit" ? c.rank : c.kind === "tarock" ? `T${c.n}` : "SK";
};
