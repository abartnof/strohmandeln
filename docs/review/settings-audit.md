# Settings audit: what is a new-sitting choice, what is an aid (17 Oct 2026)

Source: the maintainer's request (17 Oct 2026): anything that does not apply to
this game belongs in the new-game settings; separate the settings from hints and
superficialities; table top as a dropdown. No code changed.

## 1. What the table offers today

Three places: the in-game menu (`web3d/src/settings.js`), the new-sitting dialog
(`web3d/src/newsitting.js`), and the page's other switches (keys, URL, Help).
Defaults and checks: `web3d/src/prefs.js:25-80`. Spec: `docs/TABLE3D.md` 3.5, 7.1-7.7.

Kind: R = rules setting (RULES "Authority and settings", `docs/RULES.md:26-40`),
S = sitting choice (RULES 15.2; the contest), A = per-viewer aid, V = per-viewer
appearance or page. "Applies" is the row's own tag.

| Row (settings.js line) | Changes | Applies | Kind |
|---|---|---|---|
| both_pass (:30) | neither takes the game: simple game or throw-in | next sitting | R |
| values (:31) | 2/3/4, 1/2/3, 1/2/4 | next sitting | R |
| ultimo (:32) | Pagat ultimo 1 or 2 Points | next sitting | R |
| touren (:33) | 3 or 6 Touren | next sitting | S |
| skill (:36) | opponent's level 1 to 4 in halves (7 buttons) | next sitting | S |
| speed (:41) | holds and flights, 4 steps | now | V |
| faces (:42) | Auto / Classic / Jumbo | now | V |
| leftHanded (:43) | mirrors the move bar and hand | now | V |
| reducedMotion (:44) | System / On / Off | now | V |
| surface (:46) | table top: "Random" plus the kit's 18 patterns, 19 radio buttons in a wrapping flex row (`table.css:165`) | now | V |
| talk (:53) | All / Calls only / None | now | V |
| tutor (:54) | the Kibitzer: pages, tips, review chip | now | A |
| tips (:55) | advice in the Kibitzer's tab, "Next" in the review | now | A |
| hints (:56) | hint chip, public mark, Cards out; applies at once (`main.js:1293-1296`) | now | A |
| tracker "Cards out" (:57) | the tracker; shown only with Hints on | now | A |
| explain (:58) | explanations of refusals and prompts | now | A |
| announceAuto (:59) | an honour is announced without asking | now | A (play convenience) |
| slateAuto (:60) | the slate opens by itself | now | V |
| lang (:63) | English / Deutsch, redraws all (`main.js:1318`) | now | V |
| actions (:68-74) | save / load a sitting file; export / import / clear the Kibitzer's record | at once | sitting file, not a setting |

New-sitting dialog (`newsitting.js:10`): `NEW_KEYS` = touren, both_pass, values,
ultimo, skill, plus a fixed "Hand: By lot" row (`:63-65`) and the note "Kept for
your next sittings; changeable under Settings" (`strings.json:36`). So **the
dialog already holds all five; the menu repeats them**. `main.js:1061-1070` saves
the choice as prefs on Deal; `main.js:1133` starts the sitting from prefs.

Other switches, none in the menu:
- Kibitzer mute: `prefs.muted` (`prefs.js:51`), the Kibitzer's tab and the key
  `m` (`main.js:1025`); un-muted by the tutor switch (`main.js:1285`). Per-viewer aid.
- Help: a dialog of the tutorial pages, `seen` kept (`prefs.js:52`, `main.js:361`). Not a setting.
- Keys list (`main.js:1469`). Not a setting.
- URL, `withUrl` (`prefs.js:143-153`): `lang`, `speed`, `talk`, `touren`. Overrides
  (`:157-165`): `seed`, `human`, `noworker`; `main.js:1136` `level`; `lowfx`
  (`main.js:106`), `keep`, TEST. All test or sharing aids; none is a rule.
  Note: there is no URL switch for the three rules (good: no hidden variants).

