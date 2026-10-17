// The declarer's mark on the score HUD (docs/TABLE3D.md 2.5.8) and the dealer's chip.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings } from "../src/strings.js";
import { declarerMark, dealerTag } from "../src/declarer.js";
import { emptyModel, reduce } from "../src/model.js";
import { passer, playSitting, taker } from "./sim.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const strings = (lang) => createStrings(TABLE, lang).t;
const model = (declarer, dealer = 0) => ({ ...emptyModel(0), dealer, declarer });

test("the declarer's side, the mark and its tooltip, in both languages", () => {
  assert.deepEqual(declarerMark(model(0), strings("en")), { side: "you", text: "D", title: "Declarer: took it up; needs 36 card points of 70 to win." });
  assert.equal(declarerMark(model(1), strings("en")).side, "opp");
  assert.deepEqual(declarerMark(model(1), strings("de")), { side: "opp", text: "A", title: "Aufnehmer: hat aufgenommen; braucht 36 Augen von 70 zum Gewinn." });
});

test("no declarer, no mark", () => {
  assert.equal(declarerMark(model(null), strings("en")), null);
  assert.equal(declarerMark(null, strings("en")), null);
});

// The marks along a sitting's events: [{ type, simple, mark }] after each one.
async function along(policy) {
  const { replies } = await playSitting({ seed: 3, touren: 3, human: 0, level: 1 }, { policy });
  let m = emptyModel(0);
  let simple = false;
  const out = [];
  for (const r of replies) {
    for (const e of r.events) {
      m = reduce(m, e);
      if (e.type === "deal_start") simple = false;
      if (e.type === "both_pass" && e.mode === "play") simple = true;
      out.push({ type: e.type, simple, mark: declarerMark(m, strings("en")) });
    }
  }
  return out;
}

test("a simple game (both passed) has no declarer, so no mark, to the deal's end", async () => {
  const seen = await along(passer(3));
  assert.ok(seen.some((x) => x.simple), "a simple game was played");
  for (const x of seen) if (x.simple) assert.equal(x.mark, null, `after ${x.type}`);
});

test("a bid marks the declarer until the next deal", async () => {
  const seen = await along(taker(3));
  assert.ok(seen.some((x) => x.mark), "a bid was made");
  const first = seen.findIndex((x) => x.mark);
  assert.ok(seen.slice(first).some((x) => x.type === "deal_start" && !x.mark), "the mark goes at the next deal");
});

test("the dealer's chip says the word, with who speaks first in its tooltip", () => {
  assert.deepEqual(dealerTag(model(null, 0), strings("en")), { text: "Dealer", title: "You deal this deal; your opponent speaks first." });
  assert.deepEqual(dealerTag(model(null, 1), strings("en")), { text: "Dealer", title: "Your opponent deals this deal; you speak first." });
  assert.deepEqual(dealerTag(model(null, 1), strings("de")), { text: "Geber", title: "Ihr Gegner gibt diese Partie; Sie sprechen zuerst." });
  assert.equal(dealerTag(model(null, null), strings("en")), null);
});

test("no bare letter stands for the dealer: the chip has its word, the slate's English mark is not a lone G", () => {
  assert.equal(TABLE["hud.dealer.short"], undefined);
  assert.equal(TABLE["slate.dealer"].en, "Dlr");
  assert.deepEqual(TABLE["hud.opp"], { en: "Opponent", de: "Gegner" }, "the HUD's short name, so a mark leaves it two lines at most");
  assert.equal(TABLE["slate.dealer"].de, "G");
});
