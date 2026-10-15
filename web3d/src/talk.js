// What the opponent says: the protocol's events (docs/PROTOCOL.md "Events")
// mapped to the phrase bank's groups (docs/TABLE3D.md section 5.1, docs/PHRASES.md),
// one line picked per moment. The core calls `onEvent(event, ctx)` when an
// event is *seen* (5.2.1), so timing is the caller's; what is kept here is
// what the events alone decide: the quiet count (5.2.3), the modes (5.4), the
// cross-group repeat control (5.3) and the luck talk (5.1.1).
//
//   createTalk({ words, getLang, rng, say }) -> { onEvent, setMode, snapshot, restore }
//     words:   web3d/words.json, { groups: { id: [key] }, texts: { key: { en, de, sig } } }
//     getLang: () => "en" | "de"
//     rng:     () => [0, 1), seeded for a repeatable sitting
//     say:     ({ key, text, who, group, call }) -> void; `call` says whether the group is a call (5.2.2); `who` is "them" (every line of the bank
//              is the opponent's, the -yours groups being its remarks on the human's moment)
//   onEvent(event, ctx): `event` is a protocol event, `ctx.human` the human's seat.
//     Besides the protocol's, the table sends these moments (they are not game events):
//       { type: "count_start" }  the last trick is gathered: says count-start, then the
//                                table is quiet until
//       { type: "count_close" }  the last popup is gone: says count-close if the count was close
//       { type: "slow", n }      the human is idle (n = 1 at 25 s, 2 at 60 s)
//       { type: "refused" }      a card was refused (slip-yours)
//       { type: "opponent_error" } the opponent's own error (blunder-own)
//       { type: "kibitz" }       the tutor is first opened: a line of `kibitz`, `who: "kibitz"`
//       { type: "kibitz_mute" }  the tutor is muted: a line of `kibitz-mute`, `who: "kibitz"`
//                                (both whatever the mode: the Kibitzer is not table talk)
//   setMode("all" | "calls" | "none")
//   snapshot() / restore(s): the sitting-wide record of sayings heard, to save with the sitting.

import { createBags } from "../../kit/js/bag.js";

// The calls (5.2.2): lines that carry the game. Everything else is chatter.
export const CALLS = new Set([
  "greet", "touren-3", "touren-6", "cut", "deal-mine", "deal-yours", "void-deal", "doubling-applied",
  "bid-take-mine", "bid-take-yours", "bid-pass-mine", "bid-pass-yours", "both-pass", "thrown-in", "reveal-begin",
  "trull-mine", "trull-yours", "kings-mine", "kings-yours", "ultimo-won-mine", "ultimo-won-yours",
  "ultimo-lost-mine", "ultimo-lost-yours", "valat-mine", "valat-yours",
  "result-won-mine", "result-won-yours", "result-lost-mine", "result-lost-yours", "draw",
  "tour-end", "tour-last", "extend-ask", "extend-yes", "extend-no",
  "settle-opp-pays", "settle-player-pays", "settle-even", "farewell",
]);

export const MODES = ["all", "calls", "none"];
// A saying heard in the last WINDOW deals (this one included) is passed over (5.3.2).
export const WINDOW = 3;
// A wide win, in thirds (5.1.1); a lopsided first reveal, in cards.
export const WIDE = 120;
export const RUN = 3;
export const LUCKY_PULLS = 6;
// A trick worth remarking by its cards' points (K 5, Q 4, C 3, J 2, the honours 5, the rest 1).
export const RICH = 9;

const isKing = (c) => /^K[hdsc]$/.test(c);
const isTarock = (c) => /^T\d+$/.test(c) || c === "SK";
const points = (c) => (isTarock(c) ? (c === "T1" || c === "T21" || c === "SK" ? 5 : 1) : { K: 5, Q: 4, C: 3, J: 2 }[c[0]] ?? 1);

