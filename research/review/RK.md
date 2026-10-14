# RK: review of the opponent's phrase bank (`tools/phrases.py`)

Scope: all 69 groups and 3xx wordings, English and German, against `CLAUDE.md`, `research/14-consensus-rules.md`, `docs/RULES.md` and the research notes. Fixes are in commit "Phrase bank review RK"; the bank, `docs/PHRASES.md` and `web3d/words.json` were regenerated; `bin/gate` passes.

## Changes

| Group | Line | Problem | Evidence | Action |
|---|---|---|---|---|
| both-pass | "A simple game for one Point" (Ul) | Rules: simple win is 2 by default (`values` 2/3/4); German not in Ulmann | docs/RULES.md 9.1 | removed |
| draw | Ul "es zählt nicht" | Suggests nothing is paid; honours and ultimo are paid in a draw (RULES 11.7, 12.4); Ulmann's no-doubling variant contradicts rule 18 | RULES 11.7, 12.4; 14 rule 18 | rewritten "keiner gewinnt", tag T |
| draw | "Keinem sein Spiel" | Ungrammatical dative possessive | | rewritten |
| valat-mine, valat-yours | "Pagat, Ultimo, Volat" (Ne2) x2 | Reads as an announced ultimo and valat; both are quiet | 14 rule 21; RULES 10.1 | removed |
| valat-mine | "Valat! Der ganze Tisch...", "Matsch! Jeder Stich", "Kein Stich für Sie" (tagged Ne2/Ne3) | German is the table's, not Nestroy's | 05 note quotes only "Ich bin matsch", "Matsch worden" | retagged T |
| run-yours | "Der macht drei, vier Ultimo in einem Nu" (Ne1) | Several ultimos per deal is impossible; the line is Johann's boast about a player, not about a run | 05-S6 | replaced by T line |
| ultimo-lost-mine, -yours | "Ultimo and game both gone" (Nw) | Losing the ultimo costs the ultimo only; the game is scored separately | RULES 12 | English and German now "The ultimo is lost" |
| ultimo-won-mine, -yours | Wander "Behalt ein gute Kart auf die letzte Less" | 16th-century German, unreadable ("Less"); not Austrian | 05b LEX-S10 | removed, T lines added to keep 5 |
| ultimo-won-mine | Ba "Wenn mein Pagat den letzten sticht" | Conditional German, exclamation English | 05-S12 | German now "Mein Pagat sticht den letzten!" |
| ultimo-won-yours | Ba "ein Dorn in den Augen" | "my eye" in English, plural "the others" in German | | German "mir ein Dorn im Auge" |
| deal-mine, deal-yours | Wander "Hastus wol gemischt, so karts wol" | Archaic Low-German-flavoured, not Austrian, unreadable | 05b LEX-S21 | removed |
| deal-yours | "Sie geben, und gerecht" | Stiff, ungrammatical | | rewritten, T |
| void-deal | Ne1 "Vergeben. The Tour waits." | English added a claim not in German | 05 table, row 17 | English "Void: a misdeal." (Ne1 wording is Bonz 1890 only) |
| slow | Mo "Who leads? I have forgotten." | English invented "I have forgotten"; the line is a complaint about who leads | 05-S11 | replaced by T "Sie sind an der Reihe" |
| slow | Wander "Heraus mit der Ziege" | "Ziege" is a gendered jibe in modern German; obscure | 05b LEX-S20 | removed |
| cut | Mo English "Cut and see" | Not in the German | | English trimmed |
| trick-good-mine | Wander "Gestochen der Bock..." and "die geringern stechen die höhern" | Obscure, "Bock" can insult; second says the lesser beat the greater, English "big catch" | 05b LEX-S21, S13 | removed, 2 T lines added |
| trick-bad-mine | English "The better of that one is yours" | Nonsense; and the German line printed as English | | fixed |
| trick-bad-yours, result-lost-mine | Wander "Die Karte hat sich gewendet" / "Blatt hat sich gewandt" | German past tense vs English "can turn / will turn" | 05b | German now matches the English (Blatt 29) |
| nothing-mine, many | English column identical to German ("Lauter Skartindeln!", "Das Blatt hat sich gewandt.", "Es gilt ja Einem...", "Ich nehme auf!", "Weiter, bitte", "Vier Könige!", "Mit Glanz gewonnen", "Ich skisiere mich!", "Noch eine Tour", "Schluß für heute", "Der Pagat ist da", ...) | English game would speak untranslated German | | English translated; terms of art (Troule, Sküs, Mond, Touren, Tous les trois) kept |
| count-close | Pr "two Blatt" in English; Kg "two cards" | "Blatt" is a third here | 03 §4; 05c LIT-S16 | English "two thirds" |
| count-close | Wander "Spiel ... vffm Tisch" | "vffm" unreadable | 05b | removed |
| count-start | Pr "Auf die Drittel genau" | Not in Prager Tagblatt | | retagged T |
| honour-forfeited | four lines tagged Wn | German invented; "before it is played" is wrong (announce at the latest as the first of the honour is played) | 14 rule 20; 03 Werner/Ulmann | retagged T, reworded |
| both-pass, thrown-in, touren-3, tour-end, tour-last, extend-ask/-yes/-no | Ul/Be/Hs tags on invented sentences ("Einmal herum um den Tisch", "Die Kreide her!", "als Freunde", "Schluß für heute", "Noch eine Tour?" ...) | German not in the cited source; "Einmal herum" misleads (two players, Tour = 2 deals) | 13-S1, S3, S7 | retagged T or rewritten |
| extend-ask | Be "allseitiger Zustimmung" | Bermann's wording is for a table; two players need "beiderseitiger" | 13-S3 | inflected |
| tour-last | Be "Die Schluß-Touren" vs English "The last Tour" | Plural in German | | English "The closing Touren" |
| thrown-in | Ul "Die Vorhand hat die nächste Donne" | Ulmann p. 70 says this of the next deal after a counted game, not of a throw-in; "Donne" obscure | 13 note, Ulmann p. 70 | German "Der andere gibt als Nächster", T |
| settle-opp-pays | Ga "die Seufzer der Verlierenden" vs English "my sighs" | Meaning differs | 09-S14 | "und ich seufze" |
| settle-opp-pays | "The rest of the coins" | Vague, "rest" of what | | "The coins are yours" |
| settle-player-pays | Wander "Er hört auf, wenn er gewonnen hat" | German third person, English first | 05b LEX-S13 | German in first person |
| settle-even | "Ein gerader Abend" | Unidiomatic | | "ausgeglichener Abend" |
| farewell | Bl "die Ehre auf eine Partie" | Dropped "von ihnen": ungrammatical | 05b LEX-S32 | "Darf ich morgen wieder um die Ehre einer Partie bitten?" |
| farewell | Bt English "again tomorrow?" | Not in German | 05c LIT-S62 | English trimmed |
| farewell | Ba English | Added clause not in German | | trimmed |
| blunder-own | Ro "Unnütze Mühe, dieses Spiel" | Quote torn from its context (the Sküs takes the Mond) | 05d POLY-S4 | removed |
| slip-yours | Ve "I forgive" vs "wir verzeih'ns" | Person differs | | "we forgive" |
| kibitz | Zi "Nichtspieler gehören unter den Tisch" | Mannheim mock law, not Viennese; belittles onlookers | 05-S14 | removed |
| kibitz | Wander "Es ist ein Kibitz" | English adds a warning the German lacks | 05b LEX-S12 | removed |
| mond-caught-yours | Fh "Der den Mond kann stechen" | Fragment vs full-sentence English | | "Nur der Sküs kann den Mond stechen" |
| mond-caught-yours | Fl "Es fangt mir" | Ungrammatical | 05c LIT-S20 | "Wieder fangt mir ... ab" |
| valat-yours | Ra "trischakt mich mit Tarock" | Obscure verb, question in the source | 05c LIT-S8 | trimmed to "Alles spielt gegen mich!" |
| sku-played | "der höchste der Tarok" | Ungrammatical plural | | "der höchste Tarok" |
| pull-sku | Hu English left in dialect | | 05 headline 8: the application to the card is the note's inference | English translated; inference flagged here |
| SOURCES St | "Johann Strauss on the stakes" | The words are the Grazer Tagblatt's, about Strauss | 13-S13; sources/13/anno2.jsonl | citation corrected |
| SOURCES Wn | "Mondfang" and the spoken lines | Werner's Mondfang is in his Kauf / Königrufen scoring (pp. 9x), not in the Strohmann section; "Ich nehme auf/passe" sentences come from Mayr and Sedlaczek 2007 | sources/03/wb/3724659.txt lines 816, 3027 | citation text made explicit |

