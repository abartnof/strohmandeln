# Review: table interaction (tap twice, HUD, tips labels, hand order)

Scope: `git diff main...HEAD` without the built page. Rules: docs/TABLE3D.md 3.1.2, 3.10, 6.1.

## Fixed

- Scroll leaves a stale spot. After a drag-scroll or wheel of the hand, or a resize, the cards
  move under `sel.at`, so a click at the old spot played the chosen card although another card
  lay there. `at` is now cleared on drag-scroll, wheel and resize (`strings.test.js` checks the
  wiring).
- No announced way to play. The mirror buttons carry `aria-pressed` but a choice said nothing, so a
  screen-reader user did not learn that a second press plays. New `sr.chosen` (en, de) goes to the
  live region on a tap or a key choice.
- Mute button name. `aria-label` was the long tooltip (not starting with the visible label) and
  `aria-pressed` sat beside a label that already flips ("Mute tips" / "Tips on"). The name is now
  the visible label and `aria-pressed` is dropped.

## Checked, no change

- Keyboard choice then click: `at` is null, so only a click on the card itself plays it.
- Touch: `click` carries the tap point; a second tap on the lifted card hits it directly, the
  10 px rule only covers the foot. A finger may drift more than 10 px: then the tap lands on a
  neighbour and moves the choice (safe failure, never a wrong play).
- Legal cards changing: every `advance` clears `chosen`; `at` is read only while `chosen` is set.
- Click elsewhere (beyond 10 px) still deselects; Escape too.
- Pre-lift footprint hit-test rejected: the foot of the chosen card is the visible strip of its
  neighbour, so a deliberate click on that neighbour would play the wrong card. The 10 px radius
  errs only in a dense fan (neighbour closer than 10 px).
- Hand sort key: Sku 1000, XXI 1001 ... I 1021, suits below 400; the model compares through
  `sortCodes` on both sides. HUD `hud.hand` has no slot; tips strings en/de agree; TABLE3D in step.

## Open

- A deliberate click on a neighbour less than 10 px from the first click plays the chosen card
  (dense fans on narrow screens only).
