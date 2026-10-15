# Review: table gaps (1535ccb..8116071)

Scope: mirror.js, gesture.js, newsitting.js, speaker.js, the kit `apart`
category, bonus popups, the edge, trick 27, valat fan, face sizes, cover tilt,
main.js wiring. Checked against TABLE3D 3.10.1, 3.10.3, 3.1.2, 3.1.3, 2.5.3-2.5.5,
2.3.1, 2.4, 7.1, 7.6, 3.5.1, 5.2.2, 1.5.2 and RULES 10.

## Fixed

1. gesture.js: a second shake begun mid-shake took the displaced x as its rest,
   so a card tapped twice drifted for good. Rest is now kept per mesh and the
   older run yields. Test: gesture.test.js.
2. mirror.js: a focus index past the end of a list that had shrunk (a card
   played) gave every button tabindex -1, so the list was no Tab stop; focus
   restore also missed. Index clamped. Test: mirror.test.js.
3. main.js: the language switch rebuilds the HUD, dropping the `edge` class
   (2.5.4 tick). Re-applied from the kept totals. A `pointercancel` now ends a
   drag. Test: page.test.js (source check; main.js is not unit-loadable).

## Checked, no finding

- Bonuses: `apart` keeps them out of totals, segments, subtotals and the edge
  (`edgeOf(hud.totals())`); 106 chips only on exactly 106; valat's credit is a
  bonus line of 12 Points, the count event still follows a valat.
- Kit change is generic (`apart` is named by the game; no Strohmandeln logic in
  kit/js). The kit README example names Strohmandeln's bonuses as an example.
- Drag play: the click after a drag is swallowed (`afterDrag`); `pending` and
  `busy` guards stop a second play; a scroll is not a play.
- Mirror: Enter/Space prevented on keydown so no click follows; the window
  handler yields to defaultPrevented; dimmed cards are never the default stop.
- Speaker: epoch guards drop timers of an old sitting; `resetTable` calls
  `stop()` and `ui.hush()` (the boxes). A language switch leaves lines already
  chosen in their language.
- New-sitting screen applies rule keys only on Deal; Cancel changes nothing.
- The cut's two faces are cards of the human's own hand (dealt 13, 14): no leak.
- Cover tilt lifts the far edge about the card's centre, near edge stays on the
  stack; same for the opponent's covers.

## Open

- After a language switch the HUD's ledger list holds the count only; bonus
  lines already popped are not rebuilt (totals are unaffected).
- The mirror's `inside` flag can stay true if the focused button is removed by
  a redraw and no card matches the focus; the next Tab recovers.
- Focus left on a mirror card after Tab away keeps the card highlighted
  (`focus` is not cleared on focusout).
- The browser test was not run (owned by another agent); the gate's --quick was.
