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
export function hintLines(r, { t, cardName, thirds, count }) {
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
  const why = r.why
    ? {
        alt: r.why.alt ?? null,
        contrast: say(r.why.contrast) || null,
        trumped: say(r.why.trumped) || null,
        facts: (r.why.facts ?? []).map((f) => ({ text: say(f), anchor: f.anchor })),
        rule: r.why.rule ?? null,
        odds: r.why.odds ? cap(t("hint.odds", hintParams(r.why.odds, words))) : null,
      }
    : null;
  return { lead, question, text, why, reason, more };
}

// The default hint as one string, for the screen reader: the card, the question,
// the sentence.
export function hintSpoken(lines) {
  return [lines.lead, lines.question, lines.text].filter(Boolean).join(" ");
}
