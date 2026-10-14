# Strohmandeln: A Literature Review for Game Development

*Prepared 14 October 2026. Scope: history, rules and their variants, terminology, table talk, social and cultural life, the game in the languages of the Habsburg empire, related games, strategy and numbers, the physical cards, digital implementations, and what this project can reuse from the earlier games in the series (Piquet, Bezique, Cassino). Written for a team building a video-game version: a single offline web page with a computer opponent that talks in a period voice and a tutor for players who have never heard of the game.*

---

## How to read this document

- **Citation rule.** Every factual statement carries a citation in square brackets, pointing into the research notes in `research/`, where each source is listed in full (URL, page, and the local copy under `research/sources/`).
  - `[NN-S#]` is source S# of note NN (e.g. `[03-S12]`). Notes 05b, 05c and 05d number their sources `LEX-S#`, `LIT-S#` and `POLY-S#`, cited here as `[05b LEX-S12]` and so on.
  - `[NN §x]` points to a section of a note where an argument or table is developed.
  - `[RA]`…`[RE]` and `[RF]` point to the review logs in `research/review/`, which record every correction made after the notes were written. `[RF]` is the review of note 12 and of this synthesis; edits made in this document by RF carry `[REVIEW RF: …]` tags.
- **Quotations are verbatim**, with an English translation after each non-English quote. Fraktur long-s is normalised. `[OCR]` marks text read from machine OCR, `[IMG]` text read by eye from a page image.
- **Reliability flags**: `[UNVERIFIED]` means the claim could not be confirmed in a fetched source; `second-hand` means a primary text is known only as quoted by someone else. Wikipedia is treated as a pointer, never as a source: note 01 followed its citations back, and found three errors in the English article [01 §Headline 5–7].
- **Original computation** (labelled as such) comes from the scripts in `research/sim/`, re-run and reproduced exactly during review [07 §5][RB].
- **Kindness rule.** The period sources contain ethnic, religious, gender and age jokes. They are recorded in the notes as history, flagged, and never proposed as game lines (§7.6).

## Method

1. **Thirteen research strands**, each producing a fully cited note, run as parallel agents under one brief (`research/_BRIEF.md`):
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
2. **Libraries reached**:
   - the Austrian National Library's book scans (ÖNB ABO, via its IIIF API) and its newspaper archive ANNO;
   - the Wienbibliothek;
   - archive.org, Wikisource, Projekt Gutenberg, the Deutsches Textarchiv and woerterbuchnetz (Grimm, Adelung, Campe, Wander);
   - Czech Kramerius, and the Hungarian REAL-EOD;
   - the Wayback Machine, GitHub, OpenAlex.

   ANNO's search page needs JavaScript, but the JSON service behind it is open. A small harvester (`tools/anno.py`) was written for it and pulled every snippet for seven spellings of the game's name (305 newspaper issues, `research/sources/anno/strohmandl.jsonl`). Google Books, HathiTrust, Arcanum, Hungaricana, Polona and dLib were blocked or rate-limited from the research machine. No bot check or lending restriction was bypassed.
3. **Five adversarial review passes** (RA–RE) re-checked the notes against the saved sources:
   - about 2,000 quoted strings were extracted from the notes and about 1,300 of them matched the saved texts automatically; most of the rest were English translations, headings or extraction artefacts, and the genuine misses were checked by eye (counts from the logs RA–RE; the earlier "more than 1,500 matched" overstated this) [REVIEW RF];
   - dozens of newspaper page images and crops were read by eye;
   - errors were fixed in place and logged.

   No invented quotation was found. The errors were misreadings, wrong page numbers, and claims resting on a secondary source presented as if the primary had been read (§12).

---

## Executive summary

**What the game is.**
- Strohmandeln is a two-player trick-taking game of the Austrian Tarock family, played with the 54-card Tarock pack: 22 trumps (*Tarocke*) and 32 suit cards [03-S12][02-S1].
- Each player holds 15 cards in hand and has three face-down packets of four cards, the *Strohmänner* ("straw men"), in front of him.
- The top card of each packet lies face up. A tarock or king that turns up there is shown to the opponent and taken into the hand. Any other card stays face up as part of its owner's hand, to be led, followed or trumped with.
- A card used from a packet uncovers the one beneath it [03-S12][03-S13][03-S14].
- Card points are counted as in all Austrian Tarock; there are 70 in the pack and 36 wins [03-S12][07 §5.1].

