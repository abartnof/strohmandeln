# Review: styles engine (branch worktree-agent-acca58f29def9209c, cc81046..185ff5c), 17 Oct 2026

Reviewer: an agent that did not write it. Against `docs/STYLES.md`,
`docs/review/styles-spec.md`, `docs/PROTOCOL.md`, the sources
(`research/sources/03/abo/{bermann1894,ulmann1899}/ocr.txt`). Merge base
7ccc26a (the crates are identical to main's 0a67fd5). Severity: must, should,
nit.

Run: `cargo test -p strohmandeln-ai --lib -- style opponent probe golden` (45
pass), `cargo test -p strohmandeln-wasm -- style golden brief` (17 pass),
`cargo clippy -p strohmandeln-ai -p strohmandeln-wasm --all-targets -D
warnings` clean.

## 1. Normal is unchanged: confirmed

- The ai golden (`golden_tests.rs`) added to main's crate (merge base, no other
  change) passes with the recorded hashes at levels 2, 3, 4 (seeds 8, 3, 2).
  The hashes were recorded before the refactor.
- `decision_rng` keeps the retired slip's spare draw, so streams are the same.
- Wasm `tests/golden.txt`: two re-records, both explained. 30cb7cf (level 1
  changed meaning): all six hashes and results change (events 861 to 862, Contos,
  balance). 95a6967 (state gained `style`, `drawn`): hashes only; events, Contos
  and balance identical. The wasm golden plays level 1 only, so it does not
  guard levels 2-4; the ai golden does.
- Level 4's tolerance band is `wt.band` = 0.34 for Normal, as before.

## 2. Style parameters against the text

Tags and passages match STYLES 2.1/2.2 (T/I/E); temperaments are E and say so.
Findings:

2.1 **should. Ulmann's "4 or more honours" counts a short Pagat.**
`honours()` counts Pagat, Sküs, Mond and kings. U 78: "der kurz besetzte Pagat
[hat] nicht den Werth eines anderen Honneurs". Sküs, Mond, Pagat and one king
with 3 tarocks bids at once, against the text. Fix: count the Pagat only with
`pagat_guard` tarocks (6). Werner's "3 honours" uses the same function and
should read the same way (check W 131).

2.2 **should. The dealer clause (deviation 1) is dead code.** Three honours
without a queen are at most 3 x 13 = 39 thirds, under Normal's 45 that the
fall-through applies, so `dealer && n == 3 && !queen` never changes a result
(the test says so in a comment). U 78 says "namentlich in der Rückhand": not
only the dealer, so the text is already met by Normal's threshold. Fix: delete
the clause and the `dealer` parameter of `bid_as` (and `k.me != k.forehand` in
`bid`), keep a comment saying why; rename the test.

2.3 **nit. Cash-mond (deviation 4): defensible.** U 74 "suche man rechtzeitig
heimzubringen" and Be 152 "stets bei passender Gelegenheit einstechen" (read
with "Noch räthlicher ... beim Pagat, der sonst ... oft verloren geht": the own
card, lost at the end) support playing an own unescorted Mond. "A fitting
chance = a suit trick led with thirds >= 7 (Cavall or better)" is the author's
step (I). The tag in `style.rs` is "T" for the maxim; the trigger is I. Say so.

2.4 **nit. Spare-lead: highest spare, defensible; the example contradicts the
order.** Be 151 leads "Zwei" with Zwei and Vier spare (the higher). Generalising
to "the highest spare of strength <= 3" is I, not T. Be's example gives B the
Aß, and Be (p. 145, "sticht die arithmetisch niederere Karte") says in red suits the Aß beats the Zwei, so B need not take
with the Dame; the position is as stated in the text but its inference fails
under RULES 1.5. The code ignores the pips of the opponent. The test name says
"lowest spare card"; the code and doc say highest. Fix: rename the test; note
the text's inconsistency in research/16 s7.

2.5 **nit. Small tarock = II-X** (strength 1..=9): no text numbers it; U 74
"kleinsten, eventuell zwei kleiner Taroks" says the smallest one or two. As a
floor of one or two cards held back from II-X it plays like that text. I; fine.

2.6 **should. `guard_mond` is tagged T for Bermann, but Be 151 states the
reserve generally** ("immer von Vortheil, sich eine oder mehrere Tarok zu
behalten", to force the opponent to trump and to reach the rear hand), not for
a hand with the Mond. Gating it on "holding the Mond without the Sküs" is MS
"Mondfang" (defence). For Bermann it is I (narrowed). `guard_keep` 1 for Be
("eine oder mehrere") and 2 for MS ("ein oder zwei") are defensible; U 74
"kleinsten, eventuell zwei" supports either. Fix: tag Be's `guard_mond` I and
say the narrowing.

2.7 **nit. Ulmann's hunt (`draw_from` 10) ignores "no Pagat" and "closed
run"** that U 74 puts as conditions. Normal's `solid_run` and a held Pagat
partly cover them; say it in the comment.

## 3. Skill dial

- Level 1: `at_level_one` keeps `burden` only, `stable_ties` true, bid 60/0
  unless the config has a bid of its own. A style's maxims are zeroed (weights
  replaced), so at level 1 a style shows in the bid only, as STYLES 3 says. The
  check "same card under five seeds in >= 95% of 1 000 views" is a real test
  and passes. No other draw in rung 2's play for level 1.
