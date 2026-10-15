# Tutorial pacing audit

Rule (TABLE3D 6.2.2): a page is due no later than the first decision it explains
is asked, and holds only what that decision needs. "Before" means the page is in
`pagesDue` for the state/events at the moment the decision is asked, and
`rested()` opens it before the bar and prompts are drawn (`web3d/test/tutor.test.js`,
"pacing"). Elements are in the order the game presents them (PROTOCOL events and
phases; RULES clauses in brackets).

## Audit

| # | Element | First asked / met | Page | Before (found) | After (now) |
|---|---|---|---|---|---|
| 1 | Deal, packets, covers [2.3, 4.4, 5.2] | `dealt` | `packets` | before | before |
| 2 | Bid: Ich nehme auf / Weiter [3.1-3.4, 9.1] | `phase: bid`, `prompt: you` | `bidding` | before, but needed the card values and 36 to judge a hand: they were on a later page | before; card values, 36 of 70 and the values moved onto it [1.7, 8.2] |
| 3 | The turning of the packets [4.3-4.7] | `reveal_start` | `the-pull` | after, nothing asked (cards turn themselves; "tap" only hurries) | unchanged; the page says "nothing to decide" |
| 4 | Playing a card: tap, Play, the winner leads, who wins a trick [5.1-5.3] | first human play turn | none (was folded into `from-a-packet` and `following`, shown later) | missing | new `playing`, due at the first play turn (lead or follow) |
| 5 | Following suit / trump obligation [5.2, 5.4] | first human turn with a card led | `following` | before | before; the trick-winning rule and lit/dimmed moved to `playing` |
| 6 | Playing a cover [5.2, 6.1] | first human turn with a cover legal | `from-a-packet` | after (due on the `card_played` event of a cover) | before (due when a cover is a legal play); the tap/Play line moved to `playing` |
| 7 | Announcing honours [11.1-11.5] | `announce` legal | `honours` | before | before |
| 8 | Pagat ultimo [12.1-12.4] | trick 23 with `T1` held | `pagat-ultimo` | before | before |
| 9 | The count, thirds, tally [8.1, 8.2] | tally runs from the first `trick_won`; `count` | `thirds` | after (due on `count`, the end of the deal) | before (first `trick_won`) |
| 10 | Scoring the deal: game, honours, Pagat paid; slate; settlement in kreuzer [9.2, 9.3, 15.5, 15.7] | `game_scored` | none (scattered; kreuzer on the `touren-slate` page, shown after two deals) | missing / late | new `scoring`, due at trick 21, never later than `count` |
| 11 | Draw, doubling [8.4, 14] | `game_scored` draw | `draw` | after, nothing asked (the banner says it, the next deal shows x2) | unchanged |
| 12 | Tours, the slate [15.2, 15.3] | `tour_end`, slate opened | `touren-slate` | after; nothing asked; Tour length is explained by the new-sitting dialog | trimmed to the Tour and its sum; payments and kreuzer are on `scoring` |
| 13 | One more Tour [15.6] | `phase: extend`, `prompt: you` | `extension` | after: the extend dialog opened first (`refresh` before `tutorStep`), and the page waited on `blocked` | before: `tutorStep` runs first; the prompt waits while a page is open; closing a page opens the next one due, then the prompt |

Not in the pages, by design: the Valat (RULES 10; "Valat" is on the tutorial
jargon list, `tutorial.test.js`), valid-card refusals (the dimmed-card
explanation), the review. Recorded as open for the maintainer.

## Pacing mechanics fixed

- `rested()` ran `refresh()` (bar, extension prompt) before `tutorStep()`, so a
  prompt could be asked with its page not yet open, and a page could not open
  over an already open prompt. Order reversed; the extension prompt is held
  while a page is open (`main.js`).
- Pages due at one rest were shown one per rest; `dismissPage` now opens the next
  due page at once, so the first turn's `playing`, `from-a-packet` and `honours`
  are read in a row, not between plays.

## Wording cut

Sentences implied by the one before were cut in both languages: "Then the
dealer" (bidding); "Your opponent has the same" folded into the first line;
"The leader plays one card. You answer with one" (now on `playing`); "Choose
either, if both are lit"; "You may stay silent. The choice is yours"; "If I may:
a cover played is a card turned"; "If the card beneath is the last of its
packet..." (the-pull says it); "All 70 are divided..."; "Missing it loses";
the second sentence of the Pagat page's pay line merged. No rule was dropped.
