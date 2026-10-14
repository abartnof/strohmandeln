# Strohmandeln: A Literature Review for Game Development

*Prepared 14 October 2026; revised the same day (v2) to follow the canonical ruleset of note 14, the session of note 13 and reviews RF to RI (changes in `research/review/synthesis-v2-changes.md`). Scope: history, rules and their variants, terminology, table talk, social and cultural life, the game in the languages of the Habsburg empire, related games, strategy and numbers, the physical cards, digital implementations, and what this project can reuse from the earlier games in the series (Piquet, Bezique, Cassino). Written for a team building a video-game version: a single offline web page with a computer opponent that talks in a period voice and a tutor for players who have never heard of the game.*

---

## How to read this document

- **Citation rule.** Every factual statement carries a citation in square brackets, pointing into the research notes in `research/`, where each source is listed in full (URL, page, and the local copy under `research/sources/`).
  - `[NN-S#]` is source S# of note NN (e.g. `[03-S12]`). Notes 05b, 05c and 05d number their sources `LEX-S#`, `LIT-S#` and `POLY-S#`, cited here as `[05b LEX-S12]` and so on.
  - `[NN §x]` points to a section of a note where an argument or table is developed.
  - `[14 rule n]` points to row n of the table of canonical rules in note 14; `[14 §…]` to its other sections.
  - `[RA]`…`[RI]` point to the review logs in `research/review/`, which record every correction made after the notes were written. `[RF]` is the review of note 12 and of the first version of this synthesis; edits made in this document by RF carry `[REVIEW RF: …]` tags. `[RG]` is a fresh-eyes critique that edited nothing, `[RH]` reviews note 14 (the canonical rules) and `[RI]` reviews note 13 (sessions and scoring).
- **Quotations are verbatim**, with an English translation after each non-English quote. Fraktur long-s is normalised. `[OCR]` marks text read from machine OCR, `[IMG]` text read by eye from a page image.
- **Reliability flags**: `[UNVERIFIED]` means the claim could not be confirmed in a fetched source; `second-hand` means a primary text is known only as quoted by someone else. Wikipedia is treated as a pointer, never as a source: note 01 followed its citations back, and found three errors in the English article [01 §Headline 5–7].
- **Original computation** (labelled as such) comes from the scripts in `research/sim/`, re-run and reproduced exactly during review [07 §5][RB].
- **Kindness rule.** The period sources contain ethnic, religious, gender and age jokes. They are recorded in the notes as history, flagged, and never proposed as game lines (§8.6).

## Method

1. **Twelve research strands**, each producing a fully cited note, run as parallel agents under one brief (`research/_BRIEF.md`):
   - 01 Wikipedia in every language, with each citation followed back;
   - 02 modern rule descriptions, compared clause by clause;
   - 03 period rulebooks 1750–1939, transcribed and translated;
   - 04 newspapers, literature and social history;
   - 05 table talk, with three sub-notes: 05b dictionaries and proverb lexica, 05c literary and feuilleton banter, 05d Czech, Hungarian, Slovene and Polish terms;
   - 06 the family of straw-man games and the word *Strohmann*;
   - 07 strategy, statistics (with an original simulation) and digital implementations;
   - 08 the physical cards;
   - 09 a wildcard strand for hunches;
   - 10 a survey of the three earlier games for reuse;
   - 11 the game in the other languages of the empire;
   - 12 a chaser for open questions left by the others.

   Two notes were added later. Note 13 studies how a period sitting was structured, scored and settled. Note 14 is the canonical ruleset by tally of the sources (§3), with the session decision (§4).
2. **Libraries reached**:
   - the Austrian National Library's book scans (ÖNB ABO, via its IIIF API) and its newspaper archive ANNO;
   - the Wienbibliothek;
   - archive.org, Wikisource, Projekt Gutenberg, the Deutsches Textarchiv and woerterbuchnetz (Grimm, Adelung, Campe, Wander);
   - Czech Kramerius, and the Hungarian REAL-EOD;
   - the Wayback Machine, GitHub, OpenAlex.

   ANNO's search page needs JavaScript, but the JSON service behind it is open. A small harvester (`tools/anno.py`) was written for it and pulled every snippet for seven spellings of the game's name (305 newspaper issues, `research/sources/anno/strohmandl.jsonl`). Google Books, HathiTrust, Arcanum, Hungaricana, Polona and dLib were blocked or rate-limited from the research machine. No bot check or lending restriction was bypassed.
3. **Review passes.** RA–RE re-checked notes 01–11 against the saved sources. RF reviewed note 12 and the first version of this synthesis, RG gave a fresh-eyes critique (it edited nothing), RH reviewed note 14 and RI reviewed note 13. The quote-matching figures, as the logs give them:
   - RA–RE extracted about 2,000 quoted strings from their notes and about 1,300 matched the saved texts automatically (RA 287, RB about 670, RC 87, RD about 250, RE 28). Most of the rest were English translations, headings, table fragments or newspaper quotations that survive only as page images, and the genuine misses were checked by eye. The denominators are per note and inflated by those non-quotations, so the figures are not a single pooled test [RF §B item 32];
   - RF drew its own sample of 28 strings from the first version of this synthesis: 19 matched exactly, 8 missed, and 1 was a deliberate control (an invented sentence) that was correctly not found. The misses were an OCR line break, a word-order slip, a garbled snippet, image-only newspaper quotes, a spelling difference and one invented Czech question; the word-order slip and the Czech question were fixed [RF §D];
   - RH found all 13 German quotations in note 14 in the sources after OCR normalisation, with one misattributed phrase (the Valat phrase, Ulmann's, not Bermann's) and one ellipsis that hid a clause [RH];
   - RI extracted 178 fragments from note 13 and 47 matched automatically (many fragments were broken by quote parity, English glosses and OCR damage). RI checked by eye every German period quotation the session design depends on, and found them all present in the saved sources, subject to OCR noise [RI];
   - dozens of newspaper page images and crops were read by eye;
   - errors were fixed in place and logged.

   No quotation in the notes' own sources was found to be invented [RF §F]. The one invented sentence found in the first version of this synthesis, a Czech direct question, was fixed [RF §B item 22]. The errors were misreadings, wrong page numbers, and claims resting on a secondary source presented as if the primary had been read (§13).

---

## Executive summary

**What the game is.**
- Strohmandeln is a two-player trick-taking game of the Austrian Tarock family, played with the 54-card Tarock pack: 22 trumps (*Tarocke*) and 32 suit cards [03-S12][02-S1].
- Each player holds 15 cards in hand and has three face-down packets of four cards, the *Strohmänner* ("straw men"), in front of him.
- The top card of each packet lies face up. A tarock or king that turns up there is shown to the opponent and taken into the hand. Any other card stays face up as part of its owner's hand, to be led, followed or trumped with.
- A card used from a packet uncovers the one beneath it [03-S12][03-S13][03-S14].
- The last card of a packet is taken into the hand whatever it is, and shown first [14 rule 10].
- Card points are counted as in all Austrian Tarock; there are 70 in the pack and 36 wins [03-S12][07 §5.1].

**Its distinctive mechanic.**
- No relative found has the automatic "pull" of kings and tarocks from the packets into the hand [06 §Headline 4]. The search of relatives was not exhaustive [RC][RG].
- Its oldest describers prized its suit play and finesses over its trumps. Bermann (1894): "Im Strohmann-Tarok tritt die ausschlaggebende Bedeutung der Trümpfe … am wenigsten hervor" (in Strohmann-Tarok the decisive importance of the trumps shows least) [03-S13].
- Werner (1891): "Die größte Rolle beim Strohmann-Tarok spielt das Impassiren. Dies allein entscheidet meist über den Ausgang der Partie" (finessing plays the biggest part; it alone mostly decides the game) [03-S12][12 item 6].

**Two games and an ancestor have borne the name or its idea.** Note 03 counted "three games that carried the name" [03 §Headline 1]. The framing is loosened here, because the 1800 "Taroc en deux" never used the name Strohmandeln [03-S1][03-S8][RG]. Note 01 judges that dead hand not a rules ancestor, since its cards are never used, while note 06 proposes it as the first link of a line whose last link no source states [01 §History, "1800" item][06 §4], so "ancestor" is a hedge.
1. **The ancestor, "Taroc en deux"**, Berlin 1800 and reprinted to 1846: a 78-card game in which 25 cards are set aside unseen [03-S1][03-S2][03-S8].
2. **The first "Strohmandeln"**, Vienna 1829–1853: ordinary three-hand Tarok-Tappen with the missing player as a face-down pile. Whoever wins a trick takes its top card for points [03-S4][12 item 4].
3. **The modern game**, described in full from 1891 (Werner), 1894 (Bermann) and 1899 (Ulmann), and attested earlier only weakly: Ulmann's first edition (by March 1888) is known through a newspaper review (the copy read is the 1899 edition), and an 1886 list naming "Strohmann-Tarok (en deux)" is an OCR snippet whose "(en deux)" may denote Neunzehnerrufen (a different Tarock game) with two Strohmänner [03-S12][03-S13][03-S14][03-S15][03-S27][RC][REVIEW RF].

The name itself is older than any rules: it appears as a bare name in a list of Tarok kinds in a Vienna/Prague book of 1824 [06-S3][RC].

