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

### Result (2026-10-15)

1,800 played deals (none thrown in), 300 sittings (seeds 20,000,000 to
20,000,299), 116 seconds. Descriptive, a fixed count.

- Slope of net Points on the count margin: **0.04259 Point a third**, cluster
  standard error 0.00071 (by sitting), intercept -0.044, correlation 0.809.
- Thirds per Point = 1 / slope = **23.48** (95% interval 22.73 to 24.27).
- Reverse regression (thirds on Points): 15.38 thirds a Point (reported only).

The payment is nearly a step at the threshold, so the slope is an average over
the margins rung 4 meets. Mean net Points by decile of the margin:

| Margin, thirds | Mean margin | Net Points |
|---|---|---|
| -180 to -68 | -91.8 | -2.88 |
| -68 to -44 | -55.0 | -2.87 |
| -44 to -28 | -35.6 | -2.71 |
| -28 to -12 | -20.3 | -2.58 |
| -12 to 2 | -5.7 | -1.78 |
| 2 to 18 | 9.5 | 2.33 |
| 18 to 32 | 24.8 | 2.47 |
| 32 to 48 | 39.7 | 2.67 |
| 48 to 70 | 58.3 | 2.72 |
| 70 to 170 | 92.1 | 2.88 |

**Constant set.** `THIRDS_PER_POINT` 12 (the solver's ultimo weight) -> **23**
(rule: the reciprocal of the slope, rounded). A forfeited honour now costs 23
thirds and a bid's gap is priced at 23 thirds a Point. Open: the solver's
`ULTIMO_THIRDS` (12, the opponent's weight of the Pagat ultimo in the final
margin) is about half of what the payment implies; it is the opponent's and was
not changed.

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m2`
(defaults: `--seed 20000000 --sittings 300`).

## M8 The split suit

### Plan (2026-10-15, before the run)

**Question.** At what share of a suit's points does rung 4 start to prefer
another lead to opening that suit, and from how many points unplayed? This
sets `SPLIT_SHARE` and `SPLIT_MIN_POINTS`, the trigger of `behind` (`docs/TUTOR.md`
2). The sources say "about evenly" without a number (`research/16`, `behind`).

**Data.** Rung 4 against rung 4 (default budget), 100 sittings of 3 Touren from
seed 80,000,000, every lead of either seat with at least two legal cards, the
advisor under its own stream (salt 0). Fixed count. A program trial on 6
sittings from seed 85,000,000 (not used) checked that it runs and showed the
shape: the gap rises with my share, crossing 0 near 0.6; the rules below were
completed after seeing it, so the run is on fresh seeds.

**Rows.** For each lead decision and each suit with a legal lead in it and a
legal lead outside it: my share `s` = my value points of the suit unplayed
over all of them (mine, his hand and packets, and my own face-down packet
cards, `tutor::suit_share`), the total `t`, and the gap = the advisor's best
lead in the suit less its best lead outside it (thirds). Whether the lead made
was in the suit.

**Bins.** Ten bins of `s`, width 0.1, for suits with `t` at least 6 (the
provisional minimum). Per bin: rows, mean gap with the standard error clustered
by sitting, the share of rows with the gap below minus `MARGIN`, and the share
opened. A bin **prefers another lead** if the upper end of its 95% interval
for the mean gap is below 0 and at least a quarter of its rows have a gap below
minus `MARGIN`.

**Decision rules (fixed now).**

- `SPLIT_SHARE` = (low, high): the contiguous run of bins that prefer another
  lead around the bin of the lowest mean gap; its high edge is the share at
  which rung 4 starts to prefer another lead; its low edge leaves out the
  lowest bins in which rung 4 opens the suit in less than 10% of the rows (a
  suit it has no mind to open is his, not contested).
- `SPLIT_MIN_POINTS` = the start of the lowest class of `t` ([0,3), [3,6),
  [6,9), [9,12), [12, ...)) from which that class and every higher class with
  at least 30 rows, among rows with `s` in the band, prefer another lead (the
  same test).
- If the lowest bin does not prefer another lead, `behind` has no trigger and
  is told as its fact (`docs/TUTOR.md` 9).

**Stopping.** A descriptive count, no looks.

### Result (2026-10-15)

36,665 rows (suits of lead decisions) from 100 sittings (seeds 80,000,000 to
80,000,099), 92 seconds. Descriptive, a fixed count. Suits with at least 6
points unplayed, by my share of the suit; the gap is the advisor's best lead in
the suit less its best lead outside it, thirds, with the standard error
clustered by sitting:

| My share | Rows | Mean gap | Another lead clearly better | Opened by rung 4 |
|---|---|---|---|---|
| 0.0 to 0.1 | 976 | -31.85 +- 1.16 | 0.887 | 0.015 |
| 0.1 to 0.2 | 3,584 | -17.91 +- 0.41 | 0.619 | 0.111 |
| 0.2 to 0.3 | 4,124 | -14.56 +- 0.40 | 0.517 | 0.187 |
| 0.3 to 0.4 | 4,931 | -12.28 +- 0.29 | 0.485 | 0.212 |
| 0.4 to 0.5 | 3,239 | -7.22 +- 0.37 | 0.344 | 0.307 |
| 0.5 to 0.6 | 4,324 | -5.05 +- 0.30 | 0.295 | 0.351 |
| 0.6 to 0.7 | 2,203 | -1.10 +- 0.40 | 0.234 | 0.454 |
| 0.7 to 0.8 | 1,138 | +3.01 +- 0.59 | 0.164 | 0.592 |
| 0.8 to 0.9 | 922 | +3.52 +- 0.67 | 0.140 | 0.603 |
| 0.9 to 1.0 | 998 | -1.79 +- 0.49 | 0.191 | 0.407 |

The preference for another lead falls steadily as my share rises, with no dip
in the middle: leading a suit gets better the more of it I hold. The bins from
0.1 to 0.6 prefer another lead by the rule; 0.6 to 0.7 does not (its mean is
-1.10 +- 0.40 but only 23% of its rows are clearly worse); 0.0 to 0.1 does, but
rung 4 opens the suit in 1.5% of those rows (his suit). Rows with the share in
the band (0.1 to 0.6), by the suit's points unplayed:

| Points unplayed | Rows | Mean gap | Another lead clearly better |
|---|---|---|---|
| 0 to 3 | 1,192 | -3.80 +- 0.27 | 0.147 |
| 3 to 6 | 2,288 | -3.95 +- 0.22 | 0.189 |
| 6 to 9 | 3,185 | -5.79 +- 0.28 | 0.264 |
| 9 to 12 | 3,242 | -7.59 +- 0.29 | 0.353 |
| 12 and more | 13,942 | -13.44 +- 0.21 | 0.515 |

**Constants set.** `SPLIT_SHARE` (0.3, 0.7) -> **(0.1, 0.6)**;
`SPLIT_MIN_POINTS` 6 -> **6** (the lowest class that prefers another lead, with
every class above it).

**Reading.** The data do not show a contested middle: the cost of opening a
suit is highest where I hold little and falls steadily with my share. "Split"
in the sources' sense ("about evenly") is a weaker claim than what rung 4 does;
the trigger is "a suit in which I hold at most 60% of the points unplayed and
the opponent could hold more", and the example names the better lead, not a
rule (`docs/TUTOR.md` 9, `behind` is a fact). The share counts my own
face-down packet cards as the opponent's, which biases it low by about my
hidden fraction of the suit.

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m8`
(defaults: `--seed 80000000 --sittings 100`).

## M3 What the strongest rung meets

### Plan (2026-10-15, before the run)

**Question.** Judged by the tutor's own evidence, what share of its clear chances
does rung 4 meet, how many chances a deal does each skill offer, and what does a
deal lose to each skill? These are `top_rate` and `top_cost` of the ladder, and
the chances a deal that set `MIN_CHANCES`. The run uses the constants as they
stand after M1, M2 and M8 (`MARGIN` 10, `THIRDS_PER_POINT` 23, `SPLIT_SHARE`
(0.1, 0.6)).

**Data.** Rung 4 (the player, default budget) against rung 3, sittings of 3
Touren; the player sits in seat A in the even sittings and in seat B in the odd
ones (both seats); every deal's evidence is worked out for the player with no
assisted decisions (`tutor::evidence`). Rule sets: the default (values 2/3/4,
both pass: play, ultimo 1 Point), 200 sittings (1,200 deals) from seed
30,000,000; and each other setting alone (values 1/2/3, values 1/2/4, both pass:
throw in, ultimo 2 Points), 40 sittings (240 deals) each, from seeds
31,000,000, 32,000,000, 33,000,000, 34,000,000. Fixed counts.

**Reported, by skill** (default rule set): chances a deal; met and the met
rate, with a standard error clustered by sitting (the ratio estimator); the
missed thirds a deal. For the other rule sets, the chances and the met rate.

**Decision rules (fixed now).**

- `top_rate(skill)` = met / chances, default rule set, to two decimals.
- `top_cost(skill)` = missed thirds a deal, default rule set, to two decimals.
  (It is conditional on the resolution: a miss costs at least `MARGIN`.)
- `min_chances(skill)` = the whole number of chances a skill offers in six deals
  (a sitting), halved and rounded down, at least 2 and at most 4.
- `DISCOUNT` = 1 - 1/6 = 0.83: a window of about a sitting of six deals
  (`docs/TUTOR.md` 5). M6 may change it, tuned against the lesioned students.
- A rule set whose met rate for a skill differs from the default's by more than
  twice the larger of the two standard errors is reported as differing (the
  rules for that skill are told only under the settings they were confirmed
  under, `docs/TUTOR.md` 2).

**Stopping.** A descriptive count, no looks.

### Result (2026-10-15)

Rung 4 against rung 3, 200 sittings (1,200 deals) at the default settings (seeds
30,000,000 to 30,000,199), 92 seconds; four other rule sets of 40
sittings (240 deals) each. Descriptive, a fixed count. The evidence is the
player's (rung 4), with the constants after M1, M2 and M8.

| Skill | Chances | Met | Met rate (se) | Chances a deal | Lost a deal, thirds |
|---|---|---|---|---|---|
| worth | 2,026 | 2,006 | 0.990 (0.002) | 1.69 | 0.322 |
| cheap | 2,200 | 2,107 | 0.958 (0.004) | 1.83 | 1.346 |
| off | 587 | 542 | 0.923 (0.011) | 0.49 | 0.716 |
| run | 1,324 | 1,252 | 0.946 (0.007) | 1.10 | 0.927 |
| pagat | 1,017 | 996 | 0.979 (0.005) | 0.85 | 0.223 |
| void | 1,190 | 1,148 | 0.965 (0.006) | 0.99 | 0.616 |
| behind | 2,029 | 1,948 | 0.960 (0.004) | 1.69 | 1.416 |
| endgame | 1,357 | 1,264 | 0.931 (0.008) | 1.13 | 0.855 |
| bid | 665 | 657 | 0.988 (0.005) | 0.55 | 0.093 |
| announce | 94 | 94 | 1.000 (0.000) | 0.08 | 0.000 |

The met rate (chances in brackets) under each rule set; `*` marks a difference
of more than twice the larger standard error from the default:

| Skill | Default | Values 1/2/3 | Values 1/2/4 | Throw in | Ultimo 2 |
|---|---|---|---|---|---|
| worth | 0.990 (2026) | 0.974 (392) | 0.989 (373) | 0.989 (187) | 0.989 (376) |
| cheap | 0.958 (2200) | 0.941 (457) | 0.959 (369) | 0.963 (216) | 0.957 (414) |
| off | 0.923 (587) | 0.915 (106) | 0.881 (109) | 0.908 (76) | 0.966 (117) `*` |
| run | 0.946 (1324) | 0.945 (274) | 0.973 (298) `*` | 0.953 (191) | 0.962 (212) |
| pagat | 0.979 (1017) | 0.963 (191) | 0.991 (217) | 0.969 (96) | 0.989 (188) |
| void | 0.965 (1190) | 0.965 (256) | 0.946 (260) | 0.955 (132) | 0.954 (219) |
| behind | 0.960 (2029) | 0.974 (431) | 0.970 (434) | 0.957 (231) | 0.975 (360) |
| endgame | 0.931 (1357) | 0.921 (254) | 0.947 (262) | 0.891 (110) | 0.943 (227) |
| bid | 0.988 (665) | 0.985 (137) | 0.981 (156) | 0.976 (126) | 1.000 (135) `*` |
| announce | 1.000 (94) | 1.000 (25) | 1.000 (19) | 1.000 (11) | 1.000 (22) |

Three of 36 comparisons are flagged (`off` and `bid` under ultimo 2, `run` under
values 1/2/4), about what chance alone gives at this threshold; none is large
(0.04 at most). The settings change little of what rung 4 meets; the rules that
depend on them (`bid`, `pagat`) are confirmed under each in M5.

**Constants set** (`tutor::LADDER`, `tutor::PARAMS`):

| Skill | `top_rate` was -> now | `top_cost` was -> now | `min_chances` was -> now |
|---|---|---|---|
| worth | 0.96 -> **0.99** | 0.96 -> **0.32** | 4 -> **4** |
| cheap | 0.96 -> **0.96** | 1.22 -> **1.35** | 4 -> **4** |
| off | 0.93 -> **0.92** | 0.95 -> **0.72** | 4 -> **2** |
| run | 0.91 -> **0.95** | 1.92 -> **0.93** | 4 -> **3** |
| pagat | 0.94 -> **0.98** | 0.57 -> **0.22** | 4 -> **2** |
| void | 0.96 -> **0.96** | 0.32 -> **0.62** | 4 -> **2** |
| behind | 0.91 -> **0.96** | 1.76 -> **1.42** | 4 -> **4** |
| endgame | 0.93 -> **0.93** | 1.00 -> **0.86** | 4 -> **3** |
| bid | 1.00 -> **0.99** | 0.00 -> **0.09** | 4 -> **2** |
| announce | 1.00 -> **1.00** | 0.00 -> **0.00** | 2 -> **2** |

`PARAMS.discount` = **0.83** (new; the other constants of `PARAMS` are
provisional until M4 and M6). The earlier values were fitted to 54 deals against a
rung-4 opponent with the margin of 6.

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m3`
(defaults: `--sittings 200 --other-sittings 40`; the ladder lines are printed).

## M4 What a skill is worth

### Plan (2026-10-15, before the first batch)

**Question.** What does rung 4 lose when one skill is knocked out, in Points, and
does a skill's worth depend on the skill the ladder puts before it? This sets
`PARAMS.min_gain` and `PARAMS.costly_gain` (the cost of a skill the learner may
call worth working on) and checks the prerequisites of `docs/TUTOR.md` 2.

**Comparison.** A student (`students.rs`, rung 4 with the knock-out) against rung
4 whole, mirrored pairs from seed 40,000,000 (the same seeds for every
knock-out), settings default, sittings of 3 Touren. The observation is the
student's Points less rung 4's over the pair (12 deals each). The effect of a
knock-out is the mean; its **worth** is minus the effect.

**Knock-outs (eleven).** `Worth`, `Cheap`, `Off`, `Run`, `Pagat` (a coin, halved),
`Void`, `Behind`, `Endgame`, `Bid` (a coin, halved), `AlwaysPass`, `Announce`
(`students.rs` and `docs/TUTOR.md` 11). `AlwaysTake` is not run (the plan of
`docs/TUTOR.md` 11 names the pass and the coin).

**Sequential plan, fixed now.** Batches of 40 pairs, at most 4 looks (160 pairs),
two-sided O'Brien-Fleming boundaries 4.05, 2.86, 2.34, 2.02 (`stats.rs`), a
futility stop from the second look if the whole 95% interval lies within +-0.5
Point a pair (about 0.5 thirds a deal). Stops as soon as a boundary is crossed
or the effect is negligible; the interval at the last look is reported.

**Prerequisites.** For each edge "A needs B" of `Skill::needs` (ten edges): the
worth of A with B already knocked out, measured as a student with B and A out
against a student with B out (`Bid` for the `bid` edge is the knock-out with the
larger single effect). Batches of 40, at most 3 looks, the same boundaries rule
(`obf_constant(3)`), the same futility band. An edge **holds** if the worth of A
with B out is smaller than its worth alone by more than twice the standard error
of the difference (A matters less once B is gone: its value rests on B). Otherwise
the edge is recorded as **not shown**, and kept as the logical order of the
sources (`research/16`): the skills may stand in for one another, as Cassino
found for its own.

**Decision rules (fixed now).**

- The worth of a skill in thirds a deal = (-effect) / 12 x `THIRDS_PER_POINT`.
- `min_gain` = half the smallest worth in thirds a deal among the knock-outs that
  are **clear** with a negative effect (the skill is worth something), rounded
  to 0.1.
- `costly_gain` = four times `min_gain` (Cassino's ratio, 2.0 to 0.5), and
  `costly_chances` = 8 (twice the largest `min_chances` of M3, as Cassino's 8 is
  twice its 4).
- A knock-out that is clear with a **positive** effect (the student wins) says
  the skill, as the advisor defines it, is not worth Points; it is reported, and
  does not set `min_gain`.

**Stopping.** As above; the maximum is 160 pairs for a single knock-out and 120
for an edge.

## M5 Rule precision

### Plan (2026-10-15, before the selection run)

**Question.** Which of the rules of `docs/TUTOR.md` 9 may the tutor tell as a
rule? A rule is a **trigger** (a position) and a **prescription** (what to do
there), each a test of the mover's own view and the move made
(`crates/strohmandeln-ai/src/rules.rs`, `CANDIDATES`). Its **precision** is how
often rung 4 does what the rule prescribes when the trigger fires. The gate
(`docs/TUTOR.md` 11) is a clustered Wilson lower bound of at least 0.80,
confirmed on fresh seeds, and each shipped rule confirmed under each other
setting; a rule that fails is told as its fact.

**Candidates (27, written before the first run; the ids are the ones in
`rules.rs`).** Each rule of the table has one to five candidates, which differ
in the number in the trigger or in how much the prescription asks:
`worth.rich.1-3` (led card worth 7, 10, 13 thirds or more: win),
`worth.poor.1-2` (led card worth 1 or at most 4 thirds, a king wins: no king),
`cheap.1-2` (the cheapest card of the outcome played; the cheapest winner),
`off.pip.1-2` (a cover and a hand card legal: a cover, following or leading),
`off.figure.1-3` (a Dame or Cavall cover legal: a cover; that figure (the higher)
played; the same unless he shows two low cards of its suit),
`run.1-2` (a tarock led from two or more: the top of the longest solid run,
else the lowest; a card of any solid run, else the lowest) and the diagnostics
`run.part.3-4` (the two halves, never shipped), `pagat.cash.1-2` (following
with the Pagat and another card, 7 or 8 tarocks or fewer: the Pagat),
`pagat.keep.1-2` (12 or 11 tarocks or more: keep it), `void.1` (a lead into a
shown void and another lead: not into it), `behind.1-3` (a lead in a split suit
and another: not in a split suit; a tarock; a tarock or a suit mostly mine),
`bid.1-2` (60 to 75 thirds of sure winners and 6 or 7 tarocks; 60 thirds and 6
tarocks or more: take up), `announce.1` (about to play a card of a held honour:
announce). A "solid run" is a block of my tarocks that are neighbours among the
tarocks not yet played. `endgame` is not tested: its rule ("count what you still
need") prescribes no card, so it has no precision; it is told as written, as a
prompt, and the verdict is "not measurable".

**Data.** Rung 4 (the player, default budget) against rung 3, both seats
(alternating by sitting), 3 Touren, every deal's decisions of the player.
- Selection: 400 sittings (2,400 deals), default settings, seeds from 50,000,000.
- Confirmation: the candidate of each rule with the best lower bound at or above
  0.80 in the selection, 400 sittings from 55,000,000 (fresh seeds).
- Settings: each confirmed rule under each other setting alone (values 1/2/3,
  values 1/2/4, both pass: throw in, ultimo 2), 200 sittings each, seeds from
  56,000,000, 57,000,000, 58,000,000, 59,000,000.
A trial of the program on 6 sittings from seed 95,000,000 checked that it runs;
its numbers are not used.

**Estimate.** Per rule: firings, firings satisfied, precision = satisfied /
firings, with the ratio estimator clustered by deal giving an effective sample
`n_eff` = p (1 - p) / variance (at most the firings), and the lower end of the
Wilson interval (z = 1.96) at `n_eff`.

**Decision rules (fixed now).**
- A candidate **passes** if its lower bound is at least 0.80. Per rule (the id
  up to its last dot) the candidate with the best lower bound among those that
  pass is **selected** (`run.part.*` are not eligible).
- A selected candidate is **confirmed** if its lower bound on the fresh seeds is
  at least 0.80. Only the selected candidate of each rule is confirmed.
- A confirmed rule is told **under a setting** if its lower bound there is at
  least 0.80; otherwise under that setting its fact is told (`docs/TUTOR.md` 2,
  `lessons::taught`).
- A rule with no selected candidate, or not confirmed, is told as its fact.

**Stopping.** Descriptive fixed counts, no looks.

### Result (2026-10-15)

Singles: the program ran as planned (batches of 40, at most 4 looks). The pairs
ran as a second process of the same program (`--only pairs --bid coin`), to keep
each run under half an hour; the `Bid` coin knock-out has the larger single
effect (the `bid` edge uses it). A first launch of the program was started
twice by mistake and killed within minutes, before any look was read; the
results are a deterministic function of the plan (the same seeds), so nothing
was seen twice. Singles about 25 minutes, pairs 1,073 seconds (the machine was
shared with builds).

**What a knock-out costs rung 4** (Points per mirrored pair of 12 deals, the
student's less rung 4's; `worth` is minus this, in thirds a deal x 23 / 12):

| Knock-out | Verdict (look, pairs) | Effect +- se | 95% interval | Worth, thirds a deal |
|---|---|---|---|---|
| Worth | clear, look 1 (40) | -6.300 +- 1.153 | -8.56 to -4.04 | 12.1 |
| Cheap | clear, look 1 (40) | -13.675 +- 0.998 | -15.63 to -11.72 | 26.2 |
| Off | clear, look 1 (40) | -14.025 +- 1.103 | -16.19 to -11.86 | 26.9 |
| Run | clear, look 3 (120) | -1.300 +- 0.556 | -2.39 to -0.21 | 2.5 |
| Pagat (coin) | clear, look 2 (80) | -2.438 +- 0.842 | -4.09 to -0.79 | 4.7 |
| Void | inconclusive, look 4 (160) | +0.487 +- 0.446 | -0.39 to +1.36 | none shown |
| Behind | inconclusive, look 4 (160) | -0.494 +- 0.469 | -1.41 to +0.43 | none shown |
| Endgame | clear, look 4 (160) | -1.575 +- 0.489 | -2.53 to -0.62 | 3.0 |
| Bid (coin) | clear, look 2 (80) | -2.650 +- 0.814 | -4.25 to -1.06 | 5.1 |
| AlwaysPass | inconclusive, look 4 (160) | -0.062 +- 0.479 | -1.00 to +0.88 | none shown |
| Announce | clear, look 4 (160) | -1.106 +- 0.511 | -2.11 to -0.11 | 2.1 |

Two skills the tutor teaches, `void` and `behind`, have no measurable worth in
Points against the strongest rung (a student made to lead into a shown void or
into the split suit loses nothing it can show in 160 pairs: the interval for
`void` includes a gain of up to 1.4 a pair). Always passing costs nothing
either: rung 4 rarely finds taking up clearly right.

**The prerequisites** (the skill's worth alone, and with its prerequisite
already knocked out; the drop is alone less with, in Points a pair; the edge
holds if the drop exceeds twice the standard error of the difference):

| Skill | Needs | Alone | With the prerequisite out | Drop +- se | Verdict |
|---|---|---|---|---|---|
| Cheap | Worth | 13.68 | 14.30 (look 1) | -0.62 +- 1.42 | not shown |
| Off | Worth | 14.03 | 8.70 (look 1) | +5.33 +- 1.48 | **holds** |
| Run | Cheap | 1.30 | 3.01 (look 2) | -1.71 +- 0.98 | not shown |
| Pagat | Run | 2.44 | 1.99 (look 2) | +0.45 +- 1.08 | not shown |
| Void | Worth | -0.49 | -0.06 (look 3, inconclusive) | -0.43 +- 0.69 | not shown |
| Behind | Off | 0.49 | -3.33 (look 1, a gain of 3.3) | +3.82 +- 0.91 | **holds** |
| Behind | Void | 0.49 | 2.10 (look 3) | -1.61 +- 0.82 | not shown |
| Endgame | Cheap | 1.57 | 1.38 (look 3) | +0.20 +- 0.80 | not shown |
| Endgame | Void | 1.57 | 1.70 (look 3) | -0.12 +- 0.81 | not shown |
| Bid | Worth | 2.65 | 4.62 (look 1) | -1.98 +- 1.43 | not shown |

Two edges hold (`off` needs `worth`, `behind` needs `off`); in no pair did the
second skill lose all its worth, and in several it was worth more with the
first gone (the skills stand in for one another, as in Cassino). The other
eight edges are **not shown**: the prerequisites of `docs/TUTOR.md` 2 stay as the
logical order of the sources (`research/16`); they are not contradicted except
that the data cannot support the claim that `void` or `behind` matter at all.

**Constants set.**

| Constant | Was | Now | Rule |
|---|---|---|---|
| `PARAMS.min_gain` | 1.0 (provisional) | **1.1** | half the least clear worth (`announce`, 2.12 thirds a deal), to 0.1 |
| `PARAMS.costly_gain` | 4.0 (provisional) | **4.4** | four times `min_gain` |
| `PARAMS.costly_chances` | 8 | **8** | twice the largest `min_chances` (4) |

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m4 --only
singles` and `... m4 --only pairs --bid coin` (defaults: `--seed 40000000
--batch 40 --looks 4 --pair-looks 3`).

### Decision on M4's result (15 Oct 2026, the orchestrating session)

`void` and `behind` showed no worth in Points against whole rung 4 (M4), so they
are **praise-only**: judged, counted and praised, never the focus
(`SkillSpec::may_focus` false in `kit/learner`; `docs/TUTOR.md` 2). Only 2 of
the 10 prerequisite edges were shown; the rest are kept as the logical order.
M5 therefore needs no gate for their rules; they are reported below as not
applicable.

### Result (2026-10-15)

**Selection**: rung 4 against rung 3, 400 sittings (2,400 deals), default
settings, seeds from 50,000,000, 78 seconds. Firings, those satisfied, precision
and the lower bound of the Wilson interval at the effective sample (clustered by
deal); `*` marks a lower bound of at least 0.80.

| Candidate | Fired | Satisfied | Precision | n_eff | Lower |
|---|---|---|---|---|---|
| worth.rich.1 (led 7 thirds or more: win) | 115 | 112 | 0.974 | 115 | **0.926** `*` |
| worth.rich.2 (10 or more) | 29 | 29 | 1.000 | 29 | 0.883 `*` |
| worth.rich.3 (13) | 0 | | | | |
| worth.poor.1 (led 1 third, a king wins: no king) | 1,896 | 1,550 | 0.818 | 1,795 | 0.799 |
| worth.poor.2 (4 thirds or less) | 1,976 | 1,589 | 0.804 | 1,879 | 0.786 |
| cheap.1 (the cheapest of the outcome) | 20,432 | 15,027 | 0.735 | 19,224 | 0.729 |
| cheap.2 (the cheapest winner) | 14,487 | 10,348 | 0.714 | 14,487 | 0.707 |
| off.pip.1 (following: a cover) | 4,524 | 2,405 | 0.532 | 4,524 | 0.517 |
| off.pip.2 (leading: a cover) | 25,212 | 6,713 | 0.266 | 25,212 | 0.261 |
| off.figure.1 (a figure cover legal: a cover) | 11,524 | 3,564 | 0.309 | 11,524 | 0.301 |
| off.figure.2 (that figure) | 11,524 | 2,321 | 0.201 | 11,524 | 0.194 |
| off.figure.3 (... unless he shows two low cards) | 10,910 | 2,314 | 0.212 | 10,890 | 0.205 |
| run.1 (top of the longest run, else lowest) | 9,533 | 416 | 0.044 | 9,382 | 0.040 |
| run.2 (a card of any run, else lowest) | 9,533 | 7,228 | 0.758 | 8,839 | 0.749 |
| run.part.3 (holding a run: a card of it) | 9,065 | 7,089 | 0.782 | 8,999 | 0.773 |
| run.part.4 (no run: the lowest) | 468 | 139 | 0.297 | 468 | 0.257 |
| pagat.cash.1 (7 tarocks or fewer: the Pagat) | 2,410 | 446 | 0.185 | 2,088 | 0.169 |
| pagat.cash.2 (8 or fewer) | 2,888 | 510 | 0.177 | 2,681 | 0.163 |
| pagat.keep.1 (12 or more: keep it) | 117 | 89 | 0.761 | 113 | 0.674 |
| pagat.keep.2 (11 or more) | 269 | 214 | 0.796 | 258 | 0.742 |
| void.1 (not into a shown void) | 12,607 | 7,594 | 0.602 | 12,070 | 0.594 |
| behind.1 (not in a split suit) | 26,015 | 13,855 | 0.533 | 15,086 | 0.525 |
| behind.2 (a tarock) | 25,113 | 6,146 | 0.245 | 14,163 | 0.238 |
| behind.3 (a tarock or a suit mostly mine) | 25,850 | 12,231 | 0.473 | 13,680 | 0.465 |
| bid.1 (60-75 thirds sure, 6-7 tarocks: take up) | 58 | 44 | 0.759 | 58 | 0.635 |
| bid.2 (60 thirds sure, 6 or more tarocks) | 72 | 58 | 0.806 | 72 | 0.700 |
| announce.1 (a held honour's card: announce) | 233 | 233 | 1.000 | 233 | **0.984** `*` |

Two rules pass: `worth.rich` (candidate 1) and `announce`. `worth.poor.1` misses
the gate by 0.001 (lower bound 0.799) and fails by the rule. Rung 4 plays a
cover when it has the choice only 53% of the time when following and 27% when
leading, and a lying figure at once only 20% of the time: the books' advice on
`off` (and `pagat` cashing at 7 or fewer tarocks, 19%) is not what the
strongest rung does.

**Confirmation** on fresh seeds (55,000,000, 400 sittings, 81 s) of the selected
candidate of each rule that passed: `worth.rich.1` 121 firings, 115 satisfied,
0.950, lower bound **0.896**; `announce.1` 206 of 206, lower bound **0.982**.
Both confirmed.

**Under each other setting** (200 sittings each, 1,200 deals):

| Setting (seeds) | worth.rich.1: fired, satisfied, lower | announce.1: fired, satisfied, lower |
|---|---|---|
| Values 1/2/3 (56,000,000) | 60, 56, 0.841 | 110, 110, 0.966 |
| Values 1/2/4 (57,000,000) | 60, 60, 0.940 | 95, 95, 0.961 |
| Both pass: throw in (58,000,000) | 25, 24, 0.804 | 71, 71, 0.949 |
| Ultimo 2 Points (59,000,000) | 42, 40, 0.842 | 98, 98, 0.962 |

Both rules are confirmed under every setting; they are told under any.

**Verdicts and gates** (`lessons::LESSONS`):

| Rule | Verdict | Gate |
|---|---|---|
| worth.rich | passed, confirmed, all settings | Passed |
| announce | passed, confirmed, all settings | Passed |
| worth.poor | failed (best 0.799) | Failed |
| cheap | failed (0.729) | Failed |
| off.pip, off.figure (with its exception) | failed (0.517, 0.301) | Failed |
| run | failed (0.749) | Failed |
| pagat.cash, pagat.keep | failed (0.169, 0.742) | Failed |
| bid | failed (0.700) | Failed |
| endgame | not measurable: it prescribes no card | Failed (the fact is told) |
| void | not applicable (praise-only, M4); measured 0.594 | Failed |
| behind | no rule exists (candidates 0.525, 0.238, 0.465); praise-only | n/a |

A rule that fails is told as its fact. Coverage (the share of the skill's clear
chances in which the trigger fires) was not measured.

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m5`
(selection), `... m5 --seed 55000000 --only worth.rich.1,announce.1`
(confirmation), `... m5 --seed 56000000 --sittings 200 --setting v123 --only
worth.rich.1,announce.1` and the same with `57000000 v124`, `58000000 throwin`,
`59000000 ult2`.

## M6 Diagnosis

### Plan (2026-10-15, before the run)

**Question.** With which constants does the learner name the skill a lesioned
student lacks within a few deals, and name none for rung 4 whole? This sets
`PARAMS.discount`, `lower`, `upper`, `master_of_top` and `lose_below`
(`docs/TUTOR.md` 5; the gains and `costly_chances` are M4's).

**Students and windows.** Twelve students play against rung 3 (as in M3, in seat A
in the even sittings and in seat B in the odd ones): rung 4 whole; the knock-outs
`Worth`, `Cheap`, `Off`, `Run`, `Pagat` (coin), `Void`, `Behind`, `Endgame`,
`Bid` (coin), `AlwaysPass`, `Announce`. `Void` and `Behind` are praise-only after M4
(`may_focus` false): nothing is to be detected in them, and the focus their students
get after the last deal is reported. A **window** is 3 consecutive sittings
(18 deals) from consecutive seeds, the same seeds for every student; each deal's
evidence (no assisted decision) is one summary; the learner is run on each
prefix (after 1 deal, 2 deals, ...). Tuning windows: 6, from seed 60,000,000.
Fresh windows: 6, from seed 65,000,000. Fixed counts.

**Measures.** For a knock-out other than those two, its **detection**: the share of windows in which
the skill it lacks is the learner's focus after some deal up to the limit (6
deals for the total knock-outs, 12 for the coin ones `Pagat`, `Bid`,
`AlwaysPass`, and 18 for `Announce`, whose chances come once in about twelve
deals). The **sensitivity** is the mean detection over the nine knock-outs that can be detected.
For rung 4 whole: the **false-focus rate**, the share of (window, prefix from
the second deal) at which any focus is named, and the skills mastered after the
last deal.

**Grid and choice, fixed now.** `discount` in {0.75, 0.80, 0.83, 0.88, 0.92},
`lower` in {0.1, 0.2, 0.3}, `upper` in {0.8, 0.9, 0.95}, `master_of_top` in
{0.75, 0.80, 0.85, 0.90}, `lose_below` in {0.1, 0.2, 0.3} (540 sets), with the
ladder of M3 and the gains of M4. On the tuning windows: among the sets with a
false-focus rate of at most 0.10 and at least 3 skills mastered by rung 4 whole,
the highest sensitivity; ties go to the lower false-focus rate, then to the set
nearest the provisional constants. If none qualifies, the highest sensitivity
less the false-focus rate.

**Acceptance, on the fresh windows with the chosen set.** Sensitivity at least
0.7, false-focus rate at most 0.15, at least 3 skills mastered by rung 4
whole. Reported as met or not met; the set is kept either way and the result
says which.

**Stopping.** Descriptive fixed counts, no looks. `lose_below` shows only when
a mastered skill is lost, which 18 deals seldom do; the grid reports whether it
moved the scores, and if not the Cassino value 0.2 is kept.

### Result (2026-10-15)

Twelve students, 6 windows of 18 deals each on the tuning seeds (60,000,000) and
6 on the fresh seeds (65,000,000); 178 seconds to play and judge the 144
windows, 500 seconds to score the 540 sets of the grid on the tuning windows.
Descriptive, fixed counts. Detection is the share of the 6 windows in which the
skill the student lacks was the focus within the limit (6 deals; 12 for
`Pagat`, `Bid`, `AlwaysPass`; 18 for `Announce`).

| | Sensitivity | Worth | Cheap | Off | Run | Pagat | Endgame | Bid | AlwaysPass | Announce | False focus | Whole mastered |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| Provisional constants, tuning | 0.537 | 1.00 | 0.50 | 1.00 | 0.17 | 0.83 | 0.50 | 0.67 | 0.17 | 0.00 | 0.098 | 4.00 |
| Provisional constants, fresh | 0.537 | 1.00 | 0.83 | 1.00 | 0.33 | 0.50 | 0.33 | 0.67 | 0.17 | 0.00 | 0.059 | 4.33 |
| **Chosen**, tuning | 0.574 | 1.00 | 0.50 | 1.00 | 0.33 | 0.83 | 0.50 | 0.67 | 0.33 | 0.00 | 0.078 | 6.50 |
| **Chosen**, fresh | 0.556 | 1.00 | 0.83 | 1.00 | 0.33 | 0.50 | 0.33 | 0.83 | 0.17 | 0.00 | 0.039 | 6.33 |

The chosen set by the rule of the plan: `discount` 0.88, `lower` 0.2, `upper`
0.8, `master_of_top` 0.80, `lose_below` 0.2 (the three values of `lose_below`
gave the same scores; the one nearest the provisional constants is kept).

**Acceptance on the fresh windows: not met** (sensitivity 0.556 against 0.7;
the false-focus rate 0.039 and the 6.33 skills mastered by rung 4 whole meet
theirs). `worth` and `off` are named in every window within six deals, `cheap`
and `bid` in 5 of 6, the rest less often: `run` in 2 of 6 (a random tarock lead
costs only 1.3 Points a pair, M4, and is within the margin of the best often),
`pagat`, `endgame` and the coin `bid` in 3 to 5 of 6, `AlwaysPass` in 1 of 6
(it is rarely clearly wrong, M4), and `announce` never (about 1.4 chances in
18 deals against a bar of 2). The rare skills cannot be certified in one
window of 18 deals. The set is kept; the finding is that the learner names the
two skills worth most (`worth`, `off`; also `cheap` and `bid` often), not all
ten, in a few deals.

The students lacking the praise-only skills: after the last deal of the 6
tuning windows the focus is none in all of them for `Void` and `Behind`; on
the fresh windows `Void` is told `off` in 2 and `Behind` `run` in 1 (a class
knock-out hurts a neighbour: M4's matrix, `docs/TUTOR.md` 11).

The test `a_student_who_lacks_worth_or_off_is_pointed_at_it_and_rung_four_whole_is_not`
(`tutor_tests.rs`, run by `bin/lesions`, seeds from 66,000,000) holds the two
reliable detections and the false-focus rate on a third set of windows.

**Constants set** (`tutor::PARAMS`):

| Constant | Was | Now |
|---|---|---|
| `discount` | 0.83 (M3) | **0.88** |
| `lower` | 0.2 | **0.2** |
| `upper` | 0.9 | **0.8** |
| `master_of_top` | 0.85 | **0.80** |
| `lose_below` | 0.2 | **0.2** |

**Command.** `cargo run --release -p strohmandeln-ai --bin measure -- m6
--windows 6` (defaults: `--seed 60000000 --fresh-seed 65000000 --sittings 3`;
`--tune-cache FILE` and `--fresh-cache FILE` keep the windows' summaries so the
grid can be rerun without playing).
