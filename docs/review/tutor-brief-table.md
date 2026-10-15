# Review: the tutor's brief, table, words and storage

Scope: `web3d/src/{tutorstore,tutorwire,briefing,review,tutorview,settings,prefs,main}.js`
(the brief dialog, Tips setting, export/load/clear, nudge, assisted list),
`web3d/strings.json` (the `tutor.*` keys and the progress/tips keys),
`docs/MESSAGES.md` "Tutor keys", `tools/test_tutor_strings.py`, `docs/TABLE3D.md`
6.4, the `deal_brief` browser check. Branch `worktree-agent-abd5f18ed8764998e`,
against `docs/TUTOR.md` 6, 8, 9, 14, TABLE3D 6.4-6.5, `research/16-skill-ladder.md`.
The Rust engine and wasm are reviewed separately.

Run: `node --test` on tutorstore, briefing, review, strings, tutorview, settings
(79 pass); `python3 -m unittest tools.test_tutor_strings` (4 pass);
`bin/browser-full deal_brief` (ok, 63 s). Word limits checked in both languages
over all `tutor.*` strings: none over (rule/fact 15, try 12, nudge 10, well 12,
example 18).

Screenshots (desktop 1280x720 light and dark, German, 1024x768, 390x844):
`/tmp/claude-1000/-home-andrewbartnof-strohmandeln/02d38afc-3916-4bc6-862c-8b8c552d5a27/scratchpad/shots/`
`dlg_en_1280_light.png`, `dlg_en_1280_dark.png`, `dlg_de_1280.png`, `en_1024.png`,
`en_390.png`. The dialog fits all (318-361 px high, 488 px wide; at 390 it is
full-width), the focus lands on Close, Escape closes it, it has `role=dialog`,
`aria-modal` and `aria-label` = its title, and it uses only the theme tokens
(dark: on-surface on surface-container-high, no custom colours).

## What was checked and is sound

- Storage: versioned key `strohmandeln.tutor`; every read, write and removal in
  try/catch; a newer store is read as empty and `save` and `importText` refuse
  to write over it; caps (60 deals on load and on save, 20 KB a record, 2 KB a
  summary, 200 assisted numbers below 1000, 1 MB import); a summary that does
  not read as the engine's text is dropped to null and recomputed from its
  record; a bad record is dropped on load, a bad file refused whole on import
  with nothing changed; a summary of a newer evidence is not recomputed back; a
  record the engine cannot restore is dropped after one try; tests use a memory
  store unless `?keep`.
- Assisted marking: the hint's `deal_action` and the nudge's are the engine's
  numbers, kept both seats' counted; the `deal_brief` check shows the hint's
  number equals `state.deal_actions`, reaches the kept deal, and is cleared for
  the next deal. Cleared on `thrown_in`; a `void_deal` happens before any
  action, so nothing leaks. The list is in the saved sitting's extras and is
  read back on restore.
- Mute and Tips: muted, no chip, no dialog, no nudge, no hint (the engine's
  evidence still accrues, which is right). Tips off: no focus kept, no nudge
  asked, the brief keeps the count and one "Went well" bullet.
- Spot-check of five rules against `research/16` and its sources:
  `rule.worth.poor` (MS "den König schinden und ein Skartindel zugeben"): faithful.
  `rule.cheap` (MS, U 75): faithful, and gated. `rule.announce` (W 133, MS
  "Die Prämien": "spätestens im Augenblick der Verwendung"): faithful.
  `rule.pagat.cash` (U 74 "sieben bis acht", MS "7 oder weniger"): faithful to
  MS, U allows eight. `rule.void` (Be 150, U 75-76): stronger than the sources,
  finding 8.
- Terminology: "Point" only for payments (`fact.pagat`, `miss.announce`);
  "points"/"Augen" for the count everywhere; "Partie" for the deal and "Spiel"
  for the declared game, consistent with the scoring pages.
- The brief's size: 12 + 15 + 18 = 45 words is the sum of the body limits, which
  is where "about 45" comes from. With the leads, the skill name and "If I may:"
  the sample is 53 words and the worst case about 58. See nit 19.

## Findings

Severity: bug, should, nit. Files are in the worktree.

### Bugs

1. **bug** `web3d/strings.json:313` (`error.progress-bad`, de): "Diese Datei sind
   nicht die Aufzeichnungen ..." is ungrammatical (singular subject, plural verb).
   Fix: "Diese Datei enthält nicht die Aufzeichnungen des Kiebitzes dieses Spiels.
   Nichts wurde geändert."

