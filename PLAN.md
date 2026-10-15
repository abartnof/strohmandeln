# Plan

Strohmandeln as one offline web page: a Rust engine compiled to WebAssembly, a
three.js table, a computer opponent with period table talk, a tutor, and a
terminal client. Built on the parts of Cassino and Piquet that carry over.

## Decided

- 14 Oct 2026: the modern game (15 hand cards plus three 4-card straw men
  each). The 1829 dummy-pile game is not built.
- 14 Oct 2026: canonical rules by consensus of the sources, with settings
  only for disagreements that persisted through the game's history
  (`research/14-consensus-rules.md`): both pass, game values, Pagat ultimo
  value.
- 14 Oct 2026: the session is the period "Abend": 3 or 6 Touren (2 deals
  each) announced at the start, a running account on a slate, settled at
  2 kreuzer a Point (`research/14-consensus-rules.md`, "The session in detail").
- 14 Oct 2026: cards are our own large-type faces: suit cards with rank
  labels K Q C J (the jack is "J"), tarocks with Roman numerals (IIII as on
  real packs) and nicknames on I, II, III, IIII, XXI and the Sküs; a plain back.
- 14 Oct 2026: shared code is a vendored `kit/` copied from Cassino with a
  `kit/STAMP` file naming the source commit, not a submodule
  (`research/review/RC.md`). It becomes a package once a second game uses it.
- 14 Oct 2026: the game ships in English (default) and German, with an
  in-game switch. The engine emits message keys with parameters, never prose;
  every phrase, interface string and tutorial page has both languages; card
  faces swap court labels (K Q C J / K D C B) with the language.
- 14 Oct 2026: the score HUD and the count's per-card popups are in the kit
  (`kit/js/scoring.js`, `hud.js`, `count.js`, `kit/css/hud.css`), generic and
  parameterised by a game definition, because several games in the series score
  by counting cards. Strohmandeln supplies its categories, thirds format and
  36 of 70 (`kit/README.md`, "The score HUD").
- 14 Oct 2026: "Mit einem guten Herzen gewinnt man kein Spiel" (Wander, Herz 239) stays in `trick-bad-yours`: a proverb about play, within the kindness rule (open question in docs/review/talk-and-phrases.md, closed).

## Work packages

| # | Package | Depends on | State |
|---|---|---|---|
| 1 | `docs/RULES.md`: the engine's rules, every one cited; the session; the events the table and talk follow | — | done |
| 2 | `kit/`: the shared JS (render core, motion, dialogue, tutorial) and `build.py`, from Cassino; `bin/gate` | — | done |
| 3 | Phrase bank: `tools/phrases.py` → `web3d/words.json` + `docs/PHRASES.md`, with a kindness test | 1 (moments) | done |
| 4 | `strohmandeln-core`: cards, deal, straw men, legal plays, tricks, counting in thirds, bonuses, settings, session and slate, records | 1 | done |
| 5 | `strohmandeln-wasm` and `strohmandeln-cli`, with `docs/PROTOCOL.md` | 4 | done |
| 6 | Computer opponent: a ladder of rungs, each measured against the one below (`docs/OPPONENT.md`) | 4 | done |
| 7 | Card faces: 54 large-type faces and a back, drawn at runtime (`docs/img/faces.png`) | 2 | done |
| 8 | The table (spec: `docs/TABLE3D.md`): layout, choreography, play, slate, settings, phone. The count and score use the kit's score HUD and scoring popups (moved from Cassino), configured for counting won cards in threes against 36 | 2, 5, 7 | done; open items below |
| 9 | Tutor: tutorial pages, the kibitzer's hints and explanations, end-of-deal review | 5, 8 | done |
| 10 | Build, browser test, README, CREDITS, LICENSE, Pages | all | done; published 15 Oct 2026 at abartnof.github.io/strohmandeln |
| 11 | Languages: interface strings (en/de) and the switch, German court labels on the faces, German tutorial pages | 7, 8, 9 | done (built across 3, 8, 9; language_switch passes) |
| 12 | Play-through fixes (15 Oct 2026): tutorial pacing and wording; high-visibility faces as in Cassino, numerals that clip in the fan, the stripes on the backs; play by a second click, the HUD's `{n}`, the Kibitzer and § 13 labels, tarocks sorted high to low; hints that give their reasons | 9, 8, 7, 5 | in progress (four agents) |

