# Review: talk.js and the phrase bank

Branch worktree-agent-a7f9b6a5b1d2810fa against main. Checked: `web3d/src/talk.js` and its test
against `docs/TABLE3D.md` 5 and `docs/PROTOCOL.md`/`docs/RULES.md` 16; the bank changes in
`tools/phrases.py`, `web3d/words.json`, `docs/PHRASES.md`.

## Talk

Event-to-group mapping, the repeat window (3 deals, this one included), calls exempt, the three
modes, the nothing-once rule, run replacing the third pull, and determinism under a seeded rng
all match 5.1 to 5.4. Credits are ordered game first, so `credits[0]` is the winner; `count`
is always emitted after trick 27, so the luck take is read from the right deal.

Fixed:
- `count-close` fired within one third of 107, but its lines name "35 and two thirds"
  (106 or 108 made them false). Now exact threshold only.
- A count that never closed left the table quiet for the next deal, and `lastCount` carried over.
  A new deal resets both.

Open:
- `game_scored` arriving while quiet (no `count_close` sent) is dropped. The table must send
  `count_close` before `game_scored` (5.2.3); a test of the table should cover it.
- A run at the third pull replaces a Mond, Sküs, Pagat or king line (5.1.2 says the run's last
  card; the third pull is the nearest the talk can know).
- `count-close` is rare now (exactly 107); a wider close line set without the figure would
  need new lines.

## Phrase bank: citations traced in `research/sources/`

| Sig | Wording | Found in |
|---|---|---|
| Ne5 | "Beim Tarock kann ich doch 'was g'winnen, wenn ich einige Ultimo mach'" | `05/lit/nestroy/bub_gb_g4gtAAAAYAAJ.txt` l. 3648 (Unverhofft, Ledig) |
| Id | "Den hab' i an Trumpf geben" | `05/lex/Wander_Trumpf_T00755.txt`, Trumpf 5, "Idiot. Austr., 65" (Idioticon itself not read) |
| Sk | "Auf jemanden einen Pick haben ... Haß haben" | `05/dict/schranka1905_djvu.txt` l. 5190 |
| Kk | "Der Sküs sticht immer Alles" | `05/lit/anno/q_skues.txt` l. 95 (OCR "Alke»") |
| Gs | "der Mond nur, wann er in Gsell-schaft vom Gstieß und Pagat is" | `05/lit/anno/q_gstiess.txt` (OCR "interessiert der Mond nur") |
| Ba | "Ein andrer leg sich als ein Block ins weiche Bett: ich spiel Tarock" | `05/poly/ia_11251943bsb_djvu.txt` l. 10185 |
| Mo | "Jeder Karten geben will" | `05/poly/ia_00057104bsb_djvu.txt` l. 2857 |
| Wn | "so lange man nicht weiß, wo der Skis steht" | `05/web/wienbib_3724659.txt` l. 1260 |
| Ne1 | "mein Herr hat halt überall 's Glück in Pacht" | `05/lit/nestroy/11671772bsb.txt` l. 3112 |

Also traced: Wander Spielen 72 and 22, Spiel 42, Herz 239 and 197, Bock, Karte 44, Glück 850,
Blatt 53, Klein 14 (`Wander_kartenspiel_items.txt`), Hund 301; Ha, Ul, Ra, Jö, Fi, Kz, Mw, Gl,
Ro, Mu in the notes (05c, 05d, 03) and, where downloaded, the texts. No fabricated citation found.

Fixed:
- Gs German said "zählt"; the source says "interessiert". Restored (test added).
- `Wa Karte 44` had the English "The leaf can turn yet." under German "has turned". Now "The card has turned."
- `Hund` locator made `Hund 301` (the entry number in the Wander article).
- T line "Ein Stich verloren ist nicht das Spiel verloren" reworded to idiomatic German.

Notes (no change):
- Fh, Ha, Gs, Mo, Wn lines are trimmed or inflected from the source, as the PHRASES.md header
  says; Fh's source is a verse ("Bloß die Sünde ist der Sküs, der den Mond kann stechen").
- Kk and Gs are OCR snippets; the Kikeriki word "Alles" is reconstructed from "Alke»".
- The six new T lines ("Das Stroh war heut sparsam mit Ihnen", "Das Spiel geht an mich; die
  Tafel wird es sagen", "Das Spiel geht an Sie, und ehrlich gewonnen", "Diesmal meines; der Abend
  ist lang", "Die Karten fallen für Sie; viel Freude damit", "Ein verlorener Stich ist noch kein
  verlorenes Spiel") are marked T, are plain and idiomatic, and are about luck, the straw, the
  score or the run of play. Kindness test passes.
- Open: "Mit einem guten Herzen gewinnt man kein Spiel" (Wa Herz 239) in `trick-bad-yours` could
  read as a remark on the person's character; it is a Wander proverb about the game. Maintainer
  to say if it stays.
- Different English renderings of one sig in two groups are deliberate (adapted to the moment).
