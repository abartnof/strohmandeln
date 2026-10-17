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
"⅔ point"; 107 is "35⅔"), the rest are counts, which the table writes as the language does: 1,000 in English, 1.000 in German (a count of 1 000 or more is grouped, a smaller one is not). `worlds` is the deals sampled: 1 000 for `lead-rate` and `lead-cheap`, at most 50 for `odds` (fewer when a search does not finish; `docs/OPPONENT.md`, "Hint evidence"). The move is never changed by its explanation. The text is the
English and German the table shows.

| Key | Parameters | English | Deutsch |
|---|---|---|---|
| `bid-strong-hand` | `sure_thirds`, `tarocks` | Cards sure to take a trick, worth {sure_thirds}, and {tarocks} tarocks behind them: take it up. | Karten, die sicher stechen, im Wert von {sure_thirds} und {tarocks} Tarock dahinter: aufnehmen. |
| `bid-weak-hand` | `sure_thirds`, `tarocks` | Cards sure to take a trick, worth only {sure_thirds} ({tarocks} tarocks): too few to risk taking it up. | Karten, die sicher stechen, nur im Wert von {sure_thirds} ({tarocks} Tarock): zu wenig, um aufzunehmen. |
| `bid-few-tarocks` | `sure_thirds`, `tarocks` | Cards sure to take a trick, worth {sure_thirds}, but only {tarocks} tarocks to back them. | Karten, die sicher stechen, im Wert von {sure_thirds}, aber nur {tarocks} Tarock dahinter. |
| `bid-sampled-take` | `sure_thirds`, `tarocks` | Cards sure to take a trick, worth {sure_thirds}, and {tarocks} tarocks are short of the book's measure, but in most sampled deals taking it up pays. | Karten, die sicher stechen, im Wert von {sure_thirds} und {tarocks} Tarock reichen nicht ganz für die Regel, aber in den meisten durchgespielten Austeilungen lohnt sich das Aufnehmen. |
| `bid-sampled-pass` | `sure_thirds`, `tarocks` | Cards sure to take a trick, worth {sure_thirds}, and {tarocks} tarocks meet the book's measure, but in most sampled deals the simple game is better. | Karten, die sicher stechen, im Wert von {sure_thirds} und {tarocks} Tarock genügen der Regel, aber in den meisten durchgespielten Austeilungen ist das einfache Spiel besser. |
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
| `bid` | Rule: cards sure to take a trick, worth 20 to 25 points and 6 or 7 tarocks. | Regel: Karten, die sicher stechen, im Wert von 20 bis 25 Augen und 6 bis 7 Tarock. | research/07 1.1 (M&S; Werner) |
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

The end-of-deal review (`strohmandeln_review`, `docs/PROTOCOL.md`) is two
bullets, each `{lead, key, params}`: `lead` is `count`, `decided` or `sitting`.
What to work on is the tutor's brief ("Tutor keys"), not the review. Cards in `params` are codes and `thirds` are integer thirds; the table
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
| `valat-you` | none | You took every trick: a Valat. | Sie nahmen alle Stiche: ein Valat. | RULES 10.1 |
| `valat-opp` | none | Your opponent took every trick: a Valat. | Ihr Gegner nahm alle Stiche: ein Valat. | RULES 10.1 |
| `level` | none | Nothing stood out: the cards ran evenly. | Nichts fiel auf: die Karten liefen gleichmäßig. | |

The `decided` bullet is the first of: a Valat, the Pagat ultimo, a Mond caught
by the Sküs, the richest trick the human lost (from 23 thirds, a king and a
queen), the richest trick the human won, else `level`. For a sitting,
`sitting-tally` replaces `count` and the `decided` bullet is that of the deal
that went worst (else best); it carries a `deal` number.

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

## Tutor keys

