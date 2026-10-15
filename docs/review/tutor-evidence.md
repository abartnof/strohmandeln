# Review: the tutor's evidence (deal record, skills, classifiers)

Scope: `git diff efd6251 worktree-agent-a1792c49f206fa451` (b03a744, 6e18ae0,
dd7d954, b0cdcd0): `strohmandeln-core/src/{dealrec,session,view}.rs`,
`strohmandeln-ai/src/{tutor,tutor_tests}.rs`, the `docs/TUTOR.md` edits, against
`docs/TUTOR.md` 2-4, 11, 12, `research/16-skill-ladder.md`, `docs/RULES.md` 5, 11
and `CLAUDE.md`. `cargo test -p strohmandeln-core -p strohmandeln-ai` passes
(157 + 142 + 12 + 7, debug, 76 s); `cargo clippy --all-targets` is clean. The
numbers below come from throw-away tests added and reverted in the worktree (the
18 deals of `students().whole`, the `evidence_tests` fixture); they are not kept.

Severity: bug (wrong result), should (fix or record open before the package is
done), nit.

## Verdict

Accept with fixes. The structure is sound and there is **no leak**. Four items
need a fix or an open entry before "done": 1 (`off` is judged inside the advisor's
noise), 2 (the exact margin is used where the search is not exact), 3 (the
assisted numbers cannot be supplied by the table as the engine stands) and 4
(a stored record can trap the module). The rest can be recorded.

## What was checked and holds

- **No leak.** `evidence` gives the closure `deal`, but only `bare_view(deal,
  seat)` (a wrapper of `deal_view`) reaches the classifiers and the advisor
  (`tutor.rs:614-660`). `dealrec` test `a_scored_deal_replays_...` asserts that the
  views taken while replaying are equal to the views the sitting gave, deal by
  deal, over 6 sittings (>= 20 deals); `a_bare_view_is_...` asserts `bare_view`
  equals `deal_view` plus the mover's legal actions. `blind_check_values`
  (`testutil.rs:116`) really builds twins: it shuffles the cards of
  `scene.hidden_from(seat)` (opponent hand and every face-down packet card) among
  their deck positions, replays the same history, skips a twin whose view differs
  and compares the decision of the twin to the original (> 100 twins asserted).
  Its weakness is scope, not soundness (finding 12). `Miss.consequence` reads
  `deal.tricks()` after the replay has finished, every card of which is public.
  `led`, `trick`, `tarocks`, `sure_thirds` come from the human's own view.
- **Classifiers agree with the spec table** for trigger, met, cost and direction:
  `worth` (two right classes, `abs` gap, `taker` for the trick's thirds),
  `cheap` (spread within the class of the card played, met within the margin),
  `off` (cover-right only, `Figure` for strength 5 and 6 = Cavall and Dame,
  checked against `Card::value`), `run` (tarock led, best lead a tarock, at least
  two tarock leads), `void`/`behind` (leader only, other class better), `endgame`,
  `announce` (below). Costs are 0 when met. `Skill::needs` equals the spec table.
- **Determinism.** Draws are `Rng::stream(EVIDENCE_SEED, view_seed(view) ^ 0x700x)`,
  nothing hashes a map, `costliest` keeps the first of equal costs, the budget is
  nodes not time. Same record, same list, same result (test). Bid and play use
  different stream tags.
- **Replay edge cases that work:** thrown-in deal (two `Pass`), doubled deal
  (the record carries `deal.multiplier()`), void deals (never recorded: `deck` is
  assigned only when `void_seats()` is empty), a deal with no human choice (all
  tallies 0, no `costliest`), a refused action (`Err(index)`, the half-built
  chances are dropped), a duplicate card in the deck (refused by the engine).
  Valat is not an action, so nothing special. `Sitting::import` rebuilds `deck`,
  `deal_actions` and `last_deal` because it replays through `apply`.
- **`announce` and RULES 11.** `legal_actions` lists an `Announce` for each honour
  held and neither announced nor forfeited (`deal.rs:403`), so `k.announceable` is
  exactly "held, unannounced". A `Play` with an empty `announce` list of a card of
  such an honour is the forfeit of 11.5, so "met = announced, missed = played
  silent" is the `honour_forfeited` signal; 12 thirds x multiplier is the Point
  doubled (11.1, 14.2). An honour announced by its own action counts once (met);
  the later play of its card is not announceable and gives no second chance.
- **Void.** `shown_void` is the counter rung's `opp_void` (set only when he could
  not follow my lead; the playable set is hand plus covers, RULES 5.4) and lifted
  while a cover of his or a known pulled card shows the suit. A card of the suit
  that he showed and played away leaves him still unable to follow, so the flag
  stays true correctly. The Sküs gets no special following rule (5.4), so no false
  void.
