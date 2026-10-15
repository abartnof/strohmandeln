// The faces of the 54-card Tarock pack, and a back, drawn with Canvas 2D in
// the manner of the kit's Large Text (jumbo index) faces (kit/js/faces.js):
// no figure art, a large index fitted to the strip a fanned hand shows
// (JUMBO.strip, with JUMBO.margin clear either side), nothing bundled or
// fetched.
//
// Suit cards: the rank over the suit in the corner index; courts (K Q C J)
// and, in Large Text, every other card too, a large rank over the suit; in
// Classic the others show pips (10 9 8 7 in clubs and spades, 1 2 3 4 in
// hearts and diamonds), that many suit symbols.
// Tarocks: a tinted field (TAROCK.field) with a darker band down the strip;
// the Roman numeral in the strip at the top left and, turned, at the bottom
// right, as on the real pack, and large in the middle. Only the 4 is
// additive (IIII); IX, XIV, XIX are subtractive (research/12-followups.md,
// item 9). I, II, III, IIII and XXI carry their nicknames. The Sküs has a
// star and "SKÜS" instead of a numeral.
//
// Colour: suits differ by shape as well as by colour (heart, diamond, club,
// spade), so a colour-blind player loses nothing. Text colours pass WCAG AA
// (4.5:1) on their grounds; the test measures it.

import { CanvasTexture } from "three";
import { cardTexture } from "../../kit/js/materials.js";
import { JUMBO } from "../../kit/js/faces.js";
import { tarockNumeral } from "./strings.js";

// ---- Codes (docs/RULES.md 1.2) --------------------------------------------

const BLACK_RANKS = ["K", "Q", "C", "J", "10", "9", "8", "7"];
const RED_RANKS = ["K", "Q", "C", "J", "1", "2", "3", "4"];

export const ALL_CODES = Object.freeze([
  ...Array.from({ length: 21 }, (_, i) => `T${i + 1}`),
  "SK",
  ...["h", "d"].flatMap((s) => RED_RANKS.map((r) => r + s)),
  ...["c", "s"].flatMap((s) => BLACK_RANKS.map((r) => r + s)),
]);

// A code as { kind: "tarock", n } | { kind: "sku" } | { kind: "suit", rank, suit }.
export function parseCode(code) {
  if (code === "SK") return { kind: "sku" };
  const t = /^T([1-9]|1\d|2[01])$/.exec(code);
  if (t) return { kind: "tarock", n: Number(t[1]) };
  const m = /^(K|Q|C|J|10|[1-9])([hdcs])$/.exec(code);
  if (m) {
    const [, rank, suit] = m;
    if ((suit === "h" || suit === "d" ? RED_RANKS : BLACK_RANKS).includes(rank)) return { kind: "suit", rank, suit };
  }
  throw new Error(`not a card code: ${JSON.stringify(code)}`);
}

// ---- Court labels, by language --------------------------------------------------

// The letter a court shows: the code's K Q C J are the king, queen, cavalier
// (horseman) and jack; German packs letter them K D C B (Dame, Bube).
export const COURT_LABELS = Object.freeze({
  en: Object.freeze({ K: "K", Q: "Q", C: "C", J: "J" }),
  de: Object.freeze({ K: "K", Q: "D", C: "C", J: "B" }),
});

// The four court letters of a language ("en" or "de"), or of a table of the
// four codes (a game with another language passes its own).
export function courtLabels(lang = "en") {
  const labels = typeof lang === "string" ? COURT_LABELS[lang] : lang;
  if (!labels || !["K", "Q", "C", "J"].every((k) => typeof labels[k] === "string" && labels[k].length > 0)) {
    throw new RangeError(`no court labels for ${JSON.stringify(lang)}`);
  }
  return labels;
}

// The 16 courts' codes, the faces to draw again when the language changes.
export const COURT_CODES = Object.freeze(ALL_CODES.filter((c) => /^[KQCJ][hdcs]$/.test(c)));

// ---- Roman numerals, as on the real pack -----------------------------------

// (tarockNumeral lives in strings.js, which names cards in prose; re-exported here.)
export { tarockNumeral };

