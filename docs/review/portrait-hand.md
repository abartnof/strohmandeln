# Review: phone portrait hand (10ce74f, ab3c8e0, 73dc628)

Checked: layout.js (`handShape`, `rowLift`, `LAYER`), zones.js, main.js, tutor.css, tests, screenshots (before2/after2, portrait and phone landscape). `node --test web3d/test/` passes (228).

## Findings

- Fixed: `clampScroll` in main.js called `handRange` without `fill`, so the scroll range disagreed with the layout when `fill` > 1. Now passes `view().fill`.
- OK: `LAYER` accumulates at most 8 x 0.06 = 0.48 cm across a row, 0.5 cm between rows, against an eye 96 cm away. Not visible as parallax. Later cards sit nearer the eye, matching the overlap order (each card covers the previous one's right edge), so nearest-hit picking and the drag hit-test choose the visible card. The selection lift moves along the card's own up and is independent of depth.
- OK: the first half is the back, upper row; the front row is nearer by 0.5 cm. Sort order reads suits at the back, tarocks in front (after2-port.png).
- OK: choreography reads poses per card key from `layout`, so deal, play from hand and a card arriving from a packet interpolate to their slots; rows re-split when the count changes, and cards glide between rows. `fill` reaches choreography via `layoutOf`.
- OK: desktop unaffected. `git diff main...` does not touch the desktop `ZONES`; the two-dot diff shows the old desktop fan only because main moved on (main has 3.6 / 62). The merge takes main's value.
- OK: tests check corners inside the window and indices uncovered at three phone sizes plus the touch row; `eye.test.js` measures `EYE_PORTRAIT.hand`. TABLE3D 1.3 and section 10 are in step.

## Open

- Merge into main conflicts in `web3d/test/layout.test.js` (main changed its imports: `JUMBO`, `spacingFor`) and in the built page; resolve by keeping both import sets and rebuilding with `python3 kit/build.py`.
