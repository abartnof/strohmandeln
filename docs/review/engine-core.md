# Review: `strohmandeln-core`

Reviewed against `docs/RULES.md` and `docs/MESSAGES.md`, on `c12c127`. The
reviewer did not write the code. Tests added by the review: `tests/review.rs`
(independent re-computations, fuzz, leak checks) and unit tests named below.

## Method

- Every clause of RULES 1 to 16 was read against the code and the tests that
  cover it.
- Three independent re-implementations, written from RULES.md and not from the
  code, run against the engine over random sittings (all 12 setting
  combinations, 3 and 6 Touren): the score of each played deal from the event
  stream alone (game, Valat, honours, ultimo, doubling, Contos); the legal
  plays from a view alone; the conservation of the 54 cards.
- A fuzz applies arbitrary actions (any seat, any of the 54 cards, announce
  lists, bids, extension answers) to running sittings: 3,000 steps a seed.
- Leak tests on the events `apply` returns and on the view, with the cards the
  seat must not know recomputed from the full state.

## Findings

| # | Clause or area | Problem | Evidence | Status |
|---|---|---|---|---|
| 1 | 3.3, 11.4, 5.7: `Deal::legal_plays` | Returned the whole playable set to a seat that was not to move, and during the bidding (a dealt hand). Harmless in the engine (callers check the turn) but a trap for the table and a hand listing in the API. | `legal_plays_are_empty_for_a_seat_that_is_not_to_move` failed first | fixed 5597cce |
| 2 | API, MESSAGES: `Sitting::new`, `replay` | Errors were English sentences (`Result<_, String>`), against "no prose on the wire". | `setup_errors_are_keys_with_parameters_not_sentences` | fixed 2dc6575 (`SetupError`, keys in MESSAGES.md; a test keeps the doc and the enum equal) |
| 3 | 12.3: settings | `ultimo` accepted any u32. A hand-made record with 0 or a large value was accepted (large: `points * multiplier` overflows, a panic in debug and a wrong score in release). | `rules_0_the_ultimo_setting_is_1_or_2_and_nothing_else` | fixed 2dc6575 |
| 4 | Record / wasm: `Record.seed` | A u64 as a JSON number: JavaScript rounds seeds above 2^53, so a record kept by the page would replay a different game. | `the_record_keeps_a_seed_beyond_2_pow_53_exactly_through_json` | fixed 2dc6575 (string on write; number or string on read) |
| 5 | Leak API: `Sitting::apply` | Returns every event, including the other seat's `dealt` (their 15 cards) and, at a deal boundary, the next deal's. Safe only if the caller filters; only `filter_event` (one event) existed. | `rules_13_the_events_returned_by_apply_leak_nothing_once_filtered` shows the filtered stream is clean; the unfiltered one is not | fixed aecab9f (`filter_events`); the wasm layer must use it (see open items) |
| 6 | 1.2: `Card::parse` | Built the code of up to 54 cards per call (about 54 allocations per deserialised card); it dominated the run time of the leak and fuzz tests. | profile of `tests/review.rs`; no behaviour change | fixed 4994215 (+ `rules_1_2_only_the_54_codes_parse...`) |
| 7 | 12.2, 4.7, 6.1: Pagat from a packet | No test took a Pagat that is the last packet card, pulled after trick 26, and played in trick 27. Code is right. | `rules_12_2_a_pagat_pulled_from_a_packet_wins_the_ultimo`, `..._and_beaten_loses_the_ultimo` | test added 5597cce |
| 8 | 10: Valat | Valat was tested only on a faked trick log. Played out (22 tarocks, 4 kings, Qh against 27 suit cards), declared by A, by B and simple, with the ultimo: 12 + 1, no game value, takes 210:0. Code is right. | `rules_10_1_a_valat_played_out_...` | test added 5597cce |
| 9 | 4.5 | No test for a player whose three packets are all tarocks and kings (12 cards pulled, 3 last-card flags, no cover) nor for three low covers (nothing pulled). Code is right. | `rules_4_5_packets_of_only_tarocks_and_kings...`, `rules_4_5_three_low_covers_take_nothing` | test added |
| 10 | 8, 9, 10, 11, 12, 14.2: scoring | Checked by an independent scorer on 150 seeds (about 900 deals): results, credits, multiplier, Contos, takes in thirds. No difference found. The 35 1/3 / 35 2/3 edge (106 / 107) has unit tests. | `rules_8_to_12_the_engine_scores_every_played_deal_as_an_independent_reading_does` | no bug |
| 11 | 5.2 to 5.4: legal plays | Checked by an independent reading of the view on 80 seeds (over 10,000 decisions): hand plus covers, follow suit, else tarock, else anything. No difference, never an empty set. | `rules_5_2_to_5_4_legal_plays_equal_an_independent_reading_at_every_step` | no bug |
| 12 | 13: leaks | No view and no filtered event contains a hidden card (the other seat's unannounced dealt cards, any face-down packet card), over random play with announcements; nothing in a view beyond own hand, covers, tricks, known cards and the events. No unfiltered path in `view`. | `rules_13_*` in `tests/review.rs`, `view.rs` tests | no bug |
| 13 | 13, 11.5: forfeits | `honour_forfeited` is public (RULES 13.1) and is emitted when a player plays a card of an honour he held and did not announce. It tells the opponent that the player held all of Troule (or all four kings) at that moment: a hand fact that no announcement gave. The spec lists it as public, so the code is right; whether that is intended is a design question. | `Deal::play` pushes `HonourForfeited`; `DealView.forfeited` | open (maintainer) |
| 14 | 13: void deals | `view().events` starts at the last `deal_start`, so the `void_deal` event of an annulled deal is in `apply`'s return value but not in the view. The table must show it from the returned events. | `view()` in `view.rs` | open (note for work package 5) |
| 15 | Record | The record holds the seed, so whoever has the record has every future shuffle. Fine for an offline single-player game; the wasm layer must not give the record to the opponent code. | `Record` | open (design note) |
| 16 | 14.4, 14.6: throw-in and the pending doubling | The clause "a throw-in does not clear the doubling" can never apply: draws exist only under `both_pass = play`, throw-ins only under `throw_in`. The code is right (a throw-in leaves the flag alone). | `rules_14_4_...` uses `settle_for_test` to force the state | open (spec note, no action) |
| 17 | Agents | `FirstLegal::choose` and `RandomPlayer::choose` panic on a view with no legal action (a seat not to move). Documented "only on the player's turn". | `agents.rs` | open (minor) |
| 18 | Performance | `view()` clones the events of the current deal and `legal_actions` on every call, and the suite takes about 3 minutes in the gate (the 120-seed property tests dominate). Not a problem for play; the gate is slow. | `bin/gate` timing | open (minor) |
| 19 | Simplicity | `Deal` and `Sitting` expose many accessors only tests use (`packet_cards`, `leader`, `hand_cards`, `honour_state`); `honour_index` and `HONOURS` could be `Honour::ALL` and `as usize`; `legal_actions` in `Deal` mixes announce and play actions. None is wrong. | reading | open (minor) |

## Clauses and tests

Clauses 1.x, 2.3 to 2.6, 3.1 to 3.4, 4.2 to 4.8, 5.1 to 5.7, 6.1 to 6.4, 7.1,
8.1 to 8.4, 9.1 to 9.3, 10.1 to 10.2, 11.1 to 11.7, 12.1 to 12.4, 13.1 to 13.7,
14.1 to 14.6, 15.1 to 15.10 and the events of 16 each have a unit test, or are
covered by an independent-reading test above.

Clauses with no direct test, or only a thin one:

- 2.2: that the shuffle is a uniform permutation (only the bias-free `below`
  and a card count are tested; the 54 cards in a deal are checked by the
  conservation test).
- 2.7, 8.5, 10.3, 13.6, 15.10: statements of what is absent or of display; no
  code to test.
- 3.3 "each player speaks at most once": tested only through the fuzz (a third
  bid is `wrong-phase`); no named unit test.
- 4.2 in the dealer's direction: the order of the dealer's reveal is only
  checked by the monotone order test, not by its events one by one.
- 11.2 "a cover is not in the hand" for honours: no honour card can be a cover
  (4.8), so there is nothing to observe.
- 15.4 "the Touren begin with him": the first dealer's identity is tested, not
  that the deal 1 of Tour 1 is his in an extension.
- 15.9 `tour_end` payload (`balance`, Contos) is only counted, not compared.
- 16 `doubling_applied` on a redeal after a void deal repeats; not specified,
  not tested.
- Real packets and a full deal through `Deal::new` + play to the end with a
  hand-chosen deck are covered only by random play; there is no fixed,
  hand-checked full deal.

## The "property" tests

`tests/properties.rs` loops over 120 seeds with random legal players. As
shipped they were weak in three ways, now covered by `tests/review.rs`:

- They never made a Play with an `announce` list (random players chose the
  separate `Announce` action only) and never produced a Valat; the scorer was
  checked only through the sum of 210 and the Conto total, not against a second
  implementation.
- The leak test ran on 6 seeds and did not look at the events `apply` returns.
- The refusal test checked that a refusal is non-empty prose (`Display`), not
  that the wire form is a key.

They remain loops over fixed seeds (reproducible, not shrinking). The variety
guard (draw, announcement, ultimo, doubled deal) is useful; a Valat guard is
not possible with random players, so Valat is covered by the played-out test.

## Verdict

The rules are implemented as specified: no conformance bug was found in
bidding, the reveal, the legal plays, replacement, counting, the honours,
ultimo, Valat, the doubling or the session, by reading and by three
independent re-computations over several thousand deals. The faults found were
at the API edge (prose errors, unvalidated settings, a seed that JavaScript
cannot hold, an unfiltered event list, a permissive `legal_plays`), and are
fixed with tests. The no-leak property holds for views and for filtered
events.

Open: items 13 to 19 above. Item 5 depends on the wasm layer using
`view::filter_events` for every event it sends to a seat, and on keeping the
`Record` and `Sitting::events()` on the Rust side of the wasm boundary.
