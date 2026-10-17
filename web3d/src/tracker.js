// The cards-out tracker as the table shows it (docs/TABLE3D.md 6.6): the
// engine's payload (`strohmandeln_tracker`, docs/PROTOCOL.md) turned into a
// grid of cells, a legend, the learner's lines and the screen reader's
// sentences, in the current language. The engine decides the masters, the
// voids and the points; this holds no rule of play, only words and layout.
import { hintsOn } from "./tutor.js";
import { GLYPH, courtLabels, parseCode, tarockNumeral } from "./tarockfaces.js";

const SUITS = ["h", "d", "c", "s"];
const KINDS = [...SUITS, "trump"];
// The fixed order of a suit's cards, king down (the printed pips: 1 2 3 4 in
// red suits, 10 9 8 7 in black), and the tarocks from the Sküs down in rows of
// 8, 8 and 6.
const RANKS = { red: ["K", "Q", "C", "J", "1", "2", "3", "4"], black: ["K", "Q", "C", "J", "10", "9", "8", "7"] };
const TAROCK_ROWS = [8, 8, 6];
const TAROCKS_HIGH_TO_LOW = ["SK", ...Array.from({ length: 21 }, (_, i) => `T${21 - i}`)];
// How many of the tarocks out the first line names.
const NAMED = 4;
const STATES = ["mine", "played", "theirs", "out"];
// A row's three counts after its cells: yours (held or played by you), played
// by your opponent, and the two together (the row's cells not still out).
const COUNTS = ["you", "opp", "seen"];
const counts = (cells) => {
  const n = (...states) => cells.filter((c) => states.includes(c.state)).length;
  return { you: n("mine", "played"), opp: n("theirs"), seen: n("mine", "played", "theirs") };
};

// Shown while Hints is on, the setting is on, and the Kibitzer is not muted
// (the mute is the tutor's, so with the tutor off it does not apply).
export const trackerVisible = (prefs) => Boolean(hintsOn(prefs) && prefs.tracker);

// The panel is for a wide window held sideways: upright or 700 px and under it
// is not shown, and the engine is not asked (6.6.6; tracker.css hides the same).
export const trackerFits = ({ width, height }) => width > 700 && width / height > 0.85;

// Open or folded: folded at the start of every deal and on load, like the score
// HUD's by-deal list; open only once the person unfolds it. Nothing is saved.
export const trackerOpen = ({ unfolded }) => unfolded === true;

// A card as the grid and the lines write it: a suit's glyph and rank (K Q C J
// or K D C B), a tarock's Roman numeral, "Sküs".
export function cardLabel(code, { t, lang }) {
  const c = parseCode(code);
  if (c.kind === "sku") return t("card.sk");
  if (c.kind === "tarock") return tarockNumeral(c.n);
  const rank = ["K", "Q", "C", "J"].includes(c.rank) ? courtLabels(lang)[c.rank] : c.rank;
  return `${GLYPH[c.suit]}${rank}`;
}

// The cell's own label: no glyph where the row has it already.
function cellLabel(code, words) {
  const c = parseCode(code);
  return c.kind === "suit" ? cardLabel(code, words).slice(GLYPH[c.suit].length) : cardLabel(code, words);
}

const lower = (lang, s) => (lang === "en" ? s.toLowerCase() : s);
const joined = (list, { t }) => (list.length < 2 ? list.join("") : `${list.slice(0, -1).join(", ")} ${t("tracker.or")} ${list.at(-1)}`);

