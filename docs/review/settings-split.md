# Review: the settings split (sitting rows out of the menu, table-top select)

Scope: `497205b` (TABLE3D) and `e749143` (settings.js, newsitting.js, main.js, table.css, strings.json, tests) against main. Reviewer did not write it.

## Ran

- `node --test kit/test web3d/test`: 540 pass, 0 fail. `python3 kit/build.py --check`: ok.
- Browser checks `settings_no_rules`, `new_sitting_from_table`, `settings_rows`, each alone: pass.
- Red runs: `restore()`'s `onTable(sittingFrom(exported.record, ...))` temporarily replaced by a prefs-built record: `settings_no_rules` fails ("This sitting: 3 Touren, 2/3/4 ... level 3" after Continue) and `new_sitting_from_table` fails (screen starts from the defaults). Reverted; tree clean.

## Findings

1. **Medium, open item confirmed: after a restore the rules the page reads come from the prefs, not the sitting.** A restored sitting's `log` holds only the current deal (`deal_start`, `dealt`; probed: 0 `sitting_start`). Two readers use `log.find(sitting_start)`:
   - `pagesNow()` (main.js, ~317): the Kibitzer's pages fall back to `prefs` for `both_pass`, `values`, `ultimo`.
   - `pointsNow()` (~749): `begun?.settings?.ultimo ?? 1`, so a restored ultimo-2 sitting shows ultimo 1 in the live point value (worse: no prefs fallback at all). Pre-existing, same cause.
   Fix here, small, via `sittingOnTable`: in `pagesNow`, `const rule = (k) => sittingOnTable?.[k] ?? prefs[k]`; in `pointsNow`, `ultimo: (sittingOnTable?.ultimo ?? 1) * (d?.multiplier ?? 1)`. `sittingOnTable` is a `let` declared later in `main()` than these functions; they run only after init, so no TDZ, but move the declaration up beside `log` for clarity. Add a browser step: prefs differ from the sitting, Continue, open a rules page / check the point value. Recommend yes, in this package.