The tutor's brief (`strohmandeln_brief_of_record`, `strohmandeln_brief_sitting`,
`docs/PROTOCOL.md`, `docs/TUTOR.md` 6 and 9) is at most three bullets, and a
fourth, "Their style" (`tutor.lead.style`, `tutor.style.*`: `n` and `of` counts),
each `{lead, key, params}`: `lead` and `key` are both keys of this table (`lead` may be
empty). Cards in `params` are codes, `thirds` and `sure` integer thirds (the
table writes them as points, 107 is "35⅔"), `skill` and `skill2` a skill's slug
(the table says `tutor.name.<slug>`), `honour` is `trull` or `kings`, `suit` a
suit letter, `trick` a trick number of the deal, `n` a count.

Every text keyed `tutor.rule.*` and `tutor.nudge.*` is shown after
`tutor.lead.rule`, which is not counted in its words. After a lead that ends in a
colon the English text begins in lower case (the table does it); a lead, a
skill's name and a rule never stack a second colon. The words are the
Kibitzer's: dry, brief, kind (`docs/TABLE3D.md` 6.5). A skill is named by the act
(`tutor.name.*`), short, as the lead "Next: ..." and the praise "... – all N
chances met." need it. Terminology: "Point" is a payment, "points" (Augen) the
count; the card on a packet is a "cover" (Decke).

