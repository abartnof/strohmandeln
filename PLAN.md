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
- 15 Oct 2026: the phone layout is on hold until the maintainer has looked at the game on a phone (the table may hold too many cards for one). New features are built for desktop and only must not break on a narrow screen.
- 15 Oct 2026: the tutor (docs/TUTOR.md 15): a rule is shipped for a skill only if it passes the 0.80 precision gate (M5), else its fact; the brief keeps one example from the deal; the count is the dialog's header; the learner's game-independent parts go to `kit/learner` now, with an equivalence test against Cassino's; a hint "is not counted as evidence" (TABLE3D 6.3.2).
- 15 Oct 2026: the tutor's measurements (docs/TUTOR.md 11, M1-M8) run when the code is ready, without a further check with the maintainer: each planned before its first batch, stopped early on the boundary, results recorded with the commands that reproduce them.
- 15 Oct 2026: terms: a **Point** is only what is paid on the slate; the count's units are "points" in English and "Augen" in German (as `hint.unit.*`). The raw card values total 106; counted in threes they make 70.

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
| 12 | Play-through fixes (15 Oct 2026): tutorial pacing and wording; high-visibility faces as in Cassino, numerals that clip in the fan, the stripes on the backs; play by a second click, the HUD's `{n}`, the Kibitzer and § 13 labels, tarocks sorted high to low; hints that give their reasons | 9, 8, 7, 5 | done 15 Oct 2026; also: the phone's hand in two rows, the page chrome in Cassino's Material 3 colours, the gate builds the wasm before the Node tests (reviews: docs/review/faces-fan, colours, tutorial-pacing, tutorial-wording, table-interaction, portrait-hand, hint-reasons-engine, hint-reasons-table) |
| 13 | Play-through notes (15 Oct 2026): scoring explained (fuller `thirds` and `scoring` pages, an optional "Scoring in full" page from Help and the pages); a page on the pack today (the tarocks are trumps, not fortune-telling cards; the Cavall), from a cited note `research/15-the-pack-today.md`; a card tracker shown with hints (cards played, unseen, voids, masters); larger samples behind the hints' "in x of n" lines, sized by measured cost | 9, 12 | in progress; hint samples done (1,000 lead / 50 odds; review docs/review/hint-sample-sizes.md), research note 15 done, the tracker done (TABLE3D 6.6; review docs/review/tracker.md), scoring pages and the running tally done (review docs/review/scoring-pages.md, two passes), the page on the cards done (review docs/review/the-cards-page.md). Done 15 Oct 2026 |
| 14 | The tutor across deals, as Cassino's (DESIGN 12.8): a ladder of skills from the sources (`research/16-skill-ladder.md`), a learner that finds the next skill the player is ready for (prerequisites cleared, surely short of mastery), a gentle brief after each deal with one rule and one or two suggestions (`docs/TUTOR.md`); the game-independent learner moved into the kit | 9, 6 | spec done (15 Oct 2026); `kit/learner` done (review docs/review/kit-learner.md); the evidence done (`tutor.rs`, `students.rs`, `dealrec.rs`, `bin/lesions`; review docs/review/tutor-evidence.md, two passes); the brief and its dialog merged (reviews docs/review/tutor-brief-engine.md, tutor-brief-table.md; second pass running); measurements M1, M2, M3, M8 done, M4-M7 running |

## Open

- Talk (review, docs/review/talk-and-phrases.md): the table must send `count_close` before `game_scored`, or the result line is dropped (TABLE3D 5.2.3; needs a table test); a run at the third pull replaces that pull's line; `count-close` fires only at exactly 107 thirds, so it is rare. Resolved: the phrase bank's cross-group repeats (a saying in at most 2 groups) and the thin frequent groups (8 or more wordings each).

- Kit (review CR-KIT, docs/review/kit-and-faces.md): textures for phones at 384 px or less, built lazily (54 at 512 px is ~105 MB with mipmaps); the HUD must be rebuilt to switch language; Cassino defaults left in the kit (HAND_H, end = 21, cards = 52); tools/contact_sheet.mjs depends on ~/piquet/.venv.

- Table DOM modules (review, docs/review/table-dom-modules.md): a save refused as too big leaves the older save, so Continue offers an earlier state; a quota error reports "unavailable", not "too-big"; `seen` lives in `strohmandeln.prefs`, not the separate `strohmandeln.seen` of TABLE3D 7.7.1; URL overrides must be gated to test runs by the page (7.7.4); the folded slate's backdrop ignores taps (4.1.1); the ultimo setting's note names no source (7.2).

- Opponent (review, docs/review/opponent.md): rung 4's `bid_margin` is in fixed Points, so a doubled deal clears it more easily; the step from rung 2 to 3 is the smallest (rerun 15 Oct 2026 on fresh deals: +1.53 ± 0.40 Points per pair, 300 pairs, clear at look 3).

