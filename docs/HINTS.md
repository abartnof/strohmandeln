# Hints that teach: specification

> Status: **proposed**, 17 Oct 2026 (third revision the same day). Builds on
> `docs/TUTOR.md` (the ladder 2, chances 4, brief 6, strings 9, gate 11, 15),
> `docs/MESSAGES.md` ("Hint reasons") and `docs/TABLE3D.md` 6.2, 6.3, 6.5, 6.6.
> A file of its own because the hint's wire format, wording and measures change
> together and `TUTOR.md` is the brief's and the learner's. Game claims cite
> `research/07` (strategy 1.2 to 1.5) and `research/16` (the ladder); "U" is
> Ulmann, "W" Werner, "Be" Bermann, "MS" Mayr & Sedlaczek, as there.

## 0. The governing rule (maintainer, 17 Oct 2026)

> The minimal amount of information the user needs to play the right card now;
> and the minimal amount of strategy/gameplay guidance the user needs to
> bootstrap this incident into a sense of "how the game is played right". The
> user should always be able to play the right card given a hint, and know what to
> do in a similar case in the future.

Two halves, each checked (7): **(1) the hint names the card unambiguously**,
always; **(2) one generalising sentence states a condition the player can
recognise in another deal, from what they can see (their hand, covers, the cards
out), and what to do when it holds**: not a fact true only of this deal. Every
draft wording below was checked against (2); where a draft can only state a fact
(before its measure has passed, 5.2) it is marked as not meeting (2).

## 1. Problem, aim, assumption

