# Hints that teach: specification

> Status: **proposed**, 17 Oct 2026; revised after `docs/review/hints-spec.md`
> (fixes mapped there). Builds on `docs/TUTOR.md` (ladder 2, chances 4, brief 6,
> strings 9, gate 11, 15), `docs/MESSAGES.md` ("Hint reasons"),
> `docs/MEASUREMENTS.md` (M1, M4, M5) and `docs/TABLE3D.md` 6.2, 6.3, 6.5, 6.6. A
> file of its own because the hint's wire format, wording and measures change
> together and `TUTOR.md` is the brief's and the learner's. Game claims cite
> `research/07` (strategy 1.2 to 1.5), `research/16` (the ladder) and
> `docs/RULES.md`; "U" is Ulmann, "W" Werner, "Be" Bermann, "MS" Mayr &
> Sedlaczek, as there.

## 0. The governing rule (maintainer, 17 Oct 2026)

> The minimal amount of information the user needs to play the right card now;
> and the minimal amount of strategy/gameplay guidance the user needs to
> bootstrap this incident into a sense of "how the game is played right". The
> user should always be able to play the right card given a hint, and know what to
> do in a similar case in the future.

Two halves, each checked (8): **(1) the hint names the card unambiguously**,
always; **(2) one generalising sentence states a condition the player can
recognise in another deal, from what they can see (their hand, covers, the cards
out, the card led), and what to do when it holds**: not a fact true only of this
deal. Where only a fact can be given (5.4) the hint does not meet half 2, says
so, and is counted (8).

## 1. Problem and assumptions

