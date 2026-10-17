// The page chrome's colours are cassino's Material 3 tokens (web3d/src/style.css),
// light and dark; every text/ground pair the chrome uses reaches WCAG AA (4.5:1).
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const css = readFileSync(new URL("../src/style.css", import.meta.url), "utf8");
const tokens = (block) => Object.fromEntries([...block.matchAll(/--([\w-]+):\s*(#[0-9a-f]{6})/gi)].map((m) => [m[1], m[2]]));
const light = tokens(css.slice(0, css.indexOf("@media (prefers-color-scheme: dark)")));
const dark = { ...light, ...tokens(css.slice(css.indexOf("@media (prefers-color-scheme: dark)"), css.indexOf("html, body"))) };

const lum = (hex) => {
  const c = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16) / 255).map((v) => (v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4));
  return 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
};
const ratio = (a, b) => {
  const [x, y] = [lum(a), lum(b)].sort((p, q) => q - p);
  return (x + 0.05) / (y + 0.05);
};

const M = "md-sys-color-";
// [text, ground]: the pairs the stylesheets set.
const PAIRS = [
  [M + "on-surface", M + "surface-container-high"], // dialog, slate, settings
  [M + "on-surface-variant", M + "surface-container-high"], // slate's dim text
  [M + "on-surface", M + "surface-container-lowest"], // prompt, speech box, tutor note
  [M + "on-surface", M + "surface-container-low"], // tabs
  ["cassino-you", M + "surface-container-high"], // slate: your column
  ["cassino-them", M + "surface-container-high"], // slate: your opponent's column
  [M + "on-primary", M + "primary"], // big.primary, announce chip
  [M + "on-secondary-container", M + "secondary-container"], // big, settings options and select
  [M + "on-primary-container", M + "primary-container"], // selected option, review chip
  [M + "on-tertiary-container", M + "tertiary-container"], // tutor buttons
  [M + "on-tertiary", M + "tertiary"], // won chip
  [M + "on-error", M + "error"], // lost chip
  [M + "on-error-container", M + "error-container"], // refusal
  ["cassino-on-you", "cassino-you"], // edge chip
  [M + "on-surface-variant", M + "surface-container-highest"], // muted tutor button
];

for (const [name, set] of [["light", light], ["dark", dark]]) {
  test(`${name}: the chrome's text reaches AA on its ground`, () => {
    for (const [fg, bg] of PAIRS) {
      assert.ok(set[fg] && set[bg], `${fg} / ${bg} are defined`);
      assert.ok(ratio(set[fg], set[bg]) >= 4.5, `${name} ${fg} on ${bg}: ${ratio(set[fg], set[bg]).toFixed(2)}`);
    }
  });
}

test("the table-top select is in the chrome's pair, 16px or more, touch-sized", () => {
  const table = readFileSync(new URL("../src/table.css", import.meta.url), "utf8");
  const rule = table.match(/\.settings-select\s*\{[^}]*\}/)[0];
  assert.match(rule, /color:\s*var\(--md-sys-color-on-secondary-container\)/);
  assert.match(rule, /background:\s*var\(--md-sys-color-secondary-container\)/);
  assert.match(rule, /font-size:\s*(1[6-9]|[2-9]\d)px/);
  assert.match(rule, /min-height:\s*var\(--target\)/);
  for (const set of [light, dark]) assert.ok(ratio(set[M + "on-secondary-container"], set[M + "secondary-container"]) >= 4.5);
});

test("the light tokens are cassino's", () => {
  assert.equal(light[M + "primary"], "#435e91");
  assert.equal(light[M + "surface-container-high"], "#e8e7ee");
  assert.equal(light["cassino-you"], "#924c00");
  assert.equal(light["cassino-them"], "#00696e");
});

// The cards-out panel is a dark card of the score HUD's neutral greys (kit/css/hud.css), in
// either colour scheme, so the two panels at the left read as one set.
const tcss = readFileSync(new URL("../src/tracker.css", import.meta.url), "utf8");
const hcss = readFileSync(new URL("../../kit/css/hud.css", import.meta.url), "utf8");
const trackerBlock = tcss.match(/\.tracker-root\s*\{[^}]*\}/)[0];
const tracker = tokens(trackerBlock);

test("the cards-out panel sets its own palette, whatever the colour scheme", () => {
  assert.doesNotMatch(tcss, /prefers-color-scheme/);
  const used = new Set([...tcss.slice(tcss.indexOf(".tracker-root")).matchAll(/var\(--(md-sys-color-[\w-]+|tracker-(?:on-)?opp)/g)].map((m) => m[1]));
  for (const name of used) assert.ok(tracker[name], `--${name} is set on .tracker-root`);
  assert.ok(!("tracker-opp" in light) && !("tracker-opp" in dark), "the page tokens leave the tracker's to it");
});

test("the cards-out panel's ground and text are the score HUD's", () => {
  const hud = hcss.match(/\.hud,\s*\.hud-card\s*\{[^}]*\}/)[0];
  assert.equal(tracker[M + "surface-container-lowest"], hud.match(/background:\s*(#[0-9a-f]{6})/i)[1]);
  assert.equal(tracker[M + "on-surface"], hud.match(/\bcolor:\s*(#[0-9a-f]{6})/i)[1]);
});

test("the cards-out panel's text reaches AA on its ground", () => {
  const T = [
    [M + "on-surface", M + "surface-container-lowest"], // the panel
    [M + "on-surface-variant", M + "surface-container-lowest"], // byline, legend
    [M + "on-surface", M + "surface-container-high"], // the fold button
    [M + "error", M + "surface-container-lowest"], // the red suits' marks
    [M + "on-primary", M + "primary"], // a card in your hand
    [M + "on-surface-variant", M + "surface-container-highest"], // a card played by you
    ["tracker-on-opp", "tracker-opp"], // a card your opponent played
  ];
  for (const [fg, bg] of T) assert.ok(ratio(tracker[fg], tracker[bg]) >= 4.5, `${fg} on ${bg}: ${ratio(tracker[fg], tracker[bg]).toFixed(2)}`);
});

// The cards-out cell he played is one clear red: not a grey of the panel's other cells, nor the teal the slate gives him.
const hue = (hex) => {
  const [r, g, b] = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
  return r > g * 1.3 && r > b * 1.3;
};
test("the opponent's played cell is red, apart from the grey one", () => {
  assert.ok(hue(tracker["tracker-opp"]), tracker["tracker-opp"]);
  assert.equal(tracker["tracker-on-opp"], "#ffffff", "one hue on the cell: white text and bar on the red");
  for (const set of [light, dark]) assert.notEqual(tracker["tracker-opp"], set["cassino-them"]);
  for (const fill of [M + "surface-container-highest", M + "surface-container-lowest", M + "primary"]) assert.ok(!hue(tracker[fill]), `${fill} is not red`);
});

test("the opponent's played cell is a single red, and keeps its high-contrast rule", () => {
  const theirs = tcss.match(/\.tracker-cell\[data-state="theirs"\]\s*\{[^}]*\}/)[0];
  assert.doesNotMatch(theirs, /box-shadow|border-bottom/, "one colour, so the legend's swatch reads as one kind of card");
  const more = tcss.slice(tcss.indexOf("prefers-contrast: more"));
  assert.match(more, /data-state="theirs"/);
});

test("the cell played by you is dashed against the solid outline of one still out; the card's head is the HUD's card, its body scrolls under it", () => {
  assert.match(tcss, /\[data-state="played"\][^{]*\{[^}]*border-style:\s*dashed/);
  assert.match(tcss, /\.tracker\s*\{[^}]*overflow-y:\s*auto/);
});
