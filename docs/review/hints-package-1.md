# Review: hints package 1

Reviewed 17 Oct 2026: `worktree-agent-ab861426e0d012b27`, commits 622da8c (engine),
6360c92 (wire), d67bc33 (table and strings), c2038a9 (`HINTS.md` 14), against
`docs/HINTS.md` and `docs/RULES.md`. The reviewer did not write the code.

## Checked and run

- `cargo test -p strohmandeln-ai -p strohmandeln-wasm`: 275 + 44 + 3 + 2 pass, 17
  ignored (the lesions). `node --test kit/test web3d/test`: 558 pass on the second
  run; the first run, taken while another session was compiling, failed 31 (not
  traced; open). `python3 kit/build.py --check`: ok.
- `hintbench` (level 4, 300 leads, 300 follows, release, merge-base against the
  branch, interleaved, 3 runs): lead median 8.3 / 8.3 / 15.0 ms before against
  8.4 / 8.7 / 17.0 after; follow 7.1 / 8.6 / 12.7 against 7.1 / 7.2 / 14.0. The
  added work (A's column, `lead_trumped`) is inside the noise, far under the
  spec's 5 ms. Wasm not measured.
- A histogram of (question, sentence) over every play hint of 6 walked sittings
  (level 4) was used for H1, H2 and M5.

## Findings

Severity: H fix before merge, M should fix, L may be recorded open.

**H1. On a lead, `ask.keep` is asked about the Mond or the Sküs for most hints, whatever X is.**
A on a lead is the highest card held (3.2), which in a strong hand is the Mond or
the Sküs, so any X that is not a key card makes `ask.keep` decisive, and it comes
first in the teaching order (3.3). Of 929 sampled lead hints, 342 asked `ask.keep`
(103 + 92 about the Mond, 126 the Sküs, the rest the Pagat), 258 `ask.out`, 177
`ask.trumps`. Leading a master king with the Mond in hand reads "Play king of
hearts. Is the Mond safe to keep? The Sküs has not been played.": the same fault
as the `hold-pagat` complaint (an answer to a question the play does not raise).
Fix: on a lead, `ask.keep` decisive only when X is a key card, or when X and A are
both tarocks (the choice is between spending and keeping a key tarock); otherwise
the next question in order. This changes 3.3 for leads; record it in `HINTS.md`.

**H2. The keep sentence has no direction: the same fact advises opposite plays.**
"The Sküs has not been played." is the sentence for case (b) (play the Mond now)
and for a lead of a small tarock with the Mond held (keep the Mond): a newcomer
cannot tell which, so half 2 of section 0 is not met, and 5.4's "fact alone"
allowance does not excuse a fact that points both ways. Worse, when X is the key
card (`keep == X`), `ans.keep-mond-safe` ("The Sküs is not out.") answers "safe to
keep? yes" under "Play Mond.": 15 of the 54 follows with `keep == X` in the sample;
`ans.keep-skus` under "Play Sküs" likewise. Fix: split by direction (`kept == X`
play, `kept == A` hold) and state the verdict:
- Mond played, Sküs out and not mine: "Not while the Sküs is out: play it on his tarock, where it wins." (follow only);
- Mond played, Sküs not out: `ask.keep` not decisive (nothing to say); take the next question;
- Mond held (A), Sküs out: "Not yet: while the Sküs is out, the Mond is for his tarock leads, not yours." only if it can be backed by the Mondfang source (research/07 1.5); otherwise fall through to the next question.
The first fits the budget (headline 2 + question 6 + 16 words).

**M3. Worked case (a) does not say "no".**
"Play jack of diamonds. Should I lead a tarock? You hold 6 tarocks; 9 are
unseen." answers a yes/no question with two counts and no verdict or threshold;
a newcomer reads "6 against 9" as neither. It is a fact, not a rule, and counted
so, but it can point the way with a visible comparison: "Play jack of diamonds.
Lead a tarock? Not yet: you hold 6 of the 15 still out." (14 words) where mine is
under half of mine + out, "Yes: you hold {mine} of the {total} still out." where
at or over half. The rule that tarocks go first from a solid run stays unsaid (no
passed rule). Is `ask.trumps` the honest decisive question for J♦ against XIX?
Yes: they differ in exactly that (suit against tarock), and `ask.beneath` (J♦ is a
cover) comes later in the order by the spec.

**M4. Two tarock counts for the same thing.**
`ans.trumps` and `ans.out-void` use `unseen_tarocks()` ("unseen") while
`fact.tarocks-out` and `ans.keep-skus` use the tarocks neither mine nor played
("out", the tab's line 1). With one tarock he holds face up the hint says 15 and
the "Why?" 16 (probed: hand of 6 tarocks, T20 known). `HINTS.md` 14 decides for
the second. Fix: both use `tarocks_out`, the string says "still out"; add a test
that sentence and fact counts of the same quantity agree.

**M5. A regression until package 2: rule, odds and sample lines are not shown.**
`setAid(text)` drops `more`; the contrast, facts, rule and odds are on the wire
only, with no "Why?" control to read them. A player relied on "Against {alt}:
ahead in N of M sampled deals" and the rule clause ("Rule: an exposed queen...");
both are gone. Also 15% of lead hints (140 of 929: `lead-rate` 77, `lead-cheap`
29, `lead-draw` 13, `lead-trump-void` 8, `ask-none` 7, `keep-packet` 6) have no
decisive question and show the old plain reason or the sample line, so complaint
(a) is unchanged for them. Fix: record in `PLAN.md` that package 1 is not to ship
alone, or keep the odds clause in the aid line until the "Why?" lands.

**L6. German "keine Herz".**
`hint.ans.out-void` and `hint.fact.his-void` read "Er zeigte keine {suit}" with
Herz, Karo, Kreuz, Pik, all neuter: "kein". A test asserts the wrong form. Fix
both strings and the test.

**L7. The play imperative survives in three reasons.**
`hint.reason.lead-draw`, `lead-exposed`, `lead-his-none` still say "lead {card}" /
"{card} anspielen" after the headline "Play {card}.", and the test only checks for
a leading "Play". Fix: drop the clause; match `\b(lead|play) \{card\}` and
`{card} anspielen` in the test.

**L8. The budget test covers `ans`/`so` pairs only.**
The plain reasons that stand as `ans` (up to 20 words, plus headline and a 7-word
question: 29) are outside the 25-word check. Fix: add each `reason.*` the engine
can emit under a question to `budgetPairs`, or shorten.

**L9. The one passed rule is reachable only when X is the sole winner.**
When X wins and another winner exists, A is that winner, so `ask.worth` is not
decisive and `so.worth-rich` is never shown; `ask.cheap` is asked instead, though
the rule's own trigger ("can win and can not") is wider. Record in `HINTS.md`.
`so.worth-rich` also says "7 thirds" where every other figure says points (2⅓).

**L10. Tests partly written after the code.**
Meaningful where they pin the spec (`the_alternative_is_fixed_by_rule`, the budget
and table tests, the wire test). Gaps: none for `ask.suit`'s answer, none that a
sentence's direction matches the play (H2), none that counts agree (M4).
`hold_pagat_only_when_the_pagat_is_the_alternative` was not run against the old
code; by reading it fails there: `explain.rs` on main returns `hold-pagat` for the
Mond on Uhu (II) led and for a lead of II with the Pagat the lowest tarock.

## Answers to the questions put

1. Alternative A (3.2) is correct in every case (lead highest then cheapest;
   follow: X wins, cheapest other winner else cheapest loser; X loses, cheapest
   winner else cheapest other loser; only-card). `hold-pagat` is returned only
   when A is the Pagat, and a led Pagat is never A: it costs 13 thirds and
   `STRONG_TAROCKS` is 8, so with X the highest card A is a one-third tarock, and
   with X not the highest A is the highest. The decisive tests, priority and
   teaching order match 3.3 (H1 aside); keep-packet precedes hold-pagat.
2. Worked cases: see H2 and M3. The fact's condition is right (`opp_possible`
   holds the Sküs: not mine, not played; "not out" when mine or played). The
   missing direction is the fault, not the wording.
3. Facts use known cards only (`unseen`, `opp_known`, covers). `is_master` is
   "nothing out higher in its kind" for any of X, A, wider than `tracker`'s
   best-card master but true as worded. One mismatch: M4.
4. Wire and `PROTOCOL.md` agree with the wasm. No duplicate "Play X." in headline
   plus default sentence except L7 reasons; M5 is the regression. German is
   Sie-form and natural except L6.
5. Tests: L10.
6. Latency: no measurable cost (above).
7. `bin/lesions`: stale by hash only. `rung3.rs`, one of its hashed sources,
   gained `lead_trumped`, which the tutor does not call; `ask.rs`, `explain.rs`
   and `hint.rs` are not hashed and the students and classifiers do not use
   them, so the verdict cannot change. `bin/gate` will still ask for the rerun
   (about 100 s) and the publish step runs it anyway: rerun once before merge to
   refresh the stamp.

## Fixes (17 Oct 2026)

| Finding | Fix |
|---|---|
| H1 | On a lead `ask.keep` is raised only by a key card X or two tarocks (`ask.rs` `keep_case`); `HINTS.md` 3.3 and 14. Test `on_a_lead_keep_is_asked_only_of_a_key_card_or_between_two_tarocks`. |
| H2 | The keep sentences have a direction: `ans.keep-mond-play` (X is the Mond, a tarock led, the Sküs out), `-hold` (A is the Mond, the Sküs out), `ans.keep-pagat-hold` (8 or more tarocks), `ans.keep-packet`; with the Sküs not out the Mond raises no question. The `-hold` sentence is the reviewer's, backed by the Mondfang note of `research/07` 1.5 as cited in `HINTS.md` 2. Test `a_keep_sentence_points_the_way_of_the_play` matches the key's direction to who is kept, over several positions. |
| M3 | `ask.trumps` is "Lead a tarock?" with `ans.trumps-no` / `-yes` ("Not yet: you hold 6 of the 22 still out."); every sentence of a yes/no question opens with its verdict (Node test, en and de); `ans.worth-*` are "No: ..." / "Yes: ...". The total is mine plus tarocks out, so it is 22 less the played, not the reviewer's 15 (which counted only his side). |
| M4 | `ans.trumps-*` and `ans.out-void` use `tarocks_out`, as the fact does; test `the_tarock_counts_of_the_sentence_and_the_fact_agree`. |
| M5 | Package 2 is built on this branch: the "Why?" button in the hint box (intro, contrast, `trumped`, facts with `data-anchor`, rule clause, odds; at most 90 words, last lines dropped first), reachable by keyboard, closed by Escape (`overlays.js` layer `why`) and by each new hint; browser check `hint_why`. A position with no decisive question gets `ans.ask-none`, or `ans.ask-none-even` when the search sees under a third of a point, in place of the plain `lead-*` reasons. |
| L6 | German "kein {suit}" in `ans.out-void` and `fact.his-void`. |
| L7 | `lead-draw`, `lead-exposed`, `lead-his-none` lose "lead {card}"; the Node test rejects `(lead|play) {card}` and `{card} (anspielen|spielen)` in any reason. |
| L8 | Every reason that can stand as the sentence is within the budget with the headline (Node test). |
| L9 | `so.worth-rich` is in points ("2⅓ points or more"); its reach (X the sole winner only) is recorded in `HINTS.md` 14. |
| L10 | The direction, count and verdict tests above were written before the changes and watched failing. |
| Lesions | `bin/lesions` rerun after the changes: ok, stamp `6e2a6c42...` (in `target/`, not in git). |

Open: `answers` of the "Why?" is empty; the anchors are not highlighted in the
Cards-out tab; the Mond-held sentence has no measured rule behind it (a fact
about the Sküs, counted as half 1 only).
