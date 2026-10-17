# Review: docs/HINTS.md (hints that teach, third revision, 17 Oct 2026)

Scope: `docs/HINTS.md` against its section 0, `CLAUDE.md`, `docs/RULES.md`,
`docs/MEASUREMENTS.md` (M1, M4, M5), `docs/OPPONENT.md`, `docs/TABLE3D.md` 6.2,
`research/07`, `16`, and the code (`explain.rs`, `hint.rs`, `rung2.rs`,
`rung4.rs`, `know.rs`, `web3d/src/hinttext.js`, `web3d/strings.json`).
Nothing was run except word counts of the spec's own drafts. Severity: must (the
spec contradicts itself, a source or the code), should (fix or record open
before building), nit.

## Verdict

Accept with fixes. The two-half rule is stated well and the routine of fixed
questions is a sound design. Four defects stop it being buildable as written:
worked case (b) contradicts rule 4 (the Pagat is still A), eight of the ten
drafts break the word budget, the `so-*` gate lets rules ship that M5 shows the
engine breaks, and M9 does not measure cause.

## Musts

1. **Case (b) contradicts section 4 and test 1: A is the Pagat, not III.**
   Section 4: X wins, A = "the cheapest loser (least thirds, then dearness)".
   In (b) II is led and the hand holds the Pagat (`hold-pagat` fires only when
   the Pagat is the lowest legal tarock, `explain.rs:62-85`). Every tarock above II
   beats it, so the Pagat is the only loser and so A. Nothing in the rule
   gives III. Consequences: `hold-pagat` is still returned ("only when the Pagat is
   A"), so the bug (b) of section 1 is not fixed; the decisive question is not
   `keep` (X = Mond and A = Pagat are both key cards, "and the other is not"
   fails) but `worth` (Mond and Pagat: exactly one wins); the "Why?" contrast would
   be about the Pagat. Test 1 ("13 tarocks, II led: A = III") fails against its own
   rule 4. Fix: define A so that a keep-worthy loser is not "the natural alternative"
   for a winner: X wins, A = the cheapest winner (the other card a thoughtful
   player weighs), or the cheapest loser only when X is the cheapest winner. Then
   (b) gives A = III, `keep` decisive (Mond key, III not), as specified. Re-derive
   (a) and (b) from the rule in a table in the document.
2. **The default hint cannot meet its 25-word budget with its own wording.**
   Headline (2 words) + the fixed question of the table + `ans-*` + `so-*`, counting
   each placeholder as one word, gives, for the ten drafts of section 8 (in order
   of the table): 29, 24, 34, 32, 26, 29, 26, 34, 34, 35. Eight of ten exceed 25.
   Only the two worked defaults fit, because they use a shortened question
   ("What lies beneath?", "Is it safe to keep?") and, in (b), no `ans-*` at all.
   That breaks "always worded the same" (section 2), contradicts "run 20 to 28"
   (section 8) and the "Now" form of (b) (which uses the table's wording). Also
   the composition is undefined: (a) shows question + answer + so, (b) shows
   question + so (the so repeats the answer). Fix: state one composition
   (headline, question, `so` that absorbs the answer's fact as its condition; the
   answer alone when `so` is null) and choose the question wording and `so-*`
   drafts to fit 25 by a test written first (test 8); list the counted words per
   draft in the document.
3. **The `so-*` gate lets through rules the engine breaks.** Section 5.1 gates the
   causal clause by M9 only and M5 only for the "Why?" `Rule:` line; Q3b says
   "precision 0.80: the M5 and M9 gates". These disagree. M5 (`MEASUREMENTS.md`,
   result 2026-10-15) passed only `worth.rich` and `announce`: `off` 0.27 (leading)
   and 0.20 (a lying figure), `pagat.keep` 0.74, `cheap` 0.73, `worth.poor` 0.799,
   `run` 0.77, `void` 0.59, `behind` 0.53. Case (a) shows the contradiction: the
   default says "A card on a packet: play it off" ("M9 passed for `off`") while
   its "Why?" says "No `Rule:` line: M5 failed for `off`". A newcomer following
   `so-beneath` plays a cover at a trigger where rung 4 does so 27% of the time.
   Fix: a `so-*` ships only if it passes M5 (precision of the very trigger and
   action it displays, Wilson 0.80, confirmed) and M9; say so once in 5.1 and
   in Q3b. Cases with no M5 candidate (`keep-mond`, `out-master`, `trumps` as worded)
   need an M5 candidate added to the plan first. On the present results only
   `so-worth-rich` would ship: record that in 7 and in open question 8
   ("except the M5 passes (`worth.rich`, `announce`)" is right; `announce` has no
   `so-*` here).
4. **M9 does not measure cause, and its design is not fixed.** (i) "Search again
   with that class removed" is unnecessary: `move_table` already returns every
   legal card's value in every world (`rung4.rs:66-122`), so "the best card outside
   the class is worse by MARGIN" is a read of `per_world`. (ii) That quantity says
   the engine's card is clearly best against the other class, a clear chance
   (M1/M3), not that the displayed idea is why. Example: a lead of J♦ over XIX can
   be clearly better because it saves XIX (`worth`), yet `beneath` "explains" it
   whenever the cover class is clearly ahead. Attribution must be per position at
   display time, or the claim reduced to "the class is clearly right". (iii)
   CLAUDE.md requires, before the first batch: the comparison, batch size, maximum
   looks and boundary. The spec gives none of the numbers ("fixed first"). Fix:
   write them in the spec or in the `MEASUREMENTS.md` plan commit: for example
   batches of 100 firings per idea, 4 looks, the OBF boundaries 4.05 / 2.86 / 2.34 /
   2.02 of M4 (`stats.rs`), a futility stop; and say whether the test is a
   z-test of p against 0.80 (then the OBF boundary applies) or only a Wilson
   bound (then there is no sequential look to adjust). Mirrored pairs are required
   by CLAUDE.md only for a game result; M9 is a search-value measure, so say that
   explicitly, and take the positions from mirrored sittings anyway.
5. **Case (b) cites a 1,000-world sample no follow has.** "The Sküs is in his hand
   in 380 of 1,000" and `fact-skus-out` ("share counted in the sample") need a
   world count of the unseen cards. Section 4 gives follows only the 50-world
   `odds_with` table, which records card values, not who holds the Sküs, and
   section 1 says `follow` draws no sample. Either add a follow sample (a
   `lead_sims`-style pass; cost to be benchmarked, the "under 5 ms" claim of 4
   excludes it) or state the figure as "in about 38 of 50" from the 50 worlds
   (needs the sampler to expose `unseen` per world), or drop it.
6. **The "Why?" budget fails in (b).** Q4 says "Why?" within 90 words; (b)'s "Why?"
   is 121 words (header included), (a)'s is 71. Fix: shorten (b) or move the
   limit; the test (8) will fail as written.
7. **`ask-differ` contradicts its own example.** (b): "The books lean the other
   way here (the cheapest card)" is followed by a Mondfang sentence citing U p. 74
   in favour of the Mond. The books do not lean one way; U p. 74 ("den Mond ... suche
   man rechtzeitig heimzubringen") supports X, and the "cheapest" maxim is the
   suit-trick `Nach-hinten-spielen`, not a tarock rule (research/16 `cheap`, M5
   0.73). Fix: state when `ask-differ` applies (a sourced prescription for the
   decision's own kind, whose condition holds and which names A), and drop it from
   (b).

## Shoulds

8. **Decisive question: undefined or misleading positions.**
   - *No A when X loses and nothing wins.* Rule 4: "X loses: the cheapest winner".
     When no legal card beats the lead, A does not exist; `worth`, `cheap`, `ask.beat`
     are undefined. Fix: A = the cheapest loser other than X (or none: use 5.2
     level 2, stated).
   - *`cheap` is nearly dead.* With A as defined, X winning gives a loser A
     whenever one exists, so `worth` is decisive first; "both lose" cannot occur
     (A is a winner when X loses). `cheap` fires only when every legal card
     wins and X is the cheapest. Yet `cheap` is a skill and a brief line. Say so
     or define A for follows as the nearest other card of the same outcome when
     `cheap` is the skill in focus.
   - *"can be beaten by a card still out"* (`ask.out`) is a possibility test and is
     true of almost every non-master card early; it is decisive only when exactly
     one of X, A is a master. Say "possible" in the rule, give the tarock/suit
     handling, and re-check (a): if XIX has a higher tarock out, "out no" holds
     for the stated reason; if XIX is the highest left, `out` is decisive and ranks
     before `beneath`, so (a) would raise `out`.
   - *`ask.suit` and `ask.trumps` on a tarock.* A tarock has no suit owner:
     `suit` is undefined when X or A is a tarock. In (a) `trumps` is also decisive
     (J♦ against XIX) and is shadowed only by the order. Define
     `suit` as decisive only when both are suit cards.
   - *Fixed priority picks a misleading question.* Lead J♦ (a cover) against XIX:
     `beneath` wins by order, but the "Why?" contrast shows the real difference is
     what XIX would spend. Any cover led over the highest card will be explained
     by `beneath`, whatever the cause. Fix: order by M9 attribution (highest
     measured first) instead of by rarity, or state the rule is a teaching order
     and accept it in Q3b's reporting.
   - *`keep` names "this card" but may concern A.* "X or A is a key card, the
     other not": if A is the key card (X = a throw, A = Mond), the question "What
     does keeping this card give me later?" is about A, and the hint card is X.
     Fix: the question and answer name the card (`{keep_card}`) or `keep` is
     decisive only for X.
   - *The Pagat led or cashed.* X = Pagat, A not key: `keep` decisive, but the only
     `so` is "keep the Pagat for the last trick", the opposite of the headline.
     The sourced cash-early rule (MS "7 oder noch weniger", U p. 74, research/07 1.4)
     has no `ans`/`so`. Add `so-cash-pagat` or exclude the case.
9. **Drafts: terms and conditions not recognisable.**
   - "With that many" (`so-keep-pagat`): the number is not stated; sources
     differ (MS more than 11, U 10-12, engine `STRONG_TAROCKS` 8, M5 best 0.74).
     A player cannot recognise it in another deal. State the number, from the
     measure.
   - "solid run" (`so-trumps`), "draw his" (`ask.trumps`, `so-trumps`), "open a
     suit" (`so-suit`; "lead" is the defined word), "key card" (3.2, internal) and
     German "Skartindel" (`so-worth-poor`, not in the Q5 term list; English says
     "small card") are not defined; add them to the term list with their page, or
     reword.
   - `so-keep-pagat` ("the last trick") needs the `pagat-ultimo` page, due only
     at the last five tricks (`TABLE3D.md` 6.2.2); a hint mid-deal would
     break Q5. Gloss it inline or use another wording.
   - `so-keep-mond`: "play the Mond when it wins a trick" holds for nearly every
     trick the Mond can be played on, and does not say why the Mond rather than
     III (A in (b)); a learner would play the Mond at once on every trick while
     the Sküs is out. Condition and action must separate X from A (add the
     cheapest-winner contrast, or a cost bound such as "when the trick is worth
     3 thirds or more").
   - `so-out-void` "he trumps" is true only if he holds a tarock; the draft says
     "{out} tarocks are out" in the answer but the `so` does not carry it.
   - `so-worth-rich` (de) uses the formal imperative "holen Sie"; the other
     `so-*` use the infinitive. Use the infinitive.
   - German packet is "Päckchen" in 8; `strings.json` and the pages use "Paket",
     "Paketblatt" (20+ occurrences). Use "Paket".
10. **Disputed rules stated as rules, against 2.1.** 2.1 says the questions "ask,
    they do not answer by rule" for the contested suit (W: no general rule; Be
    p. 150-151: the first to lead often wins) and for tarocks first. `so-suit` ("a
    suit shared evenly is a bad one to open: ... make him open it") and `so-trumps`
    ("with a solid run, lead from it to draw his", which asserts leading tarocks
    at all) state them (research/16 section 3, items 1 and 3: only "which tarock"
    is agreed). Restrict `so-trumps` to "when you lead a tarock, lead from a solid
    run" (U p. 73-74, MS) and mark `so-suit` as MS's, or keep it at fact level.
11. **Citations.** In table 2: `ask.keep` cites "W p. 133" for keeping or cashing the
    Pagat; W p. 133 says the ultimo cannot be announced (research/07 1.4), the
    7-or-fewer rule is MS and U p. 74. `ask.keep` cites "Mondfang" for playing
    the Mond; the text of research/07 1.5 and research/16 (parked skill) describes the
    defender keeping one or two small tarocks, not the Mond-holder playing it
    now; "that is Mondfang" in 5.1 and the closing sentence of (b) name it wrongly.
    `so-worth-rich` "7 thirds" is an M5 threshold, not a source (cite M5).
    `so-out-master` ("safe once he has shown no tarock") has no source: cite RULES 5.4
    or mark it derived. `fact-master` names `Group.master`: it lives in
    `tracker.rs`/`web3d/src/tracker.js`, not `Knowledge`; the hint must call the
    tracker, say so.
12. **The contrast is conditional on agreement, against "cannot flatter".**
    Section 4: A is fixed "so the contrast cannot flatter", but `odds` and the
    contrast are given "only when it agrees with the choice (`gain >= 1`)", and
    the 50-world table can disagree with the 10-world choice. A shown figure
    is therefore selected to support. Fix: record the share of hints where it is
    withheld in the quality run, and wording that does not imply the engine tested
    the claim neutrally; or show "A was better in n of 50" where it is.
13. **Quality bar (7): trivial or unmeasured targets.**
    - Q0, Q1 hold by construction (a fallback line is a "because"); Q4 is a string
      length. They are tests, fine, but not quality.
    - Q2 (85% / 70%) is satisfied by a decision table broad enough that something is
      always "decisive" (`worth`, `trumps`, `out` as possibility): it measures
      coverage of the table, not that the question is the right one. Add a bar
      that measures the right question: for the fixed positions, the share where
      the decisive question's own class, applied as the `so` says, selects X (that
      is Q3b's precision, now only a goal at 50% and not a gate), and report it per
      question; or make Q3b's precision a gate.
    - Denominators are undefined: only-card, bids, announces, and positions at levels
      below 4 must be excluded from Q2 (an only-card hint has no question).
    - "1 200 decisions in six cells of 200": say how the 200 are drawn from the
      sittings (first 200, random, the human's seat only).
    - Q5's "run half" counts terms against pages "due by that trick" but the pacing
      table is not copied; state the due trick of each page.
    - "50% goal, not a gate" for the share of hints with a `so-*`: with M5 passing
      one rule (3) the share will be far below 50%; the report must give it, as
      the section already says.
14. **M9 estimate (15-20 min) is plausible but not derived.** "About 50 ms a
    search, two a firing, some 10 000 firings": 10 000 x 2 x 50 ms is 17 min
    CPU, matching. Basis missing: why 10 000 firings (per idea? total), and the 50 ms
    (`docs/OPPONENT.md`: rung 4 decision median 7 ms, p95 135 ms at default
    budget; the mean is not given). With (i) of finding 4 the search count drops to
    one per firing, and the sittings that make the positions are extra (M5: 400
    sittings in 78 s, same machine). Wall time on 2 vCPU about half. Rare ideas
    (Mondfang) drawn by condition may need many more sittings. Fix: derive
    the count (per idea x ideas), give the sittings, give a range, and keep the
    consultation (CLAUDE.md).
15. **Self-contained spec.** Section 8 defines the `vs-*` keys by "as in the earlier
    draft" and fact wording as "the draft table of 5.3's parameters": the earlier
    draft is not in the repository. Write the wording (en/de) or defer it to
    `strings.json` with the budget test. Q3's "the registry types each
    parameter" refers to a registry no module has; name where it lives.
16. **`hint.rs` already runs at level 4 only.** Section 4 and 3.1 describe "rung
    2 and below: facts only"; `docs/OPPONENT.md` says the hint always plays rung
    4 (the `level` argument exists). Say whether lower levels are reachable (a
    `?level=` hint) or drop the clause; the facts-only branch would otherwise be
    untested code.
17. **Case (a) reads poorly.** "What lies beneath? Three cards." answers a count to a
    question about content; the `so` then repeats "beneath". What a newcomer can
    use: "turn up the card beneath". In RULES (4.4, 6.1) the card beneath is
    turned and may be pulled if it is a tarock or king; use that (the
    `from-a-packet` page's phrase). J♦ is a cover (Knowledge `covers`) of a
    4-card packet with 3 beneath (`my_hidden`), so the facts are consistent with
    RULES 4.4; the sentence is not helpful. "11 of 24 points yours": a suit is 38
    thirds, 12 2/3 points (RULES 8.1: 1+4*... K 13, Q 10, C 7, J 4, four pips 1);
    24 is not a suit total (research/16's 18 is the card-value sum). State thirds
    or `12 2/3`, and check the other worked numbers by a test (test 11 asserts
    structure only).

## Nits

18. "Nine pairs" (open question 8) vs ten `ans/so` pairs in section 8; "some 45 key
    pairs" is stated but not counted.
19. Narrative of revision history: "third revision the same day", "as in the earlier
    draft", "Against the earlier revision this drops ..." (12). CLAUDE.md: record
    decisions with date and source, not the history of drafts. Remove; the
    maintainer's quotation and date in 0 and 1 are fine. The Vygotsky paragraph
    (1) is accurate to the Wikipedia copy (checked in `research/sources/`) but
    is motivation; one sentence would do.
20. Q5's text says a hint "may use `ultimo`, `Mondfang`, `master`, `void` only in
    Why?", but `so-keep-pagat` and `so-keep-mond` carry the first two idea names
    in the default only through the closing sentence; check by test 9.
21. `ask.beat` is never decisive (3.2), so "a follow decision has five questions"
    (2) is four in the hint; say "five, one only in Why?".
22. 3.1 says the default is the headline plus "the decisive question, answered
    for this position, with the rule" in one sentence; (b) merges answer and rule
    ("Not while the Sküs is out"), (a) does not (finding 2).
23. Section 10 (a) "A = XIX" with "the highest card held": say whether XIX could
    be the Mond (then X = J♦ against a key card makes `keep` decisive and ranks
    before `beneath`; the case relies on XIX not being key).

## Checked and fine

- Citations of table 2 against research/16 and 07: `ask.worth` (Be p. 147,
  MS "den König schinden und ein Skartindel zugeben", research/07 1.3), `ask.beneath`
  (MS "volle Einsicht", W p. 134, U p. 71-75, exposed Dame/Cavall), `ask.out` (W p.
  134, MS counting suits and trumps), `ask.suit` (MS, W p. 134-135, W "keine
  allgemeinen Regeln"), `ask.trumps` (MS, W p. 135), the keep-a-packet clause (W p.
  134) match what the notes say. The ZPD sentence quoted in 1 matches the copy in
  `research/sources/wikipedia-zone-of-proximal-development.html`.
- Engine facts: `unseen_tarocks`, `my_tarocks` (`know.rs:179-186`), `opp_void`,
  `opp_covers`, `opp_known`, `my_hidden`, `SPLIT_SHARE` (`tutor.rs:80`),
  `reserved_packet`/`THREAT_TAROCKS` (`rung2.rs:70, 125`), `lead_sims`/`lead_wins`
  (`rung3.rs:33, 43`), `odds_with` and `per_world` (`hint.rs:217`, `rung4.rs:66`)
  exist as described; the "Play X." duplication is real (`strings.json`
  `hint.play` and `hint.reason.hold-pagat`); `hold-pagat` tests the Pagat first
  (`explain.rs:70-86`) as section 1 says. Timings of 4 (1.9 ms / 13 ms, 4.6 / 4.5
  ms) match `docs/OPPONENT.md`.
- Word counts of the two worked defaults: (a) 20, (b) 21, as stated.
- Thirds in (b): II 1, Mond 13 (RULES 8.1).
- Headline-once, "Why?" closed by default, only-card line, wire shape and
  `hintLines` change are implementable against `hinttext.js` (a small change).
