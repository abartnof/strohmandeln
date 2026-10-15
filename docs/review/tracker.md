# Review: the cards-out tracker (`main..worktree-agent-a94cde8a8f8bb5814`)

Scope: `crates/strohmandeln-ai/src/tracker.rs`, `strohmandeln_tracker` in
`crates/strohmandeln-wasm/src/lib.rs` and `smoke.mjs`, `web3d/src/{tracker,trackerview}.js`,
`tracker.css`, the hooks in `main.js`, `ui.js`, `prefs.js`, `settings.js`, `engine.js`,
`strings.json`, TABLE3D 6.6, PROTOCOL "The tracker", tests. Checked against RULES 4, 5.4,
6, 8, 13, TABLE3D 6.3.4 and CLAUDE.md.

Run: `cargo test -p strohmandeln-ai tracker` (10 pass), `cargo test -p strohmandeln-wasm tracker`
(2 pass), `node --test web3d/test/` (276 pass), `kit/build.py --check` (current),
`bin/browser-full tracker_clear,tracker_fold_and_settings,tracker_languages` (all ok).
Screenshots taken at 1280x720: English light and dark, `prefers-contrast: more`, German
left-handed. The smoke test (`smoke.mjs`) was not run.

Severity: bug, should, nit. No bug found.

## Author's checklist

