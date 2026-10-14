# Review: kit/, tarockfaces.js, contact_sheet.mjs, bin/gate

Reviewer CR-KIT. Scope: `kit/` (js, css, build.py, tests), `web3d/src/tarockfaces.js` and its
tests, `tools/contact_sheet.mjs`, `bin/gate`. Cassino compared at `73f0cd7` (its working tree has
no tracked change). `npm ci` in `web3d/` done; the gate passes (134 Rust, 199 Node, 25 Python,
21 phrase bank) before and after the fixes.

## Findings

| # | Area | Problem | Evidence | State |
|---|---|---|---|---|
| 1 | Vendoring | None found. Every file STAMP calls `verbatim` (13 js, 6 tests) is byte-identical to Cassino's after the header (tests: `../src/` to `../js/`); every line of `css/hud.css` occurs in Cassino's `style.css`. Changed files (`units`, `reveal`, `director`, `faces.test`) differ by exactly what STAMP says; the `ZONES*`, `COURTS*`, `FIGURE*` removals are whole blocks. | diff of each STAMP row against `git show 73f0cd7:<path>` | ok |
| 2 | STAMP | Accurate for what it lists, but nothing stopped a kit file from being unlisted or a "verbatim" file from losing its header. | `bin/gate` only checked listed files exist | fixed (gate checks both) |
| 3 | director | `choreography` undefined gave `Cannot destructure`; `handEdge` of an empty hand with neither `zones` nor `view().zones` gave `undefined[...]`. | read of `createDirector`, `handEdge` | fixed, tests in `director.test.js` |
| 4 | engine | `loadWasm` checked `<prefix>_alloc/_out/_out_len` but not `memory`, which README requires. | `engine.js` | fixed, test |
| 5 | Seams | Second-game fit is good: director (choreography, zones, card count), `loadWasm(bytes, prefix)`, `defineGame` (validated: categories, duplicates, target, line, integer scale; `line` output is checked per call), `createHud`, `countSchedule`, `kit_styles` all take everything game-specific as arguments, and the tests include a non-Cassino game (thirds, 36 of 70). Piquet could use them without editing kit files, with the caveats in 6, 7, 8. | `fixtures/games.js`, `counting.test.js` | ok |
| 6 | Seams | The HUD fixes its words, goal text and category labels when `defineGame` runs; there is no way to change language on a live HUD. The en/de switch must call `defineGame` and `createHud` again and `reset` with the ledgers. Works, but undocumented. | `hud.js` `words`, `game.goal`; only `reset(given, names)` changes names | open (document, or add `hud.relabel(game)` when the table is built) |
| 7 | Seams | Cassino leftovers: `HAND_H` (six categories), `listHeight`'s default `hand = HAND_H`, `segmentState`'s default `end = 21`, the director's default `cards = 52`. Harmless (the HUD passes the game's values) but misleading, and tests pin `HAND_H`. | `scoring.js`, `director.js` | open, minor |
| 8 | Seams | `handEdge` and `screenPoint` look for zones literally named `your-hand` / `their-hand`; the HUD is two sides only (`you`/`opp`). Documented in README; a three-hand game would need a change. | `director.js`, `scoring.js` `read()` | open, by design |
| 9 | `three` | The symlink is tracked (mode 120000, `.gitignore`'s `node_modules/` matches directories only). Safe in worktrees (relative link; each worktree needs its own `npm ci`), a fresh clone (dangling until `npm ci`; the gate says so) and GitHub Pages (only the built page is served, it contains no link). It is **not** safe on Windows without `core.symlinks`/developer mode: the link becomes a one-line text file and `kit/node_modules/three` fails. | `git ls-files -s kit/node_modules` | open, see recommendation |
| 10 | `three` | README said either mechanism is enough. They are not redundant: `NODE_PATH` is honoured by esbuild only; Node's ES module resolver ignores it, so the Node tests depend on the symlink. | Node ESM resolution rules; `modules.test.js` | fixed (README) |
| 11 | build.py | The wasm embeds absolute source paths of panics (cargo registry under the user's home, the repository, the toolchain), so `--check` would differ between machines, users and CI, while everything else in the build is deterministic. | cargo behaviour; no `--remap-path-prefix` | fixed (`rust_env`, test); verify on the first real crate that two checkouts give equal bytes |
| 12 | build.py | Only the bundle and worker were checked for `</script`; a JSON asset (`words.json`), a text asset or the stylesheet containing it or `</style` broke the page. | `make_page` | fixed (all values checked; JSON writes `<\/`), tests |
| 13 | build.py | `read_text()` without an encoding (Windows: cp1252) and `write_text` (Windows: CRLF, so the committed page and `--check` disagree). | `build.py` | fixed (UTF-8, bytes), test |
| 14 | build.py | A missing or failing esbuild/cargo raised a traceback; a crate that built no `.wasm` raised `FileNotFoundError`. | `bundle`, `build_wasm` | fixed (`run_tool`), test |
| 15 | build.py | `node_modules/.bin/esbuild` does not exist as such on Windows (`esbuild.cmd`). | `bundle`, `dry_run` | open (Windows is unsupported anyway, see 9) |
| 16 | build.py | Output size cannot be measured yet (no `game.toml`). Expectation: three.js minified for the modules the kit imports, roughly 0.6-0.8 MB; wasm base64 is 4/3 of the module, so a 300 KB engine costs 400 KB; assets and fonts none. Well under the 5 MB guideline unless card art is added, and the weigh-in names each part. | `make_page` | open, measure with the first page |
| 17 | faces | Numerals, subtractive forms (IIII only), nicknames, 54 codes, pip counts (1 2 3 4 7 8 9 10 = 1 2 3 4 7 8 9 10) are right and tested; contrast formula is WCAG (checked 21:1 for black/white); ink 15171c on paper/field/band and red c8202c on paper pass 4.5:1 in the test. | `tarockfaces.test.js` | ok |
| 18 | faces | Pip bug: the draw call enlarged glyphs by 1.25 but `pipLayout` and its bounds test used the unscaled size, so the test passed while a 4's right-hand pips reached the card edge. The 2/3/4 layouts also sat to the right of the body's centre. | `docs/img/faces.png` before the fix | fixed: sizes are the drawn sizes, columns symmetric about the body, test measures ink (`PIP_INK`) and spacing; image regenerated |
| 19 | faces | No language hook for courts. | PLAN, "faces swap court labels" | fixed: `drawFace(ctx, w, code, lang)`, `faceTextures({ lang })`, `COURT_LABELS` (K Q C J / K D C B), `courtLabels`, `COURT_CODES` (the 16 faces to redraw on a language switch); tests that only courts change |
| 20 | faces | Cost on a phone. Drawing is cheap (a few dozen `measureText` calls a face; some repeat per face and could be cached). The real cost is memory and upload: 54 textures at 512 px wide are 54 x 512 x 717 x 4 B = 79 MB, about 105 MB with mipmaps; at 384 px 60 MB; at 256 px 26 MB. All are uploaded on the first render if the table creates them eagerly. A card spans about 130-150 CSS px on a phone. | arithmetic, `faceTextures` | open: use 384 or less, build faces in idle slices or on first use, dispose canvases after upload, measure on a device |
| 21 | faces | Text fitting is sound (condense to 0.62, then shrink; test walks transforms and asserts every index inside its strip with the margin). Fonts are system fonts only, so metrics differ by platform; the fit adapts but the contact sheet shows one platform's (WebKit on Linux). | `fitText`, test | ok |
| 22 | contact_sheet | Hard-wired to `~/piquet/.venv` Playwright WebKit and `~/` paths; unusable on another machine, and the sheet always draws English courts. | `contact_sheet.mjs` | open, minor (an optional lang argument is a one-liner when needed) |
| 23 | gate | The page check was skipped when `web3d/strohmandeln3d.html` was absent, even once `game.toml` existed: the one moment it must not skip. | gate "page not built yet" branch | fixed |
| 24 | gate | `node --test` that finds no files, or a Rust run whose output is not recognised, passed with 0. | gate | fixed (zero fails) |
| 25 | gate | Under `set -euo pipefail` a diagnostic `grep` matching nothing ended the gate before the `FAILED` line (still non-zero, but silent). | gate | fixed |
| 26 | gate | Failure kinds checked by reading and by one run: missing STAMP entry fails with a message (run); missing `node_modules` or broken symlink fails (read); clippy/fmt/test fail (read); stale page fails (`build.py --check`, tested in Python). Rust skip is only "no Cargo.toml"; clippy runs on all members of the virtual workspace. Not caught: the wasm crate failing to build for `wasm32` is found only through the page check, so it needs `game.toml` to exist before the crate does. | gate | ok, with the note |
| 27 | gate | A cold gate takes 3-4 minutes (compile, `opt-level = 2` property tests); warm about 1 minute. | timing | open, note |
| 28 | Simplicity | `kit/` carries `HAND_H`, `defaultNames` etc. (7) and `css/hud.css` embeds Cassino-specific provenance comments; otherwise no dead code found. `kit/package.json` is needed for ES modules in `kit/js`. | read | ok |

## Recommendation on `three`

Keep the symlink for now, keep `NODE_PATH` (they cover different tools, finding 10), and
decide before the first Windows contributor or CI on Windows: replace both with a root
`package.json` using npm workspaces (`["web3d", "kit"]`). Then `npm ci` at the root puts one
`node_modules` above both folders, every resolver (Node, esbuild, editors) finds `three` by
walking up, there is nothing to commit that Windows cannot check out, and `NODE_PATH` goes. The
cost is moving the lockfile to the root and pointing `bin/gate`, `build.py` (esbuild path) and
the README at it. If Windows is never a target, the current arrangement is fine.

Also worth adding with it: a `.gitattributes` (`* text=auto eol=lf`) so a Windows checkout does
not change the bytes `--check` compares.

## Verdict

The vendoring is faithful and the seams are real: a second game can use the director, engine
wrapper, scoring definition, HUD, count timing and builder without editing kit files, and the
tests exercise a non-Cassino game. Defects found were in the margins (error paths, the build's
reproducibility across machines, the pips, gate skip paths) and are fixed with tests. Open: a
language switch on a live HUD (6), Cassino defaults left in `scoring.js`/`director.js` (7), the
Windows story for `three` (9, 15), the faces' texture budget on a phone (20), the first real
size and cross-checkout wasm determinism measurements (11, 16), and the contact sheet's
machine-specific paths (22).
