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
| 4 | Searcher | Perfect-information Monte Carlo: samples worlds (rung 3's sampler), solves each double-dummy, plays the best average margin; bids by the sampled expectation. Budget: 10 worlds, 30 000 nodes per decision, 40 worlds for a bid (`Budget::default`). |

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
