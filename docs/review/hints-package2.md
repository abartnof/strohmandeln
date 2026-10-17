# Review: hints package 2 (Why?, M5r `so`, tarocks_out, plain price words)

Scope: `git diff main...HEAD` on `worktree-agent-ab861426e0d012b27`
(ask.rs, hint.rs, wasm lib.rs, hinttext.js, ui.js, strings.json, docs).
Spec: `docs/HINTS.md` 0, 3.1, 5, 7; `docs/RULES.md`.

Gate: cargo fmt, clippy -D warnings, cargo test, node --test kit/test web3d/test
(635 pass), kit/build.py --check, tools/phrases.py --check: all green after the
fix below. (Node tests and build --check need the wasm in `target/`; with a
private target dir, link `target` to it.)

## Checked, no finding

- M5r `so` gating: `so_for` takes only `m5r::matching` rules (ships, rule holds
  at this position, the hinted card is in the class) whose class answers the
  chosen question, and `passes_under` requires n_eff >= 60 and lower >= 0.80 in
  the settings row for every setting the sitting changes; a rule with no row is
  not told. All three Settings fields are covered (values, both_pass, ultimo).
- `worth.rich` (M5) is not gated by settings: confirmed under every setting
  (MEASUREMENTS.md M5), so correct.
- `tarocks_out` is `Knowledge::out_tarocks()` (not mine, not played), as the
  Cards-out tab counts.
- Wire: PROTOCOL.md and MESSAGES.md match `hint_line`/`why` in wasm; null
  fields on a non-turn hint tested; Why? at most two facts, 90-word budget in
  hinttext.js, keyboard open/close and Escape in overlays.js.
- No "cheap/cheapest/dear/costs/billig/teuer/kostet" in any `hint.*` string,
  hinttext.js or M5r wording (tests hold it; German uses wertniedrigste,
  punkteärmste). Remaining hits are keys, identifiers and comments.

## Findings

1. Low, fixed. `ans.trumps-no/-yes` said "you hold 6 of the 22 still out", but
   "still out" means not yours everywhere else (Cards-out tab, `fact.tarocks-out`),
   and 22 includes the player's own. Now "of the 22 tarocks left" (en); German
   "im Spiel" was already right. Test, strings, MESSAGES.md updated.
2. Low, fixed. `docs/HINTS.md` 6 answer table still gave the `cheap` rule as
   "costs less" / "billigere", words the same document bans (section 7) and the
   engine no longer says. Row now gives the M5r wording.
3. Low, open. `passes_under` checks each changed setting alone; a sitting that
   changes two settings at once relies on the singles (M5r measured singles
   only). Acceptable, stated here.
4. Low, open (outside this package). `tutor.name.cheap`, `tutor.rule.cheap`,
   `tutor.fact.cheap`, `tutor.try.cheap.*`, `tutor.nudge.cheap` still say
   "cheapest" / "billigsten" / "teurere" to the player. They belong to the
   tutor package, not the hints; the plain-words rule should be extended there.
5. Info. `hint.vs.lead` (de) is clumsy ("gab ... her, wenn nicht") but correct.
