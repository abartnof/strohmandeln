#!/usr/bin/env node
// Renders the 54 faces and the back (web3d/src/tarockfaces.js) to a PNG contact
// sheet, with a fanned hand under them to show what a hand shows of each card.
// Usage: node tools/contact_sheet.mjs [out.png]   (default docs/img/faces.png)
// esbuild (web3d/node_modules) bundles the faces for the browser; Playwright's
// WebKit from ~/piquet/.venv (python) draws the canvases and takes the picture.
import { build } from "../web3d/node_modules/esbuild/lib/main.js";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve, dirname } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { homedir } from "node:os";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const out = resolve(process.argv[2] ?? join(root, "docs/img/faces.png"));
const dir = mkdtempSync(join(tmpdir(), "faces-"));

const entry = join(dir, "entry.js");
writeFileSync(
  entry,
  `import { ALL_CODES, drawFace, drawBack } from ${JSON.stringify(join(root, "web3d/src/tarockfaces.js"))};
import { JUMBO } from ${JSON.stringify(join(root, "kit/js/faces.js"))};
const W = 200, H = Math.round(W * JUMBO.h / JUMBO.w), GAP = 12;
const rows = [
  ALL_CODES.slice(0, 11), [...ALL_CODES.slice(11, 21), "SK"],
  [...ALL_CODES.slice(22, 30), "BACK"], ALL_CODES.slice(30, 38), ALL_CODES.slice(38, 46), ALL_CODES.slice(46, 54),
];
const hand = ["T21", "T18", "T14", "T9", "T4", "T1", "Kh", "3d", "Cc", "10s", "7c", "4h", "T17", "Js", "SK"];
const cols = 11, strip = Math.round(W * JUMBO.strip / JUMBO.w);
const sheetW = GAP + cols * (W + GAP), sheetH = GAP + rows.length * (H + GAP) + H + 2 * GAP;
const canvas = document.createElement("canvas");
canvas.width = sheetW; canvas.height = sheetH;
document.body.style.margin = "0";
document.body.appendChild(canvas);
const ctx = canvas.getContext("2d");
ctx.fillStyle = "#3a5a3a"; ctx.fillRect(0, 0, sheetW, sheetH);
const card = (code, x, y) => {
  const c = document.createElement("canvas"); c.width = W; c.height = H;
  const g = c.getContext("2d");
  if (code === "BACK") drawBack(g, W); else drawFace(g, W, code);
  ctx.save();
  ctx.beginPath(); ctx.roundRect(x, y, W, H, 10); ctx.clip();
  ctx.drawImage(c, x, y);
  ctx.restore();
  ctx.strokeStyle = "#101010"; ctx.lineWidth = 2; ctx.beginPath(); ctx.roundRect(x, y, W, H, 10); ctx.stroke();
};
rows.forEach((row, r) => row.forEach((code, i) => card(code, GAP + i * (W + GAP), GAP + r * (H + GAP))));
hand.forEach((code, i) => card(code, GAP + i * strip, GAP + rows.length * (H + GAP) + GAP));
window.done = true;
`,
);
const bundle = join(dir, "bundle.js");
await build({ entryPoints: [entry], bundle: true, format: "iife", outfile: bundle, nodePaths: [join(root, "web3d/node_modules")], logLevel: "warning" });
const page = join(dir, "sheet.html");
writeFileSync(page, `<!doctype html><meta charset="utf-8"><body><script src="bundle.js"></script></body>`);

const py = `
import sys
from playwright.sync_api import sync_playwright
page, out = sys.argv[1], sys.argv[2]
with sync_playwright() as p:
    b = p.webkit.launch()
    pg = b.new_page()
    pg.on("pageerror", lambda e: print("pageerror", e))
    pg.goto("file://" + page)
    pg.wait_for_function("window.done === true")
    pg.locator("canvas").screenshot(path=out)
    b.close()
`;
const r = spawnSync(join(homedir(), "piquet/.venv/bin/python"), ["-I", "-c", py, page, out], { stdio: "inherit" });
if (r.status !== 0) process.exit(r.status ?? 1);
console.log("wrote", out);
