# RJ: review of strohmandeln-lit-review.md (v2), changed text

Scope: the v2 diff (sections 1, 3.2 to 3.4, 4, 6 Elizza item, 13, 14, executive summary, 2.3). Rest read lightly. All fixes are in place in the synthesis, marked [REVIEW RJ]. No commits.

## Changes and problems

| Location | Problem | Evidence | Action |
|---|---|---|---|
| Exec summary, 2.3, timeline 1887 and 1888 rows | The 1888 Klagenfurt praise and the 1887 Brno sentence were treated as two texts, with the praise possibly about Neunzehnerrufen | Page images `research/sources/03/anno/kfz18880330_p1_a/b/c.jpg` and Brno OCR `research/sources/06/cz/ocr/0429f8b0...txt`: both papers print one and the same paragraph, a survey of Tarock kinds. The praise is its own sentence about "Das 'Strohmann-Tarok' (en deux)", after a separate sentence about Neunzehnerrufen ("auch en deux mit zwei Strohmännern"). Ulmann's preface (`ulmann1899/ocr.txt` ~l. 795) lists "das Strohmann-Tarok en deux, das Paskiewitsch, das Neunzehnerrufen mit Strohmann" as three games. Klagenfurt links the paragraph to Ulmann only by a footnote after it; the Brno OCR ends mid-column, so a footnote there is unknown | Rewrote: one syndicated text, not two attestations; praise most probably about the two-hand game; the Neunzehnerrufen reading is unsupported for this sentence; the words are the reviewer's |
| 2.3 | "The preface is known through the review" | The preface (p. V, 1st-ed. preface) is in the 1899 OCR | Corrected |
| 14 "Decided" and section 10 | Dates and attribution | Coordinator's instruction | Rewritten: maintainer decisions on 14 Oct 2026, "the maintainer's decision in the project's working session" (game, rules, session A, cards); coordinator chose "J" under the maintainer's instruction; code sharing stays open. Note: the notes themselves do not carry these dates |
| 1.1 | "kings and the three top trumps (Sküs, XXI, I)": trump I is the lowest trump | Note 14 rule 15 | Corrected |
| 1.1 | Terms undefined or confusing for a newcomer (see list below) | Read as a newcomer | Added definitions: tarock vs Tarock, counting in threes, Pagat ultimo at first mention, simple game, draw doubling, void deal, "announce", "notes advise" changed to "strategy notes" |
| 4.1 | Haas quote "kleine Schiefertafeln mit Griffeln" | Saved OCR (`haas1829/ocr.txt` l. 455) reads "mit Niedenſtiften"; "Griffeln" appears only in note 13, no page image of that page saved | Marked [UNVERIFIED — review]; the sense is secure |
| 4.2 | "Dummett double the next round of two deals" | Only Mayr & Sedlaczek say a round of two deals (RI (d), note 02 A12); Dummett says "following round"; Bermann leaves it to agreement | Corrected |
| 4.3 | Ranzenhofer 1917 "Nur noch eine Runde" is from a Königrufen story (four-hand) | Note 05c LIT-S8 | Flagged in the table |
| 6 | "the Hofoper singer Elise Elizza" | Page crop `research/sources/04/crops/ANNO_nwj19120407_k4_p11_1.jpg` reads only "(Hofoper)". The quotation matches the page word for word | "singer" flagged as an assumption |

## Checks that passed

- Rules in 1.2, 3.2 and 3.3 match note 14: last packet card taken and shown (rule 10); void deal with no tarock, same dealer redeals (rule 4); both pass defaults to the simple game (rule 7); values 2/3/4, honours 1 each from hand, quiet Pagat ultimo 1, Valat 12, forehand leads (rules 19, 20, 21, 25, 14); draw doubles the next deal (session section); three settings only. The tally margins in the 3.2 table match note 14 (4-1, 6-1, 4-3, 3-1, 6-2, 5-1, 5-1, 3-2).
- Section 4 follows note 14 "The session in detail" (Tour = 2 deals, 3 or 6 Touren, extension by consent, slate, 2 kreuzer labelled as Tarock evidence, pending doubling lapses) and states plainly that it is a reconstruction. The 4.1 evidence matches the extracts in `research/sources/13/extracts/` (Ulmann: "drei oder sechs Touren", loser announces; Bermann: Courtoisie, allseitige Zustimmung; Werner: Conto, fortlaufende Addition, differences).
- The statistics in 1.2 (1 in 15,298; 2.76 cards, 13.0% none) match note 07, and the sim takes the last card regardless, so it is consistent with rule 10.
- Section 14 now matches the coordinator's dates. The "Decided" list no longer holds the old match-length or Vienna 1891 advice.

## Quote statistics

Sample of the new or changed German quotations traced to saved sources: 26 checked, 25 found (several by eye through OCR noise: Werner "Impassiren" appears as "Jmpassiren"). Found: Cäsar 1800, Haas 1829 rules and verdict, Haas "Mit Kreide befleckte Finger", Bermann Courtoisie and "nur mit allseitiger Zustimmung", Ulmann "drei oder sechs Touren" and preface, Werner "bei fortlaufender Addition" and "natürlich einander gleich", Werner "Wechseln und Herausgeben", Strauss "zwei bis drei Kreuzer der Point", "den Point um 5 kr.", Agramer "drei Meister", "dringende Revanche", "Spielschulden sind Ehrenschulden", "Wachmann schon bedeutungsvoll", Nestroy "Beutel zu Haus", "meine Frau deckt als letzte Karte", "schäbiges Strohmandl", Elizza (page image), "Tarockirern" (Auschowitz), Klagenfurt and Brno praise (page image, OCR). Not confirmed: Haas "Griffeln" (above). Not in the saved text under the quoted spelling: "Werner: eine außerordentliche Seltenheit" was found in note 03's transcript but not in a saved OCR line I could locate (not re-run on the Werner PDF text; low risk). The Ranzenhofer and Freiheit 1927 quotes were taken from note 05c and not re-checked.

## Newcomer read of 1.1: what was confusing (before my edits)

1. "tarock" and "Tarock" and "trump" were used before being tied together.
2. "Counted in thirds" was unexplained.
3. "The notes advise": the reader has no notes.
4. Pagat ultimo appeared in the straw-men passage before it was defined.
5. "Simple game": never said who wins, or that a draw is possible.
6. "Announced" (Trull, kings) was undefined.
7. "Held together in the hand": when? Not clear that cards taken from the packets count.
8. "Declarer" is defined, but the text never says what happens to the opponent in a won deal.
9. The void deal and the draw were missing from the plain-English part.

Items 1 to 6 and 9 are fixed. Items 7 and 8 remain: whether packet cards count toward the honours is in 1.2 but not in 1.1; the scoring of the opponent is left implicit. Left for the coordinator.

## New findings, not put into the synthesis

- Whether the counting of left-over cards (one or two cards over, note 02 A17) needs a decision before the tutor explains scoring.
- Ulmann's text gives Touren as dealing rounds ("Bei Beginn der Touren hat jeder Partner nur noch drei-, respective sechsmal Karten zu geben"), which supports Tour = one deal per player; still an inference for two players.

## Verdict

Synthesis v2: usable with care, after the fixes above. The rules and session sections are faithful to notes 13 and 14. The 1888/1887 doubt is resolved in favour of the praise being about the two-hand game, in one syndicated paragraph.
