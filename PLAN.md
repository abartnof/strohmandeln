# Plan

Strohmandeln as one offline web page: a Rust engine compiled to WebAssembly, a
three.js table, a computer opponent with period table talk, a tutor, and a
terminal client. Built on the parts of Cassino and Piquet that carry over.

## Decided

- 14 Oct 2026: the modern game (15 hand cards plus three 4-card straw men
  each). The 1829 dummy-pile game is not built.
- 14 Oct 2026: canonical rules by consensus of the sources, with settings
  only for disagreements that persisted through the game's history
  (`research/14-consensus-rules.md`): both pass, game values, Pagat ultimo
  value.
- 14 Oct 2026: the session is the period "Abend": 3 or 6 Touren (2 deals
  each) announced at the start, a running account on a slate, settled at
  2 kreuzer a Point (`research/14-consensus-rules.md`, "The session in detail").
- 14 Oct 2026: cards are our own large-type faces: suit cards with rank
  labels K Q C J (the jack is "J"), tarocks with Roman numerals (IIII as on
  real packs) and nicknames on I, II, III, IIII, XXI and the Sküs; a plain back.
- 14 Oct 2026: shared code is a vendored `kit/` copied from Cassino with a
  `kit/STAMP` file naming the source commit, not a submodule
  (`research/review/RC.md`). It becomes a package once a second game uses it.

## Work packages

| # | Package | Depends on | State |
|---|---|---|---|
| 1 | `docs/RULES.md`: the engine's rules, every one cited; the session; the events the table and talk follow | — | todo |
| 2 | `kit/`: the shared JS (render core, motion, dialogue, tutorial) and `build.py`, from Cassino; `bin/gate` | — | todo |
| 3 | Phrase bank: `tools/phrases.py` → `web3d/words.json` + `docs/PHRASES.md`, with a kindness test | 1 (moments) | todo |
| 4 | `strohmandeln-core`: cards, deal, straw men, legal plays, tricks, counting in thirds, bonuses, settings, session and slate, records | 1 | todo |
| 5 | `strohmandeln-wasm` and `strohmandeln-cli`, with `docs/PROTOCOL.md` | 4 | todo |
| 6 | Computer opponent: a ladder of rungs, each measured against the one below | 4 | todo |
| 7 | Card faces: 54 large-type faces and a back, drawn at runtime | 2 | todo |
| 8 | The table: layout, choreography, play, slate, settings, phone | 2, 5, 7 | todo |
| 9 | Tutor: tutorial pages, the kibitzer's hints and explanations, end-of-deal review | 5, 8 | todo |
| 10 | Build, browser test, README, CREDITS, LICENSE, Pages | all | todo |

## Open

- The currency on the slate (gulden and kreuzer, or krone). Note 13's dates for the krone are unsourced.
