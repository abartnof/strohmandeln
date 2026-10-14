# RG: fresh-eyes critique of strohmandeln-lit-review.md

Reviewer RG. Read the synthesis in full; skimmed notes 02, 07, 09, _FOLLOWUPS; did two web searches (below). Nothing edited.

## (a) Game designer's seat

### Must be decided before the first line of engine code
1. **Card-point remainder arithmetic is not nailed down, and the "70 in all" claim may be off.** Note 02 (rows "Counting", A17) says: groups of three, minus 2 per group, a leftover pair counts 1, a single leftover counts 0. Each player's won-card count is even (2 per trick) but can be 1 mod 3. Total raw card value is 106. If both remainders are 0 the total is 70. If the remainders are (1,2), you get 106 - 34 - 1 = 71. So "70 points, 36 wins" holds only sometimes. The synthesis states it flatly (exec summary, §1) and explains "35 und 2 Blatt" only by pointer. The engine needs one exact function from "cards won" to "score", plus an exhaustive test over all tie and threshold cases (35+1 Blatt loses; 35+2 wins; what is 35-35 with unequal remainders?). I would simulate all remainder cases before trusting 70/36.
2. **Bidding, in exact terms.** Is there any overbid or counter after "Ich nehme auf"? What if forehand takes up and the dealer also wants to? (The synthesis implies no.) What do packet cards do in a both-pass simple game: does the opponent defend with a 35-35 draw rule, and who is "declarer" for the draw doubling?
3. **Reveal order and timing at the opening.** Simultaneous or alternating packet flipping? Does the dealer's flipping see forehand's results first? This changes information and the AI. The synthesis says "in turn" with no order for the two players.
4. **Every "sources disagree" row in §3.3 is a coding branch.** Pick the default and put the rest behind flags, or the test matrix explodes. There are about 14 options, so about 2^14 combinations. Ship at most 3 to 4 toggles in v1 (last-card rule, both-pass, packets dealt before or after bidding, honours announced or quiet). The rest should be presets, not independent switches.
5. **Honours announcement UI.** "Announce at the latest when one is first played, paid even if lost" needs a defined moment, a button and a deadline, a rule for the AI, and a tutor nudge when you forget. Nothing in §1 or §3 specifies this.
6. **What happens when the hand is empty but packets remain?** The synthesis says "never run out of hand cards while packets remain" as strategy, but never defines legality of leading from a face-up packet when the hand is empty or when the hand could follow. A2 says the choice is free; A4 (is the last card shown?) is unresolved.
7. **Match structure.** Unsourced (§13 item 3). Pick it now, because AI tuning, stakes, UI and the tutor's "game over" copy depend on it.
8. **Redeal rules.** Dummett's "no tarock in hand" redeal is a 1 in 15,298 event (§8). It is trivial, so default it off; do not spend UI on it.

### Tutor and UI under-specification
- The synthesis lists the information the interface must show (§3.4), but never says how the player sees the opponent's three packets' face-up cards versus the pulled cards (animation order matters, since pull-then-flip is the core "show"). Needs a storyboard.
- Peeking: Ulmann's Codex §31 treats peeking as an offence. A fun mechanic (a "peek" the player can try, with the kibitzer scolding) is not even considered.
- No tutorial curriculum. Nothing says what to teach first. My suggestion: lesson 1 plays a pre-set hand with packets pre-turned; lesson 2 adds bidding; lesson 3 adds Trull/kings; Pagat ultimo last.
- No mention of how the kibitzer tutor avoids spoiling the game (a hint system that reveals the opponent's pulled cards is zero-cost, one that reveals the declarer's choice is not). "§ 13" as a mute button is lovely but is the only design given.

### Banter system
- The inventory is excellent, but no trigger model: which of about 20 moments fire, with what cooldown, and what happens when the line is a quote with a citation requirement (§7.3 note: Nestroy edition must be named)? Is the credit shown in-game? The kindness rule is good but the "Hungarian Jókai in English" example shows lines will be translated; who owns the translations?
- Voice period: the opponent is 1891 but the game ships "Vienna 2007" as a preset. Banter bound to the 1891 preset (no Uhu, no bird bonuses) will say wrong things in the 2007 preset (Uhu bonus is announced). Needs per-preset line pools, or banter that is silent about bonuses.
- Dialect: lines mix Hochdeutsch, Wienerisch and Czech/Hungarian. Decide the language of the game. The synthesis never says whether the UI is English, German or both, yet half the charm is in the original. I would run English text with the German original on hover.

