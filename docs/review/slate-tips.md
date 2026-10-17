# Review: the slate's tooltips (TABLE3D 4.7)

Reviewed: branch `worktree-agent-a3dd172e968bbfe39` (6 commits over main, head 1a5ec25), 17 Oct 2026.
Scope: `slate.tip.*` in `web3d/strings.json`, `web3d/src/slate.js`, `web3d/src/table.css`,
`web3d/test/slate_tips.test.js`, browser check `slate_tips`, TABLE3D 4.7.

## Runs

- `node --test web3d/test/slate_tips.test.js`: 10/10 pass.
- Browser `slate_tips`, `slate_open_close`, `slate_points_and_settlement_note`: ok.
- Browser `slate_rows`: FAIL "no doubled deal in the seeded sitting". It is the same cause the last
  commit fixed in `slate_tips` (the seed no longer reaches a doubled deal); `slate_rows` was not
  touched by the branch (not re-run on main, so "pre-existing" is inferred). Open; fix by driving to a doubled deal as
  `slate_tips` now does for its first scored deal, or by a seed that reaches one.
- The rest of `web3d/test` needs the wasm build (ENOENT on `target/.../strohmandeln_wasm.wasm`);
  not run.

## 1. Truth against RULES 9-12, 14, 15

Checked each template for each case against the rules and the engine's `score()`
(`crates/strohmandeln-core/src/deal.rs` 595-700). Credits are always to the receiver; the
`you`/`opp` split in `creditKind` is by the credited seat, and every sentence follows from that.

| Case | Verdict |
|---|---|
| Declarer wins / loses (9.2) | True; credited to the declarer / to the defender. |
| Simple game won (9.2, 8.4) | True. |
| Draw (8.4, 9.2, 11.7, 12.4, 14.1) | True: no game payment, honours and ultimo paid, next deal doubled. |
| Doubled deal (14.1-14.4) | True, including the thrown-in variant (flag not consumed). |
| Trull, four kings (11.1) | True: 1 Point, paid to the announcer. |
| Ultimo won / lost (12.2) | True: the credited player is the other one for "lost". |
| Valat (10.1, 10.2) | True: 12, replaces the game payment (the engine emits no `game` credit with a Valat). |
| Throw-in (15.3) | True: counts as a Tour deal. |
| Conto (15.5) | True: gains only. Tour line "n of them in this Tour" is `end - previous end`, correct. |
| Balance, direction, settlement (15.8) | True: payer has the smaller Conto, `|balance| x 2`. |

Five tips from the probe sitting (human at seat 1; deal 1, seat 0 dealt, `declarer_loss`, credits
seat 0 game 4 and seat 1 ultimo_lost 1; Contos 4:1; sitting end payer 1, 6 kreuzer), read against
the record:

1. "Game: you declared and lost, so you pay your opponent 4 Points (the declarer-loses value)." seat 0 credited 4 on `declarer_loss`: correct.
2. "Pagat ultimo lost: your opponent lost the last trick with the Pagat, so they pay you 1 Point." seat 1 credited: correct.
3. "Your opponent's account at deal 1: 4 Points ...": Conto[seat 0] = 4: correct.
4. "Difference of the two accounts: your opponent is 3 Points ahead, which is 6 kr. ..." 1 - 4 = -3, x2: correct.
5. "You pay: your account is the smaller one, by 3 Points." payer 1 = human: correct.

Findings on truth and wording:

- **T1 (medium) `slate.tip.total.*`: the kreuzer figure is misleading.** "Total of this deal's entries credited
  to you: 4 Points, which is 8 kr. at 2 kreuzer a Point." Money moves only on the difference of the
  two accounts at the end (15.8); a deal's one-sided credits are not an amount owed (the other player
  may have credited entries in the same deal). The screen already shows "4 Pt · 8 kr." so the
  tip should explain it rather than repeat it. Fix: "Total of this deal's entries credited to you:
  {pts}. Money is settled on the difference of the accounts at the end, at 2 kreuzer a Point."
- **T2 (low) doubled game tips contradict themselves.** "pays you 4 Points (the declarer-wins
  value). Counted double: 2 x 2." The declarer-wins value is 2 here. Fix: put the doubling first or
  say "(the declarer-wins value, doubled)" and drop the clause "(the ... value)" when `x2` is set.
- **T3 (low) the on-screen label is "Balance" ("Saldo"), the tip says "Difference of the two
  accounts".** A newcomer cannot connect them. Fix: start the tip with "Balance:".
- **T4 (low) `header.rules` gives "Pagat ultimo 1" with no unit.** Fix: "Pagat ultimo {ultimo} Point(s)" via the plural helper.
- **T5 (low) `note.draw` says "36 card points"**; the threshold is 35 2/3 (8.2). The game shows 36 as the round figure elsewhere (hint strings); acceptable, say "36 (35 2/3)" only if the maintainer wants exactness.

## 2. Newcomer clarity, length, language

- **L1 (medium) the 25-word limit is exceeded and the test cannot see it.** `header.rules` filled with
  `both_pass = play` is 28 words ("... Pagat ultimo 1; if both pass, the deal is played as a simple
  game."). The test substitutes `{pass}` with one word. Fix: shorten (drop "Settings of this evening:",
  or shorten `pass.play` to "both pass: simple game") and make the test fill `{pass}` with the real
  strings for both values.
- **L2 (medium, German) the declarer is "Spieler" / "angesagt", not "Aufnehmer".** Elsewhere the
  German strings say "Aufnehmer" (hud.declarer, banner.declarer-win, sr.game_scored, settings.values.note).
  `header.rules` de has "Spieler gewinnt ... verliert"; the game tips have "Sie haben angesagt".
  Fix: "Aufnehmer gewinnt 3, verliert 4 Points" and "Sie haben aufgenommen und gewonnen". English
  keeps "declared" as the generic verb elsewhere in the tips while the HUD says "Declarer"; use "took
  it up" or leave, but the German must match.
