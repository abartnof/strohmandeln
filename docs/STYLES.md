# Skill and style: the opponent's two dials

Design, 17 Oct 2026. No code yet. Supersedes the random slips of
`docs/OPPONENT.md` ("Level dial": rung 1 and the half levels). Sources are cited
as in `research/16-skill-ladder.md`: W = Werner [03-S12], U = Ulmann [03-S14],
Be = Bermann [03-S13], MS = Mayr and Sedlaczek [07-S1]; page numbers are those
of research/16; "row" is a row of its "Contradictions" (section 3).

## 1. Principle

Two independent axes, both fixed for the sitting (the level already is:
`docs/review/settings-audit.md`, the "skill" rows; the record and the Kibitzer's
brief assume one level a sitting).

- **Skill** is what the opponent knows and how far it looks: how much of the
  book it has learned, whether it remembers the cards seen, how many worlds it
  samples, its search budget. Deterministic, consistent; a function of seed and
  view as now. Nothing is ever played at random for the sake of weakness.
- **Style** is a school of play: the advice of one writer where the sources
  disagree, the consensus elsewhere. A style is a fixed set of parameters
  (below), not a draw.

Why: a known tendency is learnable and beatable; a random slip is not. The
sources say the play is a matter of judgement, not rules [16 headline 1: W
p. 135, U p. 72], and disagree in five places [16 section 3], so the
disagreements are the honest material.

### 1.1 What replaces rung 1 and the half levels

Rung 1 (a random legal move) is dropped from the dial; it stays in the code as
the test baseline (`random` policy), not selectable. Levels 2, 3 and 4 are
unchanged (rung 2, rung 3, rung 4, `docs/OPPONENT.md`), so every existing
measurement of them stands. The levels below and between are built from three
knobs, all deterministic:

| Level | Knows (maxims) | Memory | Worlds / budget |
|---|---|---|---|
| 1 | the stage-1 maxims only (`worth`, `cheap`: take a rich trick, pay cheaply) [16 s2] | none | none |
| 1.5 | + the covers off (`off`) | none | none |
| 2 | all of rung 2 (today) | none | none |
| 2.5 | rung 2 | cards seen and voids shown (rung 3's `Knowledge`) | 6 worlds |
| 3 | rung 3 (today) | all | 24 worlds (`rung3::WORLDS`) |
| 3.5 | rung 3 | all | rung 4 with 3 worlds, 8 000 nodes |
| 4 | rung 4 (today) | all | `Budget::default` (10 worlds, 30 000 nodes) |

Level 1 is a beginner's opponent that is weak and readable: it takes a rich
trick, otherwise pays with the cheapest card, and knows nothing else. The order
of maxims is the logical order of the ladder [16 s2], not a teachers' sequence.
The intermediate rows are a first proposal; 5.2 says how the order is checked,
and a row that does not separate from its neighbour is merged (the dial may end
with fewer than seven positions). Rung 3 over rung 2 is already thin (+1.53 +-
0.40 Points a pair, `docs/OPPONENT.md`), so 2.5 may not survive.

## 2. Styles

Dropdown, default **Normal**.

| Id | Label (en / de) | Plays |
|---|---|---|
| `normal` | Normal / Normal | Today's maxims and search, bit for bit: the consensus of the sources [14]. Mostly MS's maxims [07 s1]. |
| `random` | Random / Zufall | One of the writers below, drawn at the sitting's start (2.4). |
| `werner` | After Werner (1891) / Nach Werner (1891) | W's book. |
| `ulmann` | After Ulmann (1899) / Nach Ulmann (1899) | U's book. |
| `bermann` | After Bermann (1894) / Nach Bermann (1894) | Be's book. |
| `ms` | After Mayr and Sedlaczek / Nach Mayr und Sedlaczek | Candidate; ships only if it passes the gate of 5.1. |

The label reads as a school, never as the opponent's name: the opponent stays
"your opponent" / "Ihr Gegner" (`docs/TABLE3D.md` 0.4, check `opponent_unnamed`,
not yet written). A writer's name occurs only as the object of "after" / "nach"
in the style row, the sitting line and the brief; no string has the writer as
speaker or as the opponent ("Werner plays", "you face Bermann"). The check is
extended to assert that. The date is the edition read for the page numbers
(W's 2nd edition is reviewed in 1891 [04]; U's first edition is 1888 [03-S15],
the pages cited are the 2nd, 1899).

### 2.1 What each writer says, and what it changes

Parameters are the names in `rung2.rs` (`Weights`, `BidRule`, `STRONG_TAROCKS`,
`THREAT_TAROCKS`). A multiplier is on the Normal value. All values are
provisional design values: each is the smallest change at which the tells of 5.1
show, with the style's cost (5.3) inside its bound. New maxims are marked "new"
and need a reason key (`docs/MESSAGES.md`).

**Werner** [W 131, 133-135].
- Bid: "3 Honneurs, 6 bis 7 Taroks und eine oder zwei besetzte Damen"; a weakly
  guarded Pagat is not counted (W 131). `min_tarocks` 6; the Pagat counts as a
  sure winner from 7 tarocks (Normal 6).
- Tarocks: "In zahlreichen Fällen wird man sich erst von seinen Taroks befreien
  müssen" (W 135, row 3): `draw_trumps` x1.5, `keep_tarocks` x0.5.
- The contested suit: do not bring it, force the opponent to (W 134-135, row 1):
  `split_suit` x1.5.
- Packets: off as fast as possible (W 134): `cover` x1.25.
- Counting the tarocks (W 133-134, row 2): no change (2.3).
- Tells: leads tarocks early, even with few; never opens a split suit; passes
  below 6 tarocks. Exploit: keep your own tarocks until its are spent
  (`keep_tarocks` x1.5, `draw_trumps` x0.5, cheap follow to a tarock lead); its
  pass tells you it holds at most 5 tarocks, so take up lighter after it
  (`min_tarocks` 4 as dealer after its pass).

**Ulmann** [U 69-78].
- Bid: three well-guarded honours with some queens; four or more
  unconditionally; three weak honours without queens "namentlich in der
  Rückhand" not enough (U 77-78). Rule: bid at once from 4 honours (sure
  thirds 52, any tarock count); otherwise Normal's rule, the dealer (the
  Rückhand; to be confirmed in the text, open question 6) needing 55 sure
  thirds.
