# Review: docs/STYLES.md (branch worktree-agent-a040aa1e3acb64a58, commit 73231e0), 17 Oct 2026

Reviewer: an agent that did not write it. Read against the texts themselves
(`research/sources/03/wb/3752218.txt` = W pp. 129-135; `.../abo/ulmann1899/ocr.txt`
= U pp. 69-78; `.../abo/bermann1894/ocr.txt` = Be pp. 143-153;
`research/sources/07d/ms_web_2002.html.txt` = MS), `research/03`, `07`, `16`,
`docs/RULES.md`, `docs/OPPONENT.md`, `docs/PROTOCOL.md`, `crates/strohmandeln-ai`
(`rung2.rs`, `rung3.rs`, `rung4.rs`, `stats.rs`, `bin/tournament.rs`).
Severity: must (blocks), should, nit. Tags: T = the text states it, I = inferred
from the text, E = emphasis (a multiplier or threshold no text gives).

Checked and correct: every quotation and page in STYLES 2.1 reads as cited
(W 131, 133, 134, 135; U 71, 72-73, 73-74, 74, 75, 77-78; Be 145-146, 148,
150-151; MS "Wie stark ist dein Blatt", "Mondfang", Pagat sections). The OBF
arithmetic (5.1) reproduces. `rung4::best_card` band 0.34 ("a third of a third")
is right. The mirrored-pair design and the opponent staying unnamed are right.

## 1. Citations

1.1 **must. U 74 "Mond" does not say guard it.** U 74: "Auch den Mond, wenn er
nicht vom Sküs begleitet ist, suche man rechtzeitig heimzubringen" (bring it
home in time, i.e. take a trick with it before the Sküs finds it). The
sentence on keeping the smallest tarock ("Das Zurückbehalten des kleinsten ...
weil man mit einem solchen den Gegner ans Spiel bringen kann") sits a
paragraph earlier, in the tarock-lead paragraph, and is not about the Mond.
The small-tarock reserve is tied to the Mond only in Be 151-152 ("sich eine
oder mehrere Tarok zu behalten ... so kann man ... den Mond abfangen"; and
"diesen ... stets bei passender Gelegenheit einstechen" when not beside the
Sküs) and MS "Mondfang" (keep one or two small tarocks as a defence).
Fix: `guard-mond` (`keep_tarocks` up) belongs to Be (151-152) and MS; Ulmann
gets at most "cash an unescorted Mond early" (T). Correct research/16 section 1
("parked" row), which merges the two sentences.

1.2 **must. Ulmann's "hold the Pagat from 9" and "10 and more hold it to the
last trick" are not in U.** U 74 gives two things: no Pagat in hand and 10-12
tarocks with a closed run, hunt it (T); the Pagat with "nur wenigen, etwa nur
sieben bis acht Taroks", take it in at the first chance (T). Nothing says what
to do with the Pagat and nine or more. `STRONG_TAROCKS` 9 is only the
complement of "7-8 cash" (I), and the tell "against 10 and more it holds it to
trick 27" and the exploit `THREAT_TAROCKS` 4 / "keep a high tarock for trick 27"
are E. MS ("7 or fewer" cash) gives exactly Normal's 8 (`rung2.rs` comment: with
few tarocks cash it), so U differs from Normal over the Pagat only at exactly 8
tarocks. Also `draw_trumps` in rung 2 already fires from 7 tarocks
(`STRONG_TAROCKS - 1`), rung 3 from 6, so Normal hunts more eagerly than U (10+)
and MS (12+): the spec's `hunt-pagat` would make U and MS lead fewer tarocks
below the threshold, which the spec does not say. Fix: drop the hold claims and
the trick-27 exploit; define `hunt-pagat` as a threshold on `draw_trumps`.

