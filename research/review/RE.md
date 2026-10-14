# Review RE: note 11 (11-habsburg-languages.md)

Reviewer RE. Brief: research/_REVIEW_BRIEF.md. Changes in the note are marked `[REVIEW RE ...]`. Scratch scripts are in the session scratchpad, not in sources/.
Checked against: saved Czech OCR in sources/11/cz, the Parlaghy PDF (page images and pdftotext -layout, printed page = PDF page - 2), Kramerius v5 API (metadata, search, highlights), ANNO JSON search (tools/anno.py endpoints), sources/01, 06, 08, 12 and notes 05d and 06.

## Change / problem log

| Location | Problem | Evidence | Action |
|---|---|---|---|
| Headline 1, 1.1 | 1912 form is "stromandle" (OCR), not "štromandla" | cz/ln1912.txt line 379; Kramerius index "štromandle" 1 hit | annotated |
| Headline 1, 1.1 | 1930 Lidové noviny "štromandla" has no verified card context; may be a surname | Kramerius: that page has none of taroky/karty/hráli/hrát/partie/hra/mariáš/sázku; a Czech surname Štromandl exists (Naše pravda 27 Oct 1953 "soudruhem Štromandlem", a tractor driver) | annotated as unverified |
| 1917 Právo lidu | card context plausible | same page has "karty", "hráli", "sázku" (search check); text still restricted; "korun" is an expansion of snippet "koř" | OK, kept [snip] |
| Headline 2, 1.1 (1891/92 anecdote) | Quotes and translations correct. But the text never says cards/Tarock next to štromandl; "playing cards with the priest" and "classic parish-parlour game" are inferences | cz/ceskaskola1891.txt lines 40-42 (card-game is inferred from "Hrajete v karty? Co hráváte nejraději?" in the same speech); cz/vychovatel1892.txt | softened in note |
| 1.1 Společenský rádce 1932 | "Contents list (p. 369)" wrong: contents page 495 lists "Štromandl ... 370", "Taroky ve třech ... 371" (369 is the preceding entry). Same error in note 06 (not edited) | Kramerius highlight on uuid:4e66db7f (title "495", TableOfContents); page uuid:389ef0b2 is p. 370 | fixed |
| 1.1 | "first source that gives Czech rules" but only a heading was seen | json highlight shows only "Štromandl. / Taroky ve dvou"; heading follows rule 15 of another game | fixed wording |
| 1.1 | Book metadata supplied (Andreas, Gottfried; Čajčík; Moravská Ostrava-Přívoz: I. Buchsbaum, 1932) | BIBLIO_MODS of uuid:29b474c0... | added |
| 1.2 1912 Lidové noviny | "top of the ladder... most thought-demanding" is inference ("od ... až po" only); also the 1880 Pester Lloyd calls Strohmandl "den Geist nicht besonders anstrengend" | cz/ln1912.txt; ANNO_pel18801016 p. 5 | annotated |
| 1.2 Tepl 1893 | quote and translation faithful (OCR noise only). Date 2.2.1893 confirmed. The report is a reprint: Prager Tagblatt 2 Feb 1893 p. 6 says "dem 'Frmdbl.' geschrieben" (Wiener Fremden-Blatt). It reads partly jocular | ANNO_ptb18930202 snippet; Kramerius supplement "Brünner Morgenpost" 02.02.1893 | annotated; "unified rules" unverified |
| 1.2 Bohemia 1906 | quote verified (OCR "Gesellig-/leitsvereln"); a "Geselligkeitsverein" named Strohmandl in a comic sketch | cz/bohemia1906.txt lines 153-155 | OK |
| 1.2 Prager Tagblatt 1920 | gloss "[= 2 more points?]" probably wrong; "2 Blatt" = 2 cards. Mayr & Sedlaczek: "mindestens 35 Punkte und 2 Blatt" in Dreierlagen | sources/01/cites/wz.txt; ANNO_ptb19200924 p. 8; the 1908 Kärntner Zeitung phrase sits in a meeting/vote metaphor | annotated |
| 1.3 Dennice 1911 | "confession of love" is a man's tale of declaring love to his mother-in-law; catch-phrase status unproved (one occurrence) | cz/dennice1911.txt lines 395-433 | annotated |
| 1.3 Rozkvět 1926 | line expanded from snippet: verified "Hráli jsme celé odpoledne v taroky — — ve dvou!" | Kramerius highlight | OK |
| Headline 3, 2.1 | Claim "Dummett-only" for Strohmandli is wrong in scope: Mayr & Sedlaczek 2007 ("Dort heißt es Strohmandli"), Stuf7023 and a modern Hungarian page scordatura.hu ("A 'strohmandli' egy különleges tarokk ... két játékos ... A pakli 42 lapos", citing "a Jánoska Antal-féle tarokkismertető") use it. None is period | sources/01/cites/wz.txt; sources/08/pdf/Stuf7023.txt; sources/12/hb/strohmandeln.md §9.12; I fetched scordatura.hu/stuszivadasz/paros.html | annotated; lead logged below |
| Headline 3 | "only two are played" overstates "rendesen csak kétfélét játszanak" (ordinarily) | parlaghy PDF p. 126 | fixed |
| 2.1 | "Dummett's phrase ... only as a later or border usage": the "only later/border" is invented | Dummett p.4: "also called Strohmandeln ... or, in Hungary, Strohmandli. It does not seem ever to have been known in Germany, but only in Austria and Hungary" | fixed |
| Headline 4 / 2.2 | Whist "strohmannal" page given as "p. 99-ish", "~99": it is printed p. 124 (PDF 126). Text and translation fine ("Négynek kell játszani, vagy helyettesül egy vagy legfölebb két strohmannal") | page image/layout text | fixed |
| 2.1 | Tarok section "pp. ~126-138": actually printed pp. 126-141 (four-hand 127-139, three-hand from 139) | PDF pp. 128-143 | fixed |
| 2.1 | Dating: title page has no year; 1888 is the catalogue date; printer footers read "62-1889. Kártyakönyv" | page images u-001, t-003 | annotated (1888/1889 uncertain) |
| 2.2 "Tarok fekszik" | gloss "A trump lies in the talon" wrong: the discard (skart) laid against the bidder; duty on the three others; said to the bidder; p. 128 (not ~129) | PDF p. 130 | fixed |
| 2.2 "Segít a XX-as" | quote and p. 128 verified. "invit for a pair game" is a misreading (kettes/egyes/solo = number of talon cards taken). "survives as the dummy's help in two-hand play" is speculation | PDF pp. 130, 131, 133 | fixed/annotated |
| 2.2 contra ladder | ladder verified (contra, recontra, rerecontra, supercontra, then Hirsch and Pápa "very rare"), printed p. 131; glossary entries Hirsch/Super contra/Pápa on glossary pp. III-VI, not "p. 131/~210" | PDF pp. 133, 203-206 | fixed |
| 2.2 "sír és nevet" | verified, printed p. 138 (not ~137); dealer is against the bidder, "Biká-val" = "with the bull" | PDF p. 140 | fixed |
| 2.2 "ugratás" | translated "jumping": wrong, "ugrat" = goad/egg on; a four-hand variant, printed pp. 138-139 | PDF pp. 140-141 | fixed |
| 2.2 tariff | "three-hand scale ... uniform" mistranslation: text is "a négyes tarok díjszabása" (four-hand); "hármas játék" = the bid "three" = 1 point. Values (ultimo 10/5, volat 6x/3x, XXI-fogás 21/42, 8/9 trumps 1/2) verified | PDF pp. 139-140 | fixed |
| 2.1 Paskievics | "Russian commander in the 1849 war" not in the cited sources (Pester Lloyd 27 Sep 1875 says only "dem genannten großen Feldherrn") | ANNO snippets | annotated |
| 2.1 Agramer 1895 | bracketed completion "[Tapp-Tarock] and Königrufen are played [with three/four]" is the note's invention; p. 5 confirmed. Claim that Klagenfurter 1888 / Brünn 1887 are "the same article" unverified (different wording; 06 also warns that the Brünn text may mean Neunzehnerrufen with two Strohmänner) | ANNO snippets; note 06 review remark | fixed |
| 2.3 Gaal | 8 Oct 1838 verified in Pallas (iconv 8859-2). "Second Pallas entry" is the same entry plus the Peleskei entry, which says only 1838. Caveat: 05d used the 1881 text | pc003932.html, pc008080.html | clarified |
| 3 Polish | "taroka, wista, preferansa" is a splice of two truncated snippets | anno_foreign.jsonl Kuryer Lwowski 19 Dec 1902 | annotated |
| Headline 7 vs 05d | contradicts 05d item 8 (1821 Breslau Polish almanac, "Tarok en deux") if read as "no Polish two-hand description" | 05d line 34 | annotated |
| 1.2 1824 book | quote verified; 06 attributes it to "G. Haas", note 11 doesn't. Not checked further | Kramerius page 109 title page uuid:531a5ef0 | unresolved, minor |

