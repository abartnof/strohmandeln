# Message keys

The engine (`strohmandeln-core`) emits no prose. Refusals carry a stable key
and parameters; the table translates them (English default, German). Cards are
codes (RULES 1.2), suits are the letters `h d c s`, seats are `0` or `1`,
honours are `trull` or `kings`. `Display` on `Refusal` gives an English
rendering for tests and the terminal client only.

A refusal serialises as `{"key": ..., <parameters>}`; absent parameters are
omitted.

| Key | Parameters | When |
|---|---|---|
| `not-your-turn` | `speaker` (seat whose turn it is) | an action out of turn (3.1, 5.1) |
| `wrong-phase` | none | a bid during play, a card before the bidding ends, an extension answer during a deal |
| `finished` | none | any action after the deal is over |
| `not-playable` | `card` | the card is not in the hand or a cover (5.2) |
| `must-follow-suit` | `card`, `led` (`h d c s` or `trump`), `allowed` (cards) | 5.4 rule 1 |
| `must-trump` | `card`, `allowed` (tarocks) | 5.4 rule 2 |
| `not-held` | `honour`, `missing` (cards) | a false announcement (11.4) |
| `already-announced` | `honour` | 11.3 |
| `honour-forfeited` | `honour` | announcing after playing a card of it unannounced (11.5) |
| `duplicate-announcement` | `honour` | the same honour twice in one play action |

Starting or replaying a sitting can fail too; the error has the same form
(`{"key": ..., <parameters>}`, `SetupError` in the code).

| Key | Parameters | When |
|---|---|---|
| `bad-touren` | `touren` | a sitting of other than 3 or 6 Touren (15.2) |
| `bad-ultimo` | `ultimo` | an ultimo setting other than 1 or 2 (12.3) |
| `record-version` | `found`, `expected` | a record of another `RECORD_VERSION` |
| `replay-refused` | `index`, `refusal` (a refusal as above) | a recorded move refused on replay |

A record writes its `seed` as a decimal string, since a JavaScript number cannot
hold every 64-bit seed (a plain number is read too).

Events (RULES 16) carry codes and ids only and need no keys of their own; the
table speaks to their `type`.
