// The strings: both languages complete, slots matching, every message key
// of docs/MESSAGES.md and every event of docs/PROTOCOL.md voiced.
import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { createStrings, t, tn, getLang, setLang, onLang, thirds, kreuzer, signed } from "../src/strings.js";

const TABLE = JSON.parse(readFileSync(new URL("../strings.json", import.meta.url), "utf8"));
const doc = (p) => readFileSync(new URL(`../../docs/${p}`, import.meta.url), "utf8");
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

test("keys are namespaced", () => {
  const ok = /^(ui|chip|banner|hud|slate|refusal|hint|review|sr|card|settings|error|tutor|keys)\./;
  for (const key of Object.keys(TABLE)) assert.match(key, ok, key);
});

test("every refusal key of docs/MESSAGES.md has a refusal.* string", () => {
  const keys = [...doc("MESSAGES.md").matchAll(/^\| `([a-z-]+)` \|/gm)].map((m) => m[1]);
  assert.ok(keys.length >= 10);
  for (const k of keys) assert.ok(TABLE[`refusal.${k}`], k);
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
