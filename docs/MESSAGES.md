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

The tutor's hint (`strohmandeln_hint`, `docs/PROTOCOL.md`) says which card and
why. It carries a reason key with parameters, optionally a rule key (a second
clause naming the maxim the move follows) and, at the top rung, `odds`. The
table translates all three. A parameter naming a card is a code (the table
names it), one ending `_thirds` is thirds of a point told with its unit ("4⅔ points",
"⅔ point"; 107 is "35⅔"), the rest are counts. The move is never changed by its explanation. The text is the
English and German the table shows.

| Key | Parameters | English | Deutsch |
|---|---|---|---|
| `bid-strong-hand` | `sure_thirds`, `tarocks` | Sure winners worth {sure_thirds} and {tarocks} tarocks behind them: take up the 4. | Sichere Stecher im Wert von {sure_thirds} und {tarocks} Tarock dahinter: die Partie aufnehmen. |
| `bid-weak-hand` | `sure_thirds`, `tarocks` | Sure winners worth only {sure_thirds} ({tarocks} tarocks): too few to risk the 4. | Sichere Stecher nur im Wert von {sure_thirds} ({tarocks} Tarock): zu wenig für das Aufnehmen. |
| `bid-few-tarocks` | `sure_thirds`, `tarocks` | Sure winners worth {sure_thirds}, but only {tarocks} tarocks to back them. | Sichere Stecher im Wert von {sure_thirds}, aber nur {tarocks} Tarock dahinter. |
| `bid-sampled-take` | `sure_thirds`, `tarocks` | Sure winners worth {sure_thirds} and {tarocks} tarocks are short of the book's measure, but in most sampled deals taking up pays. | Sichere Stecher im Wert von {sure_thirds} und {tarocks} Tarock reichen nicht ganz für die Regel, aber in den meisten durchgespielten Austeilungen lohnt sich das Aufnehmen. |
| `bid-sampled-pass` | `sure_thirds`, `tarocks` | Sure winners worth {sure_thirds} and {tarocks} tarocks meet the book's measure, but in most sampled deals the simple game is better. | Sichere Stecher im Wert von {sure_thirds} und {tarocks} Tarock genügen der Regel, aber in den meisten durchgespielten Austeilungen ist das einfache Spiel besser. |
| `only-card` | `card` | {card} is your only legal card. | {card} ist Ihre einzige erlaubte Karte. |
| `hold-pagat` | `card`, `tarocks` | Play {card}, not the Pagat: with {tarocks} tarocks you can bring the Pagat home in the last trick. | {card} spielen, nicht den Pagat: mit {tarocks} Tarock lässt sich der Pagat im letzten Stich heimbringen. |
| `keep-packet` | `card` | His Pagat ultimo threatens: play {card} and keep the fullest packet unplayed. | Sein Pagat ultimo droht: {card} spielen und das vollste Paket liegen lassen. |
| `win-cheapest` | `card`, `led`, `alt`, `trick_thirds` | {card} beats {led} and takes {trick_thirds}; it is the cheapest winner, and {alt} also wins but costs more to spend. | {card} sticht {led} und holt {trick_thirds}; es ist der billigste Stecher, und {alt} würde auch stechen, ist aber teurer. |
| `win-over-cheaper` | `card`, `led`, `alt`, `trick_thirds` | {card} rather than {alt}: both beat {led} for {trick_thirds}, but {card} is the better play here. | {card} statt {alt}: beide stechen {led} für {trick_thirds}, aber {card} ist hier das bessere Spiel. |
| `win-sole` | `card`, `led`, `trick_thirds` | Only {card} beats {led}: it takes {trick_thirds}. | Nur {card} sticht {led}: es holt {trick_thirds}. |
| `decline-poor` | `card`, `led`, `alt`, `trick_thirds`, `given_thirds` | {led} is a poor trick (only {trick_thirds} even with {alt}): give up {card}, which costs {given_thirds}, and let him lead. | {led} ist ein armer Stich (selbst mit {alt} nur {trick_thirds}): {card} abgeben, das {given_thirds} kostet, und ihn anspielen lassen. |
| `decline-rich` | `card`, `led`, `alt`, `trick_thirds`, `given_thirds` | {led} is worth taking ({trick_thirds} with {alt}), but {card} ({given_thirds}) is the better play here: let him lead. | {led} lohnt sich ({trick_thirds} mit {alt}), aber {card} ({given_thirds}) ist hier das bessere Spiel: ihn anspielen lassen. |
| `lose-cheapest` | `card`, `led`, `given_thirds` | Nothing beats {led}: give up {card}, the cheapest card, for {given_thirds}. | Nichts sticht {led}: {card} abgeben, die billigste Karte, für {given_thirds}. |
| `lose-other` | `card`, `led`, `alt`, `given_thirds` | Nothing beats {led}: {card} goes for {given_thirds}; {alt} would be cheaper. | Nichts sticht {led}: {card} geht für {given_thirds}; {alt} wäre billiger. |
| `lose-packet` | `card`, `led`, `given_thirds` | Nothing beats {led}: give up the face-up packet card {card} ({given_thirds}) to see what lies beneath. | Nichts sticht {led}: das offene Paketblatt {card} ({given_thirds}) abgeben, um das darunter zu sehen. |
| `lead-king` | `card`, `out` | {card} cannot be beaten in its suit: only a tarock takes it, and {out} tarocks are unseen. | {card} ist in seiner Farbe nicht zu schlagen: nur ein Tarock holt es, und {out} Tarock sind unbekannt. |
| `lead-draw` | `card`, `mine`, `out` | You hold {mine} tarocks to {out} unseen: lead {card} to draw his trumps. | Sie halten {mine} Tarock gegen {out} unbekannte: {card} anspielen, um seine Tarock zu ziehen. |
| `lead-exposed` | `card` | An exposed queen or cavall goes at once: lead {card}. | Eine offene Dame oder ein Cavall geht gleich: {card} anspielen. |
| `lead-his-none` | `card` | He shows no packet card in this suit: lead {card}. | Er zeigt in dieser Farbe kein offenes Paketblatt: {card} anspielen. |
| `lead-trump-void` | `card`, `out` | He has shown no tarock in hand, so only one from a packet could answer {card}; {out} tarocks are unseen. | Er hat kein Tarock in der Hand gezeigt, also könnte nur eines aus einem Paket {card} beantworten; {out} Tarock sind unbekannt. |
| `lead-rate` | `card`, `wins`, `worlds` | {card} takes the trick in {wins} of {worlds} sampled deals. | {card} gewinnt den Stich in {wins} von {worlds} durchgespielten Austeilungen. |
| `lead-cheap` | `card`, `wins`, `worlds`, `cost_thirds` | {card} usually loses the trick (it wins in {wins} of {worlds} sampled deals), but it costs least: {cost_thirds}. | {card} verliert den Stich meist (es gewinnt in {wins} von {worlds} durchgespielten Austeilungen), kostet aber am wenigsten: {cost_thirds}. |
| `lead-cost` | `card`, `cost_thirds` | If he takes the trick, {card} costs only {cost_thirds}. | Holt er den Stich, kostet {card} nur {cost_thirds}. |
| `decline-extension` | none | The sitting is not extended. | Die Runde wird nicht verlängert. |
| `announce-honour` | none | Announce the Troule or the four kings. | Troule oder die vier Könige ansagen. |