1.3 **must. Bermann 148 is the opposite of Normal's exposed-figure exception,
not a x1.5.** Be 148: "Liegen Figuren in einer Farbe offen auf, in welcher die
Päckchen des Gegners leere Blätter zeigen, so sind sie stets abzuspielen." Rung 2
plays an exposed Dame/Cavall unless the opponent shows two low cards of the suit
(`low_shown < 2`, U 74 and MS agree on that exception). Be says always, exactly
when the opponent shows blanks ("leere" = blank pips, as Be 151 "leeren Karten").
Research/16 section 2 (`off`) says the exception "agrees"; it does not, so this
is a sixth contradiction. The spec's "x1.5 on that condition" does nothing:
Normal already plays the figure whenever the exception is absent. Fix: Be =
remove the `low_shown < 2` exception (T, with the reading "leere Blätter =
blanks" stated); add the row to research/16 section 3. This is the most
distinct and best-sourced Bermann parameter.

1.4 **must. Multipliers on shared maxims are E, and contradict 2.2.** `cover`
x1.25 (W, U), `solid_run` x1.5 (U), `split_suit` x1.5 (W): the texts state these
maxims in the same words as Be 145 / 151 and MS and at no greater strength than
Normal already has (`cover` 0.8, `solid_run` 2.0, `split_suit` 4.0, all from
the consensus rows of research/16). None says "more". STYLES 2.2 itself puts
"shared rules" in Normal. Also inconsistent between writers: Be 145 and 148 say
packets off "möglichst rasch" and "stets" yet get no `cover` change. Fix: a
column T/I/E per parameter in 2.1; keep only T and I in the claim "plays W's
book"; label E "emphasis (design)" or drop it.

1.5 **must. Row 3 is not a W vs U/MS contradiction.** W 135 "In zahlreichen
Fällen wird man sich erst von seinen Taroks befreien müssen, um im Farbenspiel
vortheilhaft agiren zu können" says the same as U 74 ("nachdem man sich zuvor
der Taroks entledigt hat") and MS ("zwischendurch tarockieren, ehe er eine nicht
geschlossene Farbe anzieht"). Only Be (146, 150: tarocks "selten einen anderen
... als den Werth der leeren Stiche") recedes, and Be 151 still says draw the
Pagat out when strong. So W `draw_trumps` x1.5 / `keep_tarocks` x0.5 is E, and
the W tell "leads tarocks early, even with few" extrapolates ("in zahlreichen
Fällen", no count). Same for Be: `draw_trumps` x0.5 conflicts with Be 151
("fortgesetztes Tarokschlagen" with a strong tarock hand); the T part is the
bid and the figure rule only. Fix research/16 section 3 row 3 and STYLES 2.1.

1.6 **should. Row 2 (counting) is also not a split.** W 133-134 asks for
"Tarokzählen und ... Aufmerksamkeit auf jede einzelne Farbenkarte" and "wie viel
Karten von jeder Farbe schon draußen"; U 76 says the suits deserve attention "in
weitaus höherem Maße" than the tarocks. So W and U agree with Be and MS. The
decision "not modelled" stands; "disagree in five places" (STYLES 1) is three
(rows 1, 4, 5) plus the figure exception and Be's tarock stance.

1.7 **should. Werner's bid and its exploit.** W 131: "Hat man 3 Honneurs, 6 bis 7
Taroks und eine oder zwei besetzte Damen ... so kann man eine Aufnahme schon
riskiren" is a sufficient condition, not a floor, and honours are the Troule and
the four kings. "Passes below 6 tarocks" and `min_tarocks` 6 are I at best (the
floor is MS's "mindestens 6 bis 7 Tarock", so W = MS here). "Pagat sure winner
from 7 tarocks (Normal 6)": W gives no number ("schwach mit Taroks besetzt"),
and 6 is the engine's own (`sure_thirds`, `tarocks >= 6`), so 7 is E. The
exploit "its pass tells you it holds at most 5 tarocks" is invalid: a hand with
8 tarocks and one honour passes too.

1.8 **should. Bermann's bid and contested-suit parameters.** Be 145-146: "20 bis
25 Points in Honneurs und höheren Figuren"; no word on a tarock count, so
`min_tarocks` 0 is I (from "Tarok ... Nebenrolle", Be 146). "Every queen counted"
is a new counting rule (`sure_thirds` counts a queen only with its king or from
three cards). Be 150-151 is one position: a suit wholly split, a fork each (king
and cavall against queen and jack), the leader holding spare low cards; the spec
generalises it to `split_suit` x0.25 (E), and "at least 2 spare low cards" is
design. Be's Dame-and-Bube-with-Cavall-on-a-packet advice (148) is T and unused.

1.9 **should. Rückhand (open question 6): the dealer.** Evidence: U 70 (Vorhand
declares first; if she passes "steht es dem Kartengeber frei aufzunehmen"); W 130
("der Kartengeber, die Hinterhand, das Recht, die Partie aufzunehmen"); RULES line
19 (forehand = non-dealer, dealer = Hinterhand); RULES 3.2. U also uses
"Rückhand/Hinterhand" for a trick position ("sich in die Rückhand zu spielen",
U 73; Be 151 "Hinterhand bringen"), which is the dealer's in trick 1 as well. At
U 77 the context is the take-up, so Rückhand there is the dealer. But U says
only "namentlich in der Rückhand" for three weak honours without queens:
"the dealer needs 55 sure thirds" (+10 over Normal, for every hand) is E. Fix:
apply the dealer's stricter bid only to the hand type U names (three honours,
no queen), and record the reading; close question 6.

1.10 **nit. Dates in labels.** W's 2nd edition is undated in `research/03` S12
(1884-1906; 1891 is a review date from 04/12, a terminus ante quem), yet the
label says "(1891)" as the edition. MS is undated while the others are dated.
Use "(2nd ed.)" or no years; if a date is wanted for MS, the Wayback copy is 2002.
Question 9 can be closed with "no dates".

## 2. Distinctness

2.1 **must. Only Bermann will read as different.** W and MS share the 6-tarock
floor, the contested-suit rule and the tarock direction; U differs from Normal at
exactly 8 tarocks (1.2) and by E multipliers. With E removed (1.4, 1.5) the
sourced differences are: the bid rules (all four), Be's figure rule (1.3), Be's
tarock stance, the Pagat thresholds (U vs MS vs Normal, rarely reached), the Mond
reserve (Be, MS). Bid rates alone, by my reconstruction of the spec's rules on
40 000 random 15-card hands (Normal 45/5 18%, W 6 tarocks 12%, U 14%, Be 60/0
every queen 6%, MS 60/6 3%; rough, a throwaway script, not in the repo), give a
pairwise AUC on a window of 2 deals of at most 0.63 for every pair, 0.72-0.81
at 6-12 deals only for Normal v Be and Normal v MS. A gate of "AUC at least 0.8
on a 2-deal window" fails every style on bids and can pass only on in-play tells
that occur rarely (Pagat at 7-8 tarocks, a lying figure with blank tops). It is
also not defined: features, classifier, pairs (10 including Normal), train/test
split, what counts as observable.
Fix: window = one sitting (6 deals) pooled over features read from the deal
record (the human sees every card after the deal, so private hands are fair);
one-vs-one logistic regression on pooled counts, cross-validated; pass = AUC >=
0.8 against Normal and against the nearest style; a null control (Normal v Normal,
disjoint seeds, about 0.5) fixes the classifier's overfit; 1 000 deals at skill 3
first. Cheap: native rung 3 is about 0.05 s a mirrored pair, so 1 000 deals are
seconds; rung 4 about 7 minutes a style (1.3 s a rung-4 seat-sitting, below).

2.2 **must. Rung 3/4 let weights act rarely; measure it before any tournament.**
Rung 3's leads are dominated by the sampled-trick mean; weights of 0.2-1.0
thirds (the E multipliers on `cover` 0.8) flip few decisions. At rung 4
weights act only within 0.34 thirds of the best average margin. Add a stage 0:
for each style and skill, the share of decisions where the styled choice differs
from Normal's on the same views (200-1 000 deals), plus, at rung 4, the share of
decisions inside the band. A style below about 1% divergence cannot be readable
and is dropped without a tournament.

2.3 **should. Rung 4's bid is not a `BidRule`.** `rung4::decide` bids by
`bid_advantage > bid_margin` when a margin is set (`BidRule` is used only when
`bid_margin` is `None`). The spec says a style "shifts the margin, in Points" but
gives no map from "20-25 points, no tarock count" to a margin. Define it
(for example the Points advantage at which the writer's rule and the sampled
rule bid the same share of hands), or state that styles do not change rung 4's
bid. The 2-thirds tolerance band for rung 4 is new and, wider than Normal's 0.34,
makes the style weaker by construction (it is what 5.3 then measures).

2.4 **should. All styles bid less than Normal** (2.1 figures), so at rungs 2-3
each style is probably weaker through bidding alone; with E removed this is the
main component of the 5.3 "cost". Say so, and decide whether the writers' bids
are literal (question 5) before measuring.

## 3. Skill rows

3.1 **must. Level 1 is under-specified.** "Stage-1 maxims only" gives no lead rule
and no bid. With the other weights at zero rung 2 leads by `-0.5*thirds` (the
cheapest card) and tarocks by `spend`; the bid is not mentioned (the rung-2 rule
would make level 1 bid exactly like level 2, which is not a stage-1 skill). Give
the 12 `Weights`, the bid and `STRONG/WEAK/THREAT` for each of 1 and 1.5 in a
table; a test per row (deterministic function of seed and view, as
`rung2` tests do).

3.2 **should. Ordering and fit.** 1 to 1.5 adds only `cover` (0.8 thirds), which
is likely not separable; 1.5 to 2 adds ten maxims at once, so 1.5 is near 1 or
near 2 by accident. Order by the ladder's stages (`worth`, `cheap`, then `off`
with `exposed_figure`, then `behind`/`split_suit`, then tarock maxims). 2.5 is "rung 2 maxims
with Knowledge and 6 worlds": rung 2's maxims never read `opp_void`, so memory
alone adds nothing; the row is rung 3 with 6 worlds instead of 24, a
difference of sampling noise on a step that is already +1.53 +- 0.40. Merge it
now, not after 600 pairs. 3.5 (rung 4, 3 worlds, 8 000 nodes) is the only
intermediate with a wide gap to fill (4 v 3 is +6.73); keep it. Code facts:
`STRONG_TAROCKS`, `WEAK_TAROCKS`, `THREAT_TAROCKS` are consts (and rung 3 uses
`STRONG_TAROCKS - 2`, rung 2 `STRONG_TAROCKS - 1`; rung 3's split band is
0.35-0.65, rung 2's 0.3-0.7), `sure_thirds` hard-codes Pagat from 6 and the
queen rule, `BidRule` has two fields: all become fields of `Config`/`BidRule`,
and `tournament`'s `parse_spec` needs them.

3.3 **should. Level 1 as a beginner's opponent.** Beatable: yes (a greedy
single-trick taker with no packet or suit play). Readable and "not random": the
design says so, nothing measures it. Add two checks: level 1 against random
(sanity: clear and large, replaces the old 2 v 1 +18.36) and level 2 against
level 1 (the first dial step). Whether it suits a human beginner is a maintainer
playtest, not a tournament.

3.4 **should. Side effects missing from section 6.** `Level::from_f64` and
`tournament` specs use halves that currently mean rung L+1 with slips: every
old `2.5`/`3.5` string, test (`the_rules_of_thumb_beat_random_play`,
`tournament.rs` tests), `docs/OPPONENT.md` speed table rows and the golden
file for level 1 change meaning. Saved sittings at 1 or x.5 replay (the
opponent's moves are recorded, PROTOCOL 315) but continue under the new meaning:
say so.

## 4. Measurement plan

4.1 **verified.** `stats.rs` `obf_constant(6)` = 2.053; z_k = 2.05*sqrt(6/k) =
5.02, 3.55, 2.90, 2.51, 2.25, 2.05 reproduce, and 2.90 at look 3 matches the
3-v-2 rerun. Mirrored pairs: `tournament::mirrored`.

4.2 **must. The "too small to matter" stop cannot fire as written.** The tool's
stop (`stats.rs`) is the whole 95% interval inside +-`negligible` from look 2; the
spec says "upper end below +0.5". The per-pair sd is about 6.9 (3 v 2: se 0.40
at 300) to 7.3 (4 v 3: se 1.16 at 40); at 600 pairs the half-width is 0.56 > 0.5,
so a true zero never stops. For the exploit statistic D (two arms, sd up to
about 10) the half-width at 600 pairs is 0.8. Needed n for half-width 0.5 is
about 750 (sd 7) or 1 500 (sd 10). Fix: `--negligible 1.0` (about two thirds of
the thinnest dial step, 3 v 2 = +1.53), which fires at about 190 pairs (sd 7) or
380 (sd 10); or lift the maximum. Say which statistic is one-sided.

4.3 **must. "Expected a third of the worst case" is the wrong expectation.** Real
effects stop early (3 v 2 stopped at look 3); a null stops only on futility,
and the likely exploit result is null (1.2, 2.2), so null runs cost the maximum.
Plan on the maximum for tests that are expected to fail.

4.4 **should. Claim and multiplicity.** Boundaries are two-sided but the claim is
D > 0 and "ships if the lower end is above 0": say "clear with D > 0" and, since
a counter that gains under 1 Point a pair is not worth a rule, "lower end above
`negligible`". The plan runs about 30 sequential tests (3 styles x 2 skills of
exploits, 6 + 9 steps, 6 costs) at 5% each, about 1.5 false clears expected if
all were null. Gate the families (below) and, for any counter that ships, repeat
once on fresh seeds (the 3 v 2 rerun precedent).

4.5 **should. Exploit arm at skill 4 is null by construction.** The counters are
weights and consts; rung 4 ignores weights outside the 0.34 band and bids by
margin, so `THREAT_TAROCKS`, `keep_tarocks` and `min_tarocks` changes cannot
act. Skip rung-4 exploits unless stage 0 shows divergence.

4.6 **should. Compute estimate.** The spec derives pair costs from the wasm
median per decision times 495 decisions (`docs/OPPONENT.md` speed table). That is
a median (bids have a p95 of 135 ms), in wasm in Node, for both seats; the
tournament is native. Measured with the existing release `tournament` binary
(built 15 Oct; first seed 9 000 000, 6 pairs, a throwaway probe; load average
about 4 on 2 vCPU, so an upper bound): `3 2` 0.3 s (0.05 s a pair); `4 3` 15.8 s
(2.6 s a pair); `4 4` 17.7 s (2.95 s a pair). The spec's "rung 3 about 2 s a pair"
is about 40x high; "rung 4 14 s a pair" is about 5x high. Its arithmetic from
its own inputs is consistent (7 h + 7 h + 3.6 h + M3 = about 20 h); the inputs are
not. Re-derived below.

### Staged plan (cheapest question first; the tool is single-threaded, so two
runs fit two vCPUs)