**Its distinctive mechanic.**
- No relative found has the automatic "pull" of kings and tarocks from the packets into the hand [06 §Headline 4].
- Its oldest describers prized its suit play and finesses over its trumps. Bermann (1894): "Im Strohmann-Tarok tritt die ausschlaggebende Bedeutung der Trümpfe … am wenigsten hervor" (in Strohmann-Tarok the decisive importance of the trumps shows least) [03-S13].
- Werner (1891): "Die größte Rolle beim Strohmann-Tarok spielt das Impassiren. Dies allein entscheidet meist über den Ausgang der Partie" (finessing plays the biggest part; it alone mostly decides the game) [03-S12][12 item 6].

**Three games have carried the name** [03 §Headline 1]:
1. **"Taroc en deux"**, Berlin 1800 and reprinted to 1846: a 78-card game in which 25 cards are set aside unseen [03-S1][03-S2][03-S8].
2. **The first "Strohmandeln"**, Vienna 1829–1853: ordinary three-hand Tarok-Tappen with the missing player as a face-down pile. Whoever wins a trick takes its top card for points [03-S4][12 item 4].
3. **The modern game**, firmly attested from 1888 (an 1886 list naming "Strohmann-Tarok (en deux)" is an OCR snippet only): the game described above [03-S27][03-S15][03-S14] [REVIEW RF: "firmly" now rests on the 1888 review and the 1891/1894/1899 books; the 1886 hit is snippet-only].

The name itself is older than any rules: it appears as a bare name in a list of Tarok kinds in a Vienna/Prague book of 1824 [06-S3][RC].