export const NICKNAMES = Object.freeze({ 1: "PAGAT", 2: "UHU", 3: "KAKADU", 4: "QUAPIL", 21: "MOND" });

export function nickname(code) {
  const c = parseCode(code);
  return c.kind === "tarock" ? (NICKNAMES[c.n] ?? null) : null;
}

// ---- Colour -----------------------------------------------------------------

export const TAROCK = Object.freeze({
  field: "#ece0b8", // the tinted field of every tarock
  band: "#d9c78a", // the darker band down the strip
  rule: "#8a6d1d",
  star: "#15171c",
});

const COLOUR = { s: JUMBO.ink, c: JUMBO.ink, h: JUMBO.red, d: JUMBO.red };

const lum = (hex) => {
  const v = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16) / 255).map((c) => (c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4));
  return 0.2126 * v[0] + 0.7152 * v[1] + 0.0722 * v[2];
};
// WCAG contrast ratio of two "#rrggbb" colours.
export function contrast(a, b) {
  const [hi, lo] = [lum(a), lum(b)].sort((x, y) => y - x);
  return (hi + 0.05) / (lo + 0.05);
}

// ---- Fonts -------------------------------------------------------------------

const RANK_FONT = "-apple-system, system-ui, 'Helvetica Neue', Arial, sans-serif";
const SUIT_FONT = '"Apple Symbols", "Segoe UI Symbol", "Noto Sans Symbols 2", "DejaVu Sans", system-ui, sans-serif';
const GLYPH = { s: "♠︎", h: "♥︎", d: "♦︎", c: "♣︎" };
const rankFont = (px) => `700 ${px}px ${RANK_FONT}`;
const suitFont = (px) => `${px}px ${SUIT_FONT}`;

// ---- Fitting -----------------------------------------------------------------

const inkOf = (m) => m.actualBoundingBoxLeft + m.actualBoundingBoxRight;

// The size and horizontal squeeze at which `text` fits `room` units: at most
// `maxSize`; if wider, first condensed (scaleX) down to `minCondense`, then
// made smaller. `ink` is the text's ink width per unit of size; the text's
// width in units is ink * size * scaleX.
export function fitText(ctx, text, { room, maxSize, minCondense = 1, font }) {
  ctx.font = font(100);
  const ink = inkOf(ctx.measureText(text)) / 100;
  const full = ink * maxSize;
  if (full <= room) return { size: maxSize, scaleX: 1, ink };
  const f = room / full;
  if (f >= minCondense) return { size: maxSize, scaleX: f, ink };
  return { size: maxSize * (f / minCondense), scaleX: minCondense, ink };
}

const CAP = (ctx, font, size) => {
  ctx.font = font(100);
  return (ctx.measureText("H").actualBoundingBoxAscent * size) / 100;
};

// How far the rank's ink dips below its baseline (a J, in some fonts).
function dip(ctx, text, size) {
  ctx.font = rankFont(100);
  return ((ctx.measureText(text).actualBoundingBoxDescent ?? 0) * size) / 100;
}

// Text centred on x (units), its capitals' baseline at y, in the fit.
function put(ctx, u, text, { x, y, size, scaleX, font, colour }) {
  ctx.fillStyle = colour;
  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "left";
  ctx.font = font(size * u);
  const m = ctx.measureText(text);
  const centre = (m.actualBoundingBoxRight - m.actualBoundingBoxLeft) / 2;
  ctx.save();
  ctx.translate(x * u, y * u);
  ctx.scale(scaleX, 1);
  ctx.fillText(text, -centre, 0);
  ctx.restore();
}

// A suit glyph centred on (x, y) by its measured ink, `size` units.
function glyph(ctx, u, suit, { x, y, size, colour }) {
  ctx.fillStyle = colour;
  ctx.textBaseline = "alphabetic";
  ctx.textAlign = "left";
  ctx.font = suitFont(size * u);
  const m = ctx.measureText(GLYPH[suit]);
  const cx = (m.actualBoundingBoxRight - m.actualBoundingBoxLeft) / 2;
  const cy = (m.actualBoundingBoxAscent - (m.actualBoundingBoxDescent ?? 0)) / 2;
  ctx.fillText(GLYPH[suit], x * u - cx, y * u + cy);
}