**What its contemporaries thought of it.**
- The first version was scorned in print. From 1829 the same sentence calls it a game for "Leuten …, denen das edle Piquetspiel gänzlich unbekannt ist" (people to whom the noble game of Piquet is entirely unknown) [03-S4]. It was reprinted to 1853 [03-S5][03-S7][12 item 4].
- The modern game was praised in one syndicated paragraph, which §2.3 examines [REVIEW RJ: the same paragraph is also in a Brno paper of 22 Oct 1887; it is most probably about the two-hand game, not Neunzehnerrufen]. The *Klagenfurter Zeitung*'s review of Ulmann's *Tarokbuch* (30 March 1888) says Strohmann-Tarok "ist hiedurch zum Lieblingsspiele aller denkenden Tarokfreunde geworden" (has thereby become the favourite game of all thinking Tarock lovers) [03-S15] [REVIEW RF: the words are the reviewer's, seen on the page image, not Ulmann's; the synthesis had said "Ulmann (1888) calls it"]. Bermann (1894) says it is "unter feinen Spielern sehr beliebt" (much loved among fine players) [03-S13].
- In the press, from 1850 to 1938, it was the coffee-house game for two, and the fallback when the third player for a Tapper failed to come [03 §4][04 §Headline 1]. Many newspaper hits for the word are other senses (§5), and no hit rate was computed for the 305 issues [RE][RG]. "Können Sie nicht 'Strohmandeln'? — Zu dienen." (Can't you play Strohmandeln? At your service.) (*Figaro*, 1888) [03 §4].

**Where it lived.**
- Vienna above all, and Austria and Bohemia more widely [04][03 §4].
- Czech borrowed the name as *štromandl*, declined as a Czech noun from at least 1891, though two of the five attestations are weak [11][RE].
- A modern Hungarian name, *Strohmandli*, is given by Mayr & Sedlaczek and by a modern Hungarian site [RE].
- No Slovene, Polish, Croatian or Italian description was found. Several of those libraries could not be searched [11][RE].

**Why it matters for this project.**
- The rules are well documented in three period books (Werner 1891, Bermann 1894, Ulmann 1888/1899) and four modern ones (Beck 1972, Dummett 1980, Mayr & Sedlaczek 2007, Furr 2009) [03 §2][02 §1]. Beck was never read (known via Dummett and Wikipedia), and Dummett is a digest of other authors [REVIEW RF: caveat added].
- Note 14 tallies the sources rule by rule into one canonical game, with settings only for the three disagreements that persisted through the game's history: both players passing, the game values, and the Pagat ultimo value (§3) [14].
- Period practice gives the shape of an evening: Touren announced, a running account on a slate, settlement in kreuzer. It is described for multi-hand Tarock, and the game's session design is a reconstruction from it (§4) [13][RI].
- There is a deep well of period table talk: calls, kibitzer jokes, court reports, Nestroy couplets [05][05c][05d].
- No playable digital version of the game was found [07 §3.1][12 item 8]. Austrian Tarock apps and publishers' digital products were not searched [RG].

---

## Contents

1. What the game is (in plain English, then the canonical rules)
2. History: two games and an ancestor
3. Rules: the canonical set, the settings, the rules left out, and the questions a programmer will meet
4. The session: how an evening was played and settled
5. The family: straw men, dead men and dummies
6. Social life: coffee-houses, courts, couples and kibitzers
7. The game in the empire's other languages
8. Table talk and banter
9. Strategy and numbers
10. The cards
11. Digital implementations and the computer opponent
12. Reusing the earlier games
13. What review changed, and how far to trust each note
14. Decided, and open questions
- Appendix A. Research notes and review logs

---

## 1. What the game is

### 1.1 How it plays, in plain English

Strohmandeln is a game for two people, played in Vienna and the rest of Austria with the 54-card Tarock pack [03-S12][02-S1]. *Strohmann* is German for "straw man", a front man or a dummy, and *Strohmandl* is its Viennese diminutive [06-S7][06-S9][09-S8].

**The cards.**
- The pack has four ordinary suits of eight cards, 32 in all. Each suit runs king, queen, *Cavall* (the knight), jack, then four numbered cards [03-S12][08 §Headline].
- It also has 22 *trumps* (*Tarocke*; in this review "tarock" in lower case means one trump card, and "Tarock" the family of games): a fifth, permanent suit with its own ranking. The *Sküs* is the highest trump, then XXI down to I. Trump I has the nickname *Pagat* [03-S12][02-S1][08 §Headline].
- A *trick* is one round of play in which each player puts down one card. The winner of a trick leads the next one [03-S12]. You must follow the suit that was led if you can. If you cannot, you must play a trump if you have one ("trump" is also a verb: the trump takes the trick) [03-S12][14 rule 12].

**The straw men.**
- Each player is dealt 15 cards to hold. The other 24 cards are laid face down as six packets of four, three in front of each player: his *Strohmänner*, his straw men [03-S12][14 rule 3].
- Before play, each player turns up the top card of each of his packets. If it is a trump or a king, the opponent is shown it, the player takes it into his hand, and the next card is turned. The first card that is neither stays face up on its packet [03-S12][14 rule 9].
- A face-up packet card can be played like a hand card, and playing it uncovers the card beneath [03-S12][14 rules 11, 13].
- Each player thus plays 27 cards in all, 15 from the hand and 12 from the packets, so there are 27 tricks [03-S12][02 A8].

**Why the straw men matter.**
- Part of what each player will hold lies face up where the opponent can see it. The rest lies face down where even its owner cannot see it: a player's own unturned packet cards are unknown to him too [03-S12][07 §Headline 5].
- So both sides play with half-open cards, and a packet can hand over a tarock or a king at any moment. Werner (1891) says finessing (*Impassiren*, playing a suit so that the opponent must commit first) plays the biggest part and alone mostly decides the game [03-S12][12 item 6]. The strategy notes advise playing the packets off early to learn what lies under them, and keeping one back when the opponent threatens a Pagat ultimo (a bonus for winning the last trick with trump I, explained under Points below) [07 §1.2][03-S12].
- The hidden cards are also why one bonus, the Pagat ultimo (below), can never be announced: the opponent "may yet turn up a tarock as the last card of his packets" [03-S12]. An 1887 feuilleton joke has exactly this happen [03 §4].

**Bidding and winning.**
- The player who is not dealing is *forehand* and leads the first trick. He may say "Ich nehme auf" ("I take it up"), which makes him the *declarer*, or pass ("Weiter"). If he passes, the dealer may take it up [03-S12][14 rule 5].
- Each card has a value: kings, the Sküs, trump XXI and trump I 5 each, queens 4, Cavalls 3, jacks 2, everything else 1 [REVIEW RJ: corrected from "the three top trumps"; I is the lowest trump, and note 14 rule 15 lists Sküs, XXI and I]. Won cards are counted three at a time, and 2 is taken off the total of each group of three, which comes to the same as taking 2/3 of a point off every card; so a score can be a whole number and a third or two thirds, and the whole pack comes to 70 points [14 rule 15][02 §4 item 2][07 §5.1]. [REVIEW RJ: follows note 02's counting table and note 14; the handling of one or two left-over cards (note 02 A17) is open and the tutor needs it.]
- The declarer must win 36 of the 70 to win the deal [14 rule 16].
- If both players pass, a "simple game" is played instead: nobody is the declarer, whoever makes 36 of the 70 wins, 35 each is a draw, and the stakes are lower. (Whether to play it or throw the cards in and redeal is one of the three settings, §3.3.) After a draw the next deal counts double [14 rules 7, 17, 18].
- If a player's 15 hand cards hold no trump at all, the deal is void and the same dealer deals again [14 rule 4].

**Points.**
- A declarer who wins gets 3 points; one who loses pays 4 to the opponent. The winner of a simple game gets 2 [14 rule 19][03-S12].
- Extra points, 1 each: the *Trull* (*tous les trois*: the Sküs, XXI and I held together in one hand) and all four kings held together. A player says so aloud ("announces" it) at the latest when he first plays one of those cards [14 rule 20][03-S12].
- The *Pagat ultimo* (*ultimo* is "last"): winning the last trick with trump I earns 1, and losing it costs 1. It is never announced beforehand [14 rule 21][03-S12].
- A *Valat* (winning all 27 tricks) earns 12 [14 rule 25].

### 1.2 The rules in detail

These are the canonical rules of note 14, which tallies the sources rule by rule; Werner (1891) is the base text, with two points where the consensus differs from him (rules 4 and 10 below) [14 §What the consensus amounts to]. Each rule's tally is in §3.2.

- **Players and pack.** Two players, the 54-card pack [14 rule 1][03-S12][02-S1]:
  - the Sküs (highest trump), then XXI down to I;
  - in each suit K, Q, C (Cavall, the knight), J;
  - then 10 9 8 7 in black suits and 1 2 3 4 in red suits, highest first [08 §Headline][02 §4 item 1].
- **Deal** [14 rules 2–4]:
  - the first dealer is drawn by lot, and the deal then alternates [03-S12][14 rule 2];
  - the dealer gives 5 cards to forehand, then 5 to himself, three times over, so each has 15 hand cards [03-S12];
  - the remaining 24 cards are laid face down as six packets of four, alternately, three in front of each player, forehand's first [14 rule 3];
  - **a deal in which either player has no tarock among his 15 hand cards is void:** the same dealer deals again, without penalty (Dummett; Ulmann and Bermann in their general rules; Mayr & Sedlaczek make it optional) [14 rule 4][RH]. Original computation puts this at 1 deal in 15,298 [07 §5].
- **Bidding** [14 rules 5–8]:
  - forehand says "Ich nehme auf" or passes ("Weiter", "Ich passe"); if he passes, the dealer may take it up [03-S12][02-S2];
  - there is no Kontra (doubling a bid) [14 rule 6];
  - if both pass, the default is that a "simple" game is played (Werner, Mayr & Sedlaczek, Löw, Piatnik). The alternative is to throw the cards in and redeal (Ulmann's usual practice, Beck, Bamberger). This is a setting, §3.3 [14 rule 7];
  - the packets are laid at the deal. With the throw-in setting they are laid only after a bid, as in Ulmann and Beck [14 rule 8][03-S14].
- **Opening the straw men** [14 rule 9]:
  - each player turns the top card of each of his packets in turn, left to right. A tarock or king is shown to the opponent and taken into the hand, and the next card is turned. The first card that is neither stays face up on its packet [03-S12][03-S14];
  - on average 2.8 cards come into the hand this way, and none at all 13% of the time (original computation) [07 §5.3].
- **The last card of a packet** [14 rule 10]:
  - the last card of a packet is **taken into the hand whatever it is, and shown to the opponent before it is taken** [14 §Decisions];
  - this is the one point where the canon departs from Werner, who leaves a last card unfit for the hand face up "bis es gelegentlich gespielt wird" (until it is played). The count is 3 to 1 on explicit statements (Beck, Babsch and Mayr & Sedlaczek against Werner), and RH calls it the weakest tie-break in the note [14 rule 10][RH].
- **Play** [14 rules 11–14]:
  - forehand leads; the winner of each trick leads to the next. 27 tricks are played [03-S12][02 A8];
  - the face-up packet cards are part of the hand: a player may lead, follow or trump with them [03-S12][03-S13];
  - you must follow suit, from the hand or from a face-up packet card. If both can follow, the choice is free. If you cannot follow, you must play a tarock (from hand or packet); if you have none, any card. There is no obligation to win the trick [14 rule 12][03-S13 "steht die Wahl … frei"];
  - after the trick is gathered, each used packet card uncovers the next. A tarock or king is shown and taken (and the next card turned); anything else is the new face-up card [14 rule 13][03-S12][02 A5].
- **Counting** [14 rules 15–17]:
  - card values as in §1.1; counted in thirds, 70 in all, kept exact (`research/sim/points70.py`) [14 rule 15];
  - the declarer needs 36, i.e. at least 35⅔; 35⅓ loses. Period sources say "35 und 2 Blatt" (35 and two thirds), and the 1908–1920 press columns that seem to state three different thresholds all state this one [14 rule 16][RB][03 §4];
  - in a simple game whoever makes 36 wins, and 35–35 is a draw [14 rule 17].
- **Scoring** [14 rules 18–25]:
  - declarer wins 3, loses 4; a simple game pays 2. These are the default values (a setting, §3.3) [14 rule 19];
  - after a 35–35 draw, the next game counts double with its announcements (Werner, Beck, and Bermann by agreement; Ulmann voids the game) [14 rule 18][03-S12]. Whether "next game" means the next deal or the next round is settled for this project in §4;
  - Trull and four kings: 1 point each if held together in the hand (cards pulled from the packets count), announced at the latest when one of them is first played, and paid even if later lost [14 rule 20][03-S12];
  - Pagat ultimo: quiet only, ±1 [14 rule 21]. Werner's reason is that it cannot be announced, "da es ja möglich ist, daß der Gegner … als letzte Karte seines Talons noch einen Tarok aufdeckt" (since the opponent may yet turn up a tarock as the last card of his packets) [03-S12];
  - Valat (all 27 tricks): 12 points. Werner calls it "eine außerordentliche Seltenheit" (an extraordinary rarity); Ulmann and Bermann call it impossible and give it no premium, and the canon keeps 12 with no setting [14 rule 25 and Decisions][03 §2].
- **Not in the canon:** bird bonuses, Rostopschin, Mondfang, Kontra, quiet honours, first lead by the bidder, big-win bonuses (§3.4).

---

## 2. History: two games and an ancestor

The history is told as two games and an ancestor: the 1800 "Taroc en deux" never bore the name Strohmandeln, and whether it is an ancestor at all is disputed between notes 01 and 06 (§2.1) [03-S1][RG].

### 2.1 The ancestor, before the name: the dead hand (1781–1846)
- A two-hand French tarot with a *mort*, "a dead hand of twenty-five cards", is described by de Gébelin (1781) as reported by Dummett [06-S1][RC].
- The German "Taroc en deux" text first appears in Julius Cäsar's *Neuester Spielalmanach für das Jahr 1800* (Berlin) [03-S1]:
  - 78 cards; the dealer deals in fives, 25 to his opponent and 28 to himself, and lays 3 into the Scat;
  - "die übrigen 25 Karten werden bei Seite gelegt, und gar nicht zum Spiele gerechnet" (the remaining 25 are put aside and not counted in the game at all);
  - it is played "wie das Piquet, nach Partie bis 100 Points" (like Piquet, to a partie of 100 points).
- The same text, word for word, is in Mößler's Vienna *Neuestes Spielbuch* of 1805, pp. 145–146 (the pages Wikipedia cites for "1800") and in Berlin 1810 [03-S2][03-S3][RD].
- It was still being reprinted in Vienna in 1845–46, in the same books as the first Strohmandeln [03-S8][03-S9].
- A Polish Breslau almanac of 1821 translates the same text, so it is not independent evidence [05d][RD].
- Note 01 judges the dead hand not a rules ancestor of the modern game, since its cards are never used [01 §History, "1800" item][01-S5] [REVIEW RF: was cited as §Headline 3, which does not say this]. Note 06 points to an Italian parallel for the next stage: the Piedmontese two-hand *Morto* (Turin 1787), in which the trick-winner takes the top card of an undealt stock [06-S1][RC].

### 2.2 The first Strohmandeln (1824–1853)
- **1824:** "Strohmandeln" appears in a list of Tarok kinds in a Vienna/Prague book: "Quadrille, en trois, Pagatjagen, Strohmandeln, Tappen zu drey und vier Personen" [06-S3]. It is a bare name with no rules [RC].
- **1829: the first rules**, Haas, *Neuestes allgemeines Spielbuch*, Vienna, p. 94, read on the page image [03-S4][01-S6]:

  > "E. Tarok-Tappen unter zwey Personen, oder das sogenannte Strohmandeln. Dieses Spiel wird ganz so wie das gewöhnliche Tappen unter drey Personen gespielt … Die dritte fehlende Person ist der Strohmann, dessen Karten verdeckt … liegen bleiben. … wirft sich derjenige, welchem der Stich gehört, allemahl das oberste Blatt von den Karten den Strohmanns zu, das jedoch, wäre es auch der Scüs, nicht sticht, sondern nur dem Empfänger des Stiches zu dessen Vortheil zählt."

  "Tarok-Tappen between two persons, or the so-called Strohmandeln. This game is played exactly like ordinary Tappen among three … The third, missing person is the Strohmann, whose cards stay face down … whoever wins the trick throws himself the top card of the straw man's cards, which, even if it were the Sküs, does not take a trick, but only counts to the advantage of the trick's winner."

- **The verdict that follows:**

  > "Indeß da hierbey weit weniger die Geschicklichkeit als der blinde Zufall wirkt, möchte diese Unterhaltung wohl nur von Leuten gewählt werden, denen das edle Piquetspiel gänzlich unbekannt ist, zumahl da es in Einseitigkeit noch das veraltete Mariage und jene neuere Mißgeburt »Tatteln« benannt, bey weitem übertrifft." [03-S4]

  "But since in this blind chance works far more than skill, this pastime would probably be chosen only by people to whom the noble game of Piquet is entirely unknown, all the more as in one-sidedness it far surpasses even the antiquated Mariage and that newer abortion called 'Tatteln'."

- **The sneer is aimed at this first game, the dummy-pile game of 1829, not at the modern one.** The modern game, with its packets, is a different game that the sources praise (§2.3) [03-S4][03-S13][03-S15][RG].
- **The sentence was reprinted for decades:**
  - Haas 1834 and 1842 [03-S5][03-S7].
  - Tendler 1830, 1845 and 1846, reworded and with Écarté added, and ending "so ist es natürlich keine Kunst, das Spiel zu gewinnen" (so it is naturally no art to win the game) [03-S6][03-S8][03-S9].
  - Alvensleben's *Encyklopädie der Spiele* (1853) pp. 576–577, with "Erfindung" (invention) softening "Mißgeburt" (abortion) [12 item 4][05d].
  - Wikipedia dates it to an 1839 edition. That edition exists [12 item 7]. Its p. 121 is unread, but the 1842 edition has the sentence on p. 121 [03-S7][RD].
- **For this project, the joke writes itself, with a care.** The same author series that made a Piquet game is now making a game with the name the books said only Piquet-ignorant people would play. The books said it of the 1829 dummy-pile game; any banter that uses the line should say so, and not attach it to the modern game without that note [03-S4][RG].

### 2.3 The modern game (1855?–1938)
- **The name, without rules.** Vanderheid (1855) ranks "Strohmann-Tarok" among the three "besonders beliebte" (especially popular) Tarok games, and an 1866 advertisement mentions an appendix on it. The ÖNB copy has no such chapter, so which game he meant is unknown [03-S10][03-S11].
- **The modern game is described in full from 1891; the earlier attestations are weaker:**
  - "das Strohmann-Tarok (en deux)" in a list of Tarok kinds in *An der schönen blauen Donau*, 1886: **downgraded.** It is an OCR snippet only, and "(en deux)" is also a label for Neunzehnerrufen with two Strohmänner, as an 1887 Brno article shows ("auch en deux mit zwei Strohmännern gespielt"). RC found that the Brno article may mean Neunzehnerrufen, not Werner's pile game, so the pile game is not certain in 1886 [03-S27][06-S4a][RC][REVIEW RF];
  - the *Klagenfurter Zeitung* review of Ulmann's *Tarokbuch*, 30 March 1888, which names "Strohmann-Tarok (en deux)" [03-S15]. [REVIEW RJ: checked on the page images (`research/sources/03/anno/kfz18880330_p1_a/b/c.jpg`) and the saved Brno OCR (`research/sources/06/cz/ocr/0429f8b0...txt`, *Mährisch-schlesischer Correspondent*, 22 Oct 1887). Both papers print the same syndicated survey of Tarock kinds, so these are not two independent attestations. The praise is a separate sentence naming "Das 'Strohmann-Tarok' (en deux)" as its own game; the sentence before it, "Diese in Mähren so beliebte Tarockart kann als eine gute Schule … bezeichnet werden", concerns Neunzehnerrufen, which "auch en deux mit zwei Strohmännern gespielt wird". Ulmann's preface (1899 ed., p. V, 1st-ed. preface) lists "das Strohmann-Tarok en deux, das Paskiewitsch, das Neunzehnerrufen mit Strohmann" as three games [03-S14]. The praise is most probably about the two-hand game; "(en deux) = Neunzehnerrufen" is not supported for this sentence, though it cannot be excluded for the 1886 list. The Klagenfurt page ties the paragraph to Ulmann only by a trailing footnote ("Illustrirtes Wiener Tarokbuch"); the Brno OCR ends mid-column, so the footnote there is unknown. Both texts are reviewers' prose, not Ulmann's];
  - Ulmann's *Illustrirtes Wiener Tarokbuch* (1st ed. by March 1888), whose preface claims the game had been "in der Tarokliteratur bisher gar nicht besprochen, ja nicht einmal erwähnt" (not discussed in the Tarock literature, indeed not even mentioned). [REVIEW RJ: the preface text was read in the 1899 edition (p. V, headed as the first edition's preface; OCR at `research/sources/03/abo/ulmann1899/ocr.txt` line ~795), not only through the review.] The copy read is the 2nd edition of 1899 [03-S14][03-S15];
  - Werner's *Das moderne Tarokspiel*, 2nd ed., 1891, which adds the chapter and calls the game "Strohmann-Tarok (in der Umgangssprache Strohmanndl genannt)" (colloquially called Strohmanndl), an "ältere" (older) variant [03-S12][12 item 6];
  - Bermann's *Der praktische Tarokspieler*, 1894 [03-S13];
  - Ulmann's 2nd ed., 1899 [03-S14].
