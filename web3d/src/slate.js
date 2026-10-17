// The slate (docs/TABLE3D.md 4): the running account of a sitting, written as
// on the Tafel of the period: a row per deal with its credits, a rule per
// Tour with the Contos, the balance at the foot and, at the end, the
// settlement in kreuzer ("Quitt." when square).
//
// Three layers, so the logic is testable without a DOM:
//   createLedger()  folds protocol events (docs/PROTOCOL.md) into a record;
//   slateModel()    projects the record for the human's seat;
//   createSlate()   draws the model under a root element.
// The slate is a projection of the events; it adds nothing to the rules
// (docs/RULES.md 15.9). It is a reconstruction (4.6): its tooltip says so.
import { h } from "./dom.js";
import { kreuzer, signed, onLang as sharedOnLang } from "./strings.js";

const VERSION = 1;
const KREUZER_PER_POINT = 2; // docs/RULES.md 15.8

// ---- the record ------------------------------------------------------------

// A saved record is from storage or a file: trusted only if its shape is.
const isNum = (x) => Number.isInteger(x);
const isPair = (x) => Array.isArray(x) && x.length === 2 && x.every(isNum);
const validRow = (r) =>
  Boolean(r) && isNum(r.number) && isNum(r.tour) && (r.dealer === 0 || r.dealer === 1) && isNum(r.multiplier) && typeof r.status === "string" && Array.isArray(r.credits) && r.credits.every((c) => c && (c.seat === 0 || c.seat === 1) && isNum(c.points) && typeof c.cause === "string") && isPair(r.contos);
function validRecord(rec) {
  return (
    Boolean(rec) && rec.v === VERSION && isNum(rec.touren) && isNum(rec.lastTour) && isPair(rec.contos) && Array.isArray(rec.rows) && rec.rows.every(validRow) &&
    Boolean(rec.tourEnds) && typeof rec.tourEnds === "object" && Object.values(rec.tourEnds).every((e) => e && isPair(e.contos)) &&
    Array.isArray(rec.extensions) && rec.extensions.every(isNum) &&
    (rec.settlement === null || (Boolean(rec.settlement) && isPair(rec.settlement.contos) && isNum(rec.settlement.balance) && isNum(rec.settlement.kreuzer)))
  );
}

export function createLedger(saved) {
  const blank = () => ({ v: VERSION, touren: 0, settings: null, firstDealer: null, rows: [], tourEnds: {}, extensions: [], contos: [0, 0], lastTour: 0, settlement: null });
  let rec = validRecord(saved) ? JSON.parse(JSON.stringify(saved)) : blank();

  const live = () => {
    const last = rec.rows[rec.rows.length - 1];
    return last && last.status === "live" ? last : null;
  };

  function apply(e) {
    switch (e.type) {
      case "sitting_start":
        rec = blank();
        rec.touren = e.touren;
        rec.settings = e.settings;
        rec.firstDealer = e.first_dealer;
        break;
      case "deal_start": {
        // A deal begun again after a void deal replaces the live row; the same
        // deal fed twice (a reload replays the deal in progress) is one row.
        if (live() || rec.rows[rec.rows.length - 1]?.number === e.deal_number) rec.rows.pop();
        rec.rows.push({ number: e.deal_number, tour: e.tour, dealer: e.dealer, multiplier: e.multiplier, status: "live", result: null, credits: [], contos: [...rec.contos] });
        break;
      }
      case "void_deal":
        if (live()) rec.rows.pop();
        break;
      case "thrown_in": {
        const row = live();
        if (row) Object.assign(row, { status: "thrown_in", contos: [...rec.contos] });
        break;
      }
      case "game_scored": {
        const row = live();
        rec.contos = [...e.contos];
        if (row) Object.assign(row, { status: e.result === "draw" ? "draw" : "played", result: e.result, multiplier: e.multiplier, credits: e.credits.map((c) => ({ ...c })), contos: [...e.contos] });
        break;
      }
      case "tour_end":
        rec.tourEnds[e.tour] = { contos: [...e.contos] };
        rec.contos = [...e.contos];
        rec.lastTour = e.tour;
        break;
      case "extension_decided":
        if (e.accepted && !rec.extensions.includes(rec.lastTour)) {
          rec.touren += e.touren_added;
          rec.extensions.push(rec.lastTour);
        }
        break;
      case "sitting_end":
        rec.contos = [...e.contos];
        rec.settlement = { contos: [...e.contos], balance: e.balance, payer: e.payer, kreuzer: e.kreuzer };
        break;
      default:
    }
  }

  return {
    // Replace the record by a saved one (a snapshot), if it is whole.
    load(snapshot) {
      if (validRecord(snapshot)) rec = JSON.parse(JSON.stringify(snapshot));
    },
    ingest(events) {
      for (const e of events ?? []) apply(e);
    },
    // The current Contos, by seat, as the state gives them.
    setContos(contos) {
      if (Array.isArray(contos) && contos.length === 2) rec.contos = [...contos];
    },
    snapshot: () => JSON.parse(JSON.stringify(rec)),
  };
}