1. **Hidden cards: holds.** `tracker()` takes `&View` and builds on `Knowledge::from_view`;
   the twin test (`blind_check_values`) re-shuffles the hidden cards under an identical
   view and asserts equal output (more than 20 twins). Both wasm test and `smoke.mjs` assert
   every `audit()` card (his private cards and every face-down packet card, both players')
   is `unseen`. Nothing in the wire carries more than `place`, `by`, `trick`.
2. **Masters never overstated: holds.** A master needs `out` (his hand shown, his covers,
   unseen) to hold nothing higher; my own face-down packet cards are in `unseen`, so they
   count against me. `a_master_is_my_best_card_with_nothing_higher_out` checks it against
   the true hands. A master in a suit can still be trumped (see 6), which is the gap a new
   player falls into, but it is not a false claim of the engine.
3. **Void lifting: sound.** The void is set by `Knowledge` only from tricks I led (a reply
   of another kind, and no tarock when a suit was led, RULES 5.4). Everything that can
   enter his playable set afterwards is shown first (pull, cover, honour, RULES 4.5-4.6,
   6.1, 13.4), so "no card of the kind shown in his hand or covers" is exactly "still none".
   After the shown card is played the flag returns, which is right. The test checks the flag
   against his true hand and covers over 40 deals. Wording: see 9.
4. **TABLE3D 6.6, PROTOCOL.md, code agree** on places, groups, kinds `h d c s trump`, the
   line order, the visibility rule, fold default (900x600) and the 700 px / upright cut-off,
   with the exceptions at 5 (docs say "Points") and 12 (tab order).
5. **Strings en/de complete, formal German, opponent never named.** Every key has both
   languages, "Ihr Gegner" / "your opponent" throughout, Sie-form. Issues are wording only:
   see 7, 8, 10.
6. **Mute semantics mirror the hint chip.** `trackerVisible` is `hints && tracker &&
   !(tutor && muted)`; `askHint` is `prefs.tutor && prefs.muted`. Same condition, same
   exception when the tutor is off. Tested.
7. **Engine called only at rest: holds.** `updateTracker` runs only inside the rest handler
   (`main.js:649`); `showTracker` calls the engine only with `!director.busy() && pending === 0`
   and an empty cache, so a setting change mid-animation redraws the old payload and never
   reads ahead of the table. The cache is dropped on a new sitting.
8. **Dark and high contrast: holds.** Every colour in `tracker.css` is a `--md-sys-color-*`
   token or `--cassino-you/them` (both have dark values, `style.css:34,74`); the
   `prefers-contrast: more` block is present (solid 2 px border, thicker bars, borders in
   `on-surface`). Dark and contrast screenshots read well. No animation anywhere.

## Findings

### Terminology

1. **should: "Points still out" breaks the one-meaning rule** (`web3d/strings.json:483`,
   `tracker.line.points`). "Points still out: 30⅔ of 70" uses the capital for the count.
   Other uses to change: `web3d/test/tracker.test.js:154-156` (names and assertions),
   `:160` (test title "Points"), `web3d/src/tracker.js:5` (comment), `tracker.rs:67-68`
   ("thirds of a Point", "70 Points"), `docs/PROTOCOL.md:417` ("thirds of a Point", "70
   Points"), `docs/TABLE3D.md:901` ("the Points") and `:941` ("4. Points: ... Points still
   out"). German is already right ("Augen noch draußen: ... von 70"). Fix: en
   `"Still out: {points} of 70 points"`, de `"Noch draußen: {points} von 70 Augen"` (the
   unit as `hint.unit.other`; or follow `hud.thirds` and drop the unit: "Count still out:
   {points} of 70" / "Zählung noch draußen"). Rename "Points" to "points" or "count" in the
   places listed. RULES 8.2 itself says "36 Points": not this branch's to change.

### Cell states and layout

2. **should: played-by-you vs played-by-opponent is nearly invisible** (`tracker.css:69-70`,
   `:88-89`). At 1280x720 the two states differ by a 3 px bar, below versus above, in brown
   (`#924c00`) versus teal, on a cell that is otherwise identical: a struck-through 11 px
   label, a lighter border, no fill. Colour is the real signal, the position of the bar is not
   readable at a glance, and the legend swatches are blank cells with a bar, so they teach
   nothing the grid does not. The three states that matter for counting (mine, played, still
   out) do read apart; the fourth does not. Options, simplest first: (a) one "Played" state,
   with who played it and which trick in the cell title only (the player rarely counts by
   whom; a three-entry legend also fixes 3); (b) keep two, but give each a tinted background
   (`color-mix` of `--cassino-you/them` at about 25 percent) so the difference is a field and
   not an edge. Played cells should also be visibly dimmer than "out" cells (a grey fill):
   today "out" (outlined, dark text) versus "played" (outlined, grey struck text) is the only
   cue besides the strike, which at 11 px hides half of the "K" and "C" glyphs ("€" in the
   screenshot).
3. **should: in German the legend is cut off at 1280x720** (`tracker.css:18`, `main.js:placeTracker`).
   German lines wrap to two lines each, so the panel is taller than the room between the HUD
   and the Kibitzer's tabs; the panel scrolls inside, but the bottom of the legend, the two
   entries that explain the bars ("Vom Gegner gespielt", "Noch draußen"), sit below the fold
   with no visible cue that there is more (`de-left.png`, panel 320x370 px; the English panel
   fits). Fix: three-state legend (2a); or put the legend above the grid; or tighten the
   lines (`line-height 1.25`, smaller `margin`); or let the panel extend behind the Kibitzer's
   tabs row on a 720 px window. A browser check at 1280x720 in German that the legend's last
   item is inside the panel's visible rectangle would have caught this.
4. **nit: 11 px cell labels** (`tracker.css:213`). XVIII and Sküs fit but are small for a
   learner; the cells are about 29 px wide, so 12 px fits and helps. The panel is 300 px
   wide; 320 would allow 12 px with the same grid.
5. **nit: the `.info { z-index: 13 }` rule lives in `tracker.css:6`** though `.info` is
   defined in `style.css:87`. Move it there so z-order is read in one place.

### Lines: usefulness

6. **should (judgement): the lines are good, but not the most useful ones for learning to
   count.** What a counting player needs after "how many tarocks are out" is "what do I
   need". The panel's tarock line and the masters line are right. The points line is the
   weakest: it is the cards not seen (including 3 to 9 of my own face-down packet cards,
   which are not "out" for any purpose of his) over the whole pack, so "30⅔ of 70" is not
   a number a player can act on; the number that decides the deal is the count so far against
   35⅔ (RULES 8.2, 107 thirds), and the HUD's "Count" shows each side's taken points only as
   "n of 70". Suggest: replace the points line by "You have 18⅔; 17 more win" / "Sie haben
   18⅔; 17 fehlen noch" (the engine's `take_thirds` is in the view; one more field), keeping
   "still out" in the grid. Two smaller additions: the masters line should say when a suit
   master can be trumped ("he has shown no spades: your ♠K may be trumped"), since the master
   claim is about rank only and a new player equates master with sure trick; and the
   tarock line should say what the "still out" count covers (see 7).
7. **should: "still out" includes your own face-down packet cards and the tarock line does
   not say so** (`tracker.line.trumps`, the legend "Still out"). A beginner reads "Tarocks
   still out: 15" as "he has 15 or could have". The grid has the information (`unseen_count`,
   `g.unseen` are in the payload and unused). Fix: word it "not yet seen" for the count or add
   a tooltip/legend gloss: "Still out: not played and not yours, including your face-down
   packet cards". Cheap and the most likely beginner misreading.
8. **nit: the tarock line's order is not stated** (`tracker.line.trumps`): "Tarocks still
   out: 11 — XXI, XVIII, XVII, XVI…" lists the highest four; the ellipsis reads as "and
   more" but not as "lower". Fix: "Tarocks still out: 11; the highest: XXI, XVIII, XVII,
   XVI". German "Tarock noch draußen: 15 — Sküs, XXI, XX, XVIII…" wraps to a line holding
   only "XVIII…" at 300 px.

### Strings

9. **should: "Tafel" / "slate" names two things** (`strings.json:454, 458, 460`; TABLE3D
   6.6.1). The tab row in German is "Einstellungen | Tafel | Ausgespielt": "Tafel" is the
   score slate (`ui.menu.slate`), yet the tracker's own strings call the tracker "die Tafel
   des Kiebitz" (tab title, setting note, "Die Tafel zuklappen"); English likewise says
   "Kibitzer's slate" next to a "Slate" tab. Rename the tracker's: "the Kibitzer's count of
   the cards" / "Die Kartenzählung des Kiebitz"; fold title "Fold the count away" / "Die
   Zählung zuklappen".
10. **nit: German wording** (`strings.json`):
    - `tracker.fold` "Falten" (459) reads as wrinkles; "Zuklappen" or "Einklappen".
    - `tracker.line.master` (480) "Ihr {card} ist das höchste {kind}": the card is a symbol
      plus K, D, C, B, XXI, so the article is wrong for half of them ("Ihr ♥D"). Make it
      article-free: "Höchstes {kind} im Spiel: {card}, und es gehört Ihnen." (en "The
      highest {kind} left is yours: {card}.")
    - `tracker.line.void` (482) "hat {kinds} nicht bedient" states the evidence, not the
      conclusion; the English "has shown no hearts" is nearer. Prefer "Ihr Gegner hat kein
      {kinds} mehr gezeigt." or follow `explain.void-opp` ("hat weder ... noch").
    - `tracker.line.trumps.none` (479) "alle sind Ihre oder gespielt": "alle sind in Ihrer
      Hand oder gespielt".
    - "Cards out" / "Ausgespielt" name opposite halves of the same set (out = still out in
      English panels, but "played out" in German); the panel shows both. The maintainer
      chose them, so only noted: the legend and lines carry the distinction.
11. **nit: card names in cell titles and the screen-reader text use `cardName`** (Arabic:
    "tarock 12"; `main.js:252`, `tracker.js:59,98`) while the cells show Roman numerals
    ("XII"); the hint and review already use `romanName` (`main.js:239, 839`; hint review
    "Open"). Pass `romanName` for the title (hover text); `cardName` is arguably better for
    the screen reader, so give the model both.

### Docs, wire and code

12. **nit: tab order** (TABLE3D 3.10.1 lists "the cards-out tab" but not the panel's Fold
    button). The panel's root precedes the slate root in the DOM (`ui.js`), so Fold is
    reached before the move bar. List it, or give the panel `tabindex` order consistent with
    the text.
13. **nit: unused payload fields.** `unseen_count`, `unseen_thirds`, `groups[].unseen` and
    `top_out` are computed, tested, documented and never read by the page; `outCount` in the
    model is read by nobody. Use `unseen` for finding 7, or drop them with the tests.
14. **nit: `trick` after the last trick.** `open = tricks.len() + 1` is 28 once the 27th is
    won while the deal is still in the view; PROTOCOL says "1 to 27". Unverified whether the
    view still carries the deal then; if so clamp or state it.
15. **nit: the engine is asked on a phone** where the panel and tab are `display: none`
    (`tracker.css:231`); `trackerVisible` ignores the media query. Harmless (once a rest),
    but a `matchMedia` guard in `showTracker` would also stop the `JSON.stringify` of the
    payload there.

## Verdict

Correct and well tested. The engine side (hidden cards, masters, voids, counts) is sound and
I found no bug. Fix before done: 1 (terminology), 2 and 3 (the cell states and the German
legend, which together decide whether a beginner can read the grid), 7 and 9 (two wordings
that mislead). 6 is a design suggestion for the maintainer; record as open in `PLAN.md` if not
taken. The rest are nits.
