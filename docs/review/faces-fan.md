# Review: Large Text faces, wider desktop fan, public-card dot

Branch `worktree-agent-a63284c38048bfd24` against main (3 commits).

## Checked
- Large Text: numbered suit cards share the court path (big rank, big suit, right
  of the strip); Classic keeps pips. Tests cover both (strip clearance, sizes,
  suit-glyph counts). Correct.
- Fan: spacing 2.9 to 3.6, width 46 to 62. Tests assert the visible strip is at
  least the index strip in every arrangement, touch/phone pitch unchanged, and
  the 18-card half-width is inside `camera.widthTan * distance`. The check is on
  width, so it holds at any desktop aspect (1280x720, 1366x768, 1920x1080 share
  16:9). After-land screenshot (1280x720): fan spans x 362-922, lowest card
  y~630, clear of the bid buttons (y 665+) and the HUD/Slate/Kibitzer.
- Public mark: 10x10 ringed dot, offset adjusted in main.js; sits on the card
  corner, readable on gold tarocks and on backs.
- Tests: layout and tarockfaces suites pass (27/27).

## Fixed
- `docs/TABLE3D.md` 1.5.7 still described the mark as a thin outline on the lower
  edge, and 7.6 did not mention Large Text numbered faces or the wider fan.

## Open
- None. No 1366x768 shot taken: same aspect as 1280x720 and the fit test is
  width-based. The portrait screenshot (unchanged arrangement) shows an unsettled
  mid-animation hand; not part of this change.