**What its contemporaries thought of it.**
- The first version was scorned in print. From 1829 the same sentence calls it a game for "Leuten …, denen das edle Piquetspiel gänzlich unbekannt ist" (people to whom the noble game of Piquet is entirely unknown) [03-S4]. It was reprinted to 1853 [03-S5][03-S7][12 item 4].
- The modern game was praised. The *Klagenfurter Zeitung*'s review of Ulmann's *Tarokbuch* (30 March 1888) says Strohmann-Tarok "ist hiedurch zum Lieblingsspiele aller denkenden Tarokfreunde geworden" (has thereby become the favourite game of all thinking Tarock lovers) [03-S15] [REVIEW RF: the words are the reviewer's, seen on the page image, not Ulmann's; the synthesis had said "Ulmann (1888) calls it"]. Bermann (1894) says it is "unter feinen Spielern sehr beliebt" (much loved among fine players) [03-S13].
- In the press, from 1850 to 1938, it was the coffee-house game for two, and the fallback when the third player for a Tapper failed to come [03 §4][04 §Headline 1]. "Können Sie nicht 'Strohmandeln'? — Zu dienen." (Can't you play Strohmandeln? At your service.) (*Figaro*, 1888) [03 §4].

**Where it lived.**
- Vienna above all, and Austria and Bohemia more widely [04][03 §4].
- Czech borrowed the name as *štromandl*, declined as a Czech noun from at least 1891, though two of the five attestations are weak [11][RE].
- A modern Hungarian name, *Strohmandli*, is given by Mayr & Sedlaczek and by a modern Hungarian site [RE].
- No Slovene, Polish, Croatian or Italian description was found. Several of those libraries could not be searched [11][RE].

**Why it matters for this project.**
- The rules are well documented in three period books (Werner 1891, Bermann 1894, Ulmann 1888/1899) and four modern ones (Beck 1972, Dummett 1980, Mayr & Sedlaczek 2007, Furr 2009) [03 §2][02 §1]. Beck was never read (known via Dummett and Wikipedia), and Dummett is a digest of other authors [REVIEW RF: caveat added].
- Each difference between them becomes a rules option (§3.3).
- There is a deep well of period table talk: calls, kibitzer jokes, court reports, Nestroy couplets [05][05c][05d].
- No playable digital version of the game exists [07 §3.1][12 item 8].

---

## Contents

1. What the game is (the modern rules, in brief)
2. History: three games under one name
3. Rules: the canonical set, the options, and the questions a programmer will meet
4. The family: straw men, dead men and dummies
5. Social life: coffee-houses, courts, couples and kibitzers
6. The game in the empire's other languages
7. Table talk and banter
8. Strategy and numbers
9. The cards
10. Digital implementations and the computer opponent
11. Reusing the earlier games
12. What review changed, and how far to trust each note
13. Open questions and decisions for the maintainer
- Appendix A. Research notes and review logs

---

## 1. What the game is (the modern rules, in brief)

The account below follows Werner (1891), the earliest full text, with the modern consensus noted. Every point and every divergence is sourced in §3.

- **Players and pack.** Two players, with the 54-card Tarock pack [03-S12][02-S1]:
  - the Sküs (highest trump), then XXI down to I;
  - in each suit K, Q, C (Cavall, the knight), J;
  - then 10 9 8 7 in black suits and 1 2 3 4 in red suits, highest first [08 §Headline][02 §4 item 1].
- **Deal** [03-S12]:
  - the dealer gives 5 cards to forehand (the non-dealer), then 5 to himself, three times over, so each has 15 hand cards;
  - the remaining 24 cards are laid face down as six packets of four, alternately, three in front of each player.

  Ulmann lays the packets only after someone has bid [03-S14]; Bermann has two modes, one of them after the bid [03-S13]. Mayr & Sedlaczek deal the packets with the hands; their page says dealing them after the bid is only expedient if one adopts the "old books'" throw-in rule for both-pass [02-S2] [REVIEW RF: the synthesis had put Mayr & Sedlaczek with Ulmann].
- **Bidding.**
  - Forehand says "Ich nehme auf" (I take it up, i.e. I'll play this hand as a contract) or passes ("Weiter", "Ich passe"). If forehand passes, the dealer may take it up [03-S12][02-S2].
  - If both pass, the sources split: play a "simple" game for less (Werner, Mayr & Sedlaczek, Furr and others) or throw the cards in (Ulmann's usual practice, Beck as reported by Wikipedia, Bamberger) [03 §2][02 A11].
- **Opening the straw men.**
  - Each player turns the top card of each of his packets in turn. A tarock or king is shown to the opponent and taken into the hand, and the next card is turned. The first card that is neither stays face up on its packet [03-S12][03-S14].
  - On average 2.8 cards come into the hand this way, and none at all 13% of the time (original computation) [07 §5.3].
- **Play.**
  - Forehand leads; the winner of each trick leads to the next. 27 tricks are played [03-S12][02 A8].
  - The face-up packet cards are part of the hand: a player may lead, follow or trump with them [03-S12][03-S13].
  - You must follow suit. A player who cannot follow from hand must follow from a face-up packet card of that suit if there is one [03-S12][02-S2]. If you can't follow at all, you must trump if you can (Werner: "sonst mit Tarok gestochen werden"); if you can do neither, any card may be played (Furr). Sources imply no obligation to win the trick (Stichzwang) [02 §4 item 9][03-S12][03-S13] [REVIEW RF: wording tightened to what Werner and Furr say; the no-Stichzwang point is an implication, not an explicit statement in Werner or Bermann].
  - After the trick is gathered, each used packet card uncovers the next. A tarock or king goes to the hand; anything else stays face up [03-S12][02 A5].
- **The last card of a packet.**
  - Modern sources (Beck, Babsch, Mayr & Sedlaczek) take it into the hand whatever it is.
  - Werner leaves it face up "wenn es sich nicht zur Aufnahme in die Handkarte eignet" (if it is not fit to be taken into the hand), i.e. if it is a low suit card [03-S12][02 §Headline 3][RA].
- **Counting.**
  - Card values: kings and the three Trull cards (Sküs, XXI, I) 5; queens 4; cavalls 3; jacks 2; everything else 1.
  - Cards are counted in groups of three, minus 2 per group. That makes 70 in all [02 §4 item 2][07 §5.1].
  - The declarer needs 36. Period sources say this as "35 und 2 Blatt" (35 and two thirds), and the 1908–1920 press columns that seem to state three different thresholds all state this same one [RB][03 §4].
- **Scoring (Werner).**
  - A declarer who wins is paid 3 points; one who loses pays 4. A simple game pays 2 [03-S12].
  - A 35–35 simple game is a draw, and the next game counts double [03-S12].
  - The Trull (*tous les trois*: Sküs, XXI and I together in the hand) and all four kings each earn 1 point. They must be announced, at the latest when one of them is first played, and are paid even if the cards are later lost [03-S12].
  - Pagat ultimo (winning the last trick with trump I) earns 1 point; losing it costs the same. It can never be announced in advance, "da es ja möglich ist, daß der Gegner … als letzte Karte seines Talons noch einen Tarok aufdeckt" (since the opponent may yet turn up a tarock as the last card of his packets) [03-S12].
  - A Valat (all 27 tricks) earns 12 points and is "eine außerordentliche Seltenheit" (an extraordinary rarity); Bermann and Ulmann call it impossible [03 §2].

---

## 2. History: three games under one name

### 2.1 Before the name: the dead hand (1781–1846)
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

- **The sentence was reprinted for decades:**
  - Haas 1834 and 1842 [03-S5][03-S7].
  - Tendler 1830, 1845 and 1846, reworded and with Écarté added, and ending "so ist es natürlich keine Kunst, das Spiel zu gewinnen" (so it is naturally no art to win the game) [03-S6][03-S8][03-S9].
  - Alvensleben's *Encyklopädie der Spiele* (1853) pp. 576–577, with "Erfindung" (invention) softening "Mißgeburt" (abortion) [12 item 4][05d].
  - Wikipedia dates it to an 1839 edition. That edition exists [12 item 7]. Its p. 121 is unread, but the 1842 edition has the sentence on p. 121 [03-S7][RD].
- **For this project, the joke writes itself.** The same author series that made a Piquet game is now making the game the books said only Piquet-ignorant people would play.

### 2.3 The modern game (1855?–1938)
- **The name, without rules.** Vanderheid (1855) ranks "Strohmann-Tarok" among the three "besonders beliebte" (especially popular) Tarok games, and an 1866 advertisement mentions an appendix on it. The ÖNB copy has no such chapter, so which game he meant is unknown [03-S10][03-S11].
- **The modern game is firmly attested from 1888; a mid-1880s hit is weaker:**
  - "das Strohmann-Tarok (en deux)" in a list of Tarok kinds in *An der schönen blauen Donau*, 1886: an OCR snippet only, and "(en deux)" is also a label for Neunzehnerrufen with two Strohmänner (compare the 1888 review and RC on the 1887 Brno article), so the pile game is not certain here [03-S27][RC][REVIEW RF];
  - the *Klagenfurter Zeitung* review of Ulmann's *Tarokbuch*, 30 March 1888, which names "Strohmann-Tarok (en deux)" [03-S15];
  - Ulmann's *Illustrirtes Wiener Tarokbuch* (1st ed. by March 1888), whose preface claims the game had been "in der Tarokliteratur bisher gar nicht besprochen, ja nicht einmal erwähnt" (not discussed in the Tarock literature, indeed not even mentioned) [03-S14][03-S15];
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
| 1886 | "das Strohmann-Tarok (en deux)" in a list of Tarok kinds (OCR snippet only) | [03-S27][RC] |
| 1888 | Ulmann's Tarokbuch: the modern game in full | [03-S15] |
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
- Pagat ultimo cannot be announced in advance [03 §2 obs. 1].
- The Trull and four kings are paid separately (Werner and Ulmann say 1 point each; Bermann only "besonders berechnet", reckoned separately) [03 §2][RD].
- Dummett, comparing the descriptions in Ulmann (two books), Bermann, Werner, Unger, Beck, Löw, Babsch, Widder Lajos and the Piatnik leaflet, found they "agree very closely" [02-S1] [REVIEW RF: "eight" was not Dummett's number].

### 3.2 The canonical set we propose
Note 02 proposes a canonical set in which every default has a named source [02 §4]. Note 03 adds the period texts [03 §2].

Two coherent presets come out of them:

- **"Vienna 1891" (period), after Werner** [03-S12]:
  - all 24 packet cards dealt before bidding;
  - both-pass means a simple game for 2 points;
  - declarer wins 3 or loses 4;
  - a 35–35 draw doubles the next deal;
  - a low last packet card stays face up until played;
  - Trull and four kings 1 point each if announced from the hand;
  - Pagat ultimo ±1, never announced;
  - Valat 12;
  - no bird bonuses (Uhu, Kakadu, Quapil are 20th-century, §7.4).
- **"Vienna 2007" (modern), after Mayr & Sedlaczek** [02-S2]:
  - packets dealt with the hands (the page suggests dealing them after the bid only if one uses the throw-in rule for both-pass) [REVIEW RF: was "packets dealt after bidding"];
  - both players pass: a simple game is played (the throw-in rule is only the "old books'" alternative);
  - the last packet card always taken into the hand;
  - honours counted "quiet", in tricks;
  - Pagat, Uhu and Kakadu bonuses;
  - a two-deal "Runde" (round) and a "Nullrunde" doubling.

  The game values are the same as Werner's: 2 / 3 / 4 [02 §3].

We recommend "Vienna 1891" as the default. The rules then match the period of the table talk the opponent speaks, and "Vienna 2007" can be offered as a preset.

### 3.3 The options menu (each choice has a source)

| Question | Options and sources |
|---|---|
| When are the packets laid? | at the deal (Werner; Mayr & Sedlaczek's default) / after a bid (Ulmann; Beck as reported by Wikipedia; one of Bermann's two modes; Mayr & Sedlaczek advise it only with the throw-in rule) [03 §2][02 §1][02-S2] [REVIEW RF: Mayr & Sedlaczek moved] |
| Both pass | simple game for less (Werner, Mayr & Sedlaczek, Furr, Löw) / throw in (Ulmann's usual practice, Beck, Bamberger) [03 §2][02 A11] |
| Last packet card | taken (Beck, Babsch, Mayr & Sedlaczek) / stays up if low (Werner) [02 §Headline 3] |
| Values: simple / win / loss | 2/3/4 (Werner, Mayr & Sedlaczek, Furr) / 1/2/3 (Beck, Dummett) / 1/2/4 (Ulmann) [02 §3][03 §2] |
| Big wins | Furr: declarer ≥45 scores 4, defender ≥44 scores 5 [02-S3] |
| Ties | next deal doubles (Werner, Beck) / next round (Mayr & Sedlaczek, Dummett) / for Ulmann a bid hand cannot tie [02 A12][03-S14] |
| Honours | announced in hand (Werner, Dummett, Beck) / quiet in tricks (Mayr & Sedlaczek) / none (Furr) [02 §3] |
| Pagat ultimo | 1 (Werner), 1 or 2 "in some circles" (Ulmann), a lost one costs double (Bermann) [03 §2] |
| Bird bonuses | Uhu, Kakadu (Mayr & Sedlaczek, Furr); Quapil (Mayr & Sedlaczek) [02 §3] |
| Rostopschin | XVII then XVIII on consecutive tricks, called "Ross!" … "Topschin!" (Furr; Torberg via Wikipedia and Mayr & Sedlaczek's 2002 page) [02-S3][12 item 1] |
| Kontra | Löw lets the opponent double a bid; no other source does [02 A18] |
| Valat | 12 (Werner, Mayr & Sedlaczek, Furr, Dummett) / none, "weil ein solcher nie vorkommt" (because one never happens) (Ulmann) [03 §2] |
| Redeal with no tarock in hand | Dummett's Austro-Hungarian rule [02 §4 item 4] |
| Who leads | forehand (most sources) / the bidder (Löw, Furr) [02 A10] |

Rejected options:
- **Mondfang** (catching the XXI with the Sküs) appears in no primary Strohmandeln description and is left out [02 §Headline 4].
- **Rostopschin**: in Wikipedia's scoring table the trumps are "VII and VIII", which is an editing error [01 §Headline 5]. Its glossary has "XVII and XVI", an outlier [12 item 1].

### 3.4 Questions the sources leave open
Note 02 lists 18 of these [02 §2]. The ones that matter most:

- **Following suit.** The cards you must follow from are the hand plus the face-up packet cards. Nothing forbids playing a face-up card when the hand could follow [02 A2]. Bermann: "ist beides der Fall, so steht die Wahl … frei" (if both are possible, the choice is free) [03-S13].
- **When packet cards are replaced.** After the trick is gathered, not during it, so the follower never sees what the leader's flip reveals [02 A5].
- **Is a packet's last card shown when it is taken into the hand?** Sources disagree [02 A4].
- **Pagat ultimo and the packets.** The last trick is trick 27, whether the card comes from the hand or a packet. Werner warns that a buried tarock in the opponent's packets can stop a Pagat ultimo, and advises keeping packet cards back for exactly that purpose [02 A13][03-S12].
- **Information the interface must show** [02 §4 item 15][03-S14]:
  - the opponent's face-up packet cards;
  - each card pulled into a hand, shown before it is taken;
  - how many unseen cards each packet still holds.

  Ulmann's Codex §31 treats peeking under a packet card as an offence that "rigorose Strohmannspieler" (strict straw-man players) rank with a revoke.

---

## 4. The family: straw men, dead men and dummies

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

## 5. Social life: coffee-houses, courts, couples and kibitzers

- **A coffee-house game for small and large stakes.**
  - 1850: a man invited to play by a stranger in Adami's coffee-house wins "beim Strohmandeln 17–18 kr. C.M." (17–18 kreuzer, convention coinage) from him, then 1 fl. 20 kr. more that the stranger stays owing; the report is a court/crime item, but that it was a fraud case is unverified [03-S16][04 §2] [REVIEW RF: the synthesis had the winner reversed; note 04's correction of the subject of "gewann" was not carried into note 03].
  - 1893: "die aufgenommene Partie zu 4 fl, die einfache zu 2 fl" (a taken-up game at 4 florins, a simple one at 2) [03-S33].
  - 1894: a spendthrift's daily Strohmandl "um einen Gulden den Point" (at a gulden a point) [03 §4].
  - 1921: a Viennese coffee-house card-fee tariff [03-S39].
- **Second best to a game for three.** The 1910 ladder runs "Der Königrufer sinkt zum Tapper, dieser zum Strohmandl herab" (King-calling sinks to Tapper, Tapper to Strohmandl) [03 §4]. In 1914 it is "mehr ein stummes Spiel von Mann zu Mann" (rather a silent game, man to man) [03 §4]. In 1926 it is "ein schäbiges 'Strohmandl' zu zweit" (a shabby Strohmandl for two) [03 §4].
- **A couples' and household game, often in jokes** [04 §Headline 2][03-S29]:
  - an 1887 feuilleton husband loses his Pagat ultimo when "meine Frau deckt als letzte Karte ihres 'Strohmandls' den 'Tarok-Zweier' auf" (my wife turns up the Tarock deuce as the last card of her Strohmandl). This is the packet mechanic in a period text;
  - an 1887 joke: "Liebe ist wie das 'Strohmandel-Tarok' eine Angelegenheit unter Zweien; Dritte finden dabei nicht einmal als Kiebitze Verwendung" (love, like Strohmandel-Tarok, is a matter between two; third parties aren't even needed as kibitzers) [03 §4].
- **Courts and police.**
  - 1895: a libel case over a Strohmandl game at the Café de l'Europe, with a kibitzer accused of signalling [03-S34].
  - 1895: a court declines Tarock experts because "das Tarokspiel weder eine Wissenschaft noch eine Kunst sei" (Tarock is neither a science nor an art) [04 §Headline 3].
  - 1912: police commissioner Fried raided a gambling den by asking for "Strohmandelkarten" (Strohmandl cards) and playing a game with a colleague, "sich scheinbar um die Hasardspieler nicht bekümmernd" (seemingly paying the gamblers no heed), "wobei ihm der Polizeiagent Garhofer kiebitzte" (while police agent Garhofer kibitzed); "Die Hasardspieler wurden tatsächlich auf die harmlose Strohmandlpartie nicht aufmerksam" (the gamblers really did not notice the harmless Strohmandl game) [04-S33]; page image read by the coordinator, settling the 03/04 difference [03 §4][04 §2].
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

## 6. The game in the empire's other languages

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

## 7. Table talk and banter

The detail is in notes 05 (glossary and a 60-row table of lines), 05b (dictionaries and proverbs, about 70 sayings), 05c (literature and feuilletons, about 100 rows) and 05d (other languages, about 60 rows). Each row gives the original, a translation, the source and date, the game moment it fits, and a short PG line. This section picks out what shapes the design.

### 7.1 The calls
- **Bidding.**
  - "Ich nehme auf!" (I take it up), "Weiter!" (pass), "Ich passe!" (I pass) [02-S2][05 §2].
  - Older calls from Tapp-Tarock and Königrufen, not recorded for Strohmandeln: "Ich spiele" (I play), "Halte ich" (I hold), and "Ich tappe" (I tap), later replaced by "Weiter" (Dummett) [05d POLY-S32] [REVIEW RF: Dummett pp. 453-454 give these for the three-hand games; they were presented here as Strohmandeln bids].
- **Honours.** "Tous les trois!" and "Tous les quatre!" (Ulmann's names for the Trull and four kings) [03-S14]. "Ich skisiere mich!" (I play my Sküs and excuse myself) [05b LEX-S22].
- **Doubling.** The Viennese word for doubling a bid (Kontra) was *spritzen*: "Ich spriz den Dreier" (I double the Dreier) (Hügel via Wander, 1876). In 1898 a paper writes of "einem gespritzten 'Dreier' und einem angesagten 'Pagat'" (a doubled Dreier and an announced Pagat) [05b LEX-S15][05b LEX-S28].
- **Rostopschin.** "Ross!" … "Topschin!" (Torberg, as quoted on Wikipedia; Mayr & Sedlaczek's 2002 page gives XVII/XVIII and calls it "recht beliebt", fairly popular, between the wars, "vermutlich vor allem im Vierer-Tarock", probably mostly in four-hand Tarock; the same page adds "Auch im Strohmann-Tarock führt sie zu interessanten Konstellationen", in Strohmann-Tarock too it leads to interesting situations) [12 item 1][12-S5] [REVIEW RF: the last clause was omitted; it is on the saved Wayback page].

### 7.2 The kibitzer as the stock comic figure
- **"Kiebitz, halt's Maul!"** (Kibitzer, shut your trap!):
  - "einer der Hauptparagraphen" (one of the main paragraphs) of card-table law in 1895 [03-S34];
  - "§ 13" from 1897 [RD];
  - "Das 'Kiebitz'-Reglement hat auch nur einen Paragraphen" (the kibitzers' regulations also have only one paragraph) in 1904 [05c LIT-S44] [REVIEW RF: German wording corrected to the note's quotation];
  - on a large sign in an inn, Grazer Volksblatt 1904 [05c LIT-S40].
- Wander records the word as Austrian by 1880: "Es ist ein Kibitz. So nennt man in Oesterreich die Zuseher beim Spiele" (He's a kibitz: that's what they call onlookers at a game in Austria) [05b LEX-S12].
- A Baden club of 1908 had an "Ehrenkibitz" (honorary kibitzer) [05b LEX-S29].
- **Design proposal.** Make the tutor a period kibitzer at the player's shoulder, and use "§ 13" as the button that silences it [05c §Headline 1] [REVIEW RF: was §Headline 2, which is the Nestroy item].

### 7.3 Lines that fit game moments (a shortlist)

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
| The Piquet joke | "nur von Leuten …, denen das edle Piquetspiel gänzlich unbekannt ist" (only for people who don't know the noble game of Piquet) (1829) [03-S4] | "Have you not heard of Piquet?" |

Note 05c gives the Nestroy lines from the 1890–91 Bonz edition. A modern text differs, so every Nestroy line in the game must name its edition [RB][RD].

### 7.4 Period-correctness traps
- **The bird names come too late for a 19th-century voice.** Uhu, Kakadu and Quapil for trumps II, III and IIII date from the 20th century: Uhu first appears in Hungary in 1902, according to Wikipedia (second-hand) [05 §Headline 6][RB].
  - The opponent's 19th-century voice should say "der Zweier" (the deuce), not "der Uhu".
  - The card faces may still carry the nicknames as a learning aid (§9).
- **Pagat ultimo was not announced in Strohmandl.** Werner, Bermann and Ulmann all rule out announcing it [03 §2]. An 1887 feuilleton's "angesagte" (announced) game is about the bid, not the Pagat [RB].
- **"Mondfang" is not recorded for Strohmandeln in any period source** [02 §Headline 4]. It is attested in period German for the other Tarock games (Werner's Königrufen section; press 1887, 1893, 1925) [05 §1 Mondfang row][05c LIT-S20, S55, S56]; note 05d's "no period German Mondfang" reflects only an archive.org full-text search [05d] [REVIEW RF: contradiction between notes 05 and 05d resolved in favour of 05/05c, which quote the texts].

### 7.5 A Viennese opponent's vocabulary
- *Patzer* (bungler), *Gstieß* (Sküs) and *Spatz* (sparrow, for the Pagat) [05c §Headline 8].
- *Skartindeln* (worthless low cards) and *packeln* (to stack the trumps when gathering) [05c §Headline 8].
- "Er hängt scho' der Max'" (Hügel 1873: "his wish is close to fulfilment"; Max' is also, separately, a dialect name for the Sküs card, with a folk etymology). Whether the phrase was ever said of drawing the Sküs is the note's inference, not Hügel's [05 §Headline 8][05-S1] [REVIEW RF: was cited as [05 §3] and glossed as if the phrase meant the Sküs].
- "Schmied von Penzing" (the smith of Penzing: one made to deal again and again) [05 §3].
- "Zieh endlich deine Fäustlinge aus!" (take your mittens off at last!): known only from a newspaper review of Mayr & Sedlaczek's *Kulturgeschichte*, not from the book, and said of a slow or fumbling deal [09-S5][RA] [REVIEW RF: second-hand status added].

### 7.6 What stays out
Excluded under the kindness rule, though attested [03 §4][04][05 §Kindness][05c §K][RB][RD]:
- the Pagat-ultimo slang "der Jud" (the Jew), and the Czech *žid* for the kitty;
- a run of 1900s–1930s ethnic jokes that use Strohmandl as a prop;
- "von ein paar alten Jungfern erfundenes Kartenspiel" (a card game invented by a couple of old maids);
- Nestroy's wife joke;
- "Tschappel" (simpleton) and "Graßl" (a notorious robber);
- one double meaning [RA].

---

## 8. Strategy and numbers

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

## 9. The cards

**The decision (taken by the maintainer):** we draw our own cards, in the large-type high-visibility style of the cassino and piquet phone faces. We don't reproduce period art.

**What the real pack looks like, as a guide:**
- **Suits** run K Q C J, then 10 9 8 7 in black and 1 2 3 4 in red [08 §Headline].
  - Real suit cards have no corner indices at all; our rank labels are a deliberate departure, and the tutor can say so [08 §Headline].
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

## 10. Digital implementations and the computer opponent

- **No playable digital Strohmandeln was found.**
  - The nearest is `smatkovi/harbour-tarock` (GitHub, created September 2026, GPL-3.0-or-later). It has a long German specification of the game, a "hints for the computer opponent" chapter and a straw-man row component, but plans to make Strohmandeln playable only "after 1.0" [07 §3.1][12 item 8].
  - We cite it and copy nothing. Its 21 rule interpretations are listed in note 12 for comparison [12 item 8].
- **Officers' Skat**, with the same pile mechanism, exists in KDE LSkat and on iOS [07 §3.2].
- **AI.**
  - The relevant literature is Gams & Luštrek's Tarok program (alpha-beta search over sampled deals) and Long et al. (2010) on why sampling a complete deal and solving it works [07-S6][07-S7].
  - Strohmandeln is a textbook case for that method, with one twist: a player's own unturned packet cards are unknown to him too, so the opponent must sample its own hidden cards as well as the other player's [07 §Headline 5].

---

## 11. Reusing the earlier games

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

## 12. What review changed, and how far to trust each note

| Note | Verdict | Main corrections |
|---|---|---|
| 01 Wikipedia | usable with care, close to trustworthy | "zu 5 Karten" means dealing in fives; a Ulmann quote concerned Neunzehnerrufen [RA] |
| 02 Modern rules | usable with care | Werner's last-card condition restored; Torberg relabelled second-hand [RA][RC] |
| 03 Period rulebooks | **trustworthy**, with one uncorrected slip found later (the 1850 report's winner is reversed; see §5 and [RF]) | modern game dated c. 1886–88, not 1883; "1839 = 1842" marked as inference [RD] |
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

---

## 13. Open questions and decisions for the maintainer

**Decisions:**
1. **Which game?** The modern game is the obvious choice. The 1829 "dummy pile" game could be a short historical curiosity mode: it is simple to add, and the period called it dull, which is itself a joke.
2. **The default rules.** We recommend the "Vienna 1891" preset (Werner) as the default, so the rules match the period voice. "Vienna 2007" (Mayr & Sedlaczek) would be a preset, and every other divergence in §3.3 an option.
3. **Match length.** No source gives one [02 §Headline 7]. A design choice is needed: for example, first to a fixed number of points, or a set number of two-deal rounds.
4. **How to share code** with the earlier games: a vendored kit (the review's advice) or a submodule (§11).
5. **Jack labels.** The maintainer left "J" or "V" (Valet) open. For an English-speaking audience "J" reads at once. The period German name was *Bube* [08].

**Research still open:**
- Torberg's *Tante Jolesch* p. 217 (lending-restricted) [12 item 1].
- The Freud–Ferenczi letter, which has no primary text found [12 item 2].
- Haas 1839 p. 121 [12 item 7].
- Ulmann's *Buch der Familienspiele* [12 item 3].
- Unger's manuals of 1918–26 [03 §8].
- The text of the Czech 1932 Štromandl section [RE].
- The Hungarian Jánoska Antal tarokk guide [RE].
- The Vanderheid 1855 appendix [03-S11].

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
| `research/review/RA.md` … `RF.md` (and RG, a critique that edited nothing) | review logs |
| `research/_FOLLOWUPS.md` | the coordinator's running list |
| `research/sim/` | the original-computation scripts |
| `tools/anno.py` | the ANNO harvester |
