# Strohmandeln: the rules as implemented

> Status: **adopted**, 2026-10-14. The engine (`strohmandeln-core`) is built to
> this document. Any change to it starts as a change to the tests.

Two-hand Tarock with straw men (Strohmann-Tarok, Strohmanndl), modern form:
15 hand cards plus three 4-card packets ("straw men") each, 27 tricks, 36 of 70
to win.

Citations use the research ids: `[14 rule N]` is a row of the consensus table in
`research/14-consensus-rules.md`; `[14 S]` its Settings section; `[14 D]` its
Decisions and session sections; `[02 An]` is ambiguity n of
`research/02-modern-rules.md` section 2; `[03-Sn]` a source of
`research/03-period-rulebooks.md` (S12 Werner, S13 Bermann, S14 Ulmann);
`[13-Sn]` and `[13 §n]` are from `research/13-sessions-and-scoring.md`.
`[D]` marks an engine decision where no source settles the point; each is listed
again in section 17.

Terms: **forehand** (Vorhand) is the non-dealer; **dealer** (Hinterhand) deals.
**Declarer** (Aufnehmer) is the player who said "Ich nehme auf"; the other is the
**defender**. **Packet** (Talon-Häufchen) is a 4-card face-down pile; its
**cover** is the face-up card on top of it. **Hand** is a player's hand cards,
including cards pulled from packets. **Playable set** is the hand plus the
player's covers.

## Authority and settings

The rules are the consensus of Werner (1891), Ulmann (1888/99), Bermann (1894),
Mayr and Sedlaczek (2007), Furr (2009) and Dummett's digest (1980) [14 Method].
The game has **three settings**, each a difference of practice that persisted
from the period to the modern sources [14 S]. Everything else is one rule.

| Setting | Values | Default | Clause |
|---|---|---|---|
| `both_pass` | `play` (simple game), `throw_in` | `play` | 3.4 |
| `values` | `2/3/4`, `1/2/3`, `1/2/4` (simple / declarer wins / declarer loses) | `2/3/4` | 9.1 |
| `ultimo` | `1`, `2` Points | `1` | 12.3 |

[14 rules 7, 19, 21][14 S]. The sitting's length (3 or 6 Touren) is chosen at
the start of each sitting and is not a rules setting (15.2).

## 1. The pack

1.1 One pack of 54 cards: 22 tarocks and four suits of 8 cards [14 rule 1][02 §1].

1.2 **Card codes.** A code is a case-sensitive string:

| Cards | Code | Count |
|---|---|---|
| Tarocks I to XXI | `T1` ... `T21` | 21 |
| The Sküs | `SK` | 1 |
| Suit cards | rank code, then lowercase suit code: `Kh`, `Qd`, `Cc`, `Js`, `10s`, `7c`, `1h`, `4d` | 32 |

Suit codes: `h` hearts, `d` diamonds, `c` clubs, `s` spades. Rank codes: `K`
king, `Q` queen, `C` cavall (the knight), `J` jack; `10 9 8 7` in clubs and
spades; `1 2 3 4` in hearts and diamonds. `SK` (uppercase S) is the Sküs and `Ks`
(lowercase s) is the king of spades; a code ends in a lowercase suit letter if
and only if it is a suit card, so no parser needs context. The wasm protocol uses
these strings verbatim. [D: the pack is [14 rule 1]; the codes are an engineering
choice.]

1.3 **Tarocks** (trumps) are the 22 cards `SK`, `T1` ... `T21`. The Sküs is a
tarock, not an excuse: it is the highest trump and is played and won like any
other trump [02-S1 via 02 §1 "Skis highest trump"][14 rule 1].

1.4 **Trump order**, high to low: `SK`, `T21`, `T20`, ..., `T2`, `T1`.