2. **bug** `web3d/strings.json:524` and `docs/MESSAGES.md` (`tutor.rule.off.figure`,
   de): "Eine Dame oder ein Cavall auf einem Paket gleich wegspielen." An
   infinitive instruction takes the accusative: "einen Cavall". Fix: "Eine
   offene Dame oder einen Cavall auf einem Paket gleich wegspielen." or, as the
   spec had it, the statement "Eine offene Dame oder ein Cavall geht gleich."
   Same family, `tutor.miss.worth.poor` (de): "{better} hätte ihn für einen
   größeren aufgespart": "ihn" reads as the trick, the thing saved is the played
   card (gender varies); recast: "... holte {played} nur {thirds} Augen; mit
   {better} wäre der Stich billiger gewesen und die Karte für einen größeren
   Stich geblieben." Also `tutor.miss.off.figure` (de) has lost "Ihre" and
   "seinen" ("{card} fiel ... unter {king}"): say whose.

3. **bug** `web3d/src/briefing.js:19-21`, `web3d/src/main.js:1183`: "Clear what the
   Kibitzer keeps" empties the store but not `briefing`'s in-page `memory`, and
   `kept()` falls back to it when the store is empty. Until the next scored deal
   writes one entry, `settleFocus()` (on opening the chip, on a settings change)
   rebuilds the focus from the cleared deals, and the nudge follows it. Fix:
   `briefing.forget()` (empty `memory` and `cache`), called by the clear handler,
   plus a test in `briefing.test.js`.

### Should

4. **should** words, `tutor.well.met`, `tutor.well.mastered(-two)`,
   `tutor.miss.*` (en), `tutor.lead.deal-n`: the stacked colons and capitals the
   author flagged are real. Rendered: "Went well: You met all 3 chances: the
   contested suit.", "Went well: Mastered: judging a trick's worth.", "This deal:
   Trick 21: playing ...", and in the sitting's brief "Deal 3: Trick 21: ...".
   German avoids two of them ("Im 21. Stich hätte ...") but has "Gut gelaufen: Sie
   nutzten alle 2 Gelegenheiten: der Wert des Stichs." Fix (en): examples start
   "In trick {trick}, ..." (words +1, worth.poor stays at 18); `well.met`
   "{skill}: all {chances} chances met." (de "{skill}: alle {chances}
   Gelegenheiten genutzt."); `well.mastered` "{skill}: mastered." as the author's
   first draft in TUTOR 9 had it, so the skill name leads and the line carries one
   colon. The capital after "If I may:" is acceptable (a full sentence), but
   `tutorWords` could lower the first letter in English (no rule starts with a
   proper noun).

5. **should** `tutor.miss.worth.rich` and card names without articles. The author's
   sample, "Trick 23: jack of hearts led, with 5⅔ points up for grabs; king of
   hearts would have taken them", does not say who led (the opponent: the human
   was the follower), and "led" reads as a noun. Same sentences lack "the":
   "playing jack of clubs from its packet", "king of hearts would have taken".
   Fix: "In trick {trick}, your opponent led the {led}: {thirds} points. The
   {better} would have taken them." and "the" before every `{better}`, `{played}`
   and `{led}` in the English examples. German: "Im {trick}. Stich lagen mit
   {led} {thirds} Augen aus" is not idiomatic; "Im {trick}. Stich spielte Ihr
   Gegner {led} an, es ging um {thirds} Augen; {better} hätte sie geholt." German
   card names stay bare (gender varies by rank), as the older review keys do.
   "up for grabs" is casual for a dry kibitzer.

6. **should** `strings.json` `tutor.name.behind`, `tutor.name.void`: the skill names
   are used as labels in "Next: {skill}." and "You met all N chances: {skill}".
   "the contested suit" is not understandable cold (a suit the two hands share
   evenly; the first fact explains it only when it is the focus), and "met 3
   chances at the contested suit" says nothing about what was done (three times
   you did not open it). Both are restraint skills. Fix: name by the act: behind
   "leaving a shared suit unopened" / "eine geteilte Farbe nicht eröffnen";
   void "not leading into his empty suits"; and fold "your opponent's" out of the
   name ("Next: your opponent's empty suits." is not a thing to do).