- **Timing.** `evidence_latency` (release, native, 18 deals): median 31 ms, p95
  39 ms, max 40 ms; the 42-56 ms in TUTOR.md 11 is under a loaded VM. Consistent.

## Findings

### 1. should - `off` is judged inside the advisor's noise (`tutor.rs:42`, `:335`; `tutor_tests.rs:205-215`)

Rung 4 meets 14 of 24 `off` chances (58%), against 85-96% on every other play
skill (`print_students`: worth 28/33, cheap 44/48, run 19/20, void 23/26, behind
20/22). It is advisor noise, not a classifier bug, and not a stream mismatch:

- The student's draws are `Rng::stream(opp.seed, view_seed(sitting view))`, the
  tutor's `Rng::stream(EVIDENCE_SEED, view_seed(bare view) ^ 0x7001)`; they cannot
  share a stream (different views, different seeds) and need not. Any second
  draw of the same view disagrees as often.
- Measured: for each of the 178 human decisions with a cover and a hand card both
  legal in which some draw gave a gap above `MARGIN`, I re-drew the advisor under
  8 independent seeds (10 worlds, 30 000 nodes, the evidence budget). The
  within-position standard deviation of (best cover - best hand) is 6.2 thirds
  (mean; median 5.2), about the size of `MARGIN` = 6. In 29 of the 178 positions
  one seed exceeds +6 and another -6. For the 24 positions with gap > 6 under the
  first seed, the other seven seeds had a positive gap in 122 of 168 draws (73%)
  and a gap above 6 in only 84 of 168 (50%). The student's 58% agreement is of
  that order (its own draw also has the near-tie maxims).