Two plays of 17 Oct 2026 (maintainer's notes): (a) a lead of the jack of
diamonds, not the highest card held, with only the sample's win rate as reason
(`lead-rate`); (b) Uhu (II) led, hint "Play Mond. Play Mond, not the Pagat: ..."
(`hold-pagat`), which answers a question nobody asked (the Pagat cannot beat II;
the card weighed is the cheapest winner, III).

Code causes: `explain_play` tests `hold-pagat` first whenever the Pagat is the
lowest legal tarock and the hand is strong; `lead` with no maxim on top ends at
a rate that compares nothing; `follow` draws no sample; "Play X." is printed
twice (`hint.play`, then a reason beginning "Play {card}", `web3d/src/hinttext.js`).

**Assumption (maintainer, 17 Oct 2026).** The player knows the legal moves and
nothing of strategy. Working out what the player can do and what lies just
beyond is the post-game tutor's job (`TUTOR.md` 5, 6); **the hint is not driven
by the learner model**. The hint's job is to show how a good player thinks at
this decision, so that the player thinks that way while playing. For a
newcomer the hints are the main way the game is learned: they are load-bearing,
and held to a checkable bar (7).

**The Kibitzer as the more knowledgeable other.** The zone of proximal
development is "the space between what a learner is capable of doing
unsupported and what the learner cannot do even with support", the range where
the learner acts with support from a "more knowledgeable other", through whom
the learner gradually, "through social interaction and sense-making", comes to
solve problems independently (Wikipedia, "Zone of proximal development", lead;
read 17 Oct 2026, copy in `research/sources/`, which is not in git; the page
says Vygotsky introduced the concept but did not fully develop it). Only this
is taken: the Kibitzer (`TABLE3D.md` 6.5) models the thinking during play, in
short turns, and the player can ask for more. The zone itself, and what to teach
next, belong to the brief.

## 2. The thinking routine: a few questions, always worded the same

A good player asks a few things at every decision. They are fixed, three to
five for each kind of decision, in one wording, drawn from the sources:

| Key | Question (en / de) | Kind | Source | Answered by | Brief skill |
|---|---|---|---|---|---|
| `ask.beat` | Can I win it? / Kann ich ihn holen? | follow | research/16 `worth` (the classes: beats the card led, does not); the legal rule: no obligation to win (page `following`) | the cards that beat the card led | `worth` |
| `ask.worth` | Is the trick worth what it costs? / Lohnt der Stich, was er kostet? | follow | Be p. 147 (whether a smaller trick will do or one should wait for a higher count, research/16 `worth`); research/07 1.3 (MS: "den König schinden und ein Skartindel zugeben"; the winner must lead) | the trick's thirds against the card spent | `worth` |
| `ask.cheap` | What is the cheapest card that does the job? / Welche ist die billigste Karte, die es tut? | follow | research/16 `cheap` (MS "Das Nach-hinten-Spielen", U p. 75) | the lowest winner, else the lowest loser | `cheap` |
| `ask.keep` | What does keeping this card give me later? / Was bringt mir diese Karte, wenn ich sie behalte? | both | research/07 1.4, 1.5 (Pagat: MS keep it with more than 11 tarocks, cash it with 7 or fewer; W p. 133; Mondfang, MS and U p. 74, research/16 "Guarding the Mond"); research/07 1.2 (W: keep a packet while his Pagat ultimo threatens) | a key card (Sküs, Mond, Pagat, a packet's card) and what is out | `pagat` (see 2.1) |
| `ask.beneath` | What lies beneath, if I play this packet card? / Was liegt darunter, wenn ich dieses Paketblatt spiele? | both | research/07 1.2 (MS: play the packets off for "volle Einsicht"; exposed Dame or Cavall at once; W p. 134; U p. 71-75); research/16 `off` | cards face down under a cover, a card turned next | `off` |
| `ask.out` | What is still out that beats it? / Was ist noch draußen, das sie schlägt? | lead | research/07 1.2 (W: "genaues Tarokzählen", which cards of each suit are out and which is major; MS: count how often each suit was played and trumped), 1.3 (MS: a king trumped); research/16 `void`, `endgame` | tarocks out, his shown voids, his covers, masters (Cards-out tab) | `void`, `endgame` |
| `ask.suit` | Whose suit is this? / Wessen Farbe ist das? | lead | research/07 1.2 (MS: lead a suit he shows no card in, from your packets), 1.3 (MS and W p. 134-135: who opens a contested suit ends worse; W adds that no general rule can be given); research/16 `behind`, `void` | the suit's points: mine, his, shared evenly | `behind`, `void` |
| `ask.trumps` | Am I strong enough in tarocks to draw his? / Bin ich in Tarock stark genug, seine zu ziehen? | lead | research/07 1.4 (MS: the tarock-stronger player tarockiert first, from a solid run; W: first shed tarocks, then play the suits); research/16 `run` | my tarocks against those unseen, a solid run | `run` |

Counts: a follow decision has five questions (`beat`, `worth`, `cheap`, `keep`,
`beneath`); a lead has five (`keep`, `out`, `beneath`, `suit`, `trumps`).

### 2.1 Mismatches with the ladder, listed

- Two skills share a question: `void` and `endgame` use `ask.out`; `void` and
  `behind` use `ask.suit`. Two questions share a skill: `worth` has `ask.beat` and
  `ask.worth`.
- `ask.keep` is `pagat`'s question, but it also covers **Mondfang** and a packet
  kept under the threat of his Pagat ultimo, which have **no skill**
  (research/16 parks both as rare; the canon pays no Mondfang bonus,
  research/14 rule 24). They get a question and no brief line.
- `bid` and `announce` are not card plays and keep their keys.
- The sources disagree on the contested suit (W: no general rule; MS: do not
  open it) and on shedding tarocks first (W) against tarockieren first (MS)
  (research/07 1.3): the questions ask, they do not answer by rule.

## 3. The hint

### 3.1 Shape

> **Play Mond.** Is it safe to keep? Not while the Sküs is out: play the Mond when it wins a trick.

Headline ("Play X.", once, `hint.play`, the card named so that it cannot be
mistaken: half 1 of 0; nothing else repeats it), then **the decisive question,
answered for this position, with the rule to carry away**: the answer is the
condition as it stands here (read off what the player can see), the clause after
the colon is what to do whenever it holds (half 2 of 0), in one sentence in the
words of one idea. **Budget: 25 words in English, 30 in German, headline included**, a
test. Basis: the repo's own bounds for what is read in play: tutorial page lines
at most 25 words (`TABLE3D.md` 6.2.1), the whole brief about 45 (`TUTOR.md` 6).
Everything else is behind **"Why?"** (a control, closed, per hint, key and
wording `TABLE3D.md` 6.3's to fix): the answers to the other questions, the
contrast with the natural alternative (4), the sampled figures, the rule sentence
and the `odds`. Asking costs nothing and counts nothing (the hint marked the
decision assisted already, `TUTOR.md` 3). A forced card: "{card} is your only
legal card." and no question.

### 3.2 Which question is decisive

Each question has a test on the pair (X = the hinted card, A = the natural
alternative, 4). It is **decisive** when X and A fall on opposite sides:

| Question | Decisive when |
|---|---|
| `ask.keep` | X or A is a key card: the Sküs, Mond or Pagat, or a card that keeps a packet under his Pagat ultimo threat; and the other is not |
| `ask.beneath` | exactly one of X, A is a packet cover |
| `ask.out` (lead) | exactly one of X, A can be beaten by a card still out, or one leads a suit he has shown void |
| `ask.suit` (lead) | X and A are of suits that differ in whose they are (mine solid, his, shared) |
| `ask.trumps` (lead) | exactly one of X, A is a tarock |
| `ask.worth` (follow) | exactly one of X, A wins the trick |
| `ask.cheap` (follow) | X and A both win or both lose, and X is cheaper |
| `ask.beat` | never decisive: its answer opens the "Why?" |

Several decisive: the first in this order: follow `keep`, `beneath`, `worth`,
`cheap`; lead `keep`, `out`, `beneath`, `suit`, `trumps`. Fixed so the same
position always raises the same question; the order puts first what is
rare and costly (`keep`), then what shows new cards (`beneath`, MS "volle
Einsicht", research/07 1.2). **None decisive: the honest fallback (5.2).**

### 3.3 The answer, and the causal clause

The answer to the decisive question is a **fact** about this position, counted
from known cards or from the hint's own sample: always allowed. The clause that
says what to do whenever the condition holds (`so-*`) is a **claim of cause and
of a general rule**, and is added only if the idea passed its measure (5.1);
otherwise the sentence stops at the fact. **That sentence does not meet half 2 of
0**: it is the honest form while the measure is missing, and the bar (7) counts
how many hints are in it. The same words are used in the brief (6).

## 4. The natural alternative A and the sample

**A**, fixed by rule so the contrast cannot flatter the hint: *following*, X wins:
the cheapest loser (least `thirds`, then `dearness`), or with none the next dearer
winner; X loses: the cheapest winner. *Leading*: the highest card held, greatest
`(is_tarock, strength)`; if X is that card, the cheapest card. `hold-pagat` is
returned **only when the Pagat is A** (the fix of (b): II is beaten by III, not by
the Pagat).

**The sample** (computed with the hint, so the answer does not change when
asked later):

- *Lead*, rung 3 and up: the existing `lead_sims` (1 000 worlds) get one more pass
  per card for `wins`, `cost`, `trumped`; added cost one pass over the sims
  (`lead_wins` at 1 000 worlds: 1.9 ms median, 13 ms p95, `docs/OPPONENT.md`,
  "Hint evidence").
- *Follow*, rung 4: `odds_with` already solves 50 worlds; keep `per_world` and
  read A's column: `better`, `gain_thirds`; **no extra search**; given only when
  it agrees with the choice (`gain >= 1`).
- *Rung 2 and below*: facts only (thirds, `spend()`); no sampled figure is
  invented.

Added latency under 5 ms median, shown by `hintbench` rows (lead 4.6 ms, follow
4.5 ms, p95 about 13 ms before).

## 5. The gates, and the fallback

### 5.1 Which claims need a measure

- **Not gated** (positions facts): the question, its answer, the contrast and
  its figures, the facts of 5.3. Counts of known cards or of the same sample, as
  `lead-rate` is.
- **Gated by M5** (`TUTOR.md` 11; only `worth.rich` and `announce` passed, the
  rest failed): the `Rule:` clause in "Why?", as today.
- **Gated by M9, attribution** (new): the causal clause and naming an idea as the
  reason ("so play it now"; "that is Mondfang"). M5 asks whether the engine
  follows a prescription; an idea shown only where the engine already agrees
  agrees by construction, and so says nothing about cause.

**M9 (planned in `MEASUREMENTS.md`, committed before its first batch).** For each
idea (a question with its prescribed class, e.g. `keep`: not the key card), in
positions from rung 4's own sittings where its condition holds and the engine's
card is in the prescribed class: search again with that class removed
(`students.rs` style). The idea *explains* the choice when the best card outside
the class is worse by at least `MARGIN` (10 thirds, M1) over the same worlds.
Attribution = explained / fired, clustered by deal, Wilson lower bound at the
effective sample, shown at 0.80, confirmed on fresh seeds; comparison, batch
size, maximum looks and the O'Brien-Fleming boundary fixed first; stop as soon as
the effect crosses or is plainly too small (`CLAUDE.md`). Rare conditions
(Mondfang) are drawn by condition; one that cannot reach an effective sample gets
no causal clause. About 50 ms a search, two a firing, some 10 000 firings: 15 to
20 minutes CPU on 2 vCPU, a compute run: **the maintainer is consulted first**.

### 5.2 The fallback, said honestly

Two levels, both always allowed:

1. **Question and fact, no rule** (the idea has not passed): "Is it safe to keep?
   The Sküs is still out." The player still gets the thing to ask, but not yet a
   rule to carry; it does not meet half 2 of 0, and is reported as such (7).
2. **No question is decisive**: "No question settles this one; the sampled deals
   favour it: 41 of 50 came out 7 thirds better." (`ask-none`). Where the engine's
   card contradicts a sourced prescription whose condition holds, the "Why?" says
   so ("The books lean the other way here ({idea}); the sampled deals favour this
   card.", `ask-differ`), not nothing.

No hint is without a "because": level 1's fact, level 2's sample, or `only-card`.

### 5.3 Facts (the answers)

At most two shown in the default's answer or the "Why?", each naming where the
Cards-out tab shows it (`TABLE3D.md` 6.6.5):

| Key | Parameters | Engine source | Status | Shown in |
|---|---|---|---|---|
| `fact-tarocks-out` | `out`, `mine` | `unseen_tarocks()`, `my_tarocks()` | have | line 1 |
| `fact-skus-out` | `share`, `worlds` | Sküs unseen; share counted in the sample | share new | tarock rows |
| `fact-mond-open` | none | Mond mine; Sküs neither played nor mine | new, 3 lines | tarock rows |
| `fact-his-void` | `suit` | `opp_void` | have | line 3 |
| `fact-master` | `card` | `Group.master` ("he may trump" mark) | have | line 2 |
| `fact-his-covers` | `suit`, `n` | `opp_covers`, `opp_known` | have | his cover cells |
| `fact-split` | `suit`, `mine`, `out` | suit group counts; `behind`'s trigger (`SPLIT_SHARE`, M8) | have, wrapper | the suit's row |
| `fact-beneath` | `n` | `my_hidden` of the packet | have | his/your packet |

## 6. The same words in the brief and on the pages

The post-game tutor and the Kibitzer's pages use the **questions' wording and the
skills' names** the hint uses, so each reinforces the other.

- **Brief** (`TUTOR.md` 6): bullet 2 names the focus skill (`tutor.name.*`, as
  now) and its rule; the line "To try:" is the skill's main question verbatim
  (replacing `try.1`, which are imperatives of the same meaning today; `try.2`
  stays): `worth` `ask.worth`, `cheap` `ask.cheap`, `off` `ask.beneath`, `run`
  `ask.trumps`, `pagat` `ask.keep`, `void` `ask.out`, `behind` `ask.suit`,
  `endgame` `ask.out`. The hint's question is thus the brief's "to try", and a
  decision the brief names is the same one the hint worded.
- **Nudge** (`tutor.nudge`, "teaching is two touches", `TUTOR.md` 6): unchanged.
- **Pages** (`TABLE3D.md` 6.2): `following` gains the line "Ask: can I win it? Is the
  trick worth what it costs?"; `from-a-packet` "Ask: what lies beneath?";
  `pagat-ultimo` "Ask: what does keeping this card give me later?" (each within
  25 words a line and 150 a page, the test holds). The lead questions have no page
  yet: a page `leading` opening at the human's first lead is added, due
  no later than the first lead (pacing, 6.2.2).
- One source of strings: the question and the skill name are one `strings.json`
  key each, used by hint, brief and page; a test fails if one is redefined (11).

## 7. The quality bar

For a newcomer the hint is the lesson, so it is held to a bar that is checked.

**Fixed positions.** A set of 1 200 play decisions from 50 mirrored sittings at a
fixed seed (as `hintbench`, level 4 both seats), in six cells of 200: kind (lead,
follow) by phase (tricks 1 to 9, 10 to 18, 19 to 27). The seed, the cells and the
targets are fixed in `MEASUREMENTS.md` before the first run.

| # | Bar | Target | Measure |
|---|---|---|---|
| Q0 | **Half 1**: the hint names exactly one legal card, as the headline | 100% | test, free |
| Q1 | Every hint has a "because" (a fact, a sample line or `only-card`) | 100% | test, free |
| Q2 | **Question share**: a decisive question is named (not level 2 of 5.2) | at least 85% per kind, at least 70% in every cell | the run |
| Q3 | **Half 2, form**: the `so-*` sentence is a pair {condition, action} whose condition parameters are all visible state (the player's hand and covers, the cards out, the trick), none of the sample's counts, his hidden cards or a card code | 100% of the hints that carry one | test, free (the registry types each parameter) |
| Q3b | **Half 2, reach and truth**: in the fixed positions, how often the condition holds, and how often the engine's card satisfies the action when it does (a pair is shown only at precision 0.80: the M5 and M9 gates) | the share of hints carrying a `so-*`: goal 50% overall, not a gate; reach at least 20 positions per pair, else the pair is not shown | the run |
| Q4 | Default within 25 words en, 30 de; "Why?" within 90 | 100% | test, free |
| Q5 | **Readable by someone who knows only the legal moves**: every term is in the list below, defined before use | 100% | static check over strings, and over the run's hints for the pages due by that trick |

**Terms the hints may use**, each with where it is defined (everything else is
everyday English or German; a hint may use "ultimo", "Mondfang", "master" or
"void" only in "Why?" and with its gloss):

| Term | Defined by |
|---|---|
| trick, lead, win a trick | page `playing` |
| follow suit, trump | page `following` |
| tarock(s), Sküs, Mond, Pagat | page `the-cards`; Trull, `honours` |
| packet, cover, "beneath", turn up | pages `packets`, `from-a-packet` |
| point(s), thirds, "costs", "worth" | page `thirds` |
| "the last trick" (for ultimo) | page `pagat-ultimo` |
| "still out", "highest left" | the Cards-out legend and line 2 (`TABLE3D.md` 6.6.4, 6.6.5) |
| "he showed no {suit}" | Cards-out line 3 |
| "shared" (a suit) | glossed in the hint: "shared evenly between us" |
| "sampled deals" | the "Why?" header always says: "I deal his hidden cards at random 1,000 times and play it out." |

A hint is checked against the pages **due at that trick** (`TABLE3D.md` 6.2.2,
the pacing table): one that needs a page not yet due glosses the term inline or
uses the everyday word.

**Cost.** Q1, Q4 and the static half of Q5 are `cargo test`, free. Q2, Q3 and the
run half of Q5 need 1 200 hints at 20 to 65 ms (`docs/OPPONENT.md`) and the
sittings that make the positions: **under 5 minutes of CPU**, a cheap run, not a
long compute run.

**If M9 passes few ideas.** Q2 is unaffected, because an answer is a fact; Q3b
falls, and most hints stop at level 1 of 5.2 ("question and fact"), which does
not meet half 2 of 0. That is the honest form and it still teaches the routine,
but the principle is then unmet for those hints, and the report says so. The rule
is not given without its measure: the options are more positions for M9, or
measuring condition-and-action pairs directly on the engine's own play (the M5
method, for pairs the books do not state), or accepting level 1; the gate is
never lowered. **M9 is therefore on the critical path of half 2.** If Q2 falls under its floor in a cell
(an endgame cell, say), the **questions** are wrong for that kind and phase and
are revised; the target is not.

## 8. Keys, with drafts

Kibitzer voice, short and kind (`TABLE3D.md` 6.5). `*_thirds` print as points with
their unit. No text begins with the play imperative of its own card.

The questions are the table of 2 (en/de). Answers (`ans-*`, with the causal clause
`so-*`, added only when allowed, 5.1):

| Key | English | Deutsch |
|---|---|---|
| `ans-keep-mond` / `so-keep-mond` | The Sküs is still out. / Not while the Sküs is out: play the Mond when it wins a trick. | Der Sküs ist noch draußen. / Nicht, solange der Sküs draußen ist: den Mond spielen, wenn er einen Stich holt. |
| `ans-keep-pagat` / `so-keep-pagat` | You hold {tarocks} tarocks. / With that many, keep the Pagat for the last trick. | Sie halten {tarocks} Tarock. / Mit so vielen den Pagat für den letzten Stich behalten. |
| `ans-beneath` / `so-beneath` | {card} lies on a packet; {n} cards are beneath it. / A card on a packet: play it off to see what lies beneath. | {card} liegt auf einem Päckchen; darunter liegen {n} Karten. / Ein Blatt auf einem Päckchen: abspielen, um zu sehen, was darunter liegt. |
| `ans-worth-poor` / `so-worth-poor` | {led} is a poor trick: {trick_thirds}. / When a low card is led, do not win it with a king: give a small card. | {led} ist ein armer Stich: {trick_thirds}. / Wird eine niedrige Karte angespielt, nicht mit einem König stechen: ein Skartindel zugeben. |
| `ans-worth-rich` / `so-worth-rich` | He led {led}: {led_thirds}. / When he leads a card worth 7 thirds or more, take the trick. | Er spielte {led} an: {led_thirds}. / Spielt er eine Karte von 7 Dritteln oder mehr an, holen Sie den Stich. |
| `ans-cheap` / `so-cheap` | {card} is the lowest card that wins; {alt} costs more. / When two cards both win, play the lower. | {card} ist die kleinste Karte, die sticht; {alt} kostet mehr. / Stechen zwei Karten, die kleinere spielen. |
| `ans-out-void` / `so-out-void` | He showed no {suit}, and {out} tarocks are out. / A suit he lacks, he trumps: lead another. | Er zeigte keine {suit}, und noch {out} Tarock sind draußen. / Eine Farbe, die ihm fehlt, sticht er: eine andere anspielen. |
| `ans-out-master` / `so-out-master` | Only a tarock beats {card} in its suit. / A card only a tarock can beat is safe to lead once he has shown no tarock. | Nur ein Tarock schlägt {card} in seiner Farbe. / Eine Karte, die nur ein Tarock schlägt, ist sicher anzuspielen, sobald er kein Tarock gezeigt hat. |
| `ans-suit` / `so-suit` | {suit} is shared evenly: {mine} of its points yours, {out} out. / A suit shared evenly is a bad one to open: lead another and make him open it. | {suit} ist gleichmäßig geteilt: {mine} Augen bei Ihnen, {out} draußen. / Eine gleichmäßig geteilte Farbe eröffnet man nicht gern: eine andere anspielen, er soll sie eröffnen. |
| `ans-trumps` / `so-trumps` | You hold {mine} tarocks to {out} unseen, a solid run from {card}. / With a solid run of tarocks, lead from it to draw his. | Sie halten {mine} Tarock gegen {out} unbekannte, eine geschlossene Reihe ab {card}. / Mit einer geschlossenen Tarockreihe daraus anspielen, um seine zu ziehen. |

Each `so-*` is checked against half 2 of 0: its condition is visible to the
player (their hand and covers, the cards out, the card led) and its action is what
to do whenever it holds. They run 9 to 18 words, so the default runs 20 to 28 in
English; the test fails any draft over 25 (30), and the build shortens the
question line or the condition (open question 7).

"Why?" lines: the other questions' answers in the table's order; the contrast
(`vs-lead`, `vs-lead-trumped`, `vs-follow-win`, `vs-follow-give`, `vs-facts-follow`:
as in the earlier draft, "Instead of {alt}: it takes the trick in {alt_wins} of
{worlds} sampled deals and costs {alt_cost_thirds}; {card} takes it in {wins} and
costs {cost_thirds}.", each at most 30 words); `ask-none` and `ask-differ` (5.2);
the rule sentence and its source (M5); the `odds`. Fact keys and their words are
the draft table of 5.3's parameters ("{out} tarocks are still out; you hold {mine}."
and so on, 12 words or fewer, en and de). The plain local reasons (`win-*`,
`lose-*`, `lead-*`) lose their "Play {card}" and remain for rung 2 and below.

## 9. Wire and table

`strohmandeln_hint(view, seed, level)` (no learner argument) returns `{hint,
question, answer, so, why}`: `question` a key of 2, `answer` `{key, params}` (or
the `ask-none` line), `so` `{key, params}` or null (gated), `why` `{answers[],
contrast, facts[{key, params, anchor}], rule, odds}`; `reason` stays as the old key
for the older page. `anchor` is `line1`-`line4` or `row:<kind>`: the page may
highlight it in the open Cards-out panel, never opens it (6.6.2). `hinttext.js`:
`hintLines` returns `{lead, question, answer, so, why}`; the box shows the first
four and a "Why?" control. `MESSAGES.md` takes the tables; the existing test holds
it to `KEYS`. Phone layout is deferred.

## 10. Worked cases

The notes do not record the hands: cards and numbers are **illustrative**, the
shape being what is specified (the build reproduces each position and asserts the
structure). *Now* is what ships before M9 has run.

**(a) Lead of the jack of diamonds** (J♦ is a cover of yours with 3 cards beneath;
the highest card held is XIX). A = XIX. Decisive: `keep` no; `out` no; **`beneath`**
(J♦ is a cover, XIX is not).

- *Default, M9 passed for `off`* (20 words): "**Play J♦.** What lies beneath?
  Three cards. A card on a packet: play it off to see what lies beneath." Half 1: the card is named. Half 2: condition "a card
  on a packet" (visible), action "play it off".
- *Now* (no rule; does not meet half 2): "**Play J♦.** What lies beneath? J♦ lies
  on a packet; three cards are beneath it."
- *"Why?"*: "I deal his hidden cards at random 1,000 times and play it out. Can he
  beat it? Only a tarock; 7 are out. Whose suit is this? Diamonds are shared
  evenly: 11 of 24 points yours. Instead of XIX: it takes the trick in 940 of
  1,000 sampled deals but spends your best tarock on a 4-thirds trick; J♦ takes
  it in 130 and costs 4 thirds when it does not." (No `Rule:` line: M5 failed for
  `off`, `MEASUREMENTS.md`.)

**(b) Uhu (II) led, the Mond over III** (13 tarocks; A = III). Decisive: **`keep`**
(the Mond is a key card, III is not). The Pagat is not mentioned (4).

- *Default, M9 passed* (21 words): "**Play Mond.** Is it safe to keep? Not while
  the Sküs is out: play the Mond when it wins a trick." Half 2: condition "the Sküs
  is still out" (Cards-out tab), action "play the Mond when it wins a trick".
- *Now* (no rule; does not meet half 2): "**Play Mond.** What does keeping it give
  me later? The Sküs is still out."
- *"Why?"*: "Can I win it? Yes, with III or the Mond. Is the trick worth it? II is
  1 third, and the Mond brings its own 13 home. What is the cheapest card? III.
  Instead of III, which also wins and spends less: over the rest of the deal the
  Mond was better in 41 of 50 sampled deals, by 7 thirds. The Sküs is in his hand
  in 380 of 1,000. The books lean the other way here (the cheapest card); the
  sampled deals favour the Mond. Mondfang: play the Mond on a trick it takes
  safely, while the Sküs can still catch it (U p. 74, MS)." The last sentence only if
  Mondfang passed M9.

This answers "why not III": the question about the cheapest card is named in
the "Why?" and the hint says it does not decide here.

## 11. Tests

Rust (`explain.rs`, `hint.rs`, `testutil::know`):

1. `hold_pagat_only_when_the_pagat_is_the_alternative` (13 tarocks, II led: A =
   III, no `hold-pagat`; Pagat the cheapest legal tarock with a suit lead: still
   returned).
2. `the_alternative_is_fixed_by_rule`: each branch of 4.
3. `the_decisive_question_follows_the_table`: each row of 3.2 (X and A on opposite
   sides), the order, and `ask.beat` never decisive; the same position gives the
   same question twice and under blind twins (`blind_check_values`).
4. `no_decisive_question_gives_the_sample_line`, and `ask-differ` where the
   engine's card contradicts a sourced prescription.
5. `the_cause_needs_its_measure`: `so` is null unless the idea's slug is in the
   passed list (a constant checked against `docs/MEASUREMENTS.md`); with it null,
   the sentence is question and fact.
6. `lead_contrast_counts_x_and_a_in_one_sample`;
   `follow_contrast_reads_a_from_the_search_table` (one `move_table` call);
   `contrast_given_only_when_it_agrees`.
7. `facts_are_counts_of_known_cards` (recomputed from `Knowledge`, `tracker`).
8. `default_hint_within_budget` (25 / 30 words; "Why?" 90) over every key;
   `no_reason_starts_with_the_play_imperative`.
9. `every_hint_term_is_defined` (the list of 7, over `strings.json` en and de) and
   `one_question_one_string` (hint, brief and pages use the same key).
10. `quality_bar` (`#[ignore]`, the run of 7: Q2, Q3, the run half of Q5; prints the
    cells).
11. The worked positions of 10: structure only; where the engine plays another
    card, the test records it.

Node (`web3d/test/hinttext.test.js`): the lead once and no part repeating it; "Why?"
closed by default and per hint; null parts dropped (`so`); both languages; word
totals for the worked cases; a fact's anchor names a cell of the tracker fixture;
the new page `leading` and the three "Ask:" lines within their limits
(`tutorial.test.js`). Browser: `hint_and_explain` updated.

## 12. Open questions for the maintainer, and the build

1. **A on a lead** when the highest card is the Sküs or a top tarock: the highest
   card (4) or the highest suit card? Recommendation: the highest card.
2. **M9** (15 to 20 minutes CPU): approve before the batch? Recommendation: yes;
   ship the question-and-fact form first, which needs no M9.
3. **Targets of 7** (85% / 70% / 50%): fixed here as a first proposal, to be
   confirmed in `MEASUREMENTS.md` before the first run.
4. **A lead page** `leading` (6): add it with this work? Recommendation: yes, its
   questions are otherwise defined nowhere.
5. **Mondfang and a kept packet** have a question and no skill (2.1). Leave them
   outside the ladder, or add them as parked skills? Recommendation: leave.
6. **`try.1` replaced by the main question** in the brief (6): a change to
   `TUTOR.md` 9 and to `strings.json`; yes?
7. **Word budget against half 2**: the default runs 20 to 28 words with a full
   `so-*`. Keep 25 (shorten the condition) or move the budget to 30?
   Recommendation: keep 25; a hint that cannot say its rule in 25 words is two
   ideas.
8. **Half 2 before M9**: until M9 (or a direct measure of the pairs) has run, no
   hint meets half 2 except the M5 passes (`worth.rich`, `announce`).
   Recommendation: run M9 for the nine pairs first (15 to 20 minutes CPU, on
   approval) before the hint form ships; ship level 1 only if the maintainer
   prefers.

**Build estimate.** About 750 to 850 lines of Rust (the question table and the
decisive test, the selection in `explain.rs`, the contrast and facts) and 500 of
tests; about 120 lines of JS and CSS (the "Why?" control, `hintLines`); some 45
key pairs in `strings.json` and `MESSAGES.md`; the `leading` page and three page
lines; the quality-bar test; M9 about 300 lines in `measure.rs`, run only after
approval. Against the earlier revision this drops the learner plumbing (no
`learner` argument, no chance function in the hint) and adds the question table
and the quality bar: about 100 lines more in all. Order: the questions and
the default form with the `hold-pagat` fix and the duplicate fix (one package,
reviewed), then the contrast and the "Why?", then the pages and the brief's
change, then the bar's run, then M9.
