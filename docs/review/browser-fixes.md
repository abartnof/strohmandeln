# Review: browser fixes (8fa73f7, 2cf304f, 2930118, 8a1a9b2)

Scope: the four commits of `worktree-agent-a5ea2e8c476858e2a` (merged as d06f085), against
TABLE3D 6.2.2, 6.6, 7.5.2 and 10. Reviewed at main a10bd7a.

## Run

- `node --test web3d/test/tally.test.js`: 6/6 pass.
- `bin/browser-full tutor_pages,language_switch_at_count,small_landscape_tabs`: 3/3 pass (42 s).
- `liveTally` against real sittings (oracle: a `trick_won` since the last `deal_start`/`count`): 207,415
  event prefixes (3 policies x 40 seeds x both seats, 3 Touren), each prefix also trimmed to start at its
  last `deal_start` (as a restored log does): 0 mismatches. No `void_deal`, `thrown_in` or `valat` event
  occurred in those sittings (default settings); the function never reads them (a void or thrown-in deal
  has no `trick_won`, so it is false; a Valat is a declaration, tricks follow as usual).
- Language switch in the browser (seed 7, human on seat 1) at: before the first trick; after 3 tricks; at 9
  moments from the last card played through the whole count (count not yet begun, the take shown, then
  every few lines of the count): the HUD's numbers are unchanged by the switch, and the count still ends on
  the engine's numbers in every case. Human on seat 0 was not run in the browser (the code has no seat in it;
  the numbers come from `hud.totals()`).
- Layout probe, 21-card hand, tab/card intersections: 800x450, 900x500, 900x600: 0. 901x500, 960x540,
  1000x560, 1100x620, 1280x720: 0. **901x600: 3.**

## 8fa73f7 tutor_pages

1. **should** `docs/TABLE3D.md:1375` (the `tutor_pages` line, from 8a1a9b2) says `the-cards`, `packets` and
   `bidding` come "at the deal". 6.2.2's table opens `bidding` at the first `bid` turn, and its pacing
   paragraph names only two pages at the first `dealt` (the cards, then the packets). Seed 7 with the human
   in front puts the bid turn at the same rest, so three pages come together; a seed with the opponent
   speaking first would give two and the check would fail on the seed, not on the table. Fix: say in 6.2.2 and
   in the check's list that the human's first bid turn follows the packets page at the same rest when the
   human speaks first (the case the check pins), or give the check's comment the seed's reason.
2. **should** (note for the maintainer) Three pages at the first rest is a lot for a newcomer: the cards (a
   page that answers no decision), the packets, bidding, each asking for a "Not now" before a card is
   touched. `the-cards` first is the maintainer's choice (2026-10-15, 6.2.2); the other two were already due
   there by the pacing audit. Not a defect of the check; if it is felt, fold `packets` into `the-cards` or
   let `bidding` wait for the bid prompt.
3. **nit** `web3d/test/browser_aids.py:~661` `page.wait_for_timeout(300)` after each "Not now" decides
   whether another page follows. If the next page ever opens later than 300 ms the loop breaks early and the
   check fails on a short `titles` list, not on the cause. Use `wait_for_function` on
   `[data-dialog=page] h2` changing, or on the dialog count with a timeout, then read the count.
4. The check now tests what 6.2.2 says for the order and one-at-a-time (each title, in order, one dialog
   at a time, all three marked seen). It no longer asserts "never in motion" for pages 2 and 3, only for the
   first (`busy()`); the spec says every page opens at rest. **nit**: add the `busy()` assertion inside the loop.

## 2cf304f liveTally and the language switch

5. **should** Test coverage of 7.5.2 at the moments asked for is thin. `liveTally` has one Node test
   (`web3d/test/tally.test.js:78-88`) with synthetic events: empty, dealt, one trick, counted, next deal
   dealt, next deal's first trick. Missing, and cheap: a void deal then a new deal (`deal_start`, `dealt`,
   `void_deal`, `deal_start`), a throw-in (`thrown_in`), a Valat (`valat` between tricks and `count`), a log
   trimmed to start at the last `deal_start`, and a log with only `trick_won` events (a trimmed log with
   no `deal_start`). The rebuild itself (`web3d/src/main.js:1152-1170`, the choice of `hands`/`live`) is a
   closure in `onLang` and cannot be tested in Node. Fix: move the choice to a pure function in
   `counting.js`, for example `hudAfterSwitch(events, totals, game)` returning the `hud.reset` argument, with
   Node tests for each moment; the browser check then only confirms the wiring. The browser check
   `language_switch_at_count` pins one moment (the take shown, count not begun); I ran the others above
   and they hold, but nothing keeps them so.