- **Dummett on the change.** Dummett (1980) writes that the "rather jejune game was ousted by a much better and quite different one" [01-S7]. "Jejune" is Dummett's own word, not Ulmann's. Wikipedia's footnote that credits this wording to Ulmann's *Buch der Familienspiele* pp. 244–245 is therefore probably a misattribution, but those pages remain unread, so what Ulmann says there is unknown [12 item 3] [REVIEW RF: "probably" added; note 12's headline had stated it flatly].
- **Twentieth century.**
  - The game kept appearing in the press until 1938 [04][03 §3].
  - The literature of this period includes Unger's Tarock manuals (1918–26, not read) [03 §8], Löw (1956), Beck (1972), and the Hungarian Widder Lajos, all known through Dummett [02-S1].
  - Dummett and Mayr & Sedlaczek (2007) report the modern consensus [02-S1][02-S2].

### 2.4 Timeline

| Year | Event | Source |
|---|---|---|
| 1781 | Two-hand tarot with a 25-card *mort* (de Gébelin, via Dummett) | [06-S1] |
| 1787 | Piedmontese two-hand *Morto*: trick-winner takes the stock's top card | [06-S1] |
| 1800 | "Taroc en deux", 78 cards, 25 dead (Berlin) | [03-S1] |
| 1805 | Same text in Vienna (Mößler) | [03-S2] |
| 1824 | "Strohmandeln" named in a list of Tarok kinds | [06-S3] |
| 1829 | First Strohmandeln rules, with the Piquet sneer | [03-S4] |
| 1839 | Whist's "Strohmann" (dummy) attested in German (Prague) | [06-S4b][RC] |
| 1850 | Court report: a coffee-house debt from Strohmandeln | [03-S16] |
| 1853 | Alvensleben reprints the 1829 text (with "Erfindung" for "Mißgeburt") | [12 item 4] |
| 1855 | Vanderheid: Strohmann-Tarok one of three popular Tarok games | [03-S10] |
| 1880 | *Pester Lloyd*: "urwüchsig … den Geist nicht besonders anstrengend" | [03-S22] |
| 1883 | Werner, 1st ed. (no Strohmann chapter) | [03-S41][RD] |
| 1886 | "das Strohmann-Tarok (en deux)" in a list of Tarok kinds: OCR snippet only; "(en deux)" may mean Neunzehnerrufen, so weak | [03-S27][RC] |
| 1887 | Brno article: Neunzehnerrufen "auch en deux mit zwei Strohmännern", then a separate sentence on "Das 'Strohmann-Tarock' (en deux)" and its praise; the same paragraph reappears in the Klagenfurt review of 1888 [REVIEW RJ] | [06-S4a][RC] |
| 1888 | Ulmann's Tarokbuch, 1st ed., known through a review (the copy read is the 1899 edition); the review's paragraph, with the praise, is the same text as the Brno paper of 1887 above [REVIEW RJ] | [03-S15][03-S14] |
| 1891 | Werner, 2nd ed.: "Strohmanndl" chapter | [12 item 6] |
| 1891–92 | Czech "umím-li hráti štromandla" (whether I could play štromandl) anecdote | [11-S2][11-S3] |
| 1893 | Tepl "First European Tarock Congress" (a jocular local gathering of about 300; no Strohmandl rules printed) | [12 item 5] |
| 1894 | Bermann | [03-S13] |
| 1897 | "§ 13: Kiebitz, halt's Maul!" first found in the saved snippets (Kikeriki, 9 Sep 1897); earlier 1895 text says only "Hauptparagraphen" | [RD] |
| 1910 | "Der Königrufer sinkt zum Tapper, dieser zum Strohmandl herab" | [03 §4] |
| 1932 | Czech manual: "Štromandl. Taroky ve dvou" (heading only read) | [06-S10][RE] |
| 1937 | Court: "Strohmandl ist kein Hasard, sondern ein Kunstspiel" | [03-S40c] |
| 1972 | Beck, *Tarock komplett* (known via Dummett and Wikipedia) | [02-S1] |
| 1980 | Dummett, *The Game of Tarot*, pp. 457–459 | [02-S1] |
| 2007 | Mayr & Sedlaczek, *Wiener Zeitung*, "Wie sehen die idealen Regeln aus" | [02-S2] |
| 2009 | Furr, *Tarocchi*, "Straw Man Tarock" | [02-S3] |
| 2026 | harbour-tarock (GitHub) writes a Strohmandeln specification, not yet playable | [07 §3.1][12 item 8] |

---

## 3. Rules: the canonical set, the options, and the questions a programmer will meet

### 3.1 What every source agrees on
- 54 cards; 15 hand cards; three 4-card packets per player [02 §Headline 2][03 §2 obs. 1].
- Tarocks and kings are pulled from the packet tops into the hand after the opponent has seen them [02 §Headline 2][03 §2 obs. 1].
- The other face-up cards play as part of the hand [02 §Headline 2][03 §2 obs. 1].
- "Ich nehme auf" is the only positive bid [02 §Headline 2][03 §2 obs. 1].
- 36 of 70 wins [02 §Headline 2][03 §2 obs. 1].
- Pagat ultimo cannot be announced in advance in any of the primary texts read; Widder (1957, via Dummett) alone allows an announced ultimo [03 §2 obs. 1][14 rule 21][RH].
- The Trull and four kings are paid separately (Werner and Ulmann say 1 point each; Bermann only "besonders berechnet", reckoned separately) [03 §2][RD].
- Dummett, comparing the descriptions in Ulmann (two books), Bermann, Werner, Unger, Beck, Löw, Babsch, Widder Lajos and the Piatnik leaflet, found they "agree very closely" [02-S1] [REVIEW RF: "eight" was not Dummett's number].

### 3.2 The canonical rules

**Decided: one canonical game, built from a consensus of the legitimate sources, not a menu of presets.** A setting exists only for true differences of opinion that have persisted through the game's life, like whether aces are high in poker [14 §Settings]. The synthesis's earlier "Vienna 1891" and "Vienna 2007" presets and its options menu are superseded by note 14 [14 §What the consensus amounts to].

**How the canon was decided** [14 §Method]:
- The describers are five primary texts read in full (Werner 1891, Ulmann 1888/1899, Bermann 1894, Mayr & Sedlaczek 2007, Furr 2009) and others known only through Dummett's digest or Wikipedia and flagged second-hand (Dummett's norm, Beck, Löw, Babsch, Unger, Widder, the Piatnik leaflet) [14 §Method].
- The majority of independent describers decides each rule. A tie goes to the period texts (Werner, Ulmann, Bermann). A rule found in only one or two modern sources is left out as a regional or modern addition [14 §Method].
- Furr and Mayr & Sedlaczek are counted as one voice, but RH shows the dependence is not demonstrated: Furr names no source for the game and departs from them on several points. Counting him separately would move rule 7 to 5–3 and change no result [RH].

**The rules, with their tallies** (the full 25-row table is in note 14):

| Rule | Canonical choice | For – against | Margin |
|---|---|---|---|
| 1 Pack | the 54-card pack | all | unanimous [14 rule 1] |
| 2 First dealer | by lot, then alternating | Werner, Ulmann, Furr, Beck – Löw (winner deals) | 4–1 [14 rule 2] |
| 3 Deal | 5+5 three times; 24 cards as six packets of four | all | unanimous [14 rule 3] |
| 4 No tarock in hand | the deal is void; same dealer redeals, no penalty | Dummett, Ulmann, Bermann (general rules); Mayr & Sedlaczek make it optional | uncontested [14 rule 4][RH] |
| 5 Bidding | "Ich nehme auf" or "Weiter" | all | unanimous [14 rule 5] |
| 6 Kontra | none | Werner, Ulmann, Bermann, Mayr & Sedlaczek, Furr, Dummett – Löw | 6–1 [14 rule 6] |
| 7 Both pass | simple game played (**setting**) | Werner, Mayr & Sedlaczek (+Furr), Löw, Piatnik – Ulmann (usual), Beck, Bamberger | 4–3; 5–3 with Unger [14 rule 7] |
| 9 Opening the packets | tarock or king shown, then taken; next card turned | all | unanimous [14 rule 9] |
| 10 Last card of a packet | **taken into the hand and shown** | Beck, Babsch, Mayr & Sedlaczek – Werner | 3–1 on explicit statements [14 rule 10][RH] |
| 11–13 Packet cards in play | part of the hand; free choice when both can follow; replaced after the trick | all | unanimous [14 rules 11–13] |
| 14 First lead | forehand, then the trick's winner | six sources – Löw, Furr (bidder leads) | 6–2 [14 rule 14] |
| 15–17 Counting and winning | 5/4/3/2/1, in thirds, 70 in all; 36 (35⅔) wins; 35–35 draws | all | unanimous [14 rules 15–17] |
| 18 After a draw | next game counts double | Werner, Beck, Dummett, Mayr & Sedlaczek, Bermann (by agreement) – Ulmann | 5–1 [14 rule 18] |
| 19 Game values | 2 / 3 / 4 (**setting**) | Werner, Mayr & Sedlaczek (+Furr) – Ulmann 1/2/4, Dummett and Beck 1/2/3 | plurality only [14 rule 19][RH] |
| 20 Trull and four kings | 1 point each, announced from the hand | Werner, Ulmann, Bermann, Dummett, Beck – Mayr & Sedlaczek (quiet), Furr (none) | 5–1 [14 rule 20] |
| 21 Pagat ultimo | quiet only; value 1 (**setting**) | quiet in all five primary texts; Widder alone allows an announced ultimo | [14 rule 21][RH] |
| 25 Valat | 12 points, no setting | Werner, Unger, Mayr & Sedlaczek (+Furr) – Ulmann, Bermann (no premium); Beck unclear | 3–2 [14 rule 25] |

**Close calls**, the ones a reader might fairly dispute [14 §What the consensus amounts to]:
1. **Both pass (rule 7)** is 4–3. Throwing in has period weight (Ulmann's usual practice). Playing on has the period's fullest text (Werner) and the modern consensus. It also gives more play: original computation suggests a strict bidder would take up only about 1 deal in 10, so most deals would otherwise be thrown in [07 §5.4]. RG draws the same conclusion: the simple game will be the game most of the time [RG (a) 4].
2. **The last card (rule 10)** is 3–1 on explicit statements for taking it into the hand. Werner's rule has period weight, but no other source states it [14 rule 10][RH].
3. **Game values (rule 19)** is a plurality, not a majority [14 rule 19].

Two decisions recorded in note 14 [14 §Decisions]:
- **Rule 10 (the last card)** is canonical, with no setting.
- **Rule 25 (Valat)** stays at 12 with no setting. Wikipedia's table has a dash in Beck's column, which may mean "none" or merely "not mentioned"; that is too weak to establish a modern split, so the persistent-split test is not met [RH].

### 3.3 The persistent-split test, and the three settings

**The test.** A disagreement counts as persistent only if it is found both among the period describers (Werner, Ulmann, Bermann, and Unger of 1923, to 1938) and among the modern ones (Löw, Beck, Babsch, Mayr & Sedlaczek, Furr, 1956 on). A split within one era only, or a shift from one era to the next, keeps the canonical rule and no setting [14 §Settings, test; RH (a)].

**Three settings pass it:**

| Setting | Default | Alternatives | Period split | Modern split |
|---|---|---|---|---|
| Both players pass | play the simple game | throw in and redeal (packets then laid only after a bid) | Werner plays on; Ulmann usually throws in; Bermann gives both ways | Mayr & Sedlaczek, Löw, Piatnik play on; Beck, Bamberger throw in [14 rule 7] |
| Game values (simple / won / lost) | 2 / 3 / 4 | 1 / 2 / 3; 1 / 2 / 4 | Werner 2/3/4; Ulmann 1/2/4 | Mayr & Sedlaczek 2/3/4; Beck and Dummett 1/2/3 [14 rule 19] |
| Pagat ultimo value | 1 | 2 | Werner 1; Ulmann "in manchen Kreisen" 2; Bermann a lost ultimo costs double; Unger 5 or 1 | Mayr & Sedlaczek, Furr, Beck 1; Beck "some 2–3" [14 rule 21][RH] |

One caution on the second row: RH observes that 1/2/4 has one period source (Ulmann) and no modern one, so strictly it is not a persistent value. Note 14 leaves it in the menu, and the question is open [RH].

**Rules that failed the test, and so are canonical with no setting** [14 §Settings; RH (a)–(g)]:
- Last packet card: only Werner states a rule (rule 10).
- Honours declared from the hand: all three period books declare; only the modern Mayr & Sedlaczek play them quiet.
- First lead: a modern split only (Löw, Furr).
- Kontra: Löw alone.
- Misdeal penalty: period-only (Werner 1 point; Ulmann "gewöhnlich eine Strafe"; Bermann by agreement), and not in the canon.
- Tie consequence: a period-only split (Werner doubles, Ulmann does not, Bermann by agreement).
- Announced ultimo: Widder alone.

### 3.4 Rules deliberately left out

- **Bird bonuses.** Uhu, Kakadu and Quapil bonuses are in Mayr & Sedlaczek and Furr only, and the bird names are 20th-century [14 rule 22][05 §Headline 6].
- **Rostopschin.** XVII then XVIII on consecutive tricks, called "Ross!" … "Topschin!". It is in Furr alone, plus Torberg's anecdote at second hand [14 rule 23][02-S3][12 item 1]. Wikipedia's scoring table has the trumps as "VII and VIII", an editing error, and its glossary has "XVII and XVI" [01 §Headline 5][12 item 1]. Mayr & Sedlaczek's 2002 page does say it leads to interesting situations in Strohmann-Tarock, which weakens the reading that it belongs only to Königrufen [12-S5][RF §E].
- **Mondfang** (catching the XXI with the Sküs) is in no primary Strohmandeln description [14 rule 24][02 §Headline 4].
- **Kontra.** Löw lets the opponent double a bid; no other source does [14 rule 6][02 A18].
- Also modern only: quiet honours (Mayr & Sedlaczek), a first lead by the bidder (Löw, Furr), and Furr's big-win bonuses (declarer 45 or more scores 4, defender 44 or more scores 5) [14 rules 14, 20][02-S3][RH (e)].

### 3.5 Questions the sources leave open
Note 02 lists 18 of these [02 §2]. Several are now settled by note 14 (the last card, the first lead, following suit). The ones that matter most:

- **Following suit.** The cards you must follow from are the hand plus the face-up packet cards. Nothing forbids playing a face-up card when the hand could follow [02 A2]. Bermann: "ist beides der Fall, so steht die Wahl … frei" (if both are possible, the choice is free) [03-S13].
- **When packet cards are replaced.** After the trick is gathered, not during it, so the follower never sees what the leader's flip reveals [02 A5].
- **Is a packet's last card shown when it is taken into the hand?** Sources disagreed [02 A4]. Decided in note 14: it is shown, because every pulled card is shown before it is taken (rule 9 is unanimous) and Dummett says "exposed"; Wikipedia's account of Beck, "without being revealed", is the outlier [14 rule 10].
- **Pagat ultimo and the packets.** The last trick is trick 27, whether the card comes from the hand or a packet. Werner warns that a buried tarock in the opponent's packets can stop a Pagat ultimo, and advises keeping packet cards back for exactly that purpose [02 A13][03-S12].
- **Information the interface must show** [02 §4 item 15][03-S14]:
  - the opponent's face-up packet cards;
  - each card pulled into a hand, shown before it is taken;
  - how many unseen cards each packet still holds.

  Ulmann's Codex §31 treats peeking under a packet card as an offence that "rigorose Strohmannspieler" (strict straw-man players) rank with a revoke.

---

## 4. The session

The earlier synthesis said that no source gives a match length [02 §Headline 7]. Note 13 asked what a period sitting looked like, and the answer is that period players did not race to a target score. They played deal after deal, kept a running account, ended the evening with an announced ritual, and settled in money [13 §Headline 1, 3].

### 4.1 What the period sources say

- **The unit is the single deal**, called *Partie* or *Spiel*. A sitting was open-ended [13 §Headline 1][13-S1][13-S2].
- **A *Tour* is one full round of dealing, each player dealing once.** Before the company broke up, it was usual to announce three or six Touren, and play ended when they were done. Ulmann: "Es ist üblich, ehe die Spielgesellschaft auseinandergeht, drei oder sechs Touren anzusagen" (it is usual, before the company breaks up, to announce three or six Touren) [13 §Headline 1][13-S1 p. 19][13-S3 pp. 43–44].
- **The biggest loser calls it, out of courtesy.** Bermann: "Gewöhnlich überläßt man dies aus Courtoisie dem im stärksten Verlust befindlichen Spieler" (this is usually left, out of courtesy, to the player with the biggest loss). The announced Touren can be extended "nur mit allseitiger Zustimmung" (only with everyone's consent) [13-S3][RI (b)].
- **The account was a running total per player.** Werner (1891) describes a *Conto* for each player, "bei fortlaufender Addition" (with running addition), and a final settlement by differences, checked by the rule that "Gewinn und Verlust natürlich einander gleich sein müssen" (gain and loss must of course be equal). It was written on a slate: Haas (1829) prefers "kleine Schiefertafeln mit Griffeln" (small slates with styluses) to chalk [REVIEW RJ: the saved OCR of Haas (`research/sources/03/abo/haas1829/ocr.txt` line 455) reads "mit Niedenſtiften", not "Griffeln"; note 13 gives "Griffeln" without a page image, so the word is [UNVERIFIED — review]; the sense (slates or parchment with a writing tool, not dirty chalk) is secure] [13 §2.1, §2.3][13-S2][13-S7].
- **Cash after each deal was the alternative**, which "verliert aber durch Wechseln und Herausgeben des Geldes umsomehr Zeit" (loses time through changing and giving change) [13-S2].
- **Prices** are for Tarock in general. Werner prices a Point at half a kreuzer, but only as a hypothetical for a four-hand example. A newspaper of 1894 says of Johann Strauss that his winnings were "bei zwei bis drei Kreuzer der Point" (at two to three kreuzer the Point), and a 1896 snippet has "den Point um 5 kr." in a Viennese coffee house [13-S2][13-S13][13-S14][RI (c)]. These are newspaper snippets, and none is about Strohmandl.
- **For Strohmandl itself, no session rule is stated** in Ulmann or Werner. What they give is a deal scored in Points, with the deal alternating (Werner) [13 §Headline 6][13-S1][13-S2].
- **A modern race to 11 exists** (tarock.tirol, 2025: "Wer zuerst 11 Punkte erreicht, hat eine Partie gewonnen"). It is a modern club custom, not period, and it is not canonical [13-S5][14 §Decisions].

### 4.2 The canonical session: design A, "the evening"

Decided (14 October 2026): design A of note 13, the period "Abend" [14 §Decisions]. It runs as follows [14 §The session in detail][RI]:

- **A Tour is two deals.** Bermann and Ulmann say each player deals once per Tour, and Werner has the deal alternate in Strohmanndl, so with two players a Tour is two deals. "Drei Touren" is then 6 deals and "sechs Touren" is 12 [14][RI (a)].
- **The length is announced: 3 or 6 Touren.** Design A has them called at the start. The period custom was to announce them when the company wanted to stop, by courtesy the biggest loser [13-S3][RI (b)]. At the end the sitting may be extended by mutual consent, as in the period [14][13-S3].
- **A running account is kept on a slate**, with a total for each player, in the manner of Werner's Conto. With two players the final settlement is a single difference [13 §2.1, Design implications][14].
- **After a 35:35 draw the next deal counts double**, with its announcements. This is Werner's rule, the rules base, and Beck as reported. Ulmann voids the game, Bermann leaves it to agreement, Mayr & Sedlaczek double the next round of two deals, and Dummett the following round [REVIEW RJ: "of two deals" is Mayr & Sedlaczek's alone, per RI (d) and note 02 A12; note 14's session text says only "the next round"] [14][RI (d)][02 A12].
- **Points are converted to money at 2 kreuzer each, at the end.** No Strohmandl stake is sourced. The figure sits within the Tarock stake of "zwei bis drei Kreuzer der Point" reported for 1894, and **it is labelled as general Tarock evidence, not Strohmandl evidence** [14][13-S13][RI (c)].
- **Currency is open.** Gulden and kreuzer against krone and heller is undecided, because note 13's dates for the krone ("from 1892", "sole currency 1900") are unsourced [RI (c)].
- **A doubling still pending when the sitting ends lapses.** No source covers this case; it is a design choice [14][RI (e)].

**Design A is a reconstruction.** Every source for it describes multi-hand Tarock. None applies the Touren rule, the running account or the settlement to Strohmandl, and "one Tour is two deals" is an inference, not a quotation [14 §The session in detail][RI]. The game should say so wherever it explains the evening.

**Left out of the session:**
- the Ehrentour (a last extra Tour after a deal on which all passed; the trigger is everyone passing, never a tied score, and a tied-balance trigger would be an invention) [RI];
- Block-Tarok's chips, slates for debts, bookkeeper and *Brunnen* (a line drawn on the slate beneath which later debts are pooled and shared out), which are for three or four players [13-S1][13 §Cross-cutting choices];
- the period slang for the pot, which is excluded under the kindness rule (§8.6) [13 §2.2].

### 4.3 The period voice for settling up and ending the evening

All newspaper items below are OCR snippets from ANNO, with the damage that implies, unless marked [IMG] [13 §5]. The "Game line" column gives the notes' own suggested wording and is not a quotation.

| Moment | Said (source) | Game line |
|---|---|---|
| Calling the end | "Geschwind, meine Herren, drei Meister!" (Quickly, gentlemen, three Meisters!) (*Agramer Zeitung*, 14 Nov 1893). The note explains that "drei Meister" means "die letzten drei Spiele" (the last three deals), said by a husband whose wife presses him to go home. It is not the Touren call, and it was said at whist, Tarok or domino alike [13-S12][RI] | "Three Meisters, if you please." |
| Calling the end | "Nur noch eine Runde" … "die letzten drei Runden ansagend" ("Just one more round," says the doctor, announcing the last three rounds) (Ranzenhofer, *Pilsner Tagblatt*, 1917; the piece is about *Königrufen*, a four-hand game [REVIEW RJ: per note 05c LIT-S8]) [05c LIT-S8] [IMG] | "Just one more round. Well, three." |
| Who may call it | The biggest loser has the right, "aus Courtoisie" (Bermann 1894) [13-S3] | "The right to call it is yours; you've lost the most." (the note's suggested line; it ribs about luck and the score only) |
| Asking for a return | "eine dringende Revanche geben müssen" (had to give an urgent return match) (*Wiener Bilder*, 11 Jul 1897) [13-S20] | "A return match, then?" |
| Settling up | "Wenn in unserem Beispiel der Point zu einem halben Kreuzer verabredet worden wäre, so hätte X 56 kr. … gewonnen" (if in our example the Point had been agreed at half a kreuzer, X would have won 56 kr.) (Werner 1891) [13-S2] | "The Point at two kreuzer: you have won 56 kr." (the notes' wording, with the price changed) |
| Settling up | "Spielschulden sind Ehrenschulden" (gambling debts are debts of honour) (*Neues Wiener Tagblatt*, 6 Jan 1912) [13-S32] | the same, said once, at the settling |
| Settling up | "Und beim Zahl'n heißt's: Ich hab' meinen Beutel zu Haus!" (and at paying time it's "I left my purse at home!") (Nestroy 1835, Bonz text). Only this couplet is neutral; the verse around it is flagged for age and gender [05c LIT-S1] | "Purse at home, is it?" |
| Kibitzer at the settling | "Kiebitz, halt das Maul und zahl'!" (kibitzer, shut up and pay!) (*Freiheit!*, 23 Nov 1927) [05c LIT-S41] | the tutor: "I know: shut up and pay." |
| Closing time | "… und der Wachmann schon bedeutungsvoll an…" (and the watchman already, meaningfully, at the…) (*Der Abend*, 26 Jan 1928: the last round ended at ten past two) [13-S21] | "Ten past two. The watchman is coughing." |
| The slate | "Mit Kreide befleckte Finger verderben die Karten sehr, indem sie dieselben rauh und kennbar machen" (fingers stained with chalk spoil the cards badly, making them rough and recognisable) (Haas 1829) [13-S7] | a tutor hint about the slate |

---

## 5. The family: straw men, dead men and dummies

- **Relatives with face-up/face-down piles.** Officers' Skat (*Offiziersskat*, also *Kutscherskat*), Officers' Schafkopf, two-player Sjavs, a French tarot for two, and the Italian *Baronetti* (1787) all use piles whose exposed card is played and then replaced from beneath [06 §Headline 4][06-S5][06-S6].
  - Baronetti has four piles per player (10, 10, 10, 9) [RC].
  - Only Strohmandeln pulls kings and tarocks into the hand. "Unique" is too strong a word for an unexhaustive search [RC].
  - The two-player French tarot "*découverte*" in English Wikipedia is uncited and matches neither Furr nor pagat [01 §Headline 7].
- **The word *Strohmann*.**
  - As a straw doll it is attested from the 16th century, and as a front man from the 18th, "wohl nach frz. homme de paille" (probably after French *homme de paille*) [06-S7][06-S9].
  - As the whist dummy it appears in German from 1839, and Grimm gives it as a separate sense with Raabe (1864) [06 §Headline 6][05b].
  - The Austrian diminutive *Mandl* names any little man-shaped thing, such as a scarecrow [09-S8].
  - Card players also used "Strohmann" for the exposed dummy in four-hand Neunzehnerrufen, "wie im Whist" (as in whist) (Ulmann) [03-S14].
  - In 1916 and 1925 "Strohmann" alone names the two-hand game [04 §Headline 7].
- **A homonym warning for anyone searching.** In Austrian print "Strohmandl" is also a sheaf stook in a field, a scarecrow, a stooge, a carnival costume (dressed as the card game, 1909) and a surname. Most newspaper hits are not the game [03 §4][04][RE].

---

## 6. Social life: coffee-houses, courts, couples and kibitzers

- **A coffee-house game for small and large stakes.**
  - 1850: a man invited to play by a stranger in Adami's coffee-house wins "beim Strohmandeln 17–18 kr. C.M." (17–18 kreuzer, convention coinage) from him, then 1 fl. 20 kr. more that the stranger stays owing; the report is a court/crime item, but that it was a fraud case is unverified [03-S16][04 §2] [REVIEW RF: the synthesis had the winner reversed; note 04's correction of the subject of "gewann" was not carried into note 03].
  - 1893: "die aufgenommene Partie zu 4 fl, die einfache zu 2 fl" (a taken-up game at 4 florins, a simple one at 2) [03-S33].
  - 1894: a spendthrift's daily Strohmandl "um einen Gulden den Point" (at a gulden a point) [03 §4].
  - 1921: a Viennese coffee-house card-fee tariff [03-S39].
- **Second best to a game for three.** The 1910 ladder runs "Der Königrufer sinkt zum Tapper, dieser zum Strohmandl herab" (King-calling sinks to Tapper, Tapper to Strohmandl) [03 §4]. In 1914 it is "mehr ein stummes Spiel von Mann zu Mann" (rather a silent game, man to man) [03 §4]. In 1926 it is "ein schäbiges 'Strohmandl' zu zweit" (a shabby Strohmandl for two) [03 §4].
- **A couples' and household game, often in jokes** [04 §Headline 2][03-S29]:
  - an 1887 feuilleton husband loses his Pagat ultimo when "meine Frau deckt als letzte Karte ihres 'Strohmandls' den 'Tarok-Zweier' auf" (my wife turns up the Tarock deuce as the last card of her Strohmandl). This is the packet mechanic in a period text;
  - 1912: Elise Elizza of the Hofoper (court opera) [REVIEW RJ: "singer" is not on the page; it is an assumption], asked in the *Neues Wiener Journal* (7 Apr 1912, p. 11) what she likes best, answers "Am liebsten spiele ich - Tarock! Nach dem Essen muß mein Mann, ob er nun will oder nicht, schnell - vor seiner Ordination - mit mir „Strohmandl" spielen" (What I like best to play is Tarock! After the meal my husband has to play Strohmandl with me, quickly, before his surgery hours, whether he wants to or not). She adds that Bridge and Königrufen need four people and so are only possible on Sundays and holidays, with good friends. The page says only "(Hofoper)", not "soprano" or "singer". So the game is the weekday two-hander of a doctor's household, and the four-hand games are for Sundays [04-S31][04 §Headline 2]; page image `research/sources/04/crops/ANNO_nwj19120407_k4_p11_1.jpg`;
  - an 1887 joke: "Liebe ist wie das 'Strohmandel-Tarok' eine Angelegenheit unter Zweien; Dritte finden dabei nicht einmal als Kiebitze Verwendung" (love, like Strohmandel-Tarok, is a matter between two; third parties aren't even needed as kibitzers) [03 §4].
- **Courts and police.**
  - 1895: a libel case over a Strohmandl game at the Café de l'Europe, with a kibitzer accused of signalling [03-S34].
  - 1895: a court declines Tarock experts because "das Tarokspiel weder eine Wissenschaft noch eine Kunst sei" (Tarock is neither a science nor an art) [04 §Headline 3].
  - 1912: police commissioner Fried raided a gambling den by asking for "Strohmandelkarten" (Strohmandl cards) and playing a game with a colleague, "sich scheinbar um die Hasardspieler nicht bekümmernd" (seemingly paying the gamblers no heed), "wobei ihm der Polizeiagent Garhofer kiebitzte" (while police agent Garhofer kibitzed); "Die Hasardspieler wurden tatsächlich auf die harmlose Strohmandlpartie nicht aufmerksam" (the gamblers really did not notice the harmless Strohmandl game) [04-S33]; page image read, settling the 03/04 difference [03 §4][04 §2].
  - 1937: defendants argue "Strohmandl ist kein Hasard, sondern ein Kunstspiel, man muß kombinieren, allerdings kommt es auch auf das Blatt an" (Strohmandl is not a game of chance but of skill; you have to think, though the hand matters too) [03-S40c].
- **The Tepl "First European Tarock Congress"**, 15 January 1893, was a jolly local gathering called by the school catechist [12 item 5]:
  - it was greeted by telegram from, among others, "den 64 Tarockirern von Auschowitz" (the 64 Tarock players of Auschowitz, i.e. Úšovice by Marienbad) [REVIEW RF: "Anschowitz" was an OCR reading; the page image reads Auschowitz];
  - it made rules for kibitzers ("laute" and "stumme Kibitze", loud and silent) and set up a "Kuckuck" fine box;
  - it reported 22 Valats and 54 Pagat ultimos in 28 tables in one "official" hour, the Valats marked with a "Tusch" (fanfare) and the Pagat ultimos with a horn signal [REVIEW RF: the synthesis said each was greeted with both].

  The report's "einheitliche Spielregeln für Strohmandl" (uniform rules for Strohmandl) is the compression of a Vienna *Fremden-Blatt* item; the original *Neuigkeits-Welt-Blatt* report (read in full) prints no Strohmandl, Tapper or Königrufen rules, only the kibitz classes, card money and the "Kuckuck" box, and RE/RD read the whole as partly jocular [12 item 5][RE][RD] [REVIEW RF: "a joke" was too strong; the evidence is "no rules text printed"].
- **Atmosphere.**
  - "das Fallen der Karten, das Rasseln der Sechser, die Seufzer der Verlierenden" (the falling of the cards, the rattle of the Sechser (six-kreuzer coins), the sighs of the losers) (*Gartenlaube*, 1869) [09-S14].
  - Freud's Saturday Tarock (four-hand), with his pencilled dates on each pack's wrapper, is folklore relayed by Mayr & Sedlaczek, not a primary text. The famous retort is hedged even there: "soll … geantwortet haben" (is said to have answered) [09 Best of 2][RA][12 item 2].

---

## 7. The game in the empire's other languages

- **Czech.**
  - *Štromandl*, declined as a Czech noun, is attested in 1891 and 1892 in a teachers' anecdote. A parish priest asks an applicant, in the speaker's reported speech, "umím-li hráti štromandla" (whether I could play Strohmandl; 1891 text, 1892 variant "umím-li hráti štromandla?") and gives him the post when he says yes [REVIEW RF: the synthesis had printed an invented direct question "umíte hrát štromandla?"; the sources have the indirect form]. The anecdote never says "cards"; the reading comes from the questions around it [11-S2][11-S3][RE].
  - A 1932 household compendium has a section headed "Štromandl. Taroky ve dvou" (Strohmandl: Tarock for two) on p. 370. Its text is copyright-restricted and unread [06-S10][RE].
  - Czech Tarock vocabulary: *skýz* (Sküs), *mond*, *pagát*, *trulka* (from *tous les trois*), and *čumák* for a kibitzer (1890) [05d POLY-S18][05d POLY-S19].
- **Hungarian.**
  - Parlaghy's *Kártyajátékok könyve* (Budapest, 1888) [REVIEW RF: was "c. 1889"; note 11 gives 1888] has no two-hand Tarock. It uses *strohmann* only for the whist dummy [11][RE].
  - The modern name *Strohmandli* is given by Mayr & Sedlaczek and by scordatura.hu [RE].
  - The banter is rich. Jókai (1895): "Hohó! Hiszen nálam van a pagát!" (Oho! I've got the Pagat!) and "Skiz, vagy semmi!" (the Sküs or nothing!). Mikszáth: "Kontra az ultimónak, kontra a játéknak, kontra mindennek!" (contra the ultimo, contra the game, contra everything!) [05d POLY-S10][05d POLY-S11][RD].
- **Polish.** Kraków satire (*Djabeł*, 1901, no. 10): "Trul, skiz, dziesięć taroków i pagat złapany" (Trull, Sküs, ten tarocks and the Pagat caught). It is a generic Tarock noise, not a Strohmandeln description [05d POLY-S12][REVIEW RF: source and context added; the line was checked in the saved OCR].
- **Slovene, Croatian, Italian.** No description of the game found. These are weak negatives: several libraries could not be reached [11][RE].
- **German-language papers across the empire** carry it too: *Pester Lloyd* 1880, *Agramer Zeitung* 1894–95, *Brünner* and *Prager* papers [03 §1.4][11][06].

---

## 8. Table talk and banter

The detail is in notes 05 (glossary and a 60-row table of lines), 05b (dictionaries and proverbs, about 70 sayings), 05c (literature and feuilletons, about 100 rows) and 05d (other languages, about 60 rows). Each row gives the original, a translation, the source and date, the game moment it fits, and a short PG line. This section picks out what shapes the design.

### 8.1 The calls
- **Bidding.**
  - "Ich nehme auf!" (I take it up), "Weiter!" (pass), "Ich passe!" (I pass) [02-S2][05 §2].
  - Older calls from Tapp-Tarock and Königrufen, not recorded for Strohmandeln: "Ich spiele" (I play), "Halte ich" (I hold), and "Ich tappe" (I tap), later replaced by "Weiter" (Dummett) [05d POLY-S32] [REVIEW RF: Dummett pp. 453-454 give these for the three-hand games; they were presented here as Strohmandeln bids].
- **Honours.** "Tous les trois!" and "Tous les quatre!" (Ulmann's names for the Trull and four kings) [03-S14]. "Ich skisiere mich!" (I play my Sküs and excuse myself) [05b LEX-S22].
- **Doubling.** The Viennese word for doubling a bid (Kontra) was *spritzen*; Kontra is not in the canonical rules, so this is period colour only [14 rule 6]: "Ich spriz den Dreier" (I double the Dreier) (Hügel via Wander, 1876). In 1898 a paper writes of "einem gespritzten 'Dreier' und einem angesagten 'Pagat'" (a doubled Dreier and an announced Pagat) [05b LEX-S15][05b LEX-S28].
- **Rostopschin** (not in the canonical rules [14 rule 23]). "Ross!" … "Topschin!" (Torberg, as quoted on Wikipedia; Mayr & Sedlaczek's 2002 page gives XVII/XVIII and calls it "recht beliebt", fairly popular, between the wars, "vermutlich vor allem im Vierer-Tarock", probably mostly in four-hand Tarock; the same page adds "Auch im Strohmann-Tarock führt sie zu interessanten Konstellationen", in Strohmann-Tarock too it leads to interesting situations) [12 item 1][12-S5] [REVIEW RF: the last clause was omitted; it is on the saved Wayback page].

### 8.2 The kibitzer as the stock comic figure
- **"Kiebitz, halt's Maul!"** (Kibitzer, shut your trap!):
  - "einer der Hauptparagraphen" (one of the main paragraphs) of card-table law in 1895 [03-S34];
  - "§ 13" from 1897 [RD];
  - "Das 'Kiebitz'-Reglement hat auch nur einen Paragraphen" (the kibitzers' regulations also have only one paragraph) in 1904 [05c LIT-S44] [REVIEW RF: German wording corrected to the note's quotation];
  - on a large sign in an inn, Grazer Volksblatt 1904 [05c LIT-S40].
- Wander records the word as Austrian by 1880: "Es ist ein Kibitz. So nennt man in Oesterreich die Zuseher beim Spiele" (He's a kibitz: that's what they call onlookers at a game in Austria) [05b LEX-S12].
- A Baden club of 1908 had an "Ehrenkibitz" (honorary kibitzer) [05b LEX-S29].
- **Design proposal.** Make the tutor a period kibitzer at the player's shoulder, and use "§ 13" as the button that silences it [05c §Headline 1] [REVIEW RF: was §Headline 2, which is the Nestroy item].

### 8.3 Lines that fit game moments (a shortlist)

| Moment | Said (source) | Game line |
|---|---|---|
| New hand | "Ihr Blatt!" (Your hand). The full line "Ihr Blatt, Herr Rosenblatt!" is from *Der Bockerer* (1948), where it belongs to the reunion at the end of the play, not to the start of a hand; known only as reported by Sedlaczek & Mayr (second-hand), and tied to the Shoah narrative of the play. RB struck the name under the kindness rule, so only "Ihr Blatt!" is proposed [05 row 52][05c LIT-S67][RB] [REVIEW RF: name removed from the table; placement and second-hand status added] | "Your hand, sir." |
| Slow to play | "Eine Karte oder ein Scheit Holz" (a card or a log of firewood) (1924) [05c LIT-S10] | "A card or a log of firewood." |
| Slow to play | "Wecken S' mich auf, wenn der Herr dort drüben ausspielt!" (wake me when the gentleman over there leads) (1924) [05c LIT-S10] | "Wake me when you lead." |
| Catching the Mond | "'Guter Mond, du gehst so stille' summt der Bankdirektor und sticht mit dem Sküs den gegnerischen Mond ab" (the bank director hums "Good moon, you go so quietly" and takes the opponent's Mond with the Sküs) (Ranzenhofer, 1917) [05c LIT-S8] | (hums) "Good moon, you go so quietly…" |
| Misplay | "Versehen ist verspielt" (a slip is forfeited: a card played in error stands) (1892–1916) [05c LIT-S37][05c LIT-S38][05c LIT-S39] | "A slip is a play, I'm afraid." |
| Winning on 36 | "35 und 2 Blatt; das ist eigentlich der stolzeste Sieg" (35 and two cards: really the proudest win) (1908; in the source a parliamentary joke about a two-thirds vote that borrows the Strohmandl margin, not a table remark) [03 §4][RD] | "Thirty-five and two: the proudest win." |
| Lucky pull from a packet | "Schmiert … der Strohmann … dem Spieler viel Könige" (the straw man feeds the player many kings) (1829) [03-S4] | "The straw man is feeding you kings!" |
| Holding the Pagat | "Hohó! Hiszen nálam van a pagát!" (Oho! I've got the Pagat!) (Jókai, 1895) [05d POLY-S10] | "Oho! The Pagat's mine!" |
| A dud Trull | "Endlich hat man die Trull, und da heißt's, 's is vergeb'n" (at last you hold the Trull, and they say it's a misdeal) (Nestroy, 1835, Bonz 1890 text; the modern text reads "steig'n tous les trois auf") [05c LIT-S1][05 row 17] | "At last the Trull, and it's a misdeal." |
| A shuffle | "Ein guter Misch ist das halbe Spiel" (a good shuffle is half the game) (Ranzenhofer, 1917) [05c LIT-S8] [REVIEW RF: was attributed to Löwy 1903 [LIT-S9]; note 05c row and LIT-S8 give Ranzenhofer, "Königrufen", Pilsner Tagblatt 1917, p. 2] | "A good shuffle is half the game." |
| The Piquet joke | "nur von Leuten …, denen das edle Piquetspiel gänzlich unbekannt ist" (only for people who don't know the noble game of Piquet) (1829; said of the old dummy-pile game, not the modern one) [03-S4][RG] | "Have you not heard of Piquet?" (only as a joke about the 1829 game) |

Note 05c gives the Nestroy lines from the 1890–91 Bonz edition. A modern text differs, so every Nestroy line in the game must name its edition [RB][RD].

### 8.4 Period-correctness traps
- **The bird names come too late for a 19th-century voice.** Uhu, Kakadu and Quapil for trumps II, III and IIII date from the 20th century: Uhu first appears in Hungary in 1902, according to Wikipedia (second-hand) [05 §Headline 6][RB].
  - The bird bonuses are not in the canonical rules [14 rule 22].
  - The opponent's 19th-century voice should say "der Zweier" (the deuce), not "der Uhu".
  - The card faces may still carry the nicknames as a learning aid (§10).
- **Pagat ultimo was not announced in Strohmandl.** Werner, Bermann and Ulmann all rule out announcing it [03 §2]. An 1887 feuilleton's "angesagte" (announced) game is about the bid, not the Pagat [RB].
- **"Mondfang" is not recorded for Strohmandeln in any period source** [02 §Headline 4]. It is attested in period German for the other Tarock games (Werner's Königrufen section; press 1887, 1893, 1925) [05 §1 Mondfang row][05c LIT-S20, S55, S56]; note 05d's "no period German Mondfang" reflects only an archive.org full-text search [05d] [REVIEW RF: contradiction between notes 05 and 05d resolved in favour of 05/05c, which quote the texts].

### 8.5 A Viennese opponent's vocabulary
- *Patzer* (bungler), *Gstieß* (Sküs) and *Spatz* (sparrow, for the Pagat) [05c §Headline 8].
- *Skartindeln* (worthless low cards) and *packeln* (to stack the trumps when gathering) [05c §Headline 8].
- "Er hängt scho' der Max'" (Hügel 1873: "his wish is close to fulfilment"; Max' is also, separately, a dialect name for the Sküs card, with a folk etymology). Whether the phrase was ever said of drawing the Sküs is the note's inference, not Hügel's [05 §Headline 8][05-S1] [REVIEW RF: was cited as [05 §3] and glossed as if the phrase meant the Sküs].
- "Schmied von Penzing" (the smith of Penzing: one made to deal again and again) [05 §3].
- "Zieh endlich deine Fäustlinge aus!" (take your mittens off at last!): known only from a newspaper review of Mayr & Sedlaczek's *Kulturgeschichte*, not from the book, and said of a slow or fumbling deal [09-S5][RA] [REVIEW RF: second-hand status added].

### 8.6 What stays out
Excluded under the kindness rule, though attested [03 §4][04][05 §Kindness][05c §K][RB][RD]:
- the Pagat-ultimo slang "der Jud" (the Jew), and the Czech *žid* for the kitty;
- a run of 1900s–1930s ethnic jokes that use Strohmandl as a prop;
- "von ein paar alten Jungfern erfundenes Kartenspiel" (a card game invented by a couple of old maids);
- Nestroy's wife joke;
- "Tschappel" (simpleton) and "Graßl" (a notorious robber);
- one double meaning [RA].

---

## 9. Strategy and numbers

- **When to take up.**
  - Werner (1891): "3 Honneurs, 6 bis 7 Taroks und eine oder zwei besetzte Damen, respective Cavalls" (three honours, six or seven tarocks and one or two guarded queens or cavalls) [03-S12].
  - Bermann (1894): "in Honneurs und höheren Figuren etwa 20 bis 25 Points in der Handkarte" (about 20 to 25 points in honours and high figures in the hand) [03-S13] [REVIEW RF: word order corrected to the source].
  - Ulmann: three well-guarded honours; with four or more, always [03-S14].
  - Mayr & Sedlaczek: 20–25 points in sure tricks (counting a Trull card or king as 4⅓, a queen as 3⅓) and at least 6–7 tarocks [07 §1.1].
  - Original computation: only about 3–7% of 15-card hands meet the 20-point rule. Followed strictly, neither player would bid in nine deals out of ten, so the computer's bidding threshold must be tuned in play, not copied [07 §5.4].
- **Managing the straw men** [07 §1.2][03-S12][03-S14]:
  - play your packets off quickly, to learn what is under them;
  - but keep a packet unplayed when the opponent threatens a Pagat ultimo, to keep a buried tarock in reserve;
  - never run out of hand cards while packets remain;
  - play an exposed queen at once. Mayr & Sedlaczek say she gets through "4 in 10" times. Original computation confirms this exactly under their stated condition: when you don't hold her king, he is still asleep in a packet 39.8% of the time [07 §5.5][RB].
- **Suit play decides.** Finesse (*Impassiren*); don't lead a suit whose counting cards are evenly split; force the opponent to lead it [03-S12]. "Kings almost never get lost; the decision usually lies with the queens and cavalls" (Bermann) [03-S13]. Feints: follow the opponent's king with your cavall so he won't lead the queen (1898; Ulmann) [03 §4].
- **Honours.** Delay announcing them; don't trump early with an honour while the third Trull card or fourth king may still turn up [03-S12].
- **Numbers (original computation)** [07 §5]:
  - average tarocks in a 15-card hand: 6.1;
  - Trull in hand: 1 in 55; four kings in hand: 1 in 232; no tarock at all: 1 in 15,298;
  - a player's 12 packet cards average 4.9 tarocks and 0.9 kings;
  - P(at least one of your three packets yields nothing at the opening) = 0.19;
  - card points total 70 under counting by threes.
- **No published statistics** on how often games are taken up, or average scores, were found [07 §2].

---

## 10. The cards

**Decided (14 October 2026):** draw original cards in the large-type high-visibility style of the cassino and piquet phone faces; reproduce no period art.

**What the real pack looks like, as a guide:**
- **Suits** run K Q C J, then 10 9 8 7 in black and 1 2 3 4 in red [08 §Headline].
  - Real suit cards have no corner indices at all; the rank labels here are a deliberate departure, and the tutor can say so [08 §Headline].
- **Trumps** carry Roman numerals top-left and bottom-right, rotated, and the Sküs has none [12 item 9].
  - Only the four is written additively, IIII; nine, fourteen and nineteen are IX, XIV and XIX [12 item 9][RA].
- **Nicknames, for the face labels:**
  - I Pagat (also "Spatz"), II Uhu, III Kakadu, IIII Quapil, XXI Mond; the Sküs is "Gstieß" in dialect [08 §Headline][05c].
  - No nickname for XVIII or XX was found [08].
  - The nickname "Mond" is a false Germanisation of Italian *mondo* (world) [08-S17].
- **Tutor jokes:**
  - trump II, called the Uhu (eagle-owl), shows a crowned eagle on a rock lettered "INDUSTRIE UND GLÜCK" (industry and luck) [12 item 9][08-S20];
  - Quapil is the Czech surname Kvapil, "he has hurried"; the link between the card and the surname is an inference [09 Best of 8][RA].
- **History.**
  - The earliest known Industrie und Glück pack is by Hofmann, Vienna, tax-stamped 1815 [08-S1].
  - Piatnik's current pattern goes back to Josef Neumayer (1884, 1890); today's packs are 63 × 113 mm [08-S2][08-S11].
- **Backs.** Historic designs include the Wiener Halbmond (black), the Wiener Blitz, blue rocaille on white, and red/green tartan [08 §Headline]. A plain pattern is what is wanted for the face-down packets.
- **Reference scan.** A complete Piatnik pack of 1903–12 in the Volkskundemuseum Wien (ÖMV/87529) is marked CC Public Domain Mark 1.0 in the museum's own record. Contact sheets are saved in `research/sources/12/omv/` [08][RA][12 item 9].
- **The phone-face recipe** to reuse is piquet commit d2507ac (canvas-drawn bold index, fitted to the strip of each card that a fanned hand shows) [08 §3][RC].
  - Tarock needs two changes: numerals up to XVIII are wider than any French rank, and the Sküs needs a mark of its own [08].

---

## 11. Digital implementations and the computer opponent

- **No playable digital Strohmandeln was found** in the searches of notes 07 and 12. RG notes that Austrian Tarock apps by publishers, generic tarot engines and Piatnik's own digital products were not searched, so this is "none found", not "none exists" [07 §3.1][12 item 8][RG].
  - The nearest is `smatkovi/harbour-tarock` (GitHub, created September 2026, GPL-3.0-or-later). It has a long German specification of the game, a "hints for the computer opponent" chapter and a straw-man row component, but plans to make Strohmandeln playable only "after 1.0" [07 §3.1][12 item 8].
  - It is cited and nothing is copied. Its 21 rule interpretations are listed in note 12 for comparison [12 item 8].
- **Officers' Skat**, with the same pile mechanism, exists in KDE LSkat and on iOS [07 §3.2].
- **AI.**
  - The relevant literature is Gams & Luštrek's Tarok program (alpha-beta search over sampled deals) and Long et al. (2010) on why sampling a complete deal and solving it works [07-S6][07-S7].
  - Strohmandeln is a textbook case for that method, with one twist: a player's own unturned packet cards are unknown to him too, so the opponent must sample its own hidden cards as well as the other player's [07 §Headline 5].

---

## 12. Reusing the earlier games

- **Nothing is shared yet, by design.** Bezique's plan says "no shared crate until a third game" (`~/bezique/PLAN.md:155`) [10][RC].
- **Cassino is the most evolved and the best base** [10]. Piquet uniquely has the celebration scenes (7 of them) [RC].
- **Identical across piquet and cassino, apart from a header line:** `art.js`, `bag.js`, `cards.js`, `deck.js`, `materials.js`, `surfaces.js` [10][RC]. Cassino has the better version of `kinematics.js`, `framing.js`, `scene.js`, `faces.js`, `dialogue.js`, `tutorial.js` and `director.js` [10].
- **Rust:** the random-number module is almost identical in bezique and cassino. Bezique's `tricks.rs` (follow suit, else trump) is the closest ancestor for Strohmandeln's trick logic [10].
- **How to share is disputed.**
  - Note 10 recommends a `card-table-kit` git submodule.
  - The review (RC) found that the bundler cannot resolve the game's `three` from inside a submodule (reproduced), that git worktrees don't populate submodules, and that cassino keeps changing, so the kit would split off from it at once.
  - RC recommends a vendored `kit/` directory with a stamp file naming the source commit, promoted to a real package once a second game uses it [RC].
- **Rules to inherit** [10]:
  - test-driven work and atomic commits;
  - `bin/gate` before every commit, never piped;
  - a browser check before table commits;
  - simulations with sequential stopping on mirrored pairs;
  - the engine holds the rules and the clients hold none;
  - public-facing documents that never name the maintainer;
  - one offline HTML file;
  - "your opponent", never a proper name;
  - every asset credited in `CREDITS.md` in the same commit.

---

## 13. What review changed, and how far to trust each note

| Note | Verdict | Main corrections |
|---|---|---|
| 01 Wikipedia | usable with care, close to trustworthy | "zu 5 Karten" means dealing in fives; a Ulmann quote concerned Neunzehnerrufen [RA] |
| 02 Modern rules | usable with care | Werner's last-card condition restored; Torberg relabelled second-hand [RA][RC] |
| 03 Period rulebooks | **trustworthy**, with one uncorrected slip found later (the 1850 report's winner is reversed; see §6 and [RF]) | modern game dated c. 1886–88, not 1883; "1839 = 1842" marked as inference [RD] |
| 04 Cultural history | usable with care, close to trustworthy | Nestroy line is a stage metaphor; Tepl, not Teplitz; one win threshold, not three [RB] |
| 05 Table talk | usable with care | Nestroy edition named; three lines struck under the kindness rule [RB] |
| 05b Lexica | **trustworthy** | — [RB] |
| 05c Literary banter | usable with care | § 13 dated 1897; the Rosenau text survives; "Ich bin matsch" is from *Nagerl und Handschuh* [RD] |
| 05d Other languages | usable with care, largely trustworthy | the Polish 1821 text is a translation, not independent [RD] |
| 06 Family | usable with care | the 1932 page is p. 370; Baronetti has 4 piles; Werner's condition [RC][RE] |
| 07 Strategy and numbers | usable with care | the ">11 tarocks" figure is 0.04%; "4 in 10" confirmed [RB] |
| 08 Cards | **trustworthy** | licence confirmed in the museum's record [RA] |
| 09 Hunches | usable with care | Freud and the banter lines are second-hand; one line struck [RA] |
| 10 Prior games | usable with care, mostly accurate | 8 of 48 claims fixed; submodule advice judged weak [RC] |
| 11 Other languages | usable with care | Czech 1912 and 1930 forms weaker; Hungarian pages and glosses fixed [RE] |
| 12 Follow-ups | usable with care, close to trustworthy (reviewed by RF) | "Anschowitz" is Auschowitz; the Ulmann/"jejune" misattribution softened; the eagle does not hold the sword [RF] |
| 13 Sessions and scoring | usable with care (reviewed by RI) | the book evidence for Touren, the Conto, Block-Tarok and the Brunnen is accurate. Weak points: no source applies any of it to two players ("Tour = 2 deals" is inference), the Znaim "15 deals per sitting" figure is from a partnership game, the Ehrentour trigger in design C was wrong, "drei Meister" means three deals not three Touren, and several Bermann page numbers were scan numbers [RI] |
| 14 Canonical rules by consensus | usable with care, after fixes (reviewed by RH) | the "Furr follows Mayr & Sedlaczek" dependence is unproven; rule 4 (void deal) was wrong; rule 10's "3–3" was an overreach and is 3–1; Unger belongs in the period column; the Valat phrase was Ulmann's, not Bermann's; "unanimous" for a quiet Pagat ultimo failed for Widder [RH] |

**Review logs**

| Review | What it did | Main results |
|---|---|---|
| RF | reviewed note 12 and the first version of this synthesis | about 30 corrections, none touching the core rules account. The worst: the Löwy/Ranzenhofer misattribution, Mayr & Sedlaczek's packet timing (in three places), the reviewer's praise credited to Ulmann, the reversed winner of the 1850 report, the Bockerer line, the Tepl "joke" overstatement and an invented Czech quotation [RF §F] |
| RG | fresh-eyes critique, edited nothing | a plain-English "how it plays" draft, the "none found" and "two games and an ancestor" softenings, the Piquet sneer aimed at the 1829 game, the "70 in all" remainder question, and a list of unpulled leads (§14) [RG] |
| RH | reviewed note 14 | the canonical ruleset and the three settings stand. Weak points: rule 10's tie-break, Valat's possible fourth split, and the unproven Furr dependence [RH] |
| RI | reviewed note 13 | design A stands as a reconstruction of multi-hand practice, with four caveats: no source applies it to two players, "1 Tour = 2 deals" is inference, the ~15-deal sitting figure is not Strohmandl evidence, and the Ehrentour trigger was wrong [RI] |

---

## 14. Decided, and open questions

**Decided** (all on 14 October 2026; the notes do not record the date) [REVIEW RJ]:
1. **Which game: the modern one.** The 1829 dummy-pile game could still be a short historical curiosity mode, which is simple to add and which the period called dull; that is a suggestion, not a decision.
2. **The rules: canonical by consensus of the legitimate sources**, with settings only for disagreements that persisted through the game's life (both players passing, game values, Pagat ultimo value). Recorded in note 14 [14 §Settings].
   - The last packet card is taken into the hand and shown, with no setting (decided after RH, 14 October 2026) [14 rule 10].
   - Valat stays at 12, with no setting (14 October 2026) [14 §Decisions].
3. **The session: design A of note 13**, the period "Abend": 3 or 6 Touren of two deals, a running slate, settling up at 2 kreuzer a Point, a draw doubling the next deal. Details settled with RI (§4) [14 §Decisions]. A race to a target score (design B) is not canonical.
4. **The cards:** original large-type cards, with Roman-numeral tarocks, the nicknames on the faces and a plain back (§10).
5. **The jack label: "J".** Chosen "J" over "V" (Valet) on 14 October 2026, as the label that fits the audience. The period German name was *Bube* [08].

**Still open:**
- **How to share code** with the earlier games: a vendored kit with a stamp file (the review's advice) or a submodule (§12).
- **The currency of the stake:** gulden and kreuzer against krone and heller, and the dates of the change, which note 13 gives without a source [RI (c)].

**Research still open:**
- Torberg's *Tante Jolesch* p. 217 (lending-restricted) [12 item 1].
- The Freud–Ferenczi letter, which has no primary text found [12 item 2]. RG adds that it is a four-hand game and should not be used in a two-hand tutor without saying so [RG].
- Haas 1839 p. 121 [12 item 7].
- Ulmann's *Buch der Familienspiele* [12 item 3].
- Unger's manuals of 1918–26 [03 §8]. Löw 1956, Beck 1972 and Bamberger 2011 are also unread, so the modern consensus is second-hand, and the 1920s to 1950s are almost blank in rules [RG].
- The text of the Czech 1932 Štromandl section [RE].
- The Hungarian Jánoska Antal tarokk guide [RE].
- The Vanderheid 1855 appendix [03-S11].

*From RI (note 13 and its review):*
- Ulmann's *Tarok-Codex* (Spielgesetze) may have a rule on Touren and disputes; it was not read [13 §Gaps][RI].
- Newspaper items in note 13 are snippet-level. The Znaim 1880 article, the Agramer "drei Meister" note and the 1887 *Neues Wiener Tagblatt* "Strohmandl zu Zweien" feuilleton should be read in the page image [13 §Gaps].
- Strohmandl-specific stakes and money customs: the 1895 Café de l'Europe trial and the 1929 24-hour marathon could be read in full [13 §Gaps].
- The Tarockcup rules (Hausruck, Mühlviertel, Wien and others), Hungarian tarokk clubs and Czech tournaments were not retrieved [13 §Gaps].
- Piquet partie and rubber, Préférence, Mariage and Zwicken session units were not researched, and neither was the earliest source that defines "Bummerl" [13 §Gaps].
- Bermann pp. 143–146 describe Strohmann-Tarok as played "in zweifacher Weise" (in two ways): every dealt hand played out at an agreed price, or with the "Aufnehmen" declaration. RI's finding is not yet in notes 01 and 02 [RI §New findings].

*From RG's unpulled leads* [RG (b)]:
- Kosakeln, which Pagat calls the only other two-hand Tarock game, as a "see also" or second mode.
- Thomas Burgstaller, *Tarock zu zweit* (2017), with an appendix on classic two-hand variants: whether the notes read it is unchecked.
- Schnitzler's diary (digital edition, 1879–1931), searchable in full: the site's own search was not run.
- Karl Kraus, *Die Fackel* (full text), and feuilletonists such as Polgar, Friedell, Altenberg and Kisch: an in-corpus search.
- The two Wienbibliothek PDFs (3752218 and 3724659): confirm both were read in full. One may be a different rule book from Werner.
- ANNO under hyphenated and k/ck spellings: "Strohmann-Tarock", "Strohmandl-Tarok", "Strohmanntarock". The seven harvested spellings may miss them.
- Piatnik's own archive and the Vienna playing-card museum: whether a rules leaflet was printed in 1900s packs.
- Austro-Hungarian officers' mess Tarock and the novelists (Roth, Doderer, Musil, Zweig), not searched.
- Women and the game: a coffee-house game for two with husband-and-wife jokes is a gender-history angle left lying.
- The 1921 coffee-house tariff, to cross-check for inflation.
- The hit rate: how many of the 305 ANNO issues are the card game.
- A tier (primary, second-hand, legend) on every banter line used in the game, and a second source for the "Mond is a false Germanisation of *mondo*" etymology, which rests on 08-S17 alone.

*Design checks RG asked for before engine code* (not research):
- an exhaustive test over the remainder cases of the card-point count, since RG's arithmetic says the total is 70 only when both players' remainders are 0; no source in the notes addresses this [RG (a) 1][RF §E];
- the order of the opening reveal, the honours-announcement moment, and a banter trigger model [RG (a)].

*Left open by RH:* whether Beck's dash in Wikipedia's table means "no Valat"; and whether Furr is independent of Mayr & Sedlaczek (no result changes either way) [RH].

---

## Appendix A. Research notes and review logs

| File | Contents |
|---|---|
| `research/_BRIEF.md`, `_REVIEW_BRIEF.md` | the rules the notes and reviews were written under |
| `research/01-wikipedia.md` | Wikipedia in 45 editions; citations followed back |
| `research/02-modern-rules.md` | modern rules compared; 18 ambiguities; canonical set |
| `research/03-period-rulebooks.md` | period texts 1800–1899 transcribed; divergences; timeline |
| `research/04-cultural-history.md` | press, courts, literature, 1850–1938 |
| `research/05-table-talk.md` | glossary; calls; 60-row banter table |
| `research/05b-proverb-lexica.md` | Grimm, Wander, Meyers, Adelung, Campe, DTA |
| `research/05c-literary-banter.md` | Nestroy, feuilletons, ~100 banter rows |
| `research/05d-polyglot-terms.md` | Czech, Hungarian, Slovene and Polish terms and calls |
| `research/06-family-and-languages.md` | straw-man games; the word *Strohmann*; Czech |
| `research/07-strategy-stats-digital.md` | strategy; original computation; digital; AI |
| `research/08-cards-and-art.md` | the pack; nicknames; backs; asset options |
| `research/09-hunches.md` | wildcard finds, ranked by use in a game |
| `research/10-prior-games-survey.md` | piquet, bezique and cassino surveyed for reuse |
| `research/11-habsburg-languages.md` | the game in the empire's other languages |
| `research/12-followups.md` | follow-ups resolved |
| `research/13-sessions-and-scoring.md` | how a period sitting was structured, scored and settled |
| `research/14-consensus-rules.md` | the canonical ruleset by tally of the sources; the settings; the session decision |
| `research/review/RA.md` … `RI.md` (RG is a critique that edited nothing; RH reviews note 14; RI reviews note 13) | review logs |
| `research/review/synthesis-v2-changes.md` | changelog of this revision |
| `research/_FOLLOWUPS.md` | running list of follow-ups |
| `research/sim/` | the original-computation scripts |
| `tools/anno.py` | the ANNO harvester |
