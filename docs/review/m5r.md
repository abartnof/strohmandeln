# Review: M5r (rule refinement), code and measurement

Scope: branch `worktree-agent-a96035b484934e07f` against `main` (c833f5a code and
tests, 8c595e4 plan, 52c06ac results, `m5r_rules.rs`, `docs/measurements/m5r-*.txt`)
against `docs/HINTS.md` 0, 5.2, 7, `docs/MEASUREMENTS.md` (M5, M5r), `CLAUDE.md`.
Run: `cargo test -p strohmandeln-ai m5r` (24 pass). One diagnostic of the reviewer's
(40 sittings, seed 70,000,000, not a measurement, not committed) for items 3 and 7.
Severity: must (a claim is false or not reproducible), should, nit.

## Verdict

The design and the sequential procedure were followed as planned; no leak between
selection and confirmation. Accept with fixes. One must changes what the rules
say: the "tarocks out" atom is not the number the table shows. Three of the seven
passed rules (run/any, void/avoid, worth.poor) are marginal once the eight
confirmations are treated as a family.

## 1. Plan before the run, and the run followed it: holds

- Order: 07:24:11 code (c833f5a, `PASSED` empty), 07:24:53 plan (8c595e4, only
  `MEASUREMENTS.md`), 08:11:48 results (52c06ac). The three runs (608 s + 673 s +
  1,177 s wall, about 41 min) fit between the plan and the results commit. Nothing
  in the repository timestamps the output files themselves; the commit order is the
  only evidence.
- The plan's seeds (60 M selection, 65 M confirmation in batches of 100, 66 to 69 M
  settings at 200 sittings), eligibility (n_eff >= 60, Wilson lower >= 0.80, widest
  by firings then lower), boundaries 4.05 / 2.86 / 2.34 / 2.02, futility from look 2
  at z <= -1.645, fail at look 4 without a pass, are the constants of `m5r.rs` and
  `bin/m5r.rs` and are what `confirm.log` shows (every look line carries its
  boundary; pass/fail verdicts agree with `step`).
- Candidate counts (62 to 74 per class, at most 78) and the 12-atom lists agree
  with the plan; "an atom true at every firing is not tried" is in `select`.
- Changed after the plan: 52c06ac added `fits_budget` and replaced two test
  assertions (`cw <= 8`, `aw <= 10` per passed rule) with a flag check. That
  loosens a test the plan named ("tested for passed rules") after two rules failed
  it. Allowed (the budget is a presentation limit, not a gate) but should be said in
  the M5r section. Should.
- Seeds 60 M and 65 M are also the defaults of `measure` (tune/fresh,
  `bin/measure.rs:911`, `MEASUREMENTS.md` 1062). Different statistic, so no
  dependence, but the plan says nothing of the reuse. Nit.

## 2. Independence and multiplicity

Independence holds: `confirm` reads only `selection.tsv` (class, atoms); the
selection log is not read; seeds 60 M + 0..399 and 65 M + 0..399 do not overlap and
sitting seeds feed the deal shuffles directly. The settings seeds are disjoint.
The selection's winner's curse is visible and handled by design: `trumps/tarock`
0.902 on 61 firings at selection, 0.820 and a fail on fresh seeds; for the widest
candidates the shrinkage is small (cheap/outcome 0.837 to 0.843, run/any 0.822 to
0.817, worth.poor 0.851 to 0.830, void 0.836 to 0.837).

Multiplicity. The ~800 candidates are not the issue: one candidate per class goes
forward, so the confirmation is 8 tests (7 passed, 1 failed), not 15. But the 8
confirmations are 8 one-sided tests each at the O'Brien-Fleming overall level of
0.025 (the two-sided table used one-sided), with no correction across rules. A
Bonferroni level of 0.025 / 8 = 0.0031 one-sided (0.0063 two-sided) needs an
O'Brien-Fleming constant of about 2.78 (the K = 4 constant is 2.02 at 0.05; 2.78 is
reviewer's interpolation, to be recomputed from `stats.rs` before use), boundaries
about 5.6 / 3.9 / 3.2 / 2.8. Against the logged z values:

| Rule | z at stop (look) | Corrected boundary | Result |
|---|---|---|---|
| cheap/outcome | 6.65 (2) | 3.9 | pass |
| cheap/winner | 6.27 (2) | 3.9 | pass |
| pagat/keep | 4.14 (2) | 3.9 | pass, barely |
| trumps/suit | 5.00 (1) | 5.6 | not yet; needs look 2 (it stopped, no data) |
| run/any | 2.94 (3) | 3.2 | would continue to look 4 |
| void/avoid | 2.51 (3) | 3.2 | would continue |
| worth.poor | 2.71 (4) | 2.8 | fails by a hair |

