# Review: hint reasons on the table (2b37862)

Scope: web3d/src/hinttext.js, ui.js setAid, main.js askHint, style.css, strings, web3d/test/hinttext.test.js.

## Fixed
- Card names: the hint named tarocks "tarock 12" while the faces show Roman numerals. Shown hints now use `romanCardName` ("tarock XII", IIII for 4; Pagat, Mond, Sküs by name; courts and pips unchanged, German court names via `card.rank.*`). The screen-reader text keeps the plain `cardName` (Arabic reads better aloud). Test added.
- A zero-thirds parameter read "0 point"; now "0 points".
- On a phone the 2-3 line hint grew upward under the Kibitzer chips (z 15) and the first words were unreadable; `.aid-line` is now above them (z 16, 0.95 background).

## Checked, fine
- en and de say the same; `_thirds` units and fractions ("⅔ point", "4⅔ points", "⅓ Auge") correct; each hint key has exactly its MESSAGES.md parameters (test).
- Desktop 1280x800: two lines sit between the prompt and the buttons, clear of the fan. Phone 390x844: no horizontal overflow, text wraps within the 12px gutters.
- The hint clears on every state render (main.js `ui.setAid(null)`), so a move or new trick removes it. The live region is written once per hint request, never on a timer.

## Open
- Phone portrait: a hint over a hand with the Announce row shown covers the lower part of the hand fan (the suggested card's highlight edge); no room above without moving the hand.
- Bid hints say "take up the 4" and "sure winners" without explaining either; a glossary link or a short gloss belongs in MESSAGES.md wording (not edited here).
- The tutor review and `explainText` still name tarocks "tarock 12"; unify with `romanCardName` (promote it to strings.js) when those are reviewed.
- Mid-hint language switch leaves the old-language text until the next state render.