### Is "Vienna 1891 vs Vienna 2007" sound?
Mostly, but with four problems.
1. **It is really three differences, not two presets.** Per §3.2 the real divergences are (i) packets dealt before bid vs after; (ii) both-pass simple vs throw-in; (iii) last card; (iv) honours announced vs quiet; (v) birds and Runde. Only (v) is purely 20th century. The rest are inter-author, not inter-era: Ulmann (1888/1899) is contemporary with Werner (1891) and disagrees on (i) and (ii). So "1891" is "Werner", not "Vienna 1891". Call them "Werner" and "Mayr & Sedlaczek", or admit Ulmann as a third preset.
2. **Werner's last-card rule is conditional** and leaves a low card face up. The synthesis notes this (RC), but it makes "1891" the odd one out against all modern sources, and the 2007 preset a reasonable default for learners. A tutor teaching the rarer variant first is a decision deserving more weight than "matches the banter period".
3. **Pagat ultimo "never announced"** is a Werner/Bermann/Ulmann consensus, but the 2007 preset has Pagat bonuses; whether those are announced is not said. Specify.
4. **Neither preset has been playtested for fun.** The only sim result (§8) says the strict bid rule is met in 3 to 7% of hands, so the AI will pass nearly always under M&S threshold advice. The "simple game for 2 points" fallback therefore becomes the game most of the time, and in Werner's preset both-pass is a simple game, so most hands play without a declarer. This is a large design finding that the synthesis buries in §8: the default game is the simple game, and the engine should present it as such, with the tutor explaining it first.

## (b) Historian's seat

