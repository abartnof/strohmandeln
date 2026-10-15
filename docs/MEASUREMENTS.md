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
seed 10,000,000 upward, both seats. Eligible: a card played with at least two
legal cards (announcements and forced plays are not), or a bid. Plays: every
7th eligible play, until 600. Bids: every 2nd eligible bid, until 200. The
count is fixed: 600 plays, 200 bids. (A trial of the program on 40 plays and
10 bids from seeds 1 and 2, 4 streams, checked that it runs; its numbers are
not used.)

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

### Result (2026-10-15)

596 plays (4 of the 600 finished no trick under some stream and are skipped) and
200 bids, from 37 sittings (seeds 10,000,000 to 10,000,036), 8 streams. 22
seconds. Descriptive, a fixed count.

Across-stream standard deviation of the gap, thirds (positions not exact);
`cal` is the ratio of the medians of the fixed pair's spread and the reported
error (about 1: the error is honest for a fixed pair), `cal rms` the same by
root mean square over positions with a finite error, `select` the mean excess
of a stream's own best pair over the same pair in the other streams:

| Skill | Positions | Exact | Median | 90th | 95th | cal | cal rms | select |
|---|---|---|---|---|---|---|---|---|
| worth | 59 | 8 | 5.15 | 9.24 | 10.36 | 0.97 | 0.90 | 1.19 |
| cheap | 173 | 13 | 3.09 | 7.13 | 8.43 | 0.97 | 1.08 | 2.16 |
| off | 290 | 0 | 5.43 | 10.61 | 12.48 | 1.03 | 1.02 | 2.69 |
| run | 35 | 2 | 2.01 | 5.17 | 6.45 | 1.19 | 0.98 | 0.53 |
| pagat | 30 | 5 | 4.60 | 8.07 | 9.07 | 1.02 | 0.99 | 0.41 |
| void | 131 | 13 | 3.77 | 7.91 | 10.20 | 1.05 | 1.10 | 2.14 |
| behind | 177 | 0 | 6.76 | 11.48 | 14.06 | 0.99 | 1.01 | 2.59 |
| endgame | 0 | 27 | | | | | | |

Exact positions (68 reads of position and skill, all with nothing face down and
the search to the last trick): the spread across streams is 0 in every one.
Bids: spread of the mean across streams, median 0.217 Point (90th 0.313, 95th
0.342) per unit of doubling; the reported error has median 0.237 (ratio 0.92).
The calibration RMS for a skill leaves out the positions in which a stream
finished one world of many and reports no error (infinite: 2 for `off`, 1 for
`void`, 2 for `behind`); the first run of this program included them and gave 0.
The medians, which decide, were the same in both.

Replication of the clear cases at 2 standard errors beyond 10 thirds (the
case's pair, re-read in the other 7 streams, has the same sign in the mean; and
the share of other streams in which it is clear again with the same sign):

| Skill | Clear cases | Same sign | Share | Clear again |
|---|---|---|---|---|
| worth | 213 | 209 | 0.981 | 0.887 |
| cheap | 427 | 425 | 0.995 | 0.786 |
| off | 532 | 523 | 0.983 | 0.657 |
| run | 165 | 165 | 1.000 | 0.947 |
| pagat | 112 | 112 | 1.000 | 0.824 |
| void | 218 | 211 | 0.968 | 0.740 |
| behind | 371 | 362 | 0.976 | 0.548 |

At 2.5 and 3 standard errors the sign shares are 0.978 to 1.000 and 0.988 to
1.000. The command below reproduces the numbers in about 25 seconds.

**Reading.** Every skill with at least 30 clear cases meets the sign criterion
(0.968 or more) and the calibration criterion (0.97 to 1.19, inside [0.8,
1.25]) at `k` = 2: the two-standard-error rule is honest, and the argmax of
each class, which the error leaves out, shifts the gap by 0.4 to 2.7 thirds,
well inside the margin. `behind` and `off` are clear again under another
stream in only 0.55 and 0.66 of the cases: the rule wrongly calls few cases
clear, but it also misses many real ones, which is power, not error (more
chances need more deals).

**Constants set.**

| Constant | Was | Now | Rule |
|---|---|---|---|
| `MARGIN` | 6 thirds | **10** | 2 x pooled median spread (5.0 -> 10) |
| `EXACT_MARGIN` | 1 | **1** | no exact position has a spread |
| `BID_MARGIN` | 0.5 Point | **0.45** | 2 x 0.217, to 0.05 |
| standard-error multiple | 2 | **2** (kept) | sign >= 0.95 and calibration in [0.8, 1.25] |

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m1`
(defaults: `--seed 10000000 --plays 600 --bids 200 --streams 8`).

## M2 Thirds per Point

### Plan (2026-10-15, before the first deal)

**Question.** How many thirds of the count margin is one Point of the payment?
`tutor::THIRDS_PER_POINT` prices a bid's expected gap (Points) and a forfeited
honour (one Point) in the unit of every other cost (thirds); it is the solver's
ultimo weight (`sim::ULTIMO_THIRDS` = 12), which has no basis in the payment.

**Data.** Rung 4 against rung 4 (default budget, default settings), sittings of 3
Touren, 300 sittings from seed 20,000,000 (up to 1,800 deals). Every deal that
was played (not thrown in). Fixed count.

**Observation.** For each deal, seat A's count margin `x` = thirds taken by A
less thirds taken by B (the ultimo excluded), and `y` = A's credited Points
less B's, divided by the deal's multiplier (the cost is not scaled by the
doubling, `docs/TUTOR.md` 4).

**Estimate.** Ordinary least squares of `y` on `x` with an intercept; the
slope `s` is the Points per third on average over the margins rung 4 meets;
cluster-robust standard error by sitting. Reported also: the mean `y` by
decile of `x` (the payment is a step at the threshold, so the slope is an
average), the correlation, and the reverse regression's thirds per Point.

**Decision rule (fixed now).** `THIRDS_PER_POINT` = `1 / s`, rounded to the
nearest whole third (delta-method 95% interval reported).

**Stopping.** A descriptive count, no looks.