- The trigger is one-sided (cover right only), so it selects positive noise
  (winner's curse); `worth` is two-sided and does not. The cover's value moves
  with the three face-down cards under it, which differ in every sampled world, so
  the difference is far noisier than a difference of two hand cards.
- The test pins the noise: `rate(whole, Off) > 0.4`.

Effect: a person who plays exactly like rung 4 meets `off` 58% of the time, so
`top_rate(off)` is about 0.6, `master` about 0.5, and a fair share of the 42%
"misses" are noise (whole rung 4 books 106 thirds of `off` misses in 18 deals,
more than any other skill). Fix: M1 must set `MARGIN` per skill, or the trigger
must use the paired per-world differences that `MoveTable.per_world` already
holds (as `hint.rs` `odds_for`): mean difference above `max(MARGIN, 2 x standard
error)`. Replace the `> 0.4` assertion by "whole rung 4 meets at least 0.8 of
every skill's chances" once it holds, and keep the 18-deal fixture as the check.

### 2. should - the exact margin is used where the search is not exact (`tutor.rs:217`, `:290`, `:411`, `:472-486`)

`exact(k)` means "no face-down packet card", and selects `EXACT_MARGIN` = 1 third
and `worlds = 1`. The search is exact only if it reaches the end of the deal. With
the default budget (30 000 nodes, one world) it does not: over the human's
unassisted choices in the 18 deals, 29 of 72 positions classed exact have 6 or
more tricks left, and the search there reaches depth 2.0 to 4.3 on average (6 left:
4.3, 8: 2.5, 10: 3.0, 14: 2.0); 5 or fewer left it is complete. At depth 3 with a
margin of 1 third the "best less worst" is the evaluation's guess, so `endgame`
(whole rung 4: 26 of 31, 84%), `worth`, `cheap` get a false-precision margin, and
`endgame` is told as a known-position skill when it is not known.

Fix: decide exactness after the search (`table.depth >= 28 - k.trick_number`,
the same formula `move_table` uses) and take `EXACT_MARGIN` only then; otherwise
`MARGIN`. This needs `classify_play` to take the margin or an `exact: bool`
(it recomputes `exact(k)` itself now, and `decision_verdicts` cannot pass the
answer in). Alternatively spend more nodes when exact (one world, so 10x costs
the same as ten worlds). Add a test with a position 8 tricks from the end.

### 3. should - the table cannot supply the assisted numbers yet (`session.rs:269`, `dealrec.rs:5`, `docs/TUTOR.md:94-97`)

(a) Numbering is over both seats' actions of the deal, announces and bids
included. `Sitting::deal_actions` is private and there is no accessor and no
field in `render`/`hint`; `last_deal()` exists only after the deal has ended.
The table could count its own `send` and `step` calls, but a reload (import)
or a draw/extension in between loses the boundary: the exported move list has no
deal boundaries, so the count of the deal in progress cannot be recovered. Fix:
`Sitting::deal_action_count()`, returned in the hint reply (`action: n`) and in
`render`, so the table stores the number the engine gave, not one it counted.
(b) Granularity: a hint is shown for one position, and the human may then do two
actions in the same turn (`Announce`, then `Play`). Marking only action n leaves
action n + 1 (the play, which the hint was about) as evidence. Fix in
`evidence`: an assisted index covers the following consecutive actions of the
same seat (they are one turn), with a test. (c) `docs/TUTOR.md` 3 still says
"the numbers of the human's actions (action n of the deal)"; the code and the
`dealrec` header count both seats' actions. Reword to "action n of the deal,
counted over both seats".

### 4. should - a stored record can trap the module; an unfinished one gives evidence (`dealrec.rs:35-60`)

Records will come from `localStorage` through `strohmandeln_evidence_of`. A deck
of the wrong length reaches `assert_eq!` in `Deal::new` and panics (confirmed:
deck with one card removed); a multiplier of 0 replays "successfully"; a record
cut short (10 actions) returns `Ok` with evidence, while `strohmandeln_brief_of_record`
is specified to return `null` for an unfinished deal. Fix: validate in `start()` or
`replay` (54 cards, a permutation, multiplier 1 or 2) and return an error instead
of panicking (a `ReplayError` variant for a bad record); have `evidence` return an
error (or a flag) when `deal.end()` is `None`. Tests: short deck, multiplier 0,
truncated record, each an `Err`.

### 5. should - the cost unit is not one unit (`tutor.rs:55`, `:426-440`, `:446-460`)

(a) `THIRDS_PER_POINT = ULTIMO_THIRDS` = 12 is the solver's weight of the ultimo
inside the final margin (`sim.rs:18`), not an exchange rate between a game Point
and thirds of the count margin; nothing derives it. By my estimate (not measured)
the rate is higher: the payment is a step at 107 thirds and the final margin of a
deal has a spread of tens of thirds, so one third of margin moves the chance of
winning by about a hundredth, i.e. a few hundredths of a Point; a Point is then
tens of thirds, not 12. If so `bid` and `announce` costs are weighted too low
against the play skills, or too high if the true rate is below 12. M2 decides; the
constant's comment should say it is borrowed. (b) `bid` and `announce` costs are
multiplied by the deal's doubling; play costs (thirds of margin) are not, though
a doubled deal pays twice for a card as well. The learner compares gains across
skills, so a doubled deal inflates two skills against eight. Scale all costs by
the multiplier or none (the spec only says "scale its margin", section 4).

### 6. should - `pagat` chances are mostly trivial (`tutor.rs:370-392`)

Whole rung 4 has 36 `pagat` chances in 18 deals (2 a deal) and meets all; a
random player meets 42 of 49 (86%) against `master` = 0.85 x `top_rate` (1.0) =
0.85, so a random player would sit at the threshold. Of the 36, 28 are leads: the
Pagat as a lead is almost always worse by more than 6 thirds than the best tarock
(so "keep" is trivially right), not a decision a learner has to learn. The spec
asks for "holding T1 and another legal card" and the advisor's class, which this
implements; the result is a skill with a near-zero signal. Fix before `pagat` is
given a `MIN_CHANCES`: count a chance only where the Pagat is within `MARGIN` x k
of the best card or is cashed by rung 4 (the sources' cases are cash with few
tarocks and the silent ultimo), or fold the lead case into `run`. Record as open
if M3 decides it. Also: `tarocks` is the playable count at the decision (hand plus
covers), which falls during the deal; the zones (7 or fewer, 12 or more) are told
from it by the brief, so say in the doc that the count is "now", as the sources
mean ("neben nur wenigen ... Taroks"), and that the direction (`Cash`/`Keep`) is
not tied to a zone here: a `Keep` miss at 7 or fewer is still returned.

### 7. should - bid margin and the bid test (`tutor.rs:48`, `:426`; `tutor_tests.rs:229-240`)

`BID_MARGIN` = 0.5 Points is about 1.6 standard errors of a 40-world estimate if
the per-world payoff has a spread of about 2 Points (my estimate; M1 measures).
The end-to-end bid test has 9 chances, all "pass is right": `AlwaysTake` misses 0
of 9 and whole meets 9 of 9, so the "take is right" branch and the cost of a
missed take are exercised only by the two hand-fed unit tests. Add a student who
always passes (or a seed set with strong hands) so both branches run end to end.

### 8. should - the lesioned-student tests are thin (`tutor_tests.rs:31-60`, `:205-240`)

Three of the nine knockouts of `docs/TUTOR.md` 11 M4 / 12 are built (`off`,
`announce`, `bid`); `worth`, `cheap`, `run`, `void`, `behind`, `endgame` have
none, so the classifiers are untested against play that actually lacks the skill.
`NoCover` is a total lesion (0 of 47 against 14 of 24): it is diagnosed in two or
three deals under any margin. A partial lesion (covers half as often as rung 4:
rate 0.3 against 0.6, 1.3 chances a deal) needs about 46 chances, i.e. about 35
deals at the present margin (normal approximation, 80% power, my calculation), so
at the window of one sitting (6 to 12 deals) only a total lesion shows. That is
a consequence of finding 1; the test should keep a partial-lesion case as the
measure of the margin. The whole-versus-random assertions (0.85 against 0.88
observed) are close to the measured value and will need re-setting when a
constant changes; state them as the fixture's current values.

### 9. nit - `move_table` fallback returns rung 3's scores (`tutor.rs:491`)

When the budget finishes no trick (`counted == 0`, a small `Budget` for a slow
device, M7) `rung4::move_table` returns rung 3's scores, a different scale, and
`classify_play` treats them as thirds. Skip the decision when `table.per_world`
is empty.

### 10. nit - `behind`'s constants are borrowed from a different definition (`tutor.rs:61`, `:259-272`)

The doc says the split is "as rung 2's 30% to 70%"; rung 2 takes `mine / 18` (my
points over the suit's full 18, `rung2.rs:233`), the tutor takes `mine / (mine +
theirs)` over the unplayed points (as `research/16` says). The range 0.3-0.7 does
not carry over; `theirs` also holds my face-down packet cards (stated in the
comment), which biases the share low by up to my hidden fraction. M8 must set the
range for this definition; reword the comment so the borrowing is not implied.

### 11. nit - duplicated learner types (`tutor.rs:493-560`)

`Chance`, `Tally`, `Summary`, `from_chances` repeat `kit_learner`'s
(`kit/learner/src/model.rs`, merged on main, same fields). The local
`from_chances` lacks the guard that counts a non-finite or negative `gap` as 0.
On merge, use the kit types (a `Ladder` constant for the version and the slugs)
and keep only `Verdict -> Chance` here.

### 12. nit - scope of the hidden-card test (`tutor_tests.rs:348-370`)

The twin check runs `decision_verdicts` on the first and the last legal action
only, with a tiny budget, over random walks. It does not run `evidence` (so
`Miss` fields and the assisted skip are outside it) and it takes views from
`Scene`, not `bare_view`; the second gap is closed by the `dealrec` view-equality
test, the first is harmless because the `Miss` is built from the same view. A
one-line `evidence` call on a twin pair (compare `costliest`) would close it.

### 13. nit - smaller points

- `evidence` accepts a record whose `human` seat made no moves (all zero): fine,
  but `human` is unchecked against `Seat`; no issue.
- `run` is silent when the human leads a suit card where a tarock lead is best
  (only "which tarock" is judged, as `research/16` states); say so in the doc
  comment, since it is a deliberate non-chance.
- `Consequence` has one variant and `Evidence.chances` duplicates `summary` (used
  by the tests and by the brief's later passes); keep, but both are dead outside
  tests today, as are `Skill::stage`/`needs`.
- `docs/TUTOR.md` 11's "about 1 hour 10 minutes" total: the arithmetic (ten
  knockouts about an hour) is consistent with the new M1 figure; wasm is not yet
  measured, as the table says.

## Open for the next package (not findings against this one)

The learner-level diagnosis (`docs/TUTOR.md` 12: a lesioned student given the
focus within K deals; the prerequisites) needs the constants of M3 and M6 and
cannot be judged here. Findings 1, 2, 5 and 6 change what those constants are
fitted to, so they should land first and bump `EVIDENCE_VERSION`.

## Second pass (2026-10-15)

Scope: `git diff f3e3b51 HEAD` without `web3d/strohmandeln.html` and without
what came in with the merge of main (1628eb1): commits 42e75e1 to ff3c2a7
(`tutor.rs`, `tutor_tests.rs`, `students.rs`, `rung4.rs`, `dealrec.rs`,
`session.rs`, the wasm `lib.rs`, `golden.txt`, `docs/PROTOCOL.md`,
`docs/TUTOR.md`). `cargo test -p strohmandeln-core -p strohmandeln-ai
-p strohmandeln-wasm` passes (ai 190, core 150 + 12 + 7, wasm 26 + 2; 2 min 21 s
wall on the 2-core VM); `cargo clippy --all-targets -D warnings` and `cargo fmt
--check` are clean. Numbers below come from `print_students`,
`print_pagat_variants` (both kept, ignored), per-test timings and one throw-away
edit, reverted.

### Verdict

Accept with fixes. No bug and no leak. Findings 1 to 5 are *should*: fix or
record open in `PLAN.md` before the package is called done. The first pass's
findings 1 to 7, 9, 10, 11 are fixed as claimed; 8 and 12 are closed in part
(findings 4 and 12 below).

### Checked and holds

- **Golden hashes.** Only hashes moved; event counts, final Contos and balances
  of all six sittings are unchanged. The cause is `state.deal_actions`, which is
  now in every reply the hash covers (FNV over the raw reply bytes). Reproduced:
  with the `deal_actions` insertion removed from `render` and the golden file of
  1628eb1 restored, `golden_games_replay_byte_for_byte` passes with the *old*
  hashes. The opponent's decisions are therefore unchanged. The diff to the AI
  crate adds `bid_samples` (a new function; `bid_advantage`, `move_table`,
  `decide` untouched), `students.rs` and `tutor.rs`; nothing the opponent calls
  moved. The browser tests' seeds play the same game.
- **Standard error** (`tutor.rs:270`): paired per world, difference `a - b` of
  the same world, sample variance with `n - 1`, standard error `sqrt(var / n)`,
  worlds that lack either card dropped. It assumes `scores` and `per_world` cover
  the same worlds with every legal card in each (`move_table` does); then the
  mean of the differences equals the gap of `scores`. Not tested numerically
  (finding 2).
- **One rule everywhere.** Every play skill's trigger and every spread skill's
  miss goes through `Advice::clearly_better` (gap above the margin *and* above
  twice the standard error); the exact margin applies only when
  `search_complete` (nothing face down, `depth >= 28 - trick_number`, and
  `trick_number` is the sampler's `sim.trick`). `bid` has its own pair in
  `classify_bid`, `announce` needs no search. Unit test covers worth, cheap, off,
  pagat, void, behind, run with noise 0 and 30.
- **`bid_samples`** draws in the order of `bid_advantage` and the test pins
  `multiplier * mean == bid_advantage` to 1e-9. The two loops are duplicated;
  keep it so: deriving one from the other would change float rounding in the
  opponent's `> margin` comparison, and "the opponent is unchanged" is worth the
  duplicate.
- **`dealrec`.** `check` rejects a deck that is not a permutation of the 54,
  multiplier outside 1 to 2, ultimo outside 1 to 2 and a hand without a tarock;
  `start`, `replay`, `replay_to_end` return `Err`; a thrown-in deal is finished
  (`end()` is `Some`). No caller of `start()` is left outside the module.
- **Action count.** `Sitting::deal_action_count` is `deal_actions.len()`; it is
  cleared when a non-void deal starts, taken at the deal's end, not touched by
  `Extend`. Reload gives the same number (test). The wasm test pins
  `state.deal_actions` and `hint.action` over 40 steps.
- **`assisted_actions`**: an assisted `Announce` extends over following actions
  of the same seat to the card; an assisted `Play`, `Take`, `Pass` covers itself
  only; an index past the end is ignored; `covered` is sized to the record so
  `evidence` cannot index out of range.
- **web3d.** Nothing reads `state.deal_actions` or the reply's `action`.
  `hinttext.js:27` and `main.js:873` read `r.hint.action` (the string
  `"play"`), not the new top-level field. `engine.js` passes replies through.
  No table change is needed until the table stores the assisted list.
- **No leak.** Unchanged in kind: classifiers see `bare_view` only; the twin
  test now runs `decision_verdicts` on every legal action (more than 100 twins).

### Findings

#### 1. should - the ladder's constants predate the noise rule (`tutor.rs:622-636`)

`top_rate`/`top_cost` are the figures of the first pass's 18 deals (off 0.58 =
14/24, worth 0.85 = 28/33, cheap 0.92, 5.9 thirds of `off` a deal). The rule
changed what rung 4 meets: `print_students` (4 sittings) gives worth 32/35, cheap
60/60, off 14/15 (the test comment says 17/18 over 6), run 18/19, void 14/14,
behind 14/15, endgame 34/37, pagat 6/6, bid 12/12. With `off` at 0.58 the bar
(`master` at 0.85 of `top_rate`) is 0.49, which a player who covers half as often
as rung 4 clears. Costs are also censored now: a spread-skill miss is at least
the margin (6 thirds), so `top_cost` is conditional on that. The doc says
"indicative only", but `EVIDENCE_VERSION` was bumped for this change, which is
where the constants are meant to be refitted. Fix: refit from `print_students`
at 9 sittings (as the pagat figures use) before `off`/`behind` are used, or
record the constants as open for M3 in `PLAN.md` and set `off` to the measured
value now. Set `pagat` `top_rate` to the measured 0.94 (32/34), not 1.0.

#### 2. should - one counted world switches the noise rule off (`tutor.rs:284`)

`std_error` returns 0 for fewer than two worlds, so `clearly_better` falls back
to the bare margin. That is right for an exact position (one world, solved) but
wrong where the budget finished only one world of ten (a small `Budget` on a slow
device, M7; `counted == 1` in `move_table`): the noisiest estimate gets the
loosest test. It is also the same for a known but incompletely searched position
(`hidden_total() == 0`, `exact` false: one world by design), where the error is
the horizon's, not the sampling's, and the margin of 6 is all there is. Fix:
`std_error` returns infinity for `n < 2` when `!exact`, except the known-position
case, which keeps the margin (say so in the doc comment). Add tests of the number
itself: two worlds with differences 0 and 2 give `sqrt(1 / 2)`; a world missing a
card is dropped; one world.

#### 3. should - the pagat figures that justify the gate cannot be reproduced (`tutor.rs:455-460`, `docs/TUTOR.md` 4)

The comment and the doc give "rung 4 107 of 107, random 83 of 91 leads over 9
sittings". `print_pagat_variants` still has a "leader" row, but since b3ffb01 the
code gives no leader chance: the row is `0 of 0`. The following figures reproduce
(32 of 34, random 15 of 29; "sourced zones" 17 of 17 and 14 of 24). `CLAUDE.md`
asks that a computed number a document relies on has a check that reproduces it.
Fix: have the print function judge leaders with the gate bypassed (a
`pagat_chance(k, advice, following_only: bool)` parameter, or a copy in the
test), or cite the leader figures as a dated measurement at b3ffb01 and say they
are not reproducible at HEAD. Statistics of the gate: 32/34 against 15/29 is
decisive (Fisher one-sided p about 1e-4, my calculation); the fixture's 6/6
against 9/15 is not (p = 0.09), so the 6/6 is not evidence by itself. For a
learner the chance rate is about 4 a sitting; at the bar of 0.85, 10 chances
(about 3 sittings) pass a random player 1.5% of the time and rung 4 (true rate
0.94) 88%; 5 chances 4% and 73%; 2 chances (`min_chances` 2) pass a random player
27% of the time. Enough to tell over a few sittings, not within one; raise
`min_chances` of `pagat` and `bid` (3 chances a sitting) to 4 or let M3 decide.

#### 4. should - the knock-outs are not specific, and two are coins (`students.rs`, `tutor_tests.rs:265-345`)

Each test asserts only the target skill's drop, not that nothing else fell. It
does not hold: the no-cover student (`print_students`) meets void 5/7 (whole
14/14), behind 15/19 (14/15), cheap 41/47 (60/60), and has 4 endgame chances
against 37, because packets stay face down longer. A forced hand card is also a
miss of `cheap`/`void`/`behind` wherever the cover was the best of its kind. This
is inherent to a class lesion, and matters for the learner (prerequisites are
meant to absorb it, TUTOR.md 2), so it should be recorded rather than hidden.
`Pagat` and `Bid` are 50% coins (10/20, 9/17 against 18/18, 17/17), not total
knock-outs as the module header says; `Announce` is judged on 4 chances. Fix:
(a) record the matrix of all skills' rates for each knock-out in `docs/TUTOR.md`
11 (M4) from `print_lesions`; (b) add to `lesion_shows` that the target has the
lowest met rate among skills with at least 5 chances, so a knock-out that hurts
something else more fails; (c) reword the header ("one skill removed or
halved"). The thresholds of the coin tests (0.25 below, 4 and 5 chances) were
set against one realisation of the seeds: deterministic, but any change to the
advisor re-rolls them, and a rate of 8 in 10 by chance at p = 0.5 is 5%.

#### 5. should - keep the gate fast (`tutor_tests.rs`, `bin/gate`)

`bin/gate` runs `cargo test` for the workspace before every commit; the ai lib
tests now take 122 s wall (150 s CPU for the lesions; first pass: 76 s for core
and ai together). Per test, alone, in seconds (each includes the shared fixture,
about 20 s of 12 sittings, built once when run together): pagat 45, endgame 41,
bid 41, run 33, void 32, always-take 31, behind 28, off 28, announce 28, worth
27, cheap 26, and 19 to 31 for the five tests that only need records
(`an_assisted_decision`, `an_assisted_announcement`, `the_costliest_miss`,
`a_search_that_finished_no_trick`, `the_evidence_is_a_function`,
`a_record_that_does_not_replay`, `whole_rung_four`). The test profile is already
optimised, so release does not help. Proposal, keeping the check:

1. `#[ignore = "lesions, about 100 s: bin/lesions"]` on the eleven
   `a_student_who_*` tests and on `whole_rung_four_meets_nearly_every_chance...`
   (they all need the three-student fixture). Keep `a_student_who_never_plays_a_cover_off_misses_every_off_chance`
   un-ignored on a one-sitting fixture (0 of 34 against 14 of 15 is the
   strongest signal; a few seconds): the gate still proves that a lesion is seen.
2. A one-sitting fixture (`whole[..]` of seed 0, about 5 s) for the tests that
   only need records. The assisted-announcement test needs a deal in which the
   human announces; keep one hand-picked seed for it.
3. `bin/lesions`: `cargo test -p strohmandeln-ai --lib -- --ignored a_student_who whole_rung_four`,
   which on success writes `target/lesions.stamp` holding a hash of
   `crates/strohmandeln-ai/src/{tutor,students,tutor_tests,rung2,rung3,rung4,know,sampler,solver,sim,opponent}.rs`
   and `crates/strohmandeln-core/src/{deal,dealrec}.rs`. `bin/gate` compares the
   hash and fails with "run bin/lesions" only when they differ, so a commit
   that does not touch the advisor or the classifiers pays nothing, and a
   commit that does cannot skip the check silently. The merge to main (the
   publish step) runs `bin/lesions` unconditionally. The gate stays about 25 s
   dearer than before the package instead of about 100 s.

#### 6. nit - the argmax is not in the standard error (`tutor.rs:294`)

The error is that of the two chosen cards' difference; the cards were chosen as
the best of their classes by the same worlds, which biases the gap upward when a
class holds many cards (cover against up to 15 hand cards; `cheap`, `endgame`
over every legal card). The rule is a large improvement (off 58% to 93%) and the
fixture shows no excess of misses, so no change now; M1 should measure it, with
the independent-seed redraw of the first pass's finding 1, and say whether the
paired maximum over the class per world is needed.

#### 7. nit - the margin censors small losses (`tutor.rs:250-300`, `docs/TUTOR.md` 4)

A spread card is met unless it is worse than the best by more than the margin
and twice the error, so a player who loses 3 to 5 thirds on every decision is
perfect in the evidence, and a miss costs at least 6 thirds. Not undetectable (a
random player's `cheap` is 34/65 and costs 608 thirds in the fixture; the
dearest-card student 11/65), and it follows the spec ("within `MARGIN` of the
best"); say in the doc that the margin is a resolution, and that `top_cost` is
conditional on it (finding 1).

#### 8. nit - the mutation test is narrow (`dealrec.rs:328-360`)

300 cases, one base record, mutations of the deck, the action list and the
dealer only. No mutation of an action's contents (seat, card, `announce` list,
honour), multiplier or settings (those have unit cases). Leftover `let _ = n`
(`dealrec.rs:358`). The test takes well under a second; raise to 3000 cases, add
a mutator for the contents of one action and for the seat, drop `n`.

#### 9. nit - wasm reply (`strohmandeln-wasm/src/lib.rs:752`, `docs/PROTOCOL.md:345`)

The hint's top-level `action` (a number) sits beside `hint.action` (a string),
and both are named in the same paragraph; rename the new field `for_action` or
`deal_action` (nothing in web3d reads it yet, so the change is free now). The doc
comment on `strohmandeln_hint` still lists the old shape. The PROTOCOL paragraph
was re-flowed into one long line.

#### 10. nit - small inconsistencies

- `docs/TUTOR.md` 4 says rung 4 met 91% of `off` after the rule; the fixture says
  14/15 (93%) and the test comment 17/18 (94%). State the fixture's figure and
  its size.
- `Lesion::None` is "rung 4 whole" but is re-built through `play_card`; assert
  once that `Student::decide` equals `Opponent::decide` for `Lesion::None` over a
  sitting, since the tests treat it as the opponent.
- The twin test does not call `evidence` (the `Miss` fields `led`, `trick`,
  `consequence` are outside it); unchanged from the first pass's finding 12,
  harmless for the reasons given there.
- Partial-lesion case of the first pass's finding 8 (a student that covers half
  as often, to measure what the margin can see in one sitting) is still absent;
  the two coin lesions are the nearest and are not measured as a rate of power.