## Open

- Talk (review, docs/review/talk-and-phrases.md): the table must send `count_close` before `game_scored`, or the result line is dropped (TABLE3D 5.2.3; needs a table test); a run at the third pull replaces that pull's line; `count-close` fires only at exactly 107 thirds, so it is rare. Resolved: the phrase bank's cross-group repeats (a saying in at most 2 groups) and the thin frequent groups (8 or more wordings each).

- Kit (review CR-KIT, docs/review/kit-and-faces.md): textures for phones at 384 px or less, built lazily (54 at 512 px is ~105 MB with mipmaps); the HUD must be rebuilt to switch language; Cassino defaults left in the kit (HAND_H, end = 21, cards = 52); tools/contact_sheet.mjs depends on ~/piquet/.venv.

- Table DOM modules (review, docs/review/table-dom-modules.md): a save refused as too big leaves the older save, so Continue offers an earlier state; a quota error reports "unavailable", not "too-big"; `seen` lives in `strohmandeln.prefs`, not the separate `strohmandeln.seen` of TABLE3D 7.7.1; URL overrides must be gated to test runs by the page (7.7.4); the folded slate's backdrop ignores taps (4.1.1); the ultimo setting's note names no source (7.2).

- Opponent (review, docs/review/opponent.md): rung 4's `bid_margin` is in fixed Points, so a doubled deal clears it more easily; the step from rung 2 to 3 is thin (+1.39 ± 0.59 Points per pair, stopped on the boundary).

- Tutor (review, docs/review/tutor.md): a valat is invisible to the review (needs `valat-won-you`/`valat-lost-you`); TABLE3D 6.2.3 (pages opened from the review) and 6.4.3 (a Worker, `?noworker`) not built — the review takes milliseconds and the slowest hint ~160 ms on the main thread; the sitting review's `deal` parameter is not shown; unused strings tutor.muted, tutor.page.close, tutor.speech, tutor.off; on a tie `try-count` beats a specific tip; test runs write to the real `strohmandeln.progress`; the `kibitz` group's lines read as said to a kibitzer, not by him; the tutor tab's placement on phones is unchecked in a browser.

- Table (reviews docs/review/table-core.md, table-gaps.md): browser suite (`bin/browser-full`, ~35 min on this VM; review docs/review/browser-suite.md): every written check passes (15 Oct 2026; the last three confirmed singly). Not written (the `NOT_YET` set in web3d/test/browser_suite.test.js): slate_reconstruction_note, talk_timing, talk_cross_group, opponent_unnamed, tutor_mute, hint_and_explain, deal_review, strings_complete, single_file, full_sitting_3, full_sitting_6_extended, phone_sitting, watch_reload. Also: slate_open_close does not check the count or the extension prompt; a check with zero assertions still passes; no Node test for the slate's cancellable fold timer. Spec clauses not met: 2.3.1 the human's last two cards shown singly; 2.4 the valat chip lacks its value; 2.1 void deal shows the opponent's backs only (the protocol carries no cards); 1.5.5 dialogue placement rough; 3.1.2 the dragged card does not follow the finger; review item 16 (an open dialog closes on a language switch; scheduled popups and the HUD's bonus lines keep or lose the old language); the card lists keep the last focus highlighted after Tab leaves; portrait phone layout first-pass. Desktop "Auto" faces are now Classic, Jumbo on touch. Browser seeds depend on the opponent at `?level=1`; a change to the opponent or the deal needs a rescan.

- Gate: on a fresh checkout it needs `npm ci --prefix web3d` and a wasm build before the Node tests (they run before the gate's own wasm step); `cargo` must be on PATH. Werner is dated only "by 1891" (no year on the title page).

- The currency on the slate (gulden and kreuzer, or krone). Note 13's dates for the krone are unsourced.
