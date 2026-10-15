// The tutorial pages as the tests read them: both files, parsed for a set of
// the game's settings, and their lines as plain text.
import { readFileSync } from "node:fs";
import { tutorialPages } from "../../src/tutor.js";

export const read = (p) => readFileSync(new URL(p, import.meta.url), "utf8");
export const STRINGS = JSON.parse(read("../../strings.json"));
// The page keys are the table of docs/TABLE3D.md 6.2.2, in its order.
export const KEYS = [...read("../../../docs/TABLE3D.md").split("### 6.2 Tutorial pages")[1].split("### 6.3")[0].matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
const FILES = { en: "../../tutorial.en.md", de: "../../tutorial.md" };

// The pages for the game's settings (TABLE3D 6.2.2): `both_pass`, and the rest at their defaults unless given.
export const pages = (lang, both_pass = "play", more = {}) => tutorialPages(read(FILES[lang]), KEYS, { both_pass, values: "2/3/4", ultimo: 1, ...more });
// Every combination of the three settings the pages depend on (RULES 0).
export const VALUES = { "2/3/4": [3, 4, 2], "1/2/3": [2, 3, 1], "1/2/4": [2, 4, 1] }; // the declarer wins, loses, a plain game
export const COMBOS = ["play", "throw_in"].flatMap((both_pass) => Object.keys(VALUES).flatMap((values) => [1, 2].map((ultimo) => ({ both_pass, values, ultimo }))));
export const allPages = (lang) => COMBOS.flatMap((c) => pages(lang, c.both_pass, c));
export const page = (lang, key, c = {}) => pages(lang, c.both_pass ?? "play", c).find((p) => p.key === key);
export const lines = (p) => p.blocks.flatMap((b) => (b.spans ? [b.spans] : b.items)).map((spans) => spans.map((x) => x.text).join(""));
export const text = (lang, key, c = {}) => lines(page(lang, key, c)).join("\n");
export const spansOf = (p) => p.blocks.flatMap((b) => (b.spans ? [b.spans] : b.items)).flat();
export const words = (s) => s.split(/\s+/).filter(Boolean).length;
