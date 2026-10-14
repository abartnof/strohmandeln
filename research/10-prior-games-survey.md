# Survey of the three prior card-game projects (piquet, bezique, cassino)

Read-only survey, 2026-10-14. Paths are under `~/piquet`, `~/bezique`, `~/cassino`. Line counts are `wc -l` at the time of reading.

## 0. Headline findings

1. **No modularization has happened.** There is no shared crate, no common library, no submodule, no npm workspace. Each repo is a full copy-and-adapt of the previous one. This was a deliberate, documented decision, deferred until the third game:
   - `~/bezique/PLAN.md:155` "Copy and adapt; no shared crate until a third game" (`docs/DESIGN.md` §3, line 73: "A common crate or package would couple the two projects' lifecycles for a small shared surface").
   - `~/cassino/PLAN.md:303` "Copy and adapt, with origin headers; a shared package after the third table".
   - `~/cassino/docs/DESIGN.md` §3 (lines 57-88): "Copy, do not share, for now. ... a diff across the three repositories shows which files stayed identical, and those files are the package. Every copied file starts with a one-line header naming its origin (`// From piquet web3d/src/kinematics.js @ 254cb3c.`), so that diff is easy to take."
   - `~/cassino/PLAN.md:484` (What could come next): "The shared package across piquet, bezique and cassino, once the third game shows what they truly share."
   - `~/piquet/docs/DESIGN.md:45` only says the engine is organised "so that future card games can reuse the parts".
   The diff has been planned but not yet taken. Section 5 below is that diff.