1.5 **Suit order**, high to low:
- clubs and spades: `K Q C J 10 9 8 7`;
- hearts and diamonds: `K Q C J 1 2 3 4` (the 1 is the highest pip, the 4 the
  lowest) [14 rule 1][08-S9][17-R1 Bermann p. 17 "sticht unter den leeren
  Blättern die arithmetisch niederere Karte, also das Aß die Zwei"][17].

1.6 **Kings** are `Kh Kd Kc Ks`. The **Troule** (trull) is `SK`, `T21`, `T1`
[03-S12].

1.7 **Card values** [14 rule 15][02 §1]:

| Value | Cards |
|---|---|
| 5 | the four kings, `SK`, `T21`, `T1` |
| 4 | the four queens |
| 3 | the four cavalls |
| 2 | the four jacks |
| 1 | every other card (the 16 pips and `T2` ... `T20`) |

The values sum to 106 (suits 4 x 18 = 72, `SK`+`T21`+`T1` = 15, `T2`-`T20` = 19).

## 2. The deal

2.1 **First dealer** is decided by lot; the engine draws it from the sitting's
seeded random source. The deal then alternates every deal [14 rule 2][03-S12].

2.2 The pack is shuffled into an array `deck[0..53]`; index 0 is the first card
dealt.

2.3 **Hand cards.** Five cards at a time, alternately, three times: forehand
`deck[0..4]`, dealer `deck[5..9]`, forehand `deck[10..14]`, dealer
`deck[15..19]`, forehand `deck[20..24]`, dealer `deck[25..29]`. Each has 15 hand
cards [14 rule 3][03-S12][03-S14].

2.4 **Packets.** The other 24 cards `deck[30..53]` form six packets of four, face
down: packet `i` (0 to 5) is `deck[30+4i .. 33+4i]`. Packets 0, 2, 4 go to
forehand and 1, 3, 5 to the dealer, forehand's first [14 rule 3][03-S12 "der
Gegner das erste, der Geber das zweite"][03-S14]. A player's three packets are
numbered 1, 2, 3 in the order received; this is their left-to-right order on that
player's side. Within a packet, cards are listed top first (the first index of
its slice is turned first). [D: the sources give "alternately" and "left to
right", not the form of a packet.]

2.5 The packets are on the table from the deal in every setting; play does not
depend on when they are laid [14 rule 8].

2.6 **Void deal.** After the deal and before any bid, each player's 15 hand cards
are inspected. If either player's 15 cards include no tarock (the Sküs counts as
one), the deal is annulled: no bid, no packet turned, nothing scored, no
penalty. **The same dealer redeals**, with the same pending doubling. A void deal
does not count as a deal of the Tour (15.3). Annulment is compulsory [14 rule 4]
[03-S13][03-S14][02 §1] (Mayr and Sedlaczek make it optional [02-S2]). The
inspection sees hand cards only, never packet cards.

2.7 There is no misdeal and no misdeal penalty: the engine always deals
correctly [14 S (c)].

## 3. Bidding

3.1 After the void check forehand speaks first: `TAKE` ("Ich nehme auf") or
`PASS` ("Weiter", "Ich passe") [14 rule 5][03-S12][03-S14][02 §1].

3.2 If forehand passes, the dealer speaks: `TAKE` or `PASS` [14 rule 5].

3.3 A player who takes up is the declarer. `TAKE` is the only positive bid; there
is no Kontra and no raising [14 rule 6][02 A18]. Each player speaks at most once.

3.4 **Both pass** [14 rule 7][14 S]:
- `both_pass = play` (default): no declarer; the deal is played as a **simple
  game** (stille or einfache Partie) [03-S12][02-S2].
- `both_pass = throw_in`: the deal is thrown in. Nothing is scored and no packet
  is turned. It counts as a deal of the Tour (15.3), the deal passes to the other
  player, and a pending doubling carries to the next deal (14.4)
  [03-S14 "die Vorhand hat die nächste Donne"][D for the Tour count].

3.5 Bids and passes are public events (16).

## 4. The opening reveal

4.1 The reveal follows the bidding (any outcome that is played) and precedes the
first lead [03-S12 "Nachdem es entschieden ist ..."].

4.2 **Order.** Forehand reveals his three packets first, then the dealer reveals
his three. [D: the sources say both players proceed alike [03-S14] and not who
starts. The two reveals are independent, so the order changes no result.]

4.3 **Left to right.** A player's packets are turned in the order 1, 2, 3 (2.4),
each packet completed (its chain, 4.5) before the next begins [02 A7][02-S2]
[03-S12 "der Reihe nach"].