- **L3 (low, German) "der Wert für gewonnen" / "für verloren"** is stiff. Fix: "(der Wert für ein gewonnenes Spiel)" / "(... für ein verlorenes Spiel)".
- **L4 (low, German)** "Zeigen Sie auf eine Zahl, oder gehen Sie ..." has a comma before "oder" joining two imperatives
  with different subjects; fine. "Endete der Abend jetzt, wäre ..." and "zahlte Ihr Gegner ... wenn der Abend jetzt
  endete" are correct Konjunktiv II. "Points" (game) and "Augen" (card points) are used consistently
  with the rest of the strings; English "card points" / "Points" likewise. "Schluß" matches `slate.settlement`.
- Length: the longest filled tip other than L1 is 24 words; all read as one idea.
- Under Random style no template names the style (checked: none mentions a style).
- Generic declarer is "they" where it appears ("they pay you"): ok.

## 3. Keyboard and assistive technology

- **K1 (medium) Down/Up skip the settlement lines.** `settlement()` builds the heading (`h3`) after
  `line += 1` for the last time, so the heading has the highest line number (8 in the probe) while it
  comes first in the DOM (before lines 4-7). `ArrowDown` from the foot (line 3) goes to the heading
  (first element with a greater line), and from the heading nothing is greater: the lines 4-7 are
  reachable only with Left/Right. `ArrowUp` from the heading jumps to the last settlement line. Fix:
  take the heading's line number before the lines (`line += 1` once for the heading, then each
  line), and test Down through a settled sheet.
- **K2 (medium) the auto-opened slate folds under a keyboard user.** `open({ms: 3000})` after a deal
  (main.js 735, 739) arms a timer that only a click cancels. Focusing a figure (`focusin`) or
  pressing an arrow key does not, so the sheet closes in 3 s while the user reads a tip. Fix: cancel
  the timer on `focusin` and `keydown` as on `click`; test it.
- **K3 (medium) every event re-renders the whole sheet.** `main.js` 663 calls `slate.update` per event; `render()`
  replaces all children, so the focused figure is destroyed and keyboard focus falls to the body,
  and the caption resets. The rover is kept (by `data-tip` and line) but not focused. Fix: after
  `render()`, if the focus was inside the board, refocus the kept rover; preserve the caption text.
- Escape: closes via overlays.js (the `slate` kind is in `topOf`); the browser check presses Escape
  from a figure and the slate folds. Focus is not returned to the slate tab afterwards (low).
- One tab stop: verified (exactly one `tabindex=0`); arrows, Home/End as specified; `:focus-visible`
  ring in chalk is present.
- **A1 (low) `role="note"` + `aria-description` + the same `title`.** `role="note"` is a static
  structure role, not interactive; `role="button"` would promise an action the figure does not
  have, and a bare `span` may not carry `aria-description` (generic roles prohibit naming
  attributes in ARIA 1.3). `note` is the least wrong of the three and focusable notes work in
  NVDA, JAWS and VoiceOver (they read content, then the description); keep it. Risks: older
  Firefox and Safari ignore `aria-description`, where the `title` is the fallback description,
  so the text is still reached. An alternative with wider support is `aria-describedby` pointing at a
  visually hidden node per tip, at the cost of 100+ nodes; not worth it.
- The caption is `aria-hidden`, so for a screen reader the tip arrives only through the description:
  acceptable; sighted touch and keyboard users get the caption.
- Touch: a tap fires `click`, which both shows the caption and cancels the auto-fold timer (the
  existing handler); there is no hover on touch, so the caption is the only route; verified in the
  test (`mouseover`/`focusin`), the `click` listener is not tested (low).

## 4. Conflict with the dealer-chip branch (`worktree-agent-a386242a28bd5cb19`)

`git merge-tree` of the two: the only conflict is `web3d/strohmandeln.html` (generated; rebuild
after merging). `strings.json`, `TABLE3D.md` and `browser_aids.py` merge automatically. The other branch does not touch
`slate.js`; it changes `slate.dealer` (en "G" to "Dl") and TABLE3D 4.3.1. Semantic effects:

- the dealer mark's tip is `dealer.you`/`dealer.opp` ("Dealer: you dealt this deal"): still right with "Dl";
- TABLE3D 4.7.1 names the dealer mark as `("G")`: stale in English after the merge. Fix on merge: say "the dealer mark".

## 5. Tests

Meaningful: all kinds present in both languages and reached by the sample sitting, no unfilled
slot, `title` equals `aria-description`, the exact text for a declared win, one tab stop, arrow
movement, caption on focus and hover, and the browser check reads one tip against the record.

Gaps:

- **N1 (medium) word-count test is blind to `{pass}`** (L1).
- **N2 (medium) no test of Down/Up across the settlement** (K1) or of the auto-fold timer with focus (K2).
- **N3 (low) the German browser check only tests `"pays you" not in got`.** Make it compare with the
  German template, as the English one does.
- **N4 (low) the "five tips against the record" are written by hand in the test**; they restate the
  templates' text. Acceptable; the browser check is the independent one.

## Summary

No tip states anything false about the rules. To fix before done: L1 (word count and test), K1
(settlement line order), K2 (auto-fold under keyboard), K3 (focus lost on re-render), L2 (German
Aufnehmer), T1 (kreuzer on a deal's total). The rest are low. `slate_rows` is a pre-existing stale
check, to be recorded as open.
