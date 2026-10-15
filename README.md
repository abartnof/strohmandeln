# Strohmandeln

Strohmandeln is the two-hand form of Tarock, the Austrian trick-taking game
played with a 54-card pack of 22 trumps and four suits. It takes its name
from the "straw man": a second, face-down hand that stands in for the missing
third player. It was played in Vienna's coffee-houses and homes from the
1820s to the 1930s (`strohmandeln-lit-review.md`, sections 1 and 2, which
cite every claim). This is a version you can play against the computer, in
English or German, and learn as you play.

**Play it** by opening `web3d/strohmandeln.html` in a browser: it is one
self-contained page that needs no installation, no server and no network, so
a copy saved to your own machine plays offline. Once published it is at
[abartnof.github.io/strohmandeln](https://abartnof.github.io/strohmandeln/).

## Learning the game

- **The game's own tutorial** opens each idea the first time it comes up in
  play. The question mark has all of its pages at any time.
- `docs/RULES.md`: the rules exactly as implemented, every one with its
  source. They are the consensus of the legitimate sources
  (`research/14-consensus-rules.md`); the only settings are the ones the
  sources never agreed on.
- `strohmandeln-lit-review.md`: what is known about the game, with citations.

## How it works

- **The rules engine** is Rust (`crates/strohmandeln-core`): the deal, the
  straw men, legal plays, counting in thirds, bonuses, the session and its
  slate, and the computer opponent.
- **The wasm module** (`crates/strohmandeln-wasm`) exposes the engine as a
  JSON protocol (`docs/PROTOCOL.md`); the page and the terminal client both
  use it.
- **The table** is drawn with three.js (`web3d/`, on the shared `kit/`): the
  54 faces are drawn at runtime, and the opponent speaks in lines from period
  sources (`docs/PHRASES.md`).
- **A terminal version** plays the same game:
  `cargo run -p strohmandeln-cli`.

## Building

You need the Rust toolchain in `rust-toolchain.toml` with the
`wasm32-unknown-unknown` target, Node.js, and Python 3.11 or later.

```
(cd web3d && npm ci)        # three.js and esbuild, pinned by package-lock.json
python3 kit/build.py        # compile the wasm module and build web3d/strohmandeln.html
```

`kit/build.py` reads `web3d/game.toml`; `kit/README.md` describes it.

## Testing

```
bin/gate                    # run bare, never piped: formatting, lints, Rust and Node tests,
                            # the wasm smoke test, the kit and phrase-bank tests, CREDITS.md,
                            # and that the built page is current
```

`bin/gate` needs `npm ci` in `web3d/` first. The full browser suite (the page
in a real browser, offline) lives in `web3d/test/` and is not part of the
gate.

## Layout

| Path | What |
|---|---|
| `crates/strohmandeln-core` | the rules engine and the opponent |
| `crates/strohmandeln-wasm` | the engine as a JSON protocol, compiled to WebAssembly |
| `crates/strohmandeln-cli` | the terminal client |
| `kit/` | shared table code vendored from Cassino (`kit/STAMP`), and `build.py` |
| `web3d/` | the game's table: sources, tests, phrase bank (`words.json`) and the built page |
| `docs/` | the documents below |
| `research/` | numbered research notes, review logs and the original-computation scripts |
| `tools/` | the phrase bank, the credits check, the contact sheet |
| `bin/gate` | the gate before every commit |
| `index.html` | the address GitHub Pages serves, on to the page |

## Documents

- `docs/RULES.md`: the rules as implemented.
- `docs/PROTOCOL.md`: the wasm module's wire format.
- `docs/MESSAGES.md`: the messages of that protocol and the events behind the talk.
- `docs/PHRASES.md`: everything said at the table, with its source.
- `docs/TABLE3D.md`: the table, phase by phase.
- `docs/OPPONENT.md`: the computer opponent and how each rung was measured.
- `docs/review/`: code review logs.
- `PLAN.md`: where the work stands.
- `CREDITS.md`: the libraries and the period sources.

`docs/TABLE3D.md` and `docs/OPPONENT.md` land with their work packages.

## Licence

MIT (`LICENSE`). Third-party software and sources are credited in
`CREDITS.md`.