4.4 **Turning.** The top card of a packet is turned face up and is seen by both
players [03-S12][03-S14][02 A9]. Every turned card is public.

4.5 **The chain.** If the turned card is a tarock or a king it is **pulled**: it
is taken into the owner's hand, and the next card of the packet is turned and
tested the same way. A turned card that is neither a tarock nor a king stays face
up on the packet as its **cover**, and the chain ends [14 rule 9][03-S12][03-S14]
[02 A6]. A player whose packets hold only tarocks and kings takes all twelve
cards; one whose three covers are all low suit cards takes none [03-S12].

4.6 **Showing.** Every pulled card is shown to the opponent before it enters the
hand. The engine emits the turn and the pull as separate events so the table can
show the card, pause, then move it [14 rule 9][02 A7, A9][03-S12 "nachdem er sie
dem Gegner gezeigt hat"]. A pulled card is a public hand card (13.4).

4.7 **The last card of a packet** (the 4th) is taken into the hand whatever it is
(a suit card too), after the opponent has seen it. The packet is then empty, with
no cover [14 rule 10].

4.8 **Invariant.** After the reveal and after every replacement (6.1) each
non-empty packet has exactly one face-up cover, and no cover is a tarock or a
king. Covers are low suit cards: queens, cavalls, jacks and pips.

## 5. Legal plays

5.1 **Tricks.** There are 27 tricks. The leader plays one card and the other
player follows with one. Forehand leads the first trick, whoever the declarer is;
thereafter the winner of the last trick leads [14 rule 14][03-S12][03-S14]
[02 A10].

5.2 **Playable set.** A player may play any card of his hand or any of his
covers. A cover is part of its owner's hand for leading, following and trumping
[14 rule 11][02 A1][03-S12 "zum Stechen, Ausspielen und Zugeben"]. Face-down
packet cards are not playable.

5.3 **Leading.** The leader may lead any card of his playable set. The suit led
is the suit of the card; a tarock leads trump.

5.4 **Following** [14 rule 12][02 A2][03-S12][03-S13]. Let `S` be the follower's
playable set and `L` the suit led:
1. if `S` holds a card of suit `L` (a tarock, when trump was led) the follower
   must play one of those cards. The choice among them, hand cards and covers, is
   free: a hand card may be played while a cover of the suit is also available,
   and the reverse;
2. else, if `L` is a suit and `S` holds a tarock, the follower must play a tarock
   (trump if void), from hand or cover, freely chosen;
3. else the follower may play any card of `S`.

5.5 **No obligation to win.** There is no Stichzwang: a follower with the suit
need not play a higher card, and a follower who must trump need not play a higher
tarock than one already played [14 rule 12][02 A18 context].

5.6 Obligations are measured on the playable set only; a card is never forced out
of a hidden packet. A player whose hand is empty plays covers. At trick 27 each
player's single remaining card is in the playable set (by 4.8 every non-empty
packet has a cover; 27 cards make 27 tricks) [02 A1, A8].

5.7 `legal_plays(state, player)` returns the playable cards allowed by 5.3 and
5.4, each with its **source** (`hand` or `packet n`, n = 1 to 3).

## 6. Replacing packet cards

6.1 When a trick is complete its two cards are gathered face down by the winner
(7.2). **Then, for each player who played a cover in that trick, the card beneath
the vacated cover is turned** and treated as in 4.4 to 4.7: a tarock or king is
shown, pulled, and the next card turned; a low suit card becomes the new cover;
the last card of a packet is pulled whatever it is. If nothing remains in the
packet, nothing happens [14 rule 13][02 A5][03-S12 "sogleich nach Umlegung des
betreffenden Stiches"][02-S2 "sofort nach Einziehen"].

6.2 The replacement is made after the trick is gathered and before the winner
leads. Within the trick a vacated cover is not refilled, and the follower cannot
use a card that the leader's replacement would reveal [02 A5].

6.3 **Both used a cover in the same trick:** the leader's packet is replaced
first, its whole chain completed, then the follower's. [D: no source orders them
[02 A5]. The chains involve different packets and different hands, so the order
changes no result; it only fixes the event sequence.]