6. **nit** `web3d/src/counting.js:50-52` (comment) and the commit message say that before a new deal's first
   trick "the HUD shows the last count". It does not: `deal_start` resets the HUD to nothing
   (`main.js:661`, `hud.reset({hands: [], live: null})`), so the HUD shows 0:0 then, and the switch keeps
   0:0 (probe: 0:0 before and after). The behaviour is right (numbers as shown, 7.5.2); the comment is
   not. Fix: "shows what it shows (the last count until the new deal is dealt, then nothing)".
7. **nit** `main.js:1169-1170` in the not-live branch the HUD is rebuilt with one counted hand
   (`hands: [ledger]`, `current` null), also for a deal that is dealt and not yet played. The live block
   ("Hand 1 live") is then shown as "Hand 1 subtotal" until the first trick lands (`showTally` resets it).
   The numbers are right; the label is for one trick's time. Acceptable; note only.
8. Edge cases checked, no defect: a restore mid-deal (`main.js:1186` runs `showTally`/`endHand` after the
   restore's own language change, so the numbers are the log's whatever the switch did); a trimmed log
   (the scan reads backwards and never needs a `deal_start`); the human on either seat (no seat in the
   function; `hud.totals()` is already in you/opp); a switch mid-count (the rebuilt HUD holds the partial
   numbers in one counted hand; the scheduled popups then add on top, verified to end on the engine's
   numbers); a switch while a trick has been won but not yet landed (the HUD's shown numbers are kept and
   `showTally` catches up when the cards land). The old code would have used the full-log tally there
   (ahead of what was shown); the new one is the better reading of 7.5.2.

## 2930118 export_import_file

9. **nit** `browser_aids.py:~630` `tempfile.mkdtemp(prefix="sitting-")` is never removed; use
   `tempfile.TemporaryDirectory()` around the check or remove it at the end. Otherwise correct: both files
   are written under it, nothing else used `/tmp/sb`.

## 8a1a9b2 small_landscape_tabs and tutor.css

10. **should** `web3d/src/tutor.css:21` `(max-width: 900px)` includes 900; the commit and the check
    say "under 900". At 900 wide the tabs stack; at 901 they do not. No desktop size is meant to change and
    none >= 901 does, but 900x600 and 900x500 are small-desktop widths, and the boundary is not the
    documented one. Fix: `(max-width: 899.98px)` if "under 900" is meant; or state "up to 900" in the
    commit text and TABLE3D 10.
11. **should** The defect is not a function of 900. At **901x600** (landscape, 21 cards) the tab row still
    covers 3 cards of the hand (tab row 266 px wide; probe above), the same defect the commit fixes below
    901. 901x500 and 960x540 are clear; it depends on the fan. The check tests two phone sizes only.
    Fix: add 901x600 (and one more, 1000x600) to `check_small_landscape_tabs`, and either raise the
    breakpoint (the stacked column, 136 px, was clear at all sizes probed) or key the stack on the fan's
    reach rather than a width. Before 8a1a9b2 the same sizes had the same overlap; this is not a regression.
12. **should** Collision with the tracker panel: at 800x450 with the tracker unfolded
    (`trackerFits` is true for width > 700 and ratio > 0.85, so the panel is drawn; it opens folded by
    default there, `tracker.js:31`), the stacked tabs (y 248-342) now meet the panel (y 156-276, its
    120 px floor in `placeTracker`, `main.js:301`): tab 1 intersects it. With the row (forced back by a style
    override) the panel did not meet the tabs. The panel already covered 3 cards of the hand at that size, so
    the tracker is broken there with or without the fix; the stack adds a second overlap.
    700 to 900 px landscape with a low
    window is the phone-on-its-side case. Fix within the deferral of 6.6.6 ("upright or narrow, the tracker
    is simply not there"): hide the tracker also when the window is low, for example `trackerFits` requires
    `height >= 500` and `tracker.css` `@media (max-height: 499px)` hides tab and root. No HUD collision:
    tabs vs `.info .hud` is clear at every size probed (899x600, 900x600, 800x450, 720x480).
13. **ok** Desktop >= 901 px: the media query does not apply, `.tutor-row` unchanged; 1024x576 and 1280x720
    layouts probed as before (tabs in a row, clear of HUD and panel). `.left` (left-handed) keeps its
    alignment (`align-items: flex-end`). The portrait rule (`max-aspect-ratio: 85/100`, row above the
    controls) is a separate, later block; a window both portrait and <= 900 wide is not landscape, so the new
    rule does not apply to it. A phone layout is deferred (6.6.6, the maintainer): this change stays
    within "must not break" except for finding 12.

## Verdict

Accept the four commits. Findings 1, 5, 10, 11, 12 should be fixed or recorded as open in `PLAN.md`;
the rest are nits. No bug found: `liveTally` is correct at every moment probed and the HUD keeps its numbers
at every moment probed.
