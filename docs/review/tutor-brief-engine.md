# Review: the tutor's brief, engine and wasm half

Scope: `git diff main...worktree-agent-abd5f18ed8764998e` for
`crates/strohmandeln-ai/src/{lessons,brief,tutor,review}.rs` (and their tests),
`crates/strohmandeln-wasm/src/{lib,tutor}.rs` and `tests/smoke.mjs`,
`docs/PROTOCOL.md` "The tutor", the "as built" notes of `docs/TUTOR.md`, against
`docs/TUTOR.md` 5-10 and 12, Cassino's `brief.rs`/`lessons.rs`/`learner::nudge`
and `kit/learner`. The table, words and storage are `tutor-brief-table.md`.

Run: `cargo test -p strohmandeln-ai -p strohmandeln-wasm` (212 + 37 + 3 + 2
passed, 16 ignored), `cargo clippy -p strohmandeln-ai -p strohmandeln-wasm
--all-targets` clean, `cargo fmt --check` clean, the wasm built as `bin/gate`
builds it and `node crates/strohmandeln-wasm/tests/smoke.mjs 2` passes (6 deals
through the tutor's calls, 5 nudges). Beyond the suite: a Node script drove the
exports with 30 malformed inputs, each on a fresh instance, and a throwaway
native test mutated records (every action replaced by a random card, a bid or
an announcement; actions appended). The throwaway test is not in the tree.

## Findings

Severity: bug, should, nit.

### Bug

1. **A record with a bid action after the bidding panics the evidence, which
   traps the wasm module** (`tutor.rs:601-620` `decision_verdicts`, the
   `!k.playing` branch; trace: `sim.rs:404` "a legal card" from
   `rung4::bid_samples`). `DealRecord::replay` calls `before` with the next
   action *before* the engine refuses it, so `evidence` runs the advisor on the
   position the action was not legal in. For a play action this is harmless
   (`play_table` needs two legal cards); for `Take` or `Pass` appended after a
   finished deal, `decision_verdicts` samples a bid in a position with no bid
   and the rollout has no legal card. Reproduced natively for both seats on
   every seed tried (`evidence(&record_with_trailing_Take)`), and in the wasm
   module as `strohmandeln_evidence_of`, `_brief_of_record` and `_brief_sitting`
   on a record whose `actions` were doubled. A panic in wasm is `unreachable`
   and every later call traps too, the live game's included (the Worker
   decision is open, `TUTOR.md` 10). The record store is untrusted by design
   (`dealrec.rs`: "nothing in it is trusted: each of these is an error, never a
   panic"; an import file is a path to it). Mutating an action in place, or
   appending a play or an announcement, does not panic; only a bid after the
   bidding does. Fix: in `decision_verdicts` return early unless
   `view.to_move == Some(view.seat)` and `view.legal_actions.contains(action)`
   (the same guard `open_skills` has as `can_bid`); better also in
   `DealRecord::replay`, call `before` only for an action that is legal, so no
   caller can be handed an illegal one. Add the test: a record plus a trailing
   `Take`/`Pass` is `Err(Refused)`, not a panic.

### Should

2. **A provisional rule is told as a rule, and its settings predicate does
   nothing** (`lessons.rs:105-112` `rule()` sets `Gate::Provisional` and
   `under: anywhere`; `lessons.rs:215-219` `pick` treats Provisional like
   Passed; `brief.rs:309`). `TUTOR.md` 6 says "where no rule has passed its
   gate, one plain fact in its place" and 2 says a rule is told only under the
   settings it was confirmed under; neither is what is built, and the `TUTOR.md`
   status note ("every rule is provisional until M5") does not say what that
   shows. The bullet reads "Next: {skill}." / "If I may: {rule}" exactly as a
   passed rule would, and the `bid` and `pagat` rules are told under every
   `values`, `both_pass` and `ultimo` (the sources state them for the
   standard game). Honesty: acceptable only because `tutor.method` was
   rewritten to say "The rules are the advice of the standard Tarock books;
   where the books differ, a plain fact is told instead", which is true of the
   code. It is hidden behind the fold, and the rule bullet itself makes no
   claim. Fix, either: (a) until M5, `pick` tells only `Passed` rules (every
   skill then shows its fact; simplest and as the spec); or (b) keep the books'
   advice, record the decision in `TUTOR.md` 6 and 9 (a dated decision, its
   source), and set `under` for the `bid` and `pagat` rules to
   `Settings::default()` so that they are not told under settings the books do
   not describe. `lessons.rs`'s own header comment already describes (b).
3. **`TUTOR.md` 9 prints text the code no longer has.** `tutor.method` in the
   table still says "Each rule was measured against that player's own play
   before it was added", which is false today and differs from
   `web3d/strings.json`; the table also names `tutor.miss.off.figure.plain`
   where the code, `MESSAGES.md` and `strings.json` have
   `tutor.miss.off.figure-plain`; and `tutor.lead.try`, `tutor.lead.deal-n`,
   `tutor.well.mastered-two/announced/ultimo/mond` are not in 9's table. Fix:
   an "as built" line under 9 that points to `MESSAGES.md` "Tutor keys" as the
   source of the words and states the method text now in force.
4. **The evidence timing in the documents is not reproduced**
   (`PROTOCOL.md`: "60 to 70 ms median, 120 ms at most over six deals";
   `TUTOR.md` 10: "60 to 120 ms in the module"). Measured here, release wasm in
   Node, `process.cpuUsage` (so load does not count), VM load under 1, three
   sittings of six deals, human playing its first legal action: evidence median
   105-115 ms, maximum 124-135 ms; `brief_of_record` median 104-107 ms, maximum
   117-127 ms. (A first run at load 2.5 gave a median near 200 ms wall.) The
   sitting's brief re-runs the advisor on every deal, because the misses are not
   stored: 0.58-0.63 s for six deals, 1.14 s for twelve, on the main thread at
   the sitting's end, with the Worker not built. Fix: state the numbers with the
   conditions (machine, load, how the deals were played); and either show a
   working state before calling `strohmandeln_brief_sitting`, or have the table
   keep each deal's brief miss (the example key and params) so the sitting's
   brief sums stored data. Re-run the `lat` check after the fix; M7 is still
   open for wasm.
5. **No test holds the learner-level acceptance with the constants as set**
   (`tutor.rs:714-722` `PARAMS`; `TUTOR.md` 12: a lesioned student gets its
   skill as focus within K deals, whole rung 4 gets none). `PARAMS.check()` is
   tested (`brief_tests.rs:756`) and the constants are labelled provisional with
   M6 as owner (`PLAN.md` carries the M6 open item), which is fine. But
   `learn` checks them in debug builds only (`debug_assert!`), so the release
   module never does, and nothing yet shows that `min_gain` 2.0, `costly_gain`
   8.0 and `costly_chances` 8.0 (Cassino's 0.5, 2.0, 8.0, in a different unit)
   ever name a focus for a lesioned student or leave rung 4 without one. The
   brief is only as good as the focus. Fix: record in `PLAN.md` that the brief's
   focus is unvalidated until M6; call `PARAMS.check()` once in the first wasm
   call that learns (or a `const` assertion).

### Nit

6. **The nudge opens `announce` at any decision where an announcement is
   legal** (`tutor.rs:648-650`). The evidence judges `announce` only on the
   `Announce` action or the play of an honour's card; a position where the player
   plays another card has no verdict. The test (`tutor_tests.rs`, "every chance
   the evidence judges is open") is one-directional, so over-opening passes. The
   effect is that "You hold an honour: announce it" appears at the first turn
   with the honour in hand, which is the sensible moment; the cost is that the
   assisted mark lands on that turn, and the later silent play of the honour
   card, which is the judged one, stays unassisted when the player ignored the
   nudge. `assisted_actions` chains only an `Announce` to its play. Either say so
   in `open_skills`'s doc ("a superset of what the evidence judges") or open
   `announce` only where a legal card belongs to an announceable honour.
7. **No twin-deal test for the nudge.** `the_evidence_never_reads_a_hidden_card`
   covers `decision_verdicts`; `open_skills` and `nudge` are built from the
   same `Knowledge::from_view` and `classify_play`, so they are blind by
   construction, but `TUTOR.md` 12 asks the test for the nudge too. Add
   `blind_check_values(&|v| open_skills(v, tiny), 6)`.
8. **`brief_sitting` accepts a deal with no `deal N` line**
   (`wasm/tutor.rs:57`, number defaults to 0). A number of 0 is the single
   deal's "this deal" (`brief.rs:313`), so a sitting's example would say "This
   deal" with no number. Return `bad("deal number")` when a sitting's deal has
   none (or 0).
9. **`level` is any finite number** (`wasm/tutor.rs:126-135`); `PROTOCOL.md` says
   1 to 4. Harmless (`higher_than_top` is `level < 4`), but say "any real; below
   4 means a higher opponent exists", or clamp.
10. **`PROTOCOL.md` slips.** "Five of the calls are stateless": four are
    (`evidence_of`, `learner`, `brief_of_record`, `brief_sitting`). The learner
    example has `"version": 2` beside a `v3` summary. The record is "about 2 KB"
    here and "about 1 KB" in `TUTOR.md` 3 (the test asserts under 4000 bytes).
    `strohmandeln_nudge` with no sitting answers `{"error":"no-sitting"}`, which
    the text does not list. `TUTOR.md` 10 says each sitting deal has its number;
    see 8.
11. **A mastery earned in a sitting is announced twice**: in the brief of the deal
    that earned it and again in the sitting's brief, which is told against the
    history before the sitting (`brief.rs:431`). Defensible (the sitting's brief
    is its own "game"); say so in `TUTOR.md` 6 or compute the sitting's
    `before` over history plus all but the last deal.
12. **`tutor.well.announced` names the first honour announced in the deal**
    (`brief.rs:272`), which may be one announced at a nudged (assisted) decision
    while the praised chance is another. Rare; take the honour from the
    unassisted `announce` chances, or accept.

## Checked and correct

- **What went well.** The kit puts a skill just mastered first, only with
  `before.games >= MIN_HISTORY` (2 earlier deals), else the first strength whose
  `praises` does not contain the focus; the focus can never be a mastered skill.
  Luck-borne strengths (ultimo, Mond) are last, praise only, the Mond only for a
  single deal (`brief.rs:289-295`). Announce praise needs every unassisted
  announce chance met.
- **The rule by the direction of the costliest miss.** `focus_bullets` takes
  `played.misses[skill]`, which is the costliest per skill in `evidence` and, for
  a sitting, the costliest across deals (`>`; ties to the earlier deal). Worth:
  timid -> rich, greedy -> poor; off: pip/figure; pagat: cash at 7 or fewer,
  keep at 12 or more, the fact between; bid only for a pass; behind has no rule;
  a focus with no miss this deal gets no rule for a directional skill (the fact)
  and a try. `Direction` is set to the right class, used only on misses.
- **The example is from the Miss with real cards**: `played`, `better`, `led`,
  `trick_thirds` (led + the taker's value, the taker being the better card for a
  timid miss), `tarocks`, `sure_thirds`; a card or suit missing gives no example
  and falls to a try. The figure example names a later trick only for
  `Consequence::FellUnder`, found from the public tricks after the last card.
- **The next opponent up** only when `level < 4` and at least `ENOUGH_SKILLS`
  skills have enough chances; "too early" otherwise. A short deal (`FEW` 4
  decisions with a choice, unassisted) is one bullet.
- **The sitting's brief** is the deals' evidence summed (tallies, decisions,
  facts), the costliest miss per skill with its deal number, the same `compose`.
- **The nudge** is a pure function of the human's view (`Knowledge::from_view`,
  the same budget as the evidence, so `judged` is a subset of `open`); bidding
  only where `Take` is legal; nothing off the human's turn or with no focus;
  the wasm call reads `my_view`, changes nothing (the smoke test compares the
  state before and after) and reports `deal_action`. Once a position and once a
  deal are the table's (`main.js` `nudgeKey`, `nudgedDeal`); marking assisted is
  the table's (`deal_action` goes into its list).
- **Wasm calls**: stateless (`evidence_of`, `learner`, both briefs) with errors as
  values (`bad-input` with `why`, `replay` with the core's `ReplayError`; a brief
  call also says `brief: null`). Of the 30 inputs tried on fresh instances (empty,
  newlines only, `==` only, no `==`, `level` missing, `NaN`, `1e999`, negative,
  duplicated, `++` only, `++` before a record, a short, repeated or empty deck, no
  actions, flipped seats, settings `{}`, dealer 7, multiplier 0 and 4294967295,
  seat 5, 50 000 assisted numbers each 20 digits, 200 000 assisted numbers,
  negative numbers, a doubled `==`, a missing newline, 100 KB slug, garbage and
  out-of-range summaries) all but the doubled-actions record give an error value
  or a sensible reply; that one is finding 1. Huge or out-of-range assisted
  numbers are ignored (`covered.get`), a summary with an out-of-range tally is
  stale, not a trap. `set_focus` trims, rejects other case and spaces, leaves
  the focus unchanged on a bad slug. The module's `Host` keeps only the focus.
- **`review.rs`**: `try` and its three keys are gone (`KEYS`, the code, the tests);
  `valat-you`/`valat-opp` are in `KEYS`, in `MESSAGES.md` (the test holds it), and
  take precedence in `decided`; the sitting's review is the tally and the worst
  (else best) deal's `decided`, two bullets, as `TUTOR.md` 7; `PROTOCOL.md`
  and `smoke.mjs` say two.
- **Learner constants** pass `Params::check` (tested); they are Cassino's ratios
  with gains in thirds and are labelled provisional (finding 5).
- **Kindness and size**: the brief tests hold at most three bullets, about 45
  words, both languages, the parameters each template names, and that no bullet
  names the person.

## Verdict

Not done until finding 1 is fixed (a corrupt or tampered stored record can kill
the module the game runs in) and a decision is made and recorded on 2 and 3
(tell facts until M5, or record that the books' advice is told, bounded to the
default settings). The rest are documentation and small guards. The brief's logic
matches the spec; the nudge does not leak.

## Second pass (2026-10-15)

Scope: `git diff 5395639 6eee7c4` (commits f3f0ab8, 9d85656, 83ba1d7, 6bbb9e9,
b5dc8a9; the page rebuild skipped), engine and wasm half. Branch
`worktree-agent-abd5f18ed8764998e`, built in a worktree of its own.

Run: `cargo test -p strohmandeln-core -p strohmandeln-ai -p strohmandeln-wasm`
(core 151, ai 219 with 16 ignored, wasm 43 + 12 + 7 + 2 + 3, all pass),
`cargo fmt --check` clean, the wasm built for `wasm32-unknown-unknown` release and
`node crates/strohmandeln-wasm/tests/smoke.mjs 2` passes ("136 odd inputs answered,
none trapped"; 6 deals through the tutor's calls, 5 nudges). Beyond the suite, a
Node script drove the release module with a played sitting's details: every tally
and `decisions` set to 4294967295 (one deal, then three), a miss with every number
at 4294967295 and `cost` 1e308, `deal 0` and `deal 4294967296`; all answered (an
error value or a brief), and the module still rendered afterwards.

### First-pass findings, checked

1. **Fixed.** `DealRecord::replay` tries each action on a clone and calls `before`
   only after the engine accepts it; `decision_verdicts` returns early for a seat not
   to move and for a bid not in `legal_actions`. Cost: a `Deal` is a few hundred bytes
   of small vectors and a record has at most about 60 actions, so the clone is
   microseconds beside the advisor's 100 ms. Refused attempts are not in a record at
   all (`session.rs:272-293` pushes to `deal_actions` only after `deal.apply` is
   `Ok`), so no honest record has an action `before` now misses. The tests hold
   it (`an_action_the_engine_refuses_is_never_shown_to_the_caller`,
   `a_record_with_a_bid_after_the_bidding_is_an_error_not_a_panic`).
2. **Fixed.** `lessons::TOLD = [Passed]`, `pick` and `taught` take the admitted
   gates, `Played.told` carries it, and the `bid` and `pagat` rules are told only
   under `Settings::default()`. A test shows no rule is told for any skill,
   direction or tarock count today. TUTOR 6 records the decision with its date.
3. **Fixed** except one stale line (new finding 2).
4. **Fixed.** The advisor runs once per deal, when it is scored; the sitting's brief
   is made of the kept `Detail`s and took 2 ms for six deals here (a deal's own
   evidence took 114 to 227 ms at a load of 3 to 5, which bears out "can exceed 150
   ms on a loaded machine" in PROTOCOL "Timing"). The table shows the chip as working.
5. **Fixed as asked**: `constants()` checks the ladder and `PARAMS` once, in the
   release module, and the docs say the focus stays unvalidated until M6.
6. **Fixed**: `announce` is open only where a legal card belongs to an announceable
   honour, and the test is two-directional.
7. **Fixed**: `the_nudge_never_reads_a_hidden_card`.
8. **Fixed**: a sitting's deal needs `deal N` with N of 1 or more.
9. **Fixed**: `level` is 1 to 4.
10. **Fixed**: the PROTOCOL slips (four stateless calls, `"version": 3`, 2 KB,
    `no-sitting`).
11. **Fixed** by learning the sitting deal by deal; the test shows a mastery told once.
12. **Fixed**: the praised honour is one announced at a decision that counted
    (`announced_by`), tested with every action marked assisted.

### New findings

1. **should** `crates/strohmandeln-ai/src/brief.rs:411,415` (and the wasm test gap).
   `brief_from` adds the details' tallies and `decisions` with plain `+=` on `u32`,
   and a detail comes from storage. The release module wraps (no trap; I fed it
   three deals of 4294967295 each and it answered), but any build with overflow
   checks, the test profile included, panics. The wasm smoke test and
   `nothing_the_calls_are_given_panics` feed `brief_sitting` records only, never a
   mutated detail, so "no input makes the module trap" (PROTOCOL) is held for a
   detail by `detail.rs`'s own unit tests and by my run alone. Fix: `saturating_add`
   in `brief_from`; add extreme-number and out-of-range details to
   `nothing_the_calls_are_given_panics` and to the smoke test's section 7.
2. **nit** `docs/TUTOR.md:349` still says "tries at most 12" (code `MAX_TRY_WORDS`
   is 14, as MESSAGES.md and TUTOR 9's as-built note say). The sample sentence in
   section 6 ("his king") is the first draft's; section 9's as-built note says where
   the words are, so a line there would do.
3. **nit** `from_json` (`wasm/src/detail.rs`) refuses a detail whose summary is of an
   older evidence version, which fails the whole `brief_sitting` call. That is right
   for the engine; the table must therefore not hand it a stale detail (table log,
   finding 1).

### Checked and correct

- `detail.rs`: every field is read as `Option`/`Result`, no `unwrap`, a card must
  `Card::parse`, `cost` is finite and not negative, settings go through serde, the
  summary must read and be current; the sample goes to JSON and back unchanged and
  stays under 1,500 bytes, and `brief_of_record`'s `detail` equals `evidence_of`'s.
- `brief_from`: a sitting's `before` is its `after`, so a mastery is not announced
  twice; the number 0 is the single deal's "this deal"; the costliest miss across
  deals keeps the earlier deal on a tie.
- Provisional rules are dead in the product until M5 sets a gate, as the spec says;
  the method text says so truthfully.

### Verdict

Every first-pass engine finding is fixed and the tests hold the fixes. One should
(the unchecked sums, and no mutated-detail case in the smoke test) and two nits
remain; none blocks the work package if recorded as open.
