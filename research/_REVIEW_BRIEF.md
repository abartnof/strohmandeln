# Review brief (for all review agents)

You are an adversarial reviewer of research notes written by other agents for
the Strohmandeln literature review. Read `research/_BRIEF.md` first: the notes
were written under its rules, and you hold them to those rules. Assume nothing
in a note is true until you have seen it in a source.

## What to check

1. **Quotes.** Every quoted fragment (German, English or other) must be found
   in the saved source text under `research/sources/`. Write a small script
   (in your own scratch directory, not in sources/) that extracts each
   quotation from the note and searches for it, allowing for OCR noise
   (normalise whitespace, ſ→s, hyphenated line breaks, umlaut variants such
   as ä/a¨/ae). Report the match rate. For each quote that does not match,
   look by eye; if the source was never saved, re-fetch it (save it under
   `research/sources/review/<your id>/`) and check it there.
2. **Citations.** Every claim needs a citation, and the cited source must
   actually say it. Sample at least 30 claims per note across all sections,
   and all claims in headline findings. Check page numbers, dates and
   attributions (who said it, in which edition, which year).
3. **Translations.** Check that each English translation of a non-English
   quote is faithful. Flag over-confident or embellished renderings.
4. **Second-hand claims.** Flag any claim that rests on a secondary source
   quoting a primary one (e.g. Wikipedia quoting Torberg), if the note
   presents it as if the primary had been read.
5. **Kindness rule.** Flag any proposed game line that breaks it.
6. **Contradictions** with other notes in `research/` (read their headline
   findings at least).

## What to do with what you find

- Fix clear errors **in place** in the note (a wrong page, a mistranslation,
  a misattributed quote, a quote that is not in its source: correct it or
  mark it `[UNVERIFIED — review]`). Never delete a finding silently.
- Log **every** change and every unresolved problem in your review file
  `research/review/<your id>.md`: a table of (note, location, problem,
  evidence, action taken), then the quote match statistics, then a short
  verdict per note (trustworthy / usable with care / needs rework).
- Do not touch notes other than the ones assigned to you. No git commits.
- Reviewers may follow promising leads they stumble on, but they must log
  them as new findings in the review file, with sources, not slip them into
  the notes.
- Final reply: ~200 words with the verdicts and the worst problems found.
