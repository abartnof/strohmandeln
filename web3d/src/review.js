// The end-of-deal brief for its dialog (docs/TABLE3D.md 6.4, docs/TUTOR.md 6):
// the count as the header, then the module's bullets (docs/PROTOCOL.md "The
// tutor": a lead key, a key, parameters) put into the words of the current
// language, at most three, each with a bold lead, and a folded "How is this
// worked out?". The page adds no judgement of its own: every sentence is a
// key of docs/MESSAGES.md "Tutor keys" or "Review keys".
//
// Cassino's review.js (briefBlocks) is the pattern.

const CARDS = ["played", "better", "led", "card", "king", "mine", "theirs"];
const THIRDS = ["thirds", "sure"];

// Parameters the table formats before they are put into a sentence: cards are
// named, thirds written as points and thirds, a skill by its name, the honour
// and the suit named, the net of a sitting signed.
function params(bullet, { t, cardName, thirds, signed, suitName }) {
  const p = { ...bullet.params };
  for (const k of CARDS) {
    if (typeof p[k] === "string") p[k] = cardName(p[k]);
    else if (k !== "card" && typeof p[k] === "number") p[k] = thirds(p[k]);
  }
  for (const k of THIRDS) if (typeof p[k] === "number") p[k] = thirds(p[k]);
  for (const k of ["skill", "skill2"]) if (typeof p[k] === "string") p[k] = t(`tutor.name.${p[k]}`);
  if (typeof p.honour === "string") p.honour = t(`card.honour.${p.honour}`);
  if (typeof p.suit === "string") p.suit = suitName ? suitName(p.suit) : p.suit;
  if (typeof p.net === "number") p.net = signed(p.net);
  return p;
}

// A review bullet (the count, the sitting's tally) in words.
export function words(bullet, fmt) {
  return fmt.t(`review.say.${bullet.key}`, params(bullet, fmt));
}

// English goes on in lower case after a colon; German keeps the capital of a
// full sentence (and of a noun).
const after = (text, fmt) => (fmt.lang === "en" ? text.charAt(0).toLowerCase() + text.slice(1) : text);

// A brief bullet's lead and text. A rule is told after "If I may:", and after
// any lead that ends in a colon the English text begins in lower case.
export function tutorWords(bullet, fmt) {
  const p = params(bullet, fmt);
  const { t } = fmt;
  const lead = bullet.lead ? t(bullet.lead, p) : "";
  const body = t(bullet.key, p);
  if (bullet.key.startsWith("tutor.rule.")) return { lead, text: `${t("tutor.lead.rule")} ${after(body, fmt)}` };
  return { lead, text: lead.endsWith(":") ? after(body, fmt) : body };
}

// The nudge in the Kibitzer's tab: "If I may: ...".
export const nudgeText = (key, fmt) => `${fmt.t("tutor.lead.rule")} ${after(fmt.t(key), fmt)}`;

// The line under the prompt after a trick (docs/PROTOCOL.md `strohmandeln_explain`):
// the suit and the cards named in the language. `suitName` gives a suit letter's
// name as it stands in the sentence.
export function explainText(bullet, { t, cardName, suitName }) {
  if (!bullet?.key) return "";
  const p = { ...bullet.params };
  if (typeof p.suit === "string") p.suit = suitName(p.suit);
  for (const k of ["winner", "loser"]) if (typeof p[k] === "string") p[k] = cardName(p[k]);
  return t("explain." + bullet.key.slice("explain-".length), p);
}

// { title, header, bullets: [{ lead, text }], how, method } for a brief, or
// null when there is nothing to say. `review` is the module's review of the
// deal (or of the sitting with `whole`), whose first bullet (the count, or the
// sitting's tally) is the header; `brief` is `{ bullets }`. With `tips` off
// (the setting) the brief gives its first bullet only, and only if it is what
// went well: no focus, no advice (docs/TUTOR.md 6).
export function briefBlocks(brief, review, fmt, { whole = false, tips = true } = {}) {
  const { t } = fmt;
  const first = review?.bullets?.[0];
  const header = first?.key ? words(first, fmt) : null;
  let list = (brief?.bullets ?? []).filter((b) => b && b.key);
  if (!tips) list = list.filter((b) => b.lead === "tutor.lead.well").slice(0, 1);
  // at most three bullets of the focus, then "Their style" when the engine says it
  const isStyle = (b) => b.lead === "tutor.lead.style";
  const bullets = [...list.filter((b) => !isStyle(b)).slice(0, 3), ...list.filter(isStyle).slice(0, 1)].map((b) => tutorWords(b, fmt));
  if (!header && !bullets.length) return null;
  return {
    title: t(whole ? "review.sitting" : "review.deal"),
    header,
    bullets,
    // What Random drew: the engine gives `style.drawn` only with the sitting's `reveal` (STYLES 2.3).
    reveal: whole && brief?.style?.drawn ? t("style.reveal", { drawn: t(`style.${brief.style.drawn}.name`) }) : null,
    how: t("tutor.how"),
    method: t(brief?.method ?? "tutor.method"),
  };
}
