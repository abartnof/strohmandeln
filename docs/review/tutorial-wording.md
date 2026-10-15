# Review: tutorial wording

Scope: `web3d/tutorial.md`, `web3d/tutorial.en.md` on the pacing branch, against `docs/RULES.md`, `research/14-consensus-rules.md`, `web3d/strings.json`, and main's version. Tests: `node --test web3d/test/tutorial*.test.js web3d/test/tutor.test.js` pass (36).

## Rule statements checked

- packets: 2.3, 2.4, 4.4, 4.8, 5.2. Ok.
- bidding: 3.1-3.3 (forehand speaks first, one bid each, no raising), 3.4 both pass, 1.7 values, 8.2 threshold 36, 9.1 standard 2/3/4. Ok. "Then the dealer" cut: implied by 3.2 and by "speaks first".
- the-pull: 4.3, 4.5, 4.6, 4.7. Ok.
- playing / following: 5.1, 5.2, 5.4, 5.5, 7.1. Ok. "Choose either, if both are lit" dropped; covered by "Covers count as hand cards" (5.2, 5.4 item 1).
- from-a-packet: 6.1. Ok. "The card beneath is unknown" cut: said on the first page; "last card goes into the hand" cut: said on the turning page (4.7).
- honours: 11.1-11.3, 11.5. Ok. "You may stay silent" cut: implied by "Silent, it is not paid".
- thirds: 8.1, 8.2. Ok.
- pagat-ultimo: 12.1-12.4. Ok.
- scoring: 9.2, 9.3, 11.1, 12.4, 15.5, 15.8, 15.9. Ok (36 of 70, game, honours, Pagat, slate, 2 kreuzer).
- draw: 8.4, 11.7, 14.1, 14.4, 14.6. Ok.
- touren-slate / extension: 15.2, 15.3, 15.6. Ok. Dropped "opponent has had enough, the evening ends": implied by "Both players must agree".

No rule lost by the cuts.

## Fixed

1. German "Deckkarte" -> "Decke" (and Decken); the table says "Decke von Paket n" (`card.source.packet`, `sr.packets.list`).
2. Ultimo and scoring pages stated "one Point" for the Pagat as fixed; the ultimo is a setting (12.3, 1 or 2). Now "one Point in the standard values". The game values are likewise named "standard" on the scoring page (9.1).
3. Draw page: "Honours and the Pagat are still paid" stood outside the played-draw block, so under throw-in it followed "Draws do not arise" with no meaning. Moved inside; blocks merged.
4. "Neither you nor your opponent knows the card beneath" cut (implied by "Nobody knows the hidden cards").

## Open

- Bidding page states "winner gets 2 Points" (both pass) and "3 or 4" without checking the `values` setting; only `both_pass` has if-lines. Needs a code change (values placeholders), out of scope here.
- Trick 1 leader (forehand, 5.1) is not stated; the table shows whose turn it is.
- "Cavalle" (plural) is not in `strings.json`, which has only "Cavall"; no other form is attested.
- English and German agree page for page; German terms (Aufnehmer, weiter, Troule, Honneur, Tafel, Tour, Stich, Ultimo) match `strings.json`.
