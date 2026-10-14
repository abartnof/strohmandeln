# Working on this repository

Strohmandeln, the Austrian two-hand Tarock game, as one offline web page, built
like the earlier games in the series (Piquet, Bezique, Cassino). Start with
`strohmandeln-lit-review.md` (what is known, every claim cited) and
`research/14-consensus-rules.md` (the canonical rules and the only settings).

Standing rules:

- **Test-driven.** Write the test, watch it fail, then make it pass. This holds
  for the engine, the table and the research scripts in `research/sim/` alike:
  a computed number that a document relies on has a test or a check that
  reproduces it.
- **Atomic commits, often.** One logical change per commit, with a message
  that says what changed and why. Never mix a rules change, a refactor and a
  document in one commit. Keep `PLAN.md` current as work lands, once it exists.
- **Simulations stop early when the signal is clear.** Fix the comparison, the
  batch size, the maximum number of batches and a stopping boundary
  (O'Brien–Fleming) before the first batch. Run in batches and look between
  them. Stop as soon as the effect crosses the boundary, or when it is plainly
  too small to matter, and report the interval at the last look. Never peek
  with an unadjusted p < 0.05. Always use mirrored pairs: each deal played
  twice, seats swapped.
- **The rules are canonical.** Each rule is the consensus of the legitimate
  sources (`research/14-consensus-rules.md`). A setting exists only where a
  disagreement persisted through the game's history. Don't add variants
  without the maintainer asking.
- **Every claim about the game is cited.** Research notes cite the source they
  read; nothing about the game's history, rules or table talk is stated from
  memory. Downloaded sources stay in `research/sources/` (not in git).
- **Period voice, kindly.** The opponent's lines come from period sources
  (`research/05*`), with the kindness rule: luck, speed, the score and the run
  of play are fair game; looks, intelligence, age, gender, class, religion or
  nationality never are.
- **Let agents do the work.** The orchestrating session (Opus) plans,
  briefs, decides and checks; reading, research, coding, testing and review
  go to agents wherever the work can be handed off, in parallel when the
  pieces are independent.
- **Every module is reviewed.** When a module or area of code is written
  (engine, kit, table, faces, phrase bank, opponent, tutor, build), an agent
  that did not write it does a code review: correctness against
  `docs/RULES.md` and the tests, bugs, simplicity, and fit with the rest. Its
  findings are fixed, or recorded as open in `PLAN.md`, before the work
  package is called done. Review logs go in `docs/review/`.
- **Only the orchestrating session edits `PLAN.md`.** Agents report what they
  did; the plan is updated from their reports. That keeps parallel branches
  from colliding in it.
- **Agents run on Sonnet or smaller.** Pass `model: "sonnet"` (or `"haiku"`
  for simple, mechanical work) on every agent call; never Opus.
- **Documents are public-facing, and terse.** Write notes the way a software
  project keeps them: what was decided, the evidence, what follows. No
  narrative of who said what or why they felt it ("the maintainer called this
  a blind spot", "after the review the coordinator realised..."). A decision
  is recorded as a decision with its date and its source, nothing more. The
  person who commissioned the game is "the maintainer", never named.
- **Consult the maintainer before anything that spends money** or any long
  compute run, with an estimate in hand.
