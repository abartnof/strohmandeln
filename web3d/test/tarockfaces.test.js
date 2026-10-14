// The Tarock faces: the 54 codes, the tarock numerals, the index fitted to
// its strip, the nicknames, the pips, the contrast.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import {
  ALL_CODES, NICKNAMES, TAROCK, contrast, drawBack, drawFace, fitText, nickname, parseCode, pipLayout, tarockNumeral,
} from "../src/tarockfaces.js";
import { JUMBO } from "../../kit/js/faces.js";

// A recording 2D context; text is 0.62 em per character wide, 0.72 em tall.
function recorder() {
  const calls = [];
  let font = "";
  const ctx = {
    calls,
    fillStyle: "",
    strokeStyle: "",
    lineWidth: 1,
    textAlign: "",
    textBaseline: "",
    set font(f) { font = f; },
    get font() { return font; },
    measureText(text) {
      const size = Number(font.match(/([\d.]+)px/)[1]);
      const n = [...text.replace(/︎/g, "")].length;
      return { width: 0.62 * n * size, actualBoundingBoxAscent: 0.72 * size, actualBoundingBoxDescent: 0, actualBoundingBoxLeft: 0, actualBoundingBoxRight: 0.62 * n * size };
    },
  };
  for (const name of ["fillRect", "save", "restore", "translate", "scale", "rotate", "beginPath", "moveTo", "lineTo", "closePath", "fill", "stroke", "arc", "strokeRect"]) {
    ctx[name] = (...a) => calls.push([name, ...a]);
  }
  // fillText records the colour and font in force.
  ctx.fillText = (text, x, y) => calls.push(["fillText", text, x, y, font, ctx.fillStyle]);
  ctx.fillRect = (...a) => calls.push(["fillRect", ...a, ctx.fillStyle]);
  return ctx;
}

const isTarock = (c) => c === "SK" || /^T\d/.test(c);
const rules = readFileSync(new URL("../../docs/RULES.md", import.meta.url), "utf8");
const sizeOf = (call) => Number(call[4].match(/([\d.]+)px/)[1]);
const textsOf = (ctx) => ctx.calls.filter((c) => c[0] === "fillText");

test("the 54 codes are RULES.md 1.2: T1..T21, SK, and 32 suit cards", () => {
  assert.equal(ALL_CODES.length, 54);
  assert.equal(new Set(ALL_CODES).size, 54);
  assert.ok(rules.includes("`T1` ... `T21`") && rules.includes("`SK`"));
  const suits = ALL_CODES.filter((c) => !isTarock(c));
  assert.equal(suits.length, 32);
  for (const s of "cs") for (const r of ["K", "Q", "C", "J", "10", "9", "8", "7"]) assert.ok(suits.includes(r + s), r + s);
  for (const s of "hd") for (const r of ["K", "Q", "C", "J", "1", "2", "3", "4"]) assert.ok(suits.includes(r + s), r + s);
});

test("every code produces a face: a field from edge to edge, then ink", () => {
  for (const code of ALL_CODES) {
    const ctx = recorder();
    drawFace(ctx, 700, code);
    const [first] = ctx.calls;
    assert.equal(first[0], "fillRect", code);
    assert.deepEqual(first.slice(1, 4), [0, 0, 700], code);
    assert.ok(Math.abs(first[4] - 980) < 1e-9, code);
    assert.ok(textsOf(ctx).length >= 2, code);
  }
});

test("a malformed code is refused", () => {
  for (const bad of ["", "T0", "T22", "Th", "1s", "Kx", "SKs", "10h", "5c"]) assert.throws(() => parseCode(bad), bad);
});

test("tarock numerals: only the 4 is additive", () => {
  const want = {
    1: "I", 2: "II", 3: "III", 4: "IIII", 5: "V", 6: "VI", 7: "VII", 8: "VIII", 9: "IX", 10: "X", 11: "XI",
    12: "XII", 13: "XIII", 14: "XIV", 15: "XV", 16: "XVI", 17: "XVII", 18: "XVIII", 19: "XIX", 20: "XX", 21: "XXI",
  };
  for (const [n, s] of Object.entries(want)) assert.equal(tarockNumeral(Number(n)), s, n);
  assert.throws(() => tarockNumeral(0));
  assert.throws(() => tarockNumeral(22));
});

