# RF: review of note 12 and of the synthesis (strohmandeln-lit-review.md)

Reviewer RF. Targets: (1) `research/12-followups.md` against `research/sources/12/`; (2) `strohmandeln-lit-review.md`, traced claim by claim to the cited notes and, where possible, to the saved sources. Edits in the synthesis are tagged `[REVIEW RF: ...]`; edits in note 12 likewise. No git commits. A pre-edit copy of the synthesis is in the scratchpad only.

Method. Read note 12 in full and re-checked its nine items against the saved sources, looking at the images myself (museum contact sheets `sheet_a.jpg`, `sheet_b.jpg`; Alvensleben `p601.jpg`, `p602.jpg`; Neuigkeits-Welt-Blatt crops `full.jpg`, `c1.jpg`, `c2.jpg`, `d1.jpg`; Neue Illustrirte Zeitung crop `niz.jpg`; Klagenfurter Zeitung crops of 30 Mar 1888). For the synthesis I read every line, then traced all claims in the Executive summary, §1, §2, §3, §7 and §8 (about 120 claims) and the rest of §4-§6, §9-§13 at well over 50%, against notes 01-12 and logs RA-RE. 28 quotations from the synthesis were run through a normalising script against all saved source text (see statistics).

## A. Note 12: verdicts per item

| Item | Verdict | Evidence |
|---|---|---|
| 1 Torberg / Rostopschin | confirmed (second-hand, correctly labelled) | The Torberg quotation is at `sources/02b/wp_koenigrufen_de.txt` line 279 (XVII, XVIII, "Ross!", "Topschin!", "sonst galt's nicht") and the glossary line 746 says "XVII und XVI". The M&S 2002 Wayback text (`sources/12/wayback/strohmann.txt`) says XVII/XVIII, "in der Zwischenkriegszeit ... recht beliebt ... vermutlich vor allem im Vierer-Tarock", "Torbergs Die Tante Jolesch ... ausführlich beschrieben". The same page also says "Auch im Strohmann-Tarock führt sie zu interessanten Konstellationen" (not mentioned in note 12 or the synthesis; added to the synthesis). |
| 2 Freud | confirmed (second-hand, correctly labelled) | ORF page and UNESCO text both carry the "Samstag abends ..." line and the Ferenczi/"ungarischen Kollegen" wording; neither gives a date. The "Behagen in der Unkultur" line is only in the authors' own texts. |
| 3 Ulmann / "jejune" | confirmed with one overstatement fixed | Dummett's passage (`sources/12/dum/p4.txt` lines 2874-2883) says "This rather jejune game was ousted by a much better and quite different one", "described in both of Ulmann's books", "agree very closely"; dating "about 1895" at line 1547 and "present century" at 1554. Headline 2 said the Wikipedia footnote "is a misattribution", but section 3 of the same note says Ulmann's pp. 244-245 were never read. Softened to "probably". |
| 4 Alvensleben 1853 | confirmed | Both page images read: printed pp. 576-577, heading "Tarok-Tappen unter zwei Personen oder das sogenannte Strohmandeln", "Kauftalon", "Skis", "Schmiert ... viel Könige und Figuren", "jene neuere Erfindung 'Tatteln'". The transcription and translation match. The advertisement for Karl Haas's manual precedes it on p. 576. |
| 5 Tepl congress | confirmed, one name misread | Page 9 of the *Neuigkeits-Welt-Blatt* of Tue 24 Jan 1893 (header "3. Bogen ... Dienstag den 24. Jänner 1893"; ANNO snippets say p. 9). The text on the crops matches the note: Egid Ranwolf as chairman and president, Dechant Ambros Winkler's motion, telegrams, "Laute / Stumme Kibitze", Jahreskarten 50 / 25 kr, "Kuckuck" box, Kartengeld 3 kr / 1 kr, 28 Partien, "22 Valat und 54 Pagat ultimo", Valat marked with a "Tusch" and Pagat ultimo with a Hornsignal, play "noch lange nach Mitternacht". The telegram place is "Auschowitz" (Úšovice by Marienbad), not "Anschowitz". The 28 Dec 1893 report (crop `d1.jpg`) reads "Sonntag, 17. d. M." and "am 15. Jänner 1893", so 15 Jan (a Sunday) is right (an OCR snippet reading "13." is noise). No Strohmandl rules are printed. |
| 6 Werner 2nd ed. 1891 | confirmed | The NIZ crop (7 Jun 1891, p. 13) reads "(»Das moderne Tarokspiel« von K. Werner), welches soeben in A. Hartleben's Verlag in zweiter, vermehrter und verbesserter Auflage erschienen ist ... um den geringen Preis von 70 kr." Further 1892 notices say "Zweite verm. ..." / "Zweite Auflage" (`w.jsonl`). Caveat: the Pester Lloyd (4 Jun 1891), Kaufmännische Zeitschrift (1 Aug 1891) and Allgemeine Sport-Zeitung (1891, p. 636) snippets show only the title and the first-edition subtitle, so they do not themselves show a second edition; NIZ and the 1892 notices carry the proof. Also, the Sport-Zeitung of 1887 (p. 347) already recommends Werner "Verlag von A. Hartleben", so Hartleben published the first edition too (no conflict with the note). |
| 7 Haas 1839 | confirmed as stated | The note honestly says p. 121 was not opened and that the 1839 text is an inference. |
| 8 harbour-tarock | confirmed | `LICENSE` header and `SPDX-License-Identifier: GPL-3.0-or-later`; README "GPL-3.0-or-later, Copyright 2026 smatkovi"; README line 20 "planned after 1.0"; the spec is 2,864 lines; it lists Beck, Bamberger, Dummett and the 1829/1839/Ulmann texts as not read in full; its 21 interpretations are as listed. The GitHub API's NOASSERTION (RB) is consistent with the project header on the LICENSE file. |
| 9 Museum pack | confirmed, one detail fixed | On the contact sheets: numerals top-left (upper picture) and bottom-right, rotated; IIII (four thick strokes), IX, XIV, XIX, XVIII, XVII, XIII, VIII, VI, VII standard; Sküs (image 01) has no numeral; image 21 is a crowned eagle on a stone "INDUSTRIE UND GLÜCK" with flowers. The sword stands upright beside the eagle; it is not in its claws (headline said "holding a sword"; fixed). Suit cards show a large pip top-left and bottom-right, no letter index. The genre-scene descriptions (III couple, IIII groom with horse, VI harbour, XIV vintage, XVIII kneeling man with tub, XIX two oriental men) match the images. |