Two plays of 17 Oct 2026 (maintainer's notes): (a) a lead of the jack of
diamonds, not the highest card held, with only the sample's win rate as the
reason (`lead-rate`); (b) Uhu (II) led, hint "Play Mond. Play Mond, not the
Pagat: ..." (`hold-pagat`), which answers a question nobody asked.

Code causes: `explain_play` tests `hold-pagat` first whenever the Pagat is the
lowest legal tarock and the hand is strong (`explain.rs`); `lead` with no maxim on
top ends at a rate that compares nothing; `follow` draws no sample; "Play X." is
printed twice (`hint.play`, then a reason beginning "Play {card}",
`strings.json`, `web3d/src/hinttext.js`).

Assumptions (maintainer, 17 Oct 2026): the player knows the legal moves and
nothing of strategy; what the player can do next is the post-game tutor's job
(`TUTOR.md` 5, 6), so **the hint is not driven by the learner model**; for a
newcomer the hints are the main way the game is learned. The hint always plays
rung 4 at level 4 (`docs/OPPONENT.md`, "The tutor's hint"), so every sample below
is available at every hint; no lower-level branch is specified.

The Kibitzer models the thinking during play: "the more knowledgeable other" of
the zone of proximal development, through whom a learner gradually, "through
social interaction and sense-making", comes to solve problems independently
(Wikipedia, "Zone of proximal development", lead; read 17 Oct 2026, copy in
`research/sources/`, not in git). The zone itself belongs to the brief.

## 2. The thinking routine: fixed questions

A good player asks a few things at every decision. They are fixed, in one
wording, drawn from the sources. Four of the five follow questions can be the
hint's; `ask.beat` is only in "Why?".

| Key | Question en / de | Kind | Source | Brief skill |
|---|---|---|---|---|
| `ask.beat` | Can I win it? / Kann ich ihn holen? | follow | research/16 `worth` (classes: beats the card led, does not); RULES 5.5 (no obligation to win) | `worth` |
| `ask.worth` | Is the trick worth it? / Lohnt der Stich? | follow | Be p. 147 (a smaller trick, or wait for a higher count; research/16 `worth`); MS "den König schinden und ein Skartindel zugeben" (research/07 1.3) | `worth` |
| `ask.cheap` | Which card costs least? / Welche Karte kostet am wenigsten? | follow | research/16 `cheap` (MS "Das Nach-hinten-Spielen", U p. 75) | `cheap` |
| `ask.keep` | Is {keep} safe to keep? / Ist {keep} sicher aufgehoben? | both | Pagat: MS keep it with more than 11 tarocks, cash it with 7 or fewer (research/07 1.4, 1.5), U p. 74; W p. 133 says only that it cannot be announced ultimo; Mond: U p. 74 and MS's Mondfang, the Mond-holder keeps one or two small tarocks so that he leads and the Mond answers (research/07 1.5, research/16 "Guarding the Mond"); packet kept against his Pagat ultimo: W p. 134 (research/07 1.2) | `pagat` |
| `ask.beneath` | What lies beneath? / Was liegt darunter? | both | research/07 1.2 (MS: play the packets off, an exposed Dame or Cavall at once; W p. 134; U p. 71-75); RULES 6.1 (the card beneath is turned); research/16 `off` | `off` |
| `ask.out` | What can still beat it? / Was kann sie noch schlagen? | lead | research/07 1.2 (W "genaues Tarokzählen", which card is major; MS counts of suits played and trumped), 1.3 (MS: a king trumped); research/16 `void`, `endgame` | `void`, `endgame` |
| `ask.suit` | Whose suit is this? / Wessen Farbe ist das? | lead | research/07 1.2 (MS: lead a suit he shows no card in), 1.3 (MS; W p. 134-135 and Be p. 150-151 disagree: research/16 section 3, item 1) | `behind`, `void` |
| `ask.trumps` | Should I lead a tarock? / Soll ich ein Tarock anspielen? | lead | research/07 1.4 (MS: the tarock-stronger player tarockiert first, from a solid run; W: shed tarocks first, research/07 1.3); research/16 `run` | `run` |

A follow has five questions (four in the hint), a lead five.

**Mismatches with the ladder.** `void` and `endgame` share `ask.out`; `void` and
`behind` share `ask.suit`; `worth` has two questions. `ask.keep` is `pagat`'s,
but it covers the Mond and a packet kept against his ultimo, which have **no
skill** (research/16 parks both; the canon pays no Mondfang bonus, research/14
rule 24): a question, no brief line. The contested suit (`ask.suit`) and the
order of tarocks (W against MS) are **asked, not answered by a rule**: they
carry a fact and no `so` (5.4). `bid` and `announce` are not card plays.

## 3. The hint

### 3.1 Composition, defined once

The default hint is **the headline, the decisive question, and one sentence**:

> **Play Mond.** Is the Mond safe to keep? He led, so the Sküs cannot catch it: play it on his lead while the Sküs is out.

- headline "Play {card}." (once, `hint.play`; nothing else repeats it; 2 words);
- the question (table 2; the fixed wording);
- **`so`**, one sentence = the rule: the condition (which contains the position's
  fact) and what to do whenever it holds. When no rule is allowed (5), the
  sentence is **`ans`**, the fact alone (it does not meet half 2).

**Budget: 25 words in English, 30 in German, for headline plus question plus
`so` (or `ans`); "Why?" at most 90.** Placeholders count as one word, and a
refined condition `{cond}` is budgeted at 6 (5.2). The counts are in 9; a test
(12, test 8) counts every draft and every refined wording and fails the build over
budget. Basis: tutorial page lines are at most 25 words (`TABLE3D.md` 6.2.1),
the whole brief about 45 (`TUTOR.md` 6).

**"Why?"** (a control, closed, per hint; key and wording `TABLE3D.md` 6.3's to
fix): the answers to the other questions, the contrast with the alternative A (3.2),
the sampled figures, the source of the rule and its measure (5.3), the facts with
where to see them in the Cards-out tab. Asking costs nothing and counts nothing
(the hint marked the decision assisted already, `TUTOR.md` 3). A forced card:
"{card} is your only legal card." and no question.

### 3.2 The natural alternative A

Fixed by rule, from the legal cards other than X (the hinted card):

- **Following, X wins:** A = the cheapest *other winner* (suits below tarocks,
  then by strength: `dearness`), the card the player asks "why not": "why this
  winner and not a cheaper one". If X is the only winner, A = the cheapest loser
  (least thirds, then `dearness`).
- **Following, X loses:** A = the cheapest winner; if nothing wins, the cheapest
  other loser.
- **Leading:** A = the highest card held (greatest `(is_tarock, strength)`); if X
  is that card, A = the cheapest card (least thirds, then `dearness`).
- If X is the only card of its kind and no other legal card exists: `only-card`.

`hold-pagat` is returned **only when the Pagat is A**.

| Case | X | A (by the rule) | Decisive |
|---|---|---|---|
| (b) Uhu (II) led, 13 tarocks incl. I, III, Mond | Mond | III, the cheapest other winner (the Pagat loses to II, so it is a loser, not A) | `keep` (X is a key card, A is not) |
| (a) lead, cover J♦, highest card XIX (not a key card) | J♦ | XIX | by the order of 3.3 |

### 3.3 Which question is decisive

Each question has a test on (X, A). It is **decisive** when X and A fall on
opposite sides:

| Question | Decisive when |
|---|---|
| `ask.keep` | exactly one of X, A is a **key card** (Sküs, Mond, Pagat; or a card that keeps a packet against his Pagat ultimo, `rung2::reserved_packet`); `{keep}` is that card |
| `ask.beneath` | exactly one of X, A is a cover (`Knowledge::covers`) |
| `ask.out` | exactly one of X, A is a **master** (nothing out beats it: `tracker` `Group.master`, the hint calls the tracker), or exactly one leads a suit he has shown void |
| `ask.suit` | X and A are both suit cards and differ in whose the suit is (mine, his, shared evenly) |
| `ask.trumps` | exactly one of X, A is a tarock |
| `ask.worth` | exactly one of X, A wins the trick |
| `ask.cheap` | X and A both win, or both lose |
| `ask.beat` | never |

**Several decisive: the one whose `so` rule has the highest measured precision**
(5), then, among those without a passed rule, a teaching order: follow `keep`,
`worth`, `cheap`, `beneath`; lead `keep`, `out`, `trumps`, `beneath`, `suit`.
So a hint prefers a question for which a rule is known to be true, and the order
of rarity does not decide what is taught. None decisive: `ask-none` (5.4).

## 4. The sample

All of it is drawn with the hint, so the answer does not change when asked later.

- **Lead:** the existing `lead_sims` (1 000 worlds, `LEAD_WORLDS`) get one more
  pass for A: `wins` (he leaves me the trick), `cost` (thirds given when he takes
  it), `trumped` (he answers a suit lead with a tarock). `lead_wins` at 1 000
  worlds: 1.9 ms median, 13 ms p95 (`docs/OPPONENT.md`, "Hint evidence").
- **Follow:** `odds_with` already solves 50 worlds; keep `per_world` and read A's
  column. The figure is shown **as it is**, whichever card it favours: "better in
  {a} of {w} deals, {card} in {b}, the rest equal". A figure favouring A is not
  withheld (a quality-run report, 8, counts how often the sample sides with A).
  No extra search.
- **No figure without a source in the engine.** Who holds a card among the unseen
  is not recorded in either table; no share of that kind is shown (the draft of
  the Sküs "in his hand in 380 of 1,000" is dropped).

Added latency under 5 ms median, shown by `hintbench` rows (lead 4.6 ms, follow
4.5 ms, p95 about 13 ms before, `docs/OPPONENT.md`).

## 5. A `so` sentence is a rule, and ships by measure

### 5.1 What the sentence claims

The `so` sentence states a **rule**: in positions where a condition on what the
player can see holds, rung 4 plays the prescribed class. It does **not** state the
cause of this choice (no per-position attribution is claimed or measured). It
ships under M5's gate (`TUTOR.md` 11): precision, as clustered Wilson lower bound,
at least 0.80, confirmed on fresh seeds and under the settings that matter
(5.2). On the results of 15 Oct 2026 (`MEASUREMENTS.md`, M5) only `worth.rich` and
`announce` pass; `announce` has no `so` here. Not gated, always allowed: the
question, the answer (a fact), the contrast and its figures, the facts, the
fallbacks.

### 5.2 Rule refinement (M5r)

M5 tested wide triggers ("a cover and a hand card are legal: a cover") and most
failed, because the idea is right only in part of its trigger. The way out is
not a looser gate but **narrower true rules**: for each idea that failed, search
a small fixed family of visible conditions for one under which the idea's action
is right at 0.80, then say that condition in the rule.

**Plan (to be committed in `MEASUREMENTS.md` before the first run).**

- **Data.** M5's: rung 4 against rung 3, both seats alternating by sitting, 3
  Touren, every decision of the rung-4 player logged with its visible state
  (hand, covers, cards out, his shown voids, the card led, the trick number).
  Mirrored pairs are not used: they are `CLAUDE.md`'s for the comparison of two
  players' results, and this measure compares no players; it counts how often
  rung 4's own move matches a prescription, as M5 does. Seeds: selection
  60,000,000 (400 sittings), confirmation 65,000,000, settings 66,000,000 to
  69,000,000 (new, none used by M5 or M4).
- **Ideas and classes** (from `rules.rs` `CANDIDATES`, each idea's trigger as
  there, plus new triggers for the Mond and the lead of a tarock): `worth.poor`,
  `cheap`, `off` (pip, figure), `run`, `pagat` (keep, cash), `void`, `mond` (new:
  following with the Mond legal and winning, the Sküs not played and not mine:
  the Mond), `trumps` (lead a tarock, lead a suit). **Excluded:** `behind`/`ask.suit`
  (the sources disagree: no rule is stated), `worth.rich` and `announce` (already
  passed), `endgame` (no card prescribed). The Mond rule is derived from the rules
  of the trick, not from a book: the follower plays after the leader, so a Sküs
  cannot answer his Mond unless he led the Sküs (RULES 5.1, 7.1).
- **The family** (listed in `rules.rs` before the first run; atoms of **visible
  state only**): tarocks unseen at most 0, 2, 4, 6, 8; my tarocks at least 6, 8, 10,
  12 or at most 5, 7; he has shown the suit void; he has shown no tarock; the
  Sküs unplayed and not mine; the Mond mine; the suit's points left at most 13,
  26 thirds; the trick number 1-9, 10-18, 19-27; my covers at least 1, 2; his
  shown covers of the suit at least 1, 2; the card led worth at least 1, 4, 7, 10,
  13 thirds; I hold the master of the suit. A candidate is one atom or a
  conjunction of **two**, drawn from at most 12 atoms listed for the idea: at most
  12 + 66 = 78 per idea and class, at most about 800 in all.
- **Fitting.** On the selection seeds, per idea and class, a candidate is
  eligible if its effective sample is at least **60** and its Wilson lower bound
  at least 0.80; the **widest** eligible one (most firings; ties by lower bound)
  is taken forward. Choosing one candidate per idea and class before looking at
  fresh data guards against the search having found noise.
- **Confirmation, sequential.** On fresh seeds, batches of **100 sittings** (600
  deals), at most **4 looks** (400 sittings), for z = (p - 0.80) / sqrt(0.8 x 0.2
  / n_eff) the two-sided O'Brien-Fleming boundaries **4.05, 2.86, 2.34, 2.02**
  (`stats.rs`, as M4; used one-sided, which is conservative). **Pass** at the first
  look where z is at or above its boundary and n_eff is at least 60; **fail** at
  the first look from the second where the 95% one-sided upper bound of p is below
  0.80 (z at most -1.645) or at the fourth look without a pass. The interval at
  the last look is reported. A passed rule is then confirmed under each other
  setting as M5 (200 sittings each); where it fails under a setting its fact is
  told there (`lessons::taught`).
- **Reach.** A passed rule that fires in fewer than one deal in 40 is recorded as
  true and too rare to teach; it still ships (it costs nothing) but is not counted
  toward the bar's reach (8).
- **The wording is the measured condition.** Each atom has an English and German
  template ("when no tarock is out", "wenn kein Tarock draußen ist"), and the `so`
  sentence is the templates joined with the idea's action; a test fails a shown
  sentence that is not built from the passed rule's atoms. The condition is at
  most 8 words, the action at most 10.
- **Cost, derived from `MEASUREMENTS.md` and `OPPONENT.md`.** M5's selection
  (400 sittings, 2,400 deals, rung 4 against rung 3) took **78 s** on this
  machine (2 vCPU). Selection 78 s; confirmation at most 400 sittings, 78 s, less
  when it stops early; settings 4 x 200 sittings = 4 x 39 s = 156 s; evaluating
  about 800 candidates on about 65 000 logged decisions (2,400 deals x 27) is
  about 52 million tests, seconds. **About 5.5 minutes CPU in all**; if rare
  conditions need more selection data the cap is 1,600 sittings (312 s), at most
  about 10 minutes. It is a compute run: the maintainer is consulted first
  (`CLAUDE.md`), with this estimate.

### 5.3 What the "Why?" says of a rule

"This is a rule measured against the strongest player: it plays so in {p}% of the
{n} positions like this (`MEASUREMENTS.md`, M5r)." The figures come from the
measure's result table, held by a test (as the rule gates are); the source, where
there is one, follows (MS, U p. 74), the Mond's rule says it is derived from the
rules of the trick.

### 5.4 The fallbacks, said honestly

1. **Question and fact, no rule**: the rule for this question has not passed: "Is
   the Mond safe to keep? The Sküs has not been played." It does not meet half 2
   and is counted (8).
2. **No question is decisive**: "No question settles this one; the sampled deals
   favour it: {b} of {w} came out {gain} better." (`ask-none`).
3. `ask-differ`: when X and A both win, X is the dearer, and the card led is a
   suit card (the cheapness maxim is for suit tricks, research/16 `cheap`, MS
   "Nach-hinten-Spielen"): "The books prefer the cheaper card in a suit trick; here
   the sampled deals favour {card}: {a} of {w} to {b}." Not used in (b), where
   the Mond answers a tarock.

No hint is without a "because": a rule, a fact, a sample line or `only-card`.

### 5.5 Facts

At most two in "Why?", each naming where the Cards-out tab shows it
(`TABLE3D.md` 6.6.5):

| Key | Parameters | Engine source | Status | Shown in |
|---|---|---|---|---|
| `fact-tarocks-out` | `out`, `mine` | `unseen_tarocks()`, `my_tarocks()` | have | line 1 |
| `fact-skus-out` | none | the Sküs not played, not mine | new, 3 lines | tarock rows |
| `fact-his-void` | `suit` | `opp_void` | have | line 3 |
| `fact-master` | `card` | `tracker` `Group.master` (not `Knowledge`) | have | line 2 |
| `fact-his-covers` | `suit`, `n` | `opp_covers`, `opp_known` | have | his cover cells |
| `fact-split` | `suit`, `mine`, `out` | suit group counts; `SPLIT_SHARE` (`tutor.rs`, M8) | have, wrapper | the suit's row |
| `fact-beneath` | `n` | `my_hidden` of the packet | have | his/your packet |

## 6. The same words in the brief and on the pages

- **Brief** (`TUTOR.md` 6): bullet 2 names the focus skill (`tutor.name.*`) and its
  rule; "To try:" is the skill's main question verbatim (replacing `try.1`, an
  imperative of the same meaning; `try.2` stays): `worth` `ask.worth`, `cheap`
  `ask.cheap`, `off` `ask.beneath`, `run` `ask.trumps`, `pagat` `ask.keep`, `void`
  `ask.out`, `behind` `ask.suit`, `endgame` `ask.out`.
