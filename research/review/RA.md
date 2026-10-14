# Review RA: notes 01, 02, 08, 09

Reviewer RA. Brief: research/_REVIEW_BRIEF.md. Changes are marked `[REVIEW RA ...]` in the notes. Scratch script: /tmp/.../scratchpad/qc.py (not in sources/).

## Change / problem log

| Note | Location | Problem | Evidence | Action |
|---|---|---|---|---|
| 02 | Headline 3; table row "4th/last card"; A3; A13; canonical rule 7; note under table | Werner's last-card rule misreported as "stays face up even if K/Tarock". That is Dummett's gloss. Werner's own sentence carries a condition: "Auch das letzte Blatt eines jeden Talon-Päckchens muß, wenn es sich nicht zur Aufnahme in die Handkarte eignet, so lange offen liegen bleiben, bis es gelegentlich gespielt wird" (sources/02b/wb1.txt p.133; wb2.txt). Dummett p.458 says Werner holds it "remains face up ... even if it is a King or a Tarock". The real split is Beck/Babsch/M&S (take any last card, M&S: "Die letzte Karte eines Pakets wird sofort aufgenommen, auch wenn es sich um eine Farbkarte handelt", wz_2016.html) vs Werner (a low last card stays up). Also reported by RC. | Texts compared directly | Corrected in place in all 6 places, original claim kept with explanation. Dummett's wording is "as soon as it is exposed" for Beck/Babsch, which 02 reports correctly. |
| 02 | Headline 4 | "Torberg say XVII and XVIII" presented as if Torberg read. Only the Wikipedia (en/de Königrufen) quotation exists locally; Torberg's book never seen (09 torberg_*.json/txt are HTTP 401 stubs). Same Wikipedia articles' glossary says XVII and XVI. | wp_koenigrufen_en/de.txt lines 288/279 and 766/746 | Marked second-hand. |
| 02 | Headline 6 | "Rostopschin is a Torberg-era Cafe Central bonus": Torberg (as quoted) calls Rostopschin the name of a whole variant; "bonus" is Sedlaczek's correction, also only via Wikipedia (Mayr/Sedlaczek 2001 pp.255ff unseen). | same | Reworded, [UNVERIFIED]. |
| 02 | table, "sofort nach Einziehen des Stiches" | M&S quote truncated; source reads "des betreffenden Stiches" | wz_2016.html | Fixed. |
| 01 | Section 3, Mößler 1805 | Mistranslation: "dealer gives 5 cards, his opponent 25, himself 28" (5+25+28+25 = 83, pack is 78). "giebt zu 5 Karten" = deals in fives; the dealer's extra 3 go to the Scat ("Der Geber legt die sich mehr gegebenen 3 Karten in den Scat") | onb_copies/mossler1805_pp145-146_ocr.txt | Corrected with note. |
| 01 | Section 3, Ulmann "Das Spiel wird durch den Strohmann viel anziehender" | Quote is real (Ulmann p.69, read in page image Ulmann.pdf p.41) but it closes the Neunzehnerrufen chapter and refers to Neunzehnerrufen en deux ("das Neunzehnerrufen en deux, bei welchem zwei Strohmänner aufliegen, bietet ... weitaus höheres Interesse"); Strohmann-Tarok starts in the next paragraph (Cap. VI). Not evidence that Ulmann praised Strohmandeln in those words. The other Ulmann quotes (interessanteste, Vorzug, seines eigenen Glückes Schmied) are in the same page and correct. | page image | Annotated. |
| 01 | "infobox p.109 is off by one" (Furr) | OCR page numbers are ambiguous (header vs footer) so the claim cannot be verified from the saved text. | furr_djvu.txt lines 4858-5025 | Marked UNVERIFIED. |
| 09 | Best of 2 (Freud retort) | Presented as "the retort". WZ 2006 says "soll ... geantwortet haben" (is said to have replied), no source. UNESCO 2024 (same authors) words it "Lassen Sie mir doch ..." not "uns", and attributes the complaint to partners about worn cards. Not independent. | wz_freud_arch.html; unesco.txt lines 91-97 | Annotated as folklore. |
| 09 | Best of 2 / sec.1 (Ferenczi letter) | Letter known only as a quotation in a newspaper article (undated, no edition cited). UNESCO says only "einem ungarischen Kollegen". Not the Ferenczi letters read. I could not find it by web search (Briefwechsel catalogue pages only). Quote itself matches both articles. | same | Marked UNVERIFIED as primary. |
| 09 | Best of 1, "Zeig ihn her ..." | The review (nunu.at, by Peter Menasse per page metadata) lists it only among impatience nags ("oder grober"); meaning "when you hesitate to play a low trump" is the note author's, and the phrase has an evident double entendre. Kindness/taste: do not use. | nunu.html | Flagged. |
| 09 | Best of 8 (Quapil = Kvapil) | Dictionary entry (enzyklo.de) is about the surname only; link to the card is an inference. | enzyklo_quapil.html | Flagged UNVERIFIED. |
| 08 | Headline, Volkskundemuseum scan | CC PDM verified in the museum's METS record (vk_mets_201122.xml): every one of 55 images has accessCondition "CC PDM 1.0"; scans made 06.2024. But largest scans are 2314x4101/2306x4097 (note says up to 2306x4096) and the museum's own date is 1903-1912 (note: c.1904-12, from Witzmann, who argues 1904-12 from firm addresses). PDM is a mark, not a waiver. | METS xml | Annotated; no error of substance. |
| 08 | Numerals / eagle on II | Confirmed by eye on the ÖMV scan (contact sheets): trump 4 is IIII, XVIII is XVIII, II shows crowned eagle on the "Industrie und Glück" stone; Witzmann p.~98 and Alscher Stuf7023 line 777 agree ("Adler auf Tarock II (heute Uhu genannt)"). Observation: court cards carry a small suit pip in the corner, so "no corner indices" holds only for letters/numerals. | images omv87529_01..24 | Annotated. |
| 08 | IPCS "red suits A-4 ... 21 horizontally divided trumps ... Fool or Skys" | Genuine, but it is Pattern Sheet 14 (Type A). ps-15/16 (Types B/C) say "ace-4" and "21 trump cards", no "horizontally divided". Cited S4+S6. | web/ipcs14,15,16.html | Left; logged. |
| 08, 09 | XXI "Mond" description | 08 says Type C XXI = sultan scene (via WP/own reading of Alscher render); the ÖMV Type B scan's XXI is a dancing couple. Not an error but note it is type-specific. | scan | Logged only. |

