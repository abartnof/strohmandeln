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

## Review keys

The end-of-deal review (`strohmandeln_review`, `docs/PROTOCOL.md`) is up to
three bullets, each `{lead, key, params}`: `lead` is `count`, `decided` or
`try`. Cards in `params` are codes and `thirds` are integer thirds; the table
names the cards, formats the thirds (107 is "35⅔") and the honour
(`card.honour.*`). Each text is the English and German the table shows.

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `count` | `mine`, `theirs` (thirds) | You counted {mine} to your opponent's {theirs}; 35⅔ was the line. | Sie zählten {mine} gegen {theirs} Ihres Gegners; 35⅔ war die Grenze. | RULES 8.1, 8.2 |
| `sitting-tally` | `won`, `lost`, `drawn`, `net` (Points) | Deals: {won} won, {lost} lost, {drawn} drawn; {net} Points in all. | Partien: {won} gewonnen, {lost} verloren, {drawn} unentschieden; {net} Points im Ganzen. | RULES 9.2, 15.5 |
| `ultimo-won-you` | none | Your Pagat in the last trick won the ultimo. | Ihr Pagat im letzten Stich gewann das Ultimo. | RULES 12.2 |
| `ultimo-lost-you` | none | Your Pagat in the last trick was beaten, and the ultimo was paid. | Ihr Pagat im letzten Stich wurde gestochen, und das Ultimo wurde bezahlt. | RULES 12.2 |
| `ultimo-won-opp` | none | Your opponent's Pagat won the last trick and the ultimo. | Der Pagat Ihres Gegners gewann den letzten Stich und das Ultimo. | RULES 12.2 |
| `ultimo-lost-opp` | none | Your opponent's Pagat was beaten in the last trick: the ultimo is yours. | Der Pagat Ihres Gegners wurde im letzten Stich gestochen: das Ultimo gehört Ihnen. | RULES 12.2 |
| `mond-caught` | `trick` | In trick {trick} your Sküs caught the Mond. | Im {trick}. Stich fing Ihr Sküs den Mond. | RULES 1.3, 1.4 |
| `mond-lost` | `trick` | In trick {trick} your Mond fell to the Sküs. | Im {trick}. Stich fiel Ihr Mond dem Sküs zu. | RULES 1.3, 1.4 |
| `trick-lost-rich` | `trick`, `mine`, `theirs` (cards), `thirds` | Trick {trick} was the rich one you lost: {mine} against {theirs}, {thirds} between them. | Der {trick}. Stich war der reiche, den Sie abgaben: {mine} gegen {theirs}, zusammen {thirds}. | RULES 8.1 |
| `trick-won-rich` | `trick`, `mine`, `theirs` (cards), `thirds` | Trick {trick} was your best: {mine} beat {theirs}, {thirds} between them. | Der {trick}. Stich war Ihr bester: {mine} schlug {theirs}, zusammen {thirds}. | RULES 8.1 |
| `level` | none | Nothing stood out: the cards ran evenly. | Nichts fiel auf: die Karten liefen gleichmäßig. | |
| `try-pagat` | none | If I may: the Pagat is the lowest tarock; kept for the last trick it wins or loses a Point. | Wenn ich mir erlauben darf: der Pagat ist das niedrigste Tarock; für den letzten Stich aufgehoben gewinnt oder verliert er einen Point. | RULES 1.4, 5.3, 12.2 |
| `try-announce` | `honour` | If I may: you held {honour} and did not announce it; announced, it is paid. | Wenn ich mir erlauben darf: Sie hielten {honour} und sagten nicht an; angesagt, wird er bezahlt. | RULES 11.1, 11.5 |
| `try-count` | none | If I may: count the tarocks out as they fall; there are 22, the Sküs among them. | Wenn ich mir erlauben darf: Zählen Sie die Tarock mit, die fallen; es sind 22, der Sküs darunter. | RULES 1.3, 13.6 |

The `decided` bullet is the first of: the Pagat ultimo, a Mond caught by the
Sküs, the richest trick the human lost (from 23 thirds, a king and a queen),
the richest trick the human won, else `level`. The `try` bullet is the first
of: the Pagat led before the last trick, an honour held in the hand and not
announced, else `try-count`. For a sitting, `sitting-tally` replaces `count`,
the `decided` bullet is that of the deal that went worst (else best) and
carries a `deal` number, and the `try` bullet is the most frequent one.

## Explanation keys

The line under the prompt after a trick (`strohmandeln_explain`,
`docs/PROTOCOL.md`; the setting "Explanations", `docs/TABLE3D.md` 6.3.3) is one
bullet `{lead: "explain", key, params}`. `suit` is a suit letter. The
deductions rest on the rule of following (RULES 5.4) and on the playable set
being the hand and the covers (RULES 5.2): a face-down packet card may still be
of the suit, which is why the texts say "in hand or showing".

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `explain-trump-you` | `suit` | Trump: you had no {suit} in hand or showing, so a tarock was needed. | Tarock: Sie hatten kein {suit} in der Hand oder offen, also musste gestochen werden. | RULES 5.4 |
| `explain-trump-opp` | `suit` | Trump: your opponent has no {suit} in hand or showing. | Tarock: Ihr Gegner hat kein {suit} in der Hand oder offen. | RULES 5.4 |
| `explain-void-you` | `suit` | You had neither {suit} nor a tarock in hand or showing, so any card went. | Sie hatten weder {suit} noch Tarock in der Hand oder offen, also ging jede Karte. | RULES 5.4 |
| `explain-void-opp` | `suit` | Your opponent has neither {suit} nor a tarock in hand or showing. | Ihr Gegner hat weder {suit} noch Tarock in der Hand oder offen. | RULES 5.4 |
| `explain-notarock-you` | none | You had no tarock in hand or showing, so any card went. | Sie hatten kein Tarock in der Hand oder offen, also ging jede Karte. | RULES 5.4 |
| `explain-notarock-opp` | none | Your opponent has no tarock in hand or showing. | Ihr Gegner hat kein Tarock in der Hand oder offen. | RULES 5.4 |
| `explain-won-you` | `winner`, `loser` (cards) | Your trick: {winner} beat {loser}. | Ihr Stich: {winner} schlug {loser}. | RULES 5.4, 5.5 |
| `explain-won-opp` | `winner`, `loser` (cards) | Your opponent's trick: {winner} beat {loser}. | Der Stich Ihres Gegners: {winner} schlug {loser}. | RULES 5.4, 5.5 |