| Stage | Question | Method | Cost (worst) | Gate to next |
|---|---|---|---|---|
| 0 | Do the styles differ in play at all? | decision divergence vs Normal, bid rate, 1 000 deals, skills 3 and 4 (no tournament); at rung 4 the share inside the 0.34 band | rung 3 seconds; rung 4 about 4 min a style; about 20 min in all | drop a style below about 1% divergence |
| 1 | Readable? (tells gate, 2.1) | same 1 000 deals, pooled sitting windows, one-vs-one AUC with the null control | no extra | merge/drop styles failing 0.8 |
| 2 | Rung-3 exploits (D, 3 styles at most) | 100 x 6 pairs, `--negligible 1.0`, two arms | 1 200 pair-runs x 0.1 s = about 2 min a style | ship a counter only if clear, D lower end above 1, and a fresh-seed repeat |
| 2b | Style cost and dial order at rung <= 3 | S v Normal at 3; Normal steps 1 v 1.5, 1.5 v 2, 2 v 3 | minutes in all | merge dial rows that do not separate |
| 3 | Dial order through rung 4 | Normal 3 v 3.5, 3.5 v 4 | 2 x 600 x 3 s = 1 h | merge rows that fail |
| 4 | Rung 4 for styles that survived stages 0-2 only | exploit (6 s a pair) and cost (3 s a pair) | per surviving style 1 h + 0.5 h | none |
| 5 | Tutor M3 once per surviving style at skill 3 | 240 deals | 15 min a style (spec figure, unverified) | |