2. **Bezique has no web3d table at all** (no `web3d/`, no `bin/gate`, no `CLAUDE.md`, never published to Pages; its milestone 9 "The 3D table, adapted from piquet's" is unchecked, `PLAN.md:202`). So the "third table" is cassino's, and the real web3d lineage is piquet -> cassino. Bezique contributes Rust-side parts (rng, tournament, the follow-suit trick logic, the two-pack card model).
3. **Cassino is the most evolved and is the best base for nearly everything** in `web3d/` (it fixed/improved ~15 of piquet's files). Piquet keeps unique things cassino lacks: the celebration scenes (2,708 lines), the larger overlay, the Python oracle/golden vectors.
4. **Only cassino has a `CLAUDE.md` and a `bin/gate`.** Piquet's working rules live in `PLAN.md` "Working conventions" (lines 197-218); bezique's in `docs/DESIGN.md` §2 and PLAN. Cassino's `CLAUDE.md` (not the gate script itself) references `~/piquet/.venv` for Playwright.
5. **Card art is a licensing wrinkle for Strohmandeln**: the CC0 deck (AustinGabriel64) is a 52-card French-suited deck. A Tarock deck needs 22 trumps (I-XXI + the Sküs/Excuse) which this deck does not have. Cassino already solved a similar problem (12 Spanish court images cut from a c.1760 scan, `web3d/tools/courts.py`, `CREDITS.md`). Trump art will be new work or a new licensed source.

## 1. Piquet (`~/piquet`) - the origin, finished, public

Repo: `github.com/abartnof/piquet`, public since 2026-10-01, 298 commits. Play: `abartnof.github.io/piquet/`. MIT (`LICENSE`).

### Architecture
- Cargo workspace (`Cargo.toml`): `piquet-core` (engine, ~7,500 lines in src + `src/bin/` measurement tools ~1,400), `piquet-cli` (`main.rs` 754, `render.rs` 296), `piquet-wasm` (`src/lib.rs` 756). rust-version 1.97, release profile lto, codegen-units 1. No runtime deps in core.
- Core modules (PLAN.md "Where we are"): `cards.rs` (32-card pack, Hand as u32), `combos.rs`, `declarations.rs`, `rules.rs` (Deal as an immutable state machine), `scoring.rs` (ordered event log), `partie.rs`, `chances.rs`, `observation.rs` (`View`, the only way an agent sees a deal), `agents.rs`, `heuristics.rs` (ladder rungs 1-4), `style.rs`, `inference.rs`, `prior.rs`, `solver.rs` (exact endgame, 1,017 lines), `play.rs`, `table.rs` (1,203; the session: Prompt/Action/Event for any client), `options.rs`, `opponents.rs` (named roster), `tournament.rs` (mirrored-pair duels, Bradley-Terry), `mt19937.rs`, `rng.rs`.
- Also kept a Python oracle (`python/`, `vectors/` golden vectors) from which the Rust port was verified; `bin/vm` and `bin/partie-sequential` are VM helpers (it was developed on a GCP VM; see `docs/VM.md`). Cassino/bezique dropped the oracle in favour of brute-force references in tests.
- **Wasm interface/protocol**: no wasm-bindgen. Hand-rolled C ABI: JSON state out, one-line text commands in via a shared buffer (`docs/PROTOCOL.md`, 267 lines; state includes `prompt`, `aids`, `hint`, events). Same pattern in all three games. The page "holds no rules" (a standing rule). `crates/piquet-wasm/tests/dull-parties.txt` holds 15 scripted parties with state hashes; both `cargo test` and `web3d/test/wasm.test.js` must reproduce it (`PIQUET_BLESS=1` rewrites) - this is the native-vs-wasm parity check.
- **web3d**: `src/` ~8,270 lines JS+CSS at top level [review: was ~9,170; `find src -type f | xargs cat | wc -l` = 11,025 including celebrate/ and index.html] (see §5 for per-file sizes), plus `src/celebrate/` (13 files, 2,708 lines; 7 scenes in `scenes.js`: plates, pong, cupball, disco, parade, macarena, people; the rest is `runner`, `kit`, `physics`, `body`, `hud`: cupball, disco, macarena, parade, plates, pong, runner, people, physics...). `docs/TABLE3D.md` (1,130 lines) is the design + phase record P0-P21. `tutorial.md` (116 lines) is inlined as text at build.
- **build.py** (160 lines): compiles wasm (`cargo build --release -p piquet-wasm --target wasm32-unknown-unknown`), bundles `src/main.js` with esbuild (`--bundle --minify --format=iife --target=es2022 --loader:.md=text --legal-comments=eof`) from `node_modules` (three, @material/web, lit; pinned by `package-lock.json`), then fills placeholders in `src/index.html` in ONE regex pass (`/*STYLE*/`, `/*APP*/`, `__WASM_BASE64__`, `/*ART*/`, `/*WORDS*/`) with base64 wasm, WebP data-URI art, `words.json`. Prints a size table; warns over 5 MB (guideline, not wall). Aborts if the bundle contains `</script`. `--art svg` makes an alternate vector-art build (git-ignored). Output `web3d/piquet3d.html` is **committed** (3.1 MB). Index.html at repo root meta-refreshes + `location.replace` to it, preserving `?query#hash`.
- **Gate**: no script; PLAN.md:154 says `cargo fmt --check`, `cargo clippy --all-targets --release -- -D warnings`, `cargo test --release` (~3.5 min on the 2-core VM), plus node tests (`cd web3d && npm test`), plus Playwright `web3d/test/browser.py` (933 lines).
- **Tests**: 207 Rust `#[test]`; 22 `*.test.js` node test files in `web3d/test/` [review: was 24; the dir also holds browser.py, partie.js] (bag, breaks, cards, celebrations, choreography, dialogue, drag, easing, faces, hand, kinematics, layout, leftovers, scorebug, speech, staging, surfaces, talk, timeline, tutorial, wasm); `web3d/test/browser.py` uses Playwright (python, in a `.venv`) driving system Chromium on SwiftShader from a `file://` URL, blocks all network, plays a partie by clicking.
- **GitHub Pages**: serves `main` root; `.nojekyll`; `index.html` forwards. Workflow: develop on `dev`, merge to `main` only when playable and gate-passed, since any `main` commit that rebuilds the html goes live (PLAN.md:100-130). No GitHub Actions; the built html is committed.

### Card physics
Files: `web3d/src/kinematics.js` (365; cassino's later version is 458), `easing.js`, `timeline.js`, `director.js` (render-on-demand loop), `choreography.js` (732), `layout.js` (270), `hand.js`, `cards.js` (rounded-slab geometry, 3 material groups: face/back/edge), `materials.js` (toon shading), `deck.js` (meshes + ink hull outlines), `scene.js`, `framing.js` (camera for any window), `units.js` (card/table sizes, camera, zones), `surfaces.js` (procedurally drawn table tops). `layout(state)` gives the at-rest pose of every card; `choreograph(prev, next, ...)` turns engine events into a timeline between two true layouts. Primitives in `kinematics.js`: `pose`, `lying`, `cardCorners`, `transfer`, `layDown`, `pull`, `toss`, `rise`, `bob`, `pickUp`, `flip`, `flipPile`, `slide`, `fan`, `still`, `chain`, `beforeFlip` [review: the earlier list wrongly included `peelOff`, `row`, `carryBlock`, which exist only in cassino]. Easing: `minimumJerk`, `snap`, `flick`, `friction`, `gravity`, cubic-bezier, `M3` tokens. Cards are tossed on a gravity parabola, land on a leading edge tilted 10 degrees, and settle hinged; flips go along an edge that stays on the table. Hover = card lifts toward the player and ink thickens (raycasting; `docs/TABLE3D.md` §10). **No drag-and-drop of cards in any of the three games**: interaction is click/tap + keyboard; piquet's `drag.js` (18 lines) is only for a draggable floating panel on phones; TABLE3D.md lists "drag a card to the table" as "later".

### Card assets and licenses (`CREDITS.md`, 116 lines)
- Faces: "Public domain complete playing card deck" by AustinGabriel64, Wikimedia Commons, **CC0** (32 piquet cards used), rasterised by `web3d/tools/art.py` (rsvg-convert + Pillow) to WebP; sources pinned with checksums in `web3d/art/source/`.
- Back: "Reverso baraja espanola" by Germarquezm, **CC BY-SA 3.0**, re-framed to 5:7, so the adapted back is itself CC BY-SA 3.0 and is **credited on screen** as well as in CREDITS.
- "Large Text" faces (default on phones): project's own, drawn at load time in the device fonts (`faces.js`), no font bundled.
- Material Design 3 (Google), `@material/web` 2.5.0 (Apache-2.0), Lit (BSD-3), three.js 0.186.1 (MIT), Material Symbols (Apache-2.0, paths copied, no icon font). Build tools esbuild, librsvg (LGPL, not shipped), Pillow.
- Rule: every third-party asset credited in `CREDITS.md` **in the commit that brings it in**.
- Art budget: `web3d/art/cards` 1.3 MB; page 3.1 MB total (art 1.6, JS 1.0, wasm 0.4).

### Banter / narrator / table talk
- Bank: `web3d/tools/phrases.py` (331 lines) -> writes `web3d/words.json` (`{groups: {id: [key...]}, texts: {key: words}}`; 309 groups, 420 texts) and `docs/PHRASES.md` (269 lines; generated, "edit the bank, not this file"). Each phrase carries a source letter. `--check` fails if either output is stale (a Node/py test, `tools/test_phrases.py`).
- Runtime: `speech.js` (174; maps game events to `{who, phrase, delay, vars}` lines), `talk.js` (130), `dialogue.js` (84; schedules lines on the animation's clock, `GAP`, `TURN`, `saying(words)=250+65*len` ms), `bag.js` (33; shuffle-bag so no wording repeats twice running and the first of a refilled bag is never the last heard), `breaks.js` (pause between declaration rounds). Words go in speech-balloon boxes beside the speaker's hand. Voice synthesis (Piper recordings) was built then **removed entirely** (PLAN.md:121); "the page makes no sound" is a standing rule.

### Tutor / explain / hints
- Hints come from the engine (`hint` in state; `hint.command`, `hint.cards`; PROTOCOL.md "Hints": the top ladder rung/exact solver in the endgame, same answer when asked twice, changes nothing). **Explain** is a JS-side toggle (`prefs.explain`; overlay/main.js) that narrates facts over the cards. No brief/tutor in piquet: its `DESIGN.md` §8 says the tutor "fell back to naming which rung would play a move"; cassino built the real one.
- Tutorial: `tutorial.md` (116 lines; phases: Introduction, Exchange, Declarations, Tricks) + `tutorial.js` (101): pages opened at the first occurrence of each phase.

### AI opponent ladder
`heuristics.rs`: rungs 1-4 (+ `solver.rs` exact endgame as the top), `MAX_LEVEL = 4`; "Skill as a capability ladder, not a noise dial": each rung refines the one below; two proposed rungs were deleted for failing measurement. `opponents.rs` presents rungs as named opponents (Bess, Cotton...) in the engine, but UI text says "your opponent" never a name. Erraticism slips capability (not noise). `style.rs`: third axis, calibrated EV-neutral. `inference.rs`/`prior.rs`: infers opponent's possible hands (fitted conditional logit). 5 levels offered in the UI. Measured with mirrored pairs (`tournament.rs`; `measurements/`).

### Settings, phone, accessibility, offline
- Settings: Material dialog (`overlay.js`, 1,326 lines): aids (Explain, Hints, play-forced, play-winners, declare-for-me), undo, speed, table surface, sort, faces, Keys list, copy game record (seed + command list replays a game exactly), credits. Prefs in localStorage.
- Phone: `framing.js`/`units.js` have a touch/portrait camera and zones; Large Text faces on phones; floating draggable panel (`drag.js`); phone-specific notes in PLAN.md log (1-2 Oct).
- Accessibility: keyboard plays the whole game (arrow keys move a pointer, Space plays, Enter main action, U undo, H hint, Esc); prompt names the card in a polite live region; `aria-` attributes in overlay (24); Material components; reduced-motion collapses durations; Keys listed in Settings.
- Offline: single html, zero network requests (browser test blocks the network and fails on any), README tells users to save the raw file (`https://github.com/abartnof/piquet/raw/main/web3d/piquet3d.html`).

### Working rules (PLAN.md:197-218)
Test-driven; atomic commits, often, on `dev`, `main` only takes merges when playable; PLAN.md updated as work lands; `set -e` on multi-step commands; one self-contained html, offline, modest (5 MB guideline); Material Design 3 bundled, no CDNs/fonts fetched; "Your opponent", never a proper name; every aid a toggle; look is "clean, bright, a bit cartoonish, not childish"; page makes no sound; every third-party asset credited in CREDITS.md in the same commit; consult the maintainer before spending money or long compute, with an estimate. A new session reads PLAN.md, TABLE3D.md (§13 phase record), PROTOCOL.md, CREDITS.md in that order. "Hard-won lessons" table (PLAN.md:1212): mirrored pairs always; prefer measuring; beware optimizer's curse; a display that's prettier than typeable is unusable; a test that cannot fail is not a test; observation.rs is where leaks live (4 leaks found; ask of each View field "when was this said aloud, and by whom").

## 2. Bezique (`~/bezique`) - engine only, private, no table yet

Repo: `github.com/abartnof/bezique` (**private** since 2026-10-01), 120 commits, MIT. `.gitignore` already anticipates `web3d/node_modules`. Docs: `PLAN.md` (296), `PLAYTEST.md` (59), `docs/DESIGN.md` (640), `RULES.md` (293), `PROTOCOL.md` (205), `LITERATURE.md` (743), `docs/sources/` (Foster 1897, Cady 1896 text), `docs/reviews/` (three code reviews), `analysis/` (exact odds script), `tools/prng-reference/` (the authors' C code for the PRNG).

- **Architecture**: workspace `bezique-core`, `-cli`, `-wasm`. Core: `cards.rs` (527; 64 physical cards in a u64, twins interleaved, `type*2+copy`), `deal.rs`, `game.rs`, `hand.rs` (1,298), `melds.rs` (1,555; declaration rules as "one inequality on per-card class levels"), **`tricks.rs` (207; trick winner and the second-player must-follow/must-win rule for the endgame)**, `scoring` as event log, `observation.rs` (387; twin-symmetry leak property tests), `agents.rs` (889), `lookahead.rs` (633), `solver.rs` (516), `opponent.rs` (212), `race.rs`, `rng.rs` (358; xoshiro256** seeded by SplitMix64), `session.rs` (1,630; undo, replay, aids, hints), `tournament.rs` (471; `games_until`/`hands_until` with `Sequential` stopping rule), `words.rs` (183; period announcing formulae). CLI: `table.rs` 648. wasm `lib.rs` 401 with exports `bezique_alloc/new/send/state/state_len`.
- **Protocol**: JSON state + text commands (`docs/PROTOCOL.md`, 205 lines); aids `hints, play_forced, tracker`.
- **web3d, build, gate, Pages, browser tests**: none exist. Only a `PLAYTEST.md` for the terminal client.
- **Card model/assets**: `DESIGN.md` §3 plans to reuse the piquet pack twice ("33 WebP images (32 faces and a back), 1.6 MB").
- **AI ladder**: rungs 1-5 (`agents.rs`): legal; maxims of old books; memory + exact endgame (+19.2 pts/hand over rung 2); fights for the last stock trick (+1.8-2.5); rung 5 samples worlds near end of stock. Four **personas** (balanced, concentrates, close, bezique) as styles calibrated to within ~1.24 pts/hand; erraticism slips a rung per decision, repeatedly with same probability. Rung 6 (play for the game) not built.
- **Hints/aids**: engine `hint` with command; undo is a "learning aid" (undo replays same stock, so it can reveal; open question). No tutor.
- **Banter**: only `words.rs` (period formulae for announcements). No phrase bank, no dialogue timing.
- **Working rules**: no CLAUDE.md. `docs/DESIGN.md` §2: Rust from the first line, TDD, atomic commits to main, modular, plain data, engine holds rules, measure with mirrored pairs and sequential stopping (`DESIGN.md` §10.5, lines 529-545: a fixed |z| bound, 3 by default, not O'Brien-Fleming [review: the earlier text said O'Brien-Fleming and cited line 124, which is unrelated; OBF is cassino's `docs/DESIGN.md` §11.4]), public-facing docs.
- **Notable**: `PLAN.md` "To discuss": a period-marker-inspired score display (dials, counters) to replace piquet's broadcast score bug - a design discussion with the maintainer was meant to happen first.

## 3. Cassino (`~/cassino`) - the third game, best web3d, public

Repo: `github.com/abartnof/cassino`, 293 commits, Pages (root `index.html` forwards to `web3d/cassino3d.html`; **no `.nojekyll`**, unlike piquet - harmless but inconsistent). MIT. Page 4.0 MB. Not trick-taking (matching/capturing), so its engine rules are least reusable; its **table, tutor, and talk systems are the best**.

### Architecture
- Workspace `cassino-core` (~19,000 lines incl. `src/bin/` measurement tools ~1,100 [review: 19,038 counted; was ~21,000]), `cassino-cli` (`main.rs` 162, `table.rs` 507), `cassino-wasm` (`lib.rs` 1,814, `tests/smoke.mjs`, `tests/golden.txt` golden games). Notable core: `session.rs` 2,271, `lessons.rs` 1,658, `moves.rs` 1,586, `brief.rs` 1,094, `learner.rs` 995, `tutor.rs` 943, `review.rs` 729, `advice.rs` 705, `words.rs` 633, `knockout.rs`, `search.rs`, `counter.rs`, `solver.rs`, `tournament.rs`, `rng.rs` (copied from bezique @1b2179e). `rust-toolchain.toml` pins Rust. `[profile.test] opt-level = 2` for statistical tests.
- Wasm exports (docs/PROTOCOL.md "WebAssembly", lines 265-298), no wasm-bindgen: `cassino_new`, `_watch`, `_alloc`, `_send`, `_step`, `_offer`, `_hint`, `_evidence`, `_evidence_of`, `_learner`, `_brief`, `_brief_of_record`, `_set_focus`, `_nudge`, `_reveal`, `_restore`, `_render`, `_out`/`_out_len`. Same alloc/send/out buffer pattern as the others. Seed + record fix a game exactly; saving/restoring a sitting is a text record with `RECORD_VERSION`.
- `web3d/src` ~10,290 lines [review: 10,288 counted; was ~11,240] (see §5). `build.py` (196): same as piquet's plus `courts()` (12 court WebP) and a second esbuild bundle for the **tutor Web Worker** (`tutorworker-entry.js`, inlined as `<script type="text/plain" id="tutor-worker">`, run from a Blob URL), `--check` flag that builds in memory and fails if the committed page differs (used by the gate).
- **Gate** (`bin/gate`, 60 lines): `cargo fmt --all --check`; `cargo clippy --all-targets -- -D warnings`; `cargo test`; then if `web3d/node_modules` exists: build wasm, `node --test test/`, `python3 web3d/build.py --check` (committed page not stale), and a git check that a rebuilt-but-unstaged page fails when sources are staged. Prints one line `gate: ok (N tests passed + M in Node)`, exits non-zero on failure.
- **Tests**: 370 Rust `#[test]`; 29 `*.test.js` node test files [review: was 31]; `web3d/test/browser.py` (2,136 lines; `--quick` ~1 min = loads w/o console error + plays a few moves; full ~15 min on a big machine, over an hour on the 2-core VM) with about 40 named checks (offline, drawn, faces known-only, badges, trackers, new game, welcome, aid toggles, cheers, talk, ending, review, tutor, focus, assist, move bar, tutorial, phone, tablet, desktop frame, resize, rush, lost sitting, access, settings...). `?manual` mode steps the table's own clock for frame-by-frame strips; `?test`/`?speed=100` for instant.

### Card physics
Superset of piquet's: `kinematics.js` 458 (adds `snap` easing in hand moves, `peelOff`, `row`, `carryBlock` which moves a whole captured heap as one rigid block; `pickUp`, `flipPile`, `beforeFlip` are already in piquet's), `choreography.js` 767 (event table in `docs/TABLE3D.md` §6, lines 150-283: cut, dealt in twos, played/trail/build/capture "gather", swept, residue, scored), `layout.js` 307 (`ZONES`, `ZONES_PORTRAIT`, `ZONES_TOUCH`; grid placement; `handOrder`/`tableOrder` with sort), `timeline.js` 88 (adds `moved` flag), `easing.js` 114 (adds `snap`, `flick`). Rules: every movement starts with a jerk and eases into place; tests hold each path to its profile and every corner of every card above the table. Cards at rest use deterministic `jitter(key)` to look hand-placed. Opponent has a `think` pause and `look` pause with `marks` lighting cards it will take.
- Ending reveal (`reveal.js`, 74): camera pulls back and the opponent is revealed as a court card standing across the table (`COURTS`, 12 images); one-line last word.
- Selection (`selection.js`, 299, pure): choose hand card, pick table cards, engine returns the "offer" (what the selection makes) as chips; **the interface never guesses**.

### Card assets
Same CC0 deck + same CC BY-SA 3.0 back as piquet, all 52 faces now (`art.js` expects 53 images), plus 12 courts cut unchanged from a Spanish-suited pack scan c.1760 (see `CREDITS.md` lines ~50-60, `tools/courts.py`). Art 1.5 MB. Icons from Material Symbols (7, paths in `chrome.js`/`overlay.js`).

### Banter (richest)
- `web3d/tools/phrases.py` (525) -> `web3d/words.json` (123 groups, 770 texts) + `docs/PHRASES.md` (856 lines, ~30 source letters mapped to period books, e.g. D, F, Lo, P, Ec). Same schema as piquet. Each frequent moment has 5+ wordings; every phrase has a source letter ("T" = the table's own).
- Runtime: `talk.js` (409; event followers `followBuilds`, `followPlay`, `scoreSaid`), `dialogue.js` (188; adds `SLACK=250` and `LATE=2000` so chatter yields to calls and never arrives late, `choosePlace` for phone placement, `chunk`: one speaker's lines at one moment share a box), `announce.js`, `bag.js`.
- Design (`docs/TABLE3D.md` §7, lines 283-411; research `research/13-table-banter.md`, `05-table-talk-terminology.md`): setting "Table talk" = all / calls only / none; **metatextual** triggers (long time without a build -> a rib); **follow-ups** of what was said ("You did say if I could take it!"); score remarks per hand; "kind: nothing belittles anyone" with a unit test keeping a list of unkind words out of the bank; "nothing said claims a card the speaker cannot be known to hold"; **nothing said during scoring** (popups do the work); a box on a phone never covers cards or the move bar.
- Opponent is "your opponent", no name; opponent speaks one last line at game end.

### Tutor / explain / hint / brief (the best)
- Engine: `advice.rs` (hints/explanations), `tutor.rs`, `lessons.rs` (one rule of <=15 words per skill), `learner.rs` (knowledge tracing over stored game summaries to choose a focus skill), `brief.rs` (the end-of-game review: <=3 bullets: one strength, one skill to work on with this game's figure and costliest miss, how-it-was-worked-out folded), `review.rs`, `reference.rs`. `DESIGN.md` §12.4 (explanations: three layers Fact / Interpretation / Hint, each a toggle; evaluation decomposed by category; move quality only on request; opponent's moves explained too; never reveal the opponent's hand) and §12.8 (the tutor, skill graph with prerequisites).
- JS: `progress.js` (192; last 30 games stored in `cassino.progress`, all storage access try/catch, import/export JSON), `review.js` (15), `tutorworker.js` (128) runs the heavy engine calls (`evidence_of`, `brief_of_record`) in a Web Worker with a main-thread fallback (`?noworker`), `tutorial.js` (153) + `tutorial.md` (180 lines; teaching ladder; pages open at first occurrence of each concept), `aids.js`, `badges.js`, `decision.js`.

### AI ladder
Four rungs (`DESIGN.md` §11): Legal; Greedy; Counter (remembers all cards, samples opponent hands honoring build announcements); Searcher (sampled worlds + exact solver in last deal). Skill dial in halves 1-4 (`prefs.js` `SKILLS`: "plays any card it may" ... "plays the deal out in its head"); half-steps are erraticism; `Opponent{level, erraticism, seed}` where every decision is a **pure function of seed and view**, so a record replays identically and undo only restores position (`opponent.rs`). Style (aggressive builder...) EV-neutral calibration. Each rung must beat the one below in a mirrored measurement or be deleted (one design was).

### Settings (`prefs.js` 259; versioned localStorage `cassino.prefs`, defaults carry a VERSION so changed defaults are not mistaken for choices)
Rules (game, aces14, sweeps, raising), skill (1..4 in halves), speed (Leisurely 0.6 / Natural 1 / Brisk 1.7 / Instant 100), surface, faces ("auto"/classic/jumbo = Large Text), match (single / World Series best of 7: `series.js`), aids (hints, explain, play_forced), trackers + open/folded, buildValues badges, unseen, sweepWarning, tutor, sortTable, **leftHanded** (move bar mirrored), talk, tutorial, seen pages. URL params override (`?speed=`, `?seed=`, `?ending`, `?manual`, `?noworker`).

### Phone layout / accessibility / offline
- Phone: `CAMERA_PORTRAIT`, `ZONES_PORTRAIT`, `PORTRAIT_BELOW=0.85` aspect switch; `PITCH=70` eye; the HUD/overlay measure their own strips and the camera framing reserves them (`framing.js` `strips`); the move bar sits above the hand; Large Text faces auto on phone and tablet; hand drawn as a row not a fan on phones; dialogue placement avoids cards; tested in headless Chromium for phone upright/sideways, tablet, tall window (not on real devices: PLAN.md "What could come next").
- Accessibility: keyboard path (listed in Settings), `aria-label` on canvas, aria-live prompt, 34 aria uses across `chrome.js`, `hud.js`, `overlay.js`; `check_access` and `check_focus` browser checks; reduced-motion collapses durations; left-handed switch. Open item: screen-reader narration of each move (the log has the words; only the focused card is told).
- Offline: same as piquet; README line 14: "saved to your own machine plays offline". HUD springs use CSS `linear()` (Safari 17.2+).

### Working rules: `~/cassino/CLAUDE.md` (38 lines, quoted in full in §7 below).

## 4. Cross-cutting facts

| | piquet | bezique | cassino |
|---|---|---|---|
| Rust tests (`#[test]`) | 207 | 203 | 370 |
| node test files (`*.test.js`) | 22 | 0 | 29 |
| Playwright browser test | 933 lines | none | 2,136 lines |
| Built page | `web3d/piquet3d.html` 3.1 MB | none | `web3d/cassino3d.html` 4.0 MB |
| Pages | yes, `.nojekyll` | no (private) | yes |
| `CLAUDE.md` / `bin/gate` | no / no (rules in PLAN) | no / no | yes / yes |
| Phrase bank | 309 groups, 420 texts | none | 123 groups, 770 texts |
| Tutor/brief/learner | no | no | yes |
| Celebrations | yes (2,708 lines, 7 scenes) | - | no |
| Ladder top | 4 (+solver) | 5 (6 planned) | 4 |
| Persona/style | style | 4 personas | style |

## 5. Diff matrix

### 5a. web3d/src (piquet -> cassino; counts = lines in P / C; "diff" = lines differing in `diff` output; "1" means only the origin-header line)

**Near-identical, one-line origin header only (take verbatim into the kit):**

| File | P / C lines | diff | Notes |
|---|---|---|---|
| `art.js` | 60 / 61 | 1 | image-map loader |
| `bag.js` | 33 / 34 | 1 | shuffle bag |
| `cards.js` | 91 / 92 | 1 | rounded-slab card geometry |
| `deck.js` | 78 / 79 | 1 | meshes, ink hull; keyed by card code |
| `materials.js` | 91 / 92 | 1 | toon shading |
| `surfaces.js` | 256 / 257 | 1 | procedural table tops |
| `timeline.js` | 83 / 88 | 5 | cassino better (adds `moved`) |
| `easing.js` | 97 / 114 | 17 | cassino better (adds `snap`, `flick`); the M3 tokens too |

**Diverged slightly, cassino is the better/superset version:**

| File | P / C | diff | Verdict |
|---|---|---|---|
| `framing.js` | 109 / 126 | 31 | take cassino (sides/eye params) |
| `scene.js` | 237 / 318 | 105 | take cassino |
| `faces.js` (Large Text faces) | 148 / 219 | 121 | take cassino; make the rank/suit set data-driven (trumps need Roman numerals) |
| `kinematics.js` | 365 / 458 | 171 | take cassino (adds peelOff, row, carryBlock; snap is in easing.js) |
| `dialogue.js` | 84 / 188 | 126 | take cassino (SLACK/LATE, placement, chunk) |
| `tutorial.js` | 101 / 153 | 106 | take cassino |
| `scorebug.js` | 60 / 139 | 183 | game-specific score display; cassino's `hud.js` (415) supersedes it |
| `units.js` | 127 / 201 | 234 | take cassino's structure (portrait/touch cameras, zones); zones are game-specific |
| `director.js` | 367 / 357 | 400 | same role (render-on-demand loop, `createDirector`); needs a careful 3-way merge, cassino's is the later one |
| `engine.js` | 31 / 176 | 169 | wasm wrapper; piquet minimal, cassino full; the generic part (alloc/send/out, JSON decode, buffer re-fetch after memory growth) is shared, exports are game-specific |

**Rewritten per game (game logic; use as pattern, not code):** `layout.js` (270 / 307, 487 diff), `choreography.js` (732 / 767, 1,123 diff), `main.js` (1,013 / 1,364), `overlay.js` (1,326 / 624 + cassino split into `chrome.js` 559, `hud.js` 415, `prefs.js` 259), `talk.js` (130 / 409), `style.css` (1,936 / 1,559, 2,737 diff), `index.html` (49 / 30).

**Only in cassino (keep):** `prefs.js`, `progress.js`, `series.js`, `selection.js`, `reveal.js`, `review.js`, `tutorworker*.js`, `badges.js`, `aids.js`, `announce.js`, `decision.js`, `chrome.js`, `hud.js`.
**Only in piquet:** `celebrate/` (13 files), `hand.js`, `breaks.js`, `leftovers.js`, `demo.js`, `spike.js`, `drag.js`, `speech.js` (cassino folded into talk.js).

**Tools:** `web3d/tools/art.py` (card rasteriser; near-same, adapted for rank sets), `phrases.py` (331 / 525; same schema and `--check`), `scheme.mjs` (M3 colours from one seed; piquet only: cassino reuses the generated tokens), `courts.py`, `moves.mjs`. `build.py` 160 / 196: same skeleton; extract it into a parameterised script.

### 5b. Rust

| Component | Files | Verdict |
|---|---|---|
| Seeded PRNG (xoshiro256** + SplitMix64 + `stream`) | `bezique-core/src/rng.rs` 358 / `cassino-core/src/rng.rs` 357, **17 diff lines** (stream-purpose enum + doc header) | Near-identical; best shared-crate candidate. Reference vectors from C live in `~/bezique/tools/prng-reference/`. Piquet's (100 lines, MT19937-based, `mt19937.rs` 174) is superseded. |
| Mirrored-pair tournaments and O'Brien-Fleming sequential stopping | bezique 471 (`games_until`, `Sequential`), cassino 446 (`obf_constant`, `Plan`, `run_sequential`), piquet 348 (Bradley-Terry ratings, `round_robin`) | Same idea, three differently-shaped implementations. Cassino's `Plan`/`Look`/`Report` is the cleanest statistical core; piquet's `ratings()` Bradley-Terry is unique. Unify as generic `fn duel<G: Game>`. |
| Opponent = (level, erraticism, seed), pure-function-of-view | bezique 212 / cassino 211 (339 diff lines, same shape); piquet `opponents.rs` 149 | Cassino's wording/semantics (slip one rung once) vs bezique (slip repeatedly + persona). Choose cassino's with persona/style hook. |
| Observation / View discipline | piquet 272, bezique 387, cassino 430 | Pattern only; content is game-specific. |
| Session (undo, replay, record text, aids, hint) | piquet `table.rs` 1,203, bezique `session.rs` 1,630, cassino `session.rs` 2,271 | Same loop; generic shell is extractable (record/restore with version, undo via replay from seed, aid toggles, `Prompt/Action/Event`). |
| Card model | piquet: 32-card u32 bitset; bezique: 64 cards in u64 (twins); cassino: 52 cards | Strohmandeln needs 54 cards -> `u64` bitset; bezique's `CardSet` ops (`in_suit`, `intersect`, iteration) are the closest. |
| Trick logic | bezique `tricks.rs` 207 (winner, `second_wins`, `endgame_replies` = follow suit, must win, must trump if void); piquet `play.rs` 314 (no-trump, follow suit) | **Directly relevant**: Tarock requires Farbzwang/Stichzwang-like rules; bezique's endgame rule is the same "must follow, must head the trick, else trump, else anything". Good starting template. |
| Exact endgame solver | piquet `solver.rs` 1,017, bezique 516, cassino 351 | Different games; the piquet/bezique ones are trick-taking and have search structure usable for Strohmandeln's endgame. |
| Scoring as ordered event log | all three | Pattern; also drives the talk and the trackers. |
| wasm ABI | piquet 756 / bezique 401 / cassino 1,814 | Generic part is ~60 lines (alloc, send, render into out buffer). Extract as a macro or small crate (`card-wasm-abi`). |
| Gate / CLAUDE.md / build.py | see above | Copy and parameterise by game name. |

## 6. Recommendation: a "card-table kit" for Strohmandeln and later games

### 6.1 Principle
Do not retro-fit the three existing repos. **Create the kit from cassino's files (best version of each) and let Strohmandeln be the first consumer.** Later, if wanted, cassino/piquet can adopt it, but there is no obligation: they are finished and published. This respects the documented intent ("the files that stayed identical are the package").

### 6.2 What goes in the JS / three.js side (`card-table-kit/js/`)
Copy verbatim from cassino unless noted, then strip game names:
- **Render core**: `scene.js`, `cards.js`, `materials.js`, `deck.js`, `surfaces.js`, `framing.js`, `art.js`, `faces.js` (generalize ranks/suits to a card-set descriptor so trumps can have Roman numerals), `director.js` (render-on-demand loop).
- **Motion**: `easing.js`, `kinematics.js`, `timeline.js` (all of cassino's), plus the `choreography.js` *helpers* (reducer-from-events, no-card-through-table and corner-height test harnesses) while leaving the per-game event table to the game.
- **Layout helpers**: `layout.js` pure parts (`jitter`, `gridPlace`, `fan`/`row`) and the unit constants shape in `units.js` (`CARD`, `TABLE`, `CAMERA`, `CAMERA_PORTRAIT`, `PORTRAIT_BELOW`, `ZONES*` as per-game config).
- **Talk**: `bag.js`, `dialogue.js` (engine-agnostic scheduler: clock, `GAP`, `TURN`, `SLACK`, `LATE`, `choosePlace`, `chunk`), `announce.js`; `tools/phrases.py` as the **bank compiler** (schema `{groups, texts}`, source-letter provenance, `--check`, generate `PHRASES.md`), with the bank content per game.
- **Shell**: `prefs.js` (version-tagged, try/catch storage, URL overrides), `progress.js`, `series.js`, `selection.js` pattern, `chrome.js` (top bar, move bar, left-handed), `tutorial.js` + tutorial.md loader, `tutorworker*.js`, `review.js`, `reveal.js` (optional), the M3 colour tokens + `scheme.mjs`, Material Symbols paths.
- **Build/test**: `build.py` parameterised by a `game.toml` (name, wasm crate, art sets, extra assets, placeholders), with `--check`; `tools/art.py` (rasteriser, driven by a card-set manifest); `test/browser.py` helpers (`open_page`, `check_offline`, `check_drawn`, `check_no_console_error`, `settle`, `shot`, `close_all`) as an importable module, with the per-game checks living in the game; `bin/gate` template; node test harness for the pure modules.
- Optional: `celebrate/` (piquet's 7 end-of-game scenes) as a separate plugin dir if a celebration is wanted.

### 6.3 What goes in the Rust side (workspace of small crates)
- `cardkit-rng`: xoshiro256** + SplitMix64 + `Rng::stream(purpose)` (the 358-line file), with the C reference vectors in-tree.
- `cardkit-bits`: `CardSet(u64)` with suit/rank iteration, `in_suit`, `intersect`; generic over a deck definition (54 for Tarock).
- `cardkit-measure`: mirrored-pair harness (`Factory`, `mirrored_game`), O'Brien-Fleming `Plan/Look/Report`, Bradley-Terry ratings (from piquet).
- `cardkit-session`: generic session shell: `Prompt/Action/Event` traits, command log + seed record with `RECORD_VERSION` and golden-game test, undo-by-replay, aids toggles, `hint` plumbing, `Observation` marker trait.
- `cardkit-agent`: `Agent` trait, `Opponent{level, erraticism, seed}` (decision = pure function of view; slip one rung), `Style` hook, capability-ladder test helper ("each rung must beat the one below in mirrored measurement").
- `cardkit-wasm`: the alloc/send/out ABI as a macro, so a game's wasm crate is ~100 lines plus its own exports. Keep it free of wasm-bindgen and serde (the engine has zero runtime deps today).
- `cardkit-tutor` (optional, later): `learner`/`brief`/`lessons` scaffolding (skill graph with prerequisites; summaries per game; one-skill focus) - the machinery is generic, the skills are game data. Only cassino has this, and it is heavily cassino-shaped, so extract after Strohmandeln needs it.
- Trick logic: do **not** put in the kit yet; write Strohmandeln's own `tricks.rs` starting from bezique's `tricks.rs` and piquet's `play.rs`, then promote a `follow-suit/must-win` helper if it proves general.

### 6.4 How to consume it (the constraint is one self-contained offline HTML per game)
The constraint only bites at **build time**: whatever the kit is, `build.py` ends by inlining everything, so the shipped html is self-contained regardless. So choose by workflow:

1. **Recommended: a separate git repo `abartnof/card-table-kit`, pulled into each game as a git submodule at `kit/` (JS and Python) plus a Cargo `git`/`path` dependency on the same checkout (`cardkit-* = { path = "../kit/rust/cardkit-rng" }`)**. One checkout serves both languages; pin by commit; the origin-header convention (`// From card-table-kit @ <sha>`) is replaced by the submodule pin. esbuild resolves `import ... from "../../kit/js/scene.js"` directly (no publish step), and `build.py` imports `kit/tools`. GitHub Pages is unaffected: it serves the committed built html. The gate should fail if `git submodule status` shows a dirty or unpinned kit. Caveat: submodules are a known foot-gun with Claude Code worktrees (`.claude/worktrees` already in use by cassino) - document `git submodule update --init` in `CLAUDE.md`.
2. **Fallback: vendored directory** (`web3d/kit/` copied by a `bin/sync-kit` script with a version stamp file). Zero tooling friction, matches the current practice of copying with headers, but drifts. Acceptable if submodules prove annoying.
3. Not recommended: npm workspace (the kit is not published and the Python/Rust halves wouldn't benefit); a cargo workspace spanning games (couples lifecycles, the very thing bezique's DESIGN.md §3 warned against).

Tests: the kit carries its own node tests (the pure modules: bag, easing, kinematics, timeline, layout, dialogue, prefs, selection) and Rust tests; each game's gate runs the game's tests only, plus checks the kit pin.

### 6.5 What is Strohmandeln-specific (not in the kit)
- **Engine rules**: 54-card Tarock deck (22 trumps incl. the Sküs/Excuse, 4 suits x 8 [K, Q, C, J + pip order differing between red and black suits]), the two-hand deal, the **three straw-man (Strohmann) stacks per player** with face-down/face-up exposure rules and who may play from which stack; trick-taking with must-follow / must-trump rules; scoring (card points, Königsrufen-style bonuses if any), game end. Observation: the face-up straw-men are public information, face-down are hidden until exposed - a classic leak risk (see piquet lessons: audit every View field with "when was this said aloud, and by whom").
- **Engine opponent ladder**: new rungs (Legal -> Greedy/maxim-following -> Counter with card memory incl. trump counting -> Searcher, with hidden stack inference); bezique's world-sampling (rung 5) and exact endgame solver (rung 3) are the nearest ancestors. Measure each rung by mirrored pairs and OBF stopping, delete non-winners.
- **Table layout/choreography**: per-player hand + 3 stacks (each 2 faces: up/down card piles), a trick area, a talon (if any) and score area; stack-flip choreography (turn a face-down straw man up when exposed, using `kinematics.flip`), trick gather (cassino's `carryBlock` carries the trick to the winner's pile). `ZONES`, `ZONES_PORTRAIT`, `ZONES_TOUCH` all new; mind the phone: 3 stacks + hand + trick area in portrait needs its own staging test (cassino's `staging.test.js` pattern).
- **Card art**: faces for 54 cards. The CC0 deck gives 4 suits x pips and court cards but **no Roman-numeral trumps and no Sküs**; either draw a trump set (project's own, in the same vector style, or Large-Text faces drawn at runtime from numerals, which `faces.js` already supports in spirit - this is the cheapest path and has no license burden) or source a public-domain Tarock deck (e.g., Industrie und Gluck / Wiener Tarock scans, check licenses; add to `CREDITS.md` in the same commit). Back: the CC BY-SA 3.0 back stays as is (credit on screen).
- **Phrase bank**: Tarock/Austrian table talk (Wiener Schmah, "Ouvert", "Stich", Viennese coffeehouse register), with source letters in the same schema; reuse `bag`, `dialogue`, `talk` plumbing.
- **Tutorial and tutor**: skill graph for Tarock (trump management, following, straw-man exposure timing, counting), `tutorial.md`, hint/advice generators.
- **Rules doc**: `docs/RULES.md` per bezique/cassino practice (authority, worked examples becoming fixtures) and `docs/LITERATURE.md`/`research/` dir (this very folder follows the cassino `research/` convention).

## 7. Standing working rules to inherit

Essentials of `~/cassino/CLAUDE.md` (38 lines; mostly verbatim, but the clauses "about 15 minutes on a big machine, over an hour on the 2-core VM" and the `--quick` flag's "loads w/o console error" come from `cassino/PLAN.md:494` and `browser.py`, not from CLAUDE.md [review]):
- Start with `PLAN.md` (where things stand), then `docs/RULES.md` (rules, settled), then `docs/DESIGN.md` (why it is built this way).
- **Test-driven.** Write the test, watch it fail, then make it pass.
- **Atomic commits, often**: one logical change each, with `PLAN.md` kept current as work lands.
- **The gate before every commit**: `bin/gate && git commit ...`. It runs `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, then (with `web3d/node_modules` present) builds the wasm module, runs the table's Node tests and checks that the committed page is built from the current sources (`python3 web3d/build.py --check`; rebuild it with `python3 web3d/build.py` and commit it with any engine or table change). One-line summary; non-zero exit on any failure. **The gate does not open a browser**: before committing a table change also run `web3d/test/browser.py --quick` (about a minute: page loads with no console error and plays a few moves), and the full browser test (~15 min; over an hour on the 2-core VM) before a phase is called done. **Never pipe the gate** (`bin/gate | tail` reports tail's status), and in a multi-step shell command write `bin/gate || exit 1` before the commit. `set -e` has no effect in Claude Code's shell, and a heredoc ends an `&&` chain at its closing line, so a commit after one runs regardless. "Broken builds were committed four times before this rule."
- **Simulations stop when the signal is clear.** Run in batches, look between them, stop when the effect crosses an O'Brien-Fleming boundary fixed before the first batch; otherwise report the interval at the last look (`docs/DESIGN.md` §11.4). Never peek with an unadjusted p < 0.05. Always mirrored pairs.
- **The engine holds the rules; clients hold none.**
- **Documents are public-facing.** The person who commissioned the game is "the maintainer", never named; docs are project notes, not a diary.
- **Consult the maintainer before anything that spends money** or any long compute run, with an estimate in hand.

Additional rules from piquet's PLAN.md "Working conventions" (lines 197-218) and bezique's DESIGN §2 that cassino's CLAUDE.md does not repeat:
- One self-contained HTML page, works offline, modest in size (5 MB guideline, not a limit); Material Design 3 bundled, no CDNs, no fonts or icons fetched at runtime.
- "Your opponent", never a proper name, in anything a player reads (cassino extends this to the whole UI).
- Every aid is a toggle. The page makes no sound.
- The look: clean, bright, a bit cartoonish, not childish.
- Every third-party asset credited in `CREDITS.md` in the commit that brings it in (and on-screen where the license demands: the CC BY-SA back).
- Branching: piquet develops on `dev` and merges to `main` when ready (Pages serves `main`); cassino and bezique commit straight to `main` until public. For Strohmandeln: use `dev` + `main` once Pages is enabled.
- Rust from the first line, no runtime deps in core, plain copyable data, modular, test-only deps allowed when earned; slow tests: when the loop feels slow, find the one test; statistical tests stay in the default run (opt-level 2 for tests).
- Observation is where subtle defects live: audit each View field with "when was this said aloud, and by whom?"; narrate the dialogue as a player would read it.
- Origin-header convention for copied files: `// From <game> <path> @ <sha>` (to be replaced by the submodule pin if the kit is adopted).

## 8. Suggested first steps for Strohmandeln
1. Read `~/cassino/docs/TABLE3D.md` §3-7, `~/cassino/docs/PROTOCOL.md`, `~/piquet/PLAN.md` "Hard-won lessons".
2. Scaffold the repo: copy `bin/gate`, `CLAUDE.md`, `rust-toolchain.toml`, `Cargo.toml` profile settings, `.gitignore`, `.nojekyll`, root `index.html`.
3. Create the kit repo from cassino's best files per §5a (header-only-diff files verbatim), then write Strohmandeln's `RULES.md` and engine tests first (TDD), starting from bezique's `rng.rs`, `cards.rs`, `tricks.rs`.
4. Decide the trump-art route early (it gates `build.py`'s art step and `CREDITS.md`).
