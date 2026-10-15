# Review: the tutor (work package 9)

Scope: commits after 17ad01d (`crates/strohmandeln-ai/src/review.rs`,
`crates/strohmandeln-wasm/src/lib.rs` review/explain, `docs/MESSAGES.md`
"Review keys" and "Explanation keys", `docs/PROTOCOL.md` "The review",
`web3d/src/{tutor,review,tutorview,progress}.js`, `tutor.css`, the hooks in
`main.js`, `ui.js`, `talk.js`, `engine.js`, `strings.json`, `tutorial*.md`, tests)
against TABLE3D 6, RULES 3.4, 5.2-5.4, 8.1, 10-14 and CLAUDE.md. Node tests for
tutor, review, tutorview, progress, strings, talk, tutorial pass; cargo and the
browser were not run.

## Findings

Severity: bug, spec gap, nit. Ordered by severity.

### Bugs

1. **German gender slip in `try-announce`** (`web3d/strings.json:179`,
   `docs/MESSAGES.md:99`). "angesagt, wird er bezahlt" refers to `card.honour.trull`
   ("die Troule") or `card.honour.kings` ("die vier Könige"): "er" is wrong for
   both. Fix: "angesagt, wird sie bezahlt." The English has the same slip for the
   kings ("did not announce it ... it is paid"): "announced, they are paid" does not
   fit the Trull; reword to "announce it and the Point is paid" or take the honour
   as an object that has a number (`{honour}` plus a verb-free clause: "announced,
   that is a Point").
2. **Valat is invisible to the review** (`review.rs:172` `decided`, `:90` `read`).
   A Valat (RULES 10, 12 Points, replaces the game payment) is read nowhere: the
   `Valat` event is ignored, so a 27-trick sweep is described as "the rich trick you
   lost" or "level", and the sitting's worst-deal pick works only by `net`. Fix: add
   `valat-won-you` / `valat-lost-you` keys (en+de, MESSAGES.md, KEYS) as the first
   `decided` case after the ultimo check (or before: it is worth more), with a test
   that sweeps 27 tricks.
3. **Shade click or Escape on a due page marks it seen** (`main.js:229`
   `dismissPage(false)`, shade `onClose` at `main.js:226`, Escape at `main.js:1062`).
   6.2.2: only an explicit choice marks seen; an accidental click outside discards
   the page for good ("Remind me" is the only way back, otherwise Help). Fix: let the
   shade click and Escape call `dismissPage(true)` (remind) for a due page, keep
   "Not now" the only seen-marking act.

### Spec gaps

4. **`M` key not wired** (TABLE3D "M mute the tutor"; `strings.json` `keys.mute`
   exists, unused; the `?` list at `main.js:1081` omits `mute`). Fix: `if (key ===
   "m") return tutor.toggleMute();` beside `h`, and add `"mute"` to the keys list.
5. **`touren-slate` second moment never fires**: `tutor.slateOpened()`
   (`tutor.js:85`) is not called anywhere in `main.js`; the slate opened by hand
   never opens the page (6.2.2 "or the first slate opening"). Fix: call it in
   `toggleSlate` when the slate opens.
6. **6.2.3** (pages openable from the review) and **6.4.3** (heavy calls in a
   Worker, `?noworker`) are not implemented. The review is two synchronous wasm
   calls on the main thread (`main.js:241`). Record as open in PLAN.md, or amend
   TABLE3D 6.4.3 (a review of a sitting is cheap: one pass over the events).
7. **Sitting review drops the deal** (`review.rs:410` pushes `deal`;
   `review.js` `words` never shows it; the `decided` text says "In trick 4" with no
   deal). Fix: add a `{deal}` clause for the sitting's moment, or leave the param
   off the wire.
8. **Draw page under `throw_in` is self-contradicting** (`tutorial.en.md` /
   `tutorial.md`, "When nobody reaches 36"): the kept bullets "The next deal played
   counts double ... ×2" describe a doubling that never arises (RULES 14.6). Move
   them into the `both_pass=play` block, keeping "Honours and the Pagat are still
   paid" outside.
9. **Explanations tied to the Tutor setting** (`main.js:437`
   `prefs.explain && tutor.active()`): 7.1 gives "Explanations" its own switch and
   "Tutor off" only "no pages, no tips, no review chip". With the tutor off the
   explanation line is dead though the player set it on. Muting is 6.1's (explicit),
   so gate only on `!prefs.muted`, or amend 7.1.
10. **`docs/PROTOCOL.md:361` and `MESSAGES.md:80`** say `lead` is `count`, `decided`
    or `try`; the sitting adds `sitting`, and `review.rs` types the lead as
    `sitting`/`explain` as well. Add them.

### Nits

11. **`try-count` can win over a concrete tip in a sitting** (`review.rs:434`):
    ties go to the first, and `try-count` is the filler. Count the specific tips
    first (skip `try-count` unless nothing else occurs).
12. **`ultimo-lost-*` de "gestochen"** (`strings.json`, MESSAGES): the Pagat is
    beaten by a higher tarock, not trumped; "übertroffen" or "geschlagen" is the
    period word (explain keys use "schlug").
13. **Unused strings**: `tutor.muted`, `tutor.page.close`, `tutor.speech`,
    `tutor.off`, `ui.review.sitting`, `keys.mute`. Use or delete (strings test
    would catch the orphans if it were asked to).
14. **Voice**: the `kibitz` intro group (`words.json` `kibitz.0`, `.3`) are lines
    said *to* a kibitzer ("§ 13: Kiebitz, halt's Maul!"); spoken by him they read
    as a self-mute. 6.5 asks for "if I may". Not a kindness breach; re-pick the
    group for the intro or add a few "Nicht dreinreden, aber ..." lines.
15. **Test runs write real progress** (`main.js:536` `storage()` is not gated by
    `TEST`): browser scenarios pollute `strohmandeln.progress`. Pass `null` when
    `TEST && !params.has("keep")`.
16. **`MESSAGES.md` `level` Basis cell empty**; give it "none (no key moved the count)"
    or the nearest RULES clause, as the other rows do.
17. Tests: `the_explanation_follows_each_trick_with_a_listed_key` (lib.rs) checks
    only that the key is listed, not that it is the right deduction; the review.rs
    unit tests cover the deductions, so this is a wiring test: rename it.
18. `tutor.css`: the mute and review chips use hex colours outside the existing
    palette variables; fine for a paper note, nothing to fix unless the kit has a
    token for it.

## Checked and fine

- **Information.** `review`/`explain` read `filter_events` (the public log with the
  human's own `dealt`); `read` returns `None` for an unscored deal, so a mid-deal
  `review_sitting` ignores the running deal; after trick 27 every card is in the
  log. The explanations use only cards played, and "in hand or showing" is
  RULES 5.2/5.4's playable set (hand plus covers; a face-down packet card is not
  playable and may be of the suit, so the claim is not "has none").
- **Explanation deductions** equal RULES 5.4 exactly: trump on a suit led -> no suit
  in the playable set (rule 1); other suit -> neither suit nor tarock (rules 1, 2);
  suit card on a tarock led -> no tarock (rule 1); following suit or tarock proves
  nothing, so it names the winner (5.5). Trick pairing takes the two plays nearest
  before the last `TrickWon` with that number, so repeated trick numbers across
  deals are safe.
- **Hand tracking** (`read`): `dealt` + own `card_pulled` - own `card_played`;
  covers never enter the hand (11.2, 4.8); `held` checked after every event, so a
  honour completed by a late pull counts (6.4); `announced` is the human's only.
- **Wasm wiring**: `review`/`explain` take `&self`, do not move the game (tested);
  JSON shape matches PROTOCOL; `null` before a scored deal/trick.
- **Sitting_end** is recorded once: live `endSitting` only from the animated
  event; a restored finished sitting passes `live: false`; `summaryOf` is null
  when no deal scored.
- **tutor.js state machine**: one page at a time (`current`); `blocked` keeps
  events owed; `shown` stops a reminder repeating in a deal and clears at
  `deal_start`; `reset` in `resetTable` clears everything; muted or off drops owed
  pages but Help works; the mute says its line once and un-muting is silent; the
  setting un-mutes (6.1); the chip is hidden when muted; a muted hint asks "§ 13"
  (6.3.4). Language switch redraws only the open page and closes the lists.
- **Dialog ids**: `closeDialog` does not call `onClose`, so "Remind me" is not
  turned into "Not now" by a re-entrant close; reopening the same id replaces it.
- **tutorialPages**: variant blocks drop whole, markers included; tests cover both
  languages under both settings, line and page word limits, and same shape.
  The throw-in sentence matches RULES 3.4.
- **German** is formal Sie throughout; Sküs is masculine ("dem Sküs"); "Partien"
  for deals as in the earlier pages; kindness rule holds (luck, cards and the run
  of play only; "reiche, den Sie abgaben" names a trick, not the person).
- **progress.js**: try/catch on every access, newer-version store untouched,
  30-sitting cap, sound() validation on load and save.
- Docs terse and cited (RULES clauses per row); the MESSAGES test splits the hint
  section from the review section correctly.
