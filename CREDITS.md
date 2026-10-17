# Credits

The project's own work (the rules engine, the table, the faces, the phrase
bank, the documents) is released under the **MIT License** (`LICENSE`).
Everything listed below is someone else's and keeps its own licence.
Quotations in the research notes, the literature review and the phrase bank
remain their authors', credited where they appear.

Every third-party asset is recorded here, in the commit that brings it in:
art, fonts, icons, libraries, build tools. `tools/credits_check.py` (run by
`bin/gate`) fails if a direct dependency in `web3d/package.json` or any
crate's `Cargo.toml` is not named in this file, and `tools/test_credits.py`
fails if a source of the phrase bank is not named below.

## Art and fonts

None. The 54 card faces and the back are drawn at runtime with Canvas 2D
(`web3d/src/tarockfaces.js`); no image, scan or figure art is bundled or
fetched. Text is set in the reader's own system fonts (the stacks are in
`tarockfaces.js` and the stylesheets). The slate uses the page's system font
(`web3d/src/style.css`); no font file is embedded or fetched. No icon set is used.

The structure of the pack (22 trumps, the Sküs, four suits of eight) and the
trumps' nicknames follow the sources in `research/08-cards-and-art.md`.

## Software in the page

The single-file page (`web3d/strohmandeln.html`) bundles three.js. The build
keeps its licence comment at the end of the script. The game logic runs as
WebAssembly compiled from this repository's own Rust; the Rust dependencies
below are compiled into that module.

| Package | Version | Licence | Holder |
|---|---|---|---|
| three.js | 0.186.1 | MIT | three.js authors |
| serde | 1 | MIT OR Apache-2.0 | David Tolnay and the Serde authors |
| serde_json | 1 | MIT OR Apache-2.0 | David Tolnay and the Serde authors |

serde brings in serde_core and serde_derive, and through them proc-macro2,
quote, syn and unicode-ident (each MIT OR Apache-2.0). serde_json brings in
itoa (MIT OR Apache-2.0), memchr (Unlicense OR MIT) and zmij (MIT). The exact
versions are in `Cargo.lock`. `serde_json` is also a dev-dependency of the engine
(`strohmandeln-core`). `strohmandeln-ai` and `strohmandeln-cli` depend on
nothing outside this repository.

## Code the kit was copied from

`kit/` is a vendored copy of the game-independent parts of the earlier
table in the series, **Cassino** (`kit/STAMP` gives the commit and, file by
file, how each differs). Cassino and Strohmandeln share an author and a
licence (MIT).

## Build tools (used, not shipped)

| Tool | Version | Licence | Use |
|---|---|---|---|
| esbuild | 0.28.2 | MIT | Bundling the page's JavaScript |
| Rust toolchain (`rust-toolchain.toml`) | pinned there | MIT OR Apache-2.0 | Building the engine and its wasm module |
| Playwright (Python) with the system Chromium (SwiftShader for WebGL) | unpinned | Apache-2.0 | `web3d/test/browser.py`, the page in a real browser, offline (`--quick` is part of the gate when installed) |
| Pillow | unpinned | MIT-CMU (HPND) | `web3d/test/browser.py`, reading screenshots |
| Playwright (WebKit) | unpinned | Apache-2.0 | `tools/contact_sheet.mjs`, which draws the faces to `docs/img/faces.png` (not run by the gate) |

## The period sources behind the table talk and the tutor

The opponent's lines (`docs/PHRASES.md`, generated from `tools/phrases.py`)
and the tutor quote or adapt the texts below. Quotations are short and
attributed; each
phrase carries its source key and locator in `docs/PHRASES.md`, and the
research note named in brackets holds the transcription and the URL where it
was read.

- Werner, *Das moderne Tarokspiel*, 2nd ed. (Vienna: Hartleben, by 1891)
  [`research/03-period-rulebooks.md`]. The Strohmann-Tarok section.
- Ulmann, *Illustrirtes Wiener Tarokbuch*, 2nd ed. (1899) [03].
- Bermann, *Der praktische Tarokspieler* (Vienna, 1894) [03].
- Haas, *Neuestes allgemeines Spielbuch* (Vienna: Haas, 1829) [13].
- Alvensleben, *Encyklopädie der Spiele* (Leipzig: Wigand, 1853), pp.
  561, 576-577 [`05d`, `12`].
- Nestroy, *Zu ebener Erde und erster Stock* (1835), *Nagerl und Handschuh*
  (1832), *Die Familien Zwirn, Knieriem und Leim* (1834), *Einen Jux will er
  sich machen* (1842), *Unverhofft* (1845), in the Bonz editions of 1890-91 and Projekt
  Gutenberg [`05`, `05c`].
- Moser, "Die Tarok- und Hundert-Eins-Brüder" (Vienna, 1844) [`05`].
- Bauer, *Ode auf das Taroc-Spiel* (Vienna and Nuremberg, 1756) [`05`, `05d`].
- Rosenau, *Scüs, Mond und Pagat* (Vienna, 1821) [`05d`].
- Hügel, *Der Wiener Dialekt* (1873) [`05`].
- *Idioticon Austriacum* (Vienna, 1824), as also given in Wander [`05b`].
- Schranka, *Wiener Dialekt-Lexikon* (Vienna, 1905) [`05`].
- Wander, *Deutsches Sprichwörter-Lexikon* (Leipzig, 1867-1880) [`05b`].
- *Meyers Konversations-Lexikon*, 4th ed. (1889), s.v. Tarock [`05`].
- Henrici (1727), as quoted in Grimm, *Deutsches Wörterbuch*, s.v. Stich
  [`05b`].
- Blum, *Spatziergänge* II (Berlin, 1775) [`05b`].
- Schubin, *Vollmondzauber* (1899) [`05`].
- *Zigo-Taroc* (Mannheim, 1860) [`05`].
- *Die Gartenlaube* (1869) [`09`].
- Ranzenhofer, "Königrufen", *Pilsner Tagblatt*, 30 Sep 1917; Löwy, "Der
  Giftnik'l", *Illustrirtes Wiener Extrablatt*, 23 Aug 1903 [`05c`].
- Periodicals read through ANNO, the Austrian National Library's digitised
  newspapers (<https://anno.onb.ac.at/>): *Figaro* (1887, 1888), *Der Floh*
  (1887, 1900), *Neue Freie Presse* (1893), *Neues Wiener Journal* (1895),
  *Neues Wiener Tagblatt* (1887), *Illustrirtes Wiener Extrablatt* (1900),
  *Linzer Volksblatt* (1891, 1892), *Grazer Tagblatt* (1894, 1896),
  *Kärntner Zeitung* (1908, 1911), *Die Muskete* (1913, 1923),
  *(Neuigkeits) Welt Blatt* (1904, 1924), *Das kleine Volksblatt* (1934), *Jörgel Briefe* (1883),
  *Glühlichter* (1898), *Bettauers Wochenschrift* (1927), *Prager
  Tagblatt* (1892), *Kikeriki* (1887, 1897). Each citation gives date and page.

## Rules, history and method

Modern descriptions consulted for the rules, not quoted in the game:
Mayr and Sedlaczek in the *Wiener Zeitung* (25 Sept 2007); Furr, *Tarocchi*
(2009); Dummett, *The Game of Tarot* (London: Duckworth, 1980). The consensus is worked out in
`research/14-consensus-rules.md` and every rule in `docs/RULES.md` cites it.
The full source list is in `strohmandeln-lit-review.md` and the research
notes' source tables. A GPL-3.0 implementation of the game, smatkovi's
harbour-tarock, was read for questions only; none of its text or code is
used (`research/12-followups.md`).