So a family-wise correction is advisable for the claim "true precision >= 0.80", and
it would move three rules (run/any, void/avoid, worth.poor) from passed to
undecided, trumps/suit to undecided, and leave the two cheap rules and pagat/keep.
Mitigations: the figure told to the player is the point estimate with its n, which is
true whatever the gate (0.82 to 0.83 for the marginal ones); the rules are shown
one by one, not as a set; a false pass costs little (every p is above 0.78 at
every look). Still: (a) the same three also fail a setting at 200 sittings, so
their reach under the settings is already thin; (b) void/avoid is over the word
budget and run/any needs 13 action words, so neither can ship as measured.
Recommendation (should): record the Holm/Bonferroni boundaries beside the table,
ship the three firm rules plus trumps/suit if a fifth, pre-registered look (not
used so far) passes it at the corrected level, and mark run/any, void/avoid and
worth.poor "passed at 0.025 each, not at the family level". The decision is the
maintainer's (it is a gate change, `TUTOR.md` 11). The settings checks
(7 rules x 4 settings, lower bound >= 0.80 at 200 sittings, uncorrected) err towards
telling fewer rules; no change needed.

Other weaknesses, nit: clusters are deals, but the three deals of a sitting share
one opponent object and seed; positions arise against rung 3, not a human, so
"plays so in x% of positions like this" is rung 4's behaviour on its own
trajectory (which is what the sentence says). cheap/outcome and cheap/winner
overlap (the second is a subset) and are counted as two tests.

## 3. Atoms from visible state only, and "tarocks out": must

All atoms take a `Knowledge` built by `Knowledge::from_view(bare_view(d, who))` and
the classes' `check` functions read only `Knowledge` and the action; no atom reads
a hidden card. Checked one by one: `skus` (not played, not in hand or covers) equals
the table's "out" for the Sküs; `ml*`/`mt*` count hand and face-up covers, the
table's "Yours"; `void`/`notarock` from the tricks and his face-up cards, as
Cards-out line 3; `left*` thirds of the suit not yet played (mine included); `ph*`
the trick number; `led*` the card led; `master` from `played` and `playable()`;
`cov*`/`hc*` face-up covers. Fine.