test("nicknames only on I, II, III, IIII and XXI", () => {
  assert.deepEqual(NICKNAMES, { 1: "PAGAT", 2: "UHU", 3: "KAKADU", 4: "QUAPIL", 21: "MOND" });
  for (const code of ALL_CODES) {
    const want = { T1: "PAGAT", T2: "UHU", T3: "KAKADU", T4: "QUAPIL", T21: "MOND" }[code] ?? null;
    assert.equal(nickname(code), want, code);
    const ctx = recorder();
    drawFace(ctx, 700, code);
    const texts = textsOf(ctx).map((c) => c[1]);
    for (const nick of Object.values(NICKNAMES)) assert.equal(texts.includes(nick), nick === want, `${code} ${nick}`);
  }
});

test("the Sküs has no numeral: a star and SKÜS", () => {
  const ctx = recorder();
  drawFace(ctx, 700, "SK");
  const texts = textsOf(ctx).map((c) => c[1]);
  assert.ok(texts.includes("SKÜS"));
  assert.ok(!texts.some((t) => /^[IVX]+$/.test(t)));
  assert.ok(ctx.calls.some((c) => c[0] === "fill"), "the star is a path");
});

const spec = (room) => ({ room, maxSize: 2.2, minCondense: 0.6, font: (px) => `700 ${px}px x` });

test("fitText: never wider than the room, condensed before it shrinks", () => {
  const ctx = recorder();
  for (const room of [2.4, 3.5]) {
    for (const text of ["I", "XVIII", "XXI", "10", "KAKADU", "QUAPIL", "SKÜS"]) {
      const f = fitText(ctx, text, spec(room));
      assert.ok(f.ink * f.size * f.scaleX <= room + 1e-9, `${text} in ${room}: ${f.ink * f.size * f.scaleX}`);
      assert.ok(f.scaleX >= 0.6 - 1e-9 && f.scaleX <= 1 && f.size <= 2.2 + 1e-9);
    }
  }
  const short = fitText(ctx, "I", spec(2.4));
  assert.deepEqual([short.size, short.scaleX], [2.2, 1]);
  const mid = fitText(ctx, "XVIII", spec(2.4));
  assert.ok(mid.scaleX < 1 && mid.scaleX >= 0.6, "XVIII is condensed...");
  assert.ok(mid.size < 2.2 && mid.size > 1.2, "...and then made a little smaller, but not small");
  const quick = fitText(ctx, "XX", spec(2.4));
  assert.ok(quick.size === 2.2 && quick.scaleX < 1, "XX is only condensed");
});

// Every fillText that lands in an index strip (left of JUMBO.strip, or the
// same width at the right, turned), walking the face's transforms.
test("the index text of every card stays inside its strip, with the margin", () => {
  const width = 700;
  const u = width / JUMBO.w;
  for (const code of ALL_CODES) {
    const ctx = recorder();
    drawFace(ctx, width, code);
    const m = [{ tx: 0, ty: 0, sx: 1, sy: 1 }];
    const found = [];
    for (const c of ctx.calls) {
      const t = m[m.length - 1];
      if (c[0] === "save") m.push({ ...t });
      else if (c[0] === "restore") m.pop();
      else if (c[0] === "translate") { t.tx += c[1] * t.sx; t.ty += c[2] * t.sy; }
      else if (c[0] === "scale") { t.sx *= c[1]; t.sy *= c[2]; }
      else if (c[0] === "rotate") { t.sx *= -1; t.sy *= -1; }
      else if (c[0] === "fillText") {
        const w = 0.62 * [...c[1].replace(/︎/g, "")].length * sizeOf(c) * Math.abs(t.sx);
        const x0 = t.tx + c[2] * t.sx;
        const lo = Math.min(x0, x0 + (t.sx < 0 ? -w : w)) / u;
        const hi = Math.max(x0, x0 + (t.sx < 0 ? -w : w)) / u;
        found.push([c[1], lo, hi, t.sx < 0]);
      }
    }
    let inStrips = 0;
    for (const [text, lo, hi, turned] of found) {
      const inLeft = hi <= JUMBO.strip + 1e-6;
      const inRight = turned && lo >= JUMBO.w - JUMBO.strip - 1e-6;
      if (!inLeft && !inRight) continue;
      inStrips++;
      const okLeft = inLeft && lo >= JUMBO.margin - 1e-6 && hi <= JUMBO.strip - JUMBO.margin + 1e-6;
      const okRight = inRight && lo >= JUMBO.w - JUMBO.strip + JUMBO.margin - 1e-6 && hi <= JUMBO.w - JUMBO.margin + 1e-6;
      assert.ok(okLeft || okRight, `${code} "${text}" spans ${lo.toFixed(2)}..${hi.toFixed(2)}`);
    }
    assert.ok(inStrips > 0, code);
  }
});

