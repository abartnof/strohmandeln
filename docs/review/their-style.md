# Review: "Their style" (branch worktree-agent-a51195fd33d220306)

Scope: `tell.rs`, `brief_of_record`'s `earlier` header, the table's wiring
(`briefing.js`, `review.js`, `tutorwire.js`), strings, docs. Checked against
`docs/STYLES.md` 5 and 9, `docs/TUTOR.md` 6, `docs/PROTOCOL.md`.

Run: `cargo test -p strohmandeln-ai tell` and `-p strohmandeln-wasm their_style`
pass; `node --test web3d/test/theirstyle.test.js web3d/test/review.test.js` 17/17;
`python3 kit/build.py --check` clean. `browser.py --only=deal_brief` fails with
the 30 s timeout on `[data-dialog=review] li` (the known seed drift on main; not
re-run on main here, so unconfirmed that the failure is identical).

## Findings

1. **No leak (checked).** `Style::POOL` is Bermann, MS, Bold, Cautious; Random
   never draws Normal, so presence of the line under Random says only "not
   Normal", which the player already knows. The text is one of three count
   keys, no id; the Rust test asserts the Random line is identical before and
   after `reveal`. `earlier` carries whole `DealRecord`s (deck, so the
   opponent's hand), but they are the human's own finished deals, already
   shown in review, and are never echoed in the reply. Only Take/Pass of the
   opponent's seat are read. None.

2. **Statistic (checked).** Normal-approximation z of the take count at p = 0.19;
   `bids >= 3` keeps the denominator off zero. Behaviour: Bold at 2 of 3 shows;
   Bermann/MS/Cautious show "has not taken up" from 6 bids (0 of 6, z -1.19);
   "only once" is reachable only from about 14 bids. A threshold of 1 sd is
   loose (a true Bold sits under it on roughly a quarter of looks and a styled
   opponent can show a count that points the wrong way early), acceptable for a
   fact line. Low, no fix; note only.

3. **"Bids" wording (checked, low).** Every chance to speak is a Take or a Pass,
   so "bids" and "chances" are the same count. The dealer speaks only when
   forehand has passed (`RULES.md` 3.2), so as dealer the opponent's bids are
   conditioned on the player's pass; stage 0's 19% is per bid, counted the same
   way, so the comparison holds. "Taken up 4 of 6 bids" is accurate; German
   "Geboten aufgenommen" is the game's term ("ich nehme auf"). Optional: say
   "of its 6 bids" so no one reads it as deals. No fix required.

4. **Budget clause (medium, docs).** The engine and table honour the new item 4,
   but the places that state the cap were not updated: `docs/TUTOR.md` line 645
   (acceptance: "At most three bullets, at most 45 words") and line 678 (the
   brief's shape), `docs/TABLE3D.md` lines 970 and 1474 ("at most three bullets").
   They now contradict item 4 read literally. Fix: add "and, for a styled
   opponent, the fourth line of item 4" to those four lines (or "three
   bullets of advice"). `brief_tests` has no check that the three-bullet
   advice plus fourth line stays within the 62-word cap; the 14-word cap of the
   line itself is only in prose. Add a one-line test over the three keys' English
   text (longest: took-once, 10 words at most), so the claim has a check.

5. **Wire (checked).** Records are at most `MAX_RECORD` 20,000 characters (about
   2,000 in practice) and the store keeps `MAX_DEALS` 60, so a 6-Tour sitting
   (72 records, capped at 60) sends about 120 KB at most, once per uncached
   brief and only for a styled opponent. The 1 MB limit in TABLE3D is the import
   file, not this call. Parsing 60 records in wasm is negligible. `earlier` is
   documented in `PROTOCOL.md`; the `bad-input` path is tested. The brief cache is
   keyed by record alone, fine as style is fixed per sitting. Deals not kept
   (unscored, beyond the cap of 60, no `sit`) are not counted, so the line can
   undercount; harmless, but `PROTOCOL.md` could say "the kept deals".

6. **Tests and docs.** Meaningful: header order, Normal/no-style/first-deal
   absence, bad record, Random sameness, four lines shown, tips off keeps only
   "well". The Rust wiring test repeats one record six times (shows the plumbing,
   not a realistic sitting; the boundary cases live in `tell.rs`). Docs are terse;
   the STYLES paragraph carries "Built (17 Oct 2026)" as a dated decision, as
   wanted. One long unwrapped line in `PROTOCOL.md` (the `style` sentence).

## Verdict

No blocking defect. Fix item 4 (four doc lines, and a word-cap test) before the
work package is called done; items 2, 3, 5 are notes.
