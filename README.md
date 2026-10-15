# Strohmandeln

Strohmandeln is the two-hand form of Tarock, the Austrian trick-taking game
played with a 54-card pack of 22 trumps and four suits. It takes its name
from the *Strohmänner* ("straw men"): the three face-down packets of four cards
that each player has in front of him (`strohmandeln-lit-review.md`, section
1.1). The name is in print from 1824, and in the press until 1938 it was the
coffee-house game for two (sections 2.2 and 2.3; the review cites every
claim). This is a version you can play against the computer, in
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
  slate.
- **The computer opponent** (`crates/strohmandeln-ai`, `docs/OPPONENT.md`) is a
  ladder of four rungs, plus the tutor's hint, and the tournament that measures
  them.
- **The wasm module** (`crates/strohmandeln-wasm`) exposes the engine and the
  opponent as a JSON protocol (`docs/PROTOCOL.md`); the page and the terminal client both
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

`bin/gate` needs `npm ci` in `web3d/` first. It also runs the browser test
in its quick form when Playwright and Chromium are found, and says so when it
skips it (`GATE_NO_BROWSER=1` leaves it out).

**The browser test** (`web3d/test/browser.py`, `docs/TABLE3D.md` section 10)
opens the built page from a `file://` URL in a real browser with the network
off, plays it by clicking, and fails on any console error or network request.
It needs Python with Playwright and Pillow, and the system Chromium
(`/usr/bin/chromium`; WebGL runs on SwiftShader):

```
python3 kit/build.py
python3 web3d/test/browser.py --quick          # about a minute
python3 web3d/test/browser.py [--only=a,b] [screenshot-dir]   # the full suite, about half an hour
```

Use whichever Python has Playwright, for instance a virtualenv's.

## Layout

| Path | What |
|---|---|
| `crates/strohmandeln-core` | the rules engine |
| `crates/strohmandeln-ai` | the computer opponent, the hint and the tournament |
| `crates/strohmandeln-wasm` | the engine as a JSON protocol, compiled to WebAssembly |
| `crates/strohmandeln-cli` | the terminal client |
| `kit/` | shared table code vendored from Cassino (`kit/STAMP`), and `build.py` |
| `web3d/` | the game's table: sources (`src/`), tests (`test/`, including the browser test), tutorial pages, phrase bank (`words.json`) and the built page |
| `docs/` | the documents below |
| `research/` | numbered research notes, review logs and the original-computation scripts |
| `tools/` | the phrase bank, the credits check, the contact sheet, the newspaper search |
| `bin/gate` | the gate before every commit |
| `index.html` | the address GitHub Pages serves, on to the page |

## Documents

- `docs/RULES.md`: the rules as implemented.
- `docs/PROTOCOL.md`: the wasm module's wire format.
- `docs/MESSAGES.md`: the messages of that protocol and the events behind the talk.
- `docs/PHRASES.md`: everything said at the table, with its source.
- `docs/TABLE3D.md`: the table, phase by phase.
- `docs/OPPONENT.md`: the computer opponent and how each rung was measured.
- `docs/review/`: code review logs, one per module.
- `PLAN.md`: where the work stands.
- `CREDITS.md`: the libraries and the period sources.

## Licence

MIT (`LICENSE`). Third-party software and sources are credited in
`CREDITS.md`.
