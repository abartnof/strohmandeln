# The table: specification

> Status: **draft for build**, 2026-10-14. Work packages 8, 9 and 11 of
> `PLAN.md` are built to this document. A change to it starts as a change to a
> test. Modelled on Cassino's `docs/TABLE3D.md`; what carried over is named.

References: `[R n]` is a clause of `docs/RULES.md`; `[P]` is `docs/PROTOCOL.md`;
`[M]` is `docs/MESSAGES.md`; `[K]` is `kit/README.md`; `[PH]` is
`docs/PHRASES.md`; `[13]` and `[14]` are the research notes of those numbers;
`[05c]` is `research/05c-literary-banter.md`.

## 0. Principles

0.1 **The page is a client.** It holds no rules. What may be played is
`state.legal` [P]; what a refusal means is its message key [M]; what is known
is the human's view [R 13.7]. The interface never guesses a legal move.

0.2 **Identities only where the view has them.** A mesh shows a face only if the
view reveals the card to the human: the human's hand, covers, pulled cards
(public, [R 13.4]), cards played, cards in the won piles as the count turns
them up. The opponent's hand, every face-down packet card and every won pile
show backs. A test walks scripted sittings and checks that no mesh carries a
face the view does not give (`strohmandeln_audit`, [P]).

0.3 **Every movement starts with a jerk and eases into place** (`kit/js/kinematics.js`:
`toss`, `pull`, `flip`, `peelOff`, `carryBlock`, `slide`, `row`, `fan`; `snap`
for cards moving within a hand). A card leaving a stack is peeled off first.
No card passes through the table; every corner of every card stays above it
(the kit's path tests, applied to every timeline).

0.4 **The opponent is "your opponent"**, never a proper name, in every string,
tooltip, screen-reader text and tutorial page. The human is "you" / "Sie" (the
period formal address; the tutor and the opponent use it, never "du").

0.5 **The page makes no sound.** Every aid is a toggle. Every string has a key
(8). Every third-party asset is credited in `CREDITS.md` in the commit that
brings it in.

0.6 **The engine's seats.** Seat 0 or 1; `state.human` is the human's. "Mine /
yours" in `docs/PHRASES.md` is the speaker's: `-mine` is the opponent's own
event, `-yours` the human's.

## 1. Layout

`layout(state) -> Map<code, Pose>` is a pure function of the protocol state,
node-tested. Camera numbers are set after the zones are measured (`kit/js/framing.js`,
`units.js`); this section fixes the arrangement and its constraints.
Zone names are in `web3d/src/zones.js` (the game's own, [K]).

### 1.1 Zones

| Zone | Contents | Cards shown |
|---|---|---|
| `your-hand` | the human's hand cards, incl. pulled ones | faces |
| `your-packets` (3) | the human's packets 1, 2, 3, left to right | top (cover) face up; hidden cards beneath face down |
| `their-hand` | the opponent's hand cards | backs; count = `opponent_hand_size` |
| `their-packets` (3) | the opponent's packets, 1 to 3 | cover face up to the human, readable; the rest face down |
| `trick` | the open trick: the lead, then the follow | faces |
| `your-won`, `their-won` | the won-trick piles | backs, squared; thickness = count |
| `slate` | the running account (4) | a panel, not a card |
| `hud` | the score HUD [K] | a panel |
| `pack` | the 54 cards before the deal, and the cut | backs, a cut card face up |

### 1.2 Landscape (computer, tablet held sideways)

From the human's side to the opponent's, bottom to top of the screen:

1. **Your hand**, fanned, floating, square to the eye (`kinematics.fan`). Choosing
   a card lifts it clear of the fan (1.6 cm, as Cassino).
2. **Your packets**, in a row on the table above the hand, three stacks left to
   right (packet 1 leftmost), each stack's thickness showing the hidden cards
   (`hidden[you][i]`, 0 to 3): cards one thickness each, squared, with the cover
   lying flat on top. A **count badge** on the stack's near corner says the
   hidden number ("3") in a small disc (white on dark, ringed white, clear of
   the cover's index); an empty packet shows no stack, no badge.
3. **The trick**, in the middle: the lead and the follow side by side, the lead
   on the leader's side. A card lies where its player's side is.
4. **Their packets**, a row mirroring (2), covers face up *toward the human*: the
   cover is turned so its index reads upright from the human's seat. Their
   badges sit on the far corner.
5. **Their hand**, a fan of backs along the far edge, square to the eye.

Won piles lie at the table's right (the human's nearer, the opponent's farther),
clear of packets and trick. The HUD is top left; the slate's tab top right
(opened over the table, 4.1). Left-handed mode (3.8) mirrors the right/left
placement of piles, slate and move bar.

### 1.3 Portrait (phone upright, below `PORTRAIT_BELOW`)

Top to bottom: HUD strip; the opponent's hand as a short overlapped row of
backs; their packets (three stacks); the trick; your packets; the move bar (the
announce chip and the bid buttons live here); your hand as a **row**, not a fan,
spaced wide (`kinematics.row`), 15 cards as a single row that scrolls
horizontally when it exceeds the width, never wrapping; won piles as two small
squared stacks beside the trick. The slate opens as a sheet from the bottom.
Cards on the table are the hand's size ("make the cards on the table the same
size", Cassino's play-testing).

### 1.4 Touch (tablet held upright, large touch screens)

As landscape, with the camera nearer and cards about a tenth larger; targets at
least 44 px (3.3). A tablet held sideways uses 1.2 with the touch camera.

### 1.5 Constraints (each is a node test over states from real sittings, in all three arrangements and at the test window sizes)

1.5.1 **No overlaps.** No two zones' card rectangles intersect on screen. The only
overlaps are inside a zone: a fan or an overlapped row, a stack's thickness, and
the trick's lead and follow, which lie side by side with at most a sliver
touching.

1.5.2 **A face-up packet card is as legible as a hand card.** The cover is drawn at
the hand card's size on the screen (to a tenth, as Cassino matched the table's
cards to the row's), with its full index in view: no other card, badge or
stack edge covers its corner index. The badge sits on the opposite corner.

1.5.3 **The human's covers are playable and look it**: on the human's turn the
covers and hand cards in `legal` lift or glow, the others dim (3.1). A cover
is still a cover, not a hand card: it keeps its place on its stack.

1.5.4 **Hidden counts are exact**: the badge equals `hidden[owner][i]`. A packet
that is empty has no stack and no badge.

1.5.5 **Covers of both players are in view at all times**, including with the
slate open and with a dialogue box shown. A box never covers a cover, the move
bar, the HUD or the slate tab (it takes the first free place, as Cassino's
`choosePlace`).

1.5.6 **The trick card is not hidden**: the trick zone is clear of both packet rows
by at least a card's half-height.

1.5.7 **Pulled cards enter the hand in place**: the hand reflows (fan or row) to
make room; public cards (pulled, [R 13.4]) carry a small mark in the hand: a
thin outline on the lower edge, so the human can tell the cards the opponent
has seen (3.7 explains it).

1.5.8 **Opponent's hand count** (`opponent_hand_size`) shown as cards, never as a
number alone; at least one back is visible until it is empty.

1.5.9 **Faces size** (7.1): "auto" (large type on a phone or tablet), "classic" or
"jumbo". Court labels follow the language (7.5).

## 2. Choreography

`choreograph(prev, next, placement, view)` turns the events since the last
state into a timeline (`kit` director, as Cassino). A reducer replays events
into the intermediate states, each laid out by `layout`; a final settle lands
every card on `layout(next)`; a new sitting or an import is one direct
transition. Tests hold every timeline of real sittings to its layout, its
faces, no leak, no card through the table.

Timing is in the speed setting's units (7.1); "Instant" waits for nothing.
Numbers below are at normal speed, starting values, tuned in play.

### 2.1 Events

| Event [R 16] | Motion |
|---|---|
| `sitting_start` | The table is empty; a pack of 54 backs lies at the centre. The greeting is said (5). The sitting-length choice (3.5) is made before this, on the new-sitting screen |
| first dealer (`sitting_start`) | **The cut.** A card is peeled from the pack for each player and laid face up beside it, yours nearer you, both seen a moment (the two faces are decorative: cards of the human's own hand, drawn from nothing the opponent holds); the dealer is drawn by lot (R 2.1), so no high or low rule is claimed. The cards go back on the pack face down; the dealer marker is set and `cut` is said. With reduced motion: the marker appears and nothing is cut |
| `deal_start` | Every card is collected to the pack at the new dealer's right, a face-up card turning over on its way; a **dealer marker** (a small disc, "D" or "Geber" by language) sits by the dealer. A doubled deal (`multiplier` 2) shows the **doubling marker** (a "×2" disc, chalk-white, by the HUD) |
| `dealt` | **The deal, 3 × 5.** From the top of the pack, five cards at a time to forehand, five to the dealer, round three times: each card from the pack's top straight to its place, the human's turning to face as they rise. Then **six packets of four**: four cards each, face down, to forehand's first packet, the dealer's first, forehand's second ... (R 2.4); each packet lands as a squared stack in its place, the cards offset a hair so the thickness reads. The deal is quick and rhythmic: the pack's top card is flicked (`toss`) at 70 ms intervals within a hand, 160 ms between packets |
| `void_deal` | Each player's hand cards are turned up and fanned a moment: the player without a tarock shows it (only that player's cards are named in the line; for the human's own cards, that is the fan; for the opponent's, the backs turn up, a legitimate reveal since the deal is annulled and nothing is played). A "no tarock" tag rises over the hand; the cards sweep back to the pack; the same dealer deals again (`deal_start` follows) |
| `bid` | Speech and a badge. The bidder's line is said (5); a **bid badge** ("Ich nehme auf" / "I take it up") rises over the bidder's hand zone and stays there the whole deal as a small chip by the dealer marker, so the declarer is always visible; the HUD marks the declarer. The human's bid is made by a button (3.4) and badges the same |
| `pass` | Speech; a "Weiter" / "Pass" chip rises over the passer for 1.2 s and fades |
| `both_pass` (`play`) | A "Stille Partie" / "Simple game" chip, kept until the reveal ends. (`both_pass` is followed by `reveal_start`) |
| `both_pass` + `thrown_in` | The 54 cards are collected into the pack, the dealer marker moves to the other player, a "Eingeworfen" / "Thrown in" chip is shown; the Tour counter on the HUD advances (R 15.3). Nothing is scored |
| `reveal_start` | The seat's three packets are named in turn: the active packet lifts a little (a hair) to show it is the one in play. Forehand first (R 4.2) |
| `packet_turned` | **The turn.** The top card of the packet is lifted off its stack (`peelOff`), turned over (`flip`) along an edge that stays on the table, and held up about a third of a card above the packet, facing the human, long enough to read: **800 ms** at normal speed, 1400 ms for a card the opponent speaks to (a pull of the Sküs, the Mond, the Pagat, a king: the line begins as the card is raised). Always shown, whoever's it is |
| `card_pulled` | The held card is **shown, then flown**: after the hold, it moves (`pull`, with a small tug first) to the owner's hand and settles in its place, the hand reflowing. A pulled card is a public hand card (1.5.7). The human's own pulled card turns to face the human as it arrives; the opponent's arrives as a back (`their-hand`), because the view says it is public but the hand is shown in backs: the **public mark** (1.5.7) is also drawn on the opponent's back, so the human can see which of their cards they know. The chain continues: the packet's next card is turned straight away (the hold of the previous card still holds the interval), until: |
| `cover_set` | The turned card, a low suit card, is lowered back onto its packet, face up, and settles as the cover; the chain ends. It is held no longer than a pulled card (800 ms) and is legible there (1.5.2). Its owner's next packet then begins |
| `card_pulled` (`last: true`) and `packet_empty` | The fourth card, whatever it is, is turned, held, flown to the hand; the empty packet's stack and badge fade. A short "last card" tag (not a line of speech) |
| `announcement` | The honour's cards are **not** displayed [R 11.6]. A chip ("Troule" / "Vier Könige") rises over the announcer with a point badge "+1"; the card then played follows. The line is said (5) |
| `honour_forfeited` | A "verwirkt" / "forfeited" chip rises over the player and fades; said only for the opponent's (the human is told through the engine's reply, 3.3) |
| `card_played` (from hand) | The card is tugged out of the hand, arcs a little above it, and falls into the trick zone (`toss`), tilted ten degrees on its leading edge and sliding flat. The hand reflows (the gap closes by a `snap`) |
| `card_played` (from a packet) | The cover lifts off its stack (`peelOff`), turns slightly toward the middle and falls into the trick zone like a hand card. The stack's top is bare for a moment: the hidden card beneath is *not* shown (the replacement flip follows the trick, 2.2). The badge keeps its count until the replacement. A packet playing the same trick as a hand card is distinguished by the departure point only |
| `trick_won` | **The gather.** Both cards rest a moment (1000 ms, 600 for a trick the human has already seen coming: the leader's card, the follower forced). The cards are gathered into one block (`carryBlock`) and carried to the winner's won pile, face down, a trick at a time (squared on the pile, thickness the count). The winner is named by the pile they land on; the HUD's count line moves (the running count shows only the winner's take in thirds) |
| the replacement (after `trick_won`) | **The replacement flip** (R 6.1): for each seat that played a cover, leader first, the packet's next card is turned, held, and either pulled (the chain continues as in the opening reveal) or lowered as the new cover. The two chains run one after the other, never together. A packet with nothing left is empty and fades |
| `pagat_ultimo` | See 2.3 |
| `valat` | See 2.4 |
| `count` | See 2.5 |
| `game_scored` | See 2.6 |
| `doubling_set` | A "×2" disc settles by the HUD with the draw's chip (2.6) and stays until applied |
| `doubling_applied` | The disc moves to the new deal's HUD line; a line is said (5) |
| `doubling_lapsed` | The disc fades with a dry chip ("verfällt" / "lapses") |
| `tour_end` | See 2.7 |
| `extension_offered`, `extension_decided`, `sitting_end` | See 2.7 |

2.1.1 **The sequence of a chain is one timeline.** The held card of a pull stays
held until the previous card has settled; the next card's `peelOff` begins as
the previous card's arrival begins (an overlap of 200 ms), so a long chain reads as
a run. A chain of four or more is a **run**: the hand reflows once at the end
(each pulled card taking its place as it lands, not all at once).

2.1.2 **Both sides turn the same way.** The opponent's packets are turned exactly as
the human's: lifted, turned, held facing the human, flown. Only the destination
differs (the opponent's hand shows backs, with the public mark).

2.1.3 **The reveal waits for the bidding** to be spoken; the human's first lead
waits for the reveal's last card to settle. The opponent's first play waits
for the opening's speech.

### 2.2 A play: hand versus packet

2.2.1 A play from the hand and a play from a packet land alike in the trick zone;
they differ in where they leave from. A packet play leaves a bare stack top (the
hidden card unrevealed) until the trick is gathered and the replacement flip
begins. The badge number does not change until the replacement turns the
next card.

2.2.2 The follower's play from a packet is made *before* the leader's replacement
(R 6.2): the leader's replacement is shown only after the gather, so the
follower plays without seeing it. The choreography never turns a replacement
card before `trick_won` is complete.

2.2.3 A play with an announcement (3.3) shows the chip first (400 ms), then the card.

### 2.3 The last trick and Pagat ultimo

2.3.1 Trick 27 is slower: the lead is laid with a longer pause (1.2 times) and the
HUD's trick counter says "27 / 27". The human's two last cards (one hand card,
or two covers, or one and one) are shown singly, so the last play is not lost.

2.3.2 **`pagat_ultimo`** (`won` or `lost`): the Pagat (`T1`) is on the table in
trick 27. After `trick_won`, the Pagat is held up a moment (1.4 s) where it
lies, a gold ring behind it, and the ultimo chip ("Pagat ultimo" with "+1" or
"+2" by setting, "−1" ... for the loser) rises over the player concerned. If won,
the ring is white and the points burst up (the kit's celebration style); if lost,
the ring is grey and the chip drops. The line is said. This is in addition to
the gather, which follows.

### 2.4 Valat

Quiet, as the rules say [R 10.1]. On `valat` the last trick's gather is
followed by the whole won pile fanned a moment (it is only the winner's), a "Valat · 12" chip rising over the winner. The line is said (5).

### 2.5 The count

2.5.1 After trick 27 and its gather (and a Pagat ultimo, 2.3), the `count` event
starts the count ritual, using the kit's HUD and popups [K]:

```js
const game = defineGame({
  categories: [["count", "Won cards"], ["bonus", "Honours and ultimo"]], // en/de by language (8)
  target: 107, scale: 3, total: 210,       // whole thirds; 107 = 35⅔
  format: (units) => thirds(units),        // 107 -> "35⅔", 106 -> "35⅓", 105 -> "35"
  goal: "36 of 70",                        // "36 von 70" in German
  names: { you: "You", opp: "Your opponent" },
  line, popup, cards,
});
```

(`thirds(n)` is whole points, a space-less vulgar fraction ⅓ or ⅔, nothing for a
whole number; `format` is rebuilt on a language switch, 7.5.)

2.5.2 **Won cards are turned up in threes.** Each take (the human's first, then
the opponent's) is taken from its won pile, 3 cards at a time: each group peeled
off the pile (`peelOff`), turned face up (`flip`), and laid in a row in the
count zone in front of its owner, the groups laid out left to right in the order
counted, every group's cards overlapping by a third so their indices read. The
group's popup (`+n` thirds, in the value of the group: the sum of 3×value−2 for
each card, [R 8.1]) rises over the group, and the running tally in the HUD
ticks up. A last group of fewer than three shows its popup the same way. One
popup a beat (`countSchedule`, `scalePace`); a side's next popup waits for its
last.

2.5.3 **The running tally in exact thirds.** The HUD's line for each side runs to
36 (a segmented line to 107 thirds, segments by the kit); the number beside it
shows the exact tally, `format(units)`, e.g. "11⅓", "34⅔", "35⅔". The tally
rounds nothing [R 8.1]. The target mark at 107 is the HUD's finish (a wider
segment), labelled "36" (7.5). When a side's tally reaches 107 the finish
lights; when a side ends at 106 the line stops one short of the finish and the
tally reads "35⅓", with the chip **"35⅓ is not 36"** (the edge shown, 5.4; tutor
page "the count in thirds", 6.2). The other side's tally is shown the same way.
The two takes always sum to 210 thirds; the HUD does not display the sum.

2.5.4 **The edge.** At the count's end the winning tally and the target are
joined by a tick: a hairline at 107 on both lines; a side at 106 stands one
segment shy of it for a beat (600 ms) before the result is drawn. A tally of
105:105 (an undecided simple game) shows both lines one short of the finish and
no tick lights.

2.5.5 Bonuses (honours, ultimo, valat) are paid in Points and are shown in the
`bonus` category; they come after the count's cards, each as a popup over the
credited player's HUD row, never as a card.

2.5.6 Nothing is said during the count (5.3). The popups are the voice.

2.5.7 At Instant speed no popup waits; the cards are laid in one step. With reduced
motion, the groups fade in place.

### 2.6 The game scored, the draw, the void deal, the throw-in

2.6.1 **`game_scored`** (`declarer_win`, `declarer_loss`, `simple_win`, `draw`):
after the count's popups, a **result banner** rises at the table's centre
(not on the trick zone, which is clear): "Ich nehme auf: gewonnen · +3" /
"Declarer wins · +3", with the credits listed beneath it as chalk rows (game,
Troule, kings, ultimo, valat), each with its Points; doubled, a "×2" follows.
The slate's row for the deal is written at the same time (4.3). The banner
stays until the human taps or 3 s (at normal speed) pass; the next deal does
not begin before it is gone.

2.6.2 **The draw.** `draw`: the banner says "Unentschieden" / "A draw" and the
tallies (e.g. "35 : 35"). The honours and ultimo, which are paid, are listed.
The **doubling marker** (a "×2" chalk disc) appears and stays at the HUD
until `doubling_applied`; the next deal shows it on its HUD line.

2.6.3 **The void deal** (`void_deal`) is a short scene (2.1 table); the HUD does
not count it as a deal of the Tour. A line is said (5).

2.6.4 **The throw-in** (`thrown_in`) is the collection of cards and a chip (2.1). The
doubling marker, if pending, stays.

### 2.7 The Tour's end, the extension, the settlement, the farewell

2.7.1 **`tour_end`**: after the second counted deal of a Tour, the HUD's Tour
counter advances ("Tour 2 of 3") and a rule is drawn under the Tour's two rows
on the slate, with the Contos written on it (4.3). The slate opens for a
moment (4.1) with the new rule. A line is said (5).

2.7.2 **`extension_offered`** (after the last Tour): the settlement is *not*
shown yet. The opponent says "Noch eine Tour?" (`extend-ask`) and a prompt with
two buttons appears: **"Noch eine Tour"** / **"One more Tour"** and
**"Genug"** / **"Enough"** (3.6). The prompt names the length that will be
added (the sitting's choice, R 15.6). The opponent's answer follows by the
engine (`extension_decided`): `extend-yes` / `extend-no` is said and the prompt
closes. An extension adds a rule and a header on the slate; the Contos carry.

2.7.3 **`sitting_end`**: the camera pulls back over 2.6 s, past the table's near
edge, as Cassino's end does; the cards fade back to the pack; the slate opens
large, centred, and the settlement is written on it: the two Contos, the
balance, and the sum "Bezahlt: 12 kr." / "Owed: 12 kr." (4.4) with the
payer named ("Sie zahlen" / "You pay"; "Ihr Gegner zahlt" / "Your opponent
pays") or "Quitt." / "Square." The farewell is said (5) after the figures are
written (one line, no more, as in Cassino: the last word). The page then offers
"New sitting" and "Review" (6.4). With reduced motion or Instant, the camera cuts.

2.7.4 The settlement figures are written one by one onto the slate (4.4), each
number appearing as the chalk would write it (a stroke wipe, 120 ms a figure).

### 2.8 Speed

Four speeds as Cassino: Slow, Normal, Fast, Instant (7.1). The timeline's clock
scales; Instant collapses every wait and runs the timeline in one frame, still
ending on `layout(next)`. A waiting human can skip a hold by tapping the table
(a tap in the reveal's hold sends the card on). Reduced motion (3.9) cuts to
state and keeps the cards' positions only.

## 3. Interaction

### 3.1 Choosing a card

3.1.1 The legal cards are exactly those in `state.legal` with `action: "play"`; a
`source` of `hand` or `packet n` says where. The page lights each such card
(hand and cover alike) and dims the rest. It makes no judgment of its own:
it never hides, disables or allows a card by a rule of the page. (A test fails
if the page code contains the follow-suit rules: `must-follow`, suit
comparisons in the table code.)

3.1.2 **Tap or click** a lit card: it lifts (hand) or rises (cover) and the move
bar offers **Play** ("Ausspielen" / "Play"); a second tap on the card, or a
tap on Play, plays it. A double-tap or a drag of the card to the trick zone
plays it directly. Where a card of a held, unannounced honour is chosen, the
announce prompt appears first (3.3).

3.1.3 **A card that is not lit**: a tap sends the card anyway only when a refusal
is wanted (3.2); the default is a gentle shake of the card and the *reason*
shown at once, derived from the engine, not guessed (3.2).

3.1.4 The human's turn is shown by a pulse of the lit cards and the prompt text
under the hand ("Your lead." / "Sie spielen aus."; "Follow suit." / "Farbe bedienen."
only where the legal list has narrowed to the suit led, taken from `legal` and
the open trick: the prompt is a hint, not a rule, so it is under explanations
(7.1)).

### 3.2 Why a card is refused

3.2.1 A tap on a dimmed card sends `play` to the engine. The engine answers
`ok: false` with a `refusal` [P][M]; nothing changes. The page renders the
refusal's key in the current language in a **refusal chip** that stays under the
prompt until the next move (and is announced to the screen reader, 3.10):

| Key | English | German |
|---|---|---|
| `must-follow-suit` | "Follow suit: {led}." (with the allowed cards lit) | "Farbe bedienen: {led}." |
| `must-trump` | "You must trump." | "Sie müssen stechen." |
| `not-your-turn` | "Not your turn." | "Sie sind nicht am Zug." |
| `not-playable` | "That card is not in your hand or showing." | "Diese Karte ist nicht in Ihrer Hand und nicht offen." |
| `wrong-phase` | "Not now." | "Jetzt nicht." |
| `finished` | "The deal is over." | "Die Partie ist aus." |
| `not-held` | "You do not hold {honour}." | "Sie haben {honour} nicht." |
| `already-announced` | "Already announced." | "Schon angesagt." |
| `honour-forfeited` | "That honour is forfeited." | "Die Ansage ist verwirkt." |
| `duplicate-announcement` | "Announce each once." | "Jede Ansage nur einmal." |

(The strings are keys in `strings.json`, 8; `{led}` is `Herz / Hearts`,
`Karo / Diamonds`, `Kreuz / Clubs`, `Pik / Spades` or `Tarock / Tarocks`.) For
`must-follow-suit` and `must-trump` the cards in `allowed` are lit for 1.5 s.
The refusal also tells the opponent's `slip-yours` line (5): said at most once
a deal, on the first refusal, kindly.

3.2.2 The refusal is learned from the engine, not predicted. A hint text
(3.1.4) uses the same keys; a test checks the page never refuses a play on its
own.

### 3.3 Announcing the Trull or the four kings

3.3.1 `legal` lists `{action: "announce", honour}` whenever the human holds an
honour [R 11.3, 11.4]. The page shows an **announce chip** in the move bar for
each ("Troule ansagen" / "Announce Trull"; "Vier Könige ansagen" / "Announce four
kings"), always available while held, dimmed when not.

3.3.2 **The prompt before a held honour is played** [R 11.4]: when the human
selects a card of a held, unannounced honour (`SK T21 T1`; any king, when all four
are held), a prompt appears over the move bar: "Announce the Trull before you
play it?" with **Announce** and **Do not announce** (and a note in explanations:
"Unannounced, it is not paid"). Announce sends one `play` with
`announce: [honour]`; Do not announce sends the bare `play`, which forfeits the
honour (R 11.5, the engine). Cancel (Esc, or a tap away) returns to choosing.
Both are real choices; neither is the default (focus is on Announce, and the
default can be changed to "never ask" only by announcing in the settings,
7.1, which sends the announcement before each play automatically).

3.3.3 The prompt does not appear for an honour already announced, forfeited, or not
held (the engine says: `legal_announcements`). A card of an honour that is not
held (the Pagat while the Troule is split) plays without a prompt.

3.3.4 An announcement made on its own (the chip, no card chosen) sends `announce`
and leaves the human to choose; it is public (R 11.6).

### 3.4 Bidding

3.4.1 When `phase` is `bid` and `prompt` is `you`, the move bar shows two large buttons:
**"Ich nehme auf"** / **"I take it up"** (`take`) and **"Weiter"** /
**"Pass"** (`pass`). In English the tutor's gloss reads "Weiter = pass". They
are always the same two places, left and right, and are in the same place on
every device. With the tutorial on, a one-line aid under them says what taking
up risks (the declarer wins 3, loses 4 by default; the numbers come from the
settings in force, R 9.1).

3.4.2 The buttons carry the `legal` list: a button the engine did not list is not
shown. After a bid, the buttons vanish.

### 3.5 The sitting-length choice

3.5.1 The **new-sitting screen** (also reached by "New sitting" at the end and the
plus at the top) offers: **Length**, "3 Touren (6 deals)" / "6 Touren (12 deals)",
default 3 [R 15.2, 17.14]; the three rule settings (7.2); the opponent's skill
(7.3); the seat's "Hand" (the human's seat is chosen by lot unless the page
passes `human`); and **Deal** and **Cancel**. The screen's helper line says "A Tour
is two deals, one dealt by each."

3.5.2 The choice is kept as the next sitting's default (7.7). A sitting in
progress is never changed by the screen's controls except through
Settings, and the three rule settings and the length are not changeable
mid-sitting (they are shown, greyed, with "applies from the next sitting").

### 3.6 Extension prompt

As 2.7.2: "Noch eine Tour?" with **Noch eine Tour** / **Genug** buttons. The
engine asks seat 0 first (R 15.6). If the human is seat 1, the opponent's
answer comes first and a refusal ends the sitting at once, without the human's
prompt; the chip says "Your opponent has had enough." (key `ext-opp-no`).

### 3.7 The public mark

Public hand cards (pulled; and cards of an announced honour, R 13.5) carry the
mark of 1.5.7 on both hands. The legend is in the tutor's page on packets and
in the card's tooltip ("The other player has seen this card.").

### 3.8 Left-handed mode

Mirrors the placement of the move bar's buttons, the piles, the slate tab and
the dialogue side. The order of the two bid buttons and the extension
buttons is reversed (the more used, "Weiter" in practice, under the dominant
thumb: the same measurement Cassino used; the order for a right-handed player is
Ich nehme auf | Weiter, and the reverse left-handed). It does not change the
camera or the layout of the packets (the packets keep their 1-2-3 order left to
right, as the rules number them [R 2.4]).

### 3.9 Reduced motion

`prefers-reduced-motion` or the setting: no flights; each change cuts to its
layout after a 150 ms cross-fade; the reveal's held card is shown by a 700 ms
highlight outline at its place; the count's popups fade in place; the camera
cuts at the end. The HUD collapses its durations [K]. The order of events,
the speech and the holds are unchanged, so nothing is missed.

### 3.10 Keyboard and screen reader

3.10.1 The page is operable by keyboard alone. `Tab` order: the move bar; the hand
(one tab stop, arrow keys move within it, left/right, `Home`/`End`); the
packets (one stop, `1 2 3` choose a packet, or arrows); the slate tab; the
settings. Keys: `Enter` or `Space` choose or play the focused card; `A`
announce (first held honour); `T` take, `W` weiter; `S` the slate; `H` the hint;
`M` mute the tutor (6.3); `?` the keys list; `Esc` closes a sheet or cancels the
prompt. Keys are discoverable under `?`.

3.10.2 **The screen reader** gets a live region (`aria-live="polite"`) with one
sentence per event in the current language: the deal, each bid, each card
turned ("Your first packet turns the king of hearts: pulled"), each cover set
("Their second packet shows the queen of spades"), each play and trick won
("You lead the jack of diamonds; the opponent follows with the nine of
clubs; you take the trick"), each count popup ("Your count: eleven and
one-third"), each refusal, the settlement. The hand and packets are lists with
labelled items ("7 of clubs, from your hand"; "Queen of spades, cover of
packet 2, 3 cards beneath"). Buttons carry labels in the language. A "Where
am I" key (`I`) reads the position: trick number, whose turn, the counts so
far, the covers. The strings are keys (8); card names are built from the
code by language (en: "king of hearts"; de: "Herz-König"; Sküs, "Sküs").

3.10.3 Focus never lands on a dimmed card by default; after a refusal focus
stays on the refused card.

3.10.4 Targets are at least 44 × 44 CSS px on touch; text contrasts at 4.5:1 or
better; no information is carried by color alone (lit cards also lift). The
accessible card lists (3.10.1) are a visually hidden 1 px box, reached by key or
by a screen reader's own gestures, never by a finger on them; they are exempt from
the 44 px rule as long as they stay hidden. A finger taps the cards on the canvas.

## 4. The slate

The running account (Conto, [R 15.5, 15.9]) shown as a slate in the manner of
Haas's and Ulmann's Tafel [13-S7, 13-S1].

### 4.1 When it opens and closes

4.1.1 A **slate tab** (a small slate corner with a chalk stub, "Tafel" / "Slate")
is always at the top right. A tap, `S`, or a swipe opens the slate over the
table, not framing the table again (as Cassino's HUD ledger: it overlaps). It
folds on a tap outside, `Esc`, or `S`. In portrait it is a sheet from the
bottom.

4.1.2 It opens by itself, for 3 s or until tapped, at the end of each Tour
(2.7.1), at the extension prompt, and large at the sitting's end (2.7.3). It
never opens on its own during play or the count. The setting "Slate opens by
itself" (7.1) turns the first two off.

### 4.2 The look

A dark green-black board with a wooden frame (a CSS border), chalk white and
two chalk colors (yellow and pale blue for the two players), set in a
hand-drawn chalk face (a web font embedded in the page; Cassino's single-file
rule keeps it small), with a fallback of a system cursive that remains
legible at 14 px. Rows are ruled with a chalk line. Low-contrast chalk
(pale on dark) is held to a 4.5:1 ratio; a high-contrast setting renders pure
white and a flat board. No glow effect. Reduced motion draws each entry at
once.

### 4.3 The rows

4.3.1 Two columns, "Sie" / "You" and "Ihr Gegner" / "Your opponent"; the player
who deals the first deal is marked "Geber 1" in the left margin.
**A row per deal**: its number ("1"), the dealer's mark (a small "G" at
the row's margin), and for each player the **credits** of that deal (R 15.9):
cause and Points, in chalk shorthand, the cause named in words by language:

| Cause | de | en |
|---|---|---|
| `game` | Spiel | Game |
| `trull` | Troule | Trull |
| `kings` | Könige | Kings |
| `ultimo_won` | Ultimo + | Ultimo won |
| `ultimo_lost` | Ultimo − | Ultimo lost |
| `valat` | Valat | Valat |

Each row shows the deal's total for each player and, in the right margin, the
**Conto after the deal** (R 15.5), running, in chalk. A doubled deal has a
"×2" at the row's margin; a drawn deal "unentschieden / draw"; a thrown-in deal
"eingeworfen" with no entries; a void deal is not a row. A deal in progress has
a live row with "…".

4.3.2 A **Tour** is two deal rows followed by a ruled line with the Contos
(R 15.3, 15.5) and the Tour's number. The sitting's header is "3 Touren" or "6
Touren" and the settings in force ("2/3/4 · Ultimo 1 · beide passen: spielen").
An extension adds a double rule and "Verlängert" / "Extended".

4.3.3 **Points with their kreuzer**: every total is shown "n Pt" with its
kreuzer equivalent in a lighter chalk beside it ("7 Pt · 14 kr."), at 2
kreuzer a Point (R 15.8). Contos are gains only, so the columns do not sum to
zero; the **balance** (Conto A minus Conto B) and the amount owed are shown at
the foot ("Saldo +7 Pt: 14 kr." / "Balance +7 Pt: 14 kr."), with the direction
("Sie bekommen" / "You receive"; "Sie zahlen" / "You pay"). The kreuzer are
labelled "kr." in both languages. Currency beyond kreuzer is open (`PLAN.md`);
the slate shows kreuzer only, and the code keeps the display in one function
(`money(kreuzer)`) so the open decision (gulden and kreuzer, or krone) changes
one place.

### 4.4 The settlement

At `sitting_end` the slate is cleared of nothing; a ruled box below the last Tour
is written: the two Contos, the difference, the sum owed in kreuzer, and the
payer, one figure at a time (2.7.4). "Quitt." if square.

### 4.5 Its period voice

The slate's labels speak as the books did: "Tafel", "Conto", "Saldo",
"Schlußrechnung" for the settlement, "Touren", "Geber"; the English labels
are plain ("Slate", "Account", "Balance", "Settlement"). The opponent mentions
it with period words in the lines of 5; the labels are not jokes.

### 4.6 It is a reconstruction

The Touren, the running account and the settlement are Tarock customs
described for multi-hand Tarock; no source applies them to Strohmandl
([R 15], [14 D], [13 headline 6]). The tutor says so **once**, on the tutorial
page "Touren and the slate" (6.2), in one line ("This is how an evening of
Tarock was kept. No source says Strohmandl was kept this way: we have borrowed
it.") and in the slate's tooltip. It is not repeated.

## 5. Talk

The opponent speaks from the phrase bank (`web3d/words.json`, [PH]), generated
by `tools/phrases.py`. The kindness rule is in the bank and tested there. The
engine's events and the table's state decide when; the bank decides what.

### 5.1 Events and groups

`-mine` is the opponent's event, `-yours` the human's [0.6]. Group ids are those
of [PH].

| Moment (event) | Group |
|---|---|
| sitting begins | `greet` |
| length chosen | `touren-3` or `touren-6` |
| first dealer (the cut) | `cut` |
| `deal_start` (opponent deals / human deals) | `deal-mine` / `deal-yours` |
| `void_deal` | `void-deal` |
| `doubling_applied` | `doubling-applied` |
| `bid` (opponent / human) | `bid-take-mine` / `bid-take-yours` |
| `pass` (opponent / human) | `bid-pass-mine` / `bid-pass-yours` |
| `both_pass` (`play`) | `both-pass` |
| `thrown_in` | `thrown-in` |
| `reveal_start` | `reveal-begin` |
| `card_pulled`: a king (opponent's / human's) | `pull-king-mine` / `pull-king-yours` |
| `card_pulled`: a tarock other than the three below | `pull-tarock-mine` / `pull-tarock-yours` |
| `card_pulled`: `SK` | `pull-sku` |
| `card_pulled`: `T21` (the Mond) | `pull-mond` |
| `card_pulled`: `T1` (the Pagat) | `pull-pagat` |
| a chain of three or more pulls from one packet | `run-mine` / `run-yours` |
| a reveal pulling nothing (all three covers) | `nothing-mine` / `nothing-yours` |
| the human is slow (idle 25 s, again at 60) | `slow` |
| `trick_won` | `trick-good-mine` / `trick-good-yours` (won with a tarock or a king, or a high gain) ; `trick-bad-mine` / `trick-bad-yours` (a king or a tarock lost to a higher) |
| `SK` played | `sku-played` |
| `T21` taken by `SK` (Mondfang) in the trick | `mond-caught-mine` (opponent's `SK` takes) / `mond-caught-yours` |
| `pagat_ultimo` | `ultimo-won-mine` / `-yours`, `ultimo-lost-mine` / `-yours` |
| `announcement` | `trull-mine` / `trull-yours`, `kings-mine` / `kings-yours` |
| `honour_forfeited` | `honour-forfeited` |
| `valat` | `valat-mine` / `valat-yours` |
| count begins (just before the first card turns) | `count-start` |
| count ends (after the last popup) | `count-close` |
| `game_scored` | `result-won-mine`, `result-lost-mine`, `result-won-yours`, `result-lost-yours` ("mine" = opponent's); a `draw`: `draw` |
| `doubling_lapsed` | `doubling-lapsed` |
| `tour_end` (not the last / the last) | `tour-end` / `tour-last` |
| `extension_offered` | `extend-ask` |
| `extension_decided` | `extend-yes` / `extend-no` |
| `sitting_end` | `settle-opp-pays` / `settle-player-pays` / `settle-even`, then `farewell` |
| the opponent's own error | `blunder-own` |
| a refused card (3.2) | `slip-yours` |
| luck observed (below) | `luck-yours`, `luck-mine`, `luck-bad-mine` |
| the tutor speaks / is muted | `kibitz` / `kibitz-mute` (the kibitzer's voice, not the opponent's, 6) |

5.1.1 **Luck talk** (`luck-*`): after a deal's result, if the winner's take is 120
thirds or more (a wide win), or the deal's first reveal pulled six cards or
more for one side and none for the other. The luck lines refer to luck, the
run of play and the score only, never to the person (the kindness rule).

5.1.2 `nothing-*` is spoken at most once per side per deal. `run-*` replaces the
`pull-*` line that would have been spoken for the run's last card.

5.1.3 The group a moment takes is chosen by the moment's data; a line is chosen
from the group by a shuffle bag so the same line does not repeat before the
group is exhausted (`kit/js/bag.js`).

### 5.2 Timing

5.2.1 A line is said when its event is *seen*: a pull as the card is raised for
the human; a bid as the badge rises; a play as the card lands; a trick as the
gather begins; an announcement as the chip rises. Lines are said in the order of
their moments. One speaker's lines at one moment are one box (`chunk`).

5.2.2 **Calls and chatter.** The *calls* are the lines that carry the game:
`greet`, `touren-*`, `cut`, `deal-*`, `void-deal`, `doubling-applied`, `bid-*`,
`both-pass`, `thrown-in`, `reveal-begin`, `trull-*`, `kings-*`, `ultimo-*`,
`valat-*`, `result-*`, `draw`, `tour-*`, `extend-*`, `settle-*`, `farewell`.
Everything else is *chatter*: the `pull-*`, `run-*`, `nothing-*`, `trick-*`, `sku-played`,
`mond-caught-*`, `luck-*`, `slow`, `blunder-own`, `slip-yours`,
`honour-forfeited`, `doubling-lapsed`, `count-start`, `count-close`. Chatter
yields to calls, by the kit's rules (`dialogue.js`): a chatter line is left
out if it would make a call of a later moment more than **SLACK (250 ms)** late,
or if it would come more than **LATE (2000 ms)** after its own moment. A chatty
table plays at the pace of a quiet one.

5.2.3 **Nothing is said during the count.** The window from the first card turned
up to the last popup is quiet (`quietWindow`, `muted`): lines that would be
said in it are dropped. `count-start` is said as the last trick is gathered,
ending before the first card turns; `count-close` after the last popup;
`result-*` after that.

5.2.4 The opponent's first play waits for the opening's calls (`openingWaits`,
as Cassino): "I take it up" is heard before the first lead.

5.2.5 `slow` is spoken only while the human is idle and the cards are at rest.

### 5.3 Cross-group repeat control (a `PLAN.md` open item)

The bag in `kit/js/bag.js` avoids repeats within a group only. A proverb or a
source (Wander) can appear in several groups, so the same saying can recur
across groups in one sitting.

5.3.1 Each text in `words.json` carries a **`sig`**: its source-and-locator
(for example `Wa Glück 360`, `Ve`, `Ra`), as in [PH]. `tools/phrases.py` emits
it; a test fails if a text has none (a table's own text, `T`, takes its key as
its `sig`).

5.3.2 The talk keeps a **sitting-wide record of `sig`s spoken**, saved with the
sitting (9). A line whose `sig` was spoken in the last **3 deals** (or, for the
English and German forms of one source, the same sitting) is passed over if
another line of its group is available. When every line of a group is spent, the
oldest is allowed again. A calls-only line (5.2.2) is not subject to it, because
its moment is rare.

5.3.3 A `sig`-less test (a bank with a duplicate `sig` across groups) fails the phrase
test with the pairs listed, so the bank's content is made better, not the
runtime gentler: the open item's other half, **more wordings** in `pull-*`,
`nothing-*` and `trick-*` (five or more), is in the bank's package, not the
table's.

5.3.4 A test plays 5 sittings and checks that no `sig` is spoken twice within a
sitting's 3-deal window where an alternative existed.

### 5.4 The setting "Table talk"

All / Calls only / None (`talk: "all" | "calls" | "none"`, 7.1): **All** says
calls and chatter; **Calls only** says the calls (5.2.2) and nothing else;
**None** says nothing, and the chips and badges still appear. The tutor's
voice (6) is separate, controlled by its own setting.

### 5.5 Where a line is said

By its speaker's hand: on a desktop the opponent's box is by their hand (the far
edge), the human's lines (when the table speaks for the human: the cut, the
bid, the announce) by the human's hand, to its right; on a phone the box takes
the first free place (1.5.5). The box has a tail pointing at its speaker. A line
is shown for `saying(words)` plus 600 ms. Phone portrait: the box is above the
hand and below the move bar's top, never over the hand or move bar.

### 5.6 The opponent's name

"Your opponent" / "Ihr Gegner" everywhere; it has no portrait, no name, and the
sitting's end has no court card (Cassino's end figure is not carried over). Its
voice is the bank's: formal, dry, period, kind. The opponent does not address
the human by name.

## 6. The tutor: a period kibitzer

The tutor is a **Kiebitz**, the onlooker who sits at the player's shoulder
and offers advice nobody asked for: the stock figure of the Viennese card table
([05c headline 1]). In a two-player game he is the third at the table. His
voice is dry, brief and kind (the kindness rule: luck, speed, the score and
the run of play, never the person). He is addressed as "Kiebitz", is never
named, and has no portrait: a small speech tab at the player's shoulder
(bottom left, beside the hand) with a paper-and-pencil icon.

### 6.1 The mute

**"§ 13: Kiebitz, halt's Maul!"** is the tutor's mute button, as the research
suggests ([05c headline 1], [PH `kibitz`, `kibitz-mute`]). It is a button in
the speech tab with the text in German, and in English
"§ 13: Kibitzer, shut your trap!" in the title/tooltip with the button label
"§ 13". A tap mutes the tutor for the sitting (hints, tips, pages, review
prompt); the tutor answers with `kibitz-mute` once, then says nothing. A tap
again, or the setting (7.1), un-mutes. **Muted, the tutor does not speak, and
no tutorial page opens by itself**; the Help pages remain reachable from the
menu. The screen-reader sentences about the game continue (they are the
game's). The mute is kept (7.7). Muting is never penalised or commented on
beyond the one line. When the human first opens the tutor, the intro line is
`kibitz`.

### 6.2 Tutorial pages

6.2.1 Pages are Markdown in `web3d/tutorial.md` (de) and `web3d/tutorial.en.md`
(en), parsed by `kit/js/tutorial.js` `parseTutorial(markdown, keys)`. Each is
written for its moment, for a new player: what is about to happen, only what is
needed to play it, the interface named as it is. Sections of one-line bullets,
no preamble; every line at most 25 words (a test holds this, and a jargon list
out of the pages, as Cassino's).

6.2.2 **A page opens at the first occurrence of its concept** in a real sitting,
each once (`seen` in the preferences, 7.7), and only while the tutor is on and
not muted. The page opens when the cards are at rest, never in motion. A
"Not now" dismisses it without marking it seen only if the human chooses
"Remind me"; otherwise it is marked seen. Pages are reachable any time from the
menu ("Help").

**Pacing.** A page is due no later than the first decision it explains is
asked, and says only what that decision needs; detail needed later goes on the
later page (`web3d/test/tutor.test.js` pins each decision to its page; the
audit is `docs/review/tutorial-pacing.md`). Pages due at one rest follow in the
table's order, each after the last is closed; the page opens before the action
bar and the prompts are drawn, so it is on screen when the question is asked.

| Page key | Opens at | Says |
|---|---|---|
| `packets` | the first `dealt` | what the three packets are, the face-up card on top, the count of hidden cards beneath, that you and your opponent each have three, that a cover is yours to play |
| `bidding` | the first `bid` turn (the human's, or the first `bid` event seen) | who speaks first; "Ich nehme auf" or "Weiter"; the declarer; the card values (the 70 Points) and the 36 needed; what is won (3 / 4 by default); both pass plays a simple game or throws in, by the setting |
| `the-pull` | the first `reveal_start` (no decision is asked: the cards turn themselves) | the cards are turned in turn; a tarock or a king goes to the hand; the turning continues; a low card stays; the last card of a packet always goes |
| `playing` | the human's first play turn | how a card is played (tap, then tap again or Play); the leader; the trick winner leads; the HUD |
| `following` | the first human play turn with a card led | the follower must follow suit, else trump, else any; covers count as hand cards for this; no obligation to win |
| `from-a-packet` | the first human play turn with a cover legal (or the first cover played) | afterwards the card beneath is turned and may be pulled; the opponent plays before seeing it |
| `honours` | the first time the human may announce, or the first `announcement` | announce the Trull (Sküs, Mond, Pagat) or four kings before the first card of it is played; 1 Point; paid even if lost; staying silent is allowed, unpaid; the prompt (3.3) |
| `thirds` | the first `trick_won` (the tally begins to run), or the first `count` | the count in thirds: each card its value less two-thirds (a group of three: the sum less 2) [RULES 8.1]; 36 of 70 is 35⅔; 35⅓ loses; the HUD shows the tally |
| `pagat-ultimo` | the first time the human holds `T1` at the last 5 tricks, or the first `pagat_ultimo` | the Pagat in the last trick wins or loses a Point; it is quiet, never announced; the Pagat is the lowest tarock |
| `scoring` | trick 21, or the first `count` / `game_scored` | what the deal pays: the game (2 / 3 / 4), the honours, the Pagat, all paid between the two players [RULES 9.2, 9.3]; credited to the slate; the evening settled at 2 kreuzer a Point [RULES 15.5, 15.7] |
| `draw` | the first `draw` | a simple game that ends 35 : 35 is undecided; the next deal counts double |
| `touren-slate` | the first `tour_end`, or the first slate opening | the Touren, two deals each; the slate; the note that this is a reconstruction (4.6), said once |
| `extension` | the first `extension_offered`, or the human's extend prompt | one more Tour needs both players' consent |

The pages are authored in English and German at the same time (11); each is a
key in the tutorial file. The `keys` argument of `parseTutorial` is the list
above.

6.2.3 A page can also be opened from the hint chip (6.3) and from the review.

### 6.3 Hints and explanations

6.3.1 The tutor's hint is the engine's `strohmandeln_hint()`: `{hint, reason}`
where `hint` is `null` when unavailable [P]. The engine returns message
keys with parameters (a hint is `{key, params}`, to be added to `docs/MESSAGES.md`
by the opponent package; until then `reason: "unavailable"` hides the button).
The page renders the key in the current language; it never builds prose from
cards itself.

6.3.2 **Hint** (the `H` key, the chip's "Hint"; setting "Hints", 7.1): shows, under
the prompt, one sentence ("Lead the Pagat? Too early: it is your ultimo card."
by key) and lights the suggested card with a thin gold outline. Never plays
for the human. Asking costs nothing and is never counted.

6.3.3 **Explain** (setting "Explanations", 7.1): after each of the human's plays and
the opponent's replies, a one-line explanation under the prompt names what
happened, from the engine's message keys ("Trump: your opponent has no
hearts."). The helper text of 3.1.4 and the hint and explanation share this
line; off, the line is empty. The game's result is said whatever.

6.3.4 Hints and explanations are asked only with the cards at rest (the director's
`rested`), once per position, never in an animation. If the muted tutor is
asked for a hint, the chip says "§ 13", unmuting only by the human's tap.

### 6.4 The end-of-deal review

6.4.1 After each deal's `game_scored` (not after a void deal or a throw-in), a
small "Review" chip appears at the tutor's tab. It opens a dialog, **"How the
deal went"**: one to three bullets, each with a bold lead, from the engine's
review (message keys with parameters, like the hint): the count against 107; the
place the deal was decided (the trick, the honour, the Pagat); one thing to
try. It is gentle and about trends, not a list of faults (Cassino's rule).
A folded "How is this worked out?" has the method.

6.4.2 At the sitting's end the chip says "Review the sitting": the same dialog over
the whole sitting, three bullets. Not shown when muted.

6.4.3 The heavy calls (the review of a record) run in a Web Worker as Cassino's
(`tutorworker`), falling back to the main thread, `?noworker` forces it. The
engine's review of a record is stateless; the worker has its own instance of
the wasm. The review is asked for once the deal's ending has settled.

6.4.4 What the tutor keeps: the last 30 sittings' summaries (`strohmandeln.progress`,
version 1, try/catch) for trends, as Cassino's `progress.js`; exportable and
clearable in Settings; absent storage means a sitting alone.

### 6.5 Voice

Kibitzer lines are short and period: "Nicht dreinreden, aber ..." (kept to the
bank's `kibitz` group for the intro and the mute, and to the tutorial pages'
own text for the rest). The kibitzer says he is a kibitzer and admits it: "A
kibitzer may buy supper, not advise" is his; tips are offered as "if I may".
He is never the opponent's voice and is never in a dialogue box of the
opponent's: the tab's bubble is a different shape (a ruled paper note).

## 7. Settings

### 7.1 The menu

Under the cog, in sections. `localStorage` key `strohmandeln.prefs`, version
numbered (7.7).

| Setting | Values | Default | Applies |
|---|---|---|---|
| Rule: both pass | play (simple game) / throw in | play | next sitting |
| Rule: game values | 2/3/4 · 1/2/3 · 1/2/4 | 2/3/4 | next sitting |
| Rule: Pagat ultimo value | 1 / 2 Points | 1 | next sitting |
| Sitting length | 3 / 6 Touren | 3 | next sitting |
| Opponent skill | 1 to 4 in halves (1, 1.5, ... 4) | 3 | next sitting |
| Speed | Slow / Normal / Fast / Instant | Normal | now |
| Table talk | All / Calls only / None | All | now |
| Tutor | On / Off (off: no pages, no tips, no review chip) | On | now |
| Hints | On / Off (the hint chip) | Off | now |
| Explanations | On / Off | Off | now |
| Announce automatically | Off / On (an honour is announced before it is played, with no prompt) | Off | now |
| Slate opens by itself | On / Off | On | now |
| Faces size | Auto / Classic / Jumbo | Auto | now |
| Left-handed | On / Off | Off | now |
| Reduced motion | System / On / Off | System | now |
| Language | English / Deutsch | English | now |
| Table top | the kit's surfaces (a random choice by default) | random | now |

The rule settings and the length apply to the next sitting: shown, greyed, in a
sitting in progress. A mid-sitting change is never silent; the screen says
"from the next sitting."

### 7.2 The three rule settings

As R "Authority and settings": `both_pass`, `values`, `ultimo`. Each has a
one-line explanation beside it, sourced as in the rules (`Werner 2/3/4`,
`Dummett 1/2/3`, `Ulmann 1/2/4`). The new-sitting screen shows the three with
the defaults; picking another is never penalised, and no other variant is
offered (CLAUDE.md: no variants without the maintainer).

### 7.3 The opponent's skill

A dial of 1 to 4 in halves, as Cassino's; passed as `level` in `new` [P] (an
integer for now: the dial maps to the engine's levels when the opponent
package defines them; until then the wasm carries the opaque value). The
label under the dial says in a sentence what the level plays like, once the
package defines it.

### 7.4 Speed

Scales every hold and flight of 2 and the dialogue's timing; the count's popups
are scaled by `scalePace` [K]. Instant waits for nothing (2.8).

### 7.5 Language

7.5.1 English or German; the default is English. `navigator.language` starting with
"de" offers German once in a one-line banner; it never switches by itself.

7.5.2 **The switch re-renders everything, mid-sitting, without losing state.** The
engine holds no language; so a switch changes only the page:

- every interface string by key (8), including buttons, chips, banners, the slate's
  labels, the refusal chips (3.2) and the screen-reader sentences (3.10.2);
- the dialogue: a line already being said is redone in the new language from its
  key; lines to come take it;
- the cards' faces: the court labels swap (K Q C J / K D C B) in the faces' texture
  (re-drawn lazily; textures for phones at 384 px or less, built lazily, the
  `PLAN.md` open kit item), so the table's cards, hand, packets and piles show
  the new labels at once;
- **the HUD is rebuilt** (the kit's `createHud` is called anew with the new
  `defineGame` strings; `hud.reset` is given the same ledgers, so every number is
  kept): the HUD keeps no language [K] (open kit item);
- the tutorial pages (the parsed pages of the other language are swapped in, the
  open page keeping its scroll and its place), and the tutor's chips;
- the slate (rows re-rendered from the same record);
- the document's `lang` attribute and the title.

The cards' positions, the game state, the dialogue's bag, the timeline and the
camera are untouched. The Tarock numerals, "SK" and codes are language-free.
Names of the suits in text follow the language (hearts / Herz, diamonds /
Karo, clubs / Kreuz, spades / Pik).

7.5.3 A switch during a held card (the reveal) or a popup leaves the animation
running; the next text drawn is in the new language.

### 7.6 Faces

Our own faces, drawn at runtime (`web3d/src/tarockfaces.js`, [PLAN]); the court
labels (K Q C J / K D C B) are the language's. "Jumbo" enlarges the ranks;
"Auto" picks large on phone and tablet.

### 7.7 Persistence of settings: versioned localStorage with try/catch

7.7.1 Keys: `strohmandeln.prefs` (settings), `strohmandeln.sitting` (the saved
sitting, 9), `strohmandeln.progress` (the tutor's summaries, 6.4.4),
`strohmandeln.seen` (tutorial pages seen, with the prefs' version).

7.7.2 Every read and write is in a `try/catch`. A missing, full or blocked store
(a private window, cleared site data) means defaults, a one-time notice, and a
page that works as a single sitting.

7.7.3 The prefs record is `{v, ...all settings, defaults too}`, `v` the version of
the defaults it was kept under, as Cassino's: a default changed later is not
mistaken for the person's choice. A store written by a newer page (`v` above
this page's) is read as empty and left as it is, and said once. A record whose
fields are of the wrong type is read field by field; bad fields take defaults.

7.7.4 URL overrides for tests: `?seed=7`, `?lang=de`, `?speed=instant`, `?talk=none`,
`?noworker`, `?touren=3`, `?human=0|1` (each ignored in a real sitting unless the
test flag is set).

## 8. Strings

8.1 Every interface string has a key. `web3d/strings.json` is
`{ "key": {"en": "...", "de": "..."} }` in one file (the phrase bank and the
tutorial pages are the other two sources of text and have their own files, `words.json`
and `tutorial*.md`, already keyed). Parameters are `{name}` slots, filled by
`fmt(key, params)`; plurals are separate keys. The page has no other literal
text: card names, suits, the thirds' fraction glyphs and the refusals' texts are
all in it.

8.2 **Keys are namespaced**: `ui.*` (buttons, menus), `chip.*`, `hud.*`, `slate.*`,
`refusal.*` (one per key of `docs/MESSAGES.md`), `hint.*`, `review.*`, `sr.*`
(screen reader), `card.*` (names), `settings.*`, `error.*`.

8.3 **A test checks both languages are complete** (`web3d/test/strings.test.js`):
- every key has a non-empty `en` and `de`;
- no key has extra languages or missing ones;
- every `{slot}` in `en` is in `de` and vice versa;
- every key used in the page's source (`t("...")`, `data-i18n`) exists, and every
  key in the file is used (a dead key fails);
- every refusal key of `docs/MESSAGES.md` has a `refusal.*` string;
- every cause of `credits[].cause` has a `slate.cause.*` string;
- every tutorial page key (6.2) has both pages;
- `words.json`: every text has `en` and `de`; every group of [PH] is present;
- no literal user-visible English or German text in `web3d/src/*.js` outside
  `strings.json` (a regexp lint over string literals in DOM-writing calls);
- the jargon and unkind-word lists (CLAUDE.md's kindness rule) hold for strings
  as for the bank.

8.4 The German is the period wording where the bank gives it, else plain modern
Austrian-leaning German ("Sie", "Weiter", "Ich nehme auf", "Stich", "Tarock",
"Sküs"); no dialect spelling outside the opponent's lines.

## 9. Persistence

9.1 **Save and restore a sitting.** After every accepted action (and each opponent
step), the page stores `strohmandeln_export()` [P] (the record: seed,
settings, Touren, every move) in `strohmandeln.sitting`, with the page's own
extras: the language, the talk's `sig` record (5.3.2), the tutorial's `seen`
keys and the human's seat; try/catch, a record over 200 KB is refused and
the sitting continues unsaved with a one-time notice.

9.2 **On load**, if a saved sitting exists and is not `over`, the welcome offers
**Continue the sitting** and **New sitting**; Continue calls
`strohmandeln_import` [P]. A failed import (`bad-record`) leaves the old
sitting, says so in a sentence, and offers New sitting. The client rebuilds from
`state` (whose `events` hold the deal in progress, [P]), by one direct
transition (2): no replay of the deal. An imported sitting continues with a
freshly seeded opponent [P].

9.3 **Export and import as a file.** Settings has "Save the sitting" (downloads
`strohmandeln-sitting.json`: the export plus the page extras, `{v, protocol,
export, extras}`) and "Load a sitting" (a file chosen; a bad one is refused in a
sentence and changes nothing; over 1 MB refused).

9.4 **A finished sitting** is cleared from `strohmandeln.sitting` at the farewell
(its summary goes to the tutor's progress, 6.4.4).

9.5 **Offline single file.** `web3d/strohmandeln.html`, built by `kit/build.py` from
`web3d/game.toml`: the bundle, the wasm, the faces, `words.json`,
`strings.json`, the tutorial pages, the chalk font and the HUD's CSS inlined; no
network request (the browser test blocks them all). Target size under 10 MB, the
guideline in `game.toml`; the weigh-in names the kit's, three.js's and the
game's bytes. `kit/build.py --check` fails if the committed page differs.

## 10. The browser test plan

`web3d/test/browser.py`, Playwright with the system Chromium, headless; all
network blocked; software compositing for CSS animation (`--disable-gpu-compositing`).
Modelled on Cassino's. Each check is named, runs in a fresh context, and fails on
any console error or network request. `?seed` and `?speed=instant` fix the
sittings where a check does not need the motion. Screenshots and strips of the
motion are saved to `web3d/test/out/`.

### 10.1 Page and layout

- `offline_opens`: the page opens from disk; the table is drawn; no network request; no
  console error.
- `layout_landscape`, `layout_portrait`, `layout_touch`: at 1280×720, 390×844,
  1024×768 portrait (tablet): the deal is dealt; zone rectangles read from the
  page's debug hook do not intersect (1.5.1); a cover's index is unobstructed
  (1.5.2); each badge equals `hidden` (1.5.4).
- `cover_legibility`: a cover's screen size is within a tenth of a hand card's.
- `no_face_leak`: at every rest of a whole sitting by clicking, no mesh shows a face
  the view does not give; `strohmandeln_audit` agrees (0.2).
- `phone_row`: portrait hand is a row, scrolls, never wraps; 15 cards reachable.
- `window_resize`: a resize mid-deal re-lays the table without losing a card.

### 10.2 Choreography

- `cut_and_deal`: the cut shows two cards, the deal makes 3 × 5 then six packets of 4
  (counts read from the page at rest: hand 15, packets 4 × 6).
- `bid_badge`: a take puts the declarer's badge up; a pass shows its chip and fades.
- `reveal_shows_cards`: each pulled card is held face up toward the human for at
  least the hold before it enters the hand (a frame sample); a chain of 3 or
  more in a seeded sitting pulls in order; a low card stays as the cover.
- `last_card_pulled`: a seeded packet whose fourth card is low pulls it; the empty
  packet's stack fades.
- `play_hand_vs_packet`: a play from a hand and from a cover land alike; the packet
  play leaves its stack top bare and the badge unchanged until the replacement.
- `replacement_after_trick`: the card beneath is turned only after the gather;
  leader's replacement before the follower's when both played covers.
- `trick_gather`: both cards go face down to the winner's pile; the pile's
  thickness grows by one.
- `last_trick_ultimo`: a seeded Pagat-ultimo sitting shows the ring and chip,
  won and lost.
- `count_thirds`: the count lays each take in threes; the popups' sum equals the
  HUD tally; `35⅔` shown for 107, `35⅓` and its "not 36" chip for 106 (seeded
  deals); the two takes sum to 210 thirds.
- `count_quiet`: no dialogue box appears from the first count card to the last
  popup.
- `game_scored_banner`: banner and slate row at once; the next deal waits for it.
- `draw_and_doubling`: a seeded draw shows the draw banner and the doubling marker,
  applied to the next deal, then gone.
- `void_deal`: a seeded void deal shows the no-tarock tag and redeals with no Tour
  counted.
- `throw_in`: under `both_pass = throw_in`, a double pass collects the cards, moves
  the dealer and counts the Tour.
- `tour_end_and_extension`: the Tour's rule on the slate; at the last Tour the prompt
  "Noch eine Tour?" appears with both buttons; accept extends and the Contos
  carry; refuse ends.
- `settlement`: the slate's settlement figures equal `sitting_end`
  (`kreuzer`, payer); "Quitt" for square.
- `farewell_once`: one farewell line; then "New sitting" is offered.
- `motion_profile`: sampled frames of a played card show a nonzero initial speed
  (the jerk), an easing down and no card below the table.

### 10.3 Interaction

- `legal_only`: with the page's lit set read from the DOM, it equals the engine's
  `legal` plays in 200 seeded positions.
- `refusal_message`: a tap on a dimmed card shows the refusal chip, in English and
  in German, with the engine's key; nothing changes; focus stays.
- `announce_prompt`: selecting a held honour card prompts; Announce pays the point
  and appears on the slate; Do not announce forfeits (the engine's
  `honour-forfeited` on a later try); the prompt is absent for an unheld honour.
- `announce_chip`: the chip announces without playing; it is refused when not
  held (`not-held`).
- `bid_buttons`: "Ich nehme auf" / "Weiter" in German and the English
  equivalents; only `legal` buttons shown.
- `length_choice`: 3 Touren makes 6 deals, 6 makes 12 (the HUD's Tour counter).
- `keyboard_sitting`: a whole sitting played by keys alone (choose by arrows,
  `Enter`, `A`, `T`, `W`, `S`, `H`, `M`, `?`, `Esc`).
- `screen_reader`: the live region has a sentence for each event in the current
  language; the lists have labels; the sentence for a refusal is announced.
- `left_handed`: the move bar, piles and slate mirror; the packets stay 1-2-3 left
  to right.
- `reduced_motion`: with the setting, no card in flight at any frame; the same
  sentences and results.
- `touch_targets`: every visible control on touch is at least 44 px; the visually
  hidden accessible card lists (3.10.4) are exempt, and the check asserts they are hidden.

### 10.4 Slate, talk, tutor

- `slate_open_close`: the tab, `S`, outside tap and `Esc`; auto-open at Tour end
  and at the extension; never during play or the count.
- `slate_rows`: rows per deal with causes, "×2" on a doubled deal, "draw",
  "eingeworfen"; Points with kreuzer; the Conto after each deal equals the
  engine's `contos`; the balance and amount at the foot.
- `slate_reconstruction_note`: the "borrowed" note appears once in a sitting.
- `talk_events`: in a seeded sitting each event of 5.1 that occurs shows a line of
  its group (read from the bank).
- `talk_setting`: All, Calls only (only calls of 5.2.2), None (no box).
- `talk_timing`: a chatter line never delays a call by more than SLACK; none is
  more than LATE late.
- `talk_cross_group`: over 5 sittings no `sig` is repeated in a 3-deal window
  where an alternative existed.
- `opponent_unnamed`: no string or DOM text names the opponent other than "your
  opponent" / "Ihr Gegner".
- `tutor_pages`: in a seeded sitting each page opens at its first occurrence, once
  (`packets` at the deal, `the-pull` at the reveal, ... `extension`); not when
  muted or the tutor is off; each page is in the current language.
- `tutor_mute`: "§ 13" mutes (one line, then silence); no page, hint or review chip;
  unmute restores them.
- `hint_and_explain`: the hint chip lights a card and shows a rendered key; explanations
  off leaves the line empty.
- `deal_review`: after a scored deal the chip opens "How the deal went" with up to
  three bullets; none after a void deal or a throw-in.

### 10.5 Settings, language, persistence

- `settings_rules`: the three rule settings and the length apply to the next sitting
  and are greyed mid-sitting; `values` change the credits' amounts on the next
  sitting.
- `skill_speed`: the dial and the speed keep and apply.
- `language_switch`: mid-sitting (during a deal, after a reveal, at the count)
  English to German and back: every visible string, the faces' court labels,
  the HUD (rebuilt, numbers kept), the slate, the open tutorial page and the screen-reader text
  change; the state (hands, packets, trick, tallies, Contos) is identical before and
  after; no console error. A scan finds no English string left after a switch to
  German (apart from numerals).
- `strings_complete`: the test of 8.3, run once in the page's context too.
- `prefs_storage`: prefs persist across reload; blocked storage (throwing
  accessors) gives defaults and a notice and still plays; a newer-version record is
  left as it is.
- `save_restore`: reload mid-deal offers Continue; the restored state equals the
  saved one (hands, packets, tallies, Contos), the talk's record carries, the
  opponent continues; a corrupt record is refused with the old sitting kept.
- `export_import_file`: save to a file and load it back; a bad file is refused.
- `single_file`: the built page has no external reference (a scan), is under the
  size guideline, and equals a fresh build (`build.py --check`).

### 10.6 Whole sittings

- `full_sitting_3`: a 3-Touren sitting played by clicking from the cut to the
  farewell, checking at every rest no leak, at every deal's end the HUD and
  slate against the engine's `contos`, at the end the settlement.
- `full_sitting_6_extended`: a 6-Touren sitting with an extension accepted.
- `phone_sitting`: a sitting played by tapping in a 390×844 portrait window, and
  one held sideways.
- `watch_reload`: a sitting reloaded after each phase (bid, reveal, play, count,
  extension) restores to the same state.

### 10.7 Running the suite

`bin/browser-full [a,b,c]` runs the checks one per browser process, each under
`CHECK_TIMEOUT` (default 600 s), and prints one summary line; `browser.py --only=a,b`
runs named checks in one process; the gate runs `--quick`. Checks live in
`browser.py` (10.1), `browser_scenarios.py` (10.2, 10.3) and `browser_aids.py`
(10.4, 10.5).

The software GPU spends about a second a frame, so the checks that read state and
the DOM, not pixels, turn the draw calls into no-ops (`NO_DRAW`), and the drivers
`drive` and `legal_only` loop inside the page, one round trip a sitting. Talk checks
run at Normal speed on the test's clock (`?manual`): the Instant speed leaves the
talk out (2.8). `keyboard_sitting` plays a whole 3-Touren sitting by keys (about two minutes). `talk_repeat` bounds the repeats
rather than forbidding them: the talk's record also holds lines the speaker dropped,
which the page never shows (the exact rule is in `talk.test.js`).

Wall time of the whole suite on the 2-core VM, load from other work included:
53 checks took 3057 s (51 minutes) on 15 Oct 2026, of which 1200 s were two checks that timed out (no_face_leak, since sped up to 103 s, and keyboard_sitting, open); the rest ran in 1857 s. Since fixed: keyboard_sitting (a whole sitting by keys, 2 min), announce_prompt (the auto-opened slate covered the cards), touch_targets (3.10.4).

## 11. Order of work

Not a plan of days; the dependencies. Each step ends with its tests and a commit,
and `PLAN.md` records it.

1. Scaffold `web3d/` (`game.toml`, entry, template), `strings.json` and the strings
   test, `prefs.js` after Cassino's pattern, the zones and `layout` with its
   tests (1).
2. The engine wrapper and the state drawn: a dealt sitting at rest in all three
   arrangements; the audit test.
3. The choreography of the deal, bid, reveal, play, trick and replacement (2.1 to
   2.2); the reducer test against the engine's own states.
4. The interaction (3), the refusal chip, the announce prompt, the keyboard and
   the screen reader.
5. The count with the kit's HUD and popups, the banner, the draw, the void deal
   and the throw-in (2.5, 2.6).
6. The slate, the Tour's end, the extension, the settlement and the farewell
   (2.7, 4).
7. The talk (5), with the `sig` addition to the bank and the cross-group test.
8. The tutor (6) after the engine's hint exists; its pages, the mute, the
   review.
9. Languages (7.5, 8): the strings, the faces' court labels, the HUD rebuild,
   the tutorial pages in German, the language-switch test.
10. Persistence (9) and the build; the browser test (10) throughout, each check
    added with its feature.