// ---- the projection --------------------------------------------------------

// The record for the human at seat `human`: `you` and `opp` columns.
export function slateModel(rec, human) {
  const other = 1 - human;
  const side = (row, seat) => {
    const credits = row.credits.filter((c) => c.seat === seat).map((c) => ({ cause: c.cause, points: c.points }));
    return { credits, total: credits.reduce((s, c) => s + c.points, 0) };
  };
  const closers = (tour) => {
    const out = [];
    const end = rec.tourEnds[tour];
    if (end) out.push({ type: "tour", tour, youConto: end.contos[human], oppConto: end.contos[other] });
    if (rec.extensions.includes(tour)) out.push({ type: "extended", tour });
    return out;
  };

  const blocks = [];
  let current = null;
  for (const row of rec.rows) {
    if (current !== null && row.tour !== current) blocks.push(...closers(current));
    current = row.tour;
    blocks.push({
      type: "deal",
      number: row.number,
      tour: row.tour,
      dealerIsYou: row.dealer === human,
      multiplier: row.multiplier,
      status: row.status,
      result: row.result,
      you: side(row, human),
      opp: side(row, other),
      youConto: row.status === "live" ? rec.contos[human] : row.contos[human],
      oppConto: row.status === "live" ? rec.contos[other] : row.contos[other],
    });
  }
  if (current !== null) blocks.push(...closers(current));

  const balance = rec.contos[human] - rec.contos[other];
  const s = rec.settlement;
  return {
    header: { touren: rec.touren, settings: rec.settings, youDealtFirst: rec.firstDealer === human },
    blocks,
    foot: { youConto: rec.contos[human], oppConto: rec.contos[other], balance, kreuzer: Math.abs(balance) * KREUZER_PER_POINT, direction: balance > 0 ? "receive" : balance < 0 ? "pay" : "square" },
    settlement: s
      ? { youConto: s.contos[human], oppConto: s.contos[other], difference: Math.abs(s.balance), kreuzer: s.kreuzer, payer: s.payer === null ? null : s.payer === human ? "you" : "opp" }
      : null,
  };
}

// ---- the board -------------------------------------------------------------