export function createTalk({ words, getLang, rng = Math.random, say }) {
  const pick = createBags(rng);
  let mode = "all";
  let human = 1;
  let deal = 0;
  let touren = 3;
  let quiet = false;
  const spoken = new Map(); // sig -> [the deal it was last heard in, its order among all heard]
  let seq = 0;

  // Per-deal state.
  let nothingSaid;
  let reveal; // { seat, covers, pulls }
  let chain; // { seat, packet, n }
  let played; // seat -> the card played in the trick in progress
  let firstPulls; // seat -> pulls before the first trick
  let firstTrickDone;
  let luckSaid;
  let lastCount = null;
  const newDeal = () => {
    nothingSaid = { mine: false, yours: false };
    reveal = null;
    chain = null;
    played = new Map();
    firstPulls = new Map();
    firstTrickDone = false;
    luckSaid = false;
    lastCount = null;
    quiet = false;
  };
  newDeal();

  const side = (seat) => (seat === human ? "yours" : "mine");
  const heard = (sig) => spoken.has(sig) && deal - spoken.get(sig)[0] < WINDOW;

  // The key to say from a group: the bag's choice among the lines whose saying
  // has not been heard lately (calls are exempt); if every line is spent, the
  // one heard longest ago.
  function choose(group) {
    const keys = words.groups[group];
    if (!keys || !keys.length) return null;
    if (CALLS.has(group)) return pick(group, keys);
    const sigOf = (k) => words.texts[k].sig ?? k;
    const free = keys.filter((k) => !heard(sigOf(k)));
    if (!free.length) return keys.reduce((a, b) => ((spoken.get(sigOf(b))?.[1] ?? -1) < (spoken.get(sigOf(a))?.[1] ?? -1) ? b : a));
    for (let i = 0; i < keys.length * 2; i++) {
      const k = pick(group, keys);
      if (free.includes(k)) return k;
    }
    return free[Math.floor(rng() * free.length)];
  }

  // Say one line of a group, if the mode and the quiet allow. Returns whether it was said.
  function speak(group) {
    if (mode === "none" || quiet) return false;
    if (mode === "calls" && !CALLS.has(group)) return false;
    const key = choose(group);
    if (!key) return false;
    const t = words.texts[key];
    spoken.set(t.sig ?? key, [deal, ++seq]);
    say({ key, text: t[getLang()] ?? t.en, who: "them", group, call: CALLS.has(group) });
    return true;
  }

  // The Kibitzer's line (docs/TABLE3D.md 5.1, 6.5): his own voice, said whatever
  // the table-talk mode and the quiet of the count, as `who: "kibitz"`. The
  // table shows it in his tab, never in the opponent's box.
  function kibitz(group) {
    const keys = words.groups[group];
    if (!keys || !keys.length) return false;
    const key = pick(group, keys);
    const t = words.texts[key];
    say({ key, text: t[getLang()] ?? t.en, who: "kibitz" });
    return true;
  }

  function pulledCard(e) {
    firstPulls.set(e.seat, (firstPulls.get(e.seat) ?? 0) + (firstTrickDone ? 0 : 1));
    if (reveal && reveal.seat === e.seat) reveal.pulls += 1;
    chain = chain && chain.seat === e.seat && chain.packet === e.packet ? { ...chain, n: chain.n + 1 } : { seat: e.seat, packet: e.packet, n: 1 };
    // The run replaces the line of the pull that makes it (the table cannot
    // know which pull is a chain's last); later pulls of the chain are silent.
    if (chain.n === RUN) return speak(`run-${side(e.seat)}`);
    if (chain.n > RUN) return false;
    const s = side(e.seat);
    if (e.card === "SK") return speak("pull-sku");
    if (e.card === "T21") return speak("pull-mond");
    if (e.card === "T1") return speak("pull-pagat");
    if (isKing(e.card)) return speak(`pull-king-${s}`);
    if (isTarock(e.card)) return speak(`pull-tarock-${s}`);
    return false;
  }

  function trickWon(e) {
    firstTrickDone = true;
    chain = null;
    reveal = null;
    const [a, b] = e.cards;
    const mine = played.get(e.winner);
    const won = mine && e.cards.includes(mine) ? mine : a;
    const lost = won === a ? b : a;
    played = new Map();
    if (won === "SK" && lost === "T21") return speak(`mond-caught-${side(e.winner)}`);
    if (e.cards.some((c) => c === "SK" || c === "T21")) return false;
    const loser = 1 - e.winner;
    const wonBig = isTarock(won) || isKing(won) || points(won) + points(lost) >= RICH;
    const lostBig = isTarock(lost) || isKing(lost);
    if (wonBig && lostBig) return speak(rng() < 0.5 ? `trick-good-${side(e.winner)}` : `trick-bad-${side(loser)}`);
    if (wonBig) return speak(`trick-good-${side(e.winner)}`);
    if (lostBig) return speak(`trick-bad-${side(loser)}`);
    return false;
  }

  function scoredGame(e) {
    if (e.result === "draw") return speak("draw");
    const winner = e.credits?.[0]?.seat;
    if (winner === undefined) return false;
    const us = winner !== human; // the opponent won (us: its side)
    // Two groups tell each outcome, one from each speaker's side; either will do.
    const groups = us ? ["result-won-mine", "result-lost-yours"] : ["result-lost-mine", "result-won-yours"];
    speak(groups[rng() < 0.5 ? 0 : 1]);
    // Luck talk (5.1.1): once a deal, after the result.
    if (luckSaid) return true;
    const take = lastCount?.thirds?.[winner] ?? 0;
    const lucky = [...firstPulls].find(([seat, n]) => n >= LUCKY_PULLS && (firstPulls.get(1 - seat) ?? 0) === 0)?.[0];
    let luck = null;
    if (take >= WIDE) luck = us ? "luck-mine" : rng() < 0.5 ? "luck-yours" : "luck-bad-mine";
    else if (lucky !== undefined) luck = lucky === human ? "luck-yours" : "luck-mine";
    if (luck) {
      luckSaid = true;
      speak(luck);
    }
    return true;
  }

  function onEvent(e, ctx = {}) {
    if (ctx.human !== undefined) human = ctx.human;
    switch (e.type) {
      case "sitting_start":
        touren = e.touren ?? touren;
        deal = 0;
        speak("greet");
        speak(touren === 3 ? "touren-3" : "touren-6");
        speak("cut");
        break;
      case "deal_start":
        deal = e.deal_number ?? deal + 1;
        newDeal();
        speak(`deal-${e.dealer === human ? "yours" : "mine"}`);
        break;
      case "void_deal":
        speak("void-deal");
        break;
      case "doubling_applied":
        speak("doubling-applied");
        break;
      case "bid":
        speak(`bid-take-${side(e.seat)}`);
        break;
      case "pass":
        speak(`bid-pass-${side(e.seat)}`);
        break;
      case "both_pass":
        if (e.mode === "play") speak("both-pass");
        break;
      case "thrown_in":
        speak("thrown-in");
        break;
      case "reveal_start":
        reveal = { seat: e.seat, covers: 0, pulls: 0 };
        chain = null;
        speak("reveal-begin");
        break;
      case "card_pulled":
        pulledCard(e);
        break;
      case "cover_set":
        if (reveal && reveal.seat === e.seat && ++reveal.covers === 3 && reveal.pulls === 0 && !nothingSaid[side(e.seat)]) {
          if (speak(`nothing-${side(e.seat)}`)) nothingSaid[side(e.seat)] = true;
        }
        break;
      case "card_played":
        played.set(e.seat, e.card);
        if (e.card === "SK") speak("sku-played");
        break;
      case "trick_won":
        trickWon(e);
        break;
      case "pagat_ultimo":
        speak(`ultimo-${e.result === "won" ? "won" : "lost"}-${side(e.seat)}`);
        break;
      case "announcement":
        speak(`${e.honour === "trull" ? "trull" : "kings"}-${side(e.seat)}`);
        break;
      case "honour_forfeited":
        speak("honour-forfeited");
        break;
      case "valat":
        speak(`valat-${side(e.seat)}`);
        break;
      case "count":
        lastCount = e;
        break;
      case "count_start":
        speak("count-start");
        quiet = true;
        break;
      case "count_close": {
        quiet = false;
        const d = lastCount;
        // The lines name the figure (35 and two thirds), so only the exact threshold is close.
        if (d && d.thirds?.[d.declarer] === d.threshold) speak("count-close");
        break;
      }
      case "game_scored":
        scoredGame(e);
        break;
      case "doubling_lapsed":
        speak("doubling-lapsed");
        break;
      case "tour_end":
        speak(e.tour >= touren ? "tour-last" : "tour-end");
        break;
      case "extension_offered":
        speak("extend-ask");
        break;
      case "extension_decided":
        if (e.accepted) touren += e.touren_added ?? 0;
        speak(e.accepted ? "extend-yes" : "extend-no");
        break;
      case "sitting_end":
        speak(e.payer === null || e.payer === undefined ? "settle-even" : e.payer === human ? "settle-player-pays" : "settle-opp-pays");
        speak("farewell");
        break;
      case "slow":
        speak("slow");
        break;
      case "refused":
        speak("slip-yours");
        break;
      case "opponent_error":
        speak("blunder-own");
        break;
      case "kibitz":
        kibitz("kibitz");
        break;
      case "kibitz_mute":
        kibitz("kibitz-mute");
        break;
      default:
        break;
    }
  }

  return {
    onEvent,
    setMode(m) {
      if (!MODES.includes(m)) throw new Error(`unknown talk mode: ${m}`);
      mode = m;
    },
    snapshot: () => ({ deal, touren, seq, spoken: [...spoken] }),
    restore(s) {
      deal = s.deal ?? 0;
      touren = s.touren ?? touren;
      seq = s.seq ?? 0;
      spoken.clear();
      for (const [k, v] of s.spoken ?? []) spoken.set(k, v);
    },
  };
}