- 3.5: `Opponent::budget()` caps at 3 worlds, 8 000 nodes; a smaller caller
  budget binds (tested). `bid_worlds` is not capped (so 3.5 bids as 4 does);
  STYLES 1.1 does not say otherwise. Note: 3.5 now means a different thing
  from the old "rung 3 or 4 by a coin": saved sittings at 3.5 continue under
  the new meaning (documented in OPPONENT.md).
- Snapping: ties down, 1.5 to 1, 2.5 to 2, 3.25 to 3, 3.75 to 3.5; tested to
  the edges including NaN and infinities.
- **nit.** `Level::rungs()` returns `(rung, None)` and is read only by
  `hint.rs:163` (`let (rung, _) = ...`). Nothing reads a slip. Fix: use
  `rung()` there and delete `rungs()` and its test line. `rung_for` is gone;
  `grep` finds no other reader.
- Tournament tests: `level_one_beats_random_play` (random now a local
  baseline, `mean > 5`, stop by look 2) and `level_two_beats_level_one`
  (`mean > 0`, cleared by OBF) are meaningful, if loose; the "+18.36" row is
  relabelled in OPPONENT.md. The level 1 and 2 results are not measured at
  length yet (docs say so).

## 4. Protocol

- Random: `Style::drawn(seed)` = FNV of seed and `style`, high bits folded in,
  modulo the pool (7); tested for spread. `new` resolves it; import uses a
  saved `drawn` and the hash only when absent. Authoritative on import: yes.
- `drawn` outside the pool, or with a non-random style, is `bad-record` with
  `field: "drawn"`; an unknown `style` is `bad-record`/`field: "style"` on
  import and `bad-config`/`field: "style"` on `new`. Consistent with
  PROTOCOL.md (which says `bad-config` for `new`). **nit:** STYLES 6 still says
  `bad-input` for `new`; update STYLES 6.
