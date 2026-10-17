// The tutor's hint as words (docs/HINTS.md 3.1, docs/MESSAGES.md "Hint reasons"
// and "Hint questions, sentences"): the engine sends keys with parameters; this
// names the cards, formats the thirds and returns what the table shows under the
// prompt: the headline (the card), the decisive question and one sentence, and
// the parts of the "Why?".

// A parameter's value as it is told: a `suit` by its name ("hearts" / "Herz"), a
// card code by name, a `_thirds` count as points with their unit ("4⅔ points",
// "⅔ point"), a count as the language groups it ("1,000" / "1.000", strings.js
// `count`; without `count`, as it is). Shown names come from the caller's
// cardName (strings.js `romanName`).
export function hintParams(params, { t, cardName, thirds, count }) {
  const out = {};
  for (const [name, v] of Object.entries(params ?? {})) {
    out[name] = name === "suit" ? t(`hint.suit.${v}`) : typeof v === "string" ? cardName(v) : name.endsWith("_thirds") ? `${thirds(v)} ${t(v >= 1 && v <= 3 ? "hint.unit.one" : "hint.unit.other")}` : count && typeof v === "number" ? count(v) : v;
  }
  return out;
}

// A sentence begins with a capital, whatever name opens it ("tarock 19 ...").
const cap = (s) => (s ? s[0].toUpperCase() + s.slice(1) : s);

// The card `ask.keep` is about, as it is spoken: the three key cards with their
// article ("the Mond"), a kept packet by the card it shows.
const keepName = (code, { t, cardName }) => (["T1", "T21", "SK"].includes(code) ? t(`hint.keep.${code}`) : t("hint.keep.packet", { card: cardName(code) }));

// `r` is the engine's reply ({hint, reason, params, rule, odds, question, keep, so,
// ans, why}). Returns { lead, question, text, why, reason, more }:
//  - `lead` names the card to play (empty for a bid or a decline), said once;
//  - `question` is the decisive question ("" when none);
//  - `text` is the one sentence: the rule (`so`), else the fact (`ans`), else the
//    plain reason (a bid, or an older reply without them);
//  - `why` is the "Why?": { alt, contrast, trumped, facts: [{ text, anchor }],
//    rule, odds }, or null for a reply with none;
//  - `reason` and `more` are the older shape: the reason's sentence and the rule
//    and odds clauses after it.
// The words come from `t`.
export function hintLines(r, { t, cardName, thirds, count, whyWords = WHY_WORDS }) {
  const words = { t, cardName, thirds, count };
  const say = (l) => (l ? cap(t(`hint.${l.key}`, hintParams(l.params, words))) : "");
  const play = r.hint?.action === "play" ? r.hint.card : null;
  const lead = play ? cap(t("hint.play", { card: cardName(play) })) : "";
  const reason = cap(t(`hint.reason.${r.reason}`, hintParams(r.params, words)));
  const question = r.question ? cap(t(`hint.${r.question}`, r.keep ? { keep: keepName(r.keep, words) } : {})) : "";
  const text = say(r.so) || say(r.ans) || reason;
  const more = [];
  if (r.rule) more.push(t(`hint.rule.${r.rule}`));
  if (r.odds) more.push(cap(t("hint.odds", hintParams(r.odds, words))));
  const facts = (r.why?.facts ?? []).map((f) => ({ text: say(f), anchor: f.anchor }));
  const oddsText = r.why?.odds ? cap(t("hint.odds", hintParams(r.why.odds, words))) : null;
  const why = r.why
    ? {
        alt: r.why.alt ?? null,
        contrast: say(r.why.contrast) || null,
        trumped: say(r.why.trumped) || null,
        facts,
        rule: r.why.rule ?? null,
        odds: oddsText,
        lines: whyLines(
          [
            deals(r, words),
            say(r.why.contrast),
            say(r.why.trumped),
            ...facts.map((f) => f.text && f),
            r.rule ? t(`hint.rule.${r.rule}`) : "",
            oddsText,
          ],
          whyWords,
        ),
      }
    : null;
  return { lead, question, text, why, reason, more };
}

// The Why? is at most 90 words (docs/HINTS.md 3.1): its lines in order of weight,
// the last ones left out when the budget is spent. A line is { text, anchor? }.
export const WHY_WORDS = 90;
export const wordCount = (s) => s.trim().split(/\s+/).filter(Boolean).length;

function whyLines(parts, limit) {
  const lines = [];
  let total = 0;
  for (const part of parts) {
    const line = typeof part === "string" ? { text: part } : part;
    if (!line?.text) continue;
    const n = wordCount(line.text);
    if (total + n > limit) break;
    total += n;
    lines.push(line.anchor ? line : { text: line.text });
  }
  return lines;
}

// "I deal his hidden cards at random {w} times and play each out." when there is
// a sample to speak of: the deals of the contrast, else of the odds.
function deals(r, { t, count }) {
  const p = r.why?.contrast?.params;
  const w = p ? (p.worlds ?? p.w) : r.why?.odds?.worlds;
  return w ? cap(t("hint.why.intro", { w: count ? count(w) : w })) : "";
}

// The default hint as one string, for the screen reader: the card, the question,
// the sentence.
export function hintSpoken(lines) {
  return [lines.lead, lines.question, lines.text].filter(Boolean).join(" ");
}
