// What the screen reader is told (docs/TABLE3D.md 3.10.2): one sentence an
// event, in the current language, from the `sr.<event>[.you|.opp]` keys of
// strings.json. Nothing here is a rule.
import { cardName, t, thirds } from "./strings.js";

const side = (seat, human) => (seat === human ? "you" : "opp");

// The sentence for an event, or null (an event nothing is said of). `model`
// is the model after the event.
export function sentence(e, model) {
  const h = model.human;
  const s = (seat) => side(seat, h);
  switch (e.type) {
    case "sitting_start":
      return t("sr.sitting_start", { touren: e.touren });
    case "deal_start":
      return t("sr.deal_start", { n: e.deal_number, tour: e.tour, dealer: t(`sr.dealer.${s(e.dealer)}`) });
    case "dealt":
      return t("sr.dealt", { n: e.cards.length, cards: e.cards.map(cardName).join(", ") });
    case "void_deal":
      return t("sr.void_deal");
    case "bid":
      return t(`sr.bid.${s(e.seat)}`);
    case "pass":
      return t(`sr.pass.${s(e.seat)}`);
    case "both_pass":
      return t(`sr.both_pass.${e.mode}`);
    case "thrown_in":
      return t("sr.thrown_in");
    case "reveal_start":
      return t(`sr.reveal_start.${s(e.seat)}`);
    case "packet_turned":
      return t(`sr.packet_turned.${s(e.seat)}`, { packet: e.packet, card: cardName(e.card) });
    case "card_pulled":
      return t(`sr.card_pulled.${s(e.seat)}`, { packet: e.packet, card: cardName(e.card) });
    case "cover_set":
      return t(`sr.cover_set.${s(e.seat)}`, { packet: e.packet, card: cardName(e.card) });
    case "packet_empty":
      return t(`sr.packet_empty.${s(e.seat)}`, { packet: e.packet });
    case "announcement":
      return t(`sr.announcement.${s(e.seat)}`, { honour: t(`card.honour.${e.honour}`) });
    case "honour_forfeited":
      return t(`sr.honour_forfeited.${s(e.seat)}`, { honour: t(`card.honour.${e.honour}`) });
    case "card_played":
      return t(`sr.card_played.${s(e.seat)}`, { card: cardName(e.card) });
    case "trick_won":
      return t(`sr.trick_won.${s(e.winner)}`);
    case "pagat_ultimo":
      return t(`sr.pagat_ultimo.${s(e.seat)}`, { result: t(`sr.pagat_ultimo.${e.result}`) });
    case "valat":
      return t(`sr.valat.${s(e.seat)}`);
    case "count":
      return t("sr.count", { you: thirds(e.thirds[h]), opp: thirds(e.thirds[1 - h]) });
    case "game_scored":
      return t("sr.game_scored", { result: t(`sr.game_scored.${e.result}`) });
    case "doubling_set":
      return t("sr.doubling_set");
    case "doubling_applied":
      return t("sr.doubling_applied");
    case "doubling_lapsed":
      return t("sr.doubling_lapsed");
    case "tour_end":
      return t("sr.tour_end", { tour: e.tour, balance: e.balance * (h === 0 ? 1 : -1) });
    case "extension_offered":
      return t("sr.extension_offered");
    case "extension_decided":
      return t("sr.extension_decided", { answer: t(`sr.extension_decided.${e.accepted ? "yes" : "no"}`) });
    case "sitting_end":
      return t("sr.sitting_end", { settlement: e.payer === null ? t("slate.square") : t(`slate.payer.${s(e.payer)}`) + " " + e.kreuzer + " kr." });
    default:
      return null;
  }
}

// "Where am I": the trick, whose turn, the counts so far.
export function where(wasm) {
  const d = wasm.deal;
  const h = wasm.human;
  return t("sr.where", {
    trick: d?.trick_number ?? 0,
    turn: t(wasm.prompt === "you" ? "sr.turn.you" : "sr.turn.opp"),
    you: thirds(d?.take_thirds?.[h] ?? 0),
    opp: thirds(d?.take_thirds?.[1 - h] ?? 0),
  });
}
