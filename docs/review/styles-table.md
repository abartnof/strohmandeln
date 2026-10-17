# Review: the style row and the dial at the table

Branch `worktree-agent-a5a5f98eae6c311bb` (6 commits after the merge of main)
against main, 17 Oct 2026. Spec: `docs/STYLES.md` 1, 3, 4, 6, 9. Reviewer did not
write the code. Engine parameters read in `crates/strohmandeln-ai/src/style.rs`
and `rung2.rs` (Normal: `STRONG_TAROCKS` 8, `draw_trumps` from 7 at rung 2 and 6
at rung 3, bid 45 thirds and 5 tarocks).

Run: `node --test kit/test web3d/test` 569 pass, 0 fail; `python3 kit/build.py
--check` clean; browser checks `new_sitting_style` and `random_style_hidden`
ok singly. Both are registered in `CHECKS_104`.

## Findings

### H1. Random can draw Werner and Ulmann, which are not offered (Werner has no name)

`Style::POOL` (style.rs) still holds all seven candidates. The page offers five.
Verified in the page's engine: `new` with `style: "random"` draws `werner` for
seed 2 (and 4, 13, 14) and `ulmann` for seed 10; 3 sittings in 7 draw one of the
two. Consequences: the reveal (`style.reveal`, the This-sitting line, the brief)
names a style the player could never pick, and for Werner `style.werner.name`
does not exist, so the key text shows. The browser check passes only because
seed 7 draws Bold and the check asserts `drawn` in the offered five without
ever drawing the others. `reviewstyle.test.js` uses `ulmann` as the drawn
example, which hides it.