- Tutor (review, docs/review/tutor.md): a valat is invisible to the review (needs `valat-won-you`/`valat-lost-you`); TABLE3D 6.2.3 (pages opened from the review) and 6.4.3 (a Worker, `?noworker`) not built — the review takes milliseconds; the whole level-4 hint is measured in docs/OPPONENT.md "Hint evidence" (15 Oct 2026, a loaded VM: lead p95 42 ms native; the wasm ratio not settled: 0.55 to 1.8 median across two runs); a Worker would allow 10,000 / 200 sampled deals; the sitting review's `deal` parameter is not shown; unused strings tutor.muted, tutor.page.close, tutor.speech, tutor.off; on a tie `try-count` beats a specific tip; test runs write to the real `strohmandeln.progress`; the `kibitz` group's lines read as said to a kibitzer, not by him; the tutor tab's placement on phones is unchecked in a browser.

- Table (reviews docs/review/table-core.md, table-gaps.md): browser suite (`bin/browser-full`, ~35 min on this VM; review docs/review/browser-suite.md): every written check passes (15 Oct 2026; the last three confirmed singly). Not written (the `NOT_YET` set in web3d/test/browser_suite.test.js): slate_reconstruction_note, talk_timing, talk_cross_group, opponent_unnamed, tutor_mute, hint_and_explain, deal_review, strings_complete, single_file, full_sitting_3, full_sitting_6_extended, phone_sitting, watch_reload. Also: slate_open_close does not check the count or the extension prompt; a check with zero assertions still passes; no Node test for the slate's cancellable fold timer. Spec clauses not met: 2.3.1 the human's last two cards shown singly; 2.4 the valat chip lacks its value; 2.1 void deal shows the opponent's backs only (the protocol carries no cards); 1.5.5 dialogue placement rough; 3.1.2 the dragged card does not follow the finger; review item 16 (an open dialog closes on a language switch; scheduled popups and the HUD's bonus lines keep or lose the old language); the card lists keep the last focus highlighted after Tab leaves. Desktop "Auto" faces are now Classic, Jumbo on touch. Browser seeds depend on the opponent at `?level=1`; a change to the opponent or the deal needs a rescan.

- Gate: on a fresh checkout it needs `npm ci --prefix web3d`; `cargo` must be on PATH. Werner is dated only "by 1891" (no year on the title page).

- The currency on the slate (gulden and kreuzer, or krone). Note 13's dates for the krone are unsourced.

- Play-through round (work package 12): the gold dot on a seen card is unexplained on the table (keep, explain, or drop: the maintainer's call); a click less than 10 px from the first click on a neighbour plays the chosen card in a dense narrow fan; a hint on a phone with the Announce row covers the lower hand; a language switch leaves a shown hint in the old language; the odds line's mean can come from a few large deals; `keep-packet` is named when another maxim decided; the bidding page's Points ignore the `values` setting; no page says who leads the first trick; the turning page opens after the turn (nothing is asked); `prefers-contrast: more` maps to token extremes, not black and white.

- Tracker (review docs/review/tracker.md): the tarock line lists the highest four and "…" without saying the order; cell labels 12 px, no larger-text option; wire fields the page does not read (`unseen_count`, `unseen_thirds`, `groups[].unseen`, `top_out`); dark and high-contrast looks checked once, before the three-state cells; phone layout deferred.

- kit/learner (review docs/review/kit-learner.md): one `short` key where Cassino has two (no decisions vs a short game); `Ladder` takes `&'static` data, so a ladder that varies with the rules needs several statics; a focus whose closure gives no bullets reads as "nothing stood out"; Cassino has not adopted the crate back.

- Scoring pages and tally (review docs/review/scoring-pages.md): the HUD drops to nothing when the count begins and recounts group by group (by design; a "Count" chip at `count_start` is optional); no unit test of `showTally` or the restore path, the landing moment pinned in Node only; `cards.rs` reads the value fixture by a relative `include_str!` into `web3d/`; `thirds` has 2 words of headroom under the 150-word cap; a language switch resets the long page's scroll; `kit/STAMP` lists a non-existent `web3d/src/tutorial.js`; no browser check that the long page's Close is visible without scrolling.

- The cards page (review docs/review/the-cards-page.md): it opens at the first deal although it answers no decision (TABLE3D 6.2.2, the one exception); TABLE3D :825 has an overlong line; the header test pins source ids, not their content.

- Tutor evidence (review docs/review/tutor-evidence.md): the LADDER constants are provisional until M3; a class lesion also moves other skills (the student x skill matrix, TUTOR.md 11); the Pagat and Bid knock-outs meet about half their chances; an always-pass player is not detectable (taking up is rarely clearly right); no partial-lesion case (M6); the argmax of each class is not in the standard error (M1); a merge to main runs `bin/lesions` unconditionally (TUTOR.md 12).

- Tutor brief (reviews docs/review/tutor-brief-engine.md, tutor-brief-table.md): no rule is told until M5 passes it (facts meanwhile); the focus is unvalidated until M6; the dialog title does not name the deal; the chip and the scored deal do not survive a reload; `?noworker` still listed in TABLE3D and prefs.js; the `deal_brief` browser check covers no void deal or throw-in; the sitting brief omits the Mond praise; the dialog's phone layout deferred.
- Opponent: the solver values the Pagat ultimo at 12 thirds (`ULTIMO_THIRDS`), about half the 23.5 thirds a Point is worth in play (M2, docs/MEASUREMENTS.md); a tournament with the measured value would show whether rung 4 gains.
