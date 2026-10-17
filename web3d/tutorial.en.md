<!--
The Kibitzer's pages (docs/TABLE3D.md 6.2), English. Keys by position, as the
table in 6.2.2: the-cards, packets, bidding, the-pull, playing, following, from-a-packet,
honours, thirds, pagat-ultimo, scoring, draw, touren-slate, extension,
scoring-full. Parallel to tutorial.md. Each page holds what its first decision
needs, no more (docs/review/tutorial-pacing.md); scoring-full is the one long
page, never due, read from Help or from the links on bidding (to thirds) and on
thirds and scoring (to it): `[text](#key)`, TABLE3D 6.2.4.
Sources: the-cards 15-S1 (Dummett, ch. 4 and p. 100: made for play, 1442; no cartomancy before
1781), 15-S6 (Court de Gébelin 1781), 15-S7 and 15-S8 (Tarock and Tarot in German), 15-S9
and 15-S10 (the Austrian games; the trumps), 15-S11 (Piatnik sells Tarock packs, checked 15 Oct 2026; the page names neither Vienna nor the pattern), 15-S12
and 15-S14 (the Cavall, worth 3), 08-S9 and 08-S10 (a real pack has no indices on suit
cards), all in research/15-the-pack-today.md.
Clauses (docs/RULES.md): the-cards 1.2 1.4 1.5 1.7; packets 2.3 2.4 4.4 4.8 5.2 13.1; bidding 3.1-3.4 8.1
8.2 8.4 9.1 (card values 1.7); the-pull 4.3 4.5 4.6 4.7; playing 5.1-5.3 5.5;
following 5.2 5.4; from-a-packet 6.1 6.2; honours 11.1-11.5; thirds 1.7 8.1 8.2;
pagat-ultimo 1.4 12.1-12.4; scoring 9.1-9.3 11 12.3 15.5 15.8; draw 8.4 9.2 11.7
14.1-14.4; touren-slate 15.2 15.3 15.5 (reconstruction: 15 intro, TABLE3D 4.6);
extension 15.6; scoring-full 1.3 1.7 8.1-8.5 9.1-9.3 10.1 10.2 11.1-11.7
12.1-12.4 14.1-14.5 15.2 15.3 15.5 15.6 15.8 15.9 (the worked deal and evening
are held by core tests, web3d/test/scoring_pages.test.js). Period terms: 03-S12
(Werner), 03-S14 (Ulmann), 05c headline 1 (Kiebitz). The voice is TABLE3D 6.5.
Text before the first heading is ignored by the parser. Blocks marked with an
if-line on one or more settings are kept for those settings only (tutor.js,
tutorialPages): the pages that name a game value, the ultimo or what happens
when both pass have one block for each value of `values`, `ultimo` and
`both_pass` (RULES 3.4, 9.1, 12.3, 14.6).
-->

# The cards

## The tarocks

- The tarocks are trumps: the Sküs is the highest, then XXI, XX down to II; I, the Pagat, is the lowest.
- Rank is not value: the Pagat is lowest, yet worth 5 points, the most a card is worth, like the Sküs and XXI.
- Tarot cards were made to play with, in Italy, by 1442. Fortune-telling is first heard of in 1781, in France.
- Nothing here tells your fortune. Austrians still play Tarock, and Piatnik still sells Tarock packs.

## The suits

- Each suit runs K, Q, C, J. The C is the Cavall, a knight, between queen and jack, worth 3 points.
- The 52-card pack dropped him. Pips: 10 9 8 7 in clubs and spades; 1 2 3 4 in hearts and diamonds, the 1 highest.
- Real courts carry pictures, not letters. These big letters are for a quick look.

# The packets

## What is on the table

- You hold 15 cards and, face down in front of you, **three packets** of four. Your opponent has the same.
- Nobody knows the hidden cards, not even the owner.
- Soon the top card of each packet is turned face up. A card left face up is a **cover**.
- A cover is yours to play, like a card in your hand.
- The small number on a packet counts the cards still hidden beneath.
- If I may: look at your opponent's covers too. They are no secret.

# Bidding

## Take it up, or pass