7. **should** the tries are not standalone under "To try:". `try.off.2` "Exception:
   your opponent shows two low cards of the same suit." (an exception to what?),
   `try.behind.2` "Spare low cards in the suit can change this." ("this"),
   `try.announce.2` "With two of three, wait; ..." (two of three what?),
   `try.run.2` "bring your opponent in" is obscure ("make your opponent lead";
   de "ans Spiel bringen" is fine), `try.pagat.2` is a fact, not a try. Fix by
   naming the subject: "Keep a queen on a packet only if your opponent shows two
   low cards in that suit."; "Spare low cards in a suit can turn that around: count
   them."; "With two Trull cards or three kings, wait; the rest may come from a
   packet."

8. **should** faithfulness. `rule.void` ("Lead no suit your opponent could not
   follow; it can be trumped."): Be 150 says it is "wohl zu überlegen" whether to
   lead the single card, U 75-76 says not to come out with the king; neither
   forbids the suit, and "it can be trumped" is the page's inference (the
   opponent trumps only with a tarock, RULES 5.4, so it is false against a hand
   with none). Soften to the source: "Think twice before leading a suit your
   opponent could not follow; a king there can be trumped." (de likewise). It is
   gated (M5) but the method text says "the rules are the advice of the books".
   `rule.bid` "20 to 25 points ... and 6 or 7 tarocks" reads as a ceiling;
   MS says 25 or more is very good, "mindestens 6 bis 7": write "at least". 
   `rule.pagat.cash`: "cash" is not clear to a new player; U's own word is
   "einstechen"; en "use the Pagat to win a trick by trumping at the first chance".
   `rule.pagat.keep` "worth keeping" is firmer than MS ("only in the rarest cases
   with 12+; you need a long suit too").

9. **should** terminology: the table calls the open card on a packet a "cover" /
   "Decke" (`card.source.packet`, the tutorial); the tutor says "open packet
   cards" / "Paketkarten" (`tutor.rule.off.pip`, `try.off.1`, `tutor.fact.off`).
   Use "cover" / "Decke" so the person can find the thing on the table.

10. **should** `web3d/src/main.js:421-425, 1463`: `refreshStale` is documented
    (TABLE3D 6.4.3, comment) as running "while the table is idle", but it starts 3 s
    after load and steps every 250 ms without asking `director.busy()`, `pending`
    or the prompt. After an evidence bump it is up to 60 main-thread tasks of
    60-120 ms during play. Gate each step on a rested table (re-arm on `rested()`).
    It is also not started when the Kibitzer is turned on mid-page (case "tutor",
    `main.js:1216`), though deals kept while it was off have no summary.

11. **should** `web3d/src/main.js:947, 1088`: a hint's assisted number is saved only
    with the next `persist` (the next action). Hint, reload, and the decision it
    was shown for counts as unassisted. Likewise `nudgedDeal`/`nudgeKey` are not
    saved, so a reload shows a second nudge in the deal. Fix: `persist({state: wasm})`
    after the push, and keep `nudgedDeal` in the extras.

12. **should** `settings.tips.note` (en/de), `strings.json:310`: "Off: no work to do and
    no nudge; the review keeps its first line." uses the spec's internal words
    (work, nudge, first line). The behaviour is the count and one "Went well"
    line. Fix: "Off: no advice in the Kibitzer's tab and no 'Next' in the
    review; the review shows the count and what went well." (de: "Aus: keine
    Hinweise im Reiter des Kiebitzes und kein 'Als Nächstes' im Rückblick; der
    Rückblick nennt die Zählung und was gut lief.") The same words are in TABLE3D
    7.1 and 6.4.6.

13. **should** `tutor.method` (en/de): "the next on the ladder" uses a word the
    person was never given; and the claim that "each rule was measured" (the spec
    text) is replaced by "the rules are the advice of the standard Tarock books",
    which finding 8 shows is too strong for one rule. Say "the first of ten skills
    (listed in the order the books teach them) at which you are surely short".

### Nits

14. **nit** the review chip now stays until the next deal is scored (it was cleared
    at `deal_start`). Judgement: acceptable and useful (the brief can be read after
    the next deal has begun, and the nudge refers to it); the dialog is the engine's
    last-scored-deal review and `scored` is the same deal, so they agree (checked
    in `wasm/lib.rs` `review_last`). Two small things: the dialog title does not
    name the deal ("How the deal went" over deal 2's table; use "How deal {n}
    went" from `entry.n`), and the chip does not survive a reload (`reviewChip` and
    `scored` are not restored), so "stays until" holds only within a page.

15. **nit** `main.js:947` `r.deal_action ?? r.action`: `r.action` does not exist in a
    hint reply (PROTOCOL "Hints"); drop the fallback. `main.js:416` pushes the
    nudge's number without the `includes` check the hint uses (a hint then a nudge
    at one decision lists it twice; `recordDeal` dedupes, `persist` saves both).