- **Nudge** (`tutor.nudge`): unchanged.
- **Pages** (`TABLE3D.md` 6.2): `following` gains "Ask: can I win it? Is the trick
  worth it?"; `from-a-packet` "Ask: what lies beneath?"; `pagat-ultimo` "Ask: is the
  Pagat safe to keep?" (25 words a line, 150 a page, held by the page tests). The
  lead questions have no page: a page `leading`, opening at the human's first
  lead, is added.
- One `strings.json` key per question and per skill name, used by hint, brief and
  page; a test fails on a redefinition. German says "Paket", "Paketblatt" as the
  table does.

## 7. Terms the hints may use

Everything else is everyday English or German. A hint may use a term only if its
page was opened at or before that decision in the sitting (`TABLE3D.md` 6.2.2:
`playing` at the human's first play turn, `following` at the first with a card
led, `packets` and `the-cards` at the first deal, `from-a-packet` at the first
cover legal, `thirds` at the first trick won, `honours` at the first announce
chance, `pagat-ultimo` when the human holds the Pagat in the last five tricks), or
glosses it in the same hint.

| Term | Defined by |
|---|---|
| trick, lead, win | `playing` |
| follow suit, trump | `following` |
| tarock, Sküs, Mond, Pagat | `the-cards` |
| packet, cover, "beneath", turned up | `packets`, `from-a-packet` |
| point(s), thirds | `thirds` |
| "still out", "master" (top card left) | Cards-out legend and line 2 (`TABLE3D.md` 6.6.4, 6.6.5) |
| "showed no {suit}" | Cards-out line 3 |
| "sampled deals" | the "Why?" first line: "I deal his hidden cards at random {w} times and play each out." |
| "Mondfang" | glossed in "Why?" only: "answer his lead with the Mond, so the Sküs cannot catch it" |