// The size at which every suit's ink is `room` wide or less, at most `max`.
function suitSize(ctx, room, max) {
  ctx.font = suitFont(100);
  const w = Math.max(...Object.values(GLYPH).map((g) => inkOf(ctx.measureText(g)))) / 100;
  return Math.min(max, room / w);
}
// The tallest suit's ink, per unit of size.
function suitInkHeight(ctx) {
  ctx.font = suitFont(100);
  return Math.max(...Object.values(GLYPH).map((g) => ctx.measureText(g).actualBoundingBoxAscent + (ctx.measureText(g).actualBoundingBoxDescent ?? 0))) / 100;
}

// Faces size (7.1, 7.6): "jumbo" is the full index in the strip; "classic" draws
// it smaller, in the same strip. drawFace sets this for the face it draws.
export const INDEX_SCALE = Object.freeze({ jumbo: 1, classic: 0.78 });
let indexScale = INDEX_SCALE.jumbo;

const ROOM = JUMBO.strip - 2 * JUMBO.margin; // the strip's room for ink
const AXIS = JUMBO.margin + ROOM / 2; // the strip's centre line
const BODY_LEFT = JUMBO.strip + 0.3; // the body is right of the strip...
const BODY_RIGHT = JUMBO.w - 0.3;
const BODY_W = BODY_RIGHT - BODY_LEFT;
const BODY_X = (BODY_LEFT + BODY_RIGHT) / 2;
const MID = 3.9; // the middle's centre line, above the turned index at the bottom right

// ---- The star of the Sküs -----------------------------------------------------

function star(ctx, u, cx, cy, r, colour) {
  ctx.fillStyle = colour;
  ctx.beginPath();
  for (let i = 0; i < 10; i++) {
    const a = -Math.PI / 2 + (i * Math.PI) / 5;
    const rad = i % 2 ? r * 0.42 : r;
    const x = (cx + Math.cos(a) * rad) * u;
    const y = (cy + Math.sin(a) * rad) * u;
    if (i) ctx.lineTo(x, y);
    else ctx.moveTo(x, y);
  }
  ctx.closePath();
  ctx.fill();
}

// ---- Pips ----------------------------------------------------------------------

// The pip columns are symmetric about the body's centre line (BODY_X, the room
// right of the index strip), so a face reads as centred in the room it has.
const COL = { l: BODY_X - 1.1, c: BODY_X, r: BODY_X + 1.1 };
const TOP = 1.2;
const BOTTOM = JUMBO.h - 1.2;
const at = (f) => TOP + (BOTTOM - TOP) * f;
// What a glyph's ink fills of its font size (the widest of the four suits, at
// 0.8 with margin), for keeping pips inside the room.
export const PIP_INK = 0.8;

// The pips of a rank "1".."4", "7".."10": { x, y, size } in card units, `size`
// being the font size they are drawn at.
export function pipLayout(rank) {
  const col = (c, ys, size) => ys.map((f) => ({ x: COL[c], y: at(f), size }));
  const S = 1.7;
  switch (rank) {
    case "1": return [{ x: COL.c, y: JUMBO.h / 2, size: 3.75 }];
    case "2": return col("c", [0.12, 0.88], 2.4);
    case "3": return col("c", [0.05, 0.5, 0.95], 2.4);
    case "4": return [...col("l", [0.12, 0.88], 1.9), ...col("r", [0.12, 0.88], 1.9)];
    case "7": return [...col("l", [0, 0.5, 1], S), ...col("r", [0, 0.5, 1], S), ...col("c", [0.25], S)];
    case "8": return [...col("l", [0, 0.5, 1], S), ...col("r", [0, 0.5, 1], S), ...col("c", [0.25, 0.75], S)];
    case "9": return [...col("l", [0, 1 / 3, 2 / 3, 1], S), ...col("r", [0, 1 / 3, 2 / 3, 1], S), ...col("c", [0.5], S)];
    case "10": return [...col("l", [0, 1 / 3, 2 / 3, 1], S), ...col("r", [0, 1 / 3, 2 / 3, 1], S), ...col("c", [1 / 6, 5 / 6], S)];
    default: throw new RangeError(`no pips for rank ${rank}`);
  }
}

