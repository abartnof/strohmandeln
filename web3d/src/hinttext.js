// The tutor's hint as words (docs/MESSAGES.md "Hint reasons"): the engine sends
// a key with parameters, a rule key and the search's odds; this names the
// cards, formats the thirds and returns the lines the table shows under the
// prompt: the card, why, and (when there is one) the rule and the odds.

// A parameter's value as it is told: a card code by name, a `_thirds` count
// as points with their unit ("4⅔ points", "⅔ point"), a count as the language
// groups it ("1,000" / "1.000", strings.js `count`; without `count`, as it is).
// Shown names come from the caller's cardName (strings.js `romanName`).
export function hintParams(params, { t, cardName, thirds, count }) {
  const out = {};
  for (const [name, v] of Object.entries(params ?? {})) {
    out[name] = typeof v === "string" ? cardName(v) : name.endsWith("_thirds") ? `${thirds(v)} ${t(v >= 1 && v <= 3 ? "hint.unit.one" : "hint.unit.other")}` : count && v >= 1000 ? count(v) : v;
  }
  return out;
}

// A sentence begins with a capital, whatever name opens it ("tarock 19 ...").
const cap = (s) => (s ? s[0].toUpperCase() + s.slice(1) : s);

// `r` is the engine's reply ({hint, reason, params, rule, odds}). Returns
// { lead, reason, more: [rule, odds] }: `lead` names the card to play (empty
// for a bid or a decline), `reason` is the key's sentence, `more` the
// clauses that follow it. The words come from `t`.
export function hintLines(r, { t, cardName, thirds, count }) {
  const words = { t, cardName, thirds, count };
  const play = r.hint?.action === "play" ? r.hint.card : null;
  const lead = play ? cap(t("hint.play", { card: cardName(play) })) : "";
  const reason = cap(t(`hint.reason.${r.reason}`, hintParams(r.params, words)));
  const more = [];
  if (r.rule) more.push(t(`hint.rule.${r.rule}`));
  if (r.odds) more.push(cap(t("hint.odds", hintParams(r.odds, words))));
  return { lead, reason, more };
}

// The whole hint as one string, for the screen reader.
export function hintSpoken(lines) {
  return [lines.lead, lines.reason, ...lines.more].filter(Boolean).join(" ");
}