## Sourcing sample

Checked in the notes: 56 sourced lines (Fi, Sc, Ba, Mo, Ra, Gl, Jö, Cr, Lö, Lv, Kg, Fl, NF, Zi, Mw, Ha, Hn, Ga, St, Ul, Be, Wa x14, Ne1, Hu, Ki, Bl, Ve and others). Automated check of every sourced German against all notes: 100 lines were not a verbatim run of the note text; those were inspected by eye. Real quotations, inflected or trimmed: Fi, Sc, Ba, Mo, Ra, Gl, Jö, Cr, Lö, Lv, Kg, Fl, NF, Ro, Wa lines, Hn, Ha, Ne1 (tous les trois, vier Könige), Ki, Bl. Invented German under a source tag: fixed above.

Saved sources opened (17): Fi (q_strohmandeln.txt), Sc (schubin_vollmondzauber02), Ba (ia_11736211bsb), Gl (q_skartindel), Fg (q_pagatultimo), Fl (q_skues), NF (q_mondfang), Zi (Zigo-Taroc djvu l. 274), St (anno2.jsonl), Be (bermann1894 ocr l. 2586), Ul (ulmann1899, "Cadres"), Ha (alv/bsb_djvu), Wa (Liegen, Ziege, Kibitz), Hn (DWB_stich), Ga (b2/ws Regentage), Werner (Mondfang, 3724659), Lö (nestroy/11671776bsb). Not in saved text, image-read only (as the notes say): Ra (Ranzenhofer), Jö, Cr (confirmed in note 04, which quotes the page), Kg.