## Particular worries

- **Rostopschin trump pair.** Three-way conflict is real and correctly stated in 01 (VII/VIII in en Strohmandeln only since 16 June 2021; the 5 June 2020 revision had XVII/XVIII, verified in allrevs_en.json; Königrufen glossary XVII/XVI, verified in both wikitexts). VII/VIII is almost certainly a lost-digit typo. Everything about Torberg (XVII/XVIII, "Ross!"/"Topschin!", "sonst galt's nicht", Café Central, p.217) rests solely on Wikipedia's quotation; Torberg's book was never read by 01, 02 or 09 (09 states this; 02 did not carry it through; now fixed). Furr/tarocchino.com independently give XVII/XVIII but are one text.
- **Last-card rule / Dummett "exposed".** See log row 1. Dummett writes "taken into the player's hand as soon as it is exposed, even if it is not a King or a Tarock" for Beck and Babsch; 02 reports that correctly. Beck and Babsch themselves were not seen (second-hand via Dummett). 01's "de/en say last card goes in 'ohne Aufdecken' (unrevealed)" is accurate to de/en Wikipedia ("ohne Aufdecken"/"without being revealed"), which differs from Dummett's "as soon as it is exposed" and M&S ("sofort aufgenommen"); 02 A4 notes this and leaves it open.
- **Freud.** See log. Quotes match saved articles exactly.
- **Banter via review.** The five phrases are verbatim in the nunu.at review (spelling "warats a Autobus" normalised by 09 to "wär's"; "I'" for "I"). 09 labels them as via review. The review's own context: Fäustlinge = misdealing; Autobus = over-long post-mortem; Hos'n = gloating winner; Staudenhocker = cowardly players. "Staudenhocker" and "Hos'n" are acceptable only with care under the kindness rule (cowardice is character, "stink'n" is coarse).
- **08 numerals / eagle / licence.** All confirmed (above).

