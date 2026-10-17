# Review: "Cards out" starts folded

Scope: branch `worktree-agent-af0ef1a423223724b` against main: `web3d/src/main.js`,
`prefs.js`, `tracker.js`, the tests, TABLE3D 6.6.2 and 6.8, the built
`web3d/strohmandeln.html`.

## Checked, correct

- `trackerFolded` is gone from `prefs.js`; `loadPrefs` reads only the keys in
  `CHECKS`, so an old saved value is dropped (test in `prefs.test.js`). No stale
  reference remains in docs, kit, strings or the source; the only mentions are the
  deliberate "old value ignored" lines in TABLE3D and the tests.
- Load, new sitting and restore/Continue go through `resetTable`, which now clears
  `trackerUnfolded`: a restored mid-deal sitting starts folded.
- Tab state: `ui.setTracker({ tab, open })` takes `open` from the same
  `trackerUnfolded`, so `aria-expanded`, the tab and the panel cannot disagree.
- `kit/build.py` reproduces `web3d/strohmandeln.html` byte for byte from the branch.
- `node --test web3d/test/` with `three` resolvable (see 4): 350 pass, 0 fail.

## Findings

1. **High, open: a new deal inside a sitting does not fold the panel.**
   `trackerUnfolded` is reset only in `resetTable`, which runs for a new sitting
   and a restore. The next deal of a Tour, the void-deal redeal and the throw-in
   redeal are all `deal_start` events handled in `advance`/the event switch
   (`main.js` around `case "deal_start"`), none of which touches the flag, so a
   panel the person opened stays open into the next deal. This breaks 6.6.2 ("at
   the start of every deal") and the request ("always closed"). Fix: set
   `trackerUnfolded = false` in the `deal_start` case (and let `showTracker`
   follow at the rest). Add a browser check: open the panel, play a deal out (and
   a throw-in/void seed), assert the tab reads `aria-expanded=false` and no
   `.tracker` at the next deal's first rest.
2. **Low: language switch and Hints off/on keep the fold.** They call
   `showTracker` without resetting, so the state is the person's for the deal.
   That is consistent with "folded at every new deal" and is probably right; say
   so in 6.6.2 or leave it. No code change needed.
3. **Low: the unit test is weaker than it looks.** `trackerOpen` no longer takes a
   window size, yet `tracker.test.js` still passes `size` and loops over three
   sizes; the loop proves nothing about size. Drop the argument and the loop
   (keep the stale-pref and `unfolded` cases). The window-size independence is
   carried by the browser check, which is correct.
4. **Low: the browser check's fresh page is not driven to a rest.** In
   `check_tracker_fold_and_settings` the `fresh` page is read right after
   `tracker_page(..., unfold=False)`, which does drive and rest, so it is fine;
   but nothing checks the second deal (finding 1). Covered by the fix there.
5. **Low: TABLE3D 6.8 line is over-long** (`tracker_fold_and_settings` line wraps
   past the others). Rewrap. Also 6.6.8 still says the unit test covers "the
   fold"; it now covers only that the panel is folded unless unfolded, which is
   accurate enough.
6. **Info: environment.** In a fresh worktree `node --test web3d/test/` fails 22
   tests with `Cannot find package 'three'` (from `kit/js/materials.js`) after
   `npm ci --prefix web3d`, because `node_modules` is under `web3d/`. Not caused by
   this change; resolved by a temporary `kit/node_modules` link.

## Fixes

- Finding 1 (high): `trackerUnfolded` reset in the `deal_start` case; `tracker_fold_and_settings` now plays to the next deal and checks the tab and panel are folded (it failed before the fix) (4fb0163).
- Finding 3: the `tracker.test.js` loop and size argument dropped (4fb0163).
- Finding 5: the TABLE3D 6.8 line rewrapped, and says the next deal starts folded (4fb0163).
- Findings 2, 4, 6: no change.
