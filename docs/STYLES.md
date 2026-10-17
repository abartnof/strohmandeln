# Skill and style: the opponent's two dials

Design, 17 Oct 2026 (maintainer's request: a personality, not a nerf; a style
dropdown; the period writers as schools). Revised the same day after
`docs/review/styles-spec.md`. No code yet. Supersedes the random slips of
`docs/OPPONENT.md` ("Level dial": rung 1 and the half levels).

Sources are cited as in `research/16-skill-ladder.md`: W = Werner [03-S12], U =
Ulmann [03-S14], Be = Bermann [03-S13], MS = Mayr and Sedlaczek [07-S1]; "row"
is a row of research/16 section 3. Every parameter is tagged: **T** the text
states it, **I** one inferential step from the text (the step is said), **E**
emphasis or design (no text gives it). A writer style uses T, and I where said;
never E. A temperament (2.2) may use E and claims no history.

## 1. Principle

Two independent axes, both fixed for the sitting (`docs/review/settings-audit.md`:
the record and the Kibitzer's brief assume one level a sitting).

- **Skill**: what the opponent knows and how far it looks (the book it has
  learned, memory of the cards seen, sampled worlds, search budget).
  Deterministic; a function of seed and view; nothing is played at random to be
  weak.
- **Style**: a fixed parameter set, the school of a writer or a temperament.

A known tendency is learnable and beatable; a random slip is not.

### 1.1 The dial

Rung 1 (random) leaves the dial; it stays as the test baseline (`random`
policy), not selectable. Levels 2, 3, 4 are unchanged (rung 2, 3, 4,
`docs/OPPONENT.md`), so their measurements stand. Levels 1.5 and 2.5 are
removed: 1.5 adds only `cover` (0.8 thirds) to level 1, and rung 2's maxims
never read `opp_void`, so 2.5 (rung 2 with memory and 6 worlds) is rung 3 with
fewer worlds, a difference of sampling noise on a step already +1.53 +- 0.40
(`docs/OPPONENT.md`). They are merged now, not after a run. Dial: 1, 2, 3, 3.5,
4.

| Level | Plays |
|---|---|
| 1 | Rung 2 with the stage-1 maxims only (`worth`, `cheap`: take a rich trick, pay with the cheapest; research/16 s2). |
| 2, 3, 4 | Today's rungs 2, 3, 4. |
| 3.5 | Rung 4 with 3 worlds and 8 000 nodes (`Budget`); the gap 4 v 3 (+6.73 +- 1.16) is the only wide one to fill. |

**Level 1 in full** (E: no text gives a beginner's play; the order is the
ladder's logic, not a teachers' sequence):

| `Weights` | burden | cover | exposed_figure | split_suit | keep_packet | hold_pagat | cash_king | draw_trumps | solid_run | keep_tarocks | high_trump_lead | coverless_suit |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| level 1 | 3.0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

`burden` is the finesse half of `worth` (do not spend an honour on a poor
trick); the rest are zero, so a lead is the cheapest card and a tarock costs
`spend`. Bid: take up only with at least 60 sure thirds (20 points), any tarock
count (the `bid` skill presupposes `worth`, research/16 s2). `STRONG`, `WEAK`,
`THREAT` are unused (every weight that reads them is zero). A test per level:
a deterministic function of seed and view, as the `rung2` tests are.

**Level 1 is readable, not random: a check.** Over 1 000 views, the card chosen
is the same under five seeds in at least 95% of them (seeds break ties only). A
sanity run: level 1 against random (clear and large; replaces the old +18.36
of `2 vs 1`); level 2 against level 1 (the first dial step). Whether it suits a
human beginner is the maintainer's playtest, not a tournament.

**Code changes this needs** (spec for the implementer): `STRONG_TAROCKS`,
`WEAK_TAROCKS`, `THREAT_TAROCKS` become fields of the config (rung 2 fires
`draw_trumps` from `STRONG - 1`, rung 3 from `STRONG - 2`; rung 2's split band
is 0.3-0.7, rung 3's 0.35-0.65; Normal keeps all of these exactly); `sure_thirds`
takes the Pagat's guard (6) and the queen rule as fields; `BidRule` gains
fields as 2.1 needs; `tournament`'s `parse_spec` takes `style`. Test: Normal
at every level reproduces today's decisions on the recorded seeds.

**Side effects.** `Level::from_f64` and `tournament` specs: a half now snaps to
the nearest dial position, ties down (1.5 is 1, 2.5 is 2; 3.5 stays), where it
meant "rung L+1 with slips". Changed: every `2.5`/`1.5` string and test
(`the_rules_of_thumb_beat_random_play`, the `tournament.rs` tests), the
`docs/OPPONENT.md` speed table rows, and the golden file for level 1. Saved
sittings at 1 or x.5 replay (the opponent's moves are recorded, PROTOCOL) but
continue under the new meaning.

## 2. Styles

Dropdown, in this order: **Normal** (default), **Random**, then the writers that
survive the gates (3), then the temperaments that survive. Candidates:

| Id | Label (en / de) | Kind |
|---|---|---|
| `normal` | Normal / Normal | Today's maxims and search, unchanged; the consensus [14], mostly MS's maxims [07 s1]. |
| `random` | Random / Zufall | One of the survivors, drawn at the sitting's start (2.3). |
| `bermann` | After Bermann / Nach Bermann | Writer. |
| `ulmann` | After Ulmann / Nach Ulmann | Writer. |
| `ms` | After Mayr and Sedlaczek / Nach Mayr und Sedlaczek | Writer. |
| `werner` | After Werner / Nach Werner | Writer; expected to fall out (2.1). |
| `bold` | Bold / Verwegen | Temperament. |
| `cautious` | Cautious / Vorsichtig | Temperament. |
| `hunter` | Pagat hunter / Pagatjäger | Temperament. |

No years in labels (W's 2nd edition is undated, 1884-1906 [03-S12]; MS is a web
page, 2002 copy [07-S1]). The label reads as a school or a character, never as
the opponent's name: the opponent stays "your opponent" / "Ihr Gegner"
(`docs/TABLE3D.md` 0.4, check `opponent_unnamed`). A writer's name occurs only
as the object of "after" / "nach" (style row, sitting line, brief); no string
has a writer as speaker or as the opponent. The check `opponent_unnamed` is in
the `NOT_YET` set (`web3d/test/browser_suite.test.js`): it is written first,
test-first, with these cases, before the style strings exist.

### 2.1 The writers (T/I only)

Where the writers agree, the maxim is Normal's (research/16 headline 3). A
multiplier on a shared maxim would be E and is not used. So a writer differs
from Normal only where the text does. Parameters are names in `rung2.rs`.

| Writer | Parameter | Value | Text | Tag |
|---|---|---|---|---|
| Bermann | Bid | 60 sure thirds (20 points), queens counted as figures, no tarock minimum | Be 145-146 "20 bis 25 Points in Honneurs und höheren Figuren" "unbedenklich"; no tarock count is stated; "no minimum" from the tarocks' "Nebenrolle" (146) | T; I (the tarock minimum, one step) |
| Bermann | Exposed figure | no `low_shown < 2` exception: play a lying Dame or Cavall always when the opponent's packets show blanks ("leere Blätter" read as blank pips, as Be 151 "leeren Karten") | Be 148 "so sind sie stets abzuspielen"; Normal keeps the exception (U 74, MS agree on it) | T; the reading stated |
| Bermann | Mond reserve (`guard-mond`: `keep_tarocks` up when holding the Mond without the Sküs) | keep one or more small tarocks | Be 151-152 | T |
| Bermann | Cash an unescorted Mond (`cash-mond`, new) | play it at a fitting chance | Be 151-152 "stets bei passender Gelegenheit einstechen" | T |
| Bermann | Contested suit (`spare-lead`, new) | open a suit wholly split, a fork each (king and cavall against queen and jack), holding spare low cards | Be 150-151: this position only | T, narrow |
| Ulmann | Bid | 4 or more honours (Troule cards and kings; the Pagat only with 6 tarocks, U 78): bid at once, any tarocks. "3 weak honours and no queen, especially as dealer" is not modelled: such a hand is at most 39 thirds and Normal refuses it already | U 77-78 | T |
| Ulmann | Pagat hunt (`draw_from`, the tarock count at which `draw_trumps` fires) | 10 (Normal 7 in rung 2, 6 in rung 3) | U 74: no Pagat, 10-12 tarocks, a closed run: hunt | T |
| Ulmann | Pagat cash | with 7-8 tarocks, at the first chance | U 74; equals Normal at 8 | T |
| Ulmann | Cash an unescorted Mond | as Bermann | U 74 "suche man rechtzeitig heimzubringen" | T |
| MS | Bid | 60 sure thirds, 6 tarocks, the Pagat always counted | MS "Wie stark ist dein Blatt" | T |
| MS | Pagat hunt | `draw_from` 12 | MS "mehr als 11 Tarock" | T |
| MS | Mond reserve (`guard-mond`) | one or two small tarocks | MS "Mondfang" | T |
| Werner | Bid | take up with 3 honours, 6 or more tarocks and a guarded queen, otherwise not | W 131 gives this as a sufficient condition; read as the rule | I (sufficient read as necessary) |

Werner's other advice is shared (shed tarocks: W 135 = U 74 = MS; the contested
suit: W 134-135 = MS; the Pagat weakly held is not counted: Normal's) and gives
no parameter. Werner's rule above lies inside Normal's 45/5 bids, so Werner is
expected to fall out at stage 0 (3). Tarocks recede (Be 146, 150) is Be alone
and Be 151 still draws the Pagat out when strong, so no `draw_trumps` change is
claimed for Bermann.

Tells and exploits (counters are E; they are the player's, not the opponent's):
- **Bermann**: bids on few tarocks; plays a lying figure at once when your tops
  are blank (it would hold it back from Normal where you show two low cards);
  keeps small tarocks and plays its Mond early. Exploit: do not leave a blank
  on top of a packet over a figure you mean to keep; do not lead small tarocks
  into the Mond it reserves for.
- **Ulmann**: leads tarocks only from 10; cashes the Pagat with 7-8; takes up
  at once on 4 honours. Exploit: its tarocks are rarely led before the
  threshold, so keep yours and let it open the suits; against a taker on 4
  honours, defend and do not contest.
- **MS**: bids rarely and then on 60 sure thirds with 6 tarocks; leads tarocks
  only from 12. Exploit: its pass is weak information; take up lighter after
  it; do not wait for tarocks it has not got.
- **Werner**: as MS but with the honours condition; a pass tells little (a hand
  with 8 tarocks and one honour passes as well).

### 2.2 Temperaments (E allowed; the table's own characters, not any writer)

Each is a parameter set with tells and an exploit. Cost and readability are
measured as for the writers (3).

| Temperament | Parameters | Tells | Exploit |
|---|---|---|---|
| Bold: takes up on thin hands | Bid 36 sure thirds, 4 tarocks; `split_suit` x0.5 (opens contested suits); `exposed_figure` x1.5 | many bids, often losing; opens the contested suit early | let it open and keep your honours behind; bid only on a real hand and make it pay for thin ones (`bid` threshold up 10 thirds against it) |
| Cautious: takes up only on strong hands, waits for you to open contested suits | Bid 60 sure thirds, 6 tarocks; `split_suit` x1.5; `keep_packet` x1.5; `STRONG_TAROCKS` 10 (cashes the Pagat sooner) | passes most hands; never opens a split suit; packets kept | take up lighter after its pass; open the contested suit last yourself only when you hold the spare cards, else wait too |
| Pagat hunter: plays for the ultimo whenever its tarocks allow | `STRONG_TAROCKS` 6 (holds the Pagat from 6); `draw_from` 8; bid 36 sure thirds, 7 tarocks | leads tarocks from the start; the Pagat held to trick 27; takes up on tarock-rich hands | keep a high tarock for trick 27 (`THREAT_TAROCKS` 4); lead suits, its suits are thin |

The temperaments answer "reckless, cautious, loves declarations" (maintainer,
17 Oct 2026). **Loves declarations** is not offered: no source states a
positive preference; an announcement is a single honour call with a deadline,
worth exactly one Point times the multiplier, and the sources advise delaying
(W 135, MS) [16 `announce`]; announcing early changes only what the opponent
knows, a tell worth no parameter set of its own. **Cautious** overlaps MS's
bid; if the gate merges them, Cautious is kept (it has the tells and MS does
not).

### 2.3 Random

At the sitting's start `new` draws from the survivors of every gate (writers and
temperaments, never `normal`) by a hash of the seed and the label `style`. The
record carries `style: "random"` and `drawn: "<id>"`; **`drawn` is
authoritative** (the hash is used only when it is absent, since the pool may
change). Hidden in the interface until the sitting's end: no string, brief
bullet or talk line names it; the sitting line reads "Style: Random (shown at
the end)". The export file and a saved sitting carry `drawn`; the game does not
pretend otherwise. Reveal: the end-of-sitting review and brief name it and say
what it does and the counter; a Random sitting abandoned by a new sitting
reveals on leaving (a chip). Before the end the after-deal brief may state
observed tendencies as facts without the name ("it led a tarock first in 4 of 5
deals"): pending decision 3.

## 3. Skill by style

Every style at every skill is offered. At level 1 a style changes the bid and
the weights of `worth`/`cheap` only (the other weights are zero there), so
the temperaments' and writers' leads are alike; the bid shows. At 2 and 3 all
parameters act (rung 3's sampler keeps the `Weights`). At 3.5 and 4, rung 4
plays the best sampled margin, and weights decide only within 0.34 thirds of
it (`rung4::best_card`); so the style acts in the bid and in a tolerance band
for the maxims (2 thirds; set against the cost in 4.3). Rung 4 bids by
`bid_advantage > bid_margin`, not by `BidRule`: a style's rung-4 bid margin is
the Points advantage at which Normal's rung-4 sample bids the same share of
1 000 hands as the style's own rule does (calibrated in stage 0). If a style's
play diverges little at rung 4, the dialog line says "at this skill the style
shows mostly in the bidding". It is still offered.

All writer bids are literal text thresholds on Normal's sure-winner scale; most
bid less than Normal (Normal 18% of random 15-card hands; W 12%, U 14%, Be 6%,
MS 3% by the review's rough count), so a writer is probably weaker at rungs 2-3
by bidding alone. Stage 0 reports the bid rate of every style; the cost (4.3)
measures it. Whether to anchor to Normal instead is pending decision 5.

## 4. Measurement

Per CLAUDE.md: mirrored pairs, unit the Points over a pair of 3-Tour sittings,
O'Brien-Fleming boundaries (`stats.rs` `obf_constant(6)` = 2.053, so z_k =
2.05*sqrt(6/k): 5.02, 3.55, 2.90, 2.51, 2.25, 2.05), fixed before the first
batch: batches of 100 pairs, at most 6 looks, `--negligible 1.0` (about two
thirds of the thinnest dial step, 3 v 2 = +1.53; the tool stops when the whole
95% interval is inside +-1.0, which fires near 190 pairs at sd 7, 380 at sd
10; at 0.5 and 600 pairs it could never fire). First seed 4 000 000 (fresh).
Axes are measured separately. The tool's boundaries are two-sided; each claim
below is **one-sided** (stated), and a crossing in the other direction is
reported as a failure of the claim. A claim that clears is repeated once on
fresh seeds (the 3 v 2 rerun precedent); about 30 sequential tests at 5% would
otherwise give about 1.5 false clears, so the families are gated, below.

### 4.1 The staged plan

Timings are from the existing release `tournament` binary, native, 6-pair probe
under load (an upper bound): `3 2` 0.05 s a pair; `4 3` 2.6 s; `4 4` 2.95 s. They
are to be re-measured on an idle machine before approval. (The wasm median times
495 decisions of `docs/OPPONENT.md` overstates rung 3 about 40x.)

| Stage | Question | Method | Cost (worst) | Gate |
|---|---|---|---|---|
| 0 | Do the styles differ in play, and how much do they bid? | the share of decisions where the styled choice differs from Normal's on the same views, the bid rate, and the rung-4 bid margin calibration; 1 000 deals at skills 3 and 4; at rung 4 also the share of decisions inside the 0.34 band | rung 3 seconds; rung 4 about 4 min a style; about 30 min in all | drop a style below about 1% divergence |
| 1 | Readable: the tells gate (4.2) | the same 1 000 deals | no extra | drop or merge a style below AUC 0.8 |
| 2 | Rung-3 exploits (D) | 100 x 6 pairs, two arms; 1 200 pair-runs x 0.1 s | about 2 min a style | ship a counter only if clear, with D's lower end above 1.0, and a fresh-seed repeat |
| 2b | Style cost; dial steps at rung <= 3 | S v Normal at 3; Normal 1 v 2, 2 v 3 | minutes in all | dial rows that do not separate are merged |
| 3 | Dial through rung 4 | Normal 3 v 3.5, 3.5 v 4: 2 x 600 x 3 s | **1 h** | merge rows that fail |
| 4 | Rung 4, only for styles that diverge at rung 4 in stage 0 | exploit (6 s a pair) and cost (3 s a pair) | **1.5 h a surviving style** | none |
| 5 | Tutor M3 once per survivor at skill 3 | 240 deals | 15 min a style (the TUTOR figure, unverified) | |

Null runs cost the maximum: a real effect stops early (3 v 2 stopped at look 3
of 6), a null stops only on futility. The likely exploit result is null for
styles that diverge little, so the worst case is the plan, not a third of it.
Worst total about 6 h CPU (3 h wall on two cores: the tool is single-threaded,
two runs fit two vCPUs), with up to seven candidates; expected about 2 h if most
fall at stages 0-1. **Needs the maintainer's approval (CLAUDE.md, over about
30 min CPU)**: stage 3 (1 h), stage 4 (1.5 h each). Stages 0-2b together (about
30 min) are announced, with the idle-machine timings in hand, before running.

### 4.2 The tells gate (readable within a sitting)

Window: one sitting (6 deals). Features, counted per deal from the deal record
(every card is public after a deal, so hidden hands are fair): tarock leads
while holding at least 5 tarocks, tarocks led before trick 5, bid taken, bid
taken with at most 5 tarocks, Pagat played before trick 20 (by own tarock
count 7-8, 10 and more), lying figures played while the opponent's tops are
blank and while two low cards show, split-suit opens, Mond played early. Pooled
counts over the window; a one-vs-one logistic regression per pair (including
Normal), 5-fold cross-validated; **pass** = AUC at least 0.8 against Normal and
against the nearest style. Null control: Normal v Normal on disjoint seeds must
read about 0.5 (the classifier's overfit). The review's reconstruction shows a
2-deal window cannot reach 0.8 on bids alone (at most 0.63 pairwise; 0.72-0.81
at 6-12 deals only for Normal v Bermann and Normal v MS), so the window is a
sitting.

### 4.3 Exploits, dial order, style cost

- **Exploit.** For each shipped style S and skill L (3; and 4 only where stage 0
  shows divergence at rung 4): arm N = Normal at L against S; arm X = Normal at
  L with the counter's parameter changes against S; same deals;
  D = (Points X takes from S) - (Points N takes from S) per pair. Claim
  (one-sided): D > 0, clear when the lower end is above the `negligible` bound
  1.0 at the stop. Counters change weights and constants only, so they cannot
  act at rung 4 outside the band (hence the stage-0 gate).
- **Dial order.** Normal: 1 v 2, 2 v 3, 3 v 3.5, 3.5 v 4, higher against lower
  (one-sided: higher takes more). Anchors: 3 v 2 +1.53 +- 0.40 (rerun, 300
  pairs), 4 v 3 +6.73 +- 1.16. A step that stops "too small" is merged. For each
  shipped style the same at the stage-end levels 1 v 2, 2 v 3, 3 v 4.
- **Style cost.** S v Normal at the same L: the style must not take more than
  Normal's step would give (a style that beats Normal is an upgrade, not a
  style: its parameters are pulled back), and a style at the next level up
  must still beat Normal at L (one-sided, lower end above 0).

## 5. Readability: how the player learns the style

- **The sitting dialog**: a row "Style" under the skill dial, with one line under
  the selection (`style.<id>.line`), e.g. Bermann: "Takes up on points, not
  tarocks, and plays a lying figure at once." Random: "A style is drawn for you
  and named when the sitting ends."
- **The menu's "This sitting" line**: "Skill 3, after Bermann"; Random until
  revealed: "Skill 3, Random".
- **The after-deal brief** (`docs/TUTOR.md`): a bullet "Their style" with the
  tendency counted from the record (`tutor.style.<id>.seen`: "it took up on 3
  of 3 deals with 4 tarocks or fewer") and the counter
  (`tutor.style.<id>.counter`). The counter ships as a rule only if its exploit
  passes 4.3 (as TUTOR M5); until then the fact alone. The end-of-sitting brief
  names the style, Random included.
- **The talk**: no new group. The opponent's lines stay as in
  `docs/PHRASES.md` (research/05*); the tells come through the brief, not
  through speech. A `style-tell` group of writers' maxims would come from the
  rulebooks (research/03, 16), not 05, which CLAUDE.md fixes as the source of
  the opponent's lines, and a quoted maxim makes the writer the speaker: it
  needs the maintainer's word (pending decision 2) and would be anonymous
  maxims, the citation only in the bank, silent under Random, the research/16
  translations marked as the agent's own.

## 6. What changes

- `docs/OPPONENT.md`: the dial of 1.1 and level 1's table; the slip paragraph
  removed; the "2 vs 1" row replaced; the speed table rows re-measured; a
  Styles section pointing here.
- `docs/PROTOCOL.md`: `new` takes `style` (default `normal`; an unknown value is
  `bad-config`, field `style`). State and export carry `style` and, for Random, `drawn`
  (`null` in state until `sitting_over`). Import: an unknown `style`, a
  `drawn` outside the pool, or `drawn` with a non-random style is `bad-record`.
  A record or a brief header without `style` is `normal`. `protocol: 1` and
  `record.version: 1` stay (the opponent's moves are recorded). The brief entry
  points gain a `reveal` flag the end-of-sitting call sets; the engine, not the
  table, withholds the drawn id of a Random sitting before it. The golden file
  at level 1 is re-recorded. Hint: unchanged, rung 4 Normal, whatever the level
  and the style.
- `docs/TUTOR.md`: the advisor stays rung 4 Normal. Re-run only M3 per
  surviving style at skill 3 (stage 5); M1, M2, M4-M8 are rung 4 against itself
  or Normal and stand. "Next opponent up" names a level only. The bullet "Their
  style" (section 5).
- `docs/TABLE3D.md`: 7.3's label says what the level knows; a new 7.3a style
  dropdown; 3.5.1 lists it; 7.7.3's prefs record gains `style` (version
  bumped); the menu line; `opponent_unnamed` written, then extended (2).
- `docs/MESSAGES.md`: reason keys `hunt-pagat` (the `draw_from` rule),
  `guard-mond`, `cash-mond`, `spare-lead`; `tutor.style.*`. Strings en/de:
  `style.<id>.name`, `.line`, `tutor.style.<id>.seen`, `.counter`, the dropdown
  label, the reveal line.
- Settings: `docs/review/settings-audit.md`: style joins skill, New sitting only,
  read-only in the menu.
- PLAN (orchestrating session): browser seeds depend on the opponent at
  `?level=1`, so a rescan is needed.
- Stays: the hint always rung 4 Normal; levels 2-4 and their measurements;
  rung 1 as a test baseline; replay from the record.

## 7. Corrections owed to research/16 (not edited here)

- Section 1 (the "parked" Mond row) merges two sentences of U 74; the
  small-tarock reserve is tied to the Mond only in Be 151-152 and MS.
- Section 3 lacks a sixth contradiction: Be 148 "so sind sie stets abzuspielen"
  (blank packets) against the exception of U 74 and MS (play unless two low
  cards show); section 2 `off` says they agree.
- Row 3 is Be against W, U and MS: W 135, U 74 and MS agree that tarocks are led
  or shed before a suit is opened.
- Row 2 is not a split: W 133-134 and U 76 also ask for the suits.

## 8. Decisions pending

1. Which survivors ship (gates 3 and 4.1); expected, from the texts: Bermann,
   Ulmann and MS differ at least in bid and `draw_from`; Werner falls out.
2. The `style-tell` talk lines: allowed, or tells only through the brief
   (proposed)?
3. Under Random, may the pre-reveal brief state observed tendencies as facts
   (proposed)? A "guess the style" chip?
4. Level 1 as defined in 1.1 as the beginner's opponent (a maintainer playtest).
5. Writer bids literal (proposed; stage 0 reports the rates) or anchored to
   Normal's bid rate?
6. Compute approval for stages 3 and 4 (4.1) after the idle-machine timings.

## 9. Results

### Stage 0, 17 Oct 2026 (binary before the review fixes of `docs/review/styles-engine.md`)

`stage0 --deals 1000 --seed 5000000 --skills 3,4 --calib-hands 1000 --out run-a.csv`
(19 min 25 s wall, 1 169 deals in 167 sittings, a style in both seats), and the
null replication `stage0 --deals 1000 --seed 5100000 --skills 3 --calib-hands 0
--out run-null.csv`.

| Style | Skill 3 divergence | Skill 3 bid rate | Replication (seed 5100000) | Rule bids, 1 000 hands |
|---|---|---|---|---|
| normal | 0.00% | 18.9% | 20.4% | 16.9% |
| bermann | 5.85% | 5.9% | 6.36%, 6.0% | 5.7% |
| ulmann | 3.97% | 20.9% | 3.96%, 21.6% | 18.3% |
| ms | 11.30% | 3.5% | 11.56%, 3.3% | 3.3% |
| werner | 0.35% | 9.6% | 0.38%, 9.9% | 9.3% |
| bold | 8.28% | 46.8% | 8.19%, 49.6% | 41.3% |
| cautious | 6.80% | 3.5% | 7.02%, 3.3% | 3.3% |
| hunter | 3.92% | 17.4% | 4.15%, 18.2% | 15.9% |

- Gate (divergence under about 1%): **Werner drops** (0.35%, 0.38%). It stays in
  the code and is not offered. Every other style passes at skill 3, replicated on
  fresh seeds within 0.4 points.
- MS and Cautious both bid 3.5%: candidate twins for stage 1 (STYLES 2.2 keeps
  Cautious if the gate merges them).
- **Skill 4 rows are void**: the rung-4 margins were 0 and every style shared the
  2.0 band (divergence 11-13%, bid rate 21% for all). Rerun after the fixes and
  the margins below.
- The honour count changed after the run (the Pagat only with six tarocks, for
  Ulmann and Werner), so their bid figures are slightly stale; rerun skill 3
  with the margins.

### Rung-4 bid margins

Calibrated (Points; the advantage at which Normal's rung-4 sample bids the share
the style's rule bids, 1 000 first-bidder hands, 40 worlds): normal +0.100,
bermann +0.462, ulmann +0.025, ms +0.550, werner +0.288, bold -0.475, cautious
+0.550, hunter +0.100. Normal's rung 4 bids at margin 0 and must stay so (the
golden), so `style::MARGINS` holds each figure less Normal's 0.100 (STYLES 3
defines no other anchor): bermann +0.362, ulmann -0.075, ms +0.450, werner
+0.188, bold -0.575, cautious +0.450, hunter 0.000.

### Stage 1: the tells gate, 17 Oct 2026

`python3 -I research/sim/styles_stage1.py run-b.csv --null run-null.csv --reps 5`
on the post-fix stage 0 output (`stage0 --deals 1000 --seed 5200000 --skills 3,4
--calib-hands 1000 --out run-b.csv`; 167 sittings, a style in both seats, so 334
samples per style and skill). Test: `python3 -I research/sim/test_styles_stage1.py`.

Sample: one sitting (3 Touren, 6 deals) and one seat; features are the sums of
every CSV column except identifiers and `meta_*` (`tarock_lead_ge5`,
`tarock_lead_early`, `bid_decisions`, `bid_taken`, `bid_taken_le5`,
`pagat_early_78`, `pagat_early_10`, `figure_blank`, `figure_two_low`,
`split_open`, `mond_early`, `cash_mond`, `guard_mond_kept`, `spare_lead`) plus
two rates (taken/decisions, taken at most 5 tarocks/taken). Ridge logistic
regression, 5-fold CV grouped by sitting, 5 repetitions, pooled out-of-fold AUC.
Rerun with 20 repetitions and another fold seed: unchanged to 0.002.

AUC, skill 3 (* below 0.8):

| | bermann | ulmann | ms | werner | bold | cautious | hunter |
|---|---|---|---|---|---|---|---|
| normal | 0.936 | 0.667* | 0.941 | 0.728* | 0.874 | 0.879 | 0.800 |
| bermann | | 0.940 | 0.890 | 0.921 | 0.989 | 0.926 | 0.964 |
| ulmann | | | 0.951 | 0.783* | 0.868 | 0.898 | 0.854 |
| ms | | | | 0.875 | 0.994 | 0.853 | 0.957 |
| werner | | | | | 0.965 | 0.746* | 0.835 |
| bold | | | | | | 0.991 | 0.959 |
| cautious | | | | | | | 0.932 |

AUC, skill 4:

| | bermann | ulmann | ms | werner | bold | cautious | hunter |
|---|---|---|---|---|---|---|---|
| normal | 0.795* | 0.640* | 0.806 | 0.678* | 0.908 | 0.812 | 0.561* |
| bermann | | 0.855 | 0.609* | 0.658* | 0.983 | 0.604* | 0.787* |
| ulmann | | | 0.882 | 0.768* | 0.860 | 0.882 | 0.629* |
| ms | | | | 0.682* | 0.986 | 0.581* | 0.807 |
| werner | | | | | 0.959 | 0.675* | 0.661* |
| bold | | | | | | 0.988 | 0.903 |
| cautious | | | | | | | 0.809 |

Null control (Normal v Normal, disjoint sittings): 0.482 (skill 3), 0.491
(skill 4), 0.481 (skill 3, Normal of `run-null.csv`, seed 5100000, pre-fix
binary, 11 common columns). The classifier does not overfit.

Decisions (skill 3, where the styles are measured):

- **Ulmann fails** (0.667 against Normal, 0.783 against Werner): dropped. Its
  differences from Normal (the 8-tarock Pagat, `draw_from`) are too rare to
  read in a sitting. Not offered; stays in the code.
- **Werner** is below 0.8 against Normal (0.728), Ulmann and Cautious; already
  dropped at stage 0 (0.36% divergence). Not offered.
- **Hunter** is at 0.800 against Normal (standard error about 0.02): marginal,
  kept; a fresh-seed run decides it.
- **No merge**: MS v Cautious reads at 0.853 although both bid 3%, so MS and
  Cautious both ship (the rule of 2.2, keep Cautious on a merge, is not needed).
  Ship at skill 3: Bermann, MS, Bold, Cautious, Hunter (marginal), with Normal
  and Random.
- **Skill 4**: the gate fails for Bermann v Normal (0.795), Hunter v Normal
  (0.561), MS v Cautious (0.581), Bermann v MS and v Cautious (0.61, 0.60).
  The sitting-level tells are weaker at 4 than at 3 for every pair, Werner v
  Normal included (0.678 against 0.728).

Does skill 4's play divergence inflate AUC? No. Every style, Werner too,
diverges 11-12% at skill 4 because the style band (2.0 against 0.34) lets the
weights decide; that divergence is shared noise, not a tell, and Werner v
Normal falls from 0.728 to 0.678 (it rests on the bid, 12.3% against 20.0%),
as does nearly every pair. It dilutes the tells. Implication: a style at skill 4
is readable chiefly through its bid; the non-bid maxims (Bermann's figure rule,
the Pagat hunter's tarock leads) do not survive the search's own variance. A
style's tells at skill 4 need either a narrower style band or the stage-0
margins applied to the play as well; until then Random and the styles whose bid
differs (Bold, MS, Cautious) are the ones whose skill-4 tells are claimed.