// ---- The faces -------------------------------------------------------------------

// The index in the strip: top lines, each [text, kind], from the card's top.
function stripFits(ctx) {
  return {
    numeral: { room: ROOM, maxSize: 2.2 * indexScale, minCondense: 0.62, font: rankFont },
    nick: { room: ROOM, maxSize: 0.8 * indexScale, minCondense: 0.75, font: rankFont },
    big: { room: BODY_W, maxSize: 4.4, minCondense: 0.62, font: rankFont },
    ctx,
  };
}

// A tarock's index (numeral and, if any, nickname), in the strip at the top
// left of whatever transform is in force.
function tarockIndex(ctx, u, text, nick, colour) {
  const f = stripFits(ctx);
  const n = fitText(ctx, text, f.numeral);
  const base = JUMBO.rankTop + CAP(ctx, rankFont, n.size);
  put(ctx, u, text, { x: AXIS, y: base, size: n.size, scaleX: n.scaleX, font: rankFont, colour });
  if (nick) {
    const k = fitText(ctx, nick, f.nick);
    const kb = base + 0.3 + CAP(ctx, rankFont, k.size);
    put(ctx, u, nick, { x: AXIS, y: kb, size: k.size, scaleX: k.scaleX, font: rankFont, colour });
  }
}

function tarockFace(ctx, u, { text, nick, sku }) {
  const ink = JUMBO.ink;
  ctx.fillStyle = TAROCK.field;
  ctx.fillRect(0, 0, JUMBO.w * u, JUMBO.h * u);
  ctx.fillStyle = TAROCK.band; // the band down the strip, the one a fanned hand shows
  ctx.fillRect(0, 0, JUMBO.strip * u, JUMBO.h * u);
  ctx.fillStyle = TAROCK.rule;
  ctx.fillRect(JUMBO.strip * u, 0, 0.08 * u, JUMBO.h * u);

  const index = () => {
    if (sku) {
      star(ctx, u, AXIS, JUMBO.rankTop + 1.15, 1.2, TAROCK.star);
      const k = fitText(ctx, "SKÜS", stripFits(ctx).nick);
      put(ctx, u, "SKÜS", { x: AXIS, y: JUMBO.rankTop + 2.4 + CAP(ctx, rankFont, k.size) + 0.1, size: k.size, scaleX: k.scaleX, font: rankFont, colour: ink });
    } else tarockIndex(ctx, u, text, nick, ink);
  };
  index();
  // Turned, at the bottom right; the same strip width in from that edge.
  ctx.save();
  ctx.translate(JUMBO.w * u, JUMBO.h * u);
  ctx.rotate(Math.PI);
  index();
  ctx.restore();

  // The middle, between the strips' reach: the numeral large.
  const f = stripFits(ctx);
  if (sku) {
    star(ctx, u, BODY_X, MID, 1.9, TAROCK.star);
    return;
  }
  // (The nickname stays in the index: below the middle the turned index lives.)
  const n = fitText(ctx, text, f.big);
  const cap = CAP(ctx, rankFont, n.size);
  put(ctx, u, text, { x: BODY_X, y: MID + cap / 2, size: n.size, scaleX: n.scaleX, font: rankFont, colour: ink });
}

