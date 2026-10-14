// The page's interface strings (docs/TABLE3D.md section 8): one table,
// `{key: {en, de}}`, with `{slot}` parameters. English is the default; a
// switch changes only what is drawn next, so callers subscribe with onLang
// and redraw. The engine holds no language: it sends keys and codes.
import TABLE from "../strings.json" with { type: "json" };

export const LANGS = Object.freeze(["en", "de"]);

const FRACTION = { 0: "", 1: "⅓", 2: "⅔" };

export function createStrings(table, initial = "en") {
  let lang = LANGS.includes(initial) ? initial : "en";
  const listeners = new Set();

  const fill = (s, params) => s.replace(/\{(\w+)\}/g, (m, k) => (params && k in params ? String(params[k]) : m));

  function t(key, params) {
    const entry = table[key];
    return entry ? fill(entry[lang], params) : key;
  }
  // Plurals are separate keys: `key.one` for 1, `key.other` otherwise; `{n}`.
  const tn = (key, n, params) => t(`${key}.${n === 1 ? "one" : "other"}`, { n, ...params });

  function setLang(next) {
    if (!LANGS.includes(next) || next === lang) return;
    lang = next;
    for (const fn of [...listeners]) fn(lang);
  }
  function onLang(fn) {
    listeners.add(fn);
    return () => listeners.delete(fn);
  }

  // Suit names follow the language; "trump" is the tarocks' (as `led`).
  const suitName = (s) => t(`card.suit.${s}`);

  // A card's name from its code (docs/RULES.md 1.2): "king of hearts",
  // "Herz-König"; the three named tarocks and the numbered ones.
  function cardName(code) {
    if (code === "SK") return t("card.sk");
    let m = /^T(\d+)$/.exec(code);
    if (m) return m[1] === "1" ? t("card.t1") : m[1] === "21" ? t("card.t21") : t("card.tarock", { n: m[1] });
    m = /^(K|Q|C|J|\d+)([hdcs])$/.exec(code);
    if (!m) return code;
    const suit = lang === "en" ? suitName(m[2]).toLowerCase() : suitName(m[2]);
    return /^\d/.test(m[1]) ? t("card.name.pip", { n: m[1], suit }) : t("card.name.court", { rank: t(`card.rank.${m[1]}`), suit });
  }

  return { t, tn, getLang: () => lang, setLang, onLang, suitName, cardName };
}

// ---- numbers, which do not depend on the language ------------------------

// Thirds of a point as the count is told: 107 -> "35⅔", 108 -> "36".
export function thirds(n) {
  const whole = Math.trunc(n / 3);
  const frac = FRACTION[n % 3];
  return frac ? (whole === 0 ? frac : `${whole}${frac}`) : String(whole);
}

// The one place a sum of money is written (docs/TABLE3D.md 4.3.3): kreuzer
// only. If the display currency changes, it changes here.
export const kreuzer = (n) => `${n} kr.`;

// A balance with its sign: "+7", "−7", "0".
export const signed = (n) => (n > 0 ? `+${n}` : n < 0 ? `−${-n}` : "0");

// ---- the shipped table ----------------------------------------------------

const shared = createStrings(TABLE);
export const { t, tn, getLang, setLang, onLang, suitName, cardName } = shared;