## New findings / leads (not put into the notes)

1. Werner's text (p.133) says Pagat ultimo "kann natürlich nie ultimo angesagt werden" because the opponent may turn a tarock as the last card of his Talon; M&S give the same reason. Neither says what happens with a last-card tarock.
2. Ulmann p.69 first sentence of Cap. VI context: the Strohmann was already used in Neunzehnerrufen ("zwei Strohmänner aufliegen"), consistent with 01's lead.
3. UNESCO 2024 and WZ 2006 both by Mayr & Sedlaczek: treat as one source for Freud anecdotes.
4. Haas 1829 p.94 OCR reads "wäre es auch der Scüs"; the OCR has "aus". The page image (onb_copies/haas1829_p94.jpg) was not re-examined by me.

## Quote match statistics

Automatic extraction (quoted strings of 10+ characters, normalised for case, diacritics, long s, hyphenation; searched in all saved text of sources/01, 02, 02b, 03-07*, 08, 09, 11, anno plus wp_*.txt, PDFs via pdftotext). The regex pairs quote marks crudely, so most "unmatched" items are fragments between quotes, English translations, project/file names or own text.

Results (raw, regex-based, includes false misses):

| Note | quotes extracted | checked | matched automatically | unmatched |
|---|---|---|---|---|
| 01 | 142 | 142 | 73 (51%) | 69 |
| 02 | 106 | 104 | 63 (61%) | 41 |
| 08 | 100 | 99 | 51 (52%) | 48 |
| 09 | 178 | 176 | 100 (57%) | 76 |

Triage of the unmatched by eye: the great majority are artefacts (text between two apostrophes, table fragments, English translations, file names, project commit titles, own headings). Genuine German/foreign quotes that missed automatically and were then checked by hand: Mößler 1805 and Haas 1829 (OCR noise: matched by eye in OCR, page-image reading for Haas; but see translation fix); Ulmann (image-only PDF, checked by reading page 69); es/pt Wikipedia lines (present in sources/01/search2/raw); Witzmann/Alscher quotes (OCR hyphenation/line breaks; confirmed in 1705112.txt, Hofmann_doc.txt); en/de Wikipedia "corner indices" quotes (confirmed in wp/); 09 banter lines (confirmed in nunu.html with spelling "warats"); 09 "Das halbe Vermögen verspielt" (TOC wording differs from 09's rendering, fixed); 09 "Ein großartiger Pagat!" (adapted from "einen großartigen Pagat gemacht", not verbatim, see note); 09 snippet-only quotes (Alpen "Wenn der Tarok nicht wäre", Golem) are already flagged UNVERIFIED by the author and were not re-fetched. No fabricated quote found. Effective match after triage: no unexplained miss among quotes the authors marked as read; errors found are in translation and context, not in quotation.

## Verdicts

- **01 Wikipedia: usable with care to trustworthy.** Strong provenance work (revision histories re-verified for Rostopschin and playing time). Two real errors fixed (Mößler translation, Ulmann context). Ulmann 1890 pp. 244-245, 1839 p. 121, Beck, Löw, Bamberger remain unread.
- **02 Modern rules: usable with care.** Werner last-card misreport (the main "rules fork") corrected; Torberg claims relabelled second-hand. Beck, Babsch, Löw known only through Dummett/Wikipedia; table is a good menu otherwise. Re-verified: Werner pp. 129-135, Dummett pp. 457-459, M&S 2007, Furr, pagat claims.
- **08 Cards and art: trustworthy.** Licence, numerals (IIII), eagle on II verified in the museum record and scans; only small precision issues.
- **09 Hunches: usable with care.** Banter and Freud material is second-hand (a book review; a newspaper article quoting an undated letter, same authors as UNESCO text); two lines flagged as unsuitable or unverified; Quapil=Kvapil is inference. Honest labelling elsewhere.
