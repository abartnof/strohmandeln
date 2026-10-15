# Review: worktree-agent-a3d159967b6cc00a5 (table fixes + browser suite)

## src
- OK web3d/src/main.js:177-194 slate timer: id set, delete-on-fire returns true only if not cancelled, cancel() removes it; matches slate.js:162/239-257 (later/cancel contract; ids start at 1, slate tests `timer !== null`). Cancelled fold is a no-op on the director side. Correct.
- OK style.css:18 `:where(#overlay > *)`: zero specificity so .slate-root:not([hidden]) / .settings-root / .banner / .top-right `pointer-events:auto` now take effect (were dead behind the id rule). Hidden roots stay none via style.css:29. No class that should stay inert gains auto (.tags/.floats/.say-layer/.dialogs set none explicitly).
- OK hooks: window.strohmandeln3d only under TEST (seed|test) or ?hook (main.js:1231); `manual` is exposed only inside that object (main.js:150).
- LOW web3d/src/main.js: no Node test covers the new cancel path (CLAUDE.md: test first). Add a slate/main-level test: Tour-end open({ms}) then sitting-end open() must stay open past ms.

## browser.py / scenarios / aids
- MED browser_aids.py CHECKS_104 vs TABLE3D 10.4: missing slate_reconstruction_note, talk_timing, opponent_unnamed, tutor_mute, hint_and_explain, deal_review, strings_complete, single_file (and talk_cross_group is named check_talk_repeat). If the tutor's are meant to live elsewhere they are not registered anywhere either: bin/browser-full would report "ok" while section 10 is only partly covered. Add them or mark them open in PLAN.md; consider a test that every backticked name in TABLE3D 10.x has a registered check.
- MED browser_aids.py:~99-110 slate_open_close: the spec says auto-open at Tour end AND at the extension, never during the count. Only Tour end is waited for; "never during play" is sampled only at rest points with phase=="play" (not during the count, not mid-motion). Extend the sampler to phase "count"/last-trick and assert open at the extension prompt.
- MED browser_aids.py talk_events: `if any(e["type"]==event ...)` makes three of four rows conditional (vacuous if the event never occurs); only sitting_start is guaranteed. Assert the seeded sitting contains every event in `wants`, else fail.
- LOW browser_aids.py slate_rows: loops over `scored`/`credits` pass on empty lists; the only guard is `rows.count() != len(scored)+1`, which is 1==1 for an empty log. Add `fail_if(not scored, ...)`.
- LOW browser_scenarios.py motion_profile: dead line `take_if_bidding(page) if False else None`; delete.
- LOW browser_scenarios.py keyboard_sitting: now keys for the first deal + extension prompt only; spec 10.3 says "a whole sitting by keys". Docstring justifies it, but TABLE3D 10.3 should be amended (or PLAN open item) so the plan and the check agree.
- INFO NO_DRAW: scenario open_page/with_prefs default to no draw calls; correct for state/DOM checks. Pixel readers (language_switch's canvas screenshots) pass nodraw=False; any future canvas_image use via scenarios.open_page would compare blank frames (equal => spurious "no change" fail, or spurious pass for `==` asserts). Consider asserting NO_DRAW is off in canvas_image().
- OK test hooks are test-only; nothing in the drivers needs a non-gated hook. JS drivers (DRIVE_JS, LEGAL_JS) bound loops (limit; 40000 spins) and throw; but spins at 5 ms = ~200 s worst case per step with no overall cap besides the shell timeout.
- LOW legal_only: spec says 200 positions; check passes at >=150 (browser_scenarios.py:~600). Raise to 200 or amend the spec. (Guard against vacuity exists.) farewell_once asserts exactly one farewell: OK.

## bin/browser-full
- OK exits 1 on any failure/timeout (124), 2 on missing Playwright or stale build; an unknown/empty check name raises KeyError in browser.py -> non-zero, so "zero checks" cannot report ok when names are listed.
- MED bin/browser-full: no explicit zero-check guard: `bin/browser-full ""` or a names list that prints empty yields one iteration with `--only=` (KeyError -> FAIL), fine, but a future change making an empty list loop zero times would print "ok (0 checks)". Add `[ ${#list[@]} -gt 0 ] && [ -n "${list[0]}" ] || exit 2` and require pass>0.
- MED timeout kills only the Python process (`timeout` TERM); Chromium/Playwright node driver children can be orphaned and keep eating the 2 CPUs. Use `timeout -k 10 ... ` and run under `setsid`/kill the process group, or `pkill -P`.
- LOW a check that passes in browser.py with zero assertions still prints ok; consider having browser.py count fail_if calls per check and fail if 0.
- LOW `grep -E "^(FAIL|  )"` after a failure: browser.py prints failures as `       f"` (7 spaces) so matches; but a Python traceback (check crash outside run()'s try, e.g. KeyError) matches neither -> no reason shown. Print `tail -5` as fallback.
