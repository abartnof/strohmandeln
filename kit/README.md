# kit/ - the card table's game-independent code

A vendored copy of the parts of Cassino's 3D table and build tooling that do
not know which game they serve, for Strohmandeln and later games. It is
ordinary files in this repository: no submodule, no package, no sync step at
build time. `STAMP` says which Cassino commit it was copied from and lists
every file with its source path and how it differs.

Why a copy and not a submodule: `research/review/RC.md` §4. A submodule is
empty in a fresh `git worktree add` (every Claude Code agent worktree),
needs two commits for one change, and esbuild could not find `three` from
inside it. A plain directory is in every worktree from the start.

## What is in it

`js/` (every file starts with `// From cassino web3d/src/<file> @ 73f0cd7`):

| Area | Files |
|---|---|
| Render | `scene.js` (stage, lights, table, camera), `cards.js` (card slab), `materials.js` (toon shading), `deck.js` (meshes by card code, ink line), `surfaces.js` (table tops), `framing.js` (camera fit to the window), `art.js` (image-map loader), `faces.js` (Large Text faces) |
| Motion | `easing.js`, `kinematics.js` (card paths: pose, layDown, pull, toss, flip, fan, peelOff, carryBlock ...), `timeline.js` |
| Loop | `director.js` (render on demand, a clock a gate can hold, picking, screen points) |
| Talk | `bag.js` (shuffle bag), `dialogue.js` (when and where a line is said) |
| Shell | `tutorial.js` (the small Markdown parser), `engine.js` (wasm wrapper) |
| Constants | `units.js` (CARD, TABLE, CAMERA*, PORTRAIT_BELOW), `reveal.js` (table edges, fog, the camera's way back) |

`test/`: the Node tests that cover them, plus tests for what the kit changed
(`engine`, `tutorial`, `director`, `modules`) and `test_build.py` for
`build.py`. `build.py`: the page builder, below. `STAMP`: provenance.

## What was left out, and why

- `prefs.js`, `progress.js`: generic in shape (versioned `localStorage`, try/catch,
  URL overrides) but every key, default and the evidence format is Cassino's.
  Copy them as a pattern when the game needs settings.
- `layout.js`, `choreography.js`, `main.js`, `overlay.js`, `talk.js`, `hud.js`,
  `chrome.js`, `style.css`, `index.html`: game logic or page design. `ZONES*` of
  `units.js` and the courts of `reveal.js` went with them.
- `tutorial.js`'s page keys, `moveKinds`, `pageDue`: Cassino's pages. The
  parser stays; the keys are an argument.
- `tutorworker*.js`, `series.js`, `selection.js`, `review.js`, `announce.js`: tied to Cassino's
  protocol; not yet shown to be shared.

## How a game uses it

The game lives in `web3d/` next to `kit/`. Import with relative paths:

```js
// web3d/src/main.js
import { createScene } from "../../kit/js/scene.js";
import { createDirector } from "../../kit/js/director.js";
import { choreograph, initialPlacement, opening } from "./choreography.js"; // the game's
import { ZONES } from "./zones.js";                                          // the game's
const director = createDirector({ stage, deck, view, decorate, rested,
  choreography: { choreograph, initialPlacement, opening }, zones: ZONES, cards: 54 });
```

What the game supplies:

- `choreography`: `{ choreograph(prev, next, placement, view, opts), initialPlacement(state, view), opening(state) }`.
  Placements are `{ id, code, zone, pose }`; the director's `handEdge` and
  `screenPoint` look for the zones named `"your-hand"` and `"their-hand"`.
- `zones` (its own `zones.js`, replacing Cassino's `ZONES*`), and the layout.
- `units.js`'s camera numbers are Cassino's: starting values. Pass the game's
  own `eye` to `framing.js` (`aim`, `framing`, `cameraFor`) once its zones are
  measured.
- `faces.js` draws the 52-card ranks (A 2-10 J Q K) and four suits. Trumps with
  Roman numerals need the game to draw its own or to widen `faces.js` (its
  `rankAndSuit`, `GLYPH`, `fitted`).
- `engine.js`: `loadWasm(bytes, "<prefix>")` wants the module to export
  `<prefix>_alloc`, `<prefix>_out`, `<prefix>_out_len` and `memory`. The game's
  API object (its commands) is the game's; wrap it in `guard`.
- Card art and the tutorial text, the phrase bank, a page template and style.

### The `three` problem, and why it does not recur

`kit/js/*.js` import the bare specifier `three`. Node and esbuild resolve a bare
specifier by walking up from the importing file: `kit/js` -> `kit` -> the
repository root. They never visit `web3d/node_modules`, so `three` is not
found (RC reproduced exactly this with a submodule). Two things prevent it
here, and either one is enough:

1. `kit/node_modules` is a committed symlink to `../web3d/node_modules`. Node
   (the kit's tests, the game's tests) and esbuild find `three` there, from any
   kit file, in any worktree, after `npm ci` in `web3d/`. (`.gitignore` has
   `node_modules/`, which matches directories only, so the symlink is tracked.)
2. `build.py` also runs esbuild with `NODE_PATH=<web3d>/node_modules`.

There is exactly one copy of `three`: the game's, pinned by `web3d/package-lock.json`
(three 0.186.1, esbuild 0.28.2, the versions Cassino pins). `kit/package.json`
only says `"type": "module"`. `bin/gate` fails (it does not skip) when
`web3d/node_modules` is missing or the symlink is broken, and says to run
`npm ci` in `web3d/`. In a new worktree run it once before the first gate.

## build.py

`python3 kit/build.py` (default config `web3d/game.toml`) builds the one-file
page; `--check` fails if the committed page differs (the build is
deterministic); `--dry-run` prints the plan and what is missing; `--help` has
the rest. The config format is at the top of `build.py`: game name, wasm crate
(omit it before there is an engine), entry, template, style, output, optional
`worker_entry` (the tutor's worker, bundled apart and inlined as text),
`[[assets]]` (images folder, json, text) and a size guideline. The weigh-in
names the kit's bytes separately from three.js and the game's own.

## Changing the kit

A change the game needs goes into `kit/` here, with a test in `kit/test/`, in a
commit of its own (not mixed with game code). Keep the origin header and note
the change in a comment in the file and in `STAMP`.

## Re-syncing from Cassino

Cassino is read-only here and keeps improving. To take a newer version:

1. Read Cassino's `git log` since the `commit:` in `STAMP`; look at the diff of
   the files `STAMP` lists (`git -C ~/cassino diff <stamp-commit> HEAD -- web3d/src/scene.js ...`).
2. For a file marked `verbatim`, copy it over and put the header
   `// From cassino web3d/src/<file> @ <new short hash>` on top. For a changed
   file (units, reveal, director, tutorial, engine, faces test), re-apply
   the documented change by hand to the new source.
3. Copy the tests the same way (`../src/` becomes `../js/`), run `bin/gate`,
   update `STAMP` (commit, date, files), and commit the sync alone.

The kit's JS stays free of Cassino's names; `kit/test/modules.test.js` fails if a
kit file imports a file the kit lacks.
