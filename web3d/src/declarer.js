// The marks of the two roles on the table (docs/TABLE3D.md 2.5.8): the declarer's
// "(D)" after a name on the score HUD, and the dealer's chip by their hand.
// Pure, from the model and `t`.

// { side: "you" | "opp", text, title } from the bid until the deal ends, or null
// (no bid: a simple game, or a deal not yet bid).
export function declarerMark(model, t) {
  if (!model || model.declarer === null || model.declarer === undefined) return null;
  return { side: model.declarer === model.human ? "you" : "opp", text: t("hud.declarer.mark"), title: t("hud.declarer.tip") };
}

// { text, title } of the dealer's chip, or null before a dealer is known. The word
// is the chip, not a letter: "D" would read as the declarer's.
export function dealerTag(model, t) {
  if (!model || model.dealer === null || model.dealer === undefined) return null;
  return { text: t("hud.dealer"), title: t(model.dealer === model.human ? "hud.dealer.you" : "hud.dealer.opp") };
}
