# Review: page chrome on Material 3 tokens

Scope: web3d/src/style.css, table.css, tutor.css, web3d/test/colour.test.js.

## Findings

Fixed:
- Locked settings rows (`.settings-locked .settings-option`) faded to opacity 0.55, putting option text below AA and fading the selected value. Opacity removed; the row's "from the next sitting" note and the default cursor mark it. A test now guards the rule.

Checked, no change:
- No references to `--accent`, `--ink` or `--paper` remain in web3d/src (CSS or JS); JS sets no colours.
- Remaining literals: `--gold` and `--dark` (declared, game marks), `.tour-line` and the html/body ground (the table's ground stays light in both schemes), white text on the `--dark` pills (badge, tags, chip, banner), and black shadow alphas. All have a reason.
- Dark mode: tokens are the same scheme generated dark; the chrome pairs all reach 4.5:1 in both sets (colour.test.js). The canvas ground and pills over it are fixed, so they do not flip.
- The test covers 15 text/ground pairs in light and dark, plus token identity with Cassino.

Open:
- Not covered by the contrast test: white on `--dark` (over a variable ground, alpha 0.9) and `.tour-line` on the light canvas. Both are high contrast by inspection.
- `prefers-contrast: more` maps the slate and settings to token extremes rather than pure black/white; contrast is high but not maximal.