- Pagat: cash it with 7-8 tarocks, hunt it with 10-12 (U 74, row 4):
  `STRONG_TAROCKS` 9 (holds from 9); new `hunt-pagat`: with at least 10
  tarocks and no Pagat, lead tarocks first.
- The Mond: "suche man rechtzeitig heimzubringen", keep the smallest tarock to
  bring the opponent to lead (U 74): new `guard-mond`, `keep_tarocks` x1.5 when
  holding the Mond without the Sküs.
- Leads: from a closed run of tarocks, of any height (U 73-74): `solid_run`
  x1.5. Packets off at once, the higher card first (U 71, 75): `cover` x1.25.
- Whether to give up the Prozess is "der Beurtheilung des Spielers überlassen"
  (U 72-73): no change to `split_suit`.
- Tells: tarocks from a run early; the Pagat cashed or held by tarock count;
  the Mond held back with a small tarock. Exploit: against 7-8 tarocks it has
  cashed the Pagat, so no Pagat threat to guard (`THREAT_TAROCKS` 7); against
  10 and more it holds it to the last trick, so keep a high tarock for trick 27
  (`THREAT_TAROCKS` 4); do not lead small tarocks into a Mond it guards.

**Bermann** [Be 145-151].
- Bid: "20 bis 25 Points in Honneurs und höheren Figuren" "unbedenklich"; no
  tarock count is required (Be 145-146). Sure thirds 60 with every queen
  counted, `min_tarocks` 0: it takes up suit-strong, tarock-poor hands Normal
  passes, and passes tarock-rich hands with few points.
- Tarocks recede in value against the suits (Be 146, 150, row 3): `draw_trumps`
  x0.5, `high_trump_lead` x0.5, `keep_tarocks` x1.25.
- The contested suit: "Meist fällt der Gewinn dann jenem Spieler zu, welcher
  zuerst am Ausspielen ist", with the spare low cards to lead (Be 150-151,
  row 1): `split_suit` x0.25, and new `spare-lead`: open a split suit when it
  holds at least 2 spare low cards in it.
- Figures lying in a suit where the opponent's packets show blanks "sind stets
  abzuspielen" (Be 148): `exposed_figure` x1.5 on that condition.
- Counting the suits rather than the tarocks (Be 150, row 2): no change (2.3).
- Tells: opens the contested suit at once; plays a lying figure the moment your
  tops are blank; bids on few tarocks. Exploit: against a tarock-poor taker
  draw its tarocks (`draw_trumps` x1.5, `high_trump_lead` x1.5); where you can,
  do not leave a blank on top of a packet that covers a figure you want to
  keep.