function suitFace(ctx, u, { rank: code, suit }, labels) {
  const rank = labels[code] ?? code; // the letter shown; `code` decides the layout
  ctx.fillStyle = JUMBO.paper;
  ctx.fillRect(0, 0, JUMBO.w * u, JUMBO.h * u);
  const colour = COLOUR[suit];

  // The corner: the rank's capitals from rankTop, the suit below, on the axis.
  const r = fitText(ctx, rank, { room: ROOM, maxSize: 2.2 * indexScale, minCondense: 0.62, font: rankFont });
  const base = JUMBO.rankTop + CAP(ctx, rankFont, r.size);
  put(ctx, u, rank, { x: AXIS, y: base, size: r.size, scaleX: r.scaleX, font: rankFont, colour });
  const s = suitSize(ctx, ROOM, 2.8 * indexScale);
  const ink = suitInkHeight(ctx) * s;
  glyph(ctx, u, suit, { x: AXIS, y: base + dip(ctx, rank, r.size) + JUMBO.gap + ink / 2, size: s, colour });

  if (!/^\d/.test(code) || indexScale === INDEX_SCALE.jumbo) {
    // A court, or in Large Text any card: the rank large, the suit large under
    // it, right of the strip, as Cassino's (a 10 is condensed to the room).
    const f = fitText(ctx, rank, { room: BODY_W, maxSize: 5, minCondense: 0.62, font: rankFont });
    const cap = CAP(ctx, rankFont, f.size);
    const sz = suitSize(ctx, BODY_W, 3.4);
    const sInk = suitInkHeight(ctx) * sz;
    const gap = 0.6 + dip(ctx, rank, f.size);
    const top = (JUMBO.h - (cap + gap + sInk)) / 2;
    put(ctx, u, rank, { x: BODY_X, y: top + cap, size: f.size, scaleX: f.scaleX, font: rankFont, colour });
    glyph(ctx, u, suit, { x: BODY_X, y: top + cap + gap + sInk / 2, size: sz, colour });
    return;
  }
  for (const p of pipLayout(code)) glyph(ctx, u, suit, { x: p.x, y: p.y, size: p.size, colour });
}

// The face of `code` ("T7", "SK", "Kh", "10c", "3d") on a 2D context `width`
// pixels wide and 1.4 times as tall. `lang` ("en", "de" or a table of K Q C J
// letters) says how the courts are lettered.
export function drawFace(ctx, width, code, lang = "en", size = "jumbo") {
  indexScale = INDEX_SCALE[size] ?? INDEX_SCALE.jumbo;
  const c = parseCode(code);
  const labels = courtLabels(lang);
  const u = width / JUMBO.w;
  if (c.kind === "suit") suitFace(ctx, u, c, labels);
  else if (c.kind === "sku") tarockFace(ctx, u, { sku: true });
  else tarockFace(ctx, u, { text: tarockNumeral(c.n), nick: NICKNAMES[c.n] ?? null });
}

// ---- The back --------------------------------------------------------------------

export const BACK = Object.freeze({ ground: "#27424a", line: "#3f6470", edge: "#9fbac2" });

// A quiet lattice of fine diagonals on a deep teal, in a thin frame: nothing
// a face has (no text, no tint of paper, no strip).
export function drawBack(ctx, width) {
  const u = width / JUMBO.w;
  const w = JUMBO.w * u;
  const h = JUMBO.h * u;
  ctx.fillStyle = BACK.ground;
  ctx.fillRect(0, 0, w, h);
  ctx.save();
  ctx.strokeStyle = BACK.line;
  ctx.lineWidth = Math.max(1, 0.07 * u);
  const step = 0.7 * u;
  ctx.beginPath();
  for (let k = -h; k < w; k += step) {
    ctx.moveTo(k, 0);
    ctx.lineTo(k + h, h);
    ctx.moveTo(k + h, 0);
    ctx.lineTo(k, h);
  }
  ctx.stroke();
  ctx.restore();
  ctx.strokeStyle = BACK.edge;
  ctx.lineWidth = Math.max(1, 0.1 * u);
  ctx.strokeRect(0.45 * u, 0.45 * u, w - 0.9 * u, h - 0.9 * u);
}

// ---- Textures ----------------------------------------------------------------------

const canvasOf = (width) => {
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = Math.round((width * JUMBO.h) / JUMBO.w);
  return canvas;
};

// The faces as textures keyed by code, `width` pixels wide: all 54, or `codes`
// (COURT_CODES to redo only the courts when the language changes; dispose the
// textures they replace).
export function faceTextures({ width, anisotropy = 1, codes = ALL_CODES, lang = "en", size = "jumbo" }) {
  return Object.fromEntries(
    codes.map((code) => {
      const canvas = canvasOf(width);
      drawFace(canvas.getContext("2d"), width, code, lang, size);
      return [code, cardTexture(new CanvasTexture(canvas), anisotropy)];
    }),
  );
}

export function backTexture({ width, anisotropy = 1 }) {
  const canvas = canvasOf(width);
  drawBack(canvas.getContext("2d"), width);
  return cardTexture(new CanvasTexture(canvas), anisotropy);
}