## Quote statistics

Source-language quotations from the note (about 50 distinct fragments, tested after normalising case, accents, hyphenation and punctuation against sources/11, sources/anno and the Parlaghy OCR): 28 matched automatically; the remaining 22 were examined by eye and all are OCR noise or splices explained above (Löwen/Tepl/Pester Lloyd/Agramer/Jellinek/Polish snippets), except: (a) "taroka, wista, preferansa", a splice; (b) the Hungarian gloss lines "Kontra az ultimónak..." (Mikszáth) and "Mich reizet..." (1756), which come from 05d and were not checked here; (c) the Gaal quote, which matches in the ISO-8859-2 file after iconv. No quote was found to be invented. The English translations of Czech, German, Polish and Hungarian were all faithful except: "ugratás" ("jumping"), "tarok fekszik ... in the talon", the "three-hand scale", "pair game", and the Agramer bracket completion (all fixed).

Translation confidence: Czech anecdote, 1912, 1926, 1911 lines: high. Hungarian Parlaghy lines: high for "tarok fekszik", "segít a XX-as", "sír és nevet", "Biká-val", whist "strohmannal"; medium for "népes tarok" (glossed "populous"; probably "many-player") and "rendesen" (usually). German Tepl, 1824, Pester Lloyd: high.

## Negative claims (Slovene, Polish, Croatian, Italian, Hungarian, Czech)

