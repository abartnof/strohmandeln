# Hints that teach: specification

> Status: **proposed**, 17 Oct 2026. Builds on `docs/TUTOR.md` (the gate, 4, 11,
> 15), `docs/MESSAGES.md` ("Hint reasons") and `docs/TABLE3D.md` 6.3, 6.5, 6.6.
> A file of its own because the hint's wire format, wording and measure change
> together, and `TUTOR.md` is the brief's and the learner's. Game claims cite
> `research/07` (strategy, 1.2 to 1.5) and `research/16` (the ladder); "U" is
> Ulmann, "W" Werner, "MS" Mayr & Sedlaczek, as there.

## 1. Problem

Two plays of 17 Oct 2026 (maintainer's notes), both from the level-4 hint:

- (a) A lead of the jack of diamonds, not the highest card held. The reason was
  only the lead's sampled win rate (`lead-rate`, `lead-cheap`), which names no
  cause and no alternative.
- (b) Uhu (II) led; hint "Play Mond. Play Mond, not the Pagat: with 13 tarocks
  ..." (`hold-pagat`). The Pagat cannot beat II; the card the learner weighs is
  the cheapest winner, III.

Causes in the code: `explain_play` tests `hold-pagat` before anything else
whenever the Pagat is the lowest legal tarock and the hand is strong, so it
answers "why not the Pagat" where the contrast is the cheapest winner; `lead`,
with no maxim on top, ends at the sample's rate, which compares nothing;
`follow` has no sample; the table prints "Play X." twice (`hint.play`, then a
reason beginning "Play {card}", `web3d/src/hinttext.js`).

## 2. The hint's shape: two layers, four parts

Every play hint answers, in this order:

| Part | Layer | Answers | Kind |
|---|---|---|---|
| Headline | | "Play X." (`hint.play`, once; no reason repeats it) | |
| **Plan** | 1, this position | what X does, plainly: "takes II and the Mond's 13 thirds", "gives up the cheapest card" | fact |
| **Instead of** | 1 | the natural alternative A (3.1) and what the sample says it does, against X | fact (sampled) |
| **Facts** | 1 | at most two things that decided it (3.3), each found in the Cards-out tab | fact |
| **The idea** | 2, the rationale | the general play X rests on, in plain words, with its source; or the honest fallback (5.3) | gated (5) |

Layer 1 says why *this* card *here*; layer 2 says what to take to the next deal.
Both are in every play hint where they can be given truthfully. A forced card
gives `only-card` alone. Bid and announce hints are unchanged.

## 3. Layer 1

### 3.1 The natural alternative A

Fixed by rule, so the contrast is the same every time and cannot be picked to
flatter the hint. A is a legal card other than X (the hinted card).

- **Following.** If X wins: A = the cheapest *loser* (least `thirds`, then least
  `dearness`, as `follow`'s `cheapest`), or, with no loser, the next dearer
  winner. If X loses: A = the cheapest winner (`dearness`: suits below tarocks,
  then strength). In (b), X = Mond, so A = III. The Pagat is A only when it is
  the cheapest winner or loser.
- **Leading.** A = the highest card held: greatest `(is_tarock, strength)` among
  the legal cards. If X is that card, A = the cheapest card (least `thirds`, then
  `dearness`). This is the card the maintainer asked about in (a) and the lead a
  beginner picks ("the best I have").

What the sample says of A is the hint's lesson: A is what the learner would play
without it. Open question 1: with the Sküs or a long tarock run, A is a tarock;
should it be the highest *suit* card there?

### 3.2 What the sample says of X and A

From the draw the hint already makes.

- **Lead**, rung 3 and up: `lead_sims` (1 000 worlds, `LEAD_WORLDS`) exists. For
  X and for A one more pass each (play the card, the rollout policy replies)
  gives `wins` (he leaves me the trick), `cost` (average thirds given when he
  takes it) and `trumped` (he answers a suit lead with a tarock). Added cost: one
  more pass over the sims; the whole `lead_wins` at 1 000 worlds is 1.9 ms
  median, 13 ms p95 (`docs/OPPONENT.md`, "Hint evidence").
- **Follow**, rung 4: the trick's outcome is known, so the lesson is the rest of
  the deal. `odds_with` already solves 50 worlds and keeps one other card's
  column; keep `table.scores` and `table.per_world` and read A's column:
  `better` (worlds where X beats A) and `gain_thirds`. **No extra search**:
  `odds` is generalised to take A (today it takes the best other card). Given
  only when it agrees with the choice (`gain >= 1`, as today).
- **Rungs 2 and below** (no sample): facts only: the trick's thirds and `spend()`
  of X and A. No sampled figure is invented.

Latency stays inside `docs/OPPONENT.md`'s measure (lead median 4.6 ms, follow
4.5 ms, p95 about 13 ms native, run 1): the build adds under 5 ms median, proved
by `hintbench` rows (8).

### 3.3 Facts

At most two, by a fixed priority. Each comes from `Knowledge` or the tracker,
never the hidden hand, and names where the learner finds it in the Cards-out
panel (`TABLE3D.md` 6.6.5, lines 1 to 4, or the grid).

| Fact key | Parameters | Engine source | Status | Shown in |
|---|---|---|---|---|
| `fact-tarocks-out` | `out`, `mine` | `unseen_tarocks()`, `my_tarocks()` | have | line 1 |
| `fact-skus-out` | `share`, `worlds` | Sküs unseen; share counted in the sample | share is new | tarock rows |
| `fact-mond-open` | none | Mond mine, Sküs neither played nor mine | new, 3 lines | tarock rows |
| `fact-his-void` | `suit` | `opp_void` | have | line 3 |
| `fact-master` | `card` | `Group.master` (with the "he may trump" mark) | have | line 2 |
| `fact-his-covers` | `suit`, `n` | `opp_covers`, `opp_known` | have | his cover cells |
| `fact-split` | `suit`, `mine`, `out` | suit group counts; the `behind` trigger (`SPLIT_SHARE`, M8) | have, a wrapper | the suit's row |

Priority: the fact the contrast turns on (a Sküs fact when X or A is the Mond; a
void when X or A leads into it), then the first true in table order. Each is a
count of known cards: always allowed.

## 4. Reason priority

`explain_play` returns the parts in this order; the first applicable decides the
plan, and the contrast and facts follow it.

1. `only-card`.
2. The contrast keys (6), when A exists and the sample or the facts separate X
   from A.
3. `keep-packet` (his Pagat ultimo threatens): a position fact, kept.
4. `hold-pagat`: **only when the Pagat is A.** This is the fix for (b): II
   cannot be beaten by the Pagat, A is III, the Pagat is not mentioned.
5. The existing `win-*`, `decline-*`, `lose-*`, `lead-*` keys, as the plan when
   no contrast was sampled (rungs 2 and below).

## 5. Layer 2: the idea, and the gate

### 5.1 Ideas

An idea is a sourced piece of play with a condition on the view and a
prescription (a class of cards). It is told as "the idea here", never as "Rule:".

| Idea | Condition here | Prescribes | Source |
|---|---|---|---|
| `behind` | a suit split evenly between the hands | lead another suit; make him open it | research/07 1.3 (MS: whoever opens the contested suit first ends worse in it; W p. 134-135 adds that no general rule can be given); research/16 `behind` |
| `spare-honour` | a poor trick that a figure would win | the least that wins, or a pip | research/07 1.3 (MS "den König schinden und ein Skartindel zugeben"); research/16 `worth`, `cheap` |
| `draw-trumps` | tarock-strong, a solid run | lead the run | research/07 1.4 (MS) |
| `pagat-last` | many tarocks | keep the Pagat | research/07 1.4, 1.5 (more than 11 for a silent ultimo) |
| `pagat-cash` | 7 tarocks or fewer | cash the Pagat early | research/07 1.4 (MS) |
| `mondfang` | Mond mine, Sküs out | bank the Mond on a trick it takes safely, or keep a small tarock to bring him to lead | research/07 1.5 (MS "Mondfang": "ein oder zwei kleine Tarock"); U p. 74 ("den Mond ... rechtzeitig heimzubringen"), as cited in research/16, "Guarding the Mond". The canon pays no Mondfang bonus (research/14 rule 24), so it is told only as saving the Mond's 13 thirds |
| `packets-off` | covers legal | play covers and exposed figures off | research/07 1.2 (U, MS) |
| `not-into-void` | he has shown the suit void | do not lead it | research/16 `void` |

### 5.2 Does "the idea here" keep the gate's intent?

No, not by itself. The M5 gate (`TUTOR.md` 11, 15; `MEASUREMENTS.md`, M5) stops
a prescription being given as advice unless the strongest rung follows it when
its trigger fires (Wilson lower bound 0.80; two of twelve pass). Showing an idea
only where the engine's card already agrees and the condition holds is a weaker
filter: agreement is given by construction, so it says nothing of whether the
idea is the *cause* of the choice. A sourced idea that merely coincides with the
card would be taught as the reason, which is what the gate exists to prevent.
"The idea here" rewords the claim; it does not change it. So:

- `Rule:` stays gated by M5, unchanged (`worth.rich` and `announce` today).
- An **idea** is a weaker, new claim ("here, this idea explains the choice")
  with its own measure, **M9, attribution**. An idea that has not passed is not
  shown as an idea (5.3).

**M9 (to be planned in `MEASUREMENTS.md` and committed before its first batch).**
For each idea, in positions from rung 4's own sittings where its condition holds
and the engine's card is in the prescribed class: re-run the search with the
class removed (a knock-out as in `students.rs`). The idea *explains* the choice
when the best card outside the class is worse by at least `MARGIN` (10 thirds,
M1) over the same worlds. Attribution = explained / fired, clustered by deal;
Wilson lower bound at the effective sample; shown at 0.80 and confirmed on fresh
seeds, as M5. The comparison, batch size, maximum looks and the O'Brien-Fleming
boundary are fixed before the first batch (`CLAUDE.md`); stop as soon as the
effect crosses, or is plainly too small. Rare conditions (`mondfang`) are
drawn by condition (deals dealt until enough firings); one that cannot reach an
effective sample is not shown. Estimate: two searches a firing, about 50 ms each
on 2 vCPU; some 10 000 firings is 15 to 20 minutes of CPU. It is a compute run:
**consult the maintainer first** (`CLAUDE.md`), with that estimate.

Until M9 has run, only the two fallbacks of 5.3 and the M5 `Rule:` lines ship.

### 5.3 The fallback, said honestly

When no idea passed, none has its condition here, or the engine's card
disagrees with the idea whose condition holds, the idea line is the sampled
reason, told as that and not as a book idea:

- With a sample: "No book rule covers this; the sampled deals favour it: 41 of
  50 came out 7 thirds better." (`idea-none-sampled`.)
- Where the engine's card contradicts a sourced idea whose condition holds: the
  box says so (`idea-books-differ`); the disagreement is the more useful lesson,
  and W (p. 134) and MS differ on the contested suit themselves (research/07
  1.3).
- Without a sample (rungs 2 and below): the idea line is omitted rather than
  filled.

## 6. Keys, with drafts

Kibitzer voice (`TABLE3D.md` 6.5): short, kind; "if I may" belongs to the tab's
intro, not each hint. **Budget** for plan, contrast, facts and idea together, the
headline excluded: at most 55 words in English and 70 in German; each part one
sentence of at most 20 words (25 in German). `*_thirds` print as points with
their unit. No reason begins with the play imperative of its own card: the
headline carries it.

Contrast keys (plan and contrast in one sentence):

| Key | Parameters | English | Deutsch |
|---|---|---|---|
| `vs-lead` | `card`, `alt`, `wins`, `alt_wins`, `worlds`, `cost_thirds`, `alt_cost_thirds` | {card} takes the trick in {wins} of {worlds} sampled deals and costs {cost_thirds} when it does not; {alt} takes it in {alt_wins} and costs {alt_cost_thirds}. | {card} gewinnt den Stich in {wins} von {worlds} durchgespielten Austeilungen und kostet {cost_thirds}, wenn nicht; {alt} gewinnt {alt_wins} und kostet {alt_cost_thirds}. |
| `vs-lead-trumped` | `card`, `alt`, `trumped`, `alt_trumped`, `worlds` | He trumps {alt} in {alt_trumped} of {worlds} sampled deals, {card} in {trumped}. | {alt} sticht er in {alt_trumped} von {worlds} durchgespielten Austeilungen, {card} in {trumped}. |
| `vs-follow-win` | `card`, `alt`, `trick_thirds`, `better`, `worlds`, `gain_thirds` | {card} takes {trick_thirds}; {alt} would too and spends less, but over the rest of the deal {card} was better in {better} of {worlds} sampled deals, by {gain_thirds}. | {card} holt {trick_thirds}; {alt} täte es auch und käme billiger, aber über den Rest der Partie war {card} in {better} von {worlds} durchgespielten Austeilungen besser, um {gain_thirds}. |
| `vs-follow-give` | same | {card} gives up the trick ({trick_thirds}) where {alt} would win it; in {better} of {worlds} sampled deals that was worth {gain_thirds} more. | {card} gibt den Stich ({trick_thirds}) her, wo {alt} ihn holte; in {better} von {worlds} durchgespielten Austeilungen war das {gain_thirds} mehr wert. |
| `vs-facts-follow` | `card`, `alt`, `trick_thirds`, `cost_thirds`, `alt_cost_thirds` | {card} wins {trick_thirds} for {cost_thirds}; {alt} wins it too, for {alt_cost_thirds}. (no sample) | {card} holt {trick_thirds} für {cost_thirds}; {alt} holt ihn auch, für {alt_cost_thirds}. |

`hold-pagat` and `keep-packet` lose their "Play {card}": "The Pagat is worth more
kept: with {tarocks} tarocks it can win the last trick." / "Der Pagat ist
aufgehoben mehr wert: mit {tarocks} Tarock kann er den letzten Stich holen."; "His
Pagat ultimo threatens: keep the fullest packet unplayed." / "Sein Pagat ultimo
droht: das vollste Paket liegen lassen."

Facts (one sentence, 12 words or fewer):

| Key | English | Deutsch |
|---|---|---|
| `fact-tarocks-out` | {out} tarocks are still out; you hold {mine}. | Noch {out} Tarock sind draußen; Sie halten {mine}. |
| `fact-skus-out` | The Sküs is still out, in his hand in {share} of {worlds} sampled deals. | Der Sküs ist noch draußen, in seiner Hand in {share} von {worlds} Austeilungen. |
| `fact-mond-open` | Your Mond is unguarded: the Sküs is still out. | Ihr Mond ist ungedeckt: der Sküs ist noch draußen. |
| `fact-his-void` | He showed no {suit}. | Er zeigte keine {suit}. |
| `fact-master` | Nothing higher than your {card} is out. | Nichts Höheres als Ihr {card} ist draußen. |
| `fact-his-covers` | He shows {n} {suit} cards on his covers. | Er zeigt {n} {suit} auf den Päckchen. |
| `fact-split` | {suit} is split: {mine} of its points yours, {out} out. | {suit} ist geteilt: {mine} Augen bei Ihnen, {out} draußen. |

Ideas (the second layer):

| Key | English | Deutsch |
|---|---|---|
| `idea-behind` | The idea here: a split suit goes to whoever opens it last, so make him open it. | Der Gedanke hier: eine geteilte Farbe nützt dem, der sie zuletzt anspielt; er soll sie eröffnen. |
| `idea-spare-honour` | The idea here: no honour on a poor trick; a pip does the same. | Der Gedanke hier: keinen Honneur auf einen armen Stich; ein Skartindel tut es auch. |
| `idea-draw-trumps` | The idea here: the tarock-strong player draws trumps first, from a solid run. | Der Gedanke hier: wer in Tarock stark ist, tarockiert zuerst, aus einer geschlossenen Reihe. |
| `idea-pagat-last` | The idea here: with many tarocks the Pagat waits for the last trick. | Der Gedanke hier: mit vielen Tarock wartet der Pagat auf den letzten Stich. |
| `idea-pagat-cash` | The idea here: with few tarocks, cash the Pagat when you can. | Der Gedanke hier: mit wenigen Tarock den Pagat bei Gelegenheit einstechen. |
| `idea-mondfang` | The idea here (Mondfang): bank the Mond on a trick it takes safely; the Sküs can still catch it later. | Der Gedanke hier (Mondfang): den Mond auf einen sicheren Stich bringen; der Sküs kann ihn später noch fangen. |
| `idea-packets-off` | The idea here: play the covers off to see what lies beneath. | Der Gedanke hier: die Päckchen abspielen, um zu sehen, was darunter liegt. |
| `idea-not-into-void` | The idea here: he trumps a suit he cannot follow; lead another. | Der Gedanke hier: was er nicht bedienen kann, sticht er; eine andere Farbe anspielen. |
| `idea-none-sampled` | No book rule covers this; the sampled deals favour it: {better} of {worlds} came out {gain_thirds} better. | Keine Regel der Bücher deckt das ab; die durchgespielten Austeilungen sprechen dafür: {better} von {worlds} waren um {gain_thirds} besser. |
| `idea-books-differ` | The books lean the other way here ({idea}); the sampled deals favour this card. | Die Bücher raten hier anders ({idea}); die durchgespielten Austeilungen sprechen für diese Karte. |

**Rules and facts.** Gated: the `Rule:` clause (M5, unchanged) and every
`idea-*` key but the two fallbacks (M9). Always allowed: plan, contrast and
facts, all counts of known cards or of the same sample (as `lead-rate` is), and
the two fallbacks, which state only the sample.

## 7. Wire and table

- `strohmandeln_hint` returns `{hint, plan, contrast, facts, idea, rule, odds}`:
  `plan` and `contrast` are `{key, params}` (`contrast` may be null), `facts` is
  `[{key, params, anchor}]`, `idea` is `{key, params}` or null. `reason` stays as
  the plan's key for the older page. `anchor` is `line1` to `line4` or
  `row:<kind>`; the page may highlight it in the open Cards-out panel and never
  opens it (`TABLE3D.md` 6.6.2). The key tables of 6 go in `MESSAGES.md`; the
  existing test holds it to `KEYS`.
- `web3d/src/hinttext.js`: `hintLines` returns `{lead, plan, contrast, facts,
  idea, more}`; `lead` is the only place "Play X." appears; a null part is
  dropped. Phone layout is deferred (maintainer, 15 Oct 2026).

## 8. Tests

Rust (`explain.rs`, `hint.rs`; positions from `testutil::know`):

1. `hold_pagat_only_when_the_pagat_is_the_alternative`: 13 tarocks incl. I, III,
   XXI, II led: the plan is not `hold-pagat`, A = III; a hand where the Pagat is
   the cheapest legal tarock and the lead is a suit: `hold-pagat` still returned.
2. `follow_alt_is_the_cheapest_winner_or_loser`: each branch of 3.1.
3. `lead_alt_is_the_highest_card_or_the_cheapest`.
4. `lead_contrast_counts_x_and_a_in_one_sample`: X's `wins` equals today's
   `lead_wins`; two calls equal.
5. `follow_contrast_reads_a_from_the_search_table`: `odds_with(.., alt)` equals
   `better` counted over `per_world`; one `move_table` call only (a counter).
6. `contrast_given_only_when_it_agrees` (`gain < 1`: none, not a wrong one).
7. `facts_are_counts_of_known_cards`: each fact's parameters recomputed from
   `Knowledge` and `tracker`; twin deals differing only in hidden cards give the
   same facts (`blind_check_values`).
8. `no_reason_starts_with_the_play_imperative`: no English or German text in
   `MESSAGES.md` begins "Play {card}" or "{card} spielen".
9. The two worked positions of 9, from legal views: the test asserts structure
   (A, keys, facts). Where the engine plays another card, the test records it;
   the spec fixes the structure, not the move.
10. `an_idea_needs_its_measure`: the shipped idea list equals the ideas M9
    passed (a constant checked against `docs/MEASUREMENTS.md`, as the rule gates).
11. The `MESSAGES.md` tables hold every new key in both languages, within the
    word budget of 6.

Node (`web3d/test/hinttext.test.js`): the lead appears once and no part repeats
it; null parts dropped; English and German; the spoken string has each part
once; word totals within budget for each worked case; a fact's anchor names a
cell in the tracker fixture. The browser check `hint_and_explain` is updated.

Measure: `hintbench` gains lead and follow rows with the contrast (median added
under 5 ms); M9 as 5.2.

## 9. Worked cases

The notes do not record the hands, so numbers below are **illustrative**: the
shape, not a measurement. The build reproduces each position from a constructed
view (test 9).

**(a) The jack of diamonds.** Hand with a king, a queen and the jack of
diamonds; the highest card held is a tarock. X = J of diamonds, A = that tarock
(3.1). Layer 1: "The jack takes the trick in 130 of 1,000 sampled deals and costs
4 thirds when it does not; XIX takes it in 940 and costs 4 thirds." Fact:
"Diamonds are split: 11 of 24 points yours, 13 out." (row: diamonds). Layer 2,
if `behind` has passed M9: "The idea here: a split suit goes to whoever opens it
last, so make him open it." Else `idea-none-sampled`. Either way the learner
sees the highest card is not the aim: the lead is chosen for what it costs and
what it asks of him, with the diamonds' row of the tab to check. About 50 words.

**(b) Uhu led, hint Mond.** 13 tarocks; he leads II. X = Mond, A = III (the
cheapest winner); the Pagat is not mentioned (4, rule 4). Layer 1: "Mond takes
II and the Mond's 13 thirds; III would win too and spends less, but over the rest
of the deal the Mond was better in 41 of 50 sampled deals, by 7 thirds." Fact:
"The Sküs is still out, in his hand in 380 of 1,000 sampled deals." (tarock
rows). Layer 2: if `mondfang` has passed M9 (rare, so perhaps not): "The idea
here (Mondfang): bank the Mond on a trick it takes safely; the Sküs can still
catch it later." Otherwise `idea-none-sampled`, which says only what the sample
shows. If the engine plays III here, the hint is `vs-follow-win` with X = III
and A = the cheapest loser.

## 10. Open questions for the maintainer

1. A on a lead when the highest card is the Sküs or a top tarock: the highest
   card (3.1, the plain reading of the notes) or the highest suit card?
   Recommendation: the highest card; add the highest suit card as a second
   contrast only when X is a suit card.
2. M9: approve the plan and its estimate (15 to 20 minutes CPU on 2 vCPU) before
   the batch. Recommendation: yes; ship the layer 1 parts and the honest
   fallbacks first.
3. The word budget (55 / 70) against the hint box on a small window; desktop only
   while the phone layout is deferred.
4. A follow at rung 3 and below has no sample (3.2): facts only, or a one-trick
   rollout? Recommendation: facts only.
