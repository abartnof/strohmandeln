# Review: hint reasons (engine), commit 3448fe0

Scope: `explain.rs`, `hint.rs`, `rung4::move_table`, wasm `hint()`, `docs/PROTOCOL.md`, `docs/MESSAGES.md` ("Hint reasons", "Hint rules"). Tests: `cargo test -p strohmandeln-ai -p strohmandeln-wasm` pass (118 + 23 + 2).

## Findings

| # | Finding | Status |
|---|---|---|
| 1 | `decline-poor` was given for any declined winner, including a rich trick (queen led): text said "poor trick" while the points shown were large. Now "poor" is the maxim's own measure (lead thirds <= burden + spend(alt)); otherwise new key `decline-rich` (neutral text). Test: `a_rich_trick_given_up_is_not_called_poor`. | fixed |
| 2 | `hold-pagat` fired when leading a suit card (a king) with a strong hand: "play Kh, not the Pagat". On a lead it now applies only to a tarock. Test: `leading_a_king_is_not_instead_of_the_pagat`. | fixed |
| 3 | `win-over-cheaper` text claimed "the sampled deals favour {card}" at every rung, including those that sample nothing. Reworded to "the better play here" (en, de). | fixed |
| 4 | `win-cheapest` carried rule `cheapest`; `research/07` 1.3 says not to spend a king on a poor trick ("den König schinden und ein Skartindel zugeben"), not "win with the cheapest winner". Rule dropped from `win-cheapest`; `cheapest` reworded to "do not spend a king where a small card serves", basis 1.3 and 1.2 (following suit with junk). `finesse` basis adds 1.3 "nach hinten spielen". `pagat-last` basis notes the source's threshold (more than 11 tarocks) against the hint's 8. | fixed |
| 5 | Explanation cannot change the move: the reason uses streams `0x4811` (unchanged), `0x4812` (odds) and `0x4813` (lead sample), all separate from the opponent's; `rung3::lead_sims` keeps the draw order of the old `score_cards`; `move_table` only adds `per_world`. Golden/strength tests pass unchanged. | ok |
| 6 | Odds: given only when the fresh table's gain over the best other card is >= 1 third, so it never contradicts the choice; `alt` is always a different legal card. `better` counts strictly-better worlds, `gain_thirds` is the mean, so a few big worlds can give "ahead in 3 of 10, by 4 on average". Not contradictory; left. | open (minor) |
| 7 | Latency, native release, top rung, 8 mid-deal positions with a choice: opponent's own move 40-65 ms, whole hint 125 ms (the second 10-world search is about the cost of the first; measurements on this VM vary by 2x). Under 300 ms native; wasm not measured here (expect 1.5-3x). Not changed. Ignored test `hint_latency` reproduces the figure. | ok, re-measure in the browser |
| 8 | `keep-packet` is named whenever the reserved packet card is legal and not the hinted card, whatever else decided the card; true (that card is kept) but not always the deciding maxim. | open (minor) |
| 9 | `web3d/strings.json` carries the texts: it needs `hint.reason.decline-rich` and the reworded `win-cheapest`, `win-over-cheaper`, `hint.rule.cheapest` from `docs/MESSAGES.md` (a JS-side reviewer owns it; I did not edit it). | open, hand-off |

## Citations

Checked against `research/07`: `bid` 1.1, `finesse` 1.3 (Zugzwang, Werner), `keep-packet` 1.2/1.5, `packet-off` 1.2 ("rasch abspielen"), `draw-trumps` 1.4, `exposed` 1.2, `lead-none` 1.2 (the recipe) support their rules. `pagat-last` is supported by 1.5 only loosely (threshold differs); noted in the basis. `cheapest` fixed as above.
