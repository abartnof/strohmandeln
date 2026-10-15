# Review: the tutor's measurements (M1 to M8) and the constants they set

Scope: the measurement branch `worktree-agent-a4b2a4bd8a5be7ec8` as merged to
main (35df212), its own commits `d0f42b8..` (153f3d3 to 6950d82) and the earlier
ones it carried (a93c166 to 22bc48f): `strohmandeln-ai/src/{measure,rules,tutor,
lessons,students,tutor_tests,brief_tests}.rs`, `src/bin/measure.rs`,
`strohmandeln-wasm/{examples,tests}/evidencebench.*`, `kit/learner` (`may_focus`,
its tests, `STAMP`, `README`), `docs/MEASUREMENTS.md`, `docs/TUTOR.md` 2, 5, 11,
13, against `CLAUDE.md` (the simulation rules). `cargo test -p kit-learner -p
strohmandeln-ai` passes (258 + 17 ignored, kit-learner 51; debug, 92 s), `cargo
clippy -p kit-learner -p strohmandeln-ai --all-targets` and `cargo fmt --check`
are clean.

Severity: bug (wrong result), should (fix or record open before the package is
done), nit.

## Verdict

Accept with fixes. The statistics are sound where they were checked, the plans
precede the results in the log, no plan text was edited after its commit, the
stopping rules are the ones planned, every constant in the code equals the one
`MEASUREMENTS.md` derives, and the numbers reproduce (M2, M3 and M8 to the digit; M1
to the digit except the `behind` row, finding 2). Five items need a fix or an open
entry before "done": 1 (M4's result is filed under M5), 2 (M1's `behind` row does
not reproduce at HEAD), 3 (the `m6` command cannot reproduce its own
"provisional" rows), 4 (the shipped `worth.rich` rule is told under a wider
condition than the one M5 measured) and 5 (stale statements in `TUTOR.md`). The
rest can be recorded.

## What was checked and holds

- **Plan before result (1).** `git log --follow docs/MEASUREMENTS.md`: the plan
  of each of M1 to M8 is its own commit, earlier than the commit that holds its
  result (M1 a93c166 19:31 / 7b76453 19:46; M2 befc488 / 81fa260; M8 8e4a47b /
  c2ac258; M3 3177bbd / 3f9e8ae; M4 e2c6998 20:11 / 797468f 21:06; M5 22bc48f
  20:25 / d61e84a 21:21; M6 0d5a4eb 21:22 / 79eacf2 21:33; M7 3aabf41 / f6b929e).
  Diffing each plan commit against its result commit shows no deleted or
  rewritten plan text; the one edit is M1's (9dc67f0, "what is eligible", made
  before the run and noted). For M6 the session scratchpad corroborates the
  order: the run's binary is stamped 21:22, the window caches 21:24 and 21:25, the
  log 21:31, all after the plan commit at 21:22:27.
