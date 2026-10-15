// The strings: both languages complete, slots matching, every message key
// of docs/MESSAGES.md and every event of docs/PROTOCOL.md voiced.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings, t, tn, getLang, setLang, onLang, thirds, kreuzer, signed } from "../src/strings.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const doc = (p) => readFileSync(new URL(`../../docs/${p}`, import.meta.url), "utf8");
// MESSAGES.md has two tables: refusals, then (after "Starting or replaying") setup errors, before the hint reasons.
const messagesTable = (which) => {
  const [refusals, rest] = doc("MESSAGES.md").split("Starting or replaying");
  return which === "refusals" ? refusals : rest.split("\n## Hint reasons")[0];
};
const slots = (s) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]).sort();

test("every key has a non-empty en and de, and no other language", () => {
  for (const [key, v] of Object.entries(TABLE)) {
    assert.deepEqual(Object.keys(v).sort(), ["de", "en"], key);
    assert.ok(v.en.trim() && v.de.trim(), key);
  }
});

test("every {slot} in en is in de and the reverse", () => {
  for (const [key, v] of Object.entries(TABLE)) assert.deepEqual(slots(v.en), slots(v.de), key);
});

test("the word the score HUD puts before a hand number has no slot", () => {
  const main = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");
  assert.match(main, /hand: t\("hud\.hand"\)/);
  assert.deepEqual(slots(TABLE["hud.hand"].en), []);
  assert.deepEqual(slots(TABLE["hud.hand"].de), []);
});

