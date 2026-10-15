# Review: the scoring pages

Scope: `git diff main...worktree-agent-a98077b2488dd60f2` (7 commits): `web3d/tutorial.en.md`, `web3d/tutorial.md` (`bidding`, `thirds`, `scoring`, new `scoring-full`), `kit/js/tutorial.js` (link span), `web3d/src/{tutor,tutorview,main}.js`, `tutor.css`, `strings.json`, `docs/TABLE3D.md` 6.2, kit README/STAMP, the tests (`web3d/test/scoring_pages.test.js`, `support/pages.js`, tutor/tutorial/tutorview, `kit/test/tutorial.test.js`, two core tests), against `docs/RULES.md`, `kit/js/hud.js`, `web3d/src/{main,counting}.js` and CLAUDE.md.

Run: `node --test` on scoring_pages, tutorial, tutor, tutorview, strings (web3d/test) and kit/test/tutorial: 96 pass. `cargo test -p strohmandeln-core pages_worked`: 2 pass. The browser was not run; main.js link behaviour was traced by reading.

## Self-check list

1. **Rules.** Every rule on `bidding`, `thirds`, `scoring` and `scoring-full` agrees with RULES 1.7, 8.1-8.5, 9.1-9.3, 10.1-10.2, 11.1-11.7, 12.1-12.4, 14.1-14.6, 15.2, 15.3, 15.5, 15.6, 15.8, 15.9. Arithmetic checked: 106 less 36 is 70; 55 less 18⅔ is 36⅓ (109 thirds); 51 less 17⅓ is 33⅔; 36⅓ + 33⅔ = 70; a last K+J is 7 less 1⅓ is 5⅔; per-card 4⅓, 3⅓, 2⅓, 1⅓, ⅓; 105:105 and 106:104 thirds are 35 : 35 and 35⅓ : 34⅔; the evening 12 and 9 is 3 Points, 6 kreuzer, the smaller account pays. The Valat line is right but short (finding 9). Ok.
2. **HUD description.** Matches main.js and kit/js/hud.js: no tally runs during play (`trick_won` only sets the aid line); the top line is `Tour n of m · Deal n · Trick n / 27`; groups pop `+n` (`hud.group`); `lit` is `floor(units/3)` and 107 lights the finish; `chip.not-36` fires on 106; the bonus category is `apart` (in no total or segment); the chevron opens the per-deal list; `showBanner` names result and Points. Ok. TABLE3D still says otherwise in two places (finding 7).
3. **German.** Formal Sie, K D C B (`COURT_LABELS`), Tafel, Saldo, Conto, Troule, Aufnehmer, Honneur, slate causes ("Spiel 3, Troule 1", "Ultimo − 1") agree with strings.json. Idiom and term problems: finding 3.
4. **Tests pin numbers.** Mostly yes: the value table is held to literals copied from RULES 1.7 and to `valueOf` for all 54 cards; game values come from real wasm sittings (taker and passer policies, three `values` settings, asserted present); the worked deal's 55/51 split is built from the pack, not copied; thirds go through `thirds()`; two core tests pin the credits and the settlement. Gaps: finding 5.
5. **Link behaviour in main.js.** Traced, no bug: see "Link flows" below.
6. **kit.** The change is generic (`{ text, page }` span, the game's view makes the button), README and STAMP updated, kit tests added and pass. Ok.
7. **TABLE3D 6.2 and PAGE_KEYS.** PAGE_KEYS (14) equals the spec table (a test reads the table). 6.2.4 differs from the code in one phrase (finding 7).

## Link flows (item 5)

- Link on an auto-opened page: `onAct("page")` sees the dialog, sets `pageLinked`, `tutor.follow` marks the due page seen and opens the target as a Help page; `ui.dialog` replaces the "page" dialog; closing it (button, Escape, shade click all reach `dismissPage`) runs `tutorStep`, so a page still owed opens next. Chains (bidding -> thirds -> scoring-full) keep `pageLinked` because it is cleared only when a non-help page opens. Ok.
- Help opened over an auto-opened page: the shade blocks the pointer, so it is reachable only by Tab to the Help button behind the dialog. Picking a page there is read as a link: the due page is marked seen. Acceptable (the reader chose to leave it; it is the same as "Not now"). No change.
- Language switch with the long page open: `openPage(current, { help: pageHelp })` redraws it in the new language and keeps `pageLinked`. Ok, but the scroll position is lost (finding 12).
- Following a link from the bidding page marks `thirds` seen, so it will not pop up at the first trick won. Intended (the reader has read it); worth one line in 6.2.4.

## Findings

Severity: bug, should, nit. None is a bug.

### Should

1. **The long page's Close button is at the bottom of 1180 words** (`web3d/src/style.css:137` `.dialog { max-height: 92vh; overflow-y: auto }`, `web3d/src/tutor.css:56`). Escape and a click on the shade close it, but nothing says so, and on a touch screen the shade strip is about 3% wide. Fix: make `.actions` sticky for this dialog (`.dialog:has(.tutor-page[data-page="scoring-full"]) .actions { position: sticky; bottom: -24px; background: inherit; }`), and test that the page dialog has a close button reachable without scrolling in the browser suite.
2. **"Point" is used for both things on the long page, against its own definition.** Line 255 (`What the game is worth`) says a Point is a unit of payment, not a point of the count; but `tutorial.en.md:236` ("Rounded to the nearest whole Point that is 36") and `:248` ("one segment for each whole Point, up to 36") use it for the count. German `tutorial.md:237`, `:249` the same ("ganzen Point"). Fix: "nearest whole number", "one segment for each whole point of the count" (lower case), and in German "ganzen Zählpunkt" / "ganze Zahl".
3. **German a native reader would trip on.**
   - `tutorial.md:313` "Die 26 Karten Ihres Gegners gelten 106 − 55 = 51" and `:312` "haben zusammen die Werte 55": not idiom. Use "sind zusammen 55 wert" / "sind 106 − 55 = 51 wert"; `:44` "Die Karten gelten:" -> "Die Karten zählen:". `:312` "Zwei Drittel von jeder Karte sind 28 × ⅔ = 18⅔" -> "Zwei Drittel je Karte, bei 28 Karten 28 × ⅔ = 18⅔". English `tutorial.en.md:311` "have the values 55 in all" -> "are worth 55 in all".
   - **Partie / Spiel.** The page uses "Partie" for the deal (HUD "Partie 3") and for the game ("Die Partie wird dann ... bezahlt", `:261` "Wert der Partie", `:278`, `:301`), while the slate writes "Spiel". `:302` "Partie, Honneurs und Ultimo je zweimal" follows "Die nächste gespielte Partie" and means the game. Use "Spiel" for the game on the new text (Spiel 3, ein einfaches Spiel, der Wert des Spiels) and keep "Partie" for the deal.
   - **Seite** means the page ("die Seite spricht von Dritteln", `:230`) and a player's side ("jeder Seite", "beide Seiten", `:227`, `:236-238`, `:247`, `:250`, `:301`) on the same screen. Use "Spieler"/"Gewinn" for the side ("Hat ein Spieler 35⅔, hat der andere 34⅓") and "diese Seite" -> "diese Übersicht"/"man" for the page.
   - **Stiche** means tricks and the cards won in the same breath (`:311` "gewinnen 14 der 27 Stiche. Ihre Stiche sind 28 Karten"; `:132` "Zusammen zählen Ihre Stiche und die Ihres Gegners 70"). Say "Ihre gewonnenen Karten".
   - `:278` "Er zahlt 12 Points" has no clear antecedent (Valat or the player). "Das zahlt 12 Points" / "Der Valat zahlt ...".
4. **"Take" is never defined for a new player** (`thirds` "the two takes count 70", long page `:226`, `:236`, `:246`, `:310`). English: first use as "each take (the cards a player has won)". The German avoids it on `thirds` but uses "Seite" instead (3).
5. **Test gaps (numbers the page relies on that the test does not reproduce).**
   - `scoring_pages.test.js:45` is titled "the engine's, for all 54 cards" but `valueOf`/`thirdsOf` (web3d/src/cardinfo.js) are the table's JS, held to literals; nothing compares them to the Rust engine (`grep` finds no cardinfo test of it). Fix: in the sim, take a `count` event (the thirds the engine counts per side) and assert it equals the sum of `thirdsOf` over that side's won cards; rename the title otherwise.
   - `:200` the evening: `includes("12")`, `includes("9")` match any "12" in a 1180-word page. Fix: match the sentence (`/reads 12 and your opponent's 9/`, `/Konto|Conto .* 12 .* 9/`) and read the numbers from it against the core test's constants.
   - `:209` `/12 Points/` and the Valat line are not tied to the engine's 12 (core test `rules_10_1` exists but is not referenced). Fix: assert the number against the `valat` credit of a played sweep, or cite that test in the comment as for the others.
6. **`thirds` has six words of headroom** (143 en, 144 de of 150). Fine now; any later fix there will break the cap. If finding 4 adds words, drop "Together the two takes count 70." from `thirds` (it is on the long page) rather than raising the cap.
7. **Spec text that contradicts the code.**
   - `docs/TABLE3D.md:844` "opens the linked page over it": `ui.dialog` closes the "page" dialog first, so it replaces it; there is no way back to the first page except the Help list. Fix the sentence ("in its place"), or open as a second dialog.
   - `docs/TABLE3D.md:825` (thirds row) still says "the first `trick_won` (the tally begins to run)" and `:185` "the HUD's count line moves", while the same row now says the HUD runs no tally during play (and the code does not). Fix both: "(no tally runs)".
   - 6.2.4: add that a link from a due page marks it, and the target, seen (so `thirds` reached from `bidding` will not open again at its moment).

### Nit

8. **Padding on the long page** (~1180 words, otherwise coherent): `tutorial.en.md:222` ("The game is not played to 106; the next section shows why"), `:230` ("This is the traditional count in threes ... nothing is rounded") restate lines 228-229 and RULES 8.1; `:231` (18 groups of 3) is arithmetic only, since takes rarely split into threes. Cut `:230`, keep `:231`; same in German `:229`-`:231`.
9. **Valat line is short** (`tutorial.en.md:277`, `tutorial.md:278`): RULES 10.1 and 10.2 note engine say it is paid by the other player, can be the defender's, is quiet, and the honours and ultimo are added (and doubled by 14.2). One more clause: "The other player pays it. Honours and ultimo are paid as well."
10. **Draw section, play block** (`tutorial.en.md:302`, de `:303`; pre-existing in `draw`): "A deal thrown in does not use the double up" sits in the `both_pass=play` block, where there are no throw-ins (RULES 14.6: draw and throw-in never meet). The case that can happen under play is the void deal (2.6, 14.4): "A void deal, dealt again, does not use it up."
11. **HUD label vs page (German).** `strings.json` `hud.bonus` is "Ansagen und Ultimo"; the page (`tutorial.md:248`, `:250`) says "Honneurs und Ultimo". Use the label the reader sees, or change the label.
12. **Language switch resets the long page to the top** (`main.js:1086` re-opens the dialog). Keep `scrollTop` across the redraw.
13. **Titles and link text.** `thirds` is titled "Counting in thirds" but opens with the table of values; the bidding link reads "Card values and the count". Title "Card values and counting in thirds" (and the key stays). Also the link buttons (`tutor.css:55`) have `padding: 0`; on the phone layout (deferred) they need a tap target.
14. **Order of the two rules on `thirds`** (`tutorial.en.md:128`): "each card counts its value less two-thirds" and "in a group of three, the sum less 2" read as two methods. Say they are the same: "(the same thing, said for three cards at once)".
15. **STAMP** still lists `web3d/src/tutorial.js` as the game's copy of `kit/js/tutorial.js`; the file does not exist (tutor.js imports the kit). Pre-existing; fix when next touched.

## Content, as a new player

- `thirds` now does the job: the five values (5 4 3 2 1, with K Q C J and the three named tarocks), the one rule, one worked group (K, J, 7: 5 + 2 + 1 = 8, less 2 = 6), the leftovers, 70 in all and the 35⅔ line, and what the score at the top will do. Short and clear; it needs finding 4 (define "take") and 14 (the two rules are one). Same values on `bidding` with the letters the faces show, and a link: good.
- `scoring` is right and short; the example (win 3 + Trull 1 = 4) is the right one.
- The long page is accurate and well ordered (cards, thirds, 36 of 70, the HUD, the game, honours, ultimo, draw, one deal through, Touren, settlement). Repeats of honours/ultimo/draw are the point of a reference page and are not padding. The worked deal and evening are the best part. Findings 1, 2, 8 and 9 are what to fix.
- Kibitzer voice (6.5): the new pages carry no "if I may" line; the long page is reference and needs none, and `thirds`/`scoring` are plain. Acceptable. Kindness rule: nothing about the reader, only the cards, the count and the score.

## Verdict

Correct and well tested; no bug. Fix 1, 2, 3 and 5 (and 7, the spec text) before calling the work package done; 4, 6 and 8-15 may be recorded as open in PLAN.md.

---

## Second pass: the running tally and the fixes (2026-10-15)

Scope: `git diff ae8e997..HEAD` (5 commits, ef32355 to 0a558d0): `runningTally` (`web3d/src/counting.js`), `showTally` and the count's reset (`web3d/src/main.js`), the card-value fixture and its two tests (`web3d/test/fixtures/card-values.json`, `crates/strohmandeln-core/src/cards.rs`, `web3d/test/scoring_pages.test.js`), `web3d/test/tally.test.js`, the page and string wording, `docs/TABLE3D.md` 2.1, 2.5.3, 6.2.

Run: `cargo test -p strohmandeln-core`: 141 + 12 + 7 pass, including `rules_1_7_every_card_value_is_the_tables`. `node --test` on tally, scoring_pages, tutorial: 42 pass. `kit/build.py --check` (the rebuilt page matches its sources) and `cargo fmt --check` pass. The browser scenarios (`browser_scenarios.py`) were not run: Playwright is not installed here; the wiring was traced by reading.

### Checks

1. **Per-card is exact.** RULES 8.1 counts a take as the sum of `3 x value - 2` over its cards. `countLines` sums `thirdsOf` per card in each group of three, so a full group is `3(a+b+c) - 6` = (sum - 2) Points, and a leftover of one or two cards loses two-thirds each. The sum over all groups equals the sum over all cards whatever the grouping, so the running value after any trick is a partial sum of the same terms the count adds. After trick 27 it equals the engine's `count` event (`Deal::score` sums `Card::thirds()` over `takes`; `tally.test.js` asserts equality for four seeds x two policies, humans on seat 0 and 1, and that the total is 210).
2. **Nothing hidden.** `runningTally` reads only `trick_won` (winner, both cards), which RULES 13.1 lists as public ("every card played, in order, and each trick's winner ... each take's running count"; 13.6).
3. **Cases.** Pagat ultimo: the tally is set at `trick_won`, the ultimo event changes no card. Valat: all 27 tricks are played, the tally ends 210 : 0, then the count. Draw / double: 105 : 105 (or 106 : 104) is just the totals; the double is not part of it. Throw-in and void deal: no tricks; each deal (and each redeal) begins with `deal_start`, which resets the tally and the HUD. Human on either seat: `winner === human` picks the line (tested). Language switch: `hud.totals()` is kept and the HUD rebuilt (finding 3). Restore mid-deal: `showTally()` rebuilds from `r.state.events`, which hold the whole sitting; the fold resets at the last `deal_start` (finding 2).
4. **Fixture.** It binds both ways: the Rust test asserts all 54 codes, the same key set (`len == 54`), `value()`, `thirds() == 3v - 2` and the sum 106; the Node test asserts the key set equals `PACK`, `valueOf` equals the fixture and `thirdsOf == 3v - 2`. `tally.test.js` closes the loop for `thirdsOf` against the engine's own counts.
5. **Reset to nothing.** Scheduled at `count.lines[0] - 1`, i.e. as the first card of the count turns, after the last gather and any ultimo hold, so the final tally stays readable until the count starts. `hud.reset` bumps the epoch, so no queued tick survives it.

### Findings

Severity: bug, should, nit. None is a bug.

#### Should

1. **The HUD moves when the trick is complete, not when it lands on the pile.** `trick_won` is heard at `moments.start` (`web3d/src/choreography.js:580` `HEARD_WITHIN`, `:511-512`), which is the moment the second card lands, before the rest (1000 ms, 600 forced) and the carry (620 ms): the tally changes about 1.2 to 1.6 s before the cards reach the pile, while both still lie face up in the middle. At trick 27 the whole result is on the HUD before the trick is gathered. `docs/TABLE3D.md:185` says the lines move with the gather and the winner is named by the pile. The browser check only asks for `totals > 0` afterwards (`browser_scenarios.py:327`), so nothing pins the moment. Fix: expose a `landed` beat per stage (the local `marks` in `choreograph` has `moments.landed` but only `beats` leaves it; `result.marks` is the hint marks the director reads), add it to `beats`' sibling in `director.advance`, and run `showTally` from a moment at `landed`; keep the sentence, talk and aid line at `start`. `showTally(log.slice(0, index + 1))` is idempotent, so a skip or a change of speed is safe. Or amend 2.1 to "as the trick is complete". Either way add a test.
2. **Wording that still means the count as a Point.**
   - `words.json` `count-close.3` en: "By a single third of a Point!" (de: "Um ein einziges Drittel!"). Make it "By a single third!"; the phrase bank is generated (`tools/phrases.py`), so edit its source and run its check.
   - `docs/RULES.md:256` "(70 Points)" and `:258` "36 Points" are the count (RULES 8.1, 8.2); `crates/strohmandeln-core/tests/review.rs:118` "35 2/3 Points" likewise. Say "points".
   - The terminology decision (Point = payment only; points in English and Augen in German for the count) is written in no document (`grep` of `docs/` and `PLAN.md`). Record it, dated, with its source, in `docs/RULES.md` (conventions at the head) or `docs/MESSAGES.md`.

#### Nit

3. **Language switch mid-deal** (`main.js:1083-1085`) rebuilds the HUD as a counted hand (`hands: [ledger]`, `live: null`): the list reads "Deal 1 subtotal" instead of "live" until the next trick. Fix: when a deal is under way and the count has not begun, `showTally()` instead; keep the existing path for a count in progress (the ledger keeps its partial totals, and the later popups add to them correctly).
4. **A restored sitting always gets a live ledger** (`main.js:981`): after a counted deal, or between deals, the HUD says "live" where the normal flow has called `hud.endHand(1)`. Fix: after `showTally()`, call `hud.endHand(1)` when the last event of the log is `count` or later.
5. **The handover at the count is a visible drop** (the HUD shows e.g. 107 : 103, then 0 : 0 as the first card turns, then climbs back to the same numbers). It is the honest option: the ritual recounts group by group, so it must start from nothing, and the reset comes only once the reader has had the result for the whole gather and the pause. Starting the ritual from the running value would be wrong (it adds `+n` per group). Simplest improvement if wanted: nothing but a cue at the reset, the HUD's own "Count" string as a chip at `count_start`, so the drop reads as a new phase. Not needed. The spec (`TABLE3D.md:285-293`) and both pages already say it.
6. **No cue and no announcement for the tally changing.** `showTally` sets the numbers at once with no `pop` (the HUD's `replay(b.num, "pop")` is available), and the running numbers sit in a `role="img"` label, which a screen reader does not announce; `sr.trick_won` does not carry the tally. Optional: add the tally to the live sentence for `trick_won`, or say nothing and accept it.
7. **Tests for the wiring.** `tally.test.js` pins the function; nothing unit-tests `showTally`, the restore path or the moment (finding 1), and no played seed is asserted to include a valat, a draw or a pagat ultimo (they are linear, so they hold, but say so). Add a constructed valat (27 tricks to one seat is 210 : 0) and a draw (two 105-thirds takes) to the first test. Run `browser_scenarios.py` where Playwright is installed before calling the package done.
8. **German settings use "Punkt/Punkte" for payments** (`strings.json:265, 267, 268`) where every other German string says "Point(s)" for the same thing. Use "Points" ("Points für ein einfaches Spiel ...", "1 Point", "2 Points").
9. **German: one thing, two names.** The pages call the honours "Honneurs" (`tutorial.md:108, 172`) while the HUD row, and the new text at `:251`, say "Ansagen und Ultimo". Pick one for the row the reader sees, or keep the row and say "Honneurs (die Ansagen)" once.
10. **Spec leftovers.** `TABLE3D.md:280` names the tutor page "the count in thirds" (now "Card values and the count"). 2.5.3's first paragraph still describes only the count; one sentence pointing to the running tally paragraph below it would do. The `thirds` page and 2.5.3 say a side that reaches 35⅔ "lights the finish"; with the running tally this also happens during play (a declarer may be certain of the game before trick 27). True and harmless; the HUD description could say "as soon as".
11. **`cards.rs:375` reads a file in `web3d/` through a relative `include_str!`.** Fine inside the workspace; it fails if the crate is ever packaged alone. Note it next to the test or copy the fixture under `crates/` and let the Node test read that.

### Verdict

Correct and tested: the tally is exact, public, and ends on the count's own numbers for every case considered; the fixture binds Rust and JS; the wording fixes hold across the pages. No bug. Fix 1 (the moment, or the spec) and 2 (the last count-as-Point uses and the recorded decision) before calling the work package done; 3 to 11 may be recorded as open in `PLAN.md`.