// `words`: { t, lang, cardName, romanName, thirds }: `romanName` is for a cell's
// title (the faces' numerals), `cardName` for the screen reader. `p` is the engine's `tracker` object
// (`strohmandeln_tracker().tracker`), or null: no deal, no panel.
export function trackerModel(p, words) {
  if (!p) return null;
  const { t, lang, cardName, romanName, thirds } = words;
  const group = (kind) => p.groups.find((g) => g.kind === kind);
  const cell = (code) => {
    const place = p.cards[code];
    const played = place.place === "played";
    const state = played ? (place.by === p.human ? "played" : "theirs") : place.place === "hand" || place.place === "cover" ? "mine" : "out";
    const how = played ? t(`tracker.state.${place.by === p.human ? "you" : "opp"}`, { n: place.trick }) : t(`tracker.state.${place.place}`);
    return { code, label: cellLabel(code, words), state, title: `${romanName(code)}: ${how}` };
  };
  const rows = [
    ...SUITS.map((s) => ({
      id: s,
      mark: GLYPH[s],
      name: t(`card.suit.${s}`),
      cells: RANKS[s === "h" || s === "d" ? "red" : "black"].map((r) => cell(r + s)),
    })),
  ];
  let at = 0;
  TAROCK_ROWS.forEach((n, i) => {
    rows.push({ id: `t${i + 1}`, mark: i === 0 ? "★" : "", name: t("card.suit.trump"), cells: TAROCKS_HIGH_TO_LOW.slice(at, at + n).map(cell) });
    at += n;
  });

  const lines = [];
  const trumps = group("trump");
  if (trumps.out.length === 0) lines.push({ id: "trumps", text: t("tracker.line.trumps.none") });
  else {
    const named = trumps.out.slice(0, NAMED).map((c) => cardLabel(c, words));
    const list = `${named.join(", ")}${trumps.out.length > NAMED ? "…" : ""}`;
    lines.push({ id: "trumps", text: t("tracker.line.trumps", { n: trumps.out.length, list }) });
  }
  // Masters: the tarock first, then the suits.
  // A suit master is rank only: he may trump it when he has shown the suit void
  // and a tarock is still out. A tarock master is never trumped.
  const ruffable = (g) => g.kind !== "trump" && g.void && trumps.out.length > 0;
  const masters = ["trump", ...SUITS].map(group).filter((g) => g.master);
  if (masters.length === 1) {
    const g = masters[0];
    const key = ruffable(g) ? "tracker.line.master.ruff" : "tracker.line.master";
    lines.push({ id: "masters", text: t(key, { card: cardLabel(g.best, words), kind: t(`tracker.kind.${g.kind}`) }) });
  } else if (masters.length > 1) {
    const cards = masters.map((g) => `${cardLabel(g.best, words)}${ruffable(g) ? ` (${t("tracker.trumpable")})` : ""}`);
    lines.push({ id: "masters", text: t("tracker.line.masters", { cards: cards.join(", ") }) });
  }
  const voids = KINDS.map(group).filter((g) => g.void);
  if (voids.length) {
    const kinds = voids.map((g) => lower(lang, t(`card.suit.${g.kind}`)));
    lines.push({ id: "voids", text: t("tracker.line.void", { kinds: joined(kinds, words) }) });
  }
  lines.push({ id: "points", text: t("tracker.line.points", { points: thirds(p.out_thirds) }) });

  const counted = rows.map((r) => ({ ...r, counts: counts(r.cells) }));
  // A kind's counts for the screen reader: its rows summed (the tarocks have three).
  const kindCounts = (kind) => {
    const of = counted.filter((r) => (r.id.startsWith("t") ? "trump" : r.id) === kind).map((r) => r.counts);
    return Object.fromEntries(COUNTS.map((id) => [id, of.reduce((n, c) => n + c[id], 0)]));
  };
  const names = (codes) => (codes.length ? codes.map(cardName).join(", ") : t("tracker.sr.none"));
  return {
    title: t("tracker.title"),
    byline: t("tracker.byline"),
    lines,
    rows: counted,
    legend: STATES.map((state) => ({ state, text: t(`tracker.legend.${state}`) })),
    sr: KINDS.map(group).map((g) => t("tracker.sr.group", { kind: t(`card.suit.${g.kind}`), mine: names(g.mine), out: names(g.out), ...kindCounts(g.kind) })),
    countHeads: COUNTS.map((id) => ({ id, text: t(`tracker.count.${id}`), title: t(`tracker.count.${id}.title`) })),
    trick: p.trick,
  };
}
