# The computer opponent

Crate `strohmandeln-ai`. Every decision is a pure function of a seed and the
seat's own view (`strohmandeln_core::view`), so it can reach no hidden card and
a record replays.

## Rungs

| Rung | Name | What it does |
|---|---|---|
| 1 | Legal | A random legal move. |
| 2 | Rules of thumb | The maxims of `research/07` s1 as a one-trick scoring of the legal cards (finesse, packets played off, exposed figures, split suits, Pagat held for the last trick, king cashing, drawing trumps); a bid from sure winners and tarocks. No memory of earlier tricks. |
| 3 | Counter | Remembers every card seen and the voids shown; draws worlds for the unseen cards (own unturned packet cards included) and scores a lead by the sampled trick; keeps rung 2's maxims. |
| 4 | Searcher | Perfect-information Monte Carlo: samples worlds (rung 3's sampler), solves each double-dummy, plays the best average margin; bids by the sampled expectation, priced with the sitting's `values`, `both_pass` and the deal's doubling multiplier (all carried by the view; throw-in values the simple game at nothing). Budget: 10 worlds, 30 000 nodes per decision, 40 worlds for a bid (`Budget::default`). |

## Level dial

The skill dial is 1, 2, 3, 3.5, 4 (`docs/STYLES.md` 1.1; the protocol clamps and
snaps any other number to the nearest position, ties down: 1.5 is 1, 2.5 is 2).
Nothing is played at random to be weak.

| Level | Plays |
|---|---|
| 1 | Rung 2 with `burden` only (the maxims `worth` and `cheap`); ties by card order, not by the seed; takes up with 60 sure thirds and any tarocks. A style at level 1 changes the bid only. |
| 2, 3, 4 | Rungs 2, 3, 4. |
| 3.5 | Rung 4 with 3 worlds and 8 000 nodes at most (a smaller `Budget` still binds). |

Rung 1 stays as the test baseline (`rung1.rs`), not on the dial. Saved sittings
at 1 or x.5 replay (the opponent's moves are recorded) and continue under the new
meaning.

## Styles

`docs/STYLES.md`: a style is a parameter set over `rung2::Weights`, the bid rule
(`BidRule`) and Rung 4's band (`style.rs`, each parameter tagged T, I or E with
its passage). Normal is every default; `golden_tests.rs` hashes its decisions at
levels 2, 3 and 4. At rung 4 a style acts in the bid (a margin per style,
`style::MARGINS`, 0.0 until `stage0` has calibrated it) and in a band of 2
thirds where the maxims decide (Normal 0.34).
`cargo build --release -p strohmandeln-ai --bin stage0`: the stage-0 probe
(divergence from Normal, bid rate, the rung-4 margin calibration, the tells'
features as CSV). `tournament` takes `style=ID` in a spec.

The tutor's hint (`hint.rs`) always plays rung 4 at whole level 4 and names the
maxim that weighed most (reason keys: `docs/MESSAGES.md`).

## Measurements

`tournament A B`: mirrored pairs (each deal played twice, seats swapped), unit
the Points A takes from B over one pair of 3-Tour sittings; sequential with
O'Brien-Fleming boundaries fixed before the first batch, futility stop off.
`cargo build --release -p strohmandeln-ai --bin tournament`, first seed
1 000 000.

| Comparison | Plan (batch x looks) | Stopped | Pairs | Mean +- se | 95% interval at last look |
|---|---|---|---|---|---|
| 2 vs 1, old level 1 (random play) | 50 x 4 | look 1 (z 19.4 > 4.05) | 50 | +18.36 +- 0.95 | +16.50 to +20.22 |
| 3 vs 2 | 50 x 4 | look 3 (z 2.35 > 2.34) | 150 | +1.39 +- 0.59 | +0.23 to +2.56 |
| 3 vs 2, rerun (15 Oct 2026, first seed 2 000 000, `--negligible 0.5`) | 100 x 6 | look 3 (z 3.88 > 2.90) | 300 | +1.53 +- 0.40 | +0.76 to +2.31 |
| 4 vs 3 | 20 x 5 | look 2 (z 5.81 > 3.23) | 40 | +6.73 +- 1.16 | +4.46 to +8.99 |

The rows above predate the dial: "1" in them is rung 1, random play, now only a
test baseline. Level 1 against random and level 2 against level 1 are the tests
`level_one_beats_random_play...` and `level_two_beats_level_one...` in
`tournament.rs`, not yet measured at length.

Rung 3 over rung 2 is the thinnest step. The first run stopped on the boundary
at its third look; the rerun on fresh deals (twice the pairs) agrees and stops
clear of it: +1.53 Points per pair, interval +0.76 to +2.31.

## Speed

wasm build in Node on the development machine (2 vCPU), opponent `step`, whole
3-Tour sittings, three seeds per level (about 495 decisions each). Measured
before the dial changed: level 1 was random, and level 3.5 did not exist (not
re-measured):

| Level | median | 95th percentile | max |
|---|---|---|---|
| 1 | 0.7 ms | 3.7 ms | 137 ms |
| 2 | 0.7 ms | 1.2 ms | 126 ms |
| 3 | 0.8 ms | 1.3 ms | 132 ms |
| 4 | 7.0 ms | 135 ms | 192 ms |

The slowest decisions are bids (sampled deals). No cap beyond the node budget
is needed; `Budget` is the knob for a slower device. The wasm module is 466 KB
(release, unstripped).

### Hint evidence

The tutor's two sampled lines are counted over `hint::LEAD_WORLDS` = 1 000
deals (`lead-rate`, `lead-cheap`: "takes the trick in N of 1 000") and
`hint::ODDS_WORLDS` = 50 deals (`odds`), each searched with the decision's own
node share per world (`odds_config`: 30 000 / 10 = 3 000 nodes). Until
15 Oct 2026 they were 24 (`rung3::WORLDS`) and 10 with the 30 000 nodes shared.
The opponent's own lead still weighs `rung3::WORLDS` = 24; the hinted move does
not depend on either constant (separate random streams, `hint.rs` tests).

Measured by `hintbench` over 2 300 positions of 50 sittings (opponent level 3
in both seats; 1 000 leads, 1 000 follows, 300 bids), native release, 2 vCPU.
Run 1: 15 Oct 2026, with the constants at 24 / 10, machine mostly idle.

| Lead rate, n | median | p95 | max | 95% half-width at p = 0.5 | wording flips between two samples |
|---|---|---|---|---|---|
| 24 | 0.04 ms | 0.13 ms | 0.29 ms | 20 points | 2.4% |
| 100 | 0.18 ms | 0.56 ms | 5 ms | 9.8 | 1.4% |
| 1 000 | 1.9 ms | 13 ms | 24 ms | 3.1 | 0.6% |
| 10 000 | 16 ms | 66 ms | 150 ms | 1.0 | 0.2% |

| Odds worlds, scaled nodes | median | p95 | max | odds given |
|---|---|---|---|---|
| 10 | 2.3 ms | 5.7 ms | 20 ms | 39% |
| 25 | 4.3 ms | 23 ms | 67 ms | 42% |
| 50 | 12 ms | 49 ms | 94 ms | 43% |
| 100 | 17 ms | 55 ms | 125 ms | 43% |
| 200 | 41 ms | 184 ms | 369 ms | 44% |

With the 30 000 nodes left shared, more worlds are searched shallower and the
line is given less often (26% at 50 worlds, 19% at 200); hence the scaling.
"Wording flips" is how often two independent samples fall on different sides
of the `lead-cheap` threshold (wins at most 20%), computed from the binomial
at each lead's proportion over 2 000 worlds; it does not reach zero because
some leads sit on the threshold.

The whole level-4 hint (`hintbench` table 3), native, in ms:

| Record | Lead median / p95 / max | Follow median / p95 / max |
|---|---|---|
| Run 1: 15 Oct 2026, constants 24 / 10, machine mostly idle | 4.6 / 13.6 / 35 | 4.5 / 11.3 / 28 |
| Run 2: 15 Oct 2026, constants 1 000 / 50, load average 4 to 5 on 2 vCPU | 14.8 / 41.6 / 76 | 19.4 / 64.0 / 101 |

Run 2 was taken while other sessions were building, so its figures are an
upper bound (run 1's sums of deltas, about 17 ms median and 60 ms p95, are the
cleaner estimate). Neither run is a phone.

Wasm against native, level-4 hints along the same sittings (the human taking
each hint; `crates/strohmandeln-wasm/tests/hintbench.mjs` against
`examples/hintbench.rs`, Node 18, same machine). The ratio did not reproduce:

| Run | hints | wasm / native, median | p95 |
|---|---|---|---|
| 15 Oct 2026, machine lightly loaded | 1 461 | 1.8 | 2.3 |
| 15 Oct 2026, review, load average 4 on 2 vCPU | about 270 | 0.54 to 0.55 | 1.3 to 1.8 |

So the wasm build in Node is of the same order as native (0.5 to 2 times,
depending on load); no multiplier is claimed, and none is projected to a
browser or a phone, which were not measured. The hint runs on the main thread;
a Web Worker (not built) would take it off the page.

Reproduce (on an idle machine, which the figures above were not):

```text
cargo run --release -p strohmandeln-ai --bin hintbench -- --sittings 50 \
  --cap-lead 1000 --cap-follow 1000 --cap-bid 300 \
  --lead-ns 24,100,1000,10000 --odds-worlds 10,25,50,100,200
cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
node crates/strohmandeln-wasm/tests/hintbench.mjs target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm 10 > wasm.txt
cargo run --release -p strohmandeln-wasm --example hintbench -- 10 > native.txt   # compare line by line
```