- The player who did not deal speaks first.
- **I take it up** (*Ich nehme auf*) means you will play for the win. You are the declarer.
- **Pass** (*Weiter*) means you let it go.
- Cards are worth: kings (K), Sküs, Mond and Pagat 5; queens (Q) 4; cavalls (C) 3; jacks (J) 2; every other card 1.
- Counted in threes, the 54 cards make 70 points. A declarer needs 36 of them.
- [Card values and the count](#thirds)
<!-- if values=2/3/4 -->
- The declarer who wins gets 3 Points; one who loses pays 4.
<!-- end -->
<!-- if values=1/2/3 -->
- The declarer who wins gets 2 Points; one who loses pays 3.
<!-- end -->
<!-- if values=1/2/4 -->
- The declarer who wins gets 2 Points; one who loses pays 4.
<!-- end -->
<!-- if both_pass=play -->
- If both pass, a plain game is played.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- A plain game pays its winner 2.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- A plain game pays its winner 1.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- A plain game pays its winner 1.
<!-- end -->
<!-- if both_pass=throw_in -->
- If both pass, the deal is thrown in: nothing is scored and no packet is turned.
<!-- end -->
- If I may: once you have spoken, it stands. Nobody can raise.

# The turning

## Before the first card

- Packets are turned in order, left to right. Tap to go on.
- A **tarock** (any trump, the Sküs too) or a **king** is shown and goes into the hand. Then the next card is turned.
- Any other card stays face up as the cover. That packet is done.
- The last card of a packet always goes into the hand, whatever it is.
- Nothing to decide: the cards are turned for you.

# Playing a card

## Your turn

- The leader plays one card, the other answers. The winner takes the trick and leads next.
- A tarock beats any suit card; the higher tarock wins. Otherwise the higher card of the suit led wins.
- Tap a lit card, then tap it again to play it.
- Lit cards are allowed; dimmed ones are not. Tap a dimmed card and I will say why.

# Following

## Which card may I play?

- Have the suit led? You must play that suit. No such card? You must play a tarock. Neither? Any card.
- Covers count as hand cards here.
- You need not play higher.

# Playing from a packet

## Using a cover

- When the trick is gathered, the card beneath is turned. A tarock or king goes into your hand; a low card is the new cover.

# The Trull and the four kings

## Honours worth a Point

- The **Troule** is the Sküs, the Mond (XXI) and the Pagat (I). Hold all three in your hand: one Point.
- The **four kings**, held together in your hand: one Point.
- Cards pulled from packets count. Covers do not.
- Say so before the first of those cards is played.
- Press **Announce Trull** or **Announce four kings**, or answer the question when you pick a card.
- Announced, it is paid, even if the cards are lost later. Silent, it is not paid.

# Card values and the count

## What each card is worth

- **5** each: the four kings (K), the Sküs, the Mond (XXI) and the Pagat (I).
- **4** each: the four queens (Q).
- **3** each: the four cavalls (C).
- **2** each: the four jacks (J).
- **1** each: every other card, the tarocks II to XX included.

## How to count

- Count your won cards in threes: add the three values, take off 2.
- Example: a king, a jack and a seven are 5 + 2 + 1 = 8, less 2 = 6.
- One or two cards left over lose two-thirds each.
- Taking 2 off three cards is two-thirds off each, so the count runs in thirds.
- Together, the two players' cards always count 70.
- To win you need 36: that is **35⅔** or more. **35⅓** falls short.
- The score at the top runs as you take tricks.
- [More on scoring](#scoring-full)

# Pagat ultimo

## The last trick

- The Pagat (I) is the lowest tarock.
- Play it in the last trick, the 27th, and win: your opponent pays you the ultimo, one Point in the standard values.
- Lose it there: you pay the ultimo.
- It is never announced; it is simply paid when it happens, on top of the game and the honours.
- If I may: a Point won or lost at the last card. Mind your nerves.

# What a deal pays

## The reckoning

- A **Point** is what is paid. The count of 36 points only decides who wins the game.
<!-- if values=2/3/4 -->
- The declarer who wins gets 3 Points; one who loses pays 4.
- With the Trull announced as well, a win pays 3 + 1 = 4.
<!-- end -->
<!-- if values=1/2/3 -->
- The declarer who wins gets 2 Points; one who loses pays 3.
- With the Trull announced as well, a win pays 2 + 1 = 3.
<!-- end -->
<!-- if values=1/2/4 -->
- The declarer who wins gets 2 Points; one who loses pays 4.
- With the Trull announced as well, a win pays 2 + 1 = 3.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- A plain game pays its winner 2.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- A plain game pays its winner 1.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- A plain game pays its winner 1.
<!-- end -->
- Each honour you announce pays 1 Point on top.
<!-- if ultimo=1 -->
- A Pagat ultimo, won or lost, is worth 1 Point.
<!-- end -->
<!-- if ultimo=2 -->
- A Pagat ultimo, won or lost, is worth 2 Points.
<!-- end -->
- Every payment goes onto the **slate**, with its cause. Tap **Slate** to see it.
- At the end of the evening the difference is settled at 2 kreuzer a Point.
- [More on scoring](#scoring-full)

# A draw

## When nobody reaches 36

<!-- if both_pass=play -->
- In a plain game, if neither of you reaches 35⅔, the game is undecided. Nobody wins it.
- Honours and the Pagat are still paid.
- The next deal played counts double, with everything in it. A deal dealt again for want of a tarock does not use it up.
- The table shows **×2** when it applies.
<!-- end -->
<!-- if both_pass=throw_in -->
- With a throw-in, only a declared game is played, and a declared game always has a winner. Draws do not arise.
<!-- end -->

# Touren

## An evening's account

- A **Tour** is two deals, one dealt by each of you. The slate adds up each Tour.
- This is how an evening of Tarock was kept. No source says Strohmandl was kept this way: we have borrowed it.

# One more Tour

## Playing on

- When the Touren are played, you may add more. Both players must agree.
- Press **One more Tour**, or **Enough**.
- It adds 3 or 6 Touren, as you first chose. The account carries on.

# Scoring in full

## What each card is worth

- **5** each: the four kings (K), the Sküs, the Mond (XXI) and the Pagat (I). Seven cards.
- **4** each: the four queens (Q).
- **3** each: the four cavalls (C).
- **2** each: the four jacks (J).
- **1** each: every other card, that is the tarocks II to XX and the numbered suit cards.

- Rank is another matter. Tarocks run Sküs (highest), XXI, XX down to II, then I, the Pagat, the lowest tarock.
- Each suit runs K, Q, C, J, the Cavall between queen and jack, then the pips: 10 9 8 7 in clubs and spades, 1 2 3 4 in hearts and diamonds, the 1 highest.
- The faces show the letters K, Q, C and J, and the tarocks in Roman numerals.
- Add all 54 cards at these values and you get 106. The count in threes brings that down to 70.

## Counting in threes

- Count your take, the cards you have won, in threes: add the three values and take away 2.
- A king, a jack and a seven are 5 + 2 + 1 = 8, less 2 = 6. Three low cards are 1 + 1 + 1 = 3, less 2 = 1.
- One or two cards left over lose two-thirds each. A last king and jack are 5 + 2 = 7, less 1⅓ = 5⅔.
- Why thirds: taking 2 off a group of three is taking two-thirds off each card. So every card counts its value less two-thirds: a king 4⅓, a queen 3⅓, a cavall 2⅓, a jack 1⅓, any other card ⅓.
- This is the traditional count in threes, kept exact: nothing is rounded.
- The pack is 54 cards, 18 groups of three. The values add up to 106, and the 18 groups lose 2 each: 106 − 36 = 70.

## 36 of 70

- The two takes always add up to 70 points, and 36 is more than half of 70.
- The count keeps thirds, so a take can be 35⅔. Rounded to the nearest whole number that is 36; 35⅓ would be 35. The rule: 35⅔ wins, 35⅓ does not.
- When one take has 35⅔, the other has 34⅓.
- In a declared game the declarer needs 35⅔ or more. With less he loses, even with 35⅓. There is no draw.
<!-- if both_pass=play -->
- In a plain game the player with 35⅔ or more wins. If neither has, it is a draw: 35 : 35, or 35⅓ : 34⅔.
<!-- end -->

## What the score at the top shows

- While you play, the line at the top gives the Tour, the deal and the trick, such as Trick 14 / 27.
- The score runs as you take tricks. After each trick every line holds the cards won so far, counted as the count will count them.
- The number beside the score line is the tally in exact thirds. The line fills one segment for each whole point, up to 36.
- After the 27th trick the count begins again from nothing. Each take is turned up, three cards at a time, and over each group a figure rises: the group's count, such as +6 for the king, jack and seven.
- It ends on the same numbers the tally had. A take that reaches 35⅔ lights the finish; one that ends on 35⅓ is marked "35⅓ is not 36".
- Honours and ultimo come after the count, in Points. They have their own row, "Honours and ultimo", and are not added to the 36.
- The arrow beside the score opens the list by deal: the count, and the honours and ultimo. A banner then names the result and the Points paid.

## What the game is worth

- The count decides who wins. The game is then paid between the two of you, in Points. A Point is a unit of payment, not a point of the count.
<!-- if values=2/3/4 -->
- The declarer who wins gets 3 Points; one who loses pays 4. The size of the count does not matter: 36 points and 45 points pay the same.
<!-- end -->
<!-- if values=1/2/3 -->
- The declarer who wins gets 2 Points; one who loses pays 3. The size of the count does not matter: 36 points and 45 points pay the same.
<!-- end -->
<!-- if values=1/2/4 -->
- The declarer who wins gets 2 Points; one who loses pays 4. The size of the count does not matter: 36 points and 45 points pay the same.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- A plain game pays its winner 2. A draw pays nothing for the game.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- A plain game pays its winner 1. A draw pays nothing for the game.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- A plain game pays its winner 1. A draw pays nothing for the game.
<!-- end -->
<!-- if both_pass=throw_in -->
- If both pass, the deal is thrown in: nothing is counted and nothing is paid.
<!-- end -->
- If one player wins all 27 tricks, that is a Valat. The other player pays 12 Points in place of the game's value, and honours and ultimo are paid as well.

## Honours

- There are two, each worth 1 Point: the Trull (the Sküs, the Mond and the Pagat) and the four kings.
- You must hold all of its cards in your hand together. Cards pulled from the packets count; covers do not.
- Announce it before you play the first of its cards. Press **Announce Trull** or **Announce four kings**, or answer when you pick the card.
- Announced, it is paid even if you lose the cards later, and in a drawn game too. Silent, it is not paid.

## The Pagat ultimo

- Ultimo is the last trick, the 27th. Play the Pagat (I) in it and win: your opponent pays you the ultimo. Lose: you pay.
- It is never announced. It is paid when it happens, in any game, on top of the game and the honours.
<!-- if ultimo=1 -->
- A Pagat ultimo, won or lost, is worth 1 Point.
<!-- end -->
<!-- if ultimo=2 -->
- A Pagat ultimo, won or lost, is worth 2 Points.
<!-- end -->

## A draw and the double

<!-- if both_pass=play -->
- A plain game in which neither take reaches 35⅔ is a draw. Nobody wins the game. Honours and ultimo are still paid.
- The next deal played counts double: game, honours and ultimo all twice. The table shows **×2**.
- A deal dealt again for want of a tarock does not use it up. It is never more than twice, and one still waiting when the evening ends is dropped.
<!-- end -->
<!-- if both_pass=throw_in -->
- With a throw-in only a declared game is played, and it always has a winner. There is no draw, so there is no double.
<!-- end -->

## One deal from start to finish

- You take it up and win 14 of the 27 tricks. Your take is 28 cards, your opponent's 26.
- Your 28 cards are worth 55 in all. Two-thirds off each card is 28 × ⅔ = 18⅔, so you count 55 − 18⅔ = 36⅓.
- Your opponent's 26 cards are worth 106 − 55 = 51. Less 17⅓, that is 33⅔. The two takes make 70.
- 36⅓ is 36 or more, so you win the game.
- You held the Sküs, the Mond and the Pagat in your hand and announced the Trull, so it is paid.
- In the last trick you lead the Pagat and your opponent plays a higher tarock. You lose the ultimo.
<!-- if values=2/3/4 -->
- The slate writes in your account: Game 3, Trull 1.
<!-- end -->
<!-- if values=1/2/3 -->
- The slate writes in your account: Game 2, Trull 1.
<!-- end -->
<!-- if values=1/2/4 -->
- The slate writes in your account: Game 2, Trull 1.
<!-- end -->
<!-- if ultimo=1 -->
- In your opponent's account it writes: Ultimo lost 1.
<!-- end -->
<!-- if ultimo=2 -->
- In your opponent's account it writes: Ultimo lost 2.
<!-- end -->
<!-- if both_pass=play -->
- Had the deal been doubled, every one of those figures would be twice as much.
<!-- end -->

## Touren and the slate

- A Tour is two deals, one dealt by each of you. You chose 3 or 6 Touren at the start: 6 or 12 deals.
- The slate keeps an account for each of you. Every Point paid to you goes into your account, with its cause. Nothing is taken from the one who pays.
- The balance is the difference between the two accounts.
- This is how an evening of Tarock was kept. No source says Strohmandl was kept this way: we have borrowed it.

## Settling up

- At the end of the evening the difference between the two accounts is settled at 2 kreuzer a Point. The one with the smaller account pays.
- Example: after six deals your account reads 12 and your opponent's 9. The difference is 3 Points, so 6 kreuzer, and your opponent pays you.
- Equal accounts: nobody pays.
- You may add more Touren if you both agree. The accounts carry on.