**Defect (must): `tu0..tu8` is `Knowledge::unseen_tarocks()`**, the tarocks in no
hand, no cover of either side, not played and not announced/known in his hand
(`know.rs`, `unseen`). The table's line 1 "Tarocks still out: N" counts the tracker's
`out`, which adds his face-up tarocks on his covers and his known hand cards
(`tracker.rs`, `is_out`; `TABLE3D.md` 6.6.4 "unseen, or face up in his hand or on his
cover"). The legend and the hint sentence say "out"; the atom means "unseen". They
differ in 4,614 of 6,480 play decisions (71%) in the reviewer's run, because his
covers are face up for most of the deal. So the player reads N on the table, the
sentence says "4 or fewer are out", and the atom may hold at table N = 6 or 7: the
sentence contradicts what the player sees (HINTS 0 half 2: "from what they can
see"). `fact-tarocks-out` (HINTS 5.5) uses the same `unseen_tarocks()` against
"line 1" and has the same mismatch.

Fix, either: (a) define `out_tarocks()` = unseen + his face-up tarocks, make the `tu`
atoms use it and re-run selection, confirmation and settings (about 9 minutes of CPU
on the cap; the maintainer is consulted first); the diagnostic suggests the rules
survive, with higher precision and narrower reach (40 sittings, seed 70 M; fired
and p with the unseen count then with the out count: cheap/outcome tu4 971 / 0.859
then 701 / 0.889; cheap/winner tu4 608 / 0.863 then 422 / 0.884; worth.poor tu8
133 / 0.805 then 99 / 0.848; run/any tu6 788 / 0.838 then 627 / 0.863); or (b) keep
the atom and say "hidden": "tarocks neither played nor shown", which the table does
not give as a number (so it fails "recognise from what they can see"). (a).
Until then a table-reading player who applies the sentence is on the safe side (the
out count is never smaller, so "N <= 4" implies the atom) but the sentence will
show where the table says otherwise.

## 4. "Precision" and n_eff: correct

Precision of a rule is, over the decisions of rung 4 at which the idea's trigger
fires and every atom holds, the share in which the card rung 4 chose is in the
class: a statement about rung 4's behaviour, not about outcomes. `Log::record`
stores per firing the atoms that hold and `ok`; `evaluate` sums per deal; `n_eff` is
p(1-p)/Var of the deal-clustered ratio estimator, capped at the firings
(`measure::precision`); the Wilson lower bound uses n_eff; z uses n_eff against p0 =
0.80. Tests cover the selection, the minimum 60, the boundary and futility
logic. Two notes: (i) a pass on `cheap/winner` is conditional on rung 4 having
decided to win (the class fires only when the card wins), and `pagat/keep`,
`run/any`, `trumps/*` are conditional on the type of play chosen (the Pagat legal
and another card too; a tarock led with >= 2 held; a tarock and a suit card legal);
those triggers are part of the claim and are not in the drafts' conditions (see 6).
(ii) `worth.poor`'s "led <= 4 thirds" is implied by "a king wins" (a king beats
only a lower suit card), so "low lead" in the draft adds nothing and misleads (a
Dame is "low" by this definition, a king is not).

## 5. Reproducibility and the documents

- Tables in `MEASUREMENTS.md` agree with `m5r-select.txt`, `m5r-confirm.txt`,
  `m5r-settings.txt` and with `m5r_rules.rs` row for row (fired, p, n_eff, lower,
  reach, settings verdicts: trumps/suit throw-in 0.788, pagat ult2 0.791, run v123
  0.795, void all four, worth.poor throw-in 0.797 are the `settings_ok: false`
  rows).
- **Should: `m5r emit` cannot be reproduced from what is committed.** It reads
  `confirmed.tsv` and `settings.tsv`, which are not in the repository; the printed
  outputs round (3 decimals, n_eff to the unit, reach 3 decimals) where the
  generated file has 4 and one decimal (n_eff 1806.9, reach 0.9925). Commit
  `selection.tsv`, `confirmed.tsv` and `settings.tsv` to `docs/measurements/`
  (a few hundred bytes) and add a test that `emit` over them equals
  `m5r_rules.rs`. The log (`selection.log.tsv`) is large and may stay out; the
  selection table is then reproducible only by re-running (stated).
- Nit: the CPU figures ("2 min 20 s, 4 min 04 s, about 2 min 20 s") are not in any
  committed output (the files print wall seconds). Give wall seconds as printed.
- Nit: "Five of the seven conditions are one quantity" counts distinct atoms; by
  rule it is six of seven (cheap x2, run, worth.poor use `tu`, pagat and trumps use
  `ml`).
- Should: no test fails a shown `so` that contains a banned word (item 6); add one
  over `ATOMS`/`CLASSES` text.

## 6. The drafted sentences against HINTS 0 half 2 and 7

Faults in the drafts and in the template texts (`ATOMS`, `CLASSES`, `m5r.rs`):

1. **"cheap", "cheapest", "billigste" (must, new rule)**: both cheap drafts and the
   class texts. Say "worth the fewest points" / "punkteärmste".
2. **"he", "his", "er", "ihm" (must)**: the opponent is "your opponent" (atoms
   `void`, `hc1/2`, `led4..13`; classes `mond/mond`, `void/avoid`). Only void/avoid
   is passed, but the templates are generated for all.
3. **"neighbouring tarocks" (must)**: defined nowhere in `HINTS.md` (7 forbids
   "solid run" and offers "next in rank to another of yours"; the measure defines it
   in `rules.rs` 148 as neighbours among the tarocks not yet played, so a played
   card closes a gap). Use the 7 phrase and say "among those still in play" in "Why?".
4. **"tarocks are out" (must, item 3)** against "still out", the table's word.
5. **Triggers missing from the conditions (should)**: precision is conditional on
   the idea's trigger (following; the Pagat legal; a king that wins; leading a tarock
   among >= 2; a tarock and a suit card both legal). A sentence without it can be read
   as a rule for positions outside the measurement. Name it in the condition where
   it fits the budget.
6. **"low lead" (should)**: see 4 (ii).
7. **Budget and terms**: the two over-budget drafts are right to be held back. A
   noun-phrase condition ("Holding 7 tarocks or fewer, Sküs still out:") gets
   trumps/suit to 8 condition words. "points" is `thirds`' term; "Pagat", "Sküs",
   "cover", "tarock" are `the-cards`.

Proposed sentences, each the measured condition and action, with the budget count
(condition + action words), only rules that passed. Told under the settings noted
(an `*` = the measured rule is told under every setting).