- Old records and old briefs without `style` are Normal (tested).
- Hiding: state shows `drawn` only when `style == Random` and the sitting is
  over (tested through a whole sitting). Brief calls echo `drawn` only with
  `reveal`, whatever the order of the header lines (tested; a text with `drawn
  werner` and no reveal does not contain "werner"). `grep drawn` over the wasm
  crate: the only outputs are state (gated), brief `style` (gated), and
  `export`, which carries `drawn` always, by design (STYLES 2.3, PROTOCOL.md).
  No error reply echoes a value, only the field name. Open (table's job): the
  export of a Random sitting is a file the player can read; a note in the
  tutorial/menu text, not an engine matter.
- Brief header: `drawn` without `style random` is `bad-input`; unknown style is
  `bad-input`; loop reads lines until one is neither (a record's first line is
  not `style `/`drawn `/`reveal`: checked against the record format).
- **nit.** The Random style also needs the `Style::POOL` fixed while a saved
  Random sitting lacks `drawn`: handled (hash), but a pool change between an
  old export and a new build changes the opponent; only exports from 185ff5c on
  carry `drawn`, so no old record is affected.

## 5. Probe (stage 0)

Measures what STYLES 4.1 stage 0 asks: divergence from Normal on the same view,
seed and skill (self-play, the style in both seats), bid rate, near ties
inside Normal's band and the style's (rung 4: one search serves both, tested
equal to `Opponent::decide`; the depth 0 fallback is decided apart), and the
margin calibration (first bidder's hand per seed; margin = the advantage at
which Normal's rung-4 sample bids the share of hands the style's rule takes).
The CSV's feature list matches STYLES 4.2.

5.1 **must (before stage 1). The CSV carries two non-public columns.**
`decisions` and `diverged` are computed by running Normal on the view; they are
not in the public record, and `diverged` is 0 for every Normal row by
construction, so a classifier given it reads the label. Fix: write them to a
separate file or mark them in the header (`probe_` prefix) and have stage 1 use
`Features::COLUMNS[..11]` only. The other eleven are counted from the seat's own
hand, the packets and the plays, all public once the deal is scored.

5.2 **should. The features have no counterpart for the new maxims.** Bermann
and Ulmann's differences from Normal are `cash_mond`, `guard_mond`, `spare_lead`
(and `low_shown_exception`: counted). `mond_early` (before trick 14) is the
only partial one. A style whose tell is not in the features fails the AUC gate
for the wrong reason. Fix: add `mond_cash` (Mond played on a suit lead of
thirds >= 7), `small_tarocks_left_at_trick_20` and `spare_lead_open` columns
before stage 1; the first two are record-readable.

5.3 **nit.** `stage0` always runs the calibration after the plays, and has no
way to run it alone (an empty `--skills` panics at parse). Add `--skills ""` as
none, or `--calib-only`, so the calibration (cheap) runs first (see 5.4).

5.4 **must (measurement order). `MARGINS` = 0.0: the styles already diverge at
rung 4 in play, and not at all in the bid.** `apply` sets `bid_margin =
Some(0.0)`, which is Normal's: rung 4 then bids by `advantage > 0` exactly as
Normal does (the style's `BidRule` is not read when `bid_margin` is `Some`), so
every style's rung-4 bid equals Normal's and the bid tells (`bid_taken`,
`bid_taken_le5`, bid rate, the bid half of divergence) are absent at 3.5 and 4.
But the plays do differ before calibration: every style but Normal gets `band`
2.0 against 0.34, and then its own weights decide inside the band, so
divergence in play is non-zero at rung 4 for every style, temperaments with
no changed weight included. Rung-4 style results are therefore meaningless
until the margins are set (a style would be Normal's bidder with a wide
maxim band). Order: (1) calibrate (`stage0`, calibration part, 1 000 hands,
`bid_worlds` 40: minutes), put the printed margins in `style::MARGINS`, commit;
(2) rebuild; stage 0 divergence and bid rate at skills 3 and 4, and the
band shares (a styled rung-4 bid rate should now equal the rule's share, check
it); (3) stage 1 on the new CSV; (4) rung-4 cost and exploit. A skill-3 stage 0
does not depend on `MARGINS` (rung 3 bids by `BidRule`). The calibration
margins hold for level 3.5 too (same `bid_worlds`). Also record in the stage 0
note that the calibrated share is the style's `BidRule` share on forehand hands,
not a ratio to Normal's rung-4 rate (STYLES pending 5).

5.5 **nit. `a_style_carries_its_calibrated_rung_four_margin` is tautological
while every margin is 0.0** (`bid_margin == rung4_margin()`); add a test after
calibration that the styled rung-4 bid rate on the sample equals the rule's.

## 6. Tests and docs

Tests are meaningful: Normal golden, per-style bid and maxim tests built from
hands, level 1 readability, wire tests through whole sittings, probe pair =
`decide`. Weak spots: 2.2 (comment instead of a test), 5.5, the spare-lead test
name. Docs (`OPPONENT.md`, `PROTOCOL.md`) are terse and match the code, except
STYLES 6 (`bad-input` for `new`) and the speed table (flagged as not
re-measured, correctly).

## Summary of fixes owed

must: 5.1 (CSV columns), 5.4 (order: calibrate before any rung-4 style run).
should: 2.1 (short Pagat is not an honour), 2.2 (remove the dead dealer
clause), 2.6 (tag), 5.2 (features for the new maxims).
nit: 2.3, 2.4, 2.5, 2.7, 3 (`rungs()`), 4 (STYLES 6), 5.3, 5.5.

## Fixes (17 Oct 2026)

| Item | Fix |
|---|---|
| 5.1 | The CSV's `decisions` and `diverged` are `meta_decisions` and `meta_diverged`, last; stage1 reads only the public columns (documented in `stage0.rs`, `probe::Features`); a test holds the split. |
| 5.4 | `stage0 --calibrate-only` runs the calibration alone (`probe::Options`); an empty `--skills` is an error without it, not a panic. Order as the review gave it; `style::MARGINS` stays 0.0 until the measured margins are pasted. |
| 5.5 | The margins are in `style::MARGINS` (stage 0, STYLES 9); the test holds the figures and orders them by the rules' bid shares. |
| 2.1 | An honour count takes the Pagat only with six tarocks (U 78), for Ulmann and Werner; tested short and long. |
| 2.2 | Ulmann's dealer clause and `bid_as`'s `dealer` parameter are deleted (`bid_with` is the one entry); the comment in `rung2.rs`, `style.rs` and STYLES 2.1 says it could change nothing. |
| 2.6 | Bermann's `guard-mond` is tagged I. |
| 5.2 | Feature columns `cash_mond`, `guard_mond_kept`, `spare_lead`; the positions are shared with the maxims (`mond_unescorted`, `cash_mond_chance`, `spare_lead_position`). |
| 2.3, 2.4, 2.5, 2.7, 5.3 | `cash-mond` trigger and `spare-lead` tagged I; the spare-lead test renamed (`..._highest_spare_card`); Ulmann's hunt comment names the omitted "no Pagat in hand" and "closed run". |
| 3 | Not done: `hint.rs:163` still calls `rungs()`; hint.rs is changing on another branch. `Level::rung()` exists; switch the call and drop `rungs()` after that merge. |
| 4 | STYLES 6: `new` answers `bad-config`, field `style`. |