Finding: once a sitting exists, `settings.setSitting(true)` (`main.js:1143, 1196`)
greys all five rows (`settings.js:89`, `locked`). A sitting exists from page load
(Continue) or Deal, so the five rows are **never editable in play**; they are dead
controls that only say "from the next sitting". That is the maintainer's point.

## 2. Cassino's principle

- The game's own settings (rules: Royal or plain, aces 1/14, sweeps, raising
  builds; the skill dial; the match) are **chosen on one new-game screen**, opened
  from the plus, New game at the end, and the welcome (`~/cassino/docs/TABLE3D.md:545-565`).
  Motive, from play-testing: "it needs a new game to apply ... it's new game with x
  setting, all picked from one menu screen" (`:545-548`).
- The in-game Settings hold only aids and appearance: "the aids, the tutor's tips,
  Large Text, the table talk, the animation, the table top; the progress kept for
  the tutor" (`:566-569`). A browser check asserts the game's own settings are
  **absent** from the settings (`web3d/test/browser.py:394-409`).
- The Settings show **what the game under way is**, read-only: `gameSaid`
  (`prefs.js:220`), "This game: Royal Cassino, ... A new game, from the plus at the
  top, chooses the next one's" (`chrome.js:474`), with the seed once over (`:570`).
- The new-game menu starts from the game on the table (its rules and skill as the
  engine has them), else from the kept prefs (`menuChoices`, `prefs.js:208`;
  `TABLE3D.md:559-564`); what is chosen is kept as the settings (`chooseGame`,
  `prefs.js:187`).
- Skill: "can change between games, and from the next hand during one"
  (`DESIGN.md:374-378`). Strohmandeln departs here (section 3).
- Table top and faces and speed are `md-outlined-select` dropdowns
  (`chrome.js:262-269, 283-287`), grouped in a `.selects` row (`:335`); talk is a
  segmented set (`:272-283`); aids and page switches are `md-switch` rows.