The queries were reasonable. I re-ran them and more through ANNO: "taroka/taroki/tarok we dwóch", "tarok dla dwóch", "tarok za dva/dvoje", "tarokirati", "tarokovati", "tarokk kettesben", "tarokk ketten játszák", "szalmabáb tarokk", "tarocco/tarocchi a due/in due", "tarocco in due persone", "tarocchi due giocatori", "štromandl", "strohmandla", "taroky ve dvou", "Strohmandli" (ANNO also indexes Czech titles such as Dělnické listy): all 0 phrase hits. Kramerius NKP full-text counts: štromandla 4 (1891, 1892, 1930, 1917), štromandl 2 (both 1932 book), štromandle 1 (1912), štromandlem 1 (1953, a surname), strohmandla 3 (1966, 2010, 2014), štramandl 0, štromandlu 0. The note's list is therefore complete for the Kramerius index. Caveat: the index is accent-folded and phrase-less, so these are weak negatives; and the Slovene dLib, Polona, dnc.nsk.hr, Arcanum were not searched (the note says so).

## New findings (not put into the note)

1. Pester Lloyd 16 Oct 1880 p. 5 (ANNO_pel18801016), repeated in Mährisches Tagblatt 20 Oct 1880 p. 2: "jenes urwüchsige und den Geist nicht besonders anstrengende Kartenspiel, welches 'Strohmandl' heißt"; the game is called "hausbacken", and a lawyer's knowledge of it is the joke. A Budapest German paper using the name in 1880: supports the hunch that Hungarian "Strohmandli" came via German-language Pest usage. Context not read beyond the snippet.
2. scordatura.hu/stuszivadasz/paros.html (modern, Hungarian): 42-card Strohmandli, 12 cards in hand, 3x3 face-down piles, "eredeti Strohmandeln szabályai a pagat.com oldalain" (the pagat.com page does not exist, see 12/hb); cites "Jánoska Antal-féle tarokkismertető" as the Hungarian source of the rules. Lead: find Jánoska Antal's tarokk book (date unknown to me).
3. Mayr & Sedlaczek: "35 Punkte und 2 Blatt" in Dreierlagen is the modern threshold; period 1920 says "36 ... zwei leere Blätter", 1908 "36 und 2 Blatt". Worth checking in the rules strand.
4. Newspaper items from Vienna, 1893 and 1900 (Neuigkeits-Welt-Blatt 4 Jun 1893 p. 5: Strohmandl at cafés near the Börse, stake "aufgenommene Partie 4 fl, einfache 2 fl"; Wiener Allgemeine Zeitung 22 Feb 1900 p. 2: "Kennen Sie Strohmandl spielen?") look useful for the rules and banter strands; I only saw snippets.

## Verdicts

Note 11: usable with care. The sources exist and the quotations are real; the Czech findings are sound except that two of the five "štromandla" attestations are weaker than the headline suggests (1930 unverified as the game, 1912 is a different form). The Hungarian section had many wrong page numbers (fixed), three mistranslations/mislabelings (fixed) and the "Dummett-only" claim, which is false for modern sources. Negative claims are plausible but weak. Worst problems: (1) 1930 and 1912 attestations overstated; (2) "Strohmandli remains Dummett-only" (scordatura.hu, Mayr & Sedlaczek); (3) Parlaghy pages and glosses; (4) "contents p. 369" for the 1932 manual (also in note 06).
