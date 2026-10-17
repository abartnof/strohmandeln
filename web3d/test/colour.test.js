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
  [M + "on-secondary-container", M + "secondary-container"], // big, settings options
  [M + "on-primary-container", M + "primary-container"], // selected option, review chip
  [M + "on-tertiary-container", M + "tertiary-container"], // tutor buttons
  [M + "on-tertiary", M + "tertiary"], // won chip
  [M + "on-error", M + "error"], // lost chip
  [M + "on-error-container", M + "error-container"], // refusal
  ["cassino-on-you", "cassino-you"], // edge chip
  [M + "on-surface-variant", M + "surface-container-highest"], // muted tutor button
  ["tracker-on-opp", "tracker-opp"], // cards out: a card your opponent played
];

for (const [name, set] of [["light", light], ["dark", dark]]) {
  test(`${name}: the chrome's text reaches AA on its ground`, () => {
    for (const [fg, bg] of PAIRS) {
      assert.ok(set[fg] && set[bg], `${fg} / ${bg} are defined`);
      assert.ok(ratio(set[fg], set[bg]) >= 4.5, `${name} ${fg} on ${bg}: ${ratio(set[fg], set[bg]).toFixed(2)}`);
    }
  });
}

test("locked setting rows keep full-contrast text (no opacity fade)", () => {
  const table = readFileSync(new URL("../src/table.css", import.meta.url), "utf8");
  const rule = table.match(/\.settings-locked \.settings-option\s*\{[^}]*\}/)[0];
  assert.ok(!/opacity/.test(rule), rule);
});

test("the light tokens are cassino's", () => {
  assert.equal(light[M + "primary"], "#435e91");
  assert.equal(light[M + "surface-container-high"], "#e8e7ee");
  assert.equal(light["cassino-you"], "#924c00");
  assert.equal(light["cassino-them"], "#00696e");
});

// The cards-out cell he played is a dull red, not the grey of the cards played by you or the teal the slate gives him.
const hue = (hex) => {
  const [r, g, b] = [1, 3, 5].map((i) => parseInt(hex.slice(i, i + 2), 16));
  return r > g * 1.3 && r > b * 1.3;
};
for (const [name, set] of [["light", light], ["dark", dark]]) {
  test(`${name}: the opponent's played cell is red, apart from the grey one`, () => {
    assert.ok(hue(set["tracker-opp"]), set["tracker-opp"]);
    assert.notEqual(set["tracker-opp"], set["cassino-them"]);
    assert.ok(ratio(set["tracker-opp"], set[M + "surface-container-highest"]) < 2, "a red tint, not a text-weight contrast");
  });
}

test("the opponent's played cell keeps a second cue and the high-contrast rule", () => {
  const tcss = readFileSync(new URL("../src/tracker.css", import.meta.url), "utf8");
  assert.match(tcss, /\[data-state="theirs"\][^{]*\{[^}]*(box-shadow|border-bottom)/, "a mark beyond colour");
  const more = tcss.slice(tcss.indexOf("prefers-contrast: more"));
  assert.match(more, /data-state="theirs"/);
});