// root: the element the slate lives in (hidden until opened). t, tn: the
// strings. later(ms, fn) / cancel(id): the table's clock, for the sheet that
// folds itself. onLang(fn): redraw when the language switches.
export function createSlate(root, { t, tn, onLang = sharedOnLang, later = (ms, fn) => setTimeout(fn, ms), cancel = (id) => clearTimeout(id) }) {
  const plural = tn ?? ((key, n, params) => t(`${key}.${n === 1 ? "one" : "other"}`, { n, ...params }));
  const doc = root.ownerDocument;
  const ledger = createLedger();
  let human = 0;
  let timer = null;

  root.classList.add("slate");
  root.hidden = true;
  root.setAttribute("aria-hidden", "true");
  root.setAttribute("role", "dialog");

  const pts = (n) => t("slate.pts-kr", { pts: t("slate.pts", { n }), kr: kreuzer(n * KREUZER_PER_POINT) });
  const cell = (side, conto) =>
    h(doc, "div", { class: "slate-cell" },
      h(doc, "div", { class: "slate-credits" }, side.credits.map((c) => h(doc, "span", { class: "slate-credit" }, t("slate.entry", { cause: t(`slate.cause.${c.cause}`), n: c.points })))),
      side.total > 0 ? h(doc, "div", { class: "slate-total" }, pts(side.total)) : null,
      conto === null ? null : h(doc, "div", { class: "slate-conto" }, String(conto)));

  function dealRow(b) {
    const note = b.status === "draw" ? t("slate.draw") : b.status === "thrown_in" ? t("slate.thrown-in") : b.status === "live" ? t("slate.live") : null;
    return h(doc, "div", { class: `slate-row slate-${b.status}`, "data-deal": b.number },
      h(doc, "div", { class: "slate-margin" },
        h(doc, "span", { class: "slate-no", title: t("slate.deal", { n: b.number }) }, String(b.number)),
        h(doc, "span", { class: "slate-dealer", title: t("slate.dealer.title"), "data-you": b.dealerIsYou ? "1" : "0" }, t("slate.dealer")),
        b.multiplier === 2 ? h(doc, "span", { class: "slate-double" }, t("slate.double")) : null,
        note ? h(doc, "span", { class: "slate-note" }, note) : null),
      cell(b.you, b.youConto),
      cell(b.opp, b.oppConto));
  }

  function block(b) {
    if (b.type === "deal") return dealRow(b);
    if (b.type === "tour")
      return h(doc, "div", { class: "slate-rule slate-tour" },
        h(doc, "span", { class: "slate-tour-no" }, t("slate.tour", { n: b.tour })),
        h(doc, "span", { class: "slate-conto" }, String(b.youConto)),
        h(doc, "span", { class: "slate-conto" }, String(b.oppConto)));
    return h(doc, "div", { class: "slate-rule slate-extended" }, t("slate.extended"));
  }

  function settlement(s) {
    const figures = [
      h(doc, "div", { class: "slate-line" }, h(doc, "span", {}, t("slate.conto")), h(doc, "span", { class: "slate-conto" }, String(s.youConto)), h(doc, "span", { class: "slate-conto" }, String(s.oppConto))),
      h(doc, "div", { class: "slate-line" }, h(doc, "span", {}, t("slate.difference")), h(doc, "span", {}, t("slate.pts", { n: s.difference }))),
    ];
    if (s.payer === null) figures.push(h(doc, "div", { class: "slate-line slate-square" }, t("slate.square")));
    else figures.push(h(doc, "div", { class: "slate-line slate-owed" }, t("slate.owed", { kr: kreuzer(s.kreuzer) })), h(doc, "div", { class: "slate-line slate-payer" }, t(`slate.payer.${s.payer}`)));
    figures.forEach((f, i) => {
      f.classList.add("slate-step");
      f.setAttribute("style", `--step:${i}`);
    });
    return h(doc, "div", { class: "slate-settlement" }, h(doc, "h3", {}, t("slate.settlement")), figures);
  }

  function render() {
    const m = slateModel(ledger.snapshot(), human);
    const rules = m.header.settings ? t("slate.rules", { values: m.header.settings.values, ultimo: m.header.settings.ultimo, both_pass: t(`slate.both_pass.${m.header.settings.both_pass}`) }) : "";
    const foot = m.foot;
    root.replaceChildren(
      h(doc, "div", { class: "slate-board", title: t("slate.tooltip") },
        h(doc, "div", { class: "slate-head" },
          h(doc, "h2", {}, t("slate.title")),
          h(doc, "button", { class: "slate-close", type: "button", "aria-label": t("slate.close"), onclick: () => close() }, "×")),
        m.header.touren ? h(doc, "div", { class: "slate-rules" }, t("slate.header", { touren: plural("slate.touren", m.header.touren), rules })) : null,
        h(doc, "div", { class: "slate-cols" },
          h(doc, "span", { class: "slate-col-margin" }),
          h(doc, "span", { class: "slate-col slate-col-you" }, t("slate.you"), m.header.youDealtFirst ? h(doc, "small", { class: "slate-first" }, ` ${t("slate.dealer1")}`) : null),
          h(doc, "span", { class: "slate-col slate-col-opp" }, t("slate.opp"), m.header.touren && !m.header.youDealtFirst ? h(doc, "small", { class: "slate-first" }, ` ${t("slate.dealer1")}`) : null)),
        m.blocks.length ? h(doc, "div", { class: "slate-rows" }, m.blocks.map(block)) : h(doc, "div", { class: "slate-empty" }, t("slate.empty")),
        h(doc, "div", { class: "slate-foot" },
          h(doc, "span", { class: "slate-balance" }, t("slate.balance", { n: signed(foot.balance), kr: kreuzer(foot.kreuzer) })),
          h(doc, "span", { class: "slate-direction" }, foot.direction === "square" ? t("slate.square") : t(`slate.${foot.direction}`))),
        m.settlement ? settlement(m.settlement) : null));
  }

  function open({ ms } = {}) {
    if (timer !== null) cancel(timer);
    timer = null;
    root.hidden = false;
    root.setAttribute("aria-hidden", "false");
    if (ms) timer = later(ms, () => { timer = null; close(); });
  }
  function close() {
    if (timer !== null) cancel(timer);
    timer = null;
    root.hidden = true;
    root.setAttribute("aria-hidden", "true");
    root.classList.remove("slate-large");
  }

  // A tap on the sheet while it is folding itself keeps it open; a tap on the
  // backdrop folds it. (Escape is the page's: overlays.js.)
  root.addEventListener("click", (ev) => {
    if (timer !== null) {
      cancel(timer);
      timer = null;
    } else if (ev.target === root) close();
  });
  onLang(render);
  render();

  return {
    open,
    close,
    // The events since the last call, and the state they came with.
    update(state, events) {
      if (state && (state.human === 0 || state.human === 1)) human = state.human;
      ledger.ingest(events);
      if (state) ledger.setContos(state.contos);
      render();
    },
    // The sitting's end: the account is settled and the slate opens large.
    settle(sittingEnd) {
      ledger.ingest([sittingEnd]);
      render();
      root.classList.add("slate-large");
      open();
    },
    // For the page to keep with the sitting (9.1) and to restore on a reload,
    // since the engine's state holds only the deal in progress.
    snapshot: () => ({ ledger: ledger.snapshot(), human }),
    restore(saved) {
      if (!saved || typeof saved !== "object") return;
      ledger.load(saved.ledger);
      if (saved.human === 0 || saved.human === 1) human = saved.human;
      render();
    },
  };
}