| Rule | en | de | words |
|---|---|---|---|
| cheap/winner `tu4`, card wins | With 4 or fewer tarocks still out, take the trick with the fewest-points card that wins. | Mit höchstens 4 Tarock noch draußen: den Stich mit der punkteärmsten stechenden Karte holen. | 7 + 9 |
| cheap/outcome `tu4`, card loses (or either) | With 4 or fewer tarocks still out, win or lose with the card worth the fewest points. | Mit höchstens 4 Tarock noch draußen: stechen oder nicht, mit der punkteärmsten Karte dafür. | 7 + 10 |
| pagat/keep `ml5` (not under ultimo 2) | Following with 5 or fewer tarocks, keep the Pagat. | Beim Bedienen mit höchstens 5 Tarock: den Pagat behalten. | 7 + 3 |
| trumps/suit `ml7+skus` (not throw in) | Holding 7 tarocks or fewer, Sküs still out: lead a suit card. | Mit höchstens 7 Tarock und Sküs noch draußen: eine Farbkarte anspielen. | 8 + 4 |
| worth.poor `tu8` (not throw in) | With 8 or fewer tarocks still out, if your king would win, play another card. | Mit höchstens 8 Tarock noch draußen: würde Ihr König stechen, eine andere Karte spielen. | 7 + 9 |
| run/any `tu6` (not values 1/2/3) | With 6 or fewer tarocks still out, lead a tarock next in rank to another of yours, else your lowest. | Mit höchstens 6 Tarock noch draußen: ein Tarock anspielen, das im Rang neben einem Ihrer Tarock steht, sonst das niedrigste. | 7 + 13 (over; does not fit) |
| void/avoid `left13+ph1` | none: no fitting form; do not ship | | |

Notes on the table: the cheap rules are told under every setting (`*`); "the card
worth the fewest points" ties by the lowest card (not stated, a tie only among equal
points); "win or lose" refers to the two result classes of the legal cards and is the
class of the card played; the `ml` counts are the cards in hand and face-up covers
(the table's "Yours"); the `tu` counts are those of item 3 once fixed. German uses
"stechen" for taking the trick, as the existing drafts. If run/any is shipped the
action budget must be raised to 17 or the sentence dropped; the review recommends
dropping it (item 2, settings failure, budget).

## 7. Five (six) of seven are tarock counts

Not a sign of an error, a sign of the design and of the world:

- The family gives the count atoms the most slots: `tu` (5 thresholds), `mt` (4),
  `ml` (2) are 11 of 31 atoms and fill 4 to 8 of the 12 slots of each class list
  (cheap 4; run 8; trumps 8). They are nested thresholds of one quantity, so they form
  correlated candidates, and "the widest eligible" selects the loosest threshold that
  still clears 0.80: the edge of a gradient. That is acceptable (confirmation is on
  fresh seeds) but the edge is where p sits just above the gate (lower bounds 0.806
  to 0.835).
- `tu` is largely the phase of the deal. In the reviewer's run, 98% of
  cheap/outcome firings with `tu4` fall in tricks 10 to 27. Within trick bands the
  atom still separates (cheap/outcome tricks 10-18: 0.808 with `tu4`, 0.695 without;
  tricks 1-9: 0.800 on 15 firings against 0.560; tricks 19-27: 0.888), so it is not
  only the trick number; but in practice it says "late in the deal, few tarocks
  left". The `ph3` atom was in the family and lost on width. The hint should say it
  as such in "Why?": "this is how I play late in the deal, when few tarocks are
  out", and not as a general rule of the game. It is also "tarocks out = 22 - played
  - yours" (and so many of yours, early), so a player holding 12 tarocks meets
  `tu8` in trick 1.
- No bias against correctness: the other atoms (suit points left, master, covers,
  void) are tried on every class that lists them and mostly give no eligible
  candidate (off x4, mond, run/top, pagat/cash: none). That the tarock count wins
  where something wins is a result about rung 4, whose rules (`rung3`/`rung4`)
  weigh the tarocks left.

## Findings

| # | Severity | Finding | Fix |
|---|---|---|---|
| 1 | must | `tu` atoms count unseen tarocks, the table shows out (including his face-up); 71% of decisions differ | `out_tarocks()`, re-run (maintainer consulted first), fix `fact-tarocks-out` |
| 2 | must | banned words in the templates: cheap/cheapest/billigste, he/his, "neighbouring" | rewrite as in 6; add a test over `ATOMS` and `CLASSES` |
| 3 | should | 8 confirmations, no family-wise correction; three rules marginal | record the corrected boundaries; decide the three; optional fifth look |
| 4 | should | `emit` not reproducible from committed files; rounding | commit the three tsv; add an emit-equals-file test |
| 5 | should | triggers missing from drafted conditions; "low lead" | state the trigger, drop "low lead" |
| 6 | should | the tarock count is a phase proxy | say so in "Why?" |
| 7 | nit | test loosened after the run (`fits_budget`) | record it in the M5r section |
| 8 | nit | CPU times not committed; "five of seven" is six of seven by rule | print wall seconds; reword |
| 9 | nit | seed reuse with `measure` defaults; deal clusters within a sitting | note |
