# Review: release documents

Scope: `README.md`, `CREDITS.md`, `LICENSE`, `index.html`, `.nojekyll`,
`tools/credits_check.py`, `tools/test_credits.py`, the credits step of `bin/gate`.

## Findings, fixed

- README: "land with their work packages" for `docs/TABLE3D.md` and
  `docs/OPPONENT.md`, which exist. Removed.
- README: no mention of `strohmandeln-ai`, the browser test or how to run it.
  Added (crate row, opponent bullet, a Testing paragraph with the commands).
- README: the intro described the straw man as a single face-down hand for a
  missing third player and gave "1820s to the 1930s". The review (1.1, 2.2, 2.3)
  gives three face-down packets of four per player, the name in print from 1824,
  press attestation to 1938. Reworded to those sections.
- CREDITS: sources used by the phrase bank but not named: *Idioticon
  Austriacum* (1824), Schranka (1905), *Das kleine Volksblatt* (1934), Nestroy
  *Unverhofft* (1845), *Kikeriki* 1887. Added, with details as in
  `docs/PHRASES.md` and `research/05*`.
- CREDITS: Playwright with Chromium and Pillow are used by
  `web3d/test/browser.py` (gate runs `--quick`); only Playwright/WebKit for the
  contact sheet was listed. Added.
- CREDITS: Art and fonts section now says the table's text asks for "Chalk"
  and falls back to system fonts (`web3d/src/table.css`); nothing embedded.
- CREDITS: Rust dependencies stated per crate (`strohmandeln-ai` and
  `-cli` have none outside the repository).
- `credits_check.py`: `[workspace.dependencies]` path entries were counted as
  third-party. Fixed, with tests.
- `test_credits.py`: new tests that the real repo is scanned (all crates) and
  that every source key of `tools/phrases.py` is named in CREDITS.md.

## Checked, no change

- Built page: no `@font-face`, `url(`, image, texture loader or network URL in
  `web3d/src`, `kit/js`, `kit/css`; one `data:,` favicon. The only bundled
  library is three.js 0.186.1 (licence comment in the page); matches
  `package.json`. Cargo.lock packages match the transitive list in CREDITS.md.
- `LICENSE` is identical to `~/cassino/LICENSE`. `.nojekyll` present, as in
  piquet. `index.html` redirects to `web3d/strohmandeln.html` (relative, with
  query and hash kept), as piquet's.
- The commissioner is named only in the `LICENSE` copyright line. The GitHub
  user name appears in the Pages address (README, `index.html`) and in
  `research/10-prior-games-survey.md` repository addresses of the earlier
  games.

## Open

- `docs/PHRASES.md` (generated from `tools/phrases.py`) spells Ulmann's title
  "Tarockbuch"; `research/03-period-rulebooks.md` [03-S14] has "Tarokbuch".
  CREDITS follows PHRASES; the bank's spelling should be checked against the
  title page.
- `research/13` [13-S2] titles Werner "Das Tarokspiel. Eine Anleitung..."
  while [03-S12] has "Das moderne Tarokspiel"; the two notes should agree.
- Werner is dated "by 1891" (undated second edition).
- `kit/STAMP` and research notes contain the development machine's home path.
- Playwright and Pillow versions are unpinned; no lockfile exists for them.