6.4 A card pulled by a replacement is in the owner's hand from then on: it may be
played in the next trick and counts toward the honours (11.2).

## 7. The trick winner

7.1 The winner is the player of the highest trump in the trick, if any trump was
played (order 1.4); otherwise the player of the highest card of the suit led
(order 1.5). A card of another suit that is not a trump never wins. A trick has
two cards, so there is no tie.

7.2 The winner takes both cards face down into his **take**, and leads next
[14 rule 14]. The takes decide the count (8).

## 8. Counting

8.1 **Exact thirds.** A card counts `3 x value - 2` thirds of a point (values
1.7): value 5 is 13 thirds, 4 is 10, 3 is 7, 2 is 4, 1 is 1. This is the
traditional "count in threes, deduct 2 from each group of three" [14 rule 15]
[02 A17], kept exact: no group and no leftover is rounded. A take counts the sum
of its cards. The two takes always sum to 210 thirds (70 points).

8.2 The **threshold** is 36 points, that is at least 35 2/3, that is **107
thirds**. A take of 106 thirds (35 1/3) does not win [14 rule 16][03-S14 "36
(35 2/3)"][02-S2 "35 Punkte und 2 Blatt"].

8.3 **Declared game:** the declarer wins with 107 thirds or more, otherwise he
loses. There is no draw [14 rule 17][03-S12][03-S14 "der Aufnehmer ... auch mit
35 1/3 verliert"].

8.4 **Simple game** (both passed, `play`): a player with 107 thirds or more wins.
If neither take reaches 107 the game is a **draw** (undecided): 105:105, or
106:104 either way [14 rule 17][03-S12][03-S14 "35 resp. 35 1/3 gegen 34 2/3 ...
unentschieden"][02 A12].

8.5 Counts are held as integer thirds and shown as points and thirds (107 thirds
is "35 2/3").

## 9. Game values and who pays whom

9.1 Values by setting `values` [14 rule 19][14 S]:

| Setting | Simple game, winner gets | Declarer wins, gets | Declarer loses, opponent gets |
|---|---|---|---|
| `2/3/4` (default) | 2 | 3 | 4 |
| `1/2/3` | 1 | 2 | 3 |
| `1/2/4` | 1 | 2 | 4 |

`2/3/4` is Werner [03-S12] and Mayr and Sedlaczek [02-S2], `1/2/3` Dummett and
Beck [02 §3], `1/2/4` Ulmann [03-S14].

9.2 **Payment.** Every deal is zero-sum between the two players:
- declarer wins: the defender pays the declarer the "wins" value;
- declarer loses: the declarer pays the defender the "loses" value;
- simple game won: the loser pays the winner the "simple" value;
- draw: no game payment (the honours and the ultimo are still paid).

[03-S12 "muß ... 4 Points dem Gegner bezahlen"][03-S14 "hat sein Gegner zwei
Points zu entrichten ... erhält der Gegner vier"][14 Still to settle].

9.3 All payments of a deal (game, honours, ultimo, Valat) are collected, doubled
if a doubling applies (14.2), and credited to the accounts (15.5).

## 10. Valat

10.1 A **Valat** is one player winning all 27 tricks (declarer or defender, in a
declared or a simple game). It is quiet: it is not announced. It is worth **12
Points**, paid by the other player to the one who took every trick [14 rule 25]
[03-S12 "12 Points"][02 §3].

10.2 A Valat **replaces** the game payment of 9.2 (no "wins" or "loses" value is
added to the 12).

> Engine note: In a declared game the `game_scored` result of a Valat is `declarer_win` if the declarer took every trick and `declarer_loss` if the defender did; in a simple game it is `simple_win`. The threshold is not consulted. Trull, four kings and Pagat ultimo (11, 12) are separate
payments and are added. [D: the sources give 12 and are silent on combination.]

10.3 Ulmann and Bermann hold that a Valat never happens and pay nothing extra
[03-S14][03-S13]; the canon pays 12 [14 rule 25].

## 11. Trull and the four kings

11.1 There are two honours, **Troule** (`SK`, `T21`, `T1`) and **four kings**
(`Kh Kd Kc Ks`). Each is worth **1 Point** to the player who announces it, and
they are independent [14 rule 20][03-S12][03-S14 "tous les trois, tous les
quatre"][02 A15]. The value is 1 under every setting. An announced honour is
paid even if its cards are later lost in the tricks [03-S12 "Sie werden also
bezahlt, auch wenn sie nicht in die Stiche nach Hause gebracht werden"].

11.2 **Held together in the hand.** A player *holds* an honour at a moment if all
its cards are in his hand at that moment: among his hand cards, **including cards
pulled from packets**, none yet played [14 rule 20][03-S12 "in der Handkarte
gleichzeitig vereint"]. A cover or a face-down packet card is not in the hand. By
4.8 no honour card is ever a cover.

11.3 **Announcement.** A player may announce an honour (`ANNOUNCE trull`,
`ANNOUNCE kings`) on his own turn, before he plays his card, whenever he holds it
[03-S12][03-S14]. The deadline is **at the latest when he first plays one of its
cards**: leading it, following with it, or trumping with it [14 rule 20][03-S12
"spätestens in dem Augenblicke ..."][03-S14 "spätestens dann, wenn der Besitzer
einen dieser Honneurs ausspielt, zugibt oder mit demselben sticht"]. The
announcement is made first and the card is played after it. An honour is
announced at most once per deal.

11.4 **How the engine and the UI offer it.** `legal_announcements(state,
player)` returns, on the player's turn, each honour he holds and has neither
announced nor forfeited. The play action carries an optional list of
announcements. The UI shows announcing as an action available all the time an
honour is held, and prompts again when the player selects a card of a held,
unannounced honour, with "announce" and "do not announce". The engine rejects an
announcement of an honour that is not held at that moment, so a false
announcement cannot occur (Ulmann's penalty, [03-S14 Codex §33], has no case)
[D].

> Engine note: The engine offers both forms: `Announce` as an action of its own, and `Play` with an `announce` list that is validated whole before anything changes. "Do not announce" is not an action; playing a card of a held, unannounced honour forfeits it (11.5). A forfeited honour is refused with the key `honour-forfeited`.

11.5 **Announcing is optional.** A player may keep an honour silent; it is then
not paid [03-S12 "Auch kann man die Honneurs ... verschweigen, erhält sie aber
dann natürlich nicht bezahlt"]. When a player plays a card of a held, unannounced
honour (including by choosing "do not announce"), the honour is **forfeited** for
the deal.

> Decided where the sources are silent: the forfeit is private to the holder; at a real table the opponent would not know an announcement was missed.

11.6 An announcement is public. It tells the opponent that the player holds those
cards at that moment; the cards are not displayed (13.5).

11.7 Honours are paid whatever the result of the deal, a drawn simple game
included [03-S12][02 A15].

## 12. Pagat ultimo

12.1 **Pagat ultimo** concerns `T1` (the Pagat) and the 27th trick. It is
**quiet**: it is never announced [14 rule 21][03-S12 "Der Pagat kann natürlich
nie ultimo angesagt werden"][03-S14][02 §1].

12.2 **Won.** A player who plays `T1` in trick 27 and wins that trick scores
Pagat ultimo: the opponent pays him the ultimo value. **Lost.** A player who
plays `T1` in trick 27 and does not win it pays the opponent the ultimo value.
Trick 27 is the 27th trick whichever cards the players used. `T1` may be played
from the hand or from a packet; as a tarock it is always pulled and never a
cover (4.8), so at trick 27 it is a hand card [14 rule 21][03-S12][02 A13, A14].

12.3 **Value** (setting `ultimo`) [14 rule 21][14 S]: **1** Point (default) or
**2**, the same for a win and a loss [03-S12][03-S14 "in manchen Kreisen mit
zwei Points"][02 §3]. (Bermann's double loss is not a setting [14 S].)

12.4 Pagat ultimo is paid in a declared, a simple and a drawn game, and in
addition to the game payment and the honours [03-S12].

## 13. Information

13.1 **Public**, known to both players, the table and the talk, at every moment:
- dealer and forehand, the bids, the announcements, the sitting's settings
  (`values`, `both_pass`) and the deal's doubling (14);
- every card turned from a packet, whether pulled or left as a cover;
- the covers of all six packets, and the number of face-down cards in each;
- every card played, in order, and each trick's winner;
- the size of each hand, and each take's running count.

13.2 **Private** to its owner: dealt hand cards not yet played, and a forfeit of an
honour (11.5).

13.3 **Unknown to both:** the face-down cards of every packet; the owner does not
know them either [03-S14 "sie kennen einen Theil derselben gar nicht"].

13.4 A **pulled card** is public from the moment it is turned until it is played
(4.6, 6.1). Each hand card carries a flag: `public` (pulled) or private (dealt).

13.5 An announcement (11.3) makes the cards of the honour known to be in that
hand; the engine records them as known cards of that hand.

13.6 The play log is the public record. Taken tricks are not hidden from the log:
players are expected to count ("genaues Tarokzählen") [03-S12 §2], and an
interface may show the cards played and the cards still out. The tutor and the
computer opponent may use the full log. [D: no source restricts looking back.]

13.7 `view(state, player)` contains: own hand with flags, all covers and hidden
counts, the log, bids, announcements, the pending doubling and the accounts. It
never contains the other player's private hand cards or any face-down packet
card. The deck and the shuffle seed are not part of a view.

## 14. Doubling after a draw

14.1 A drawn simple game (8.4) sets a **pending doubling**. The next deal played
after the draw, or thrown in, is doubled [14 rule 18][03-S12 "die nächste Partie,
sei sie aufgenommen oder nicht, sammt allen Ansagen doppelt zählt"][02 A12]
[14 D "the next deal"]. Ulmann (a draw counts nothing and does not double
[03-S14]) is not followed [14 rule 18].

> Engine note: "or thrown in" is read with 14.4: a throw-in is not a deal that consumes the doubling. The flag stays set through void deals and throw-ins and is cleared or set again only by a played deal.

14.2 A doubling doubles every payment of that deal: game, honours, Pagat ultimo
and Valat [03-S12 "sammt allen Ansagen"].

14.3 **It does not stack.** The pending doubling is a flag, not a multiplier. A
draw in a doubled deal sets the flag again for the next deal; a doubled deal that
is not a draw clears it. The multiplier of a deal is 1 or 2 [D].

14.4 A void deal (2.6) and a throw-in (3.4) do not clear the flag. Only a played
deal clears or sets it [D].

14.5 **It lapses at the end of the sitting** (15.7): a doubling pending when the
sitting ends is dropped, not carried into the next sitting [14 D]. If the sitting
is extended (15.6) it carries on.

14.6 Under `both_pass = throw_in` a declared game cannot be drawn (8.3), so the
doubling never arises.

## 15. The session

The session is the period "Abend": Touren announced in advance, a running account
per player, settlement in money. It is a reconstruction: every source for it
describes multi-hand Tarock, and none applies the Touren, the Conto or the
settlement to Strohmandl [14 D][13 headline 6].

15.1 A **sitting** is a sequence of deals between the same two players, ended by
the announced Touren and settled in one reckoning.

15.2 **Length.** At the start the players announce **3 or 6 Touren** [14 D]
[13-S1 Ulmann "drei oder sechs Touren"][13-S3 Bermann]. The default is 3 [D].

15.3 **A Tour is two deals**, one dealt by each player: 3 Touren are 6 deals, 6
Touren are 12 [14 D][13 §1.2]. A deal counts toward the Tour when it is played
out, drawn, or thrown in; a void deal does not [D for the throw-in and void
cases].

15.4 The first dealer is drawn by lot (2.1); the Touren begin with him and the
deal alternates [13-S3 "bei jenem Spieler, der zuerst die Karten ausgab"].

15.5 **The running account (Conto).** Each player has a Conto, a running total in
Points of everything credited to him since the start of the sitting: Werner's
practice of "ein Conto, wo er den Werth eines jeden von ihm ... gewonnenen
Spieles bei fortlaufender Addition anschreibt" [13 §2.1, 13-S2]. After each deal
every payment of clauses 9 to 12 (doubled by 14.2) is credited to the Conto of the
player who receives it. Nothing is subtracted from the payer's Conto. The
**balance** is Conto(A) minus Conto(B).

> Engine note: The Contos are gains only, so they do not themselves sum to zero; "nets to zero" holds of the settlement (15.8), where the payer loses exactly what the other gains, and of the balance (Conto(A) minus Conto(B) is the negative of Conto(B) minus Conto(A)). The property tests check that the credits of all deals sum to the Contos.

15.6 **Extension.** When the announced Touren have been played the sitting may be
extended only by the consent of both players [14 D][13-S3 "Nur mit allseitiger
Zustimmung können die angesagten Touren erstreckt werden"]. An extension adds 3
or 6 Touren (the choice of 15.2); the deal alternates on, and the Contos and a
pending doubling carry over [D: no source gives the length]. The engine asks both
players; one refusal ends the sitting.

> Engine note: Seat 0 is asked first, then seat 1. A refusal by seat 0 ends the sitting at once and seat 1 is not asked.

15.7 **The end.** After the last deal of the announced (or extended) Touren the
engine emits `sitting_end`, drops a pending doubling (14.5) and settles (15.8).

15.8 **Settlement.** The price is **2 kreuzer a Point**: the amount owed is
`|balance| x 2` kreuzer, paid by the player with the smaller Conto to the other.
A balance of zero owes nothing [14 D][13-S13 "zwei bis drei Kreuzer der Point"].
The price is a design choice within Tarock evidence, not Strohmandl evidence
[14 D]. The engine reports integer kreuzer; the display currency (gulden and
kreuzer, or krone) is open (`PLAN.md`).

15.9 **The slate.** The table shows the account as a slate: per player a column of
the running Conto, one entry per credit with its cause, the balance and the sum
owed, as in Werner's ledger [13 §2.1, §2.3]. The engine provides per deal the
list of credits with their cause (`game`, `trull`, `kings`, `ultimo_won`,
`ultimo_lost`, `valat`), the multiplier and the new Contos; the slate is a
projection of this record.

15.10 There is no target score, no race to a number of Points and no Bummerl
[14 D "design B is not canonical"][13 headline 9, 10].

## 16. Events

The engine emits these events in order, as records `{type, ...}`, with cards as
codes (1.2) and players as seat ids. The table animates them and the talk speaks
to them.

| Event | Fields | When |
|---|---|---|
| `sitting_start` | first dealer, touren, settings | 15 |
| `deal_start` | deal number, tour, dealer, forehand, multiplier | a deal begins |
| `dealt` | each player's own 15 cards; packets face down | 2.3, 2.4 |
| `void_deal` | seat(s) with no tarock | 2.6 |
| `bid` | seat, `TAKE` | 3.1, 3.2 |
| `pass` | seat | 3.1, 3.2 |
| `both_pass` | `play` or `throw_in` | 3.4 |
| `thrown_in` | | 3.4 |
| `reveal_start` | seat | 4.2, once per player |
| `packet_turned` | seat, packet, card | 4.4 |
| `card_pulled` | seat, packet, card, last (bool) | 4.5, 4.7, 6.1 |
| `cover_set` | seat, packet, card | 4.5, 6.1 (packet card exposed) |
| `packet_empty` | seat, packet | 4.7, 6.1 |
| `announcement` | seat, `trull` or `kings`, cards | 11.3 |
| `honour_forfeited` | seat, honour | 11.5; private to the holder, dropped from the other seat's events (13.2) |
| `card_played` | seat, card, source (`hand` or `packet n`), trick number | 5 |
| `trick_won` | winner, both cards, trick number | 7 |
| `pagat_ultimo` | seat, `won` or `lost` | 12.2 |
| `valat` | seat | 10.1 |
| `count` | thirds of each take, declarer, threshold 107 | after trick 27 |
| `game_scored` | result (`declarer_win`, `declarer_loss`, `simple_win`, `draw`), credits with causes, multiplier, Contos | 9 to 12 |
| `doubling_set` | | 14.1 |
| `doubling_applied` | multiplier | with `deal_start` |
| `doubling_lapsed` | | 14.5 |
| `tour_end` | tour number, Contos, balance | after every second counted deal |
| `extension_offered` | | 15.6 |
| `extension_decided` | accepted (bool), touren added | 15.6 |
| `sitting_end` | Contos, balance, payer, kreuzer | 15.7, 15.8 |

After `trick_won` come the replacement events of 6.1 (`packet_turned`,
`card_pulled`, `cover_set`, `packet_empty`) for each cover used, leader first
(6.3). A turned last card produces `packet_turned`, `card_pulled` with `last`
true, and `packet_empty`. Moments for the table talk: a long chain, a high card
pulled, an announcement, the Pagat lost or won in the last trick, a draw and a
doubling, a Valat (`PLAN.md` work package 3).

## 17. Decided where the sources are silent

17.1 **Codes** (`T1` ... `T21`, `SK`, rank plus lowercase suit). Reason: the wasm
protocol needs strings that cannot be confused; `SK` and `Ks` differ by case and
by form.

17.2 *Settled 17 Oct 2026:* the red pip order is stated by the sources
(`research/17-red-pip-order.md`: Bermann 1894 p. 17, Königrufen 1840 pp. 8-9,
Werner § 1, Ulmann 1899, Dummett 1980 ch. 22, pagat), so 1.5 is no longer a
decision. The number is kept so later clauses keep theirs.

17.3 **Packet form and top-first order** (2.4). Reason: the tests need a
deterministic deal from a deck order.

17.4 **Forehand reveals first** (4.2). Reason: independent reveals; the order of
speaking and of the deal.

17.5 **Replacement order within a trick: leader first** (6.3). Reason: independent
chains; fixes the event sequence only.

17.6 **The void deal is compulsory and is not a Tour deal** (2.6, 15.3). Reason:
four of five describers annul it; a rule without an option has less to test; it
was not played.

17.7 **A throw-in is a deal of the Tour and passes the deal on** (3.4, 15.3).
Reason: Ulmann has forehand deal next [03-S14]; a Tour is two deals dealt in
turn.

17.8 **Valat replaces the game payment; honours and ultimo are added** (10.2).
Reason: 12 is the sources' whole value; honours and ultimo are separate payments
in them.

17.9 **The announcement is an optional action on the player's own turn, with a
prompt, and only true announcements are accepted** (11.3 to 11.5). Reason: the
sources give the deadline and the option to stay silent, not the mechanism; a
screen can refuse a false announcement, so Ulmann's penalty never applies.

17.10 **A doubling does not stack, and it waits through void deals and
throw-ins** (14.3, 14.4). Reason: Werner says only that the next game counts
double; a flag keeps the multiplier at 1 or 2.

17.11 **A draw still pays the honours and the ultimo** (9.2, 11.7, 12.4). Reason:
Werner pays honours "even if not brought home"; Dummett's "only announcements
paid" in a draw says the same [02 §1].

17.12 **The Conto records gains only** (15.5). Reason: it is Werner's win ledger
settled by difference; it equals a plus-and-minus ledger in its balance.

17.13 **An extension adds 3 or 6 Touren and carries the doubling** (15.6). Reason:
the sources give the consent and not the length; the announced choice is reused.

17.14 **The default length is 3 Touren** (15.2). Reason: the sources give "three
or six" without a default; 6 deals make a short first sitting.

17.15 **Taken tricks stay open to the log** (13.6). Reason: a screen cannot forbid
looking back, and the log is what a perfect memory would hold.

17.16 **The price is 2 kreuzer a Point** (15.8). Reason: within the Tarock stake of
"zwei bis drei Kreuzer der Point" [13-S13]; no Strohmandl stake is sourced.

17.17 **No misdeal** (2.7). Reason: the engine deals correctly; Werner's 1-point
penalty [03-S12] and Ulmann's Codex §30 [03-S14] have no case.

17.18 **Not implemented:** Kontra [14 rule 6], the bird bonuses [14 rule 22],
Rostopschin [14 rule 23], Mondfang [14 rule 24], big-win bonuses [14 S (e)], the
1829 dummy-pile game (`PLAN.md`), a target score (15.10), an announced Pagat
ultimo [14 rule 21], quiet honours [14 rule 20], a lead by the declarer
[14 rule 14], and Werner's rule of the low last card [14 rule 10].
