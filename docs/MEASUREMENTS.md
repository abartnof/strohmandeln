# Measurements for the tutor

Runs M1 to M8 of `docs/TUTOR.md` 11, in the style of Cassino's
`measurements/README.md`. Each section has its **plan**, committed before the
first batch of the run it describes, then its **result** and the **command**
that reproduces it. Binary: `cargo run --release -p strohmandeln-ai --bin
measure -- <m1|m2|...>`; its computations are unit-tested in
`crates/strohmandeln-ai/src/measure.rs`.

Standing rules (`CLAUDE.md`): comparisons use mirrored pairs (each seed's
sitting played twice, seats swapped), sequential batches, O'Brien-Fleming
boundaries from `stats.rs` fixed with the batch size and the number of looks
before the first batch, stopping as soon as the boundary is crossed or the
effect is plainly negligible, and reporting the interval at the last look. A
descriptive measurement fixes its count before it runs. Seeds of different
runs are disjoint; each section names its block.

Units: a **pair** is one seed's sitting of 3 Touren (6 deals) played twice,
seats swapped; its observation is A's Points less B's summed over the two
sittings (12 deals played by each side). **Thirds** are thirds of the count
margin (`sim.rs`); **Points** are the payment.

Seed blocks (first seed of each): M1 10,000,000; M2 20,000,000; M3 30,000,000;
M4 40,000,000; M5 50,000,000 (confirmation 55,000,000, settings 56,000,000);
M6 60,000,000 (fresh 65,000,000); M7 70,000,000; M8 80,000,000. The earlier
runs of `docs/TUTOR.md` 11 (`print_students`, `print_lesions`) used sittings
0 to 8 of `tutor_tests.rs`.

## M1 Advisor noise

### Plan (2026-10-15, before the first draw)

**Question.** How much does the advisor's gap between two classes of cards move
when it samples other worlds, and is the noise rule of `docs/TUTOR.md` 4 (the
gap must exceed `MARGIN` and twice its standard error over the sampled worlds)
honest? The reviewer's note (`docs/review/tutor-evidence.md`, second pass,
finding 6) is that the two cards compared are the argmax of their classes under
the same worlds, which the standard error does not take into account.

**Positions.** Rung 4 against rung 4 (default budget), sittings of 3 Touren from
seed 10,000,000 upward, both seats. Every decision with at least two legal
actions is eligible. Plays: every 7th eligible play, until 600. Bids: every 2nd
eligible bid, until 200. The count is fixed: 600 plays, 200 bids.

**Draws.** Each position is judged under 8 independent advisor streams (salt 0
to 7; salt 0 is the evidence's own stream, `tutor::EVIDENCE_SEED`), with the
evidence's budget (`Budget::default()`, a known position searched in one world
with all the nodes).

**Gap.** For each skill that applies, the classifier names the best card of the
class the rule favours (`a`) and the best of the other class (`b`); the gap is
`value(a) - value(b)` in thirds under that stream, with the standard error of
the paired per-world differences. The orientation is the classifier's:
`worth` beats / does not, `cheap`, `run`, `endgame` best / worst of the kind,
`off` cover / hand, `pagat` the Pagat / another card, `void` and `behind`
another lead / the lead into the void or the split suit. `bid` has no cards:
its gap is the mean of `rung4::bid_samples` (Points per unit of doubling) with
its standard error.

**Reported, by skill** (positions where the skill applies under all 8 streams):

1. noise: the standard deviation of the gap across the 8 streams, per
   position; its median, 90th and 95th percentile.
2. calibration: for the card pair that is best by the mean over the 8 streams,
   the standard deviation across streams of its gap against the mean reported
   standard error; the ratio of the two medians. About 1 means the error is
   honest for a fixed pair.
3. selection: for each stream, the gap of its own argmax pair less the mean gap
   of that same pair under the other 7 streams (the winner's curse, in
   thirds).
4. replication: among the (position, stream) cases that would be called clear
   (gap beyond the margin and beyond `k` standard errors), the share whose
   same pair has the same sign in the mean of the other 7 streams, and the
   share that is clear again, with the same sign, under a second stream. For
   `k` = 2, 2.5 and 3.

Exact positions (`tutor::search_complete`) are reported apart: the draws are
the same world, so the expected spread is zero.

**Decision rules (fixed now).**

- `MARGIN` = twice the pooled median of the across-stream standard deviations,
  over the play skills, positions not exact, rounded to the nearest third (at
  least 2).
- `EXACT_MARGIN` stays 1 third if every exact position has zero spread;
  otherwise twice the 95th percentile of the exact spread, rounded up.
- `BID_MARGIN` = twice the median across-stream standard deviation of the bid
  mean, rounded to 0.05 Point.
- The two-standard-error rule is kept at `k = 2` if, at `k = 2`, the sign
  replication is at least 0.95 for every skill with at least 30 clear cases
  and the calibration ratio of each such skill lies in [0.8, 1.25]. Otherwise
  `k` becomes the least of 2.5 and 3 that meets the sign criterion (a new
  constant `SE_MULTIPLE`), and if 3 does not, it is reported as not met.
- `EVIDENCE_VERSION` is bumped once, at the end of the runs, if any constant of the evidence changed.

**Stopping.** A descriptive count, no looks.