The brief is at most three bullets and about 45 words of body (the leads and the
skill's name besides; the longest brief that can be made is 62 words with them).
A rule is at most 15 words (a fact in its place also), a try at most 14, a nudge at
most 10, a well-went line at most 12 and an example at most 18, the parameters
counted as one word each. A test holds this in both languages and holds the keys
of the code to this table.

A rule is told only once it has passed its measurement (`docs/TUTOR.md` 11, M5;
`lessons::TOLD`); until then every rule is provisional and the skill's fact
(`tutor.fact.*`) is told in its place. The nudge keys (`tutor.nudge.*`) are shown
in the Kibitzer's tab at the first clear chance at the focus
(`strohmandeln_nudge`).

### Leads and fixed lines

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.lead.well` | none | Went well: | Gut gelaufen: | TUTOR 6 |
| `tutor.lead.next` | `skill` | Next: {skill}. | Als Nächstes: {skill}. | TUTOR 6 |
| `tutor.lead.deal` | none | This deal: | In dieser Partie: | TUTOR 6 |
| `tutor.lead.deal-n` | `deal` (the deal's number in the sitting) | Deal {deal}: | Partie {deal}: | TUTOR 6; the sitting's example |
| `tutor.lead.try` | none | To try: | Zum Versuchen: | TUTOR 6 |
| `tutor.lead.style` | none | Their style: | Spielweise des Gegners: | STYLES 5; TUTOR 6 |
| `tutor.style.took` | `n`, `of` | Your opponent has taken up {n} of {of} bids so far. | Ihr Gegner hat bisher {n} von {of} Geboten aufgenommen. | STYLES 5, 9 |
| `tutor.style.took-once` | `of` | Your opponent has taken up only once in {of} bids so far. | Ihr Gegner hat bisher in {of} Geboten nur einmal aufgenommen. | STYLES 5, 9 |
| `tutor.style.took-none` | `of` | Your opponent has not taken up in {of} bids so far. | Ihr Gegner hat bisher in {of} Geboten nie aufgenommen. | STYLES 5, 9 |
| `tutor.lead.rule` | none | If I may: | Wenn ich mir erlauben darf: | TABLE3D 6.5; shown before every `tutor.rule.*` and `tutor.nudge.*` text, not counted in its words |
| `tutor.well.mastered` | `skill` | {skill} – mastered. | {skill} – gemeistert. | TUTOR 5, 6 |
| `tutor.well.mastered-two` | `skill`, `skill2` | {skill} and {skill2} – mastered. | {skill} und {skill2} – gemeistert. | TUTOR 5, 6 |
| `tutor.well.met` | `chances`, `skill` | {skill} – all {chances} chances met. | {skill} – alle {chances} Gelegenheiten genutzt. | TUTOR 6 |
| `tutor.well.announced` | `honour` | You announced {honour} in time. | Sie sagten {honour} rechtzeitig an. | RULES 11.3 |
| `tutor.well.ultimo` | none | Your Pagat won the last trick and the ultimo. | Ihr Pagat gewann den letzten Stich und das Ultimo. | RULES 12.2 |
| `tutor.well.mond` | `trick` | In trick {trick}, your Sküs caught the Mond. | Im {trick}. Stich fing Ihr Sküs den Mond. | RULES 1.3, 1.4 |
| `tutor.none.early` | none | A deal or two more, and there will be more to say. | Noch eine Partie oder zwei, dann gibt es mehr zu sagen. | TUTOR 6 |
| `tutor.none.clean` | none | Nothing stood out to work on. | Nichts fiel auf, woran zu arbeiten wäre. | TUTOR 6 |
| `tutor.none.up` | none | Nothing stood out. Try the next opponent up. | Nichts fiel auf. Versuchen Sie den nächsthöheren Gegner. | TUTOR 6; OPPONENT |
| `tutor.short` | none | This deal gave you no real choices to review. | Diese Partie bot Ihnen kaum Entscheidungen, die sich zu besprechen lohnen. | TUTOR 6 |
| `tutor.how` | none | How is this worked out? | Wie kommt das zustande? | TUTOR 9 |
| `tutor.method` | none | Your cards were compared afterwards with what the strongest computer player would have played in your place, knowing only what you knew. Only clear differences count, and cards played after a hint or a tip do not. Each skill is judged over your recent deals, the newest counting most; the one shown is the earliest, in the order the books teach them, that you are surely short of. The rules are the advice of the standard Tarock books, checked against the strongest computer level where that has been measured; where it has not, or where the books differ, a plain fact is told instead. | Ihre Karten wurden nachträglich mit dem verglichen, was der stärkste Computerspieler an Ihrer Stelle gespielt hätte, wenn er nur wüsste, was Sie wussten. Nur deutliche Unterschiede zählen, Karten nach einem Hinweis oder Tipp nicht. Jede Fertigkeit wird über Ihre letzten Partien beurteilt, die neuesten zählen am meisten; gezeigt wird die früheste in der Reihenfolge der Bücher, bei der Sie sicher noch fehlen. Die Regeln sind der Rat der gängigen Tarockbücher, am stärksten Computerspieler geprüft, wo das gemessen ist; wo nicht, oder wo die Bücher sich unterscheiden, wird stattdessen eine schlichte Tatsache genannt. | TUTOR 6, 9; research/16 |

### Skill names

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.name.worth` | none | judging a trick's worth | Den Wert eines Stichs abschätzen | Be 147; MS "Das Nach-hinten-Spielen" |
| `tutor.name.cheap` | none | paying with the cheapest card | Mit der billigsten Karte zahlen | MS "Das Nach-hinten-Spielen"; U 75 |
| `tutor.name.off` | none | playing your covers off | Die eigenen Decken abspielen | W 134; U 71-75; Be 145, 148; MS |
| `tutor.name.run` | none | leading tarocks from a run | Tarock aus einer Reihe anspielen | U 73-74; MS |
| `tutor.name.pagat` | none | timing the Pagat | Den Pagat zur rechten Zeit spielen | U 74; MS; W 133 |
| `tutor.name.void` | none | not leading into suits your opponent lacks | Keine Farben anspielen, die dem Gegner fehlen | MS; Be 150; U 75-76 |
| `tutor.name.behind` | none | not opening your weak suits | Die eigenen schwachen Farben nicht eröffnen | W 134-135; U 72-73; Be 146-151; MS; M8 (docs/MEASUREMENTS.md) |
| `tutor.name.endgame` | none | counting to the finish | Bis zum Schluss mitzählen | U 77; Be 147, 150 |
| `tutor.name.bid` | none | taking up or passing | Aufnehmen oder passen | W 131; U 77-78; Be 145-146; MS |
| `tutor.name.announce` | none | announcing honours | Honneurs ansagen | W 133, 135; U 71; Be 146; MS |

### Rules

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.rule.worth.rich` | none | A trick with many points in it is worth winning; take it. | Ein Stich mit vielen Augen lohnt sich; holen Sie ihn. | Be 147; MS "Das Nach-hinten-Spielen" |
| `tutor.rule.worth.poor` | none | Do not win a trick of few points with a king; give a small card. | Einen Stich mit wenigen Augen nicht mit dem König holen; eine kleine Karte zugeben. | Be 147; MS "Das Nach-hinten-Spielen" |
| `tutor.rule.cheap` | none | Win with the lowest card that wins; otherwise give the lowest. | Den Stich mit der kleinsten Karte holen, die ihn gewinnt; sonst die kleinste zugeben. | MS "Das Nach-hinten-Spielen"; U 75 |
| `tutor.rule.off.pip` | none | Play your covers off; beneath them may lie tarocks or kings. | Spielen Sie Ihre Decken ab; darunter liegen vielleicht Tarock oder Könige. | W 134; U 71-75; Be 145, 148; MS |
| `tutor.rule.off.figure` | none | A queen or cavall lying as a cover should be played at once. | Eine Dame oder ein Cavall als Decke sollte gleich gespielt werden. | W 134; U 71-75; Be 145, 148; MS |
| `tutor.rule.run` | none | Lead tarocks from a solid run, whatever its height; with none, a low one. | Tarock aus geschlossener Reihe anspielen, gleich welcher Höhe; ohne Reihe ein niedriges. | U 73-74; MS |
| `tutor.rule.pagat.cash` | none | With seven tarocks or fewer, win a suit trick with the Pagat early. | Mit höchstens sieben Tarock früh einen Farbstich mit dem Pagat stechen. | U 74; MS; W 133 |
| `tutor.rule.pagat.keep` | none | With 12+ tarocks and a long suit, the Pagat may wait for the last trick. | Mit 12+ Tarock und langer Farbe kann der Pagat auf den letzten Stich warten. | U 74; MS; W 133 |
| `tutor.rule.void` | none | Think twice before leading a suit your opponent could not follow. | Überlegen Sie, ehe Sie eine Farbe anspielen, die Ihr Gegner nicht bedienen konnte. | MS; Be 150; U 75-76 |
| `tutor.rule.endgame` | none | Late in the deal, count what you still need to reach 35⅔. | Gegen Ende zählen und prüfen, was noch auf 35⅔ fehlt. | U 77; Be 147, 150 |
| `tutor.rule.bid` | none | Take up with about 25 points of sure winners and at least six tarocks. | Aufnehmen mit etwa 25 Augen sicherer Stecher und mindestens sechs Tarock. | W 131; U 77-78; Be 145-146; MS |
| `tutor.rule.announce` | none | Announce the Trull or the four kings before you play their first card. | Die Troule oder die vier Könige ansagen, ehe deren erste Karte gespielt wird. | W 133, 135; U 71; Be 146; MS |

### Facts

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.fact.worth` | none | A trick is worth the points of its two cards. | Ein Stich ist so viel wert wie die Augen seiner beiden Karten. | RULES 8.1 |
| `tutor.fact.cheap` | none | Two cards can win the same trick; the dearer one costs you points. | Zwei Karten können denselben Stich gewinnen; die teurere kostet Sie Augen. | RULES 8.1 |
| `tutor.fact.off` | none | Hidden cards lie under a cover until it is played. | Verdeckte Karten liegen unter der Decke, bis sie gespielt wird. | RULES 5.2, 6.1 |
| `tutor.fact.run` | none | Tarocks in an unbroken run win tricks one after another. | Tarock in geschlossener Reihe machen Stich um Stich. | U 73-74 |
| `tutor.fact.pagat` | none | The Pagat is the lowest tarock; played last, it wins or loses a Point. | Der Pagat ist das niedrigste Tarock; im letzten Stich gewinnt oder verliert er einen Point. | RULES 1.4, 5.3, 12.2 |
| `tutor.fact.void` | none | Out of the suit led, a player must play a tarock if able. | Fehlt die ausgespielte Farbe, muss ein Tarock gespielt werden, wenn eines da ist. | RULES 5.4 |
| `tutor.fact.behind` | none | Opening a suit where your opponent holds more points tends to cost you. | Eine Farbe zu eröffnen, in der der Gegner mehr Augen hält, kostet meist etwas. | W 134-135, MS (a suit the hands contest); M8: the cost of opening falls as your share of its points rises; against: Be 150-151 |
| `tutor.fact.endgame` | none | When every packet is turned, your opponent's cards can be worked out. | Sind alle Pakete aufgedeckt, lassen sich die Karten Ihres Gegners ausrechnen. | RULES 13.6 |
| `tutor.fact.bid` | none | The sure winners are the Sküs, the Mond, kings and guarded queens. | Sichere Stecher sind Sküs, Mond, Könige und besetzte Damen. | W 131; MS |
| `tutor.fact.announce` | none | An honour you hold but never announce is not paid. | Ein Honneur, den Sie halten, aber nicht ansagen, wird nicht bezahlt. | RULES 11.1, 11.5 |

### Tries

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.try.worth.1` | none | Before you follow, ask what the cards on the table are worth. | Fragen Sie vor dem Zugeben, was die ausliegenden Karten wert sind. | Be 147; MS "Das Nach-hinten-Spielen" |
| `tutor.try.worth.2` | none | A king can wait for a queen or a cavall. | Ein König kann auf eine Dame oder einen Cavall warten. | Be 147; MS "Das Nach-hinten-Spielen" |
| `tutor.try.cheap.1` | none | Compare your two cheapest winners before you pick. | Vergleichen Sie Ihre zwei billigsten Stecher, ehe Sie wählen. | MS "Das Nach-hinten-Spielen"; U 75 |
| `tutor.try.cheap.2` | none | When nothing wins, give the card that costs least. | Gewinnt nichts, die Karte zugeben, die am wenigsten kostet. | MS "Das Nach-hinten-Spielen"; U 75 |
| `tutor.try.off.1` | none | Look at your covers before your hand cards. | Sehen Sie zuerst Ihre Decken an, dann die Handkarten. | W 134; U 71-75; Be 145, 148; MS |
| `tutor.try.off.2` | none | Keep a cover queen if your opponent shows two low cards of its suit. | Zeigt der Gegner zwei niedrige Karten dieser Farbe, bleibt die Dame liegen. | W 134; U 71-75; Be 145, 148; MS |
| `tutor.try.run.1` | none | Look for your longest unbroken run before you lead a tarock. | Suchen Sie die längste geschlossene Reihe, ehe Sie Tarock anspielen. | U 73-74; MS |
| `tutor.try.run.2` | none | Keep your smallest tarock back, to make your opponent lead. | Das kleinste Tarock zurückhalten, damit Ihr Gegner anspielen muss. | U 73-74; MS |
| `tutor.try.pagat.1` | none | Count your tarocks before you decide about the Pagat. | Zählen Sie Ihre Tarock, ehe Sie über den Pagat entscheiden. | U 74; MS; W 133 |
| `tutor.try.pagat.2` | none | Count the tarocks behind your Pagat before you rely on it. | Zählen Sie die Tarock hinter Ihrem Pagat, ehe Sie sich auf ihn verlassen. | U 74; MS; W 133 |
| `tutor.try.void.1` | none | Note which suit your opponent could not follow. | Merken Sie sich, welche Farbe Ihr Gegner nicht bedienen konnte. | MS; Be 150; U 75-76 |
| `tutor.try.void.2` | none | Lead a tarock or another suit first. | Spielen Sie erst Tarock oder eine andere Farbe. | MS; Be 150; U 75-76 |
| `tutor.try.behind.1` | none | Lead a tarock or a safe suit and let your opponent open the rest. | Spielen Sie Tarock oder eine sichere Farbe; der Gegner eröffnet den Rest. | W 134-135; U 72-73; Be 146-151; MS; M8 (docs/MEASUREMENTS.md) |
| `tutor.try.behind.2` | none | Count the spare low cards you hold in a suit before you open it. | Zählen Sie Ihre überzähligen kleinen Karten in einer Farbe, ehe Sie sie eröffnen. | W 134-135; U 72-73; Be 146-151; MS; M8 (docs/MEASUREMENTS.md) |
| `tutor.try.endgame.1` | none | Add up your take and your opponent's; see who needs what. | Zählen Sie Ihre Stiche und die Ihres Gegners zusammen; wer braucht was? | U 77; Be 147, 150 |
| `tutor.try.endgame.2` | none | Count first; sometimes a low card wins what the king would not. | Erst zählen; manchmal gewinnt eine kleine Karte, was der König nicht gewinnt. | U 77; Be 147, 150 |
| `tutor.try.bid.1` | none | Add up your sure winners (Sküs, Mond, kings, guarded queens). | Zählen Sie Ihre sicheren Stecher (Sküs, Mond, Könige, besetzte Damen). | W 131; U 77-78; Be 145-146; MS |
| `tutor.try.bid.2` | none | Count your tarocks too; few make the game risky. | Zählen Sie auch Ihre Tarock; wenige machen das Spiel riskant. | W 131; U 77-78; Be 145-146; MS |
| `tutor.try.announce.1` | none | Press Announce before the first card of the Trull or kings. | Drücken Sie „Ansagen“ vor der ersten Karte der Troule oder der Könige. | W 133, 135; U 71; Be 146; MS |
| `tutor.try.announce.2` | none | With two Trull cards or three kings you may wait for the rest. | Mit zwei Troule-Karten oder drei Königen dürfen Sie auf die übrigen warten. | W 133, 135; U 71; Be 146; MS |

### Nudges

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.nudge.worth` | none | This trick is worth a thought. | Dieser Stich ist eine Überlegung wert. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.cheap` | none | Think which card is cheapest here. | Überlegen Sie, welche Karte hier die billigste ist. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.off` | none | A cover could be played here. | Hier ließe sich eine Decke spielen. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.run` | none | Think which tarock to lead. | Überlegen Sie, welches Tarock Sie anspielen. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.pagat` | none | Mind the Pagat here. | Achtung auf den Pagat. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.void` | none | Mind the suit your opponent lacks. | Achtung auf die Farbe, die Ihrem Gegner fehlt. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.behind` | none | Mind which suit you open. | Achten Sie darauf, welche Farbe Sie eröffnen. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.endgame` | none | Count now; the position is known. | Zählen Sie jetzt; die Lage ist bekannt. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.bid` | none | Look at your sure winners before you speak. | Prüfen Sie Ihre sicheren Stecher, ehe Sie sprechen. | TUTOR 6; shown after `tutor.lead.rule` |
| `tutor.nudge.announce` | none | You hold an honour; announce it. | Sie halten einen Honneur; sagen Sie ihn an. | TUTOR 6; shown after `tutor.lead.rule` |

### Rule notes

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.note.redpips` | `win`, `lose` (card names) | The {win} beats the {lose}: in hearts and diamonds the low numbers rank high. | {win} sticht {lose}: in Herz und Karo ist die 1 die höchste Zahl, die 4 die niedrigste. | RULES 1.5; TABLE3D 6.2.5; said once, without a lead |

### Examples

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `tutor.miss.worth.poor` | `trick`, `played`, `better` (cards), `thirds` | In trick {trick}, your {played} won only {thirds} points; the {better} would have kept it for later. | Im {trick}. Stich holte {played} nur {thirds} Augen; mit {better} wäre die Karte für einen größeren Stich geblieben. | TUTOR 2 (`worth`, greedy) |
| `tutor.miss.worth.rich` | `trick`, `led`, `better` (cards), `thirds` | In trick {trick}, your opponent led the {led}; the {better} would have taken its {thirds} points. | Im {trick}. Stich spielte Ihr Gegner {led} an; mit {better} hätten Sie die {thirds} Augen geholt. | TUTOR 2 (`worth`, timid) |
| `tutor.miss.cheap` | `trick`, `played`, `better` (cards) | In trick {trick}, the {better} would have done what the {played} did and left you better off. | Im {trick}. Stich hätte {better} dasselbe geleistet wie {played} und Sie besser gestellt. | TUTOR 2 (`cheap`) |
| `tutor.miss.off.pip` | `trick`, `better` (card) | In trick {trick}, playing the {better} from its packet would have shown you what lies beneath. | Im {trick}. Stich hätte {better} vom Paket gezeigt, was darunter liegt. | TUTOR 2 (`off`, pip) |
| `tutor.miss.off.figure` | `card`, `king` (cards), `late` (trick) | Your {card} fell under your opponent's {king} in trick {late}; play it at once next time. | Im {late}. Stich nahm Ihr Gegner {card} mit {king}; spielen Sie die Figur das nächste Mal gleich. | TUTOR 2 (`off`, figure); only where the log shows the figure lost to a higher card of its suit |
| `tutor.miss.off.figure-plain` | `trick`, `better` (card) | In trick {trick}, your {better}, lying on its packet, could have gone at once. | Im {trick}. Stich hätte {better} vom Paket gleich gehen können. | TUTOR 2 (`off`, figure) |
| `tutor.miss.run` | `trick`, `played`, `better` (cards) | In trick {trick}, you led the {played}; the {better} was the better tarock to lead. | Im {trick}. Stich spielten Sie {played} an; {better} war das bessere Tarock. | TUTOR 2 (`run`) |
| `tutor.miss.pagat.cash` | `trick`, `n` (tarocks held) | In trick {trick}, with {n} tarocks the Pagat should have been played at once. | Im {trick}. Stich hätte der Pagat mit {n} Tarock gleich gespielt werden sollen. | TUTOR 2 (`pagat`, cash) |
| `tutor.miss.pagat.keep` | `trick`, `n` (tarocks held) | In trick {trick}, with {n} tarocks the Pagat was worth keeping. | Im {trick}. Stich lohnte es mit {n} Tarock, den Pagat zu behalten. | TUTOR 2 (`pagat`, keep) |
| `tutor.miss.void` | `trick`, `played` (card), `suit` | In trick {trick}, you led the {played}, though your opponent had already failed to follow {suit}. | Im {trick}. Stich spielten Sie {played} an, obwohl Ihr Gegner {suit} schon nicht bedient hatte. | TUTOR 2 (`void`) |
| `tutor.miss.behind` | `trick`, `suit`, `better` (card) | In trick {trick}, you opened {suit}, in which your opponent held many points; the {better} led better. | Im {trick}. Stich eröffneten Sie {suit}, wo Ihr Gegner viele Augen hielt; {better} wäre die bessere Eröffnung gewesen. | TUTOR 2 (`behind`); M8 |
| `tutor.miss.endgame` | `trick`, `played`, `better` (cards) | In trick {trick}, with the position known, the {better} was the card, not the {played}. | Im {trick}. Stich war bei bekannter Lage {better} die Karte, nicht {played}. | TUTOR 2 (`endgame`) |
| `tutor.miss.bid.pass` | `sure` (thirds), `n` (tarocks held) | You passed with {sure} points of sure winners and {n} tarocks; on average, taking up paid. | Sie passten mit {sure} Augen sicherer Stecher und {n} Tarock; im Schnitt lohnte sich das Aufnehmen. | TUTOR 2 (`bid`, passed) |
| `tutor.miss.bid.take` | `sure` (thirds), `n` (tarocks held) | You took up with {sure} points of sure winners and {n} tarocks; on average, passing paid. | Sie nahmen auf mit {sure} Augen sicherer Stecher und {n} Tarock; im Schnitt lohnte sich das Passen. | TUTOR 2 (`bid`, took) |
| `tutor.miss.announce` | `card`, `honour` | You played the {card} without announcing {honour}, and lost a Point. | Sie spielten {card}, ohne {honour} anzusagen, und verloren einen Point. | RULES 11.5 |

### Keys of the page

The settings, the files and the store (`docs/TUTOR.md` 8), not the brief.

| Key | Parameters | English | Deutsch | Basis |
|---|---|---|---|---|
| `settings.tips` | none | Tips from the Kibitzer | Tipps vom Kiebitz | TUTOR 6 |
| `settings.tips.note` | none | Off: no advice in the Kibitzer's tab and no "Next" in the review; the review shows the count and what went well. | Aus: keine Hinweise im Reiter des Kiebitzes und kein "Als Nächstes" im Rückblick; der Rückblick nennt die Zählung und was gut lief. | TUTOR 6 |
| `settings.progress.import` | none | Load what the Kibitzer keeps (replaces it) | Aufzeichnungen des Kiebitzes laden (ersetzt die jetzigen) | TUTOR 8 |
| `settings.progress.confirm` | none | Clear everything the Kibitzer keeps? This cannot be undone. | Alle Aufzeichnungen des Kiebitzes löschen? Das lässt sich nicht rückgängig machen. | TUTOR 8 |
| `settings.progress.cleared` | none | What the Kibitzer kept is cleared. | Die Aufzeichnungen des Kiebitzes sind gelöscht. | TUTOR 8 |
| `settings.progress.loaded.one` | none | Loaded 1 deal. | 1 Partie geladen. | TUTOR 8 |
| `settings.progress.loaded.other` | `n` | Loaded {n} deals. | {n} Partien geladen. | TUTOR 8 |
| `error.progress-big` | none | That file is too large to be the Kibitzer's record. Nothing was changed. | Diese Datei ist zu groß für die Aufzeichnungen des Kiebitzes. Nichts wurde geändert. | TUTOR 8 |
| `error.progress-bad` | none | That file is not the Kibitzer's record of this game. Nothing was changed. | Diese Datei enthält nicht die Aufzeichnungen des Kiebitzes dieses Spiels. Nichts wurde geändert. | TUTOR 8 |
| `error.progress-version` | none | That record is from another version of the game. Nothing was changed. | Diese Aufzeichnungen sind von einer anderen Fassung des Spiels. Nichts wurde geändert. | TUTOR 8 |
| `error.progress-newer` | none | What the Kibitzer keeps here is from a newer version of the game, which this page leaves alone. | Was der Kiebitz hier aufbewahrt, stammt von einer neueren Fassung des Spiels; diese Seite lässt es unberührt. | TUTOR 8 |
| `error.progress-empty` | none | There are no deals in that file, so there is nothing to load. | In dieser Datei sind keine Partien, es gibt nichts zu laden. | TUTOR 8 |
| `error.progress-blocked` | none | This browser would not keep the record (its storage is blocked or full). | Dieser Browser nimmt die Aufzeichnungen nicht an (der Speicher ist gesperrt oder voll). | TUTOR 8 |
| `review.chip.working` | none | Working... | Einen Augenblick ... | TUTOR 10: the evidence of a deal can take a moment |
