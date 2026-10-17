# Review: cards-out opponent colour, panel follows the HUD (2a55d38, 26133e5)

Reviewed 17 Oct 2026 against main. Requests: the opponent's played cards in the
Cards-out panel apart from the grey of "not seen yet" (a dull red); the panel and the
score HUD both on screen, not colliding.

## Checks run

- Setup note: `node_modules` at the root must point at `web3d/node_modules`.
- `node --test kit/test web3d/test`: 252 passes, no failures in the files that finished
  (the run was cut by the time limit on a loaded 2-core VM; `tracker.test.js` and
  `colour.test.js` pass singly). `python3 kit/build.py --check`: ok.
- Browser, singly: `tracker_clear`, `tracker_languages`, `tracker_fold_and_settings`: ok.
- Contrast, computed: light `#2e0b09` on `#dba7a1` 8.6:1; dark `#ffdad6` on `#7a3a37`
  6.5:1. Both clear 4.5:1 and are in `colour.test.js` PAIRS.
- Screenshots (panel-light, panel-dark, hud-list-closed, hud-list-open) looked at.

## Findings

1. **Low: the red cell's field against the panel is under 3:1** (light `#dba7a1` on
   white about 1.9:1; dark `#7a3a37` on `#0c0e13` about 2.2:1). Not a failure: the label
   passes, the 3px bar is the second cue and shows in all four screenshots, and the
   red-versus-grey separation is clear. No fix needed.
2. **Low: "played by you" against "still out"** is the weaker pair now. Light: a pale
   grey field (dimmed) against a white cell with a dark outline: told apart by the
   missing outline and the dimmed label; fine at a glance in the screenshots, but
   K / 2 / XV are faint. Dark: dark grey on near-black, same. It is the pre-existing
   treatment, and the maintainer's complaint (grey against "not seen yet") is fixed
   because the opponent's cards left the grey. Option if it is raised again: give
   "played by you" a dashed border.
3. **Low: `prefers-contrast: more`** handled: both played states struck through, the
   opponent's dashed, text in on-surface; the test checks the rule exists. The bar
   (box-shadow) stays in that mode on a transparent field: harmless.
4. **Low: Fold scrolls away.** `.tracker` is the scroll container and the head with the
   Fold button is inside it, so with the by-deal list open and the panel scrolled the
   Fold button is out of view (at rest it is visible, as in hud-list-open.png). The
   head is not sticky. Fix: `.tracker-head { position: sticky; top: 0; background: inherit; }`
   (or put the head outside the scroller). The "Cards out" tab also folds it.
5. **Low: the open-list case is not tested against the tabs.** `check_hud_clear` checks
   HUD and tour line against the panel, closed and open, at four sizes and two
   languages; it does not check the panel's bottom against `.tutor-row`. By construction
   `--tracker-max = limit - top - 6`, so the panel ends 6px above the tabs, except the
   `Math.max(120, ...)` floor: it overlaps only if less than 126px lies between the HUD
   bottom and the tabs. At 1280x720 with the list open there is about 260px (screenshot:
   panel ends at y=562, Tips tabs at 568), so no overlap at 1024x768 or 1280x720;
   smaller heights are not in the test sizes. Suggested: add the panel-bottom versus
   tabs assertion to `check_hud_clear`.
6. **Info: observers.** `ResizeObserver` on `.info .hud` and `MutationObserver` on
   `.floats` (childList only) are created once at start, never disconnected, and live
   for the page: no leak. `placeTracker` writes only two CSS custom properties on
   `.tracker-root`, which is not inside `.hud` or `.floats`, so its own move cannot
   trigger either observer; no loop. The HUD's size does not depend on the panel.
7. **Info: the chip jump.** A "not 36" chip adds a `.floats` child and moves the
   panel down for its life, then back when it is removed (the fade class alone is not
   a childList change, so the return comes at removal). The `rise` animation means the
   first measurement may be a few px off. Acceptable: the panel is never covered, and
   a reserved gap would waste space most of the time. Reserve the space only if the
   jump is reported as annoying.
8. **Info: left-handed.** `.left .info` goes right and `.left .tracker-root` goes
   right, so the HUD and the panel stay in one column, and the geometry is
   side-independent. Not exercised by a browser test.
9. **Tests.** `colour.test.js` (red hue, not the slate's teal, not text-weight against
   grey; the second cue and the contrast rule in the CSS), `tracker.test.js` (state
   `theirs`, legend of four, German strings, counts over real sittings) and
   `tracker_clear` (HUD clear, both lists, both languages) are meaningful. The legend
   is four entries; "Played by your opponent" wraps to its own line in the screenshots,
   readable.

No blocking findings. Fix 4 and 5 (small) or record them open.

## Fixes (17 Oct 2026)

- 2: the cell played by you has a dashed border (grey kept), apart from the solid outline
  of a card still out; `colour.test.js` checks the rule.
- 4: `.tracker-head` is sticky (`top: 0`, `background: inherit`): Fold stays in view
  while the panel scrolls; `colour.test.js` checks the rule.
- 5: `check_hud_clear` (in `tracker_clear`) asserts the panel clear of the Kibitzer's
  tabs as well, list closed and open, at all four sizes in en and de.
- 1, 3, 6-9: no change.
