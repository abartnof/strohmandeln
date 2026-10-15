<!--
The Kibitzer's pages (docs/TABLE3D.md 6.2), English. Keys by position, as the
table in 6.2.2: packets, bidding, the-pull, following, from-a-packet, honours,
pagat-ultimo, thirds, draw, touren-slate, extension. Parallel to tutorial.md.
Clauses (docs/RULES.md): packets 2.3 2.4 4.4 4.8 5.2 13.1; bidding 3.1-3.4 8.3
8.4 9.1; the-pull 4.3 4.5 4.6 4.7; following 5.2 5.4 5.5; from-a-packet 5.2 6.1
6.2; honours 11.1-11.5; pagat-ultimo 1.4 12.1-12.4; thirds 1.7 8.1 8.2; draw 8.4
9.2 11.7 14.1-14.4; touren-slate 15.2 15.3 15.5 15.8 15.9 (reconstruction: 15
intro, TABLE3D 4.6); extension 15.6. Period terms: 03-S12 (Werner), 03-S14
(Ulmann), 05c headline 1 (Kiebitz). The voice is TABLE3D 6.5.
Text before the first heading is ignored by the parser. Blocks marked
with an if-line on a setting are kept for that setting only (tutor.js,
tutorialPages): bidding and draw have one for each both_pass value (RULES 3.4, 14.6).
-->

# The packets

## What is on the table

- You hold 15 cards. In front of you lie **three packets** of four cards, face down.
- Your opponent has the same. Nobody knows the hidden cards, not even the owner.
- Soon the top card of each packet is turned face up. A card left face up on a packet is a **cover**.
- A cover is yours to play, like a card in your hand.
- The small number on a packet counts the cards still hidden beneath.
- If I may: look at your opponent's covers too. They are no secret.

# Bidding

## Take it up, or pass

- The player who did not deal speaks first. Then the dealer.
- **I take it up** (*Ich nehme auf*) means you will play for the win. You are the declarer.
- **Pass** (*Weiter*) means you let it go.
- A declarer needs 36 of the 70 Points. Missing it loses.
- In the standard values the declarer wins 3 Points, or loses 4.
<!-- if both_pass=play -->
- If both pass, a plain game is played. The winner gets 2 Points.
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
- Cards are turned for you; nothing to decide.

# Following

## Which card may I play?

- The leader plays one card. You answer with one.
- Have the suit led? You must play that suit. No such card? You must play a tarock. Neither? Any card.
- Hand cards and covers count alike. Choose either, if both are lit.
- Lit cards are allowed; dimmed ones are not. Tap a dimmed card and I will say why.
- You need not play higher. A tarock beats any suit card; the higher tarock wins. Otherwise the higher card of the suit led wins.

# Playing from a packet

## Using a cover

- Tap a cover as you would a card in your hand. Tap again, or press **Play**.
- When the trick is gathered, the card beneath is turned. A tarock or king goes into your hand; a low card is the new cover.
- Neither you nor your opponent knows the card beneath until it is turned.
- If the card beneath is the last of its packet, it goes into your hand, whatever it is.
- If I may: a cover played is a card turned. That is all.

# The Trull and the four kings

## Honours worth a Point

- The **Troule** is the Sküs, the Mond (XXI) and the Pagat (I). Hold all three in your hand: one Point.
- The **four kings**, held together in your hand: one Point.
- Cards pulled from packets count. Covers do not.
- Say so before the first of those cards is played.
- Press **Announce Trull** or **Announce four kings**, or answer the question when you pick a card.
- Announced, it is paid, even if the cards are lost later. Silent, it is not paid.
- You may stay silent. The choice is yours.

# Pagat ultimo

## The last trick

- The Pagat (I) is the lowest tarock.
- Play it in the last trick, the 27th, and win: one Point from your opponent.
- Play it there and lose: you pay your opponent one Point.
- It is never announced; it is simply paid when it happens.
- The Pagat is paid on top of the game and the honours.
- If I may: a Point won or lost at the last card. Mind your nerves.

# Counting in thirds

## The count

- Kings, Sküs, Mond and Pagat count 5. Queens 4, cavalls 3, jacks 2. Every other card 1.
- Each card counts its value less two-thirds. In a group of three cards, count the sum less 2.
- 36 Points wins. The table shows **35⅔** as enough; **35⅓** is not.
- The tally runs at the top as the count is laid out.
- All 70 are divided between you and your opponent.

# A draw

## When nobody reaches 36

<!-- if both_pass=play -->
- In a plain game, if neither of you reaches 35⅔, the game is undecided. Nobody wins it.
<!-- end -->
<!-- if both_pass=throw_in -->
- With a throw-in, only a declared game is played, and a declared game always has a winner. Draws do not arise.
<!-- end -->
- Honours and the Pagat are still paid.
<!-- if both_pass=play -->
- The next deal played counts double, with everything in it. A deal thrown in does not use it up.
- The table shows **×2** when it applies.
<!-- end -->

# Touren and the slate

## An evening's account

- A **Tour** is two deals, one dealt by each of you. You choose 3 Touren or 6.
- The **slate** keeps the account. Tap **Slate** to see each payment, with its cause.
- When the evening ends, the difference is settled at 2 kreuzer a Point.
- This is how an evening of Tarock was kept. No source says Strohmandl was kept this way: we have borrowed it.

# One more Tour

## Playing on

- When the Touren are played, you may add more. Both players must agree.
- Press **One more Tour**, or **Enough**.
- It adds 3 or 6 Touren, as you first chose. The account carries on.
- If your opponent has had enough, the evening ends.