**Mayr and Sedlaczek** [MS], candidate.
- Pagat a sure winner ("meist heimzubringen"); hunt, and a silent ultimo, with
  more than 11 tarocks and a long suit; cash with 7 or fewer (MS, rows 4, 5).
- The contested suit: whoever opens ends worse (MS, row 1): `split_suit` x1.5.
- Mondfang: keep one or two small tarocks (MS): Ulmann's `guard-mond`.
- Bid: sure winners 20-25 points, 6-7 tarocks, a tight 20 risky (MS): sure
  thirds 60, `min_tarocks` 6, the Pagat always counted.
- Expected: Normal's maxims are chiefly MS's [07 s1], so this style differs
  from Normal in the bid, the Pagat and the contested suit only, and from
  Ulmann in the contested suit and the Pagat's sureness. The gate (5.1) will
  probably merge it into Normal; Normal's label then says "the consensus,
  mostly Mayr and Sedlaczek" and no fourth writer is offered.

### 2.2 Distinctive enough?

The writers say little: W "ist nur wenig zu sagen" (p. 133), U "läßt sich nicht
geben" (p. 72), W "keine allgemeinen Regeln" (p. 135) [16 headline 1]. Five
rules are shared by two or more of them [16 headline 3] and belong to Normal.
What differs: Be most (bids by points not tarocks, opens the contested suit,
tarocks recede), W next (the 6-tarock floor, sheds tarocks, forces the
opponent to open), U least from Normal (the Pagat thresholds, the Mond guard).
Expected near-twins: U and MS, and U and Normal. The gate of 5.1 decides what
ships; Ulmann ships only if his tells separate.

### 2.3 What is not a style axis

- **Counting tarocks or suits** [16 row 2]: every memory level remembers every
  card; a style that forgot would be a nerf in disguise, and memory is skill.
  Recorded as not modelled.
- **Reckless / cautious / loves declarations** (the maintainer's first
  examples): no writer advocates a looser bid than 20-25 points, and U calls
  three weak honours "nicht hinreichend" (77-78); the sources agree to delay
  an announcement (W 135, MS) [16 `announce`]. A "reckless" or
  "declaration-loving" personality would be invented, so none is offered.
  "Cautious" is Werner's 6-tarock floor and "bold" is Bermann's few-tarock
  take-up: styles with a source.
- **The feint**: the sources call it worn out [U 76, Be 149, MS]; rung 4 models
  no reader [16 s1].

### 2.4 Random

At the sitting's start `new` draws one of `werner`, `ulmann`, `bermann` (and
`ms` if shipped; never `normal`, the default and the baseline) from a hash of
the seed and the label `style`. The record carries `style: "random"` and
`drawn: "<id>"`, so a restored sitting plays the same style and replays.
Hidden in the interface until the sitting's end: no string, tell line or brief
bullet names it; the sitting line reads "Style: Random (shown at the end)".
The end-of-sitting review and brief name it, say what the style does and give
its counter (section 4). The file is not hidden: an exported record carries
`drawn` (it is also recomputable from the seed); the game does not pretend
otherwise.

Before the end the after-deal brief may say what was seen, as facts without
the name ("it led a tarock first in 4 of 5 deals"), so the player can guess:
open question 3.

## 3. Skill by style: which combinations are thin

Every style at every skill is offered. Where a style barely shows, the dialog
line says so.

| Skill | What shows |
|---|---|
| 1, 1.5 | Only the maxims known. At 1 a style changes the bid and the weights of `worth` and `cheap` only; at 1.5 the packet weights too. Werner's and Bermann's leads are alike below 2. |
| 2 to 3 | Everything: rung 2's maxims carry the weights, and rung 3's sampler keeps them (`rung3::score_cards` takes `Weights`). |
| 3.5, 4 | Rung 4 plays the best sampled margin; the weights decide only within a third of a third of the best (`rung4::best_card`). So the style shows in the bid (rung 4 prices a bid by `bid_advantage`; the style shifts the margin, in Points) and by a tolerance band for the maxims (2 thirds, to be set against 5.3). At the top the play is only faintly styled. |

If the exploit at 3.5 and 4 does not clear its bound (5.1), the dialog says "at
this skill the style shows mostly in the bidding". It is not removed (any style
at any skill, the maintainer's request).

## 4. Readability

- **The sitting dialog**: a row "Style" under the skill dial, one line under
  the selection (key `style.<id>.line`), e.g. Bermann: "Counts points, not
  tarocks: takes up on few tarocks and opens the contested suit." Under Random:
  "A school drawn for you; named when the sitting ends."
