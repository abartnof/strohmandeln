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

  // The name as shown in prose: a numbered tarock by the faces' Roman numeral
  // ("tarock XII"); the rest as `cardName`. Screen-reader text keeps `cardName`.
  function romanName(code) {
    const m = /^T(\d+)$/.exec(code);
    return m && m[1] !== "1" && m[1] !== "21" ? t("card.tarock", { n: tarockNumeral(+m[1]) }) : cardName(code);
  }

  // A card's short name in the count's arithmetic: a suit card by its rank alone ("king",
  // "seven"; the suit is no part of its value), the tarocks as `romanName`.
  function badgeName(code) {
    const m = /^(K|Q|C|J|\d+)[hdcs]$/.exec(code);
    if (!m) return romanName(code);
    return /^\d/.test(m[1]) ? t(`hud.badge.pip.${m[1]}`) : t(`card.rank.${m[1]}`);
  }

  // A whole count as the language writes it: 1,000 in English, 1.000 in
  // German (four digits and more are grouped, a smaller count is as it is);
  // anything that is not a whole number is left as it is.
  const GROUPING = { en: new Intl.NumberFormat("en-US"), de: new Intl.NumberFormat("de-DE") };
  const count = (n) => (Number.isInteger(n) ? GROUPING[lang].format(n + 0) : String(n));

  return { t, tn, getLang: () => lang, setLang, onLang, suitName, cardName, romanName, badgeName, count };
}

// Roman numerals as on the real pack: standard subtractive numerals, except 4, which is IIII.
export function tarockNumeral(n) {
  if (!Number.isInteger(n) || n < 1 || n > 21) throw new RangeError(`no tarock ${n}`);
  if (n === 4) return "IIII";
  let out = "";
  let rest = n;
  for (const [v, s] of [[10, "X"], [9, "IX"], [5, "V"], [4, "IV"], [1, "I"]]) {
    while (rest >= v) {
      out += s;
      rest -= v;
    }
  }
  return out;
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
export const { t, tn, getLang, setLang, onLang, suitName, cardName, romanName, badgeName, count } = shared;
