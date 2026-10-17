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

// The kinds of tooltip, each a `slate.tip.<kind>` template in the strings
// (docs/TABLE3D.md 4.7); the tests check the table has every one in en and de.
const BY = ["you", "opp"];
export const TIP_KINDS = Object.freeze([
  "heading.you", "heading.opp", "dealer1", "header.touren", "header.rules",
  "deal.no", "dealer.you", "dealer.opp", "double", "double.thrown", "note.draw", "note.thrown_in", "note.live",
  ...["game.declarer_win", "game.declarer_loss", "game.simple_win", "trull", "kings", "ultimo_won", "ultimo_lost", "valat", "total", "conto", "tour"].flatMap((k) => BY.map((w) => `${k}.${w}`)),
  "tour.no", "extended", "balance.ahead", "balance.behind", "balance.level", "direction.receive", "direction.pay", "direction.square",
  "settle.title", "settle.account", "settle.you", "settle.opp", "settle.difference", "settle.owed", "settle.payer.you", "settle.payer.opp", "settle.square",
]);

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
    const before = rec.tourEnds[tour - 1]?.contos ?? [0, 0];
    if (end) out.push({ type: "tour", tour, youConto: end.contos[human], oppConto: end.contos[other], youBefore: before[human], oppBefore: before[other] });
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

  // ---- the tooltips (docs/TABLE3D.md 4.7) -----------------------------------
  // Each figure, heading and mark is a `slate-tip`: a title (hover), the same
  // text as its description (screen readers) and, on touch and keyboard, the
  // caption at the foot of the sheet. One tab stop for all of them: the arrows
  // move between them (left and right by figure, up and down by line).
  let line = 0;
  let rover = null; // the tip that holds the tab stop
  let tipEls = [];
  const P = (n) => plural("slate.tip.pt", n);
  const x2 = (points, multiplier) => (multiplier === 2 ? t("slate.tip.doubled", { base: points / 2 }) : "");
  const tip = (cls, kind, params = {}) => ({
    class: `${cls} slate-tip`,
    role: "note",
    tabindex: "-1",
    title: t(`slate.tip.${kind}`, params),
    "aria-description": t(`slate.tip.${kind}`, params),
    "data-tip": kind,
    "data-line": line,
  });
  const youOpp = (mine) => (mine ? "you" : "opp");

  // The credit's tooltip kind: game by the deal's result (docs/RULES.md 9.1,
  // 9.2), the others by cause (10, 11, 12), each for whose credit it is.
  const creditKind = (c, result, mine) => (c.cause === "game" ? `game.${result === "declarer_win" || result === "declarer_loss" ? result : "simple_win"}.${youOpp(mine)}` : `${c.cause}.${youOpp(mine)}`);

  const cell = (side, conto, b, mine) =>
    h(doc, "div", { class: "slate-cell" },
      h(doc, "div", { class: "slate-credits" }, side.credits.map((c) => h(doc, "span", tip("slate-credit", creditKind(c, b.result, mine), { pts: P(c.points), x2: x2(c.points, b.multiplier) }), t("slate.entry", { cause: t(`slate.cause.${c.cause}`), n: c.points })))),
      side.total > 0 ? h(doc, "div", tip("slate-total", `total.${youOpp(mine)}`, { pts: P(side.total), kr: kreuzer(side.total * KREUZER_PER_POINT) }), pts(side.total)) : null,
      conto === null ? null : h(doc, "div", tip("slate-conto", `conto.${youOpp(mine)}`, { deal: b.number, pts: P(conto) }), String(conto)));

  function dealRow(b) {
    line += 1;
    const noteKind = b.status === "draw" ? "note.draw" : b.status === "thrown_in" ? "note.thrown_in" : b.status === "live" ? "note.live" : null;
    const note = b.status === "draw" ? t("slate.draw") : b.status === "thrown_in" ? t("slate.thrown-in") : b.status === "live" ? t("slate.live") : null;
    return h(doc, "div", { class: `slate-row slate-${b.status}`, "data-deal": b.number },
      h(doc, "div", { class: "slate-margin" },
        h(doc, "span", tip("slate-no", "deal.no", { deal: b.number, tour: b.tour }), String(b.number)),
        h(doc, "span", { ...tip("slate-dealer", `dealer.${youOpp(b.dealerIsYou)}`), "data-you": b.dealerIsYou ? "1" : "0" }, t("slate.dealer")),
        b.multiplier === 2 ? h(doc, "span", tip("slate-double", b.status === "thrown_in" ? "double.thrown" : "double"), t("slate.double")) : null,
        note ? h(doc, "span", tip("slate-note", noteKind), note) : null),
      cell(b.you, b.youConto, b, true),
      cell(b.opp, b.oppConto, b, false));
  }

  function block(b) {
    if (b.type === "deal") return dealRow(b);
    line += 1;
    if (b.type === "tour")
      return h(doc, "div", { class: "slate-rule slate-tour" },
        h(doc, "span", tip("slate-tour-no", "tour.no", { tour: b.tour }), t("slate.tour", { n: b.tour })),
        h(doc, "span", tip("slate-conto", "tour.you", { tour: b.tour, total: P(b.youConto), gain: b.youConto - b.youBefore }), String(b.youConto)),
        h(doc, "span", tip("slate-conto", "tour.opp", { tour: b.tour, total: P(b.oppConto), gain: b.oppConto - b.oppBefore }), String(b.oppConto)));
    return h(doc, "div", tip("slate-rule slate-extended", "extended"), t("slate.extended"));
  }

  function settlement(s) {
    const kr = kreuzer(s.kreuzer);
    const figures = [
      () => h(doc, "div", { class: "slate-line" }, h(doc, "span", tip("slate-label", "settle.account"), t("slate.conto")), h(doc, "span", tip("slate-conto", "settle.you", { pts: P(s.youConto) }), String(s.youConto)), h(doc, "span", tip("slate-conto", "settle.opp", { pts: P(s.oppConto) }), String(s.oppConto))),
      () => h(doc, "div", { class: "slate-line" }, h(doc, "span", tip("slate-label", "settle.difference", { pts: P(s.difference) }), t("slate.difference")), h(doc, "span", tip("slate-figure", "settle.difference", { pts: P(s.difference) }), t("slate.pts", { n: s.difference }))),
    ];
    if (s.payer === null) figures.push(() => h(doc, "div", { class: "slate-line slate-square" }, h(doc, "span", tip("slate-label", "settle.square"), t("slate.square"))));
    else
      figures.push(
        () => h(doc, "div", { class: "slate-line slate-owed" }, h(doc, "span", tip("slate-label", "settle.owed", { pts: P(s.difference), kr }), t("slate.owed", { kr }))),
        () => h(doc, "div", { class: "slate-line slate-payer" }, h(doc, "span", tip("slate-label", `settle.payer.${s.payer}`, { pts: P(s.difference) }), t(`slate.payer.${s.payer}`))));
    const lines = figures.map((make, i) => {
      line += 1;
      const f = make();
      f.classList.add("slate-step");
      f.setAttribute("style", `--step:${i}`);
      return f;
    });
    line += 1;
    return h(doc, "div", { class: "slate-settlement" }, h(doc, "h3", tip("slate-settle-title", "settle.title"), t("slate.settlement")), lines);
  }

  // The tips in the order they stand on the sheet.
  function collect(el, out = []) {
    for (const c of el.children ?? []) {
      if (typeof c.getAttribute !== "function") continue;
      if (c.getAttribute("data-tip")) out.push(c);
      collect(c, out);
    }
    return out;
  }
  const lineOf = (el) => Number(el.getAttribute("data-line"));
  function setRover(el, focus = false) {
    if (rover && rover !== el) rover.setAttribute("tabindex", "-1");
    rover = el;
    el.setAttribute("tabindex", "0");
    if (focus) el.focus();
  }
  const tipOf = (target) => {
    const el = target?.closest ? target.closest("[data-tip]") : target;
    return el && el.getAttribute && el.getAttribute("data-tip") ? el : null;
  };
  let caption = null;
  const say = (el) => {
    if (caption && el) caption.textContent = el.getAttribute("title");
  };
  function onKey(ev) {
    const cur = tipOf(ev.target) ?? rover;
    const i = tipEls.indexOf(cur);
    if (i < 0) return;
    let to = null;
    if (ev.key === "ArrowRight") to = tipEls[Math.min(i + 1, tipEls.length - 1)];
    else if (ev.key === "ArrowLeft") to = tipEls[Math.max(i - 1, 0)];
    else if (ev.key === "Home") to = tipEls[0];
    else if (ev.key === "End") to = tipEls[tipEls.length - 1];
    else if (ev.key === "ArrowDown") to = tipEls.find((e) => lineOf(e) > lineOf(cur)) ?? cur;
    else if (ev.key === "ArrowUp") {
      const before = tipEls.filter((e) => lineOf(e) < lineOf(cur));
      to = before.length ? before.find((e) => lineOf(e) === lineOf(before[before.length - 1])) : cur;
    } else return;
    ev.preventDefault();
    setRover(to, true);
    say(to);
  }

  function render() {
    const m = slateModel(ledger.snapshot(), human);
    const st = m.header.settings;
    const rules = st ? t("slate.rules", { values: st.values, ultimo: st.ultimo, both_pass: t(`slate.both_pass.${st.both_pass}`) }) : "";
    const foot = m.foot;
    line = 0;
    const heads = h(doc, "div", { class: "slate-head" },
      h(doc, "h2", {}, t("slate.title")),
      h(doc, "button", { class: "slate-close", type: "button", "aria-label": t("slate.close"), onclick: () => close() }, "×"));
    const [simple, wins, loses] = st ? String(st.values).split("/") : [];
    const header = m.header.touren
      ? h(doc, "div", { class: "slate-rules" },
          h(doc, "span", tip("slate-header-touren", "header.touren", { touren: plural("slate.touren", m.header.touren), deals: m.header.touren * 2 }), plural("slate.touren", m.header.touren)),
          rules ? " · " : null,
          rules ? h(doc, "span", tip("slate-header-rules", "header.rules", { simple, wins, loses, ultimo: st.ultimo, pass: t(`slate.tip.pass.${st.both_pass}`) }), rules) : null)
      : null;
    line = 1;
    const first = h(doc, "small", tip("slate-first", "dealer1"), ` ${t("slate.dealer1")}`);
    const cols = h(doc, "div", { class: "slate-cols" },
      h(doc, "span", { class: "slate-col-margin" }),
      h(doc, "span", tip("slate-col slate-col-you", "heading.you"), t("slate.you"), m.header.youDealtFirst ? first : null),
      h(doc, "span", tip("slate-col slate-col-opp", "heading.opp"), t("slate.opp"), m.header.touren && !m.header.youDealtFirst ? first : null));
    const rows = m.blocks.length ? h(doc, "div", { class: "slate-rows" }, m.blocks.map(block)) : h(doc, "div", { class: "slate-empty" }, t("slate.empty"));
    line += 1;
    const balanceKind = foot.balance > 0 ? "ahead" : foot.balance < 0 ? "behind" : "level";
    const bal = h(doc, "span", tip("slate-balance", `balance.${balanceKind}`, { pts: P(Math.abs(foot.balance)), kr: kreuzer(foot.kreuzer) }), t("slate.balance", { n: signed(foot.balance), kr: kreuzer(foot.kreuzer) }));
    const dir = h(doc, "span", tip("slate-direction", `direction.${foot.direction}`, { kr: kreuzer(foot.kreuzer) }), foot.direction === "square" ? t("slate.square") : t(`slate.${foot.direction}`));
    caption = h(doc, "div", { class: "slate-tip-caption", "aria-hidden": "true" }, t("slate.tip.caption"));
    const sheet = m.settlement ? settlement(m.settlement) : null;
    const board = h(doc, "div", { class: "slate-board", title: t("slate.tooltip") }, heads, header, cols, rows, h(doc, "div", { class: "slate-foot" }, bal, dir), sheet, caption);
    root.replaceChildren(board);
    // The cols were built before the rows: the line numbers are in order of
    // the sheet's top to bottom all the same (1 for the columns, then by block).
    tipEls = collect(board);
    const keep = rover && tipEls.find((e) => e.getAttribute("data-tip") === rover.getAttribute("data-tip") && lineOf(e) === lineOf(rover));
    rover = null;
    if (tipEls.length) setRover(keep ?? tipEls[0]);
    for (const ev of ["focusin", "mouseover", "click"])
      board.addEventListener(ev, (e) => {
        const el = tipOf(e.target);
        if (!el) return;
        if (ev === "focusin") setRover(el);
        say(el);
      });
    board.addEventListener("keydown", onKey);
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
