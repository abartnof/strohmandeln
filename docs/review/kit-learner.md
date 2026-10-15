# Review: kit/learner (crate kit-learner)

Scope: commits ee31577..db282d4 (`kit/learner/`, `kit/STAMP`, `kit/README.md`, `bin/gate`,
`docs/TUTOR.md` section 13). Compared with Cassino's `learner.rs` and `brief.rs` at
`73f0cd7` (read only). `cargo test -p kit-learner` (all pass), `cargo clippy -p kit-learner
--all-targets -- -D warnings` and `cargo fmt --check` clean.

Method: both learners read side by side; the Beta functions diffed against Cassino's
(`ln_gamma`, `beta_cdf`, `beta_fraction`, `beta_quantile` are byte-identical, `ln_gamma`
public); mutation of `learn.rs` against the whole suite (11 mutants, results in 8); a
scratch test (removed) for the edge cases in 1, 2, 6, 7.

Severity: bug, should, nit.

## Findings

| # | Sev | Where | Problem | Proposed fix |
|---|---|---|---|---|
| 1 | should | `model.rs:38` `Ladder::check` | A cycle in `needs` passes. With a needs b and b needs a, both short and costly, neither is ever open: no focus for ever, silently (`check` returned `Ok`, `learn` gave `focus = None` on five bad deals; no hang, `cleared` is not recursive). Longer cycles the same. | Reject cycles in `check` (DFS or repeated removal of skills whose needs are all removed; report the slugs on the cycle); test with a 2-cycle and a 3-cycle. Optionally also reject a duplicate in one `needs`. |
| 2 | should | `model.rs:167` `from_chances`, `learn.rs:78` | A NaN gap (or inf) poisons the skill for ever: `missed` becomes NaN, `missed[i]*discount + x` stays NaN, so `cost` is NaN, `gain` is 0 (`NaN.max(0)`), and that skill can never be the focus again. `to_text` then writes `NaN`, which `parse` rejects (it checks `is_finite`), so the stored summary becomes unreadable and `stale` does not list it (it is current by version). A negative gap is accepted by `from_chances` and refused by `parse`, the same asymmetry. | In `from_chances`, treat a non-finite or negative gap as 0 (or `debug_assert!` and clamp), document "gap: finite, at least 0"; a test that a NaN gap leaves the skill learnable and the text round-trips. |
| 3 | should | `model.rs:107` `Params::CASSINO` | Cassino's constants in a game-independent crate, and unused: the tests deliberately write `PARAMS` out (`tests/common/mod.rs:50`) so nothing pins `CASSINO` to Cassino's values; it can drift unnoticed. The doc comment of `Params` and `Ladder::name` ("cassino evidence v1", `model.rs:25`) also name Cassino. | Either delete it (Strohmandeln sets its own from `docs/TUTOR.md` section 11; Cassino's values live in `tests/common`), or keep it as `Params::DEFAULT`, say it is only a starting point and add a test `CASSINO == common::PARAMS`. Change the `name` example to a neutral one. |
| 4 | should | `tests/equivalence.rs`, `tests/learn.rs` | The thresholds where integers meet them exactly are not pinned. Mutants that survived the whole suite: `chances < min_chances` to `<=` in the fold (`learn.rs`), `seen` with `>` in the focus, `gain >= min_gain` to `>`, `gain >= costly_gain` to `>`, `chances >= costly_chances` to `>`, `lower` quantile `>=` to `>`, `short` `<` to `<=`, `lose` `<` to `<=`. The equivalence test cannot reach most of them (exact 4.0 / 8.0 chances, gain exactly 0.5 / 2.0 need hand-made inputs; the random cases are 0 to 15 chances per deal but mostly many deals). Killed: the discount order, the weight, the `!short` clause, the lose quantile, the tie rule (by `learn.rs` only). | Add hand-made boundary cases to `learn.rs` and to the generator (so Cassino fixes the answer): one deal with exactly `min_chances` and a clear miss; gain exactly `min_gain` (top_cost 0, missed 0.5 over one deal with 4 chances); exactly `costly_chances` chances at gain exactly `costly_gain` with a rate not surely short. Cassino's `>=` choices are then pinned by Cassino's own output. |
| 5 | should | `brief.rs:130` | Only the first skill that became mastered is announced; Cassino's brief names up to two in one bullet. A second skill mastered in the same deal is never announced (the next deal has it in `before`). The TUTOR note says "first skill of the ladder", but not that the other is dropped. | Say so in the module doc, or announce all through one bullet: `Param::Skill` repeated (`skill`, `skill2`) or a `Param::Skills(Vec<usize>)`; keep the cap of two as Cassino. Simplest: keep, and record it as a known departure in STAMP/TUTOR 13. |
| 6 | should | `model.rs:38` `Ladder::check`, `Params` | `Params` has no `check`. A NaN `discount` gives NaN chances for every skill (no mastery, no focus, no error), `lower > upper` or `discount` outside (0, 1] are accepted. `Summary` fields are public, so `met > chances` can be built directly (`b = 1 + c - m` below 1, even below 0, giving NaN quantiles); `parse` and `from_chances` are safe. | `Params::check() -> Result<(), String>` (finite, discount in (0, 1], quantiles in (0, 1), `lower <= upper`, the rest non-negative) and a doc line "pass `check`" in `learn`, as for the ladder. `is_current` could also refuse `met > chances`. |
| 7 | nit | `model.rs:130` | The `Tally` doc ("or, in the learner, discounted over several") is copied from Cassino and wrong here and there: the tally is always one deal's integers. `u32` is right for a per-deal tally; the discount lives in `learn` (`SkillState.chances` / `met` are `f64`, `learn.rs:75-78`), as in Cassino (its `Tally` is `u32` too, its learner `f64`). The brief's "discounted counts are fractional" worry does not apply. | Reword the doc: a deal's integer counts; `SkillState` carries the discounted ones. |
| 8 | nit | `brief.rs:144` | A focus whose closure returns nothing is "no focus" and the bullet then says `none_early`/`none_up`/`none_clean` ("nothing stood out") although `after.focus` is `Some`. Documented, and tested (`a_focus_with_nothing_to_say_is_no_focus`), but the message can contradict the learner. | Acceptable; the game should always supply a bullet for a focus. If kept, say in `brief`'s doc that the game's closure must not return empty. |
| 9 | nit | `brief.rs:68` `Keys::short` | Cassino distinguishes no decisions at all ("nothing to review yet") from a short deal; the shape has one `short` key. A game wanting both needs a second key. | Optional `Keys::none_decisions` or let the game pick the key by `decisions` (the cfg is `Copy`; keep as is until a game asks). |
| 10 | nit | `brief.rs:155-165` | The "seen" count (`chances >= min_chances`) repeats what `learn` computes as `seen`; the two can diverge if one changes. | Put it on `Learner` (e.g. `seen: usize` or `SkillState::seen`) and use it in both. Low value. |
| 11 | nit | `model.rs:13-23` | `&'static [SkillSpec]` and `&'static str` force a ladder to be a static. A game whose ladder varies with the rules (Cassino's NoSweep, `lessons::taught(skill, rules)`) builds several statics or leaks. | Fine for now; note it in the README, or take `&'a` with a lifetime on `Ladder`. |
| 12 | nit | `model.rs` docs | "the points a game", "cost in points" is the unit of Cassino's game score; `PLAN.md` keeps one meaning of Point (the payment) for Strohmandeln. In a generic crate it is "the game's unit of loss". | Reword to "units of the game's score". |
| 13 | nit | `tests/fixtures/cassino_learner_gen.rs:50` | The tie filter drops a case when any two skills with gain >= 0.5 tie, including two that could not be the focus; stricter than needed (harmless). Also the random cases are 100 of 100 non-tied, but the number of tries is only printed (`eprintln!`), not recorded. | Record the counts (hand-made 20, random 100 of N tried) in the header comment. |

## Answers to the questions asked

- **Correctness against Cassino.** The Beta functions are verbatim. In `learn` the order is
  as Cassino's: `before` is copied, the weight and the three counts are discounted and then
  the deal added, the `min_chances` skip comes after the counts are updated (so the counts
  keep accumulating while the state does not change), mastery uses `lower` and
  hysteresis uses `upper` against `lose`. The focus: `short`, `cleared`, `open`, `eligible`
  are the same as Cassino's; `gain = (cost - top_cost).max(0)`; the choice differs only
  in ties (earlier skill, deliberate, `total_cmp(..).is_gt()`; Cassino's `max_by` returns the
  last maximum). `MIN_CHANCES` is per skill in both `learn` and `short`/`seen`
  (`learn.rs` fold and the closures), and `brief`'s count. `just_mastered`, `games`
  (current summaries only) match. Verified by the diff, by the fixture and by mutation.
- **Is the fixture a real test.** For the arithmetic, yes: the discount order, the weight,
  the `!short` clause and the quantile used for losing each fail it (mutated and run). It
  would not catch a change at an exact threshold (finding 4). The dropped tie cases are
  covered by `exact_ties_in_gain_go_to_the_earlier_skill` (a mutant to the later skill fails
  it). It exercises both outcomes: 31 of 121 cases have a skill just mastered, and the
  test asserts that between 50 and 120 have a focus and more than 20 have mastery.
- **Generality.** Nothing Cassino- or Strohmandeln-specific in the code besides finding 3
  (`Params::CASSINO`, one doc example). `Tally { u32, u32, f64 }` is right (finding 7).
  The brief carries keys, a closure for the focus bullets (called once, only with a focus, tested),
  the `Skill` parameter for the table to name; the API can serve both games. Findings 5, 9, 11
  are the places a second game may push.
- **Edge cases.** Empty ladder: `check` refuses it; `learn` on one gives no focus and counts
  the empty summaries (no panic). Zero games: no focus, `games = 0`, cost 0 (tested). A skill
  with unmet needs for ever: it waits behind its prerequisite while that is short and
  costly (by design); a prerequisite with no chances or low cost is cleared; only a cycle
  deadlocks (finding 1). NaN: findings 2 and 6. Stale summaries mixed in: skipped by `learn`,
  listed by `stale`, `games` counts only current ones (tested); a stale one in the middle
  does not break the discount (it is filtered before the fold).
- **The 175 KB fixture.** 174,808 bytes, 8,141 lines; gzip 32 KB, which is what git stores.
  Acceptable. It is mostly zero lines (`sums 0 0 0`, `state ... 0.0 0.0 0.0 0.0`); a format that
  lists only the non-zero tallies and states would be about a third of the size, at the price
  of a less direct text form. The check that the text form is Cassino's byte for byte (all
  eight lines a deal) is a reason to keep it as is. Not worth a commit.
- **STAMP and README.** Accurate: Cassino's commit `73f0cd7`, both sources last changed at
  `69f409d` (confirmed in `~/cassino`); every file of the crate is listed, and `bin/gate`
  now fails for an unlisted file under `learner/`. One note: STAMP's `files:` block holds the
  Rust files with the `-`-column text; the gate reads only the first column, fine. The README
  paragraph is correct (no origin header on Rust files, said).
- **Simplicity.** Small (about 700 lines of source), no dependencies, no generics, the same
  shape as Cassino so it can be diffed. `Learner::state` / `mastered` and `Ladder::index_of`
  are thin but used.

## Verdict

Accept with fixes. The port is faithful: the math is verbatim, the fold and the focus match
Cassino's except the deliberate tie rule, and the fixture would fail on any change to the
discounting, the weights or the quantile logic. Fix 1 to 4 before the first game relies on
it (a cycle and a NaN gap are silent failures; the exact-threshold cases are cheap to add
and are where a future edit would slip); 5 and 6 are decisions to record or take; the nits
can wait.