Borrowed: one new-game screen, kept as defaults, the five-row dialog, the
prefs shape and versioning (`prefs.js:8`), the "applies" tag idea. Departed:
Strohmandeln kept the rules and skill rows in the menu, greyed (the pattern
Cassino's seventh play-testing removed); it has no read-only "this sitting is"
line; its table top is a 19-button row, not a select.

## 3. Proposed split

Principle: the in-game menu holds only what takes effect in this sitting now
(aids and appearance); the rules and the contest are the new-sitting dialog's.

| Setting | Goes to | Reason |
|---|---|---|
| both_pass, values, ultimo | New sitting only (remove from menu) | rules; fixed in the record's `sitting_start` (PROTOCOL 248); the slate prints them |
| touren | New sitting only (remove) | contest length; the slate and extension depend on it |
| skill | New sitting only (remove); show "Opponent: level n" read-only in the menu | carried in the record (PROTOCOL 81); the Kibitzer's brief, the assisted marks and the measured rungs assume one level a sitting |
| speed, faces, leftHanded, reducedMotion, lang | Menu, at once | appearance |
| surface | Menu, at once, as a dropdown (section 3.1) | appearance |
| talk | Menu, at once | per-viewer voice |
| slateAuto | Menu, at once | appearance |
| announceAuto | Menu, at once | play convenience, no effect on rules |
| tutor, tips, explain | Menu, at once ("Aids") | per-viewer aids |
| hints | Menu, at once | applies at once; an aid, never a rule. Also reachable at the new-sitting dialog? No: keep one home |
| tracker (Cards out) | Menu, at once (indented under Hints) | only meaningful with Hints on |
| Kibitzer mute | unchanged (tab and key) | not a setting; consider a menu switch only if asked |
| save / load sitting, Kibitzer export / import / clear | Menu "The sitting" section | actions, not settings |
| Hand: By lot | New sitting, fixed text | not a choice; keep or drop the row |
| URL `touren`, `level`, `human`, `seed` | stay; test-only | not offered |

Tests that need a DOM for the rules rows: none after the change; the model keeps
`ROWS` for the dialog (`newsitting.js:7`), so move the five rows into a
`NEW_ROWS` list in `newsitting.js` (or keep `settings.js` exporting them without
drawing them in the menu).

### 3.1 Table top as a dropdown (maintainer, 17 Oct 2026)

Now: a `choice` row with `surfaces: true` (`settings.js:46, 88`); the page passes
`PATTERNS.map(p => ({value: p.id, label: p.name}))` (`main.js:192`) from
`kit/js/surfaces.js:39` (18 patterns, English names only, no German) plus
"Random" (`strings.json:305`), rendered as 19 radio buttons (`settings.js:133-144`)
wrapping across the sheet. `chooseSurface` (`surfaces.js:65`) maps random to a
fresh pick at load. The choice applies at once (`main.js:1297`).

Proposal: a native `<select>` (`kind: "select"` in the model), label "Table top"
(`for`/`aria-labelledby`), "Random" first, then the 18 by name. Names need en and
de: add `settings.surface.<id>` keys (18 x 2 in `strings.json`), fall back to the
kit's `p.name`; the kit name is a label for tests, not the display.
Cassino offers it as `md-outlined-select` (`chrome.js:283`, English labels only);
Strohmandeln has no Material components, so the native element is the fit.
Points:
- Accessibility: a native select is keyboard-operable and read by screen readers
  with its label; the radio-group markup (`role=radiogroup`) goes for this row.
  The menu's Escape handler (`settings.js:178`) is on the sheet: a closed select's
  Escape would close the menu, an open list takes its own Escape first in
  browsers; check it.
- `open()` focuses the first checked radio (`settings.js:170`); the query keeps
  working because other rows stay radios.
- Style it with the menu's colour tokens (`colour.test.js` checks the menu's
  text/background pairs; add the select's pair, and set `color-scheme`), size
  the font 16 px or more (mobile zoom), min-height of a touch target.
- Layout: one row, label left, select right; the same pattern could take `faces`
  (3 options stay radios; no change asked).
- `set(key, value)` must update the select's value; the `surfaces` list can still
  arrive late (`setSurfaces`).

### 3.2 Edge cases

1. **A rule changed mid-sitting would change the slate's meaning**: values and
   ultimo price every credit and the slate prints them in its header
   (`slate.js:219`, from the record's `sitting_start`). Hence no mid-sitting
   change; removing the menu rows removes the greyed dead control.
2. **Opponent's level mid-sitting**: Cassino allows it between hands. Here do not:
   the level is in the record and export (PROTOCOL 81), the Kibitzer's brief and
   its `assisted` accounting (`main.js:250, 388`) and the opponent measurements
   are per level, so a sitting at two levels has no single label. Show the level
   read-only. If the maintainer wants a mid-sitting change, it needs a new
   `level_change` record event (protocol and engine change): not proposed.
3. **Hints in the menu**: yes. They apply at once and change nothing in the
   contest; the sitting records which decisions were assisted (`main.js:990`),
   so a mid-sitting toggle is already accounted for. Cards out follows Hints.
4. **A restored sitting keeps its rules**: it does, in the engine record, and the
   slate shows the record's. But the menu's greyed rows show the *prefs*, which
   may differ (a Continue after the defaults were changed; a loaded file).
   With the rows removed, replace them by one read-only line in the menu, as
   Cassino's `gameSaid`: "This sitting: 3 Touren, 2/3/4, Ultimo 1, simple game on
   a double pass, opponent level 3. A new sitting, from the plus, chooses the
   next." Source: the `sitting_start` event, not prefs.
5. **The new-sitting dialog's defaults**: Cassino starts from the sitting on the
   table, else from prefs (`menuChoices`). Strohmandeln starts from prefs, which
   Deal rewrites. Proposal: the same as Cassino (start from the finished or
   current sitting's rules, else prefs), so a person who loaded a 1/2/3 sitting
   is not offered 2/3/4 silently. Fresh install: canonical defaults (RULES: play,
   2/3/4, ultimo 1; 3 Touren; level 3, `prefs.js:25-32`).
6. **One screen, canonical default**: Length and Opponent's level on the screen;
   the three rules behind a "Rules" disclosure (`<details>`, closed), headed
   "Rules: as the consensus (change only to play a house custom)", summarising the
   current choice on the summary line ("2/3/4, simple game, Ultimo 1"). Cassino
   shows its rules on the screen (no disclosure); the disclosure is a proposal,
   since the three rules are rarely wanted and the rest is two choices. Deal
   stays the primary button and focus.
7. **The skill dial in the dialog**: 7 buttons for halves; level labels say
   nothing about strength (`strings.json:268-275`, OPPONENT.md promises a
   sentence). Optional follow-up: a one-line description under the dial.
8. **Persistence**: keep rules and skill in prefs as the next default (no
   version bump needed; keys unchanged). The menu no longer edits them, so
   `settings.set` for those keys is called only from Deal (`main.js:1068`).
9. **The `settings.mid-sitting` and `settings.applies.*` strings**
   (`strings.json:244-245, 312`) become unused; the "applies" column in `settings.js`
   goes (every remaining row is "now").
10. **The dialog's closing note** "changeable under Settings" is wrong after the
   change; it becomes "Kept for your next sittings." (en and de).

## 4. Doc clauses, tests, build

Docs that change:
- `docs/TABLE3D.md` 3.5.1 (add the Rules disclosure and the read-only level),
  3.5.2 (drop "shown, greyed"), 7.1 (remove the five rows and the "Applies"
  column and last paragraph; add the "This sitting" line, the table-top select),
  7.2 and 7.3 (move to 3.5), 7.7.4 (unchanged), 10.5 `settings_rules` (rewrite:
  absent from the menu, present in the dialog, the line reads the record).
- `docs/PROTOCOL.md`: no change (level and settings already in `new`, :72-82).
- `docs/TUTOR.md`: no change to settings; 6.3.5 / HINTS wording on Hints in the
  menu stays. Check `docs/HINTS.md` for any "Settings > Opponent" mention (none found).
- `PLAN.md`: orchestrator.

Tests that cover settings:
- `web3d/test/browser_aids.py`: `check_settings_rows` (:311-338: loop at :316-321
  over the locked rows goes; the surface count at :336 becomes
  `select option` count 19), `check_settings_next_sitting` (:341-358, keep: stored
  rules reach the sitting and the slate header; add the "This sitting" line),
  `check_skill_speed` (look at it; the dial moves to the dialog), the new-sitting
  dialog check (add: Rules disclosure closed by default, canonical defaults selected).
- `web3d/test/browser_suite.test.js:26` alias `settings_rules: "settings_rows"`
  (rename or keep).
- `web3d/test/settings.test.js:11, 38-40` (SPEC_ROWS, applies map) and its
  locked-row test (:98) go or change; add a select-kind model test; a negative
  test that no rules row is in the menu (Cassino's pattern, `browser.py:394-409`).
- A new `newsitting.test.js` case for the disclosure and the "from the sitting on
  the table" defaults.
- Strings: en+de for `settings.surface.<id>` x 18, the "This sitting" line,
  the disclosure; `strings_complete` covers them.

Build estimate (one work package, one coder agent plus a review agent):
- Menu without rules rows, "This sitting" line, select for the table top, strings: about 150 lines of source, 18 x 2 strings.
- Dialog: Rules disclosure, defaults from the sitting on the table: about 60 lines.
- Tests and doc edits: about 120 lines test, 4 doc clauses.
Roughly one agent session each for code and for review; no engine, protocol or
wasm change. Browser suite re-run required (full, about the suite's usual time).

Open for the maintainer: the Rules disclosure (item 6), the opponent's level fixed
for the sitting (item 2), and the default source of the dialog (item 5).