Corrected-row check: Bockerer is not in the bank (good, and "Ihr Blatt" is in the test's FLAGGED list). "Guter Misch" is Ranzenhofer 1917 p. 2, correctly "Ein guter Misch ist das halbe Spiel" (the Mond song is a separate line in the same story). "Max'": Hügel gives the name and the idiom separately; the pairing is the note's inference, now flagged in the group. Nestroy: Ne1 is Bonz 1890, Ne2 Bonz 1891, both flagged edition-dependent in SOURCES; only "Tous les trois" and "Vier Könige" and "'s is vergeb'n" are used. § 13: Ki is dated 1897 with the 1895 "Hauptparagraphen" quote, consistent with 05c.

## Per-criterion findings

- Rules: after the fixes nothing announces the ultimo, mentions bird bonuses or Kontra, claims a hidden card, or misstates the draw doubling, Touren (2 deals) or 2 kreuzer a Point. Mondfang is used only as table talk; it scores nothing in the canon (14 rule 24). "Special-Katastrophe" (NF) overstates that, but is the cited wording and is kept.
- Kindness: no slur found in either language. Removed "Ziege" and "Bock" (gendered/insulting senses in German) and the "under the table" line. "Kiebitz, halt's Maul" is kept: it addresses the tutor, is the period formula, and the test flags the other kibitzer lines.
- Register: "Sie" throughout; "wir" in invitations; no "du".
- Remaining mild issues, not changed: Löwy's dialect line in blunder-own ("waß i net", "fufzehn") is genuine but more Viennese than the rest of the bank; "Mondfang" in the Hungarian-game quote; "Troule" and "Honneur" are period spellings; some Wander lines ("Wer sich in die Karten sehen lässt") are generic German, not Austrian.

## Coverage

All required groups exist. Moments with no line: `dealt`, `cover_set`, `packet_empty`, `doubling_set` (covered by `draw`), `card_played` for ordinary cards, and the first sentence of the end of a sitting when the player concedes (no such event). A void deal has one group for either side.

Groups that will grate on repetition (high frequency, shared stock): "A good hand comes back" is in 6 groups, "Luck at cards is April weather" in 7, "Dealt one way, played another" in 5, "Easy to play when luck deals" in 5, "Good cards get used to you" in 5. The page already avoids the last wording within a group; it should also avoid repeating a line across groups within a deal, or the bank needs more T lines for `trick-*`, `luck-*` and `result-*`. `pull-king-*`, `pull-tarock-*` and `nothing-*` fire several times a deal and have only 5 wordings.

## Verdict

Usable after the fixes. Sourcing was weak in the table's invented sentences wearing source tags (about 30 lines), and several lines said something the rules or the English did not support; those are fixed. German is now consistent and readable. Open: add wordings to the high-frequency groups and cross-group repetition control.
