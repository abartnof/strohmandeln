// The end-of-deal review for its dialog (docs/TABLE3D.md 6.4): the module's
// bullets (docs/PROTOCOL.md "The review": a lead, a key, parameters) put into
// the words of the current language, one to three bullets, each with a bold
// lead, and a folded "How is this worked out?". The page adds no judgement of
// its own: every sentence is a key of docs/MESSAGES.md "Review keys".
//
// Cassino's review.js (briefBlocks) is the pattern.

// Parameters the table formats before they are put into a sentence: cards
// are named, thirds written as points and thirds, the honour named, the net
// of a sitting signed.
export function words(bullet, { t, cardName, thirds, signed }) {
  const p = { ...bullet.params };
  for (const k of ["mine", "theirs"]) {
    if (typeof p[k] === "string") p[k] = cardName(p[k]);
    else if (typeof p[k] === "number") p[k] = thirds(p[k]);
  }
  if (typeof p.thirds === "number") p.thirds = thirds(p.thirds);
  if (typeof p.honour === "string") p.honour = t(`card.honour.${p.honour}`);
  if (typeof p.net === "number") p.net = signed(p.net);
  return t(`review.say.${bullet.key}`, p);
}

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

// { title, bullets: [{ lead, text }], method } for a review, or null when the
// module had nothing to say. `whole` is the sitting's review.
export function reviewBlocks(review, fmt, { whole = false } = {}) {
  const { t } = fmt;
  const bullets = (review?.bullets ?? [])
    .filter((b) => b && b.key)
    .slice(0, 3)
    .map((b) => ({ lead: t(`review.lead.${b.lead}`), text: words(b, fmt) }));
  if (!bullets.length) return null;
  return {
    title: t(whole ? "review.sitting" : "review.deal"),
    bullets,
    how: t("review.how"),
    method: t("review.method"),
  };
}
