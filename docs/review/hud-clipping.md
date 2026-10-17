# Review: the score HUD's clipped score (cb0d973)

Reviewed 17 Oct 2026 against main: `kit/css/hud.css` (`.hud-line { min-width: 0 }`),
`web3d/src/style.css` (`.info` 340 to 384 px, `.info .hud-side` name column 3.9em),
`check_hud_not_clipped` (`web3d/test/browser_aids.py`, `CHECKS_104`). The built
`web3d/strohmandeln.html` carries the same three CSS changes.

Run: `node --test kit/test web3d/test` 538 pass, 0 fail; `kit/build.py --check` clean;
`--only=hud_not_clipped` and `--only=tracker_clear` pass.

## Verdict

The clipping is fixed: the score cell is 63 px at "35⅔" (was 44), and the check holds it.
The segment line, however, is not legible, and was not before; the change leaves it so.
One spec line is missing. Nothing blocks the fix itself.

## Findings

### 1. High (pre-existing, not repaired): the 36 segments are about 0.1 px wide

Strohmandeln's line has 36 segments plus the finish (`SEGMENTS = 36`,
`web3d/src/counting.js`; TABLE3D 2.5.3), not 21 (that is Cassino's `N`). The kit puts fixed
margins on them inline (`kit/js/hud.js`: 3 px between, 8 px between groups of five):
about 35 margins, roughly 140 px, before a single segment has width.

Measured at 1280x720, score "35⅔":

| | line | segment (first / middle / finish) |
|---|---|---|
| before (main's CSS) | 140 px | 0.0 / 0.0 / 0.0 px |
| after (cb0d973) | 144 px | 0.1 / 0.1 / 0.2 px |

With a score of 7 the segments are 0.3 px before and 0.9 px after. So the line is a row of
sub-pixel hairlines at every score; a reached segment (6 px high) is 0.1 px wide, and the
screenshot shows faint dashes only. `min-width: 0` makes the line shrink exactly as far as the
margins allow, and no further: the score now has its room, and the progress display has
none. TABLE3D 2.5.3's "a segmented line to 36" (and 2.5.4's "one segment shy of the finish")
are not visible in practice at any widely used size.

Fix (separate commit, kit change, check Cassino's 21): make the margins part of the budget,
not fixed. Either `--seg-gap` / `--seg-gap-group` custom properties on `.hud-line`,
read by `kit/js/hud.js` (defaults 3 px and 8 px; Strohmandeln 1 px and 3 px gives 49 px
of margins and about 2.6 px segments in the same 144 px), or a margin in `em` of the
segment count. Add a browser check that a segment is at least 2 px wide at the four sizes.
Alternative within Strohmandeln only: drop the group margins and let the HUD be 420 px.

### 2. Medium: `check_hud_not_clipped` does not see finding 1, and its spec line is missing

- The check tests text spill and the HUD's rectangle only; segments have no text. A width
  assertion on `.seg` (at least 2 px) belongs in it once finding 1 is fixed.
- TABLE3D section 10 has no line for it (grep for `hud_not_clipped` in `docs/` finds
  nothing). Add one under 10.1 (layout), for example: "`hud_not_clipped`: with the largest
  scores injected (35⅔ a side, 135⅔ in the ledger, a bonus label's popup), no text in the HUD
  overflows its box or the HUD's rectangle, at 1024x768, 1280x720, 1440x900 and 1920x1080,
  in English and German."
- Injected values: "35⅔" is the widest tally a side normally shows (tabular figures, and 69⅔
  is as wide; a valat winner's "70" is narrower). "135⅔" for the ledger is a plausible but
  arbitrary bound; a six-Touren sitting can run higher. Say in the check's docstring that the
  ledger bound is chosen, not derived. Both are fine for the purpose.

### 3. Low: "Your opponent" wraps to two lines

At 3.9em (62 px) both "Your opponent" and German "Ihr Gegner" take two lines (checked at
1024x768 and 1280x720); "You" / "Sie" take one. The two-line name sits in the 40 px block
(about 34 px of text), HUD height unchanged (120 px), no clipping. Acceptable: it is the
name label, left aligned, and the alternative costs 25 px more from the line (finding 1).
If finding 1 is fixed by smaller margins, the name column can go back to one line (about
7.5em) at no cost to the score.

### 4. None found: Cassino

`~/cassino/web3d/src/hud.js` uses `N = 21` in a `.info` 470 px wide; the line is about
300 px there, segments about 10 px. `min-width: 0` on a `flex: 1` item only changes the
outcome when the line would otherwise overflow its grid track, where it now shrinks the
segments (which are `min-width: 0`, `flex: 1 1 0`) instead of squeezing the score. Safe; on
a 390 px phone Cassino's line also benefits. Apply the same check to Cassino when
finding 1's kit change is made.

### 5. None found: overlaps from the wider `.info`

- HUD rectangle 8..392 x 8..128 at every size tried; the HUD's height is unchanged, so the
  tracker (`--tracker-top` follows the HUD and tour line; panel 8..338 wide, 330 px) stays under
  it, clear. `tracker_clear` passes at 1280x720 and 1024x768.
- Top-right buttons against the HUD at 667x375, 700x450, 800x450 and 1024x768, English and
  German, hints on: no overlap (closest, German 667: buttons from 458, HUD to 392).
  The right-hand HUD of left-handed mode mirrors the same width.
- The Kibitzer's tabs are bottom left and the move bar bottom centre: unaffected.
- The `@media (max-width: 520px)` rule's 384 px is clamped by `100vw - 16px`, as before.
- The doubling chip at (160, 120) and the "35⅓ is not 36" chips at (120, 100 and 150) sit
  over the HUD's lower edge now as before (the HUD's bottom is 128): unchanged by this
  commit, and TABLE3D 1.5.5 names a "box" (the speech box), which takes a free place.

### 6. None found: the "x2" and the valat chip

The "×2" is a word in the tour line below the HUD (`hud.double`), not a chip in it; the
tour line is 380 px wide and "Tour 2 of 3 · Deal 3 · ×2 · Trick 27 / 27" fits on one line in
both languages. The valat chip is a table chip placed at the seat, not in the HUD. The
bonus label's popup is covered by the check (`popped=True`).

## Fixes

- Finding 1 (36 segments ~0.1 px wide): the kit's segment gaps are CSS custom properties (`--seg-gap` 3 px, `--seg-gap-group` 8 px by default, so Cassino's 21 segments are unchanged; a group start is a `.grp` class, not an inline margin). Strohmandeln sets 1 px and 3 px on `.info .hud`. Narrowest segment, every size and language: 0.11 px before, 2.97 px after. `hud_not_clipped` asserts >= 2 px. The name column stays at 3.9em: "Your opponent" still wraps to two lines.
- TABLE3D section 10 names `hud_not_clipped`; its docstring says 135 2/3 is a chosen wide value.