### Overclaims
- **"No playable digital version exists"** (exec summary, §10) is a negative from a search on a few engines. Pagat's phone apps, Tarock apps by Austrian publishers (e.g. "Tarock-Königrufen" apps), Kosakeln-style generic tarot engines and Piatnik's own digital products are not searched. Say "none found".
- **"No relative found has the automatic pull"** is framed correctly. The unqualified "distinctive mechanic" headline is stronger than the evidence (the search of Baronetti, Morto, officers' skat and the like is not exhaustive; §4 itself says so).
- **"Firmly attested from 1886-88"**: the 1886 "(en deux)" cite may be Neunzehnerrufen (RC says the Brno cite might be; the timeline still lists 1886 as the first attestation). The real anchor is Ulmann 1888. Downgrade 1886 in the timeline.
- **The "three games under one name" frame** tidy-fies a continuum. The 1800 "Taroc en deux" never used the name Strohmandeln (§2.1); making it a "game that carried the name" is a stretch. Say two games, plus a two-hand ancestor.
- **"Dummett: jejune game was ousted"** is the only evidence that game 2 gave way to game 3. A change in 1830-85 is inferred; there is a 30-year gap (1855-86) with no rules. "Ousted" is Dummett's inference from what he saw, not a documented event.
- **The "Piquet sneer / series joke"** is great, but the sneer is aimed at the dummy-pile game of 1829, not the modern game. Do not attach it to the modern game in dialogue without noting that.
- **"Coffee-house game for two" (1850-1938)** rests on newspaper snippets of mixed sense (the synthesis admits most "Strohmandl" hits are not the game). Please give the hit rate: how many of 305 issues are the card game?
- **"Kiebitz halt's Maul" as § 13 "card-table law"**: it is a joke formula; only two or three attestations. Fine as flavour, not as institution.

### Underclaims / loose threads not pulled
1. **Kosakeln**, the "only other two-handed Tarock game" per Pagat (search result above), appears in notes 01/02/06 but is not in the synthesis. Likely a sibling rival and an easy "see also" or second mode.
2. **Thomas Burgstaller, *Tarock zu zweit* (2017)** has an appendix on classic two-hand variants (found via search; note 02 and 04 mention the name). Check whether the notes read the appendix; the synthesis does not cite it. A modern book solely about two-player Tarock is the most obvious gap in "modern rules".
3. **Schnitzler's diary** (digital edition, schnitzler-tagebuch.acdh.oeaw.ac.at, 1879-1931, full-text searchable) is the best untapped dated record of who actually played what with whom. Note 05 only checked *Gustl* page 1. My search did not find a hit; I did not run the site's own search, so this is a lead, not a negative.
4. **Karl Kraus, *Die Fackel*** (AAC-Fackel full text) and Kraus-era feuilletonists (Polgar, Friedell, Altenberg, Kisch) were searched on the open web only (05, 05c). Do an in-corpus search.
5. **Wienbibliothek digital** (digital.wienbibliothek.at) holds the Werner-type PDFs that the web search turned up (two PDFs: 3752218, 3724659). The notes mention "the Wienbibliothek PDF" once. Make sure both are read in full; one may be a different rule book than Werner.
6. **Unger (1918-26) manuals, Löw 1956, Beck 1972, Bamberger 2011** are all unread; the modern "consensus" is second-hand through Dummett. The 1920s to 1950s are almost blank in rules, yet that is when "Vienna 2007" practice (quiet honours, birds) must have formed. The synthesis's two-preset story skips the transition.
7. **Wiener Zeitung 1912-1930s "Tarock-Ecke" or chess-style card columns**: ANNO full-text search by author columns (not just the word "Strohmandl") for "Tarockrunde" or "Strohmann-Tarok" hyphenated forms. The seven spellings may miss "Strohmann-Tarock", "Strohmandl-Tarok", "Strohmanntarock" with the k/ck.
8. **Piatnik's own archive and the Wiener Spielkartenmuseum / Museum der Spielkarten (Vienna, Schlosspark?)**: contact them. Did Piatnik print a rulebook leaflet in the pack (little rule sheets often accompanied 1900s packs)? That would be a primary rules text of the sort the notes lack for 1900-1950.
9. **Military and café sources**: Austro-Hungarian officers' mess tarock; Josef Roth, Doderer (*Strudlhofstiege*, a Café Tarock chapter), Musil; Stefan Zweig. Not searched.
10. **Women and class.** §5 has one "Tarock-Amazonen" line (in 09, not the synthesis). A coffee-house game for two with wife-and-husband jokes is a gender-history angle left lying.
11. **Freud**: the synthesis hedges well. But the Freud-Ferenczi lead is a dead end anyway, as it is four-hand. Do not use it in a two-hand tutor without saying so.
12. **Currency**: stakes from 1850 (kreuzer C.M.), 1893 (florins) and 1921 (a tariff) give a ready way to show stakes in period money. Cross-check the 1921 tariff for inflation; a "stakes in kreuzer, then Kronen, then Schilling" game theme is cheap and charming.

### Source-quality worries
- Roughly half the banter rows come through a review (09-S5) or Wikipedia (Torberg, Freud). The synthesis flags them, but §7.3's table does not repeat the flag next to each game line. Put a tier (A: seen in primary, B: second-hand, C: legend) on every line used in game.
- The "Mond is a false Germanisation of mondo" claim (§9) rests on one source (08-S17). Fine, but an etymology claim in a tutor needs two.

## (c) A player who has never heard of the game

### What §1 fails at
- It is a spec for someone who already knows Tarock. "Sküs", "Cavall", "Trull", "forehand", "Valat", "Pagat ultimo", "finesse", "follow suit", "take up" are all used before they are explained.
- It does not say what a *trick* is, or that trumps beat suits, or that there is a fixed suit-less trump ladder. A reader from bridge knows; most don't.
- It never says why anyone would want the straw men. The key idea (some of your future cards are on the table where your opponent can see them, others are hidden even from you) is not stated plainly anywhere.
- "Declarer wins 3, loses 4, simple game pays 2" arrives with no explanation of what a simple game is, since "both pass" is two lines earlier and the term "simple" is not defined.
- 27 tricks for a game that "has 15 cards in each hand" confuses (15+12=27 is never stated).
- The counting rule (threes, minus 2) is given without the reason, so "70" looks arbitrary.
- It never says what a win *feels* like. Three sentences of "why it's fun" would help the tutor as well.

### Replacement "how it plays" paragraph (about 150 words, only facts already sourced)

Strohmandeln is a duel for two, played with the 54-card Tarock pack, in which 22 cards form a permanent trump suit that beats everything else. Each of you holds 15 cards and also has three face-down packets of four cards in front of you, your "straw men". Before play you turn the top card of each packet. If it is a trump or a king, your opponent sees it and you take it into your hand, then turn the next; any other card stays face up and can be played like a hand card. One of you says "Ich nehme auf" ("I take it up") and plays as declarer, the other defends. You take turns leading tricks, must follow suit if you can, and otherwise must play a trump. A packet card you play uncovers the one beneath. Cards you win carry points, kings and the top three trumps most of all. The declarer needs 36 of the 70. Extra points reward holding the Trull (the three top trumps), all four kings, and taking the last trick with the lowest trump, the Pagat.

(Word count about 160. Trim "Extra points..." if you need exactly 150. Note it drops the "bid, then turn" ordering because sources disagree on it; if the preset is Werner, the packets are turned before the bid, so the sentence order should follow the preset.)

## Top 10 fun or surprising findings, with risk of being wrong

| # | Finding | Where it surfaces | Risk |
|---|---|---|---|
| 1 | 1829: "only for people to whom the noble game of Piquet is unknown", reprinted for 25 years, while the game described is a different one | Series in-joke, tutor intro, loading screen | Low on the quote; MEDIUM on attaching it to the modern game (see above) |
| 2 | Trump II shows a crowned eagle labelled "Industrie und Glück" yet is called the Owl (Uhu) | Card tooltip, tutor | Low: read off the museum scan; the "joke" is the note's inference |
| 3 | "Kiebitz, halt's Maul!" as "§ 13" of card-table law, used as the tutor's mute button | UI Easter egg | Low for the phrase; MEDIUM for "§ 13" (one 1897 source) |
| 4 | 1893 "First European Tarock Congress" at Tepl: 22 Valats and 54 Pagat ultimos in one hour, fanfare for each, a "Kuckuck" fine box | Loading screen, achievement | Low (primary, correctly flagged as a joke) |
| 5 | 1937 court defence: "Strohmandl is no hazard but an art, you have to combine, though the hand matters too" | Win screen, tutor | Low |
| 6 | 1887 husband loses his Pagat ultimo when his wife turns up the tarock deuce as the last card of her packet | Tutor anecdote for why Pagat ultimo cannot be announced | Low-medium: story is a feuilleton |
| 7 | Quapil card name = Czech surname Kvapil, "he has hurried" | Card nickname gag | HIGH: card link is an inference, flagged UNVERIFIED |
| 8 | "Mond" is a false Germanisation of Italian *mondo* | Tutor trivia | Medium: single source |
| 9 | Freud's Saturday Tarock and the "Unbehagen / Behagen in der Unkultur" quip | Loading-screen line | HIGH: second-hand, hedged "soll geantwortet haben"; four-hand game |
| 10 | "Ihr Blatt, Herr Rosenblatt!" (*Der Bockerer*); "Wenn meine Großmutter vier Radln hätt'..." (a lost-hand post-mortem) | New-hand and post-mortem banter | Medium: the Bockerer line is a play from 1948 (after the 1938 end of the press record), the "grandmother" line is only from a book review |

Honourable mention: the Jókai "Skiz, vagy semmi!" and Mikszáth "Kontra az ultimónak, kontra a játéknak, kontra mindennek!" (Hungarian banter, fun, but Hungarian Tarokk is a different game, so the line sits in a different rules world; flag when used).

### Most at risk of being wrong
- Freud (second-hand, four-hand), Quapil/Kvapil (inference), Rostopschin trumps (three versions: XVII/XVIII, XVII/XVI, "VII and VIII"; Torberg unread), the "1886 (en deux)" attestation, "§ 13", the "Uhu 1902/1937" dating (Wikipedia), "Mondfang absent", Nestroy lines (edition-dependent), and anything from the Menasse review of Mayr & Sedlaczek.

## Web checks done
- Search for Schnitzler's diary plus Strohmandl: no hit found, but the diary edition exists at schnitzler-tagebuch.acdh.oeaw.ac.at and was not queried directly; the site's own search was not tried. https://schnitzler-tagebuch.acdh.oeaw.ac.at/
- Search for the game's rules turned up Pagat's mention of Kosakeln as the only other two-hand Tarock game, Burgstaller's *Tarock zu zweit* (2017), and two Wienbibliothek PDFs (digital.wienbibliothek.at/download/pdf/3752218.pdf and /3724659.pdf). Worth confirming the notes read both.
- No bot checks were bypassed.