16. **nit** `tutor.progress.loaded` "Loaded 1 deals." / "1 Partien geladen." Use a
    `.one`/`.other` pair (`tn`).

17. **nit** `tutor.miss.off.figure` (en) says "his {king}" while every other tutor
    line says "your opponent"; the phrase-bank test bans "he", not "his". "your
    opponent's {king}" (+1 word, 15). `tutor.rule.announce` (de) lacks "die" before
    "Troule" and before "vier Könige".

18. **nit** storage: `recordOk` accepts any string up to 20 KB. A record with a line
    `==`, `--` or `++` (the wire's separators, `tutorwire.js`) breaks the framing of
    `brief_sitting`/`brief_of_record` (the engine answers with an error, nothing
    worse). Require a one-line JSON object with `v === 1`. `main.js:1176` turns an
    oversize file into a 1,000,001-character string to reach `progress-big`; pass the
    size to `importText` instead. "Load what the Kibitzer keeps" replaces what is
    kept and "Clear" asks for no confirmation and says nothing after; add "(replaces
    what is kept)" or a chip.

19. **nit** docs. TABLE3D 6.4.6 says "the module is told no focus"; the code sets no
    focus and asks for no nudge (`briefing.setFocus` is not called with ""): say
    that. The `deal_brief` text (TABLE3D 10.4) claims "none after a void deal or a
    throw-in"; the check does not cover it (and `shot_name` in
    `browser_aids.py` is assigned and unused: no screenshot is taken). TABLE3D 1181
    and `prefs.js:166` still list `?noworker` although no Worker is built. TUTOR 12
    ("at most 45 words") and MESSAGES "about 45 words" are the body limits; with
    the leads the sample is 53 and the worst case about 58: say "45 words of body,
    leads and the skill's name excluded". `error.progress-*`, `settings.tips*` and
    `tutor.progress.loaded` are in no table of MESSAGES.md and `test_tutor_strings`
    does not test them for kindness.

20. **nit** voice (TABLE3D 6.5, "if I may"): the rules carry the prefix, the nudges do
    not and are imperative ("Mind the Pagat here.", "Achtung auf den Pagat."). The
    nudge is the one line shown unasked in his tab; "Which card pays matters
    here." is a garden-path ("Think which card is cheapest here."). Consider "If I
    may: mind the Pagat here." (10 words allowed). `tutor.short` (de) "ließ Ihnen kaum
    Wahl, die sich besprechen ließe" is stiff: "bot Ihnen kaum Entscheidungen, die
    sich zu besprechen lohnen."

21. **nit** narrow: at 390 px the dialog is full-bleed (x 0, width 390), unlike the
    92vw cap in `style.css:137`; phone layout is deferred, nothing breaks (no
    horizontal scroll, text wraps, Close visible).

22. **nit** `clearTutor` removes a newer page's store too (an explicit action; the
    "never written over" rule is for automatic writes). Worth one line in
    TUTOR 8 so it is a decision.

## Verdict

Sound design and code; storage and assisted marking meet the spec, tests pass and
the browser check is green. Fix the three bugs (1-3) and the words in 4-9 before it
is called done; 10-13 are small and can follow in the same package; 14-22 are
record-only. Words, as a new player reads them: the structure (count, then at most
three labelled bullets, the method folded) is clear and kind, no line judges the
person; the stumbling blocks are the stacked colons, the skill names used as
labels, tries that do not stand alone, and the German grammar slips above.

## Second pass (2026-10-15)

Scope: `git diff 5395639 6eee7c4` for `web3d/src/{briefing,review,main,tutorstore,
tutorwire,ui}.js`, `web3d/strings.json`, `docs/{MESSAGES,TABLE3D,TUTOR}.md`,
`tools/test_tutor_strings.py`, `web3d/test/*`. Branch
`worktree-agent-abd5f18ed8764998e`.

