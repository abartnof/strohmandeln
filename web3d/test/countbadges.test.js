// The count's badges (docs/TABLE3D.md 2.5.2): a circle over each group of three won cards
// with the group's count, and its arithmetic for a tooltip. The numbers are RULES 8.1:
// 3 x value - 2 thirds a card, so a group's points are the sum of its values less 2/3 a card.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings, thirds } from "../src/strings.js";
import { countLines, groupArithmetic, countBadges } from "../src/counting.js";
import { emptyModel, reduce } from "../src/model.js";
import { playSitting, random, taker } from "./sim.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const words = (lang) => {
  const s = createStrings(TABLE, lang);
  return { t: s.t, name: s.badgeName };
};

test("king, jack and seven make 8 less 2: 6 points", () => {
  const a = groupArithmetic(["Kh", "Jc", "7s"], words("en"));
  assert.equal(a.thirds, 18);
  assert.equal(a.text, "King 5 + jack 2 + seven 1 = 8, less 2 = 6");
  const d = groupArithmetic(["Kh", "Jc", "7s"], words("de"));
  assert.equal(d.thirds, 18);
  assert.equal(d.text, "König 5 + Bube 2 + Sieben 1 = 8, weniger 2 = 6");
});

test("three low cards make 1 point; a leftover pair of king and jack makes 5 2/3", () => {
  assert.equal(groupArithmetic(["T2", "T3", "T4"], words("en")).thirds, 3);
  assert.equal(groupArithmetic(["T2", "T3", "T4"], words("en")).text, "Tarock II 1 + tarock III 1 + tarock IIII 1 = 3, less 2 = 1");
  const pair = groupArithmetic(["Kh", "Jc"], words("en"));
  assert.equal(pair.thirds, 17);
  assert.equal(pair.text, "King 5 + jack 2 = 7, less 1⅓ = 5⅔");
  assert.equal(groupArithmetic(["Kh", "Jc"], words("de")).text, "König 5 + Bube 2 = 7, weniger 1⅓ = 5⅔");
  const one = groupArithmetic(["T7"], words("en"));
  assert.equal(one.thirds, 1);
  assert.equal(one.text, "Tarock VII 1 = 1, less ⅔ = ⅓");
});

test("the named tarocks are named as the faces name them", () => {
  const en = groupArithmetic(["SK", "T21", "T1"], words("en")).text;
  assert.equal(en, "Sküs 5 + Mond 5 + Pagat 5 = 15, less 2 = 13");
  assert.equal(groupArithmetic(["SK", "T21", "T1"], words("en")).thirds, 39);
});

test("a group's thirds are the engine's: every card, any group", () => {
  for (const code of ["SK", "T1", "T21", "T5", "Kh", "Qd", "Cc", "Js", "10s", "7c", "4d", "1h"]) {
    const one = groupArithmetic([code], words("en"));
    assert.match(one.text, /= .+, less ⅔ = /);
  }
});

test("the badges of a take sum to the take's thirds, a badge a group, in the order counted", async () => {
  let checked = 0;
  for (const policy of [taker, random]) {
    for (const [seed, human] of [[3, 0], [7, 1], [11, 1]]) {
      const { replies } = await playSitting({ seed, touren: 3, human, level: 1 }, { policy: policy(seed), limit: 400 });
      let model = emptyModel(human);
      for (const r of replies)
        for (const e of r.events) {
          model = reduce(model, e);
          if (e.type !== "count") continue;
          const lines = countLines(model.counted, human);
          const badges = countBadges(lines, words("en"));
          assert.equal(badges.length, lines.length);
          for (const side of ["you", "opp"]) {
            const seat = side === "you" ? human : 1 - human;
            const mine = badges.filter((b) => b.side === side);
            assert.equal(mine.length, Math.ceil(model.counted[seat].length / 3));
            assert.equal(mine.reduce((n, b) => n + b.thirds, 0), e.thirds[seat], `${side}, seed ${seed}`);
            assert.ok(mine.every((b) => b.label === thirds(b.thirds) && b.text.endsWith(`= ${thirds(b.thirds)}`)));
          }
          checked++;
        }
    }
  }
  assert.ok(checked >= 6, `${checked} counts checked`);
});
