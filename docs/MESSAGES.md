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
table speaks to their `type`. Two events are not sent to the seat that does not
own them: `dealt` (the other hand) and `honour_forfeited` (RULES 13.2, 11.5).

## Hint reasons

The tutor's hint (`strohmandeln_hint`, `docs/PROTOCOL.md`) carries one key and
an object of parameters (none yet). The table translates the key; this is the
English and German text each is shown with.

| Key | English | Deutsch |
|---|---|---|
| `bid-strong-hand` | Sure winners and the tarocks to back them: take up the 4. | Genug sichere Stiche und Tarock dazu: die Partie aufnehmen. |
| `bid-weak-hand` | Too few sure winners to risk the 4. | Zu wenige sichere Stiche für das Aufnehmen. |
| `bid-few-tarocks` | Honours, but too few tarocks. | Honneurs, aber zu wenig Tarock. |
| `bid-sampled-take` | In most sampled deals taking up pays. | In den meisten durchgespielten Austeilungen lohnt sich das Aufnehmen. |
| `bid-sampled-pass` | In most sampled deals the simple game is better. | In den meisten durchgespielten Austeilungen ist das einfache Spiel besser. |
| `take-rich-trick` | Win this trick: it is rich in points. | Diesen Stich holen: er ist augenreich. |
| `take-trick` | Win it, and cheaply. | Holen, und zwar billig. |
| `finesse` | Do not spend an honour on a poor trick; let him lead. | Keinen Honneur auf einen armen Stich: er soll anspielen (Impassiren). |
| `follow-cheaply` | Lose with the least, or win with the cheapest winner. | Mit dem Kleinsten abgeben oder mit dem billigsten Stecher holen. |
| `play-packet-off` | Play a face-up packet card to see what lies beneath. | Das offene Paketblatt spielen, um das darunter zu sehen. |
| `play-exposed-figure` | An exposed queen or cavall goes at once. | Eine offene Dame oder ein Cavall geht gleich. |
| `lead-where-he-shows-none` | Lead a suit in which he shows no packet card. | Eine Farbe anspielen, in der er kein offenes Paketblatt zeigt. |
| `dont-lead-split-suit` | The suit is evenly split: make him open it. | Die Farbe ist gleich verteilt: er soll sie eröffnen. |
| `keep-packet` | Keep the packet: his Pagat ultimo threatens. | Das Paket behalten: sein Pagat ultimo droht. |
| `hold-pagat` | Hold the Pagat for the last trick. | Den Pagat für den letzten Stich behalten. |
| `cash-king` | The king is safe to cash. | Der König ist sicher zu spielen. |
| `draw-trumps` | Strong in tarocks: draw his trumps. | Stark im Tarock: seine Tarock ziehen. |
| `lead-solid-run` | Lead from a solid run of tarocks. | Aus einer festen Tarockfolge anspielen. |
| `keep-tarocks` | Too few tarocks to lead them. | Zu wenig Tarock, um sie anzuspielen. |
| `hold-high-trump` | Do not lead high tarocks into a lost cause. | Hohe Tarock nicht in einen verlorenen Stich anspielen. |
| `lead-safe` | In the sampled deals this lead nearly always wins. | In den durchgespielten Austeilungen gewinnt dieses Anspiel fast immer. |
| `lead-sampled` | In the sampled deals this lead often wins. | In den durchgespielten Austeilungen gewinnt dieses Anspiel oft. |
| `lead-low` | In the sampled deals this lead rarely wins; it costs least. | In den durchgespielten Austeilungen gewinnt dieses Anspiel selten; es kostet am wenigsten. |
| `search-best` | The search found it best over the sampled deals. | Die Suche hat es über die durchgespielten Austeilungen als das Beste gefunden. |
| `decline-extension` | The sitting is not extended. | Die Runde wird nicht verlängert. |
| `announce-honour` | Announce the Troule or the four kings. | Troule oder die vier Könige ansagen. |