Edits made to note 12 (all marked `[REVIEW RF]`):

| Location | Problem | Evidence | Action |
|---|---|---|---|
| Headline 2 | "is a misattribution" stated flatly, though Ulmann pp. 244-245 were never read | note 12 section 3 | softened to "probably a misattribution" |
| Headline 5 | "crowned eagle holding a sword" | contact sheet 21 | "sword standing upright beside it" |
| §5, telegrams | "Anschowitz" | page image `c1.jpg` ("Aufchowitz" in Fraktur = Auschowitz) | corrected, with the reason |
| §5, officers | "Suppplent" (typo) | image: "Supplenten" | corrected |

Note 12 verdict: usable with care, close to trustworthy. Its transcriptions are accurate, its inferences are labelled, and the second-hand material is labelled as such.

## B. Synthesis: changes made

| # | Location | Problem | Evidence | Action |
|---|---|---|---|---|
| 1 | §7.3 table, shuffle | "Ein guter Misch ist das halbe Spiel" attributed to Löwy 1903 [05c LIT-S9] | 05c table row and LIT-S8: Ranzenhofer, "Königrufen", Pilsner Tagblatt 1917, p. 2 | re-attributed |
| 2 | §7.5 "Er hängt scho' der Max'" | cited as [05 §3] (no such section); glossed as if the phrase meant the Sküs | 05 Headline 8 + review note: Hügel 1873 gives the phrase ("his wish is near") and, separately, Max' as a Sküs name; linking them is the note's inference | gloss and citation corrected |
| 3 | §7.3 table, "Ihr Blatt, Herr Rosenblatt!" | RB struck the name under the kindness rule; the line is a reunion line at the end of the play, not a "new hand" cue; known only via Sedlaczek & Mayr (second-hand); 05c: tied to the Shoah narrative, "use with care or not at all" | 05 row 52 review; 05 Bockerer bullet; 05c LIT-S67 | table row now proposes only "Ihr Blatt!", with placement and second-hand status stated |
| 4 | Exec summary, praise bullet | "Ulmann (1888) calls it the 'Lieblingsspiele aller denkenden Tarokfreunde'" | page images of the *Klagenfurter Zeitung* 30 Mar 1888: the sentence is the reviewer's prose | attributed to the review |
| 5 | Exec summary, "Why it matters" | "four modern ones (Beck ...)" implies four read books | 02: Beck never read; Dummett is a digest | caveat added |
| 6 | §1 deal, §3.2 "Vienna 2007" preset, §3.3 row 1 | Mayr & Sedlaczek said to lay the packets only after a bid | `wz_2016.html`: packets are dealt with the hands; `wz_2016_p2.html`: dealing them after the bid is advised only if one uses the throw-in rule of "alte Tarockbücher"; their default for both-pass is a simple game | all three places corrected; Beck/Bermann added to the table |
| 7 | §2.4 timeline, §5 Tepl | "a joke congress"; "is a joke, not a rule code"; "each [Valat and Pagat ultimo] greeted with a fanfare and a horn signal" | note 12 + page crops: gathering of about 300 with real kibitz fees and a charity box; no Strohmandl rules printed; the "unified rules" wording is an editor's compression; Tusch for Valat, horn for Pagat ultimo; RD/RE call it partly jocular | rewritten to what the evidence shows |
| 8 | §5 Tepl telegram | "Anschowitz" | image | "Auschowitz (Úšovice)" |
| 9 | §5 1850 court report | "a stranger wins ... off a man"; "the debt ends in court" | 04 §2 review correction: the subject of "gewann" is Hr. W.; the stranger left owing; fraud case unverified; note 03 still has the old reading | corrected, cross-cited |
| 10 | §5 1912 gambling den | "a man plays Strohmandl as cover while a police agent kibitzes him" | 04 (fuller): the detective played Strohmandl with another agent as cover; 03 reads it as an agent kibitzing | worded per 04, with the 03/04 difference stated |
| 11 | Exec "three games", §2.3, §2.4 | "firmly attested from 1886-88"; 1886 Blaue Donau list | 03 sources: 1886 hit is an OCR snippet ("en ckeux"); RC: "(en deux)" can denote Neunzehnerrufen with two Strohmänner (the 1888 review names that too) | now "firmly from 1888", the 1886 hit flagged weak |
| 12 | §2.3 | "Wikipedia's footnote ... is a misattribution" | 12 section 3: Ulmann's pages unread | "probably" |
| 13 | §2.1 | dead hand "not a rules ancestor" cited to [01 §Headline 3], which does not say it | 01 line 39 (History, "1800" item) | citation fixed |
| 14 | §7.1 | "Ich spiele / Halte ich / Ich tappe" presented as Strohmandeln bids | 05d line 90 (Dummett pp. 453-454, Dreier / Tapp-Tarock calls) | relabelled |
| 15 | §7.2 | 1904 quotation printed as "Die Kiebitzordnung hat nur einen Paragraphen" | 05c LIT-S44: "Das 'Kiebitz'-Reglement hat auch nur einen Paragraphen" | quotation corrected |
| 16 | §7.2 | "on placards in inns [05c]" | LIT-S40, Grazer Volksblatt 6 Oct 1904: "a großes Bild ... wo drauf steht" | rephrased and cited |
| 17 | §7.2 | design proposal cited [05c §Headline 2] (Nestroy item) | 05c Headline 1 is the Kibitz / § 13 item | citation fixed |
| 18 | §7.4 | "Mondfang has no period German attestation [05d]" contradicts 05 (Werner's Mondfang penalty; Piatnik) and 05c LIT-S20 (1887), S55 (1925), S56 (1893) | 05 glossary, 05c rows; 05d's claim rests on an archive.org full-text search | rewritten: not recorded for Strohmandeln, but attested for other Tarock games |
| 19 | §7.5 | "Zieh endlich deine Fäustlinge aus!" listed as a plain Viennese phrase | 09 / RA: known only from a newspaper review of Mayr & Sedlaczek's book | second-hand status and context added |
| 20 | §5 atmosphere | "rattle of the sixpences" | 09: Sechser = six-kreuzer coins | corrected |
| 21 | §6 Hungarian | Parlaghy "c. 1889" | 11 headline 3: Budapest 1888 (RE: printer footers read 1889) | "1888" with the note |
| 22 | §6 Czech, §2.4 | invented direct question "umíte hrát štromandla?" printed as the priest's words | `ceskaskola1891.txt` line 41 and `vychovatel1892.txt` line 20: indirect "umím-li hráti štromandla" | quotation replaced by the real wording |
| 23 | §6 Polish | 1901 *Djabeł* line given without context | saved OCR checked: Kraków, *Djabeł* 1901 nr 10, "Trul, skiz, dziesięć taroków i pagat złapany" is generic Tarock noise | context added |
| 24 | §8 Bermann | quotation reordered: "etwa 20 bis 25 Points in Honneurs und höheren Figuren" | `bermann1894/ocr.txt`: "in Honneurs und höheren Figuren etwa 20 bis 25 Points in der Handkarte" | corrected |
| 25 | §1 follow-suit bullet | "If you can't follow at all, you must play a tarock" | Werner: "sonst mit Tarok gestochen werden"; Furr: if neither suit nor trump is possible, any card; "no obligation to win" is an implication only | tightened |
| 26 | §3.1 | "Dummett, comparing eight descriptions" | 02: Dummett names Ulmann (two books), Bermann, Werner, Unger, Beck, Löw, Babsch, Widder, the Piatnik leaflet; "eight" appears nowhere | list replaces the number |
| 27 | §7.3 table, "Winning on 36" | presented as a table remark | 03: it is a 1908 parliamentary joke about a two-thirds vote | context added |
| 28 | §7.3 table, "Versehen ist verspielt" | translation "a slip is a play" | sense is forfeiture (card played in error stands) | retranslated |
| 29 | §7.3 table, Nestroy trull line | no source id; modern edition differs | 05c LIT-S1; 05 row 17 review | cited with edition note |
| 30 | §7.1 Rostopschin | M&S 2002 page quoted without its clause on Strohmann-Tarock | `strohmann.txt` line 647 | clause added |
| 31 | §2.4 timeline | "§ 13 first attested" 1897; 1853 row | RD: first found in saved snippets; 12 item 4: "Erfindung" | reworded |
| 32 | Method | "more than 1,500 quoted fragments were matched by script"; "over 100 newspaper page images read by eye" | logs RA-RE: about 2,000 extracted, about 1,300 matched automatically (RA 287, RB about 670, RC 87, RD about 250, RE 28), the rest mostly translations and artefacts; eye checks counted in dozens (RB 34 crops, RD 31 rows, etc.) | restated |
| 33 | §12 | note 12 "not separately reviewed"; note 03 "trustworthy" | this review; the 1850 slip in 03 | rows updated |
| 34 | How to read, Appendix A | `[RF]` and `RF.md` not mentioned | | added |

## C. Checked and found sound (no change)

- Executive summary, §1: Werner's deal, bidding, scoring (3/4/2, 35-35 draw doubles, Trull and kings 1 point each, Pagat ultimo cannot be announced, Valat 12), "wenn es sich nicht zur Aufnahme ... eignet", "35 und 2 Blatt" as one threshold; all against `3724659.txt`. The quotations from Bermann, Ulmann, Werner (Impassiren, Honneurs) are in the saved OCR.
- Exec "three games": Cäsar 1800 (25/28, 25 aside, "wie das Piquet, nach Partie bis 100 Points"); Mößler 1805; Haas 1829 p. 94 text and translation; reprints 1834, 1842 (p. 121), Tendler 1830/45/46; 1824 bare name; Vanderheid 1855.
- §3.3: values (2/3/4; 1/2/3; 1/2/4), ties, honours, Pagat ultimo, bird bonuses, Kontra (Löw), Valat, redeal, who leads all match 02 §1/§3. Furr's big-win figures match `furr_djvu.txt` ("With 45: ... Scores 4"; "With 26: ... Scores 5 for the defender").
- §3.4: Bermann "ist beides der Fall, so steht die Wahl ... frei"; Codex §31.
- §8 numbers: 6.1 tarocks, 1 in 55, 1 in 232, 1 in 15,298, 4.9 / 0.9, 0.19 (0.186), 2.8 cards (2.76), 13.0 %, 39.8 % (conditional, as stated) all match 07 §5. "3-7 %" is the span of 07's readings (2.6 % to 7.1 %) rather than a single figure.
- §9: Hofmann 1815, Neumayer 1884/1890, 63 x 113 mm, backs, PDM mark (RA), numerals (my own image check), Uhu/eagle.
- §11 matches 10 §5a and RC (identical files, "better" files, seven celebration scenes, submodule objections).
- §6 Hungarian and Czech quotes (Jókai, Mikszáth) are in the saved OCR; Czech 1932 p. 370 heading only.
- Kraków 1901 is real (see change 23). The 1910 ladder and the 1914 and 1926 quotations are in the saved ANNO snippets/crops and correctly cited as [03 §4]; the 1926 quotation survives only as an OCR snippet ("zu '-reit"), so its last word rests on 03's reading.

## D. Quotation statistics (synthesis sample)

28 strings from the synthesis were normalised (lower case, long s, umlauts, punctuation dropped) and searched in all saved text: 19 exact; 8 misses; 1 deliberate control string (an invented sentence, correctly not found). Of the 8 misses: Werner "Die größte Rolle ... Impassiren" is split by an OCR line break (line 4426 of `3724659.txt`), fine; Bermann (word order) fixed; 1926 *NWT* snippet is OCR-garbled, see C; 1887 *NWT* wife passage and the 1924 *Welt-Blatt* lines are image-only in the saved sources (03 and RD read them by eye; I did not re-read these images); Jókai matched after spelling ("Hohó I Hiszen nálam van a pagát !", line 17529 of the OCR); the Czech line was an invented direct question (fixed).

Translations spot-checked (15): Haas 1829, Cäsar 1800, Werner, Bermann, Ulmann (Neunzehnerrufen, Codex §31), the Figaro line, the Jókai and Mikszáth lines, M&S: faithful. Over-confident rendering found: "a slip is a play" (fixed).

## E. New findings not put into the notes

- Note 03 (§4 "1850" and headline 5) still has the winner of the 1850 game reversed; note 04 corrected it, 03 did not. I did not edit 03.
- Note 03 and 04 read the 1912 police-raid sentence differently (agent kibitzing vs agents playing as cover); only the page image can settle it.
- Notes 05 and 05d contradict each other on period "Mondfang" (resolved in the synthesis in favour of the quoted texts).
- The 2002 M&S page says Rostopschin also leads to interesting situations in Strohmann-Tarock, which weakens the reading "Rostopschin belongs to Königrufen only"; the game's Rostopschin toggle is therefore better supported than 02 and the synthesis suggested.
- RG's point that "70 in all" depends on the leftover cards (remainders) is not addressed by any source in the notes; the synthesis states 70 as Dummett and Furr do. Left as stated.
- The Allgemeine Sport-Zeitung of 1887, p. 347, recommends Werner's book from Hartleben, i.e. the first edition of 1883 was a Hartleben book too (no consequence for the synthesis).

## F. Verdicts

- **Note 12**: usable with care, close to trustworthy. Four small edits; all nine items stand.
- **Synthesis**: usable with care. About 30 corrections, none touching the core rules account. The worst: the Löwy/Ranzenhofer misattribution, the Mayr & Sedlaczek packet timing (in three places), the reviewer's praise credited to Ulmann, the reversed winner of the 1850 report, the Bockerer line's name and placement, the Tepl "joke" overstatement, and an invented Czech quotation. No quotation was found to be invented in the notes' own sources.