Worst total: about 0.5 h (stages 0-2b) + 1 h (3) + 1.5 h x survivors (at most 3,
so 4.5 h) = about 6 h CPU, 3 h wall on two cores; expected about 2 h (most styles
should fall out at stages 0-1; stage 3 and any survivor are the real cost). Spec:
about 20 h worst, "a third" expected. All figures are from a 6-pair probe under
load and need re-measuring on an idle machine before the maintainer approves.

## 5. Protocol, record, restore, Random

5.1 **should. `drawn` must be authoritative.** The spec says it is "recomputable
from the seed"; the pool is `werner, ulmann, bermann` (and `ms`): adding or
dropping a member changes the hash result for the same seed, so a recomputation
disagrees with an old record. Define: `drawn` in the record is the truth; the
hash is used only when it is absent; an import with `style: "random"` and a
`drawn` outside the pool, or `drawn` with a non-random style, is `bad-record`;
an unknown `style` on import is `bad-record` (the spec gives `bad-input` for
`new` only).

5.2 **should. Do not leak through the engine.** The brief entry points take a
`style s` header; for Random that is the drawn id, so the engine's brief text
must not name it before the end. Put a `reveal` flag in the engine's brief
functions (the end-of-sitting call sets it) rather than trusting the table to
filter prose. Say what happens when a Random sitting is abandoned (new sitting
from the plus): reveal on leaving, or never.