test("the Kibitzer's tab and mute say what they do, and § 13 is only the flavour", () => {
  const say = (key, lang, re) => assert.match(TABLE[key][lang], re, `${key} ${lang}`);
  for (const key of ["tutor.tab", "tutor.mute.label", "tutor.unmute.label"]) {
    say(key, "en", /tips/i);
    say(key, "de", /tipps/i);
  }
  say("tutor.mute.label", "en", /§ 13/);
  say("tutor.mute.title", "en", /§ 13: Kibitzer, shut your trap!.*joke/);
  say("tutor.mute.title", "de", /§ 13: Kiebitz, halt's Maul!.*scherz/i);
  say("tutor.tab.title", "en", /tips/i);
  say("tutor.tab.title", "de", /tipps/i);
  say("hint.muted", "en", /tips.*§ 13/i);
  say("hint.muted", "de", /tipps.*§ 13/i);
});

test("keys are namespaced", () => {
  const ok = /^(ui|chip|banner|hud|slate|refusal|hint|explain|review|sr|card|settings|error|tutor|tracker|keys)\./;
  for (const key of Object.keys(TABLE)) assert.match(key, ok, key);
});

test("every refusal key of docs/MESSAGES.md has a refusal.* string", () => {
  const keys = [...messagesTable("refusals").matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
  assert.ok(keys.length >= 9);
  for (const k of keys) assert.ok(TABLE[`refusal.${k}`], k);
});

test("every setup-error key of docs/MESSAGES.md has an error.* string with its parameters as slots", () => {
  const rows = [...messagesTable("errors").matchAll(/^\| `([a-z-]+)` \| ([^|]*) \|/gm)];
  assert.equal(rows.length, 4);
  for (const [, k, params] of rows) {
    const e = TABLE[`error.${k}`];
    assert.ok(e, k);
    const want = [...params.matchAll(/`([a-z]+)`/g)].map((m) => m[1]).sort();
    assert.deepEqual(slots(e.en), want, k);
    assert.deepEqual(slots(e.de), want, k);
  }
});

test("every event type of docs/PROTOCOL.md is voiced for the screen reader", () => {
  const types = new Set([...doc("PROTOCOL.md").matchAll(/^\{"type":"([a-z_]+)"/gm)].map((m) => m[1]));
  assert.ok(types.size >= 25);
  for (const ty of types) {
    const plain = TABLE[`sr.${ty}`];
    const seated = TABLE[`sr.${ty}.you`] && TABLE[`sr.${ty}.opp`];
    assert.ok(plain || seated, ty);
  }
});

test("every cause of credits has a slate.cause.* string", () => {
  for (const c of ["game", "trull", "kings", "ultimo_won", "ultimo_lost", "valat"]) assert.ok(TABLE[`slate.cause.${c}`], c);
});

test("plural keys come in pairs", () => {
  for (const key of Object.keys(TABLE)) {
    if (key.endsWith(".one")) assert.ok(TABLE[key.replace(/\.one$/, ".other")], key);
    if (key.endsWith(".other")) assert.ok(TABLE[key.replace(/\.other$/, ".one")], key);
  }
});

test("the opponent is never named, and addressed formally in German", () => {
  for (const [key, v] of Object.entries(TABLE)) {
    assert.ok(!/\b(du|dein|deine|dir)\b/i.test(v.de), `${key}: du-form`);
  }
  assert.equal(TABLE["slate.opp"].en, "Your opponent");
  assert.equal(TABLE["slate.opp"].de, "Ihr Gegner");
});

test("t fills slots, falls back to the key, and follows the language", () => {
  const s = createStrings({ a: { en: "Hi {name}", de: "Tag {name}" } });
  assert.equal(s.getLang(), "en");
  assert.equal(s.t("a", { name: "X" }), "Hi X");
  assert.equal(s.t("missing"), "missing");
  assert.equal(s.t("a"), "Hi {name}");
  s.setLang("de");
  assert.equal(s.t("a", { name: "X" }), "Tag X");
  s.setLang("fr");
  assert.equal(s.getLang(), "de", "an unknown language is ignored");
});

test("onLang calls back on a change only, and unsubscribes", () => {
  const s = createStrings({});
  const seen = [];
  const off = s.onLang((l) => seen.push(l));
  s.setLang("de");
  s.setLang("de");
  s.setLang("en");
  off();
  s.setLang("de");
  assert.deepEqual(seen, ["de", "en"]);
});

test("tn picks the plural form", () => {
  const s = createStrings({ "x.one": { en: "{n} Tour", de: "{n} Tour" }, "x.other": { en: "{n} Tours", de: "{n} Touren" } });
  assert.equal(s.tn("x", 1), "1 Tour");
  assert.equal(s.tn("x", 3), "3 Tours");
  s.setLang("de");
  assert.equal(s.tn("x", 3), "3 Touren");
});

test("count groups four digits and more as the language writes them", () => {
  const s = createStrings(TABLE);
  const all = [0, -0, 7, 999, 1000, 12345, 1234567, -999, -1000, -1234567];
  assert.deepEqual(all.map(s.count), ["0", "0", "7", "999", "1,000", "12,345", "1,234,567", "-999", "-1,000", "-1,234,567"]);
  s.setLang("de");
  assert.deepEqual(all.map(s.count), ["0", "0", "7", "999", "1.000", "12.345", "1.234.567", "-999", "-1.000", "-1.234.567"]);
  s.setLang("en");
  assert.equal(s.count(1000), "1,000", "a language switch is followed");
  // Only whole counts are grouped.
  assert.equal(s.count(1234.5), "1234.5");
});

test("thirds are written with the fraction glyphs", () => {
  assert.equal(thirds(107), "35⅔");
  assert.equal(thirds(106), "35⅓");
  assert.equal(thirds(108), "36");
  assert.equal(thirds(2), "⅔");
  assert.equal(thirds(0), "0");
});

test("kreuzer and signed numbers", () => {
  assert.equal(kreuzer(12), "12 kr.");
  assert.equal(kreuzer(0), "0 kr.");
  assert.equal(signed(7), "+7");
  assert.equal(signed(-7), "−7");
  assert.equal(signed(0), "0");
});

test("card names are built from the code, by language", () => {
  const s = createStrings(TABLE);
  assert.equal(s.cardName("Kh"), "king of hearts");
  assert.equal(s.cardName("3d"), "3 of diamonds");
  assert.equal(s.cardName("SK"), "Sküs");
  assert.equal(s.cardName("T1"), "Pagat");
  assert.equal(s.cardName("T21"), "Mond");
  assert.equal(s.cardName("T14"), "tarock 14");
  s.setLang("de");
  assert.equal(s.cardName("Kh"), "Herz-König");
  assert.equal(s.cardName("Js"), "Pik-Bube");
  assert.equal(s.cardName("Cc"), "Kreuz-Cavall");
  assert.equal(s.suitName("trump"), "Tarock");
  assert.equal(s.suitName("h"), "Herz");
});

test("the default instance reads the shipped table", () => {
  assert.equal(getLang(), "en");
  assert.equal(t("refusal.must-trump"), "You must trump.");
  assert.equal(tn("slate.touren", 3), "3 Touren");
  const seen = [];
  const off = onLang((l) => seen.push(l));
  setLang("de");
  assert.equal(t("refusal.must-trump"), "Sie müssen stechen.");
  setLang("en");
  off();
  assert.deepEqual(seen, ["de", "en"]);
});

test("the spec's refusal wording is used", () => {
  assert.equal(t("refusal.must-follow-suit", { led: "Hearts" }), "Follow suit: Hearts.");
  assert.equal(t("refusal.not-playable"), "That card is not in your hand or showing.");
  setLang("de");
  assert.equal(t("refusal.not-held", { honour: "die Troule" }), "Sie haben die Troule nicht.");
  setLang("en");
});

test("the settlement line says what is owed, not that it was paid", () => {
  assert.ok(!/bezahlt/i.test(TABLE["slate.owed"].de), TABLE["slate.owed"].de);
});

test("the both-pass note names the sources of the rules (research/14 rule 7)", () => {
  const note = TABLE["settings.both_pass.note"];
  for (const lang of ["en", "de"]) for (const who of ["Werner", "Mayr", "Sedlaczek", "Ulmann", "Beck"]) assert.ok(note[lang].includes(who), `${lang} ${who}`);
});

// The review and explanation keys of docs/MESSAGES.md, with the texts it gives.
const messageRows = (from, to) => {
  const part = doc("MESSAGES.md").split(from)[1].split(to ?? "\n## END")[0];
  return [...part.matchAll(/^\| `([a-z-]+)` \| [^|]*\| ([^|]*) \| ([^|]*) \|/gm)].map((m) => ({ key: m[1], en: m[2].trim(), de: m[3].trim() }));
};

test("every review key of docs/MESSAGES.md has a review.say.* string with the text the doc gives", () => {
  const rows = messageRows("\n## Review keys", "\n## Explanation keys");
  assert.equal(rows.length, 13);
  for (const r of rows) {
    const s = TABLE["review.say." + r.key];
    assert.ok(s, r.key);
    assert.equal(s.en, r.en, r.key);
    assert.equal(s.de, r.de, r.key);
  }
  for (const lead of ["count", "decided", "sitting"]) assert.ok(TABLE["review.lead." + lead], lead);
});

test("every explanation key of docs/MESSAGES.md has an explain.* string with the text the doc gives", () => {
  const rows = messageRows("\n## Explanation keys");
  assert.equal(rows.length, 8);
  for (const r of rows) {
    const s = TABLE["explain." + r.key.slice("explain-".length)];
    assert.ok(s, r.key);
    assert.equal(s.en, r.en, r.key);
    assert.equal(s.de, r.de, r.key);
  }
});

test("the declarer is the Aufnehmer in German, as in the books", () => {
  for (const k of ["banner.declarer-win", "banner.declarer-loss", "hud.declarer", "sr.game_scored.declarer_win", "sr.game_scored.declarer_loss"]) {
    assert.ok(/Aufnehmer/.test(TABLE[k].de), k);
    assert.ok(!/Ich nehme auf/.test(TABLE[k].de), k);
  }
});

test("choosing a card is announced, with how to play it (there is no Play button)", () => {
  const main = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");
  assert.equal((main.match(/t\("sr\.chosen"/g) ?? []).length, 2, "tap and keyboard both announce");
  assert.match(TABLE["sr.chosen"].en, /again.*play/i);
  assert.match(TABLE["sr.chosen"].de, /nochmals.*ausspielen/i);
});

test("a scrolled or resized hand forgets where the first click fell", () => {
  const main = readFileSync(new URL("../src/main.js", import.meta.url), "utf8");
  assert.equal((main.match(/scroll: clampScroll\([^;]*\), at: null/g) ?? []).length, 2, "drag-scroll and wheel");
  assert.match(main, /sel = \{ \.\.\.sel, at: null \}; \/\/ cards move on a resize/);
});
