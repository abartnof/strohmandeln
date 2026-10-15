# Review: strings, prefs, settings, slate, persist, dom, table.css

Branch worktree-agent-a03b82705ffe395ce, against docs/TABLE3D.md 4, 7, 8, 9,
docs/PROTOCOL.md, docs/MESSAGES.md, docs/RULES.md 14-15. Reviewer did not write
the code.

## Source check (settings notes)

- `settings.both_pass.note`: [03-S14] is Ulmann (research/03 source list: "S14
  Ulmann, Illustrirtes Wiener Tarockbuch, 2nd ed."). The quoted clause is in his
  text ("entscheidet er sich fürs Passen, so wird zusammengeworfen, und die
  Vorhand hat die nächste Donne"), so the attribution of throwing in is correct.
  Incomplete: research/14 rule 7 also lists Beck and Bamberger (and Bermann
  describes both). Fixed: "Ulmann, Beck". Play-on (Werner, Mayr and Sedlaczek)
  matches rule 7 (also Loew, Piatnik, Furr, not named).
- `settings.values.note`: 2/3/4 Werner, Mayr and Sedlaczek; 1/2/3 Dummett, Beck;
  1/2/4 Ulmann match research/14 rule 19 and research/02. Correct.
- `settings.ultimo.note` names no source (TABLE3D 7.2 asks for "sourced as in the
  rules"). Open, minor: Werner 1, Ulmann "in manchen Kreisen" 2 (research/14 row 21).

## Fixed

| # | Finding | Commit |
|---|---|---|
| 1 | persist: the 200 KB and 1 MB caps used `text.length` (UTF-16 units), not bytes; German text passed a cap it exceeded | persist |
| 2 | slate: a reload replays the deal in progress's events; a finished deal's `deal_start` pushed a duplicate row, an accepted `extension_decided` counted twice | slate |
| 3 | slate: a saved ledger (storage or file) was trusted if `v` matched; a damaged one could throw in `render`. Now shape-checked, a bad one starts blank | slate |
| 4 | slate: "Geber 1" was shown only when the human dealt first, in the margin; 4.3.1 marks the first dealer's column, either player | slate |
| 5 | strings: `slate.owed` de "Bezahlt" (paid) for what is owed; now "Zu zahlen" | strings |
| 6 | strings: the declarer in German was "Ich nehme auf: ..." (wrong when the opponent declares) and "Spieler"; the books say "Aufnehmer" [03-S14] | strings |
| 7 | strings: "geborgt" -> "entlehnt"; "ausgeben" -> "herunterladen" | strings |

German checked for formal Sie, period wording and error: no du-forms; "Kiebitz,
halt's Maul!" is cited (docs/PHRASES.md Ki). English strings: no errors found.

## Open

- persist: a save refused as too big leaves the previous save in place, so
  Continue would offer an older state. 9.1 says only "continues unsaved".
- persist: a quota error on `setItem` reports "unavailable", not "too-big".
- prefs: `seen` (tutorial pages) is kept inside `strohmandeln.prefs`; 7.7.1 names
  a separate `strohmandeln.seen` key. Harmless; decide with the tutor module.
- prefs: 7.7.4 says URL overrides are ignored in a real sitting unless a test
  flag is set; `withUrl` and `urlOverrides` apply them always. The page decides
  when to call them.
- slate/css: the folded-by-itself (non-large) slate has `pointer-events: none` on
  its backdrop, so "a tap outside folds it" (4.1.1) works only for the large
  sheet. Needs the table's layout to settle.
- slate: the engine's `both_pass` event field is `mode`; the slate does not use it.
  No change needed.

## Verified without change

Slate credits, Contos, balance sign, `kreuzer` at 2 a Point (matches
`sitting_end.kreuzer`, RULES 15.8), payer mapping, square ("Quitt."), void deal
not a row, throw-in a row (15.3); prefs versioning (newer store read as empty
and not overwritten, bad fields default one by one, no storage -> defaults);
`sittingConfig` matches the `new` argument of PROTOCOL; persist's export check
matches the `strohmandeln_export` shape.