5.3 **should. Old records and callers.** A brief header without `style` is
`normal` (stated for `new` and export, not for the brief calls). Export version:
`protocol: 1` stays, `record.version` stays 1 (the opponent's moves are recorded);
state this, and that the `golden.txt` at level 1 is re-recorded (3.4).

5.4 **nit. Hidden-state wording.** "Style: Random (shown at the end)" is fine; the
export file and the browser's saved sitting carry `drawn`; the doc already says so.

## 6. Naming and kindness

6.1 **should. Style-tell lines.** CLAUDE.md: "The opponent's lines come from
period sources (`research/05*`)". The `style-tell` lines come from the rulebooks
(`research/03`, `16`). They are period sources but not 05; get the maintainer's
word or use 05 lines. A quoted maxim in the opponent's mouth is the writer
speaking, which 2 forbids ("no string has the writer as speaker"); settle it as
anonymous maxims with the citation only in the phrase bank source, never shown,
and silent under Random (already). Each needs en and de; the research/16
translations are the agent's own and must be marked as such in the bank.

6.2 **should. The `opponent_unnamed` check is in the `NOT_YET` set**
(`web3d/test/browser_suite.test.js`, PLAN line 76), so "extend it" extends
nothing. Write it first, with the style cases, test-first.

6.3 **ok.** Labels read as schools ("After X"), the opponent stays "your
opponent"; the writers are historical authors; no kindness-rule issue in the
quoted lines (all about play). MS is a modern author pair; a label without a
date is fine.

## 7. Terseness and CLAUDE.md doc rules

7.1 **should.** Section 7 ("Dropped and why", "The first draft's personalities")
and 2.3 ("the maintainer's first examples") narrate drafts and who asked, which
CLAUDE.md forbids; record the decision and its date and source instead (for
example "Reckless / declaration-loving: no source, not offered (maintainer
request, 17 Oct 2026)"). The header "Design, 17 Oct 2026" lacks the source.
Section 8 (open questions) belongs in PLAN.md (the orchestrator edits it) or
under a short "Pending decisions"; the answers found here close 6 and 9 (1.9,
1.10) and make 8 a staged plan (4.6).

7.2 **nit.** 2.1 is long prose per writer; a table (parameter, value, text,
T/I/E) is shorter and carries the citation check. 5.2/5.3 CPU figures should be
replaced by the measured ones (4.6) and the derivation dropped.

## Summary

Musts (10): 1.1 Mond guard misattributed to U; 1.2 Ulmann's Pagat hold and the
trick-27 exploit are not in U; 1.3 Bermann's figure rule contradicts Normal's
exception (not x1.5; and a sixth contradiction missing from research/16); 1.4
multipliers on shared maxims are emphasis, not sourced; 1.5 W's tarock shedding
agrees with U and MS (row 3 is mis-stated); 2.1 only Bermann is readably
distinct and the 2-deal AUC gate is unreachable and undefined; 2.2 add a
divergence stage before any tournament; 3.1 level 1 and 1.5 not specified; 4.2
the futility stop cannot fire at 0.5 and 600 pairs; 4.3 null runs cost the
maximum (the "third" is wrong). (That is nine items; 2.1 and 2.2 are one
finding if merged.)

Shoulds (18): 1.6, 1.7, 1.8, 1.9 (Rückhand = dealer, answered), 2.3, 2.4, 3.2,
3.3, 3.4, 4.4, 4.5, 4.6, 5.1, 5.2, 5.3, 6.1, 6.2, 7.1.

Nits: 1.10, 5.4, 7.2.

## Fixes (17 Oct 2026)

`docs/STYLES.md` revised; no code.

| Item | Fix |
|---|---|
| 1.1 | `guard-mond` moved to Bermann and MS; Ulmann gets `cash-mond` only; research/16 correction listed (STYLES 7). |
| 1.2 | Ulmann's Pagat hold and trick-27 claims dropped; `hunt-pagat` is `draw_from` (10 U, 12 MS; Normal 7 rung 2, 6 rung 3). |
| 1.3 | Bermann: the `low_shown < 2` exception removed (T, reading stated); sixth contradiction listed for research/16. |
| 1.4 | Writers carry T/I only; every parameter tagged; emphasis multipliers moved to the temperaments (E allowed there). |
| 1.5 | Row 3 restated; Werner and Bermann `draw_trumps` changes dropped. |
| 1.6 | "Five disagreements" claim removed; counting stays not modelled. |
| 1.7 | Werner's bid is a sufficient condition read as the rule (I); the pass-means-at-most-5-tarocks exploit removed. |
| 1.8 | Bermann: no tarock minimum is I; "every queen" now "queens as figures"; `spare-lead` limited to the one position of Be 150-151. |
| 1.9 | Rückhand is the dealer (U 70, W 130, RULES 3.2); the stricter bid applies only to the hand type U names. |
| 1.10 | No years in labels. |
| 2.1, 2.2 | Stage 0 (divergence, bid rate) added before any tournament; the tells gate redefined (sitting window, deal-record features, one-vs-one cross-validated logistic, Normal v Normal null control). |
| 2.3 | Rung-4 bid margin calibrated to the style rule's bid share (stage 0). |
| 2.4 | Bid rates reported; literal v anchored is pending decision 5. |
| 3.1 | Level 1's 12 weights, bid and constants given; a test per level. |
| 3.2 | 1.5 and 2.5 merged now; 3.5 kept; consts become config fields (listed). |
| 3.3 | A readable-not-random check, level 1 v random, level 2 v level 1; playtest is the maintainer's. |
| 3.4 | Side effects of the half levels listed (tests, tournament specs, speed table, golden, saved sittings). |
| 4.2 | `--negligible 1.0`; claims stated one-sided. |
| 4.3 | Null runs planned at the maximum. |
| 4.4 | One-sided claim with the `negligible` bound; families gated; a fresh-seed repeat for any shipped counter. |
| 4.5 | Rung-4 exploits only where stage 0 shows divergence. |
| 4.6 | Staged plan with native timings; stages over about 30 min flagged for approval. |
| 5.1-5.3 | `drawn` authoritative; `bad-record` cases; `reveal` flag in the engine; headerless is `normal`; versions stay; golden re-recorded; abandoned Random reveals on leaving. |
| 6.1 | No `style-tell` group; tells through the brief; the group is pending decision 2. |
| 6.2 | `opponent_unnamed` is written first, test-first. |
| 7.1 | Draft narrative removed; decisions carry date and source; pending decisions list is short. |
| 7.2 | Writers in a table (parameter, value, text, tag). |