Not used: "key card", "draw his", "solid run" (a tarock "next in rank to another of
yours" instead), "open a suit" ("lead"), "ultimo" except after `pagat-ultimo`.

## 8. The quality bar

**Positions.** Every play decision with at least two legal cards in the first 50
mirrored sittings from a fixed seed (level 4, both seats; only-cards, bids and
announces excluded), stratified by a seeded shuffle into six cells of 200: kind
(lead, follow) by phase (tricks 1-9, 10-18, 19-27). Seed, cells and targets are
fixed in `MEASUREMENTS.md` before the first run.

| # | Bar | Target | Measure |
|---|---|---|---|
| Q0 | **Half 1**: the headline names one legal card | 100% | test |
| Q1 | Every hint has a "because" (rule, fact, sample line or `only-card`) | 100% | test |
| Q2 | A decisive question is named (not `ask-none`) | at least 85% per kind, 70% in every cell; coverage of the table, not correctness | the run |
| Q3 | **Half 2, form**: the `so` is {condition, action} whose condition atoms are all of the visible family (5.2), none of the sample's counts or his hidden cards | 100% of those carrying one | test (each atom is typed in `rules.rs`) |
| Q4 | **Half 2, truth**: for each shown `so`, its rule's confirmed precision at least 0.80 | 100% (by construction) | test against the result table |
| Q5 | **Half 2, share**: hints carrying a `so` | reported per cell, with the reach of each rule (5.2); goal 50%; not a gate | the run |
| Q6 | Budgets: 25 / 30 words, "Why?" 90 | 100% | test |
| Q7 | **Readable with only the legal moves**: every term is in the list of 7 and defined by a page opened by then, or glossed | 100% | static check, and the run's hints against the page-open events of their sitting |
| Q8 | How often the 50-world sample sides with A (reported, 4) | reported | the run |

**Cost.** Q0, Q1, Q3, Q4, Q6 and the static half of Q7 are `cargo test`. The run
(1 200 hints at 20 to 65 ms, `docs/OPPONENT.md`, plus 50 sittings at M5's rate of
78 s a 400, about 10 s): **under 2 minutes of CPU**.

**If M5r passes few rules** (it may: M5 passed two of twelve): Q2 and Q7 are
unaffected, since answers are facts; Q5 falls; most hints stay at fallback 1, which
teaches the question but not yet a rule to carry, and the report says so. The gate
is not lowered. Options then: a wider family or more data for the ideas that came
close (`worth.poor.1` missed by 0.001), or accepting fallback 1. If Q2 falls
under its floor in a cell, the **questions** are wrong for that kind and phase and
are revised.

## 9. Keys, with drafts and counts

Kibitzer voice, short and kind (`TABLE3D.md` 6.5). Counts are words of headline +
question + `so` (or `ans`) for English / German, a placeholder one word, `{cond}`
six; every default is within 25 / 30 (test 8). Each `so` is checked against half
2: the condition is visible to the player, the action is what to do whenever it
holds.

| Key | `ans` en | `so` en | `so` de | ans / so words (en/de) |
|---|---|---|---|---|
| `keep-mond` | The Sküs has not been played. | He led, so the Sküs cannot catch it: play it on his lead while the Sküs is out. | Er hat angespielt, der Sküs kann ihn nicht fangen: ihn auf sein Anspiel spielen, solange der Sküs draußen ist. | 13/12, 25/25 |
| `keep-pagat` | You hold {tarocks} tarocks. | With {t} or more tarocks, keep the Pagat for the last trick. | Mit {t} oder mehr Tarock den Pagat für den letzten Stich behalten. | 11/10, 19/18 |
| `cash-pagat` | You hold {tarocks} tarocks. | With {t} or fewer tarocks, play the Pagat when it wins the trick. | Mit {t} oder weniger Tarock den Pagat spielen, wenn er den Stich holt. | 11/10, 20/19 |
| `beneath` | {card} lies on a packet; {n} cards are beneath it. | {cond}: play the packet card, so the card beneath is turned up. | {cond}: das Paketblatt spielen, dann wird das darunter aufgedeckt. | 15/14, 22/19 |
| `worth-poor` | {led} is a poor trick: {trick_thirds}. | When he leads a card worth {t} or less, do not win it with a king. | Spielt er eine Karte von {t} oder weniger an, nicht mit einem König stechen. | 13/11, 23/19 |
| `worth-rich` | He led {led}, worth {led_thirds}. | When he leads a card worth 7 thirds or more, take the trick. | Spielt er eine Karte von 7 Dritteln oder mehr an, den Stich holen. | 12/11, 20/18 |
| `cheap` | {card} wins for less than {alt}. | When two cards win, play the one that costs less. | Stechen zwei Karten, die billigere spielen. | 12/12, 16/13 |
| `out-void` | He showed no {suit}; {out} tarocks are out. | A suit he lacks, he trumps while a tarock is out: lead another. | Eine Farbe, die ihm fehlt, sticht er, solange ein Tarock draußen ist: eine andere anspielen. | 15/16, 20/22 |
| `out-master` | Only a tarock can beat {card}. | When he has shown no tarock, your top card of a suit cannot be beaten: lead it. | Hat er kein Tarock gezeigt, ist Ihre höchste Karte einer Farbe nicht zu schlagen: anspielen. | 13/13, 24/22 |
| `suit` (fact only: contested, 2) | {suit}: {mine} of its points are yours, {out} are out. | none | none | 16/15, none |
| `trumps` | You hold {mine} tarocks; {out} are unseen. | Leading a tarock: choose one next in rank to another of yours. | Ein Tarock anspielen: eines wählen, das neben einem anderen Ihrer Tarock liegt. | 14/14, 18 (en) |

`so-keep-mond` is the Mond rule of 5.2 (derived from RULES 5.1, 7.1) and its
refined wording will carry its measured condition; the others are templates with
`{t}` and `{cond}` filled by M5r. `so-worth-rich` is M5's passed rule (7 thirds is
value 3, RULES 8.1). The German column of the questions is table 2's. The default
for a rule not yet passed is `ans`.