test("tarocks are turned at the bottom right, suit cards are not", () => {
  for (const code of ALL_CODES) {
    const ctx = recorder();
    drawFace(ctx, 700, code);
    assert.equal(ctx.calls.some((c) => c[0] === "rotate"), isTarock(code), code);
  }
});

test("tarocks share one tinted field, unlike every suit card", () => {
  const fieldOf = (code) => {
    const ctx = recorder();
    drawFace(ctx, 700, code);
    return ctx.calls[0][5];
  };
  assert.deepEqual([...new Set(ALL_CODES.filter(isTarock).map(fieldOf))], [TAROCK.field]);
  assert.deepEqual([...new Set(ALL_CODES.filter((c) => !isTarock(c)).map(fieldOf))], [JUMBO.paper]);
  assert.notEqual(TAROCK.field, JUMBO.paper);
});

test("pips: as many symbols as the number, inside the card, none on another", () => {
  for (const [rank, n] of [["1", 1], ["2", 2], ["3", 3], ["4", 4], ["7", 7], ["8", 8], ["9", 9], ["10", 10]]) {
    const pips = pipLayout(rank);
    assert.equal(pips.length, n, rank);
    for (const p of pips) {
      assert.ok(p.x - p.size / 2 >= JUMBO.strip && p.x + p.size / 2 <= JUMBO.w, rank);
      assert.ok(p.y - p.size / 2 >= 0 && p.y + p.size / 2 <= JUMBO.h, rank);
    }
    for (let i = 0; i < n; i++) {
      for (let j = i + 1; j < n; j++) {
        const a = pips[i], b = pips[j];
        const reach = ((a.size + b.size) / 2) * 0.85;
        assert.ok(Math.abs(a.x - b.x) >= reach || Math.abs(a.y - b.y) >= reach, `${rank}: pips ${i} and ${j} touch`);
      }
    }
  }
  assert.throws(() => pipLayout("K"));
  // The index's suit and one glyph for each pip.
  for (const [code, n] of [["7s", 7], ["10c", 10], ["3h", 3], ["4d", 4], ["1h", 1]]) {
    const ctx = recorder();
    drawFace(ctx, 700, code);
    assert.equal(textsOf(ctx).filter((c) => /[♠♥♦♣]/.test(c[1])).length, n + 1, code);
  }
});

test("courts: a large rank letter and the suit, no pips", () => {
  for (const r of ["K", "Q", "C", "J"]) {
    const ctx = recorder();
    drawFace(ctx, 700, r + "h");
    const t = textsOf(ctx);
    assert.equal(t.length, 4, r);
    assert.deepEqual(t.map((c) => c[1]).sort(), [r, r, "♥︎", "♥︎"].sort());
    const big = t.filter((c) => c[1] === r).map(sizeOf);
    assert.ok(Math.max(...big) > Math.min(...big) * 1.5, "the central letter is much larger than the index's");
  }
});

test("suit colours: hearts and diamonds red, clubs and spades ink", () => {
  for (const [code, colour] of [["Kh", JUMBO.red], ["Kd", JUMBO.red], ["Kc", JUMBO.ink], ["Ks", JUMBO.ink], ["4h", JUMBO.red], ["10s", JUMBO.ink]]) {
    const ctx = recorder();
    drawFace(ctx, 700, code);
    for (const c of textsOf(ctx)) assert.equal(c[5], colour, `${code} ${c[1]}`);
  }
});

test("contrast: all text passes WCAG AA (4.5:1) on its ground", () => {
  for (const g of [JUMBO.paper, TAROCK.field, TAROCK.band]) assert.ok(contrast(JUMBO.ink, g) >= 4.5, `ink on ${g}`);
  assert.ok(contrast(JUMBO.red, JUMBO.paper) >= 4.5, "red on paper");
  const bw = contrast("#000000", "#ffffff");
  assert.ok(bw > 20.9 && bw < 21.1);
});

test("the back is a plain pattern, drawn from edge to edge, with no text", () => {
  const ctx = recorder();
  drawBack(ctx, 700);
  assert.deepEqual(ctx.calls[0].slice(0, 4), ["fillRect", 0, 0, 700]);
  assert.equal(textsOf(ctx).length, 0);
  assert.ok(ctx.calls.length > 10);
});