### Hint rules

The second clause (`rule`), when the move follows a maxim of the period
strategy notes. `research/07` is `research/07-strategy-stats-digital.md`.

| Rule | English | Deutsch | Basis |
|---|---|---|---|
| `bid` | Rule: sure winners worth 20 to 25 points and 6 or 7 tarocks. | Regel: sichere Stecher im Wert von 20 bis 25 Augen und 6 bis 7 Tarock. | research/07 1.1 (M&S; Werner) |
| `cheapest` | Rule: do not spend a king where a small card serves. | Regel: keinen König, wo eine kleine Karte genügt. | research/07 1.3 (nach hinten spielen: "den König schinden und ein Skartindel zugeben"); 1.2 (following suit with junk) |
| `finesse` | Rule: do not win a poor trick with an honour; the winner must lead, and whoever opens a contested suit ends worse in it (Impassiren). | Regel: einen armen Stich nicht mit einem Honneur holen; wer gewinnt, muss anspielen, und wer eine umkämpfte Farbe eröffnet, steht darin am Ende schlechter (Impassiren). | research/07 1.3 (nach hinten spielen; Zugzwang; Werner) |
| `pagat-last` | Rule: with many tarocks, hold the Pagat for the last trick (ultimo). | Regel: mit vielen Tarock den Pagat für den letzten Stich behalten (Ultimo). | research/07 1.4, 1.5 (the source's threshold is more than 11 tarocks for a silent ultimo; the hint holds from 8); RULES 12.2 |
| `keep-packet` | Rule: keep a packet unplayed while his Pagat ultimo threatens. | Regel: ein Paket liegen lassen, solange sein Pagat ultimo droht. | research/07 1.2 (Werner), 1.5 |
| `packet-off` | Rule: play the face-up packet cards off, to see what lies beneath. | Regel: die offenen Paketblätter abspielen, um das darunter zu sehen. | research/07 1.2 |
| `draw-trumps` | Rule: the player strong in tarocks leads them first, from a solid run. | Regel: wer im Tarock stark ist, tarockiert zuerst, aus einer geschlossenen Reihe. | research/07 1.4 |
| `exposed` | Rule: an exposed queen or cavall is played at once. | Regel: eine offene Dame oder ein Cavall wird sofort gespielt. | research/07 1.2 |
| `lead-none` | Rule: lead a suit in which he has no face-up card. | Regel: eine Farbe anspielen, in der er kein offenes Paketblatt hat. | research/07 1.2 |

### Hint odds

`odds` is `{alt, better, worlds, gain_thirds}`: the hinted card against the
best other legal card `alt`, over `worlds` deals sampled from what the seat
cannot see and solved with every card known. `better` is the deals in which the
hinted card does strictly better, `gain_thirds` the average margin it gains.
It comes from a fresh sample and is given only when it agrees with the choice
(a gain of at least a third of a point); otherwise `odds` is `null`. Shown as
"Against {alt}: ahead in {better} of {worlds} sampled deals, by {gain_thirds} on average." / "Gegen {alt}: vorn in {better} von {worlds} durchgespielten Austeilungen, im Schnitt um {gain_thirds}.". The table leads with the card:
"Play {card}." / "{card} spielen."

## Review keys

The end-of-deal review (`strohmandeln_review`, `docs/PROTOCOL.md`) is up to
three bullets, each `{lead, key, params}`: `lead` is `count`, `decided`,
`try` or `sitting`. Cards in `params` are codes and `thirds` are integer thirds; the table
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
| `try-announce` | `honour` | If I may: you held {honour} and did not announce it; announced, the honour is paid. | Wenn ich mir erlauben darf: Sie hielten {honour} und sagten nicht an; angesagt, wird der Honneur bezahlt. | RULES 11.1, 11.5 |
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