Run: `node --test` on tutorstore, briefing, review, strings, tutorview, settings,
persist, prefs (108 pass; the briefing tests need the wasm built);
`python3 -m unittest tools.test_tutor_strings` (5 pass); `bin/browser-full
deal_brief` ok (63 s; run with a short `TMPDIR`, Chromium's socket path is limited).
Beyond the suite, a Node script played a sitting through the real module and the
page's `briefing.js`, rendered the briefs in both languages, then damaged the store
(finding 1).

### First-pass findings, checked

Fixed: 1 (the German file error); 2 (`rule.off.figure` is a passive sentence;
`worth.poor` and `off.figure` recast, no card gender guessed); 3 (`briefing.forget()`
is called by Clear and by a load, and the browser check confirms the focus does not
come back); 4 and 5 (no line stacks a second colon, examples say who led and put
"the" before a card); 7 (tries stand alone, at most 14 words); 9 ("cover"/"Decke");
10 (`refreshStale` waits for a rested table with no dialog, re-arms at each rest, and
starts when the Kibitzer is turned on and after a load); 11 (a hint's and a nudge's
number are persisted at once and `nudged` is in the extras; `persist` runs after
`keepScored` has reset them, so a reload at a deal boundary restores nothing stale);
12 and 13 (plain words, "the earliest, in the order the books teach them"); 16, 17,
20. Clear asks first and says it is done. The working chip runs from `keepScored` to
the end of `settleOwed`. Finding 6 and 8 are addressed below (8 partly: finding 4).

The lowercase after a colon (`review.js` `after`) was checked against every English
body that can follow a lead ending in ":" or "If I may:" (`well.*`, `miss.*`, `try.*`,
`rule.*`, `nudge.*`): each begins "A", "Do", "Win", "In", "Your", "You", a verb or a
`{skill}` (all English skill names begin in lower case), so no card name, "I", Roman
numeral or proper noun is lowered, and names inside a line ("the Pagat", "your Sküs",
"the Mond") are untouched. German is not touched. It is safe by the strings' shape
only (finding 9).

What the brief shows today, every rule told as its fact, from a played sitting:
"Next: judging a trick's worth. A trick is worth the points of its two cards."; "This
deal: in trick 14, your opponent led the tarock XVIII; the Mond would have taken its
4⅔ points."; with no example, "To try: a king can wait for a queen or a cavall." That
is useful to a learner: the fact names the idea, the example names the trick and the
cards of their own deal, the try gives a thing to do. The `worth` fact alone is nearly
a definition and gives no direction; the direction is in the example. It is honest,
kind and as TUTOR 6 asked; nothing judges the person.

Five words spot-checked against `research/16-skill-ladder.md` and its sources:
`rule.worth.poor` (MS "den König schinden") faithful; `rule.off.figure` (U 74, MS)
faithful, the exception now a standalone try; `rule.void` ("think twice", Be 150)
faithful; `rule.announce` (W 133) and `try.announce.2` (W 135) faithful; `name.behind`
with its fact and miss are not (finding 4).

### New findings

1. **should** `web3d/src/briefing.js:84`, `web3d/src/tutorstore.js:70,85`,
   `crates/strohmandeln-wasm/src/tutor.rs` `parse_kept`. One bad stored deal makes
   the whole sitting's brief `null`, and nothing heals it: the engine answers
   `bad-input` for the call, `sittingBrief` returns `r?.brief ?? null`, and the
   sitting's review shows the count and no bullets. Reproduced four causes on a played
   sitting: a stored `n` of 0 (`sound()` keeps any integer `>= 0`, and `d.n ?? i + 1`
   keeps the 0, which the engine refuses as "deal number"); an `n` of 2^33; a detail
   with a damaged miss (`detailOk` checks only the summary and the size); a detail
   whose summary is of an older evidence version (until `refreshStale` reaches it; the
   engine requires a current summary), which is routine after any `EVIDENCE_VERSION`
   bump. Fix: `n: d.n > 0 && d.n < 10000 ? d.n : i + 1` and the same bound in
   `sound()`; in `sittingBrief` treat a detail whose summary version is not the
   learner's as missing (`summaryVersion` exists), and on `bad-input` recompute the
   details once with `evidenceOf` and retry; one test per cause in `briefing.test.js`.
2. **should** `web3d/src/main.js:1054-1063`, `web3d/src/tutorstore.js:150`. Clear on
   a newer page's store now does nothing, but the chip still says "What the Kibitzer
   kept is cleared." `clearTutor` should return whether it cleared and the handler
   show `error.progress-newer` otherwise. The handler also sets `owed = []` without
   `showTutor()`, so a chip that was "Working..." stays disabled until the next
   `showTutor` call; add it.
3. **should** `web3d/src/main.js:395-403`. `settling` is cleared after the loop, not in
   a `finally`: if `briefOf` throws (a wasm trap, a malformed reply), the chip is
   disabled with "Working..." for the life of the page, and `refreshStale` never runs
   again (`settling` is in its guard). Wrap the loop in try/finally.
4. **should** words, `tutor.name.behind`, `tutor.fact.behind`, `tutor.miss.behind`
   (en/de). `research/16` and the engine judge `behind` at the opening of a suit whose
   points the two hands share about evenly (30 to 70 percent; a suit mostly his or
   mostly mine is "not split", test `behind_a_suit_that_is_mostly_mine_or_his_is_not_split`;
   W 134 "gleichmäßig vertheilt", MS "umkämpfte Farbe"). "Not opening your weak suits"
   names another thing, "where your opponent holds more points" is not the trigger,
   and the commit message's "what the measurement shows" has no measurement behind it
   in TUTOR 11. Fix, by the act as the first pass suggested: name "leaving a shared
   suit for your opponent to open" (de "Eine geteilte Farbe den Gegner eröffnen
   lassen"); fact "A suit whose points the two hands share about evenly is better
   opened by your opponent."; miss "In trick {trick}, you opened {suit}, whose points
   the two hands shared about evenly; the {better} led better." (de likewise). Mind the
   word limits (name 8, fact 15, example 18).
5. **nit** words. `tutor.miss.cheap` (en): "the {better} would have done the
   {played}'s job" reads badly with real names ("the 4 of hearts's job", "the Mond's
   job"); "would have done what the {played} did, for less" is plain.
   `tutor.miss.worth.poor` (en): "would have kept it back" leaves "it" open (the
   played card); "the {better} would have saved your {played} for a bigger trick" says
   it. `tutor.rule.bid` (en/de): "at least 20-25 points" reads as a floor with a range;
   the sources say about 25 is very good and 20 is thin (MS), 20 to 25 unhesitatingly
   (Be): "Take up with about 25 points of sure winners and at least 6 or 7 tarocks."
6. **nit** `web3d/src/tutorstore.js:195`. `refreshOne` drops a kept deal whenever
   `evidenceOf` returns no summary and the engine has not stopped. A reply of
   `bad-constants` (impossible in a tested build) would drop every kept deal in turn.
   Drop only on `error === "replay"` (the record cannot be restored).
7. **nit** `web3d/src/main.js:441-451`. `step` clears `staleTimer` before it awaits
   `refreshOne`; a rest during the await would start a second chain. Today the engine
   is synchronous and the promise resolves before the next task; when the Worker is
   built, guard with an in-flight flag.
8. **nit** A load of a file replaces what is kept, the current sitting's deals
   included; the sitting's `sit` ids then match nothing and its review shows the count
   only. The label says "(replaces it)"; say so in TABLE3D 6.4.4, or keep the current
   sitting's deals through a load.
9. **nit** `tools/test_tutor_strings.py`. The English lowering is safe today by the
   strings' shape only. Add a check that no English body used after a lead or "If I
   may:" begins with "I ", a card name, a Roman numeral or Pagat/Mond/Sküs/Trull, so
   a later string cannot be mangled. German keeps the capital after "Als Nächstes:"
   before an infinitive phrase ("Den Wert eines Stichs abschätzen"); acceptable.
10. **nit** No automated test restores a persisted `assisted`/`nudged` (`persist`
    returns early under `TEST` unless `?keep`). `deal_brief` covers the hint's number
    reaching the kept deal, not a reload. A `?keep` reload step would close it.
11. **nit** `docs/TUTOR.md:349` says tries are at most 12 (code and MESSAGES: 14).
    The 62-word total is the longest brief with the leads and full names (a test
    holds it), and "about 45 words of body" is the sum of the limits (12 + 15 + 18),
    so those sentences are true.

### Verdict

The first-pass bugs and should items are fixed, the tests and the browser check pass,
and the brief is useful, kind and in the sources' voice, with one exception: `behind`
is named and explained for something other than what is judged (finding 4). Findings
1 to 3 are small, real failure paths in the new storage and chip code and should be
fixed before the work package is called done; 5 to 11 can be recorded as open.
