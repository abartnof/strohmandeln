# Review: cards-out panel in the HUD's greys, solid red, row counts (TABLE3D 6.6)

Scope: `git diff 6648538..HEAD` over `web3d/src` (tracker.css, tracker.js, trackerview.js,
style.css), `web3d/test` (tracker.test.js, colour.test.js), `web3d/strings.json`, TABLE3D 6.6.
`npm test`: 645 pass, 0 fail. Browser suite not run.

Checked and sound
- Nothing else reads the removed `--tracker-opp` / `--tracker-on-opp` from style.css (grep over
  src, test, kit); the swatch in the legend reuses `.tracker-cell`, so it picks up the root tokens.
- Every `var(--md-sys-color-*)` used below `.tracker-root` is set on it (colour.test.js checks this).
  The tab (`.tracker-tab`) is outside the root and keeps the page tokens, as intended.
- Counts: `you` = mine + played, `opp` = theirs, `seen` = the three; matches the cell states and
  the test; short tarock row padded with 2 spans, so counts sit in columns 10 to 12.
- Heading row: `.tracker-heads .tracker-count-head:first-child { grid-column: 10 }` is correct
  (mark col 1, cells 2 to 9); the other two heads auto-place at 11 and 12.
- Left-handed: `.left .tracker-root` only swaps the edge; the grid is symmetric, no change needed.
- `prefers-contrast: more`: theirs/played become transparent with `on-surface` text and dashed or
  struck borders; on the dark ground this still reads (the panel ground is set in the same block).
- White on #a8322d is 6.65:1 (doc says 6.6).
- Layout code in main.js (`placeTracker`) reads only the root's rect and the HUD, not a width;
  the wider panel is handled by the fan-lane check (`lane = root.getBoundingClientRect()`).

Findings
1. Medium, open: the counts never reach the screen reader. The grid is `aria-hidden`, and the
   `tracker.sr.group` sentences (strings.json:644, tracker.js:122) carry only "yours / still out".
   Sighted users now get per-row totals that blind users cannot. Fix: add the counts to the
   sentence, e.g. "{kind}: yours {mine}; still out {out}; you {you}, opponent {opp}, seen {seen}."
   (en + de), pass `g`'s counts from `counts()`, extend the sentence test, and say so in 6.6.7.
2. Medium, open: stale browser check. `check_tracker_clear` (browser_aids.py:1236) demands a scroll
   of at most 1 px; the heading row adds about 14 px of height, so 1280x720 and 1024x768 may now
   scroll and fail. Run the suite at the four sizes in en and de; if it scrolls, trim spacing or
   raise `--tracker-max` allowance. The suite also checks nothing about the counts (21 numbers, 3
   heads, none in the hidden grid's sentences); add `.tracker-count` = 21 and the row sums
   (`seen` = 54 less the tracker's "out" count) to `check_tracker_clear`. Also the width: the
   rectangle checks at 1024x768 are the only ones that would catch the panel (400 px) reaching the
   hand fan at the left or right (left-handed is not in `TRACKER_SIZES`).
3. Low, open: TABLE3D 6.6.6 still says "A card of 300 px at the left" (now 400, tracker.css:27).
   Fix the number.
4. Low, open: TABLE3D 6.6.7 still says "Colours are the table's tokens (`--md-sys-color-*`,
   `--cassino-you`, `--cassino-them`)". The panel now sets its own palette on `.tracker-root`
   (tracker.css:10-23). Reword: the HUD's greys, the one red, and the contrast figures.
5. Low, open: colour.test.js:~99 comment still reads "he played is a dull red" and the assertion
   `ratio(opp, surface-container-highest) < 2` ("a red tint, not a text-weight contrast") no longer
   describes the design (a clear, solid red); the red/grey ratio is 1.4 by accident. Drop the
   assertion or assert the real aim: red differs from every other state's fill. Also the test title
   in the same block says "dull". The test text in tracker.test.js is fine.
6. Low, open: under `prefers-contrast: more` the new count text (tracker.css:67-69) stays in
   `on-surface-variant` (#9aa3ad, 6.7:1 on the ground; passes AA) while the byline and legend
   are lifted to `on-surface`. Add `.tracker-count, .tracker-count-head` to the line at
   tracker.css:104 for consistency.
7. Low (simplicity): the padding spans in trackerview.js:40 exist only to push the counts to
   column 10. Cleaner: give `.tracker-count` and the heads explicit `grid-column` (10, 11, 12) by
   `data-count`, and drop the spans and the `:first-child` rule (tracker.css:66). Then the heading
   and the row use one rule and a missing cell cannot shift the counts.
8. Info: the tab's expanded state (tracker.css:6) uses `primary-container` of the page scheme while
   the panel is always dark; on a light scheme the tab and panel differ in tone. Accepted per the
   maintainer's HUD-matching note unless it looks off in the light scheme.
9. Info: the panel's palette is a copy of the HUD's hex values (tracker.css:11-21). colour.test.js
   pins ground and text to hud.css, but not `--md-sys-color-surface-container-high`,
   `-highest` or `outline*`; they can drift from hud.css's buttons and segment unnoticed. Either
   pin them in the same test or note that they are only the HUD's by hand.
