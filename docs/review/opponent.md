# Review: strohmandeln-ai, the wasm wiring, PROTOCOL, MESSAGES hint table, OPPONENT

Branch worktree-agent-a7597b6980b8abc6b, against docs/RULES.md and the tests.
Reviewer did not write the code.

## Checks

- **No information leak.** `Knowledge::from_view` takes only a `View`; no
  non-test code in the crate touches a `Deal` or `Sitting` (`Sim::from_deal` is
  used by tests and `bin/bench.rs` only). The wasm `step` and `hint` build their
  views with `view(sitting, seat)`. The sampler draws only from
  `Knowledge::unseen`. Blind (twin-deal) tests existed for rungs 2, 3, 4 and the
  sampler; none for the `Opponent` (level dial, slips, config) or the hint, the
  two entry points the table calls. Added both (finding 1).
- **Sampler.** Opponent hand avoids shown voids; known cards (`opp_known`,
  covers, own hand, played) are excluded; packet sizes match the table; uniform
  among consistent worlds (tested by frequency). The void fallback (voids leave
  too few cards) cannot occur on a real view. Correct.
- **Void inference** matches RULES 5.4 (no follow: none of the kind; no trump on
  a suit lead: no tarock), applied only to tricks I led, and only to dealt
  hand cards (packet cards were not playable then). Correct; tested against the
  truth.
- **Sim vs engine.** `the_sim_plays_exactly_as_the_engine_does` and the
  full-playout score test compare move by move and final margin. Adequate.
- **Stats.** O'Brien-Fleming constants times sqrt(looks/k), fixed in `Plan`
  before the first batch, z against the boundary at each look, futility only
  from look 2, last look reports the interval; false-alarm rate simulated
  (20 000 runs in 4.2% to 5.8%). Mirrored pairs in `tournament.rs`, antisymmetry
  and exact-zero tests. Matches CLAUDE.md. The interval is the plain 95% one
  at the last look, as CLAUDE.md asks, not an adjusted one.

## Fixed

| # | Finding |
|---|---|
| 1 | No blind test for `Opponent::decide` (levels 1 to 4 in halves, slips) or `hint`; added `the_opponent_decides_from_its_view_alone_at_every_level` and `a_hint_is_a_function_of_the_view_alone` |
| 2 | `measurements/README.md` is cited in lib.rs, rung2.rs, bench.rs and does not exist; now `docs/OPPONENT.md` |
| 3 | `sittingConfig` sent `round((skill-1)*2)` (0 to 6); the protocol's `level` is 1 to 4 in half steps (a 0 to 6 level made skill 1 play level 1 and 4 play 4 only by clamping, skill 2.5 play level 3). Now the dial passes as is; test covers every `SKILLS` value |
| 4 | MESSAGES.md German: "Vierer" is in research only as "Vierer-Tarock" (four-hand Tarock, research/12), not the take-up; now "aufnehmen" (Werner, research/03). "Probespiele" is attested nowhere; now "durchgespielte Austeilungen". "Anspiel" made neuter ("das Anspiel"). "Honneur", "Impassiren" (Werner, research/07), "Stecher" (sure winner, Mayr and Sedlaczek, research/07), "Paket" (research/07) are attested and kept |

## Open

| # | Finding |
|---|---|
| A | Rung 4's `bid_advantage` hard-codes the default values 2/3/4 and ignores the `values` and `both_pass` settings and the multiplier (a pending doubling); the view does not carry them. Under 1/2/3 or 1/2/4, or after a drawn game, the bid threshold is slightly off. Minor; fix by putting the settings in `DealView` if the maintainer wants the opponent to follow them |
| B | The hint's German "the 4" in the English column ("take up the 4") is unexplained to a player who does not know the 4 is the declared game; consider "declare" wording in a later strings pass |
| C | A hint whose move is a play not in the human's legal list becomes `null` in wasm `hint`; cannot happen while the hint and `legal` share a view, but no test pins it |

## Review: settings in the view, rung 4 bid estimate (3f57328, 4c868d7)

Checked against RULES 3.4, 8.3, 8.4, 9.1, 13, 14.

- **Arithmetic.** Take up: win value at p (107 thirds or more), loss value
  otherwise, so a 104-106 draw is a loss (8.3). Simple game: simple value times
  (p - q), draw nothing (8.4, 9.2). Throw-in: the alternative is 0 (3.4), and a
  declared game cannot be drawn (14.6). The multiplier scales the whole
  difference (14.2). Correct.
- **Leak.** `values`, `both_pass` and the multiplier are sitting settings and
  the pending doubling, public per 13.1 (now listed there). No hidden card is
  added to the view.
- **Golden.** Only the reply hashes changed (new fields in the reply); event
  counts, Contos and balances are identical.
- **PROTOCOL.md.** Accurate (field names, string forms checked by the view test).
- **Hint test.** Covers every level, two seeds; adequate and closes Open C.
- Open A is closed by these commits.

Note: `bid_margin` is a fixed Points slack; with the advantage now scaled by
the multiplier it is relatively halved in a doubled deal. Left as is (the
expectation, not the margin, is what a doubling changes). Fixed: RULES 13.1 now
lists the settings among the public items.