- **Stopping rules as planned.** `stats::run_sequential` is the shared
  O'Brien-Fleming routine (`obf_constant`, boundaries `c * sqrt(K/k)`: 4.05,
  2.86, 2.34, 2.02 for 4 looks); `m4` builds `Plan { batch: 40, looks: 4 (singles)
  or 3 (pairs), negligible: 0.5 }` exactly as the plan says. Each verdict in the
  M4 table agrees with its logged z against its boundary (Run 2.34 at look 3
  against 2.337, the closest; Announce 2.16 against 2.02; Endgame 3.22 at look
  4). Seed blocks are disjoint and are the ones listed (checked in `bin/measure.rs`
  defaults and the doc's commands). M1, M2, M3, M5, M6, M7, M8 are descriptive
  counts with no looks, and say so.
- **Mirrored pairs.** `tournament::mirrored` plays each seed twice with the
  factories swapped and returns `first - second`; with `Sitting::balance` =
  Conto(A) - Conto(B) the observation is the student's Points over the 12 deals of
  the pair, so "Points a pair of 12 deals" and the worth `-effect / 12 * 23` are in
  the units the doc says. M3, M5, M6 alternate the player's seat by sitting.
- **M1** computations (`noise_of`, `replication`, the three margin rules) match
  the plan's definitions and have tests with hand values; the calibration compares
  the spread of a fixed pair with the reported error, which is the right check of
  the error's honesty; the held-out mean for the winner's-curse and sign checks
  excludes the stream itself. Reproduced: see 4 below.
- **M2** slope is OLS with intercept and the cluster-robust (CR1) error by sitting
  (`ols_clustered`, tested against a hand-computed 0.1 and 0.26726); `1/slope`
  with a delta-method interval, rounded to 23 as planned.
- **M5** estimator: the ratio estimator by deal (zero-firing deals are kept as
  clusters, so G is right), `n_eff = p(1-p)/se^2` capped at the firings, Wilson
  lower bound at `n_eff` (`wilson_lower` has known values); selection on seeds
  50,000,000, confirmation of the selected candidates only on 55,000,000, each
  setting on its own block (56 to 59 million); the verdict table follows the rule
  (`worth.poor.1` 0.799 fails by the rule as written).
- **M6** has no leakage in code: `m6_choice` is called on `scored` computed from
  the tuning groups only; the fresh groups are scored only for the report; the
  windows are `first + w * sittings`, 60,000,000 to 60,000,017 and 65,000,000 on,
  disjoint, and the lesion test uses 66,000,000. The grid is the planned 540 sets
  (a test asserts `5*3*3*4*3`), the qualification rule and tie-breaks are the plan's.
- **Constants equal their derivations (3).** `MARGIN` 10, `EXACT_MARGIN` 1,
  `BID_MARGIN` 0.45 (2 x 0.217 = 0.434, to 0.05), `THIRDS_PER_POINT` 23 (23.48),
  `SPLIT_SHARE` (0.1, 0.6), `SPLIT_MIN_POINTS` 6, every `LADDER` line against the
  M3 table (rates, costs, and `min_chances` = floor(chances a deal x 3) clamped 2
  to 4, recomputed for all ten), `min_gain` 1.1 (half of announce's 2.12 thirds a
  deal = 1.06, to 0.1), `costly_gain` 4.4 (4 x), `costly_chances` 8, and the M6
  `PARAMS` (0.88, 0.2, 0.8, 0.80, 0.2). The derivation rule for `min_gain` is the
  planned one (half the least clear worth; announce is the least).
- **`may_focus` (5).** Semantics are right, including the part in question: a
  praise-only prerequisite that is short must **not** block. If it did, a skill
  that can never be chosen would hold its dependents (`behind` needs `off` and
  `void`; `endgame` needs `cheap` and `void`) back for good; the existing rule
  ("cleared: mastered, gaining less than `min_gain`, or not shown to be short")
  is about a prerequisite that can still be worked on, which this one cannot. The
  three new tests cover never-the-focus (even when costly, with the others still
  chosen among), not holding a dependent back, and still counted and mastered; the
  equivalence fixture (127 cases, all `may_focus: true`) and the threshold tests
  pass unchanged. `STAMP` and `README` say it. Nothing outside `learn` reads the
  flag, so the brief's praise ("well") still covers these skills.
- **Stream 0 (6).** `advisor_rng` XORs `salt * constant` into the evidence seed;
  for salt 0 the stream is `Rng::stream(EVIDENCE_SEED, view_seed ^ purpose)`, the
  expression the code had before (f24f091^, `0x7001`, `0x7002`). So the evidence
  at stream 0 is identical by construction (see 12 on the test).
- **The M4 double launch** is recorded and harmless: the results are a function of
  the plan's seeds, and the session scratchpad holds two complete singles logs
  (`m4b.txt`, `m4_singles.txt`) that agree line for line apart from the elapsed
  seconds. See 9 on the wording.
- **M7** is honest about what it dropped: `hint_latency` no longer exists and the
  doc says so; the decision rules (p95 <= 0.5 s on the main thread; 60 deals > 2 s
  needs a Worker) are applied as written (77 ms; 3.5 s).

## Findings

1. **should. `docs/MEASUREMENTS.md:437-719`: M4's result is filed under M5.** The
   headings run `## M4` / `### Plan`, `## M5 Rule precision` / `### Plan`, `###
   Result` (this is M4's: singles, prerequisites, constants, command), `###
   Decision on M4's result`, `### Result` (M5's). A reader of M5 finds M4's tables
   inside it, and "Result" appears twice with no name. Fix: move M5's plan after
   the M4 result and decision (or move M4's result up under M4); name each result.
   The decision is signed "the orchestrating session"; `CLAUDE.md` asks for the
   decision, its date and its source, which "the maintainer's brief" or "M4" gives
   without the actor (nit).

2. **should. M1's `behind` row does not reproduce at HEAD.** I ran `measure m1`
   (26 s read, 40 s total) on this tree. Every row and the replication tables
   match the doc to the digit except `behind`: 205 positions (doc 177), 1 exact
   (doc 0), median 6.19 / p90 11.66 / p95 14.13 (doc 6.76 / 11.48 / 14.06), cal
   1.03 / 1.03 (doc 0.99 / 1.01), select 2.67 (doc 2.59); clear cases 410, same
   sign 404, share 0.985, again 0.568 (doc 371, 362, 0.976, 0.548). The cause is
   that M8 (19:59) changed `SPLIT_SHARE` after M1 ran (19:46) and `behind`'s
   trigger (`is_split`) depends on it. `MARGIN` is 10 either way, so no constant
   moves, but the doc's command no longer gives the doc's numbers. Fix: re-run M1
   at HEAD and print both (state "run with `SPLIT_SHARE` (0.3, 0.7)"), or add to
   every section the constants it ran under and which later ones touch it. The same
   question applies to any result computed before a later measurement changed a
   constant it reads: M2 and M8 do not read the tutor's constants, M3 ran after
   M1, M2 and M8, M4 to M7 after M3; so `behind` in M1 is the only case I found
   (M3 re-run at HEAD: equal, see Reproduction).

3. **should. `bin/measure.rs:893-1006`, `measure.rs` (`m6_grid`, `m6_choice`): the
   `m6` command cannot reproduce its "provisional" rows or its choice.** It takes
   `tutor::PARAMS` as the base: the "provisional constants" it reports, the grid's
   `..*base`, and the tie-break "nearest the provisional constants". After the
   M6 commit `PARAMS` is the chosen set, so a re-run prints the chosen set under
   the label "provisional" (the table's 0.537 rows do not come back) and the
   nearest-to-base tie-break now favours the answer it is meant to find. The gains
   and `costly_chances` kept from the base happen to be unchanged, which hides it.
   Fix: a named `M6_BASE: Params` in `measure.rs` (the pre-M6 values: discount 0.83,
   lower 0.2, upper 0.9, master_of_top 0.85, lose_below 0.2, gains 1.1/4.4/8) used
   by `m6`, with a test that the grid contains it; then the doc's command returns
   the doc's table. Nit in the same place: the doc says "500 seconds to score the
   540 sets", the log (`m6.txt`) prints 500 s as the cumulative time at the end of
   the grid (89 s tuning windows, 178 s fresh, 502 s total), so the grid took about
   320 s.

4. **should. `lessons.rs:148-152` with `rules.rs:57-61`, `tutor.rs:434-445`: the
   `worth.rich` rule passed M5 under one condition and is told under a wider
   one.** M5 measured `rich(k, a, 7)`: a follower, the led card worth 7 thirds or
   more, a card that wins and one that does not legal, rung 4 wins. The lesson
   applies it when `Context.direction == Some(Direction::Timid)`, which the
   classifier sets for any `worth` miss in which winning was clearly better, whatever
   the led card. A timid miss on a trick whose led card is a pip (the win matters
   for the lead or a later trick) will be told "A trick with many points in it is
   worth winning; take it", a claim M5 did not test and whose precision is
   unknown. Fix: carry the led card's thirds (the verdict has `trick_thirds`) into
   `Context` and make `timid` require `>= 7`, with a test; or measure the precision
   of rung 4 on "a timid miss" and gate that. The `announce` rule has no such gap:
   its trigger (a held honour's card, announcement pending) is the miss itself.

5. **should. `docs/TUTOR.md` is stale or inconsistent with the measurements.**
   Line 3-5: "the measurements (11) are not [built], and every rule is
   provisional until M5". Lines 227-230: "the focus is unvalidated until M6 ...
   (`PARAMS`) are Cassino's ratios, provisional". Lines 252-257: "until M5 sets a
   rule's gate to passed, no rule is told ... the rules in the code stay
   provisional; the `bid` and `pagat` rules are told only under the default
   settings until M5 has checked others". All of these were true before and are
   false after the merge (two rules are `Passed` and told, the learner has been
   validated and its acceptance not met). In the section 11 table: `min_gain`,
   `costly_gain` "Was 2.0, 8.0" while the code had 1.0 and 4.0 (and
   `MEASUREMENTS.md` says 1.0 to 1.1, 4.0 to 4.4); `discount` "was 0.8" while
   `MEASUREMENTS.md` says 0.83 to 0.88; "two rules of fourteen passed the gate"
   while `LESSONS` has twelve rules and M5 had 27 candidates (check the count).
   Fix: update those passages and make the "Was" column agree with the doc it
   points to.

6. **should. `docs/MEASUREMENTS.md` M4 result, `lessons.rs` (`praise_only`),
   `kit/learner/src/model.rs:20-25`, TUTOR.md 2: "worth nothing" is stronger than
   M4 showed.** For `void` and `behind` the 95% intervals are +1.36 to -0.39 and
   +0.43 to -1.41 Points a pair; the latter is up to 2.7 thirds a deal, more than
   `announce`, the least clear worth (2.1), which was taught. 160 pairs cannot tell
   "nothing" from "as much as announce". Making them praise-only is a defensible
   product choice (the learner should not send the player to something unproven),
   and it was not in M4's plan (a decision after the result, as the doc says),
   but the comments say "found worth nothing". Fix: wording "not shown to be worth
   anything (M4, 160 pairs; the interval reaches 2.7 thirds a deal)"; keep
   `may_focus` as the mechanism. Related, and for the maintainer to know: the 0.7
   sensitivity bar of M6 was not met partly because the rare skills cannot be
   certified in a window; the praise-only decision removes two of the twelve
   students from that test.

7. **should. `bin/measure.rs:631-716`, `docs/MEASUREMENTS.md` M4 tables: the
   numbers the constants rest on are derived by hand.** The worth in thirds
   (`-effect / 12 * 23`), the prerequisite "drop +- se" and the `min_gain` rule are
   not computed by `m4` or by a tested function, so `CLAUDE.md`'s "a computed
   number a document relies on has a test or a check that reproduces it" is not met
   for them (the singles' effects and z are, and the logs agree with the table).
   For the prerequisites the hand computation also has weaknesses worth recording:
   the two effects compared come from runs that stopped at different looks (each at
   its own boundary, so early stops overstate the effect), the se of the difference
   treats two runs that share seeds as independent, and the 2-se rule is applied to
   ten edges without adjustment (about 0.5 false "holds" expected). The two that
   "hold" are thin: `off` needs `worth` (drop 5.3 +- 1.5, a real 3.6 se) is
   plausible; `behind` needs `off` is a sign reversal (worth +0.49 alone, a gain
   of 3.3 with `off` out) of a skill with no shown worth, which is not a
   dependence. No decision rests on them (the edges are kept as the logical order
   either way), but `TUTOR.md` 2 says "only two were shown". Fix: a function and a
   test for the worth conversion, the `min_gain` rule and the drop (with its se from
   the two reports), printed by `m4`; and in the doc say the edges were "not
   contradicted" at this power rather than "shown".

8. **should. M6's choice is within the noise of the provisional constants; say
   so.** The grid has 540 sets scored on 6 windows of 9 detectable students, each
   student's detection a multiple of 1/6; the winner has 0.574 on the tuning
   windows (provisional 0.537), and on the fresh windows 0.556 against 0.537, a
   difference of one window of one student (`bid`). The fresh set is a clean
   holdout (no leakage) and the doc reports it, but nothing shows that the chosen
   `discount`, `upper` and `master_of_top` are better than 0.83, 0.9, 0.85. The
   `lose_below` values tie exactly (no effect in 18 deals), and the five most
   sensitive sets in the log differ only in `lose_below` and `lower`. The
   acceptance (0.7) is correctly reported "not met", and the finding that only
   `worth` and `off` are reliably named is the honest summary. Fix: add one line to
   the M6 result that the change from the provisional constants is not an
   improvement the data can show (so it stands for a window of about eight deals
   and a mastery bar of 0.80, not for a better learner); `TUTOR.md` 11 already
   states the limitation for the rare skills.

9. **nit. `docs/MEASUREMENTS.md` M4 result: "started twice by mistake and killed
   within minutes, before any look was read".** The harmlessness is right (the
   seeds fix the results; the two complete logs agree). The "before any look was
   read" cannot be checked, and the killed launch's log (`m4.txt`, 20:14) holds
   three finished knock-outs (Worth, Cheap, Off, each stopped clear at look 1 with
   the numbers of the final run). Fix: "killed after about three minutes, having
   finished three single knock-outs, which the full run repeated exactly".

10. **nit. M8: the rules were "completed after seeing" a trial.** The doc says so
    and runs on fresh seeds, which is the right handling. The `prefers_other` test
    (a quarter of rows clearly worse) leaves the 0.6 to 0.7 bin out by 0.016
    (0.234), so the band's top edge of 0.6 rests on that threshold; the data show a
    steadily falling preference with no contested middle, as the doc reads it.
    `behind` is praise-only now, so the constant only shapes what is judged and
    praised. No action.

11. **nit. M2: `y` includes credits that `x` excludes.** `x` is the count margin
    with the ultimo out; `y` is the credit difference, which includes the ultimo
    and announcement credits. These correlate with the margin, so the slope is
    slightly overstated and 1/slope understated; the interval (22.7 to 24.3) is
    narrow enough that 22 and 23 are both inside it. The doc notes the open ultimo
    weight. A sentence would do. Also, as the doc says, the slope is an average over
    a step; near the threshold a third is worth more Points, so the 23 prices a
    bid's Points against thirds by the typical, not the marginal, rate.

12. **nit. `tutor_tests.rs` (the stream test, f24f091): it cannot fail on the
    property it names.** `decision_verdicts` now calls `play_table(..., 0)` itself,
    so "stream 0 gives the verdicts the evidence gives" compares the function with
    itself. The identity is true by construction (XOR 0), but a regression (e.g.
    `salt + 1`) would pass. Fix: assert that `advisor_rng(view, 0, p)` yields the
    same first draws as `Rng::stream(EVIDENCE_SEED, view_seed(view) ^ p)`, or pin
    one summary text of a fixed record.

13. **nit. Code quality of `measure.rs`.** 1,424 lines, of which about 520 are
    tests; clippy clean, no dead code. It holds six measurements' worth of
    estimators in one file (statistics, M1 reads, M2, M8 rows, M3, M5, M6) and
    would read better as a `measure/` directory (`stats`, `noise`, `rules`,
    `diagnosis`), but nothing is wrong. Tests exist for each computed number except
    the ones in 7 and the M7 timings (not testable). `bin/measure.rs` (1,006
    lines) has `m4` and `m6` as long functions; its module doc lists only the
    `m1` usage, and the doc's intro says "`measure -- <m1|m2|...>`" although M7 is
    run through `evidencebench` (the doc's M7 command says so). `kit/learner/src/
    learn.rs:59-63` has comment lines over 100 columns after the edit (rustfmt does
    not wrap comments).

## Reproduction

Built in this worktree (`target/release/measure`), same machine, other work
running.

| Command | Doc | Here |
|---|---|---|
| `m1` | 22 s; table, replication | 40 s; all rows equal except `behind` (finding 2); `MARGIN` 10, `EXACT_MARGIN` 1, `BID_MARGIN` 0.45; bids 0.217, 0.237, 0.92 |
| `m8` | 92 s | 125 s; every bin and class equal to the digit; `SPLIT_SHARE` (0.1, 0.6), `SPLIT_MIN_POINTS` 6 |
| `m2` | 116 s | 239 s (two runs sharing 2 cores); slope 0.04259 +- 0.00071, 23.48 (22.73 to 24.27), reverse 15.38, deciles equal |
| `m3` | 92 s + 4 x 20 s | 266 s; both tables and the printed ladder lines equal to the digit, same three flagged cells |
