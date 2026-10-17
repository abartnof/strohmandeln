# Review: Kibitzer pages, rank order and the count (ab3ed2e, db4aef9, 9db3c78)

Scope: `worktree-agent-ae6f374b4675c58df` against main: `web3d/tutorial.en.md`, `web3d/tutorial.md`,
`web3d/test/tutorial.test.js`; against RULES 1.4, 1.5, 1.7, 8.1, 8.2 and TABLE3D 6.2.2.

## Run

- `node --test kit/test web3d/test` on the branch tip (wasm built in a private target dir): 541 pass, 0 fail.
  No failure that also fails on main.
- At ab3ed2e alone, `tutorial.test.js` fails 2 of 24: "en/de: no page exceeds 150 words ... no line 25".
- `python3 kit/build.py --check` on the tip: "strohmandeln.html is out of date".

## Correctness (RULES 1.4, 1.5, 1.7, 8.1, 8.2)

Both languages state: Sküs highest, XXI down to II, I (Pagat) lowest (1.4); K Q C J then 10 9 8 7 in
clubs and spades, 1 2 3 4 in hearts and diamonds with the 1 highest (1.5); values 5/4/3/2/1 and the
rank-is-not-value remark (1.7: Pagat 5); groups of three less 2, leftovers two-thirds each, 106 - 36 = 70,
35 2/3 wins and 35 1/3 does not (8.1, 8.2). No factual error found. The German court labels (K, D, C, B)
match the other German pages.

## Findings

1. **should** `thirds` / "How to count" (en and de): the page is right but the order still muddles. The
   "Card by card ..." line restates the first line in different words; "So the two takes make 70 in all"
   is a non sequitur (nothing before it says whose takes or why 70); "36 rounded; the count keeps thirds"
   (de: "geschrieben 35⅔, weil Drittel gezählt werden (gerundet 36)") reads as if 35⅔ were a rounding of
   36, the wrong way round. RULES 8.2 says 36 points means at least 35 2/3. Fix, same word count:
   - en:
     - Count your won cards in threes: add the three values, take off 2.
     - Example: a king, a jack and a seven are 5 + 2 + 1 = 8, less 2 = 6.
     - One or two cards left over lose two-thirds each.
     - Taking 2 off three cards is two-thirds off each, so the count runs in thirds.
     - Together, the two players' cards always count 70.
     - A declarer needs 36: that is **35⅔** or more. **35⅓** falls short.
   - de:
     - Zählen Sie gewonnene Karten in Dreiern: die drei Werte zusammen, 2 ab.
     - Beispiel: ein König, ein Bube und eine Sieben sind 5 + 2 + 1 = 8, weniger 2 = 6.
     - Bleiben eine oder zwei Karten übrig, verliert jede zwei Drittel.
     - 2 ab von dreien sind zwei Drittel ab von jeder: darum zählt man in Dritteln.
     - Beide Spieler zusammen machen immer 70 Augen.
     - Der Aufnehmer braucht 36 Augen: das sind **35⅔** oder mehr. **35⅓** reicht nicht.
   Then drop the "(nicht|knapp)" and `/70/` pins in favour of the order check below (finding 5).
2. **should** `the-cards` dropped content that TABLE3D 6.2.2 (line 835) and research/15 "For the tutor"
   (items 4 and 5, with the maintainer's 2026-10-15 decision) still specify: the Tarock/Tarot word
   distinction, "the 52-card pack dropped him", "corner numbers", and in German "In Österreich"
   (now "Man spielt noch Tarock"; English keeps "Austrians"), so en and de are no longer parallel. The
   assertions for them were deleted only to fit the caps, which is not a reason to change a decision. Fix:
   either restore them in the page (the cap is 150 words and the new rank lines cost about 70; the
   Tarock/Tarot line and the 52-card clause are the cheapest to move to a second page) or record the
   removal as a decision in TABLE3D 6.2.2 and research/15 with the maintainer's agreement. Restore
   "Österreich" in the German line either way.
3. **should** `web3d/strohmandeln.html` embeds the Markdown (kit/build.py inlines it) and is out of date
   on the branch: run `python3 kit/build.py` and commit it, as its own commit. The gate's
   `kit/build.py --check` fails until then.
4. **should** TABLE3D 6.2.2's `the-cards` row (line 835) describes the old page ("the tarocks are trumps I
   to XXI and the Sküs", "the C ... a knight worth 3 points") and says nothing of the rank order, the
   suits or rank-not-value; the `thirds`/`scoring-full` rows, if present, need the same. Update the row
   with the page (or finding 2's decision). Also the page heading "The C" became "The suits"; no check
   or doc refers to the old heading (grep).
5. **nit** Tests: the rank and order tests are regex pins on wording; the order test
   (group, example, leftovers, 70, 35⅔) is the useful one, but `/70/` can match any earlier line. Anchor
   it to the line that says "always count 70" / "machen immer 70". One cheap non-wording check: derive 106
   and 70 from the values the page lists (5x7 + 4x4 + 3x4 + 2x4 + 1x35 = 106) rather than only matching the
   string. Indentation in the-cards test is broken in six lines (4 spaces where 2 are meant).
6. **nit** German mixes "Tarocke" (new lines) and "Tarocks" (existing scoring-full lines) within one
   page; pick one (the heading uses "Die Tarocke"). Likewise "Dreiern" (thirds) vs "Dreiergruppen"
   (scoring-full) is fine (colloquial vs full) but note it.
7. **nit** "Rank is not value" on `the-cards` comes before the page has explained value (5 for Pagat,
   Sküs, XXI); a beginner meets "worth 5" cold. One clause fixes it: "...yet worth 5 Augen/points,
   the most a card is worth, like the Sküs and XXI."

## Commits

The first commit fails the caps tests; the next two fix them. Squash all three into one (the second and
third are only layout and wording of the first), then add finding 1 to the same commit or a following
one, and the rebuilt page as its own commit.

## Fixes

| Finding | Fixed in |
|---|---|
| 1 how to count | Kibitzer pages commit (reviewer's wording; last line "To win you need 36: that is 35⅔ or more" for the plain game too) |
| 2 dropped lines | Kibitzer pages commit: 52-card clause and "In Österreich" restored; the Tarock/Tarot note and corner numbers stay off (decision 17 Oct 2026, recorded in TABLE3D 6.2.2 and research/15) |
| 3 stale html | Rebuild commit (`kit/build.py --check` passes) |
| 4 TABLE3D row | Docs commit |
| 5 tests | Kibitzer pages commit: 70 anchored to the "always count 70" line, 106 derived from the value rows, indents fixed |
| 6 Tarocke/Tarocks | Kibitzer pages commit: "Tarocks" throughout |
| 7 worth 5 | Kibitzer pages commit: "worth 5 points, the most a card is worth" |
