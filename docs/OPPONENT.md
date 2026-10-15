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

`level` is 1 to 4 in half steps (the protocol clamps and rounds). A whole level
plays that rung. Level L.5 plays rung L+1, but on each decision slips down one
rung with probability one half (a draw from a hash of the view, so it stays a
function of the seed and view). A slip is one rung only.

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
| 2 vs 1 | 50 x 4 | look 1 (z 19.4 > 4.05) | 50 | +18.36 +- 0.95 | +16.50 to +20.22 |
| 3 vs 2 | 50 x 4 | look 3 (z 2.35 > 2.34) | 150 | +1.39 +- 0.59 | +0.23 to +2.56 |
| 4 vs 3 | 20 x 5 | look 2 (z 5.81 > 3.23) | 40 | +6.73 +- 1.16 | +4.46 to +8.99 |

Rung 3 over rung 2 is the thinnest step (stopped on the boundary at the third
look); the interval excludes zero but a later tuning pass may widen it.

## Speed

wasm build in Node on the development machine (2 vCPU), opponent `step`, whole
3-Tour sittings, three seeds per level (about 495 decisions each):

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

The tutor's two sampled lines are counted over `hint::LEAD_WORLDS` = 1,000
deals ("takes the trick in N of 1,000", `lead-rate`, `lead-cheap`) and
`hint::ODDS_WORLDS` = 50 deals (`odds`), each searched with the decision's own
node share per world (`odds_config`: 30 000 / 10 = 3 000 nodes). The
opponent's own lead still weighs `rung3::WORLDS` = 24; the hinted move does
not depend on either constant (separate random streams, `hint.rs` tests).

Measured by `hintbench` over 2 300 positions of 50 sittings (opponent level 3
in both seats; 1 000 leads, 1 000 follows, 300 bids), native release, 2 vCPU,
before the change (15 Oct 2026):

| Lead rate, n | median | p95 | max | 95% half-width at p = 0.5 | wording flips between two samples |
|---|---|---|---|---|---|
| 24 (before) | 0.04 ms | 0.13 ms | 0.29 ms | 20 points | 2.4% |
| 100 | 0.18 ms | 0.56 ms | 5 ms | 9.8 | 1.4% |
| 1 000 (now) | 1.9 ms | 13 ms | 24 ms | 3.1 | 0.6% |
| 10 000 | 16 ms | 66 ms | 150 ms | 1.0 | 0.2% |

| Odds worlds, scaled nodes | median | p95 | max | odds given |
|---|---|---|---|---|
| 10 (before) | 2.3 ms | 5.7 ms | 20 ms | 39% |
| 25 | 4.3 ms | 23 ms | 67 ms | 42% |
| 50 (now) | 12 ms | 49 ms | 94 ms | 43% |
| 100 | 17 ms | 55 ms | 125 ms | 43% |
| 200 | 41 ms | 184 ms | 369 ms | 44% |

With the 30 000 nodes left shared, more worlds are searched shallower and the
line is given less often (26% at 50 worlds, 19% at 200); hence the scaling.
The whole level-4 hint was 4.6 ms median, 13.6 ms p95, 35 ms max (leads).
"Wording flips" is how often two independent samples fall on different sides
of the `lead-cheap` threshold (wins at most 20%), computed from the binomial
at each lead's proportion over 2 000 worlds; it does not reach zero because
some leads sit on the threshold.

The wasm build under Node 18 on the same machine takes 1.8 times the native
time (median 1.8, p95 2.3, over 1 461 identical hints; `crates/strohmandeln-wasm/tests/hintbench.mjs`
against `examples/hintbench.rs`). The added cost of the new constants is about
2 ms median and 13 ms p95 for the lead rate, 10 ms and 45 ms for the odds,
natively; times 1.8 in the browser, more on a phone. The hint runs on the main
thread. In a Web Worker the same budget would allow about 10 000 lead worlds
and 200 odds worlds in under a second on this machine (not built).

Reproduce:

```text
cargo run --release -p strohmandeln-ai --bin hintbench -- --sittings 50 \
  --cap-lead 1000 --cap-follow 1000 --cap-bid 300 \
  --lead-ns 24,100,1000,10000 --odds-worlds 10,25,50,100,200
cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
node crates/strohmandeln-wasm/tests/hintbench.mjs target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm 10 > wasm.txt
cargo run --release -p strohmandeln-wasm --example hintbench -- 10 > native.txt   # compare line by line
```