**"Why?" lines** (en, de): `vs-lead` "Instead of {alt}: it took the trick in
{alt_wins} of {worlds} deals and cost {alt_cost_thirds} when it did not; {card}
took it in {wins} and cost {cost_thirds}." / "Statt {alt}: gewann den Stich in
{alt_wins} von {worlds} Austeilungen und kostete {alt_cost_thirds}, wenn nicht;
{card} gewann {wins} und kostete {cost_thirds}."; `vs-lead-trumped` "He trumped
{alt} in {alt_trumped} of {worlds} deals, {card} in {trumped}." / "{alt} stach er in
{alt_trumped} von {worlds} Austeilungen, {card} in {trumped}."; `vs-follow` "Instead
of {alt}: over the rest of the deal {card} was better in {b} of {w} deals, {alt} in
{a}, the rest equal." / "Statt {alt}: über den Rest der Partie war {card} in {b}
von {w} Austeilungen besser, {alt} in {a}, sonst gleich."; `ask-none` "No question
settles this one; the sampled deals favour it: {better} of {worlds} came out
{gain_thirds} better." / "Keine Frage entscheidet das; die durchgespielten
Austeilungen sprechen dafür: {better} von {worlds} waren um {gain_thirds} besser.";
`ask-differ` as 5.4 (de: "Die Bücher bevorzugen im Farbstich die billigere Karte;
hier sprechen die durchgespielten Austeilungen für {card}: {a} zu {b} von {w}.");
the facts of 5.5 ("{out} tarocks are still out; you hold {mine}." / "Noch {out}
Tarock sind draußen; Sie halten {mine}."; "The Sküs has not been played." / "Der
Sküs ist noch nicht gespielt."; "He showed no {suit}." / "Er zeigte keine {suit}.";
"Nothing higher than your {card} is out." / "Nichts Höheres als Ihr {card} ist
draußen."; "He shows {n} {suit} cards on his covers." / "Er zeigt {n} {suit} auf den
Paketen."; "{suit}: {mine} of its points are yours, {out} are out." / "{suit}: {mine}
seiner Augen sind Ihre, {out} draußen."; "{n} cards lie beneath it." / "Darunter
liegen {n} Karten."). Each `vs-*` is at most 30 words. The plain local reasons
(`win-*`, `lose-*`, `lead-*`) lose their "Play {card}" and stay for positions with
no question. `hold-pagat` and `keep-packet` are reworded without it: "The Pagat is
worth more kept." / "Der Pagat ist aufgehoben mehr wert."; "His Pagat ultimo
threatens: keep the fullest packet unplayed." / "Sein Pagat ultimo droht: das
vollste Paket liegen lassen."

## 10. Wire and table

`strohmandeln_hint(view, seed, level)` returns `{hint, question, so, ans, why}`:
`question` a key of 2 with `{keep}` where it applies; `so` `{key, params}` or null
(a rule that passed); `ans` the fact line (shown when `so` is null); `why`
`{answers[], contrast, facts[{key, params, anchor}], rule: {id, p, n} | null, odds}`;
`reason` stays as the plan's key for the older page. `anchor` is `line1`-`line4` or
`row:<kind>`; the page may highlight it in the open Cards-out panel and never opens
it (`TABLE3D.md` 6.6.2). `hinttext.js`: `hintLines` returns `{lead, question, text,
why}`, `text` being `so` else `ans`; the box shows the first three and a "Why?"
control. `MESSAGES.md` takes the key tables, held by the existing test. Phone layout
is deferred (maintainer, 15 Oct 2026).

## 11. Worked cases

The hands are not in the notes: cards and numbers are **illustrative** (the shape is
what is specified), and the build reproduces each position from a legal view,
asserting structure. *Now* is what ships before M5r has passed a rule for the
question.

**(b) Uhu (II) led, the Mond over III** (13 tarocks: I, III, ..., the Mond). X =
Mond; A = III (3.2); `keep` is decisive (Mond key, III not); `{keep}` = Mond; the
Pagat is not mentioned.

- *Default, with the Mond rule passed* (25 words): "**Play Mond.** Is the Mond safe
  to keep? He led, so the Sküs cannot catch it: play it on his lead while the Sküs
  is out."
- *Now* (fact only; does not meet half 2): "**Play Mond.** Is the Mond safe to keep?
  The Sküs has not been played." (13 words)
- *"Why?"* (81 words): "I deal his hidden cards at random 50 times and play each
  out. Can I win it? Yes, with III or the Mond. Is the trick worth it? II is 1
  third; the Mond brings its own 13 home. Which card costs least? III. Instead of
  III: over the rest of the deal the Mond was better in 41 of 50 deals, III in 7.
  Mondfang: answer his lead with the Mond, so the Sküs cannot catch it (Mayr &
  Sedlaczek)." It answers "why not III": the cheapest-card question is answered and
  the figure shown.

**(a) Lead of the jack of diamonds** (J♦ is a cover of yours with 3 cards beneath;
the highest card held is XIX, not a key card, and a higher tarock is out, so
neither is a master). A = XIX. Decisive: `beneath` (J♦ is a cover) and `trumps`
(XIX is a tarock). No rule has passed for either, so by the teaching order of 3.3
(lead: `keep`, `out`, `trumps`, ...) the question is `trumps`.

- *Now* (fact only; does not meet half 2): "**Play J♦.** Should I lead a tarock? You
  hold 6 tarocks; 9 are unseen." (14 words)
- *If a `trumps` rule passes* (the refined condition replaces the placeholder),
  e.g. "**Play J♦.** Should I lead a tarock? {cond}: lead a suit."
- *"Why?"* (72 words): "I deal his hidden cards at random 1,000 times and play
  each out. Can he beat it? Only a tarock; 7 are out. Instead of XIX: it took the
  trick in 940 of 1,000 deals but spends your top tarock on a 4-thirds trick; J♦
  took it in 130 and cost 4 thirds when it did not. Whose suit is this? Diamonds:
  5⅔ of its 12⅔ points are yours, 4 are out." (A suit totals 38 thirds, 12⅔ points:
  K 13 + Q 10 + C 7 + J 4 + four pips, RULES 8.1.)

## 12. Tests

Rust (`explain.rs`, `hint.rs`, `rules.rs`, `testutil::know`):

1. `hold_pagat_only_when_the_pagat_is_the_alternative` (13 tarocks, II led: A =
   III, no `hold-pagat`; a suit lead with the Pagat the cheapest tarock: returned).
2. `the_alternative_is_fixed_by_rule`: every branch of 3.2, including the sole
   winner, nothing wins, the lead's highest card and the cheapest.
3. `the_decisive_question_follows_the_table`: each row of 3.3, `ask.beat` never,
   and the order by measured precision then the teaching order; stable under blind
   twins (`blind_check_values`).
4. `no_decisive_question_gives_the_sample_line`; `ask-differ` only for a suit trick.
5. `a_so_needs_a_passed_rule`: `so` is null unless the rule id is in the passed list,
   a constant checked against the M5r result table; `ans` then; and a shown `so` is
   built from the rule's atoms only (Q3).
6. `contrast_counts_x_and_a_in_one_sample`; `follow_contrast_reads_a_from_the_search_table`
   (one `move_table` call); the figure is shown in either direction.
7. `facts_are_counts_of_known_cards` (from `Knowledge` and `tracker`); none uses a
   share of unseen cards.
8. `budgets`: the default of every `so`/`ans` draft and every passed rule's wording
   within 25 / 30 words, "Why?" within 90 (a script counts as the tables of 9);
   `no_reason_starts_with_the_play_imperative` over `MESSAGES.md`.
9. `every_hint_term_is_defined` (the list of 7, over `strings.json` en and de) and
   `one_question_one_string`.
10. `quality_bar` (`#[ignore]`: the run of 8; prints the cells, Q2, Q5, Q8).
11. The worked positions of 11: structure only (A, the decisive question, the keys);
    where the engine plays another card the test records it. The numbers in the "Why?"
    lines are not asserted; the suit total is (`12⅔`).
12. `m5r_family_is_visible_only`: every atom of the family reads only `Knowledge`
    and the tracker, never a hidden card.

Node (`web3d/test/hinttext.test.js`): the lead once and no part repeating it;
"Why?" closed by default and per hint; null parts dropped; both languages; word
totals of the worked cases; a fact's anchor names a cell of the tracker fixture;
the new page `leading` and the three "Ask:" lines within their limits
(`tutorial.test.js`). Browser: `hint_and_explain` updated.

## 13. Open questions for the maintainer, and the build

1. **A on a lead** is the highest card held, the cheapest if X is it (3.2). Say so,
   or the highest suit card? Recommendation: as written.
2. **M5r** (about 5.5 minutes CPU, at most 10): approve the plan and the estimate
   before the first batch. Until a rule passes, hints meet half 1 only.
3. **Targets of 8** (85% / 70% / goal 50%): confirm before the first run.
4. **A `leading` page** with this work (6)? Recommendation: yes.
5. **Mondfang and a kept packet** have a question and no skill: leave them outside
   the ladder (recommended), or add them as parked skills?
6. **`try.1` replaced by the main question** in the brief: a change to `TUTOR.md` 9
   and `strings.json`; yes?
7. **Word budget** 25 / 30: all drafts fit, `keep-mond` exactly. Keep it, or 30 / 36?
8. **The disputed rules** (`ask.suit`, tarocks first) carry a fact and no `so`: accept
   that two of the eight questions never carry a rule?

**Build estimate.** About 750 to 850 lines of Rust (the question table and the
decisive test, the selection, the contrast, the facts, `rules.rs` family and
M5r in `measure.rs`) and 500 of tests; about 120 lines of JS and CSS; the key pairs
of 9 and 5.5 (about 40 in `strings.json` and `MESSAGES.md`, counted at the build);
the page `leading` and three page lines; the quality-bar test. Order: the questions
and the default form with the `hold-pagat` and duplicate fixes (one reviewed
package), then the contrast and "Why?", then the pages and the brief change, then
M5r (on approval), then the bar's run.

## 14. Package 1: built (17 Oct 2026)

Decisions on 13: 1 as written (A on a lead is the highest card held, the Sküs
included); 4 the page `leading` is added later; 5 Mondfang and a kept packet stay
outside the ladder; 6 `try.1` is replaced later; 7 the 25 / 30 budget stands; 8
`ask.suit` and tarocks-first carry no rule. 2 and 3 are open.

Built: `ask.rs` (A, the decisive question, the sentence, the facts), the reason
order (`only-card`, `keep-packet`, `hold-pagat` only when the Pagat is A), the
wire of 10, the strings and the budget test, the default hint in the table.

Where the build settled what 3 to 10 leave open:

- Wire: `question` is the key string and `keep` a field beside it; `contrast` is
  one line and `trumped` the second (`vs.lead-trumped`); `answers` is `[]` until the
  "Why?" is built; `suit` parameters are letters; `hint.ask.*` are the table's
  keys for the questions.
- `ans` lines added where 9 has none: `keep-mond-safe` (the Sküs is not out),
  `keep-skus`, `cheap-lose`, `out-master-tarock`. `ask-differ` is neutral
  ("{card} was better {b} times, the other {a}") and shorter, to fit the budget.
- `fact.tarocks-out` counts every tarock neither mine nor played (line 1 of the
  tab), not `unseen_tarocks()`. `fact.master` reads `Knowledge` (the same cards are
  out as in the tracker). `fact.his-covers` is not emitted yet.
- A suit's owner for `ask.suit` is `tutor::SPLIT_SHARE` with `SPLIT_MIN_POINTS`.
- A leading Pagat is never A, so `hold-pagat` follows only.
- The default box no longer shows the rule and odds clauses; they belong to the
  "Why?".