- **The table's menu** "This sitting" line: "Skill 3, after Bermann (1894)";
  Random until revealed: "Skill 3, Random".
- **The talk** (`docs/PHRASES.md`): a new group `style-tell` per style, at most
  once a deal, said when the trait acts (a first tarock lead, a pass under the
  floor, a lying figure played), never from the hidden hand. Lines are the
  writers' own maxims as quoted in research/16 and 03, each cited in the bank,
  about play and luck only (kindness rule, CLAUDE.md): e.g. W 134 "Die größte
  Rolle spielt das Impassiren"; U 75 "möglichst rasch die aufliegenden Karten
  abspielen"; Be 150 "den Stand der Farben zu kennen". The table's ordinary
  chatter (`research/05*`) stays shared. Under Random the `style-tell` group is
  silent (a quotation would give the school away); the shared bank speaks. The
  speaker is always the opponent, unnamed.
- **The after-deal brief** (`docs/TUTOR.md`): a bullet "Their style" with the
  tendency counted from the record (`tutor.style.<id>.seen`: "it led a tarock
  first in 3 of 3 deals") and the counter (`tutor.style.<id>.counter`). The
  counter ships as a rule only when its exploit passes 5.1 (as TUTOR M5: a
  rule that fails is told as its fact); until then, the fact alone.
- **The end-of-sitting brief** names the school (Random included) and how the
  sitting went against it.

## 5. Measurement

Per CLAUDE.md: mirrored pairs (each deal twice, seats swapped), unit the Points
a policy takes from the other over a pair of 3-Tour sittings, sequential with
O'Brien-Fleming boundaries; the comparison, batch size, maximum batches and
bounds are fixed before the first batch. Tool `tournament`
(`docs/OPPONENT.md`), first seed 3 000 000 (fresh), `--negligible 0.5`. The two
axes are measured separately.

### 5.1 Style, at a fixed skill: tells, then exploits

**Tells and the gate (cheap; no tournament).** Per style, tell statistics per
deal over 200 deals at skill 2.5 and 4: share of leads that are tarocks while
holding at least 5 tarocks; bid rate, and bid rate with at most 5 tarocks;
split-suit opens; the Pagat played before trick 20 with 7-8 and with 10 or more
tarocks; lying figures played while the opponent's tops are blank; the Mond
guard. Gate: a classifier on a 2-deal window separates each style from every
other (Normal included) with AUC at least 0.8; a style that fails is merged
into the nearest or dropped, its parameters not shipped. The bid rate of every
style is reported too (the book rule bids in about 4-7% of hands, `research/07`
5.4; Normal's lower 45/5 rule is the anchor, so styles are Normal-relative:
open question 5).

**Exploits.** For each shipped style S and skill L in {2.5, 4}:
- Arm N: Normal at L against S. Arm X: the exploiting policy (Normal at L with
  the counter's parameter changes of 2.1 only) against S at L. Same deals.
- Statistic D = (Points X takes from S) - (Points N takes from S), per pair of
  3-Tour sittings; 4 sittings a pair. Claim: D > 0.
- Plan: batches of 100 pairs, at most 6 looks. Two-sided boundaries z_k = 2.05
  * sqrt(6/k): 5.02, 3.55, 2.90, 2.51, 2.25, 2.05 (the constant of the 3-vs-2
  rerun's 2.90 at look 3 of 6; the tool computes the exact value). Stop at the
  boundary, reporting the interval at that look. "Too small to matter": stop
  when the 95% interval's upper end is below +0.5 Points a pair (futility on).
  No unadjusted peeking.
- Result: the counter ships as a rule if the lower end is above 0 at stopping;
  if D is too small at 4 and not at 2.5, the style's dialog line says so (3).

**CPU** (2 vCPU; `docs/OPPONENT.md`: rung 3 median 0.8 ms, about 0.4 s a
sitting; rung 4 about 3.5 s a sitting, `docs/TUTOR.md` 11). A pair is 4
sittings: rung 3 about 2 s, rung 4 about 14 s. Worst case 600 pairs: rung 3
about 20 min a style, rung 4 about 2.3 h; for three styles about 7 h CPU,
3.5 h wall on two cores; expected a third of that (the 3-vs-2 rerun stopped at
look 3 of 6).

### 5.2 Skill, at a fixed style: the order must hold

For Normal, each adjacent step of the dial (6 comparisons), higher against
lower, mirrored, 100 x 6 batches, same boundaries: the higher must take more
(lower end above 0); a step that stops "too small" is merged. Then, for each
shipped writer style, the steps 1 v 2, 2 v 3, 3 v 4. Anchors: 3 v 2 (+1.53 +-
0.40) and 4 v 3 (+6.73 +- 1.16) stand for Normal; the old "2 vs 1" (+18.36,
over random) is replaced by the new level 1 against level 2. CPU worst case:
steps without rung 4 take minutes; each step with rung 4 about 70 min at 600
pairs (2 sittings, two seats at 3.5 s); about 4 h for Normal, 3 h for the
writers.

### 5.3 The style's cost (so the dials stay independent)

S against Normal at the same L (2.5 and 4), mirrored: the mean must not exceed
the cost of one dial step, and a style at L+1 must still beat Normal at L (lower
end above 0). A style stronger than Normal at the same skill is an upgrade, not
a style: its parameters are pulled back. Worst case rung 4 about 1.2 h a style.

**Total** worst case about 20 h CPU (10 h wall), expected about a third. The
maintainer approves before any run (CLAUDE.md); this is the estimate.

## 6. What changes

- `docs/OPPONENT.md`: the level table of 1.1; the slip paragraph removed; the
  "2 vs 1" row replaced; a Styles section pointing here.
- `docs/PROTOCOL.md`: `new` takes `style` (default `normal`; unknown is
  `bad-input`); state and export carry `style` and, for Random, `drawn`
  (`null` in state until `sitting_over`); a record without `style` is `normal`;
  `strohmandeln_brief_*` headers read `level n` then `style s`; the hint is
  unchanged (rung 4, Normal, "whatever the opponent's level", and now whatever
  the style).
- `docs/TUTOR.md`: the advisor stays rung 4 Normal. Re-run: M3 (what the
  strongest rung meets) once per shipped style at skill 2.5, 240 deals each
  (about 15 min a style), to see whether chance rates move beyond the window the
  `LADDER` constants were fitted on (54 deals against unstyled play); M1, M2
  and M4 to M8 are rung 4 against itself or Normal and stand. The "next
  opponent up" suggestion names a level only. A brief bullet "Their style"
  (section 4).
- `docs/TABLE3D.md`: 7.3's label under the dial says what the level knows; a
  new 7.3a Style dropdown; 3.5.1 lists it; 7.7.3 prefs record gains `style`
  (version bumped); the "This sitting" line; `opponent_unnamed` extended (2).
- `docs/PHRASES.md` (generated from `tools/phrases.py`): the `style-tell`
  groups. `docs/MESSAGES.md`: reason keys for `hunt-pagat`, `guard-mond`,
  `spare-lead`; `tutor.style.*`.
- Strings en/de: `style.<id>.name`, `.line`, `tutor.style.<id>.seen`,
  `.counter`, the dropdown label, the Random reveal line.
- Settings: `docs/review/settings-audit.md`: style joins skill as "New sitting
  only; read-only in the menu"; no mid-sitting change.
- PLAN (orchestrating session): browser seeds depend on the opponent at
  `?level=1` (random until now), so a rescan is needed.
- Stays: the hint always rung 4 Normal; levels 2, 3 and 4 and their
  measurements; rung 1 as a test baseline; replay from the record.

## 7. Dropped and why

The first draft's personalities (trump-leader, cautious, Pagat hunter,
contested-suit waiter) are not kept: each is a trait one of the writers already
holds (Ulmann hunts the Pagat, Werner sheds tarocks and waits, Mayr and
Sedlaczek wait), and "reckless" and "loves declarations" have no source (2.3).
They would add nothing the schools do not, and would be invented.

## 8. Open questions for the maintainer

1. Build the four writers and let the gate (5.1) drop near-twins (probably Mayr
   and Sedlaczek into Normal), or offer all four regardless?
2. Random draws from the writers only (proposed), or from Normal too?
3. May the pre-reveal brief state observed tendencies as facts (proposed), or
   nothing until the end? A "guess the school" chip before the reveal?
4. Level 1 as "knows only the stage-1 maxims" replaces random play: acceptable
   as a beginner's opponent?
5. Writer bid thresholds anchored to Normal (45/5) and moved in the writer's
   direction (proposed), or literal (4-7% bids, most deals thrown in)?
6. Is the Rückhand the dealer? (RULES 3.2: the dealer speaks second; Ulmann's
   "Rückhand" is to be read in the text before the parameter is fixed.)
7. At skill 3.5 and 4 a style shows mostly in the bid. Offer it anyway with a
   dialog note (proposed), or hide the dropdown at the top skills?
8. Compute: about 20 h CPU worst case; approve, or start with rung 3 only?
9. Dates in the labels (the edition read) as proposed?