2. **Low: the This-sitting line is only partly tested against the record.** `settings_no_rules` "at the deal" runs with prefs equal to the sitting, so it cannot tell record from prefs; only the Continue step can (it does, shown red above). Not exercised in a browser: loaded file, language switch with the menu open, a new sitting from the dialog mid-sitting (all go through `onTable`/`onLang(render)` and read correctly by inspection; the unit test covers the German line). Fix: in the same check, after Continue, press Deal in `askNew` with other choices and re-read the line; switch the language and re-read.
3. **Low: stale `sittingOnTable` if `newSitting` fails.** `resetTable()` empties the table, then `engine.new` can fail; `onTable` is not reached, so the line and the next dialog still show the previous sitting. Fix: `onTable(null)` in `resetTable()`.
4. **Low: unchanged paths verified.** `newSitting` (events carry `sitting_start`), `restore` (Continue and loaded file share it, record from `exported.record`), void deal (no sitting change), language switch (`onLang(render)` keeps `sitting`), Deal from the dialog mid-sitting (`newSitting` calls `onTable`; Cancel leaves it). `sittingFrom` handles an events array with `sitting_start`, an export record, none (null), and missing settings (defaults). A record with a `touren` other than 3 or 6 would print the raw key `settings.touren.N`; the engine accepts only 3 and 6, so unreachable in practice.
5. **Low: `askNew` hook is gated.** `window.strohmandeln3d` exists only with `?seed`, `?test` or `?hook` (main.js ~1524), so `askNew` is no wider than `newSitting`, already there. TABLE3D 7.7.4 lists the URL overrides but not `?hook`; the open item stands unchanged (the test flag is the gate). Add `?hook` to 7.7.4 when that item is closed.
6. **Info: the select.** Labelled by `aria-labelledby` the row's label span; native keyboard (arrows, type-ahead) applies; 16 px, min-height `--target`, `secondary-container` pair (4.5:1 in both sets, tested). Unit and browser checks cover 19 options, Random first, change applies and is kept. German names: all 18 present and tested for presence; "Sitzsprenkel" (seat fleck), "Streukonfetti" (transit confetti) and "Karo-Augen" (pip diamonds) are loose renderings of the kit's names, acceptable for texture labels; "Funkelmuster", "Blumenmuster", "Nadelraster" read naturally. English names are the kit's.
7. **Info: Escape and the select.** TABLE3D 7.1 says "an open list takes its own Escape". Headless Chromium cannot show the native popup, so this is untested; in desktop browsers the popup consumes Escape. A closed, focused select passes Escape on, so the menu closes (as for any control in it). Keep the sentence, or soften to "a closed select passes Escape on".
8. **Info: nothing reads the removed strings.** No source or test references `settings.applies.*`, `settings.mid-sitting`, `settings.section.rules|opponent`, `.settings-locked`, `.settings-when`, `inSitting` (only the new negative asserts and `docs/review/colours.md`, a historical log). `ui.new.applies` is unused on main and here (dead string; remove or leave, not this package's).
9. **Info: tests.** Unit tests are meaningful (sittingFrom shapes, line in both languages, no sitting rows or applies column, select model and DOM, new-screen start). The colour test is a regex over `table.css` (brittle but guards the rule; the 4.5:1 assertion repeats `PAIRS`). Browser checks were run red here for the two restore paths (above); `settings_rows`' select assertions and `skill_speed` were not run red (the unit tests cover the select; `skill_speed` asserts the dial is gone from the menu and present on the screen).
10. **Info: TABLE3D.** 3.5.1, 3.5.2, 7.1, 7.2, 7.3 and 10.5 match the code. Trim: "as Cassino shows its own" (3.5.1) is an uncited comparison; drop it. 3.5.2's parenthetical on PROTOCOL "Creating a sitting" is accurate (level set only at `new`). 7.1's paragraph is long but terse enough.

## Merge with `worktree-agent-aba5e7b973d8045c4` (Escape layers)

Dry run (`git merge-tree`): `settings.js` merges cleanly (their hunk replaces `open`/`close` and removes the `keydown` Escape listener; ours touches `render`, `setSitting` and the rest, not that region); `docs/TABLE3D.md`, `browser_aids.py`, `settings.test.js` auto-merge. Conflicts: `web3d/src/main.js`, one hunk, the imports:

```
<<<<<<< ours      import { createSettings, sittingFrom } from "./settings.js";
=======           import { createSettings } from "./settings.js";
                  import { escapeTarget } from "./overlays.js";
>>>>>>> theirs
```
Resolve: keep both, i.e. `import { createSettings, sittingFrom } from "./settings.js";` then `import { escapeTarget } from "./overlays.js";`. And `web3d/strohmandeln.html` (generated): take either side, then `python3 kit/build.py` and commit; `--check` confirms. Behaviour after merge: the window handler takes Escape before the `closest("input, select, ...")` skip, so Escape on a closed focused select closes the menu through `closeLayer`; `open()` still focuses the first checked radio (speed exists), so the select change does not disturb it. Merge the escape branch first or second, no order dependence beyond the above.

## Open after this review

- Findings 1 to 3 to fix in this package (1 recommended); 5 and 10 are doc edits.

## Fixes

Merged `review-settings-split` and main (Escape layers); `main.js` imports keep both `sittingFrom` and `escapeTarget, selectGuard`; `strohmandeln.html` rebuilt, not hand-merged.

1. Fixed. `pagesNow` reads `sittingOnTable?.[k] ?? prefs[k]`; `pointsNow` reads `sittingOnTable?.ultimo ?? 1`. `let sittingOnTable` moved beside `log`. Hooks `pagesNow` and `pointsNow` added to `strohmandeln3d`. `settings_no_rules`, after Continue with prefs at the defaults: the live ultimo is 2 and the pages carry the record's both-pass rule (red before the fix: ultimo 1, pages from the prefs).
2. Fixed. `settings_no_rules` now also covers a language switch (German line), a Deal from the dialog mid-sitting (line follows the new sitting; the dialog starts from the sitting) and a loaded file in a fresh page (line returns to the record's), each with prefs differing from the sitting.
3. Fixed. `resetTable()` calls `onTable(null)`.
4. Fixed. TABLE3D 7.1 states: Escape on the open list closes only the list; on the closed, focused select it closes the menu (3.10.5). `settings_rows` presses Escape on both cases (the open case is driven with Alt+ArrowDown; headless Chromium draws no popup, so it exercises the guard, not the popup).
5. Fixed. "as Cassino shows its own" dropped from 3.5.1; `ui.new.applies` removed.
Open: finding 5 of the review (`?hook` in 7.7.4) stays with its open item.