Fix (engine, not this branch's files): `Style::POOL` is the offered five
(Bermann, MS, Bold, Cautious, Hunter), as STYLES 2.3 says ("the survivors of
every gate"). `drawn` in a record is authoritative, so old records still
replay. Add tests: every id in `POOL` has `style.<id>.name` and `.line` in
`strings.json` (a strings test over the engine's list), and the browser check
draws seeds 2 and 10 under Random and asserts the drawn id is offered.
Until then, add `style.werner.name` (en "After Werner", de "Nach Werner") as a
stopgap; `OFFERED_STYLES` and `POOL` should be one list (the page's constant
is documented as the single place to edit, the engine's is a second one).

### M1. The style lines overclaim in four places

True of the engine, line by line:

| Line | Engine | Verdict |
|---|---|---|
| Bermann "takes up on points in honours, not on tarocks" | 60 sure thirds (20 points: Sküs, Mond, kings, queens as figures, Pagat with 6 tarocks), no tarock minimum | True of the rule, but it hides that Bermann bids rarely (6% of hands against Normal's 19%, STYLES 9). A newcomer reads "takes up on points" as keen. |
| Bermann "plays a lying figure at once" | `low_shown_exception = false`: an exposed queen or cavall (strength 6, 5; not kings or jacks) is played at once even when two low cards of its kind show | Normal does this too except in that case; as written it reads as Bermann's difference from Normal, and "figure" includes kings. |
| MS "leads tarocks only from twelve" | `draw_from` 12 only moves the `draw_trumps` bonus (+3) to 12 tarocks (Normal 7 and 6). Below it a tarock lead is not forbidden, only unrewarded (`keep-tarocks` still applies); other maxims can still choose one | Overclaims "only". Say it draws tarocks from twelve. |
| MS "a strong hand with six tarocks" | `min_tarocks` 6 is a minimum | Say "at least". |
| Bold "opens contested suits early" | `split_suit` x0.5 halves a penalty on leading such a suit (4.0 to 2.0); it creates no preference | Say "readier". "Contested" is the player's own holding 30 to 70% of a suit's value, which is the hint's term (`hint.rule.finesse`). |
| Cautious "waits for you to open contested suits" | `split_suit` x1.5: the penalty rises to 6.0 | Say "slow to open". |
| Hunter "whenever its tarocks allow" | `strong_tarocks` 6: holds the Pagat for the last trick from six tarocks (Normal eight); bids with 36 thirds and 7 tarocks | Vague, not false. Say what it does. |

Proposed wording (en / de). German: "aufnehmen" for taking up (as
`tutor.rule.bid`, `sr.bid.you`), "Augen sicherer Stecher", "Tarock" as the
plural (see M2), "umkämpft" (as `hint.rule.finesse`), "offene Dame oder Cavall"
(as `hint.reason.lead-exposed`); no "Honneurs" and no "liegende Figur".

- `style.bermann.line`
  en "Takes up rarely, on 20 points of sure winners whatever its tarocks, and plays an exposed queen or cavall at once."
  de "Nimmt selten auf, mit 20 Augen sicherer Stecher, gleich wie viele Tarock es hat, und spielt eine offene Dame oder einen Cavall sofort aus."
- `style.ms.line`
  en "Takes up rarely, on a strong hand with at least six tarocks, and leads tarocks to draw yours only from twelve."
  de "Nimmt selten auf, mit starkem Blatt und mindestens sechs Tarock, und tarockiert erst ab zwölf Tarock."
- `style.bold.line`
  en "Takes up on thin hands and is readier to open contested suits."
  de "Nimmt auch mit dünnem Blatt auf und eröffnet umkämpfte Farben bereitwilliger."
- `style.cautious.line`
  en "Takes up only on strong hands and is slow to open contested suits, leaving them to you."
  de "Nimmt nur mit starkem Blatt auf und eröffnet umkämpfte Farben ungern; es überlässt sie Ihnen."
- `style.hunter.line`
  en "Holds its Pagat for the last trick from six tarocks, and takes up on hands rich in tarocks."
  de "Hält das Pagat für den letzten Stich zurück, ab sechs Tarock, und nimmt mit vielen Tarock auf."
- `style.ulmann.line` (not offered; keep consistent)
  de "Nimmt bei vier Honneurs sofort auf und tarockiert erst ab zehn Tarock."

`docs/STYLES.md` 2.1 and 2.2 carry a related error for the spec: Hunter's tell
"leads tarocks from the start" is false at skill 3 (`draw_from` 8 against
Normal's 6 at rung 3, 7 at rung 2; Cautious's `strong_tarocks` 10 likewise draws
from 8 at rung 3), so Hunter and Cautious draw tarocks later than Normal. The
Hunter's tell is the held Pagat and the bid.

### M2. German: "Tarocke" and "Tarocken" against the pages' "Tarock"

The tutor, hint and brief strings use "Tarock" as the plural throughout ("mit
{n} Tarock", "Zählen Sie Ihre Tarock", "mindestens sechs Tarock"). The new
lines use "Tarocke" (Bermann, Hunter) and "Tarocken" (MS); nothing else does.
Fix: "Tarock" everywhere (wording above). The verb is also "annehmen" in the new
lines against "aufnehmen" in `tutor.rule.bid`, `tutor.miss.bid.take` and
`sr.bid.you` (and the new `style.note.bidding`, which already says "Aufnehmen").
"Honneurs" matches the tutor strings but a newcomer may not know it; the
proposed lines avoid it. "Liegende Figur" is not used elsewhere; the pages say
"offene Dame oder Cavall" and "als Decke".

### L1. Reveal sentence reads badly mid-sentence; "Random (shown at the end)"

`style.reveal` en "...drawn at random: After Bermann." capitalises a school's
"After". Suggest quoting the name: en `Your opponent's style was drawn at
random: "{drawn}".`, de `Der Stil Ihres Gegners wurde gelost: „{drawn}“.`
Same for the This-sitting line ("style After Bermann", "Stil Nach Bermann").
`style.random.hidden` de: "Zufall (wird am Ende gezeigt)". STYLES 5 writes
"Skill 3, after Bermann"; the shipped sentence reads "..., opponent level 3,
style After Bermann". Fine, record only.

### L2. The end dialog's reveal does not follow a language switch

`endSitting` builds the dialog body once with `t(...)`. The `onLang` handler
does not redraw the "over" dialog, so after a switch at the end the reveal stays
in the old language (its title does too, so this is the dialog's existing
behaviour, not new). The menu's This-sitting line and the brief are drawn on
demand and do follow. Record as open.

### L3. A style not offered (imported file) has no name string

`sittingFrom` takes whatever the engine reports; an imported record of style
`werner` or `ulmann` shows `style.werner.name` (absent) in the line, and the
new-sitting select falls to its first option. Fixed by H1's stopgap string.
Rare; no other action.

### L4. The abandoned Random sitting reveals nothing (known)

STYLES 2.3 asks for a chip on leaving a Random sitting early; TABLE3D 7.3 and
the branch state it open. Needs a PLAN entry; not a defect of this branch.

## Checks of the brief's list

1. Descriptions against the engine: M1.
2. German: M2, L1.
3. Random leak. State `drawn` is null until the sitting is over (probe, seeds 2,
   10, 7: `state.drawn` null, `export().drawn` set). The brief header passes
   `drawn` and `reveal` only when `sittingOver` (`briefing.js`), the
   after-deal brief never; `review.js` reveals only for the whole sitting. The
   menu line, a restored sitting (`sittingFrom(..., state.drawn)`) and a language
   switch read the state's null. The one place `drawn` is in the clear before the
   end is the exported record (`export()` and the saved sitting), which STYLES 2.3
   accepts ("the game does not pretend otherwise"): it is a file download and
   local storage, never shown in the page. Not tested by the browser check
   (state, line, end dialog, review only); a check that restores a saved Random
   sitting and a language switch before the end would close that.
4. Prefs: version 2; a v1 record keeps its fields, gets `style: "normal"`, and a
   kept 1.5 or 2.5 snaps to 1 or 2 as `Level::from_f64` does (ties down);
   an unknown style (including Werner and Ulmann) falls to Normal; a v2 record is
   left alone by a v1 page ("newer"). Safe. The test covers the snaps and the
   style fallbacks; a v1 record with no `style` is covered by the defaults test.
   A restored saved sitting at 1.5 or 2.5 is snapped by the engine, so no
   `settings.skill.1.5` string is needed.
5. Tests: the unit tests (newsitting, prefs, settings, tutorwire,
   reviewstyle, strings) assert behaviour, not copies of the code. Gaps: H1's
   strings-against-pool test; `random_style_hidden` draws one seed (7); no test
   that the style line and the engine description agree (not testable
   mechanically).
6. `docs/TABLE3D.md` (3.5.1, 3.5.2, 7.1, 7.3, 7.7.3, section 10): accurate to
   the code apart from H1 (it says Werner and Ulmann "fell out", true of the
   dropdown, not of Random) and the length of the 7.3 paragraph (about 25 lines;
   the line on the browser checks in section 10 is right). Trim 7.3's list of
   labels, which `strings.json` already holds.
