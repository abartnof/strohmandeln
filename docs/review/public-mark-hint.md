# Review: the public mark as a hint

Branch `worktree-agent-a308d5540d06a56cc` (7 files), reviewed merged with main
(tutorial pages; merges cleanly). Spec: TABLE3D 1.5.7, 3.7, 6.3.5, 7.1; the
maintainer's decision of 17 Oct 2026.

Checks run on the merged result: `node --test kit/test web3d/test` 547/547 pass
(needs the root `node_modules` symlink to `web3d/node_modules`, as in the main
checkout); `python3 kit/build.py --check` clean; browser `tracker_fold_and_settings`
ok. No test, unit or browser, drives the dot or the note in the page (finding 7).

## Findings

1. **Medium. Muting the Kibitzer (key M, the mute button) or switching the tutor
   setting does not redraw the dots.** `hintsOn` depends on `prefs.muted` and
   `prefs.tutor`, but only `case "hints"` calls `director.redecorate()`. `tutor.toggleMute`
   goes `patch` -> `showTutor` -> `showTracker`, which redraws the tracker and
   nothing else, so the gold dots stay until the next redecorate (a card move, a
   focus change). Spec 1.5.7 says they "come and go at once". Fix: call
   `director.redecorate()` in `showTutor` (it already carries "the Kibitzer's mute is
   the slate's too") or in the `tutor` case and the tutor `patch`; unmuting should
   also call `publicNote()` (hints on, note unseen, never reached through
   `changed("hints")`).

2. **Medium. `hintsOn` duplicates the tracker's condition; the hint chip has a third
   form.** `trackerVisible` (tracker.js:22) is `hints && tracker && !(tutor && muted)`;
   `hintsOn` is the same minus `tracker`; the chip logic in main.js:866 and :981 tests
   `prefs.hints` and then the mute separately (to show the "Tips are off (§ 13)" line).
   Three copies of one rule (6.3.5 says "one condition"). Fix: export `hintsOn` from
   one place (tracker.js or tutor.js) and define `trackerVisible = hintsOn(p) && p.tracker`;
   main.js:981-982 can read `if (!hintsOnly...)` as `prefs.hints` then the mute test as
   now, since the chip needs to distinguish the muted case, but say so in a comment.
   Also `main.js:364` and :417 test `prefs.tutor && prefs.muted` inline.

3. **Medium. The note can fire before any dot exists.** `publicNote` runs at the first
   rest with hints on, which is the opening deal's start or the moment the setting is
   switched on, before any packet is turned: "a card both of you have seen: one pulled
   from a packet" then describes nothing on the table, and the note is spent (written to
   `seen`), so it never appears when the first gold dot does. Fix: also require at least
   one public hand card in the placement (`publicSlots(placement, prefs).length`), and keep
   the note due until then. Needs the placement at hand in `publicNote`, or a model query
   (`model.hand`/`model.opp` with `public`).

4. **Medium. The tooltip is wrong for announced-honour cards.** 3.7 and model.js:133-141
   mark cards of an announced honour public (R 13.5) on both hands, but `ui.public.tip`
   now reads "Seen by both: pulled from a packet" (de the same), and the note says "one
   pulled from a packet". Fix: "Seen by both" / "Von beiden gesehen" (and in the note,
   "for example one pulled from a packet" / "etwa eine aus einem Paket gezogene"), or
   say "pulled from a packet or announced" in both languages.

5. **Low. Docs overclaim.**
   - 1.5.7: "the first time hints are on (by default, or switched on)". Hints is Off by
     default (7.1, prefs.js:38), so only "switched on" is true. Drop "by default,".
   - 1.5.7 / 6.3.5: "the Kibitzer shows a short note", "in the Kibitzer's voice". The
     code opens a generic table dialog (id `note`, title "The gold dot"), not in the
     Kibitzer's tab, and it is shown with the tutor off too, where "I mark it" has no
     speaker. Either say "a dialog" in the docs, or gate it on `prefs.tutor` (and then
     fall back to the tooltip alone).
   - 3.7: "The legend is in the tutor's page on packets": neither `tutorial.en.md` nor
     `tutorial.md` mentions the gold dot (grep for gold/goldene/dot/Punkt finds nothing;
     pre-existing, now more visible, since with hints off the only explanation is the
     tooltip). Remove the clause or add one sentence to the packets page.
   - 6.3.5 elides the German with "..."; give it in full or leave German out of the
     spec (the string is in strings.json).
   - 1.5.7 parenthetical "(17 Oct 2026, the maintainer's decision.)" appears in 1.5.7 and
     6.3.5; once is enough (CLAUDE.md: a decision with its date and source).

6. **Low. Deferred note is retried only at a rest or a menu close.** Checked: one
   `MutationObserver`, created once in `main`, never disconnected, which is correct
   for a page-lifetime object (no leak); it fires on every `hidden` change of the menu
   but `publicNote` is idempotent, as `takePublicNote` marks seen before the dialog
   opens, so it cannot show twice. Hints turned on in the menu: menu open -> deferred,
   then the observer fires on close; if cards are still moving, `rested` -> `tutorStep` ->
   `publicNote` retries (tutorStep is called from line 353 and 797 on each rest), so it
   is not lost. Two gaps: (a) when the blocker is a dialog (the count, the slate, a tutor
   page), closing it triggers no `publicNote` unless a rest follows; at a sitting's end
   there may be none, so the note waits for the next sitting, harmless. (b) `tutorStep`
   is skipped at 353 for `pageHelp && !linked`. Acceptable; optionally call `publicNote`
   from the dialog-close path.

7. **Low. No in-page test.** `publicmark.test.js` covers the pure functions and persistence
   (good: mute/tutor-off cases, once-only, reload). Nothing drives `main.js`: dot absent
   with hints off, present with hints on, vanishes on toggle and on mute (finding 1),
   note appears once and not during a dialog/menu. `browser_aids.py` seeds `seen` to
   suppress the note in two pages, which hides the note from all other browser
   checks; other pages that turn hints on (grep `"hints": True` in browser*.py) are not
   seeded, so check for any that now see an unexpected dialog. Fix: add a
   `check_public_mark` (seed with a deal that has a pulled card; both hands).

8. **Low. Language switch with the note open.** `changed("lang")` re-renders the page
   dialog (line 1343) but not the `note` dialog; the text stays in the old language until
   closed. The settings menu must be open to switch, and the note defers while it is
   open, so it only occurs when the menu is opened over an open note (if that is
   possible). Cheap fix: close the note on a language change, or re-open it.

## Checked and fine

- Both hands: `publicSlots` filters `your-hand` and `their-hand`; the only other reader
  of `public` in `web3d/src` is the model/layout that set it. No sentence or
  accessibility text mentions a seen card (strings.json "unseen" entries are the
  tracker's and hint reasons', which already sit under hints).
- Restored sitting and language switch: dots are drawn from `prefs` at each decorate
  (main.js:1411, 1423 redecorate), and the tooltip is looked up with `t()` each time.
- German: Sie-form, natural ("Hinweise" matches the setting's label; "merke ich sie für
  Sie" fine).
- Merge with main: no conflicts; `web3d/strohmandeln.html` is the built bundle and
  rebuilds clean.
