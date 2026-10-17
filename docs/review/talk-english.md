# Review: the English talk is English

Branch reviewed: `worktree-agent-a688112ff208b7faf` (8ff907f, 20a4904, e4700c0, 1e0c8b6). Reviewer did not write it.

## Checks run

- `python3 -I tools/test_phrases.py`: 29 tests OK. `python3 tools/phrases.py --check`: rc 0 (PHRASES.md and words.json current).
- `node --test web3d/test/talk_lang.test.js`: 4 pass.
- `browser.py --only=talk_english`: ok.

## Findings

1. Translations (17 cells). Each keeps the sense of its German source; German lines and citations unchanged. Sources: Weiter, "Weiter, bitte" (05-table-talk, 05d: "Ich tappe" replaced by "Weiter"); Besser mitgenommen (05b, Wander Mitnehmen 1, gloss "Better taken along than missed out"); Glück ist besser (05b, Wander Glück 360); tous les trois (05c Nestroy, 05d Ulmann/Werner). No line touches looks, intelligence, age, gender, class, religion or nationality: the kindness rule holds. Low: "All three!" drops the French that Nestroy and the sources quote; acceptable, or keep "Tous les trois" as a cited foreign phrase via the allowlist.
2. "Luck is better than good play" (run-mine) and "Luck beats good play." (luck-yours) are the same saying (same German, same sig). Two groups, so `test_a_saying_is_in_at_most_two_groups` holds and the cross-group rule is met. Low: two English renderings of one saying; use one wording in both.
3. "Kibitzer, shut your trap: quite so." (kibitz-mute). The opponent says it when the player mutes the player's own aid; it is aimed at no trait of any person, so the kindness rule holds. But 05b line 152 records "Kiebitz, halt's Maul" as "no hits ... Not recorded": the German is uncited and its source tag `Ki` is unsupported. Medium (pre-existing, not introduced here): cite it or replace it with a sourced line. Softer English if kept: "Kibitzer, hold your tongue: quite so."
4. Allowlist. Game terms used in the English pages: tarock, touren, strohmandl, sküs, mond, pagat, troule, valat, ultimo, kreuzer, conto (strings.json, RULES.md); Mondfang is in RULES.md. Low: "matsch" appears only in the phrase bank (valat lines, e.g. "I am matsch: finished!") and is in neither strings.json nor RULES.md; the lexicon (05b, DWB) records it as the winner's shout, so "finished" mistakes the sense for the loser's side. Gloss or translate it, or add it to the English rules page. "tapper" appears in no English text; drop it from the allowlist.
5. Detector (`german_in`). I scanned every `en` text in words.json for umlauts, ß, a broad list of German function words, and -ung/-keit/-lich/-heit/-chen/-erl/-isch endings. The only hits were English words (will, was) and the allowed Sküs. No false negative found. The umlaut and ß rule catches most future leaks; the word list is the weaker part.
6. Runtime. Every bank text is static (no params), so resolving `WORDS.texts[key][lang]` at show time loses nothing; `line.text` remains the fallback. The re-lettering loop updates `data-text` and `textContent` of the existing boxes. The say layer has no `aria-live`, so a re-lettered box causes no second announcement. Low: a box is not re-placed after re-lettering, so a longer line can sit close to an edge until it is hidden; the kibitz note (`kibitzNote`, a timed `ui.kibitzNote`) is not re-lettered on a switch and finishes in the old language (at least 4 s).
7. `sr.pass.you` / `sr.pass.opp` ("You say weiter." / "Your opponent says weiter."). Recommend "You pass." / "Your opponent passes." The English button reads "Pass" and the gloss "Weiter = pass" is visible only on screen; a screen-reader user hears an unexplained German word. Keep the German strings. The detector does not cover strings.json; with the fix, extend EnglishIsEnglish to `en` values of `sr.*` keys.

## Verdict

Sound. Fix before done or record as open: finding 3 (uncited line), finding 4 (matsch gloss, drop tapper), finding 7 (sr.pass strings). The rest are low.

## Fixes

- 2 (one wording): `luck-yours` now reads "Luck is better than good play.", as `run-mine`; a test holds the `Glück 360` saying to one English wording.
- 3 (Kiebitz line): the line is cited (05c LIT-S11, Neues Wiener Journal, 29 Mar 1895: "Kiebitz, halt's Maul!"; 05b only lacks it among proverb lexica). It is dropped from `kibitz-mute` all the same, which keeps three wordings; the § 13 form stays in the tutor strings.
- 4 (matsch, tapper): the English no longer uses "matsch" ("Every last trick is mine!", "Not a trick for you. Swept clean.", "Every trick gone: I am done for!", "Not one trick for me. Done for!"); the German keeps it. `matsch` is out of the allowlist and a test keeps it out of the English. `tapper` stays: "No third for a Tapper?" (greet.4, Figaro 1888) is an English text that uses it, as the name of the game.
- 6 (Kibitzer note): the timed note is kept by key while up and re-lettered on a switch; a later note is not cleared by an earlier note's timer.
- 7 (sr.pass): `sr.pass.you` / `sr.pass.opp` read "You pass." / "Your opponent passes." in English (German unchanged); `EnglishIsEnglish` scans the `en` of every `sr.*` string.
- Kept: "All three!" (finding 1).
