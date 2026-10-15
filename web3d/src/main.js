// Strohmandeln at a three-dimensional table.
//
// A client of the wasm module (docs/PROTOCOL.md): it draws the human's view
// and sends back actions, and holds no rules. What may be played is
// `state.legal`; what a refusal means is its message key. Every change of
// state is played out on the cards by the director (choreography.js), each
// event's chips, the HUD, the talk and the screen reader's sentence at its
// moment. The plan is docs/TABLE3D.md.

import { Vector3 } from "three";
import { countEvents, countLedger } from "../../kit/js/scoring.js";
import { paceOf } from "../../kit/js/count.js";
import { choosePlace } from "../../kit/js/dialogue.js";
import { createDirector } from "../../kit/js/director.js";
import { createDeck } from "../../kit/js/deck.js";
import { createHud } from "../../kit/js/hud.js";
import { cardCorners } from "../../kit/js/kinematics.js";
import { createScene } from "../../kit/js/scene.js";
import { chooseSurface, PATTERNS } from "../../kit/js/surfaces.js";
import { facesFor, largeTextHere } from "../../kit/js/faces.js";
import { decodeBase64 } from "../../kit/js/engine.js";
import { CARD } from "../../kit/js/units.js";
import { sentence, where } from "./a11y.js";
import { choreograph, initialPlacement, LAST_TRICK, opening } from "./choreography.js";
import { dragKind, sameSpot, shake } from "./gesture.js";
import { handRange } from "./layout.js";
import { TRULL, KINGS } from "./cardinfo.js";
import { bonusLines, countGame, countLines, edgeOf } from "./counting.js";
import { loadEngine } from "./engine.js";
import { createMirror } from "./mirror.js";
import { createNewSitting } from "./newsitting.js";
import { momentsOf } from "./moments.js";
import { check, emptyModel, fromState, reduce } from "./model.js";
import { clearSitting, fileText, loadSitting, parseFile, saveSitting } from "./persist.js";
import { loadPrefsInfo, savePrefs, sittingConfig, speedScale, urlOverrides, withUrl } from "./prefs.js";
import { createSettings } from "./settings.js";
import { createSpeaker } from "./speaker.js";
import { createSlate } from "./slate.js";
import { hintLines, hintSpoken } from "./hinttext.js";
import { cardName, getLang, onLang, romanName, setLang, signed, suitName, t, tn, thirds } from "./strings.js";
import { createTalk } from "./talk.js";
import { ALL_CODES, backTexture, COURT_CODES, faceTextures } from "./tarockfaces.js";
import { createUi } from "./ui.js";
import { addSitting, clearProgress, exportText, loadProgress, summaryOf } from "./progress.js";
import { explainText, reviewBlocks } from "./review.js";
import { createTutor, extendPromptDue, PAGE_KEYS, tutorialPages } from "./tutor.js";
import { helpBody, pageBody, reviewBody, trendBody } from "./tutorview.js";
import TUTORIAL_DE from "../tutorial.md";
import TUTORIAL_EN from "../tutorial.en.md";
import { EYE, EYE_PORTRAIT, EYE_TOUCH, ZONES, ZONES_PORTRAIT, ZONES_TOUCH } from "./zones.js";

/* global WASM_BASE64, WORDS */

const params = new URL(window.location.href).searchParams;
const TEST = params.has("seed") || params.has("test");
const IDLE = [25_000, 60_000];

const storage = () => {
  try {
    return window.localStorage;
  } catch {
    return null;
  }
};

async function main() {
  document.getElementById("loading").textContent = t("ui.loading");
  let stopped = () => {};
  const engine = await loadEngine(decodeBase64(WASM_BASE64), (error) => {
    console.error(error);
    stopped();
  });

  // ---- settings, the language -------------------------------------------
  const info = loadPrefsInfo();
  let prefs = withUrl(info.prefs, params);
  const over = urlOverrides(params);
  setLang(prefs.lang);
  let stored = { ...info.prefs };
  const save = (patch) => {
    prefs = { ...prefs, ...patch };
    stored = { ...stored, ...patch };
    savePrefs(stored);
  };
  const calm = () => prefs.reducedMotion === "on" || (prefs.reducedMotion === "system" && window.matchMedia?.("(prefers-reduced-motion: reduce)").matches);
  const dirSpeed = () => {
    const scale = speedScale(prefs.speed);
    return scale === 0 ? 1000 : 1 / scale;
  };

  // ---- the scene, the cards -------------------------------------------------
  const touch = params.has("touch") ? params.get("touch") !== "0" : largeTextHere();
  const canvas = document.getElementById("stage");
  const stage = createScene(canvas, { table: chooseSurface({ chosen: prefs.surface === "random" ? null : prefs.surface, saved: null }), touch, ...(params.has("lowfx") ? { shadow: "pcf", shadowMap: 256 } : {}), eyes: { across: touch ? EYE_TOUCH : EYE, portrait: EYE_PORTRAIT } });
  const anisotropy = stage.renderer.capabilities.getMaxAnisotropy();
  const width = Math.round(512 * Math.min(Math.max(stage.renderer.getPixelRatio(), 1), 1.5));
  const faceSize = () => facesFor(prefs.faces, touch); // "jumbo" or "classic" (7.1, 7.6)
  const textures = { faces: faceTextures({ width, anisotropy, lang: getLang(), size: faceSize() }), back: backTexture({ width, anisotropy }) };
  const deck = createDeck(stage, textures);
  const zonesNow = () => (stage.portrait ? ZONES_PORTRAIT : touch ? ZONES_TOUCH : ZONES);

  // ---- state ------------------------------------------------------------------
  let wasm = null; // the module's last state
  let log = []; // every event of the sitting
  let S = null; // the director's state: { model, events }
  let human = 0;
  let sel = { chosen: null, scroll: -1e9 }; // a row of cards starts at its left end
  let focus = null; // { row: "hand" | "packets", index }
  let hintCard = null;
  let pending = 0; // scheduled moments not yet come
  let restModel = null; // the model at the last rest, for the tags
  let sittingOver = false;
  let dealNo = 0;
  let sawRefusal = false;
  let ledgerNow = null; // the count so far, for a rebuilt HUD
  let idleTimers = [];
  let sittingLength = 3; // the Touren the sitting was begun with (an extension adds as many)
  let askedExtend = false; // the human was asked about this extension
  let lastTiming = null;

  const lit = new Set();
  const view = () => ({ chosen: sel.chosen, scroll: sel.scroll, zones: zonesNow(), left: prefs.leftHanded, fill: stage.portrait ? stage.fill() : 1, reduced: calm() });

  function decorate(placement, d, meshes) {
    const mine = wasm?.prompt === "you" && !sittingOver;
    for (const m of placement) {
      const own = m.zone === "your-hand" || (m.zone === "your-packets" && m.code);
      let line = "plain";
      let dim = false;
      if (own && m.code) {
        const ok = lit.has(m.code);
        if (m.code === sel.chosen) line = "chosen";
        else if (m.code === hintCard) line = "hint";
        else if (focus && m.code === focusedCode()) line = "hover";
        else if (mine && ok) line = "hover";
        dim = mine && !ok;
      }
      d.decorate(meshes[m.id], { line, dim });
    }
  }

  const director = createDirector({
    stage,
    deck,
    view,
    decorate,
    rested: () => rested(),
    choreography: { choreograph, initialPlacement, opening },
    zones: null,
    cards: 54,
    manual: params.has("manual"),
    speed: dirSpeed(),
  });
  // The director's zones (handEdge of an empty hand) come from the view.
  const hudWords = () => ({
    category: t("hud.count"),
    bonus: t("hud.bonus"),
    goal: t("hud.thirds", { n: 36 }),
    names: { you: t("hud.you"), opp: t("hud.opp") },
    group: (n) => t("hud.group", { n }),
    hud: { score: t("hud.score"), hand: t("hud.hand"), live: t("hud.live"), subtotal: t("hud.subtotal"), total: t("hud.total"), show: t("hud.show"), hide: t("hud.hide") },
  });

  // ---- the page ---------------------------------------------------------------
  const ui = createUi(document.getElementById("overlay"), { t, onAct: (act, data) => onAct(act, data) });
  const mirror = createMirror(document, { t, tn, cardName, onFocus: (row, index) => mirrorFocused(row, index), onMove: (row, by, abs) => moveFocus(row, by, abs), onPick: (row, index) => pickCard(focusList(row)[index]), onPacket: (n) => chooseCover(n) });
  ui.controls.append(mirror.node);
  document.getElementById("loading").remove();
  canvas.setAttribute("aria-label", t("ui.stage"));

  let game = countGame(hudWords());
  let hud = createHud(ui.hudSlot, game, { later: (ms, fn) => director.at(ms, fn), reduced: () => calm() });

  const settings = createSettings(ui.settingsRoot, {
    t,
    prefs,
    surfaces: PATTERNS.map((p) => ({ value: p.id, label: p.name })),
    onChange: (key, value) => changed(key, value),
    onAction: (name) => settingsAction(name),
  });
  // The slate's self-folding timer must be cancellable (director.at returns no id): a fold set at a
  // Tour's end would otherwise close the settlement opened at the sitting's end.
  const slateTimers = new Set();
  let slateTimer = 0;
  const slate = createSlate(ui.slateRoot, {
    t,
    tn,
    later: (ms, fn) => {
      const id = ++slateTimer;
      slateTimers.add(id);
      director.at(ms, () => slateTimers.delete(id) && fn(), "linger");
      return id;
    },
    cancel: (id) => slateTimers.delete(id),
  });

  // The opponent has one voice: lines wait their turn, and chatter yields to the calls (5.2.2).
  const boxes = new Map();
  const speaker = createSpeaker({
    clock: () => director.clock(),
    later: (ms, fn, kind) => director.at(ms, fn, kind),
    show: (line) => boxes.set(line, showLine(line.text)),
    hide: (line) => {
      boxes.get(line)?.remove();
      boxes.delete(line);
    },
  });
  const talk = createTalk({
    words: WORDS,
    getLang,
    rng: Math.random,
    say: ({ text, call, who }) => (who === "kibitz" ? kibitzNote(text) : speaker.offer({ text, call })),
  });
  talk.setMode(prefs.talk);

  // ---- the tutor (docs/TABLE3D.md 6) ------------------------------------------------
  const tutorAuto = !TEST || params.has("tutor"); // a test run is not interrupted by a page
  const tutor = createTutor({
    prefs: () => prefs,
    patch: (p) => {
      save(p);
      showTutor();
    },
    open: (key, how) => openPage(key, how),
    speak: (group) => talk.onEvent({ type: group === "kibitz" ? "kibitz" : "kibitz_mute" }, { human }),
  });
  let tutorBuf = []; // events the tutor has not yet reckoned with
  let reviewChip = null; // null, "deal" or "sitting"
  let pageHelp = false; // the page open now came from the Help list
  let pastSittings = []; // the sittings kept before this one, for the sitting's review
  const showTutor = () => ui.setTutor({ on: prefs.tutor, muted: prefs.muted, review: reviewChip });
  const tutorWords = () => ({ t, cardName: romanName, thirds, signed, suitName: (c) => (getLang() === "en" ? suitName(c).toLowerCase() : suitName(c)) });
  showTutor();

  // The Kibitzer's note, in his tab and not in the opponent's box (6.5).
  function kibitzNote(text) {
    ui.kibitzNote(text);
    director.at(Math.max(4000, 250 + 70 * text.length), () => ui.kibitzNote(null), "linger");
  }

  // The pages, for the sitting's both-pass rule (6.2.2).
  function pagesNow() {
    const begun = log.find((e) => e.type === "sitting_start");
    const both_pass = begun?.settings?.both_pass ?? prefs.both_pass;
    return tutorialPages(getLang() === "de" ? TUTORIAL_DE : TUTORIAL_EN, PAGE_KEYS, { both_pass });
  }
  function openPage(key, { help }) {
    const page = pagesNow().find((p) => p.key === key);
    if (!page) return tutor.helpClosed(key);
    pageHelp = help;
    ui.dialog({
      id: "page",
      title: page.title,
      body: pageBody(document, page),
      wide: true,
      buttons: help
        ? [{ label: t("ui.close"), act: "close", primary: true, focus: true }]
        : [{ label: t("tutor.page.notnow"), act: "notnow", primary: true, focus: true }, { label: t("tutor.page.remind"), act: "remind" }],
      onClose: () => dismissPage(true),
    });
  }
  // "Not now" marks the page seen, "Remind me" leaves it for the next time (6.2.2).
  function dismissPage(remind) {
    ui.closeDialog("page");
    const key = tutor.current();
    if (!key) return;
    if (pageHelp) tutor.helpClosed(key);
    else tutor.close(key, remind);
    // A second page of the same moment follows (not after a Help page); then
    // the bar and the prompts a page was holding back.
    if (S) {
      if (!pageHelp) tutorStep();
      refresh();
    }
  }
  function openHelp() {
    tutor.intro();
    ui.dialog({ id: "help", title: t("tutor.help.title"), body: helpBody(document, pagesNow(), { intro: t("tutor.help.intro"), seen: prefs.seen }), buttons: [{ label: t("ui.close"), act: "close", primary: true, focus: true }] });
  }
  // The review of the deal, or of the sitting once it is over (6.4).
  function openReview() {
    if (!wasm) return;
    const whole = sittingOver;
    const r = whole ? engine.reviewSitting() : engine.review();
    const blocks = reviewBlocks(r?.review, tutorWords(), { whole });
    if (!blocks) return ui.setAid(t("review.empty"));
    const body = reviewBody(document, blocks);
    if (whole && pastSittings.length) {
      body.append(trendBody(document, t("tutor.progress.title"), pastSittings.slice(-5).map((s) => t("review.say.sitting-tally", { ...s, net: signed(s.net) }))));
    }
    ui.dialog({ id: "review", title: blocks.title, body, wide: true, buttons: [{ label: t("ui.close"), act: "close", primary: true, focus: true }] });
  }
  // At rest the tutor may open the page that is due.
  function tutorStep() {
    const events = tutorBuf;
    tutorBuf = [];
    if (tutorAuto && wasm) tutor.rest(wasm, events, { human, blocked: ui.dialogOpen() });
  }

  // A line in a box by the opponent's hand, clear of the cards and the controls.
  function showLine(text) {
    const edge = director.handEdge("them");
    const places = [
      { kind: "below", x: edge.x, y: edge.y },
      { kind: "right", x: edge.x + 150, y: edge.y },
      { kind: "left", x: edge.x - 150, y: edge.y },
    ];
    const box = ui.sayBox("them", text, places[0]);
    const rect = box.getBoundingClientRect();
    const keep = cardRectsOf("their-packets").concat(cardRectsOf("your-packets"), cardRectsOf("trick"), [hudRect(), ...controlRects()]);
    const place = choosePlace(places, () => ({ w: rect.width, h: rect.height, natural: rect.width, spills: false }), keep, { width: window.innerWidth, height: window.innerHeight });
    placeBox(box, place, rect);
    return box;
  }
  function placeBox(box, place, rect) {
    const w = rect.width;
    const h = rect.height;
    let left = place.x - w / 2;
    let top = place.y + 18;
    if (place.kind === "right") (left = place.x), (top = place.y - h / 2);
    if (place.kind === "left") (left = place.x - w), (top = place.y - h / 2);
    left = Math.max(8, Math.min(window.innerWidth - w - 8, left));
    top = Math.max(8, Math.min(window.innerHeight - h - 8, top));
    box.style.left = `${left}px`;
    box.style.top = `${top}px`;
  }
  const hudRect = () => ui.hudSlot.getBoundingClientRect();
  const controlRects = () => [ui.controls.getBoundingClientRect(), document.querySelector(".top-right").getBoundingClientRect()];

  // ---- where cards are on the screen ---------------------------------------------
  const rectOf = (points) => ({ left: Math.min(...points.map((p) => p.x)), right: Math.max(...points.map((p) => p.x)), top: Math.min(...points.map((p) => p.y)), bottom: Math.max(...points.map((p) => p.y)) });
  function cardRectsOf(zone) {
    return director
      .placement()
      .filter((m) => m.zone === zone)
      .map((m) => rectOf(cardCorners(m.pose).map((c) => director.toScreen(c))));
  }
  function zoneRects() {
    const out = {};
    for (const zone of ["your-hand", "their-hand", "your-packets", "their-packets", "trick", "your-won", "their-won", "pack", "count"]) {
      const cards = director.placement().filter((m) => m.zone === zone);
      if (cards.length) out[zone] = rectOf(cards.flatMap((m) => cardCorners(m.pose).map((c) => director.toScreen(c))));
    }
    return out;
  }

  // ---- tags that follow cards -----------------------------------------------------
  function placeTags() {
    if (!S || !restModel) return ui.tags([]);
    const m = restModel;
    const list = [];
    const placement = director.placement();
    const Z = zonesNow();
    // The hidden count of each packet, on the corner opposite its cover's index.
    for (const seat of [0, 1]) {
      for (let i = 0; i < 3; i++) {
        const p = m.packets[seat][i];
        if (p.hidden <= 0) continue;
        const stack = placement.filter((x) => x.packet?.seat === seat && x.packet.i === i);
        const top = stack.sort((a, b) => b.pose.position.y - a.pose.position.y)[0];
        if (!top) continue;
        const mine = seat === m.human;
        const corner = new Vector3(CARD.width / 2 - 0.1, mine ? -CARD.height / 2 + 0.1 : CARD.height / 2 - 0.1, CARD.thickness / 2);
        const at = director.toScreen(corner.applyQuaternion(top.pose.quaternion).add(top.pose.position));
        list.push({ id: `badge:${seat}:${i}`, text: String(p.hidden), x: at.x, y: at.y, kind: "badge", title: t("ui.packets." + (mine ? "yours" : "theirs")) });
      }
    }
    // The dealer's marker and the declarer's badge, by their hands.
    for (const [key, seat, text] of [["dealer", m.dealer, t("hud.dealer.short")], ["declarer", m.declarer, t("ui.take")]]) {
      if (seat === null || seat === undefined || (key === "declarer" && !wasm?.deal?.playing && m.declarer === null)) continue;
      const who = seat === m.human ? "you" : "them";
      const edge = director.handEdge(who);
      const dx = key === "dealer" ? -1 : 1;
      list.push({ id: key, text, x: edge.x + dx * (key === "dealer" ? 150 : 100), y: edge.y + (who === "you" ? -14 : 14), kind: key });
    }
    // The public mark on a hand card the other player has seen.
    for (const slot of placement.filter((x) => (x.zone === "your-hand" || x.zone === "their-hand") && x.public)) {
      const corner = new Vector3(-CARD.width / 2 + 0.2, -CARD.height / 2 + 0.2, CARD.thickness / 2);
      const at = director.toScreen(corner.applyQuaternion(slot.pose.quaternion).add(slot.pose.position));
      list.push({ id: `pub:${slot.zone}:${slot.index}`, text: "", x: at.x + 8, y: at.y - 6, kind: "public", width: 10, height: 10, title: t("ui.public.tip") });
    }
    ui.tags(list);
  }
  stage.onRender = placeTags;

  // ---- the model, kept beside the module's state ------------------------------------
  // The model after the events, and the cards of the count as it stood when `count` came
  // (a reply can run on into the next deal, whose model has none).
  function fold(events) {
    let model = S ? S.model : emptyModel(human);
    let counted = null;
    for (const e of events) {
      model = reduce(model, e);
      if (e.type === "count") counted = model.counted;
    }
    return { model, counted };
  }

  // A reply's events played out, and what is shown at each one's moment.
  function advance(reply, { first = false } = {}) {
    const events = reply.events ?? [];
    const since = log.length;
    log = [...log, ...events];
    tutorBuf.push(...events);
    wasm = reply.state;
    const { model, counted } = events.length || first ? fold(events) : { model: S.model, counted: null };
    const next = { model, events: log };
    const begun = events.find((e) => e.type === "sitting_start");
    if (begun) sittingLength = begun.touren;
    const countEvent = events.find((e) => e.type === "count");
    const lines = countEvent ? countLines(counted, human) : [];
    const pace = countEvent ? paceOf(game, lines) : null;
    const timing = first ? director.restart(next, { dealt: true }) : director.advance(next, { pace });
    S = next;
    lastTiming = { ...timing, base: director.clock() };
    sel = { ...sel, chosen: null };
    hintCard = null;
    ui.setAid(null);
    ui.setRefusal(null);
    for (const m of momentsOf({ events, since, beats: timing.beats, count: countEvent ? timing.count : null })) {
      later(m.ms, () => (m.synthetic ? talk.onEvent(m.event, { human }) : onEvent(m.event, m.index, model, timing)));
    }
    if (countEvent) schedulePopups(timing.count, lines, since + events.indexOf(countEvent), events.find((e) => e.type === "game_scored"));
    persist(reply);
    refresh();
  }
  function later(ms, fn) {
    pending++;
    director.at(ms, () => {
      pending--;
      try {
        fn();
      } catch (error) {
        console.error(error);
      } finally {
        if (pending === 0 && !director.busy()) rested();
      }
    });
  }

  // The count's popups, each at its line's moment; then the edge (the tick
  // at 107, the chip on 106) and the bonuses the deal paid, in Points (2.5.4, 2.5.5).
  function schedulePopups(count, lines, at, scored) {
    if (!count) return;
    const events = countEvents(game, lines, { at, hand: 1 });
    for (const ev of events) {
      later(ev.end ? count.end : (count.lines[ev.line] ?? count.end), () => (ev.end ? hud.endHand(1) : hud.score(ev)));
    }
    later(count.end + 1, () => {
      const edge = edgeOf(hud.totals());
      hud.node.classList.toggle("edge", edge.tick);
      for (const side of edge.short) ui.chip(t("chip.not-36", { a: thirds(hud.totals()[side]) }), { x: 120, y: side === "you" ? 150 : 100, kind: "edge", ms: 3000 });
    });
    const paid = bonusLines(scored?.credits ?? [], human, (cause) => t(`slate.cause.${cause}`));
    for (const ev of countEvents(game, paid, { at, hand: 1 })) if (!ev.end) later(count.end + 700, () => hud.score(ev));
  }

  // ---- what is shown at an event's moment ------------------------------------------
  function onEvent(e, index, model) {
    const s = sentence(e, model);
    if (s) ui.live(s);
    talk.onEvent(e, { human, state: wasm, index });
    slate.update(wasm, e.type === "card_pulled" ? [] : [e]); // the slate reads the events; the state is the reply's last
    const at = (seat) => director.handEdge(seat === human ? "you" : "them");
    const chip = (text, seat, kind = "", ms) => {
      const p = at(seat);
      ui.chip(text, { x: p.x, y: p.y + (seat === human ? -40 : 30), kind, ms });
    };
    switch (e.type) {
      case "deal_start":
        dealNo = e.deal_number;
        trickShown = 0;
        hud.reset({ hands: [], live: null });
        hud.node.classList.remove("edge");
        hud.dealt(1);
        showTour();
        ui.hideBanner();
        reviewChip = null;
        showTutor();
        break;
      case "trick_won":
        // The line under the prompt, from the module's own deduction (6.3.3).
        if (prefs.explain && !prefs.muted) {
          const line = explainText(engine.explain()?.explain, tutorWords());
          ui.setAid(line);
          if (line) ui.live(line);
        }
        break;
      case "card_played":
        trickShown = e.trick;
        showTour();
        break;
      case "bid":
        chip(t("ui.take"), e.seat, "bid", 1800);
        break;
      case "pass":
        chip(t("ui.pass"), e.seat, "", 1200);
        break;
      case "both_pass":
        if (e.mode === "play") ui.chip(t("chip.simple"), { x: window.innerWidth / 2, y: window.innerHeight * 0.4, kind: "simple", ms: 2200 });
        break;
      case "thrown_in":
        ui.chip(t("chip.thrown-in"), { x: window.innerWidth / 2, y: window.innerHeight * 0.4, kind: "simple", ms: 2400 });
        break;
      case "void_deal":
        for (const seat of e.seats) chip(t("chip.void-deal"), seat, "void", 2200);
        break;
      case "packet_empty":
        chip(t("chip.last-card"), e.seat, "tag", 1200);
        break;
      case "announcement":
        chip(t(`chip.honour.${e.honour}`, { n: pointsNow().honour }), e.seat, "announce", 1800);
        break;
      case "honour_forfeited":
        chip(t("chip.forfeited"), e.seat, "forfeit", 1600);
        break;
      case "pagat_ultimo":
        ui.chip(t(`chip.ultimo.${e.result}`, { n: pointsNow().ultimo }), { x: window.innerWidth / 2, y: window.innerHeight * 0.35, kind: `ultimo ${e.result}`, ms: 2200 });
        break;
      case "valat":
        chip(t("chip.valat"), e.seat, "valat", 2200);
        break;
      case "doubling_set":
        ui.chip(t("chip.doubling"), { x: 160, y: 120, kind: "simple", ms: 2400 });
        break;
      case "doubling_lapsed":
        ui.chip(t("chip.lapses"), { x: 160, y: 120, kind: "simple", ms: 1800 });
        break;
      case "game_scored":
        showResult(e);
        reviewChip = "deal";
        showTutor();
        break;
      case "tour_end":
        if (prefs.slateAuto) slate.open({ ms: 3000 });
        break;
      case "extension_offered":
        askedExtend = false;
        if (prefs.slateAuto) slate.open({ ms: 3000 });
        break;
      case "extension_decided":
        if (!e.accepted && !askedExtend) ui.chip(t("chip.ext-opp-no"), { x: window.innerWidth / 2, y: 200, ms: 2400 });
        break;
      case "sitting_end":
        endSitting(e);
        break;
      default:
        break;
    }
  }

  // "Tour 2 of 3 · Deal 3 · ×2 · Trick 27 / 27": from the last deal begun in the log.
  let trickShown = 0;
  function showTour() {
    const d = [...log].reverse().find((e) => e.type === "deal_start");
    if (!d || !wasm) return ui.setTour("");
    ui.setTour([t("hud.tour", { n: d.tour, total: wasm.touren_total }), t("hud.deal", { n: d.deal_number }), d.multiplier > 1 ? t("hud.double") : null, trickShown ? t("hud.trick", { n: trickShown, total: LAST_TRICK }) : null].filter(Boolean).join(" · "));
  }

  // What a Point is worth on the deal now: the ultimo setting times the doubling.
  function pointsNow() {
    const begun = log.find((e) => e.type === "sitting_start");
    const d = [...log].reverse().find((e) => e.type === "deal_start");
    return { ultimo: (begun?.settings?.ultimo ?? 1) * (d?.multiplier ?? 1), honour: d?.multiplier ?? 1 };
  }

  function showResult(e) {
    const points = e.credits.filter((c) => c.cause === "game").reduce((s, c) => s + c.points, 0);
    const title = e.result === "draw" ? t("banner.draw") : t(`banner.${e.result.replace("_", "-")}`, { points: t("banner.plus", { n: points }) });
    const rows = e.credits.map((c) => [t(`slate.cause.${c.cause}`), t("banner.plus", { n: c.points })]);
    if (e.multiplier > 1) rows.push([t("banner.doubled"), ""]);
    const count = [...log].reverse().find((x) => x.type === "count");
    const tally = count ? t("banner.tally", { a: thirds(count.thirds[human]), b: thirds(count.thirds[1 - human]) }) : null;
    ui.showBanner({ title, rows, foot: [tally, e.result === "draw" ? t("banner.doubling-next") : null].filter(Boolean).join(" · ") || null });
    director.at(3000, () => ui.hideBanner(), "linger");
  }

  function endSitting(e, { live = true } = {}) {
    sittingOver = true;
    slate.settle(e);
    clearSitting();
    // The sitting's review, offered by the chip; its tally is kept for trends (6.4.2, 6.4.4).
    reviewChip = "sitting";
    showTutor();
    if (live) {
      pastSittings = loadProgress(storage());
      const summary = summaryOf(engine.reviewSitting()?.review);
      if (summary) addSitting(storage(), summary);
    }
    stage.reveal({ instant: calm() || prefs.speed === "instant" });
    ui.dialog({
      id: "over",
      title: t("ui.prompt.over"),
      body: "",
      buttons: [{ label: t("ui.new.plus"), act: "new", primary: true, focus: true }],
    });
  }

  // ---- when the cards are still ---------------------------------------------------------
  function rested() {
    if (pending > 0 || director.busy() || !S) return;
    restModel = S.model;
    placeTags();
    tutorStep(); // a due page opens before the bar and the prompts are drawn
    refresh();
    armIdle();
    if (wasm.prompt === "opponent" && !sittingOver && !stepping) {
      stepping = true;
      director.at(350, () => {
        stepping = false;
        step();
      }, "linger");
    }
  }
  let stepping = false;

  function step() {
    if (!wasm || wasm.prompt !== "opponent") return;
    const r = engine.step();
    if (r?.error || r?.ok === false) {
      ui.dialog({ id: "error", title: t("error.opponent-refused"), body: "", buttons: [{ label: t("ui.close"), act: "close", focus: true }] });
      return;
    }
    advance(r);
  }

  function armIdle() {
    idleTimers.forEach(clearTimeout);
    idleTimers = [];
    if (wasm?.prompt !== "you" || wasm.phase !== "play") return;
    IDLE.forEach((ms, n) => idleTimers.push(setTimeout(() => wasm?.prompt === "you" && !director.busy() && !document.hidden && talk.onEvent({ type: "slow", n: n + 1 }, { human }), ms)));
  }

  // ---- what the person can do -----------------------------------------------------------
  const legalPlays = () => (wasm?.prompt === "you" ? wasm.legal.filter((a) => a.action === "play") : []);
  const honourOf = (code) => (TRULL.includes(code) ? "trull" : KINGS.includes(code) ? "kings" : null);
  const heldHonours = () => new Set((wasm?.legal ?? []).filter((a) => a.action === "announce").map((a) => a.honour));

  function refresh() {
    if (!wasm) return;
    lit.clear();
    for (const a of legalPlays()) lit.add(a.card);
    const mine = wasm.prompt === "you" && !sittingOver;
    const busy = director.busy() || pending > 0;
    // The prompt.
    let prompt = "";
    if (!busy && !sittingOver) {
      if (wasm.phase === "bid" && mine) prompt = t("ui.prompt.bid");
      else if (wasm.phase === "play" && mine) prompt = wasm.deal?.lead ? (prefs.explain && lit.size && legalNarrowed() ? t("ui.prompt.follow") : "") : t("ui.prompt.lead");
      else if (wasm.prompt === "opponent") prompt = t("ui.prompt.opp");
    }
    ui.setPrompt(prompt);
    // The bar.
    const groups = [];
    if (mine && !busy) {
      if (wasm.phase === "bid") {
        const has = (a) => wasm.legal.some((x) => x.action === a);
        const buttons = [];
        if (has("take")) buttons.push({ act: "take", label: t("ui.take"), enabled: true, kind: "take" });
        if (has("pass")) buttons.push({ act: "pass", label: t("ui.pass"), enabled: true, kind: "pass" });
        groups.push({ kind: "bid", buttons });
      } else if (wasm.phase === "play") {
        const held = heldHonours();
        groups.push({
          kind: "announce",
          buttons: ["trull", "kings"].map((h) => ({ act: "announce", data: { honour: h }, label: t(`ui.announce.${h}`), enabled: held.has(h), ask: true })),
        });
      }
    }
    if (mine && !busy && prefs.hints && wasm.phase !== "extend") groups.push({ kind: "hint", buttons: [{ act: "hint", label: t("ui.hint"), enabled: true }] });
    ui.setBar(groups, { left: prefs.leftHanded });
    // The extension prompt.
    if (extendPromptDue({ mine, busy, phase: wasm.phase, extendOpen: ui.dialogOpen("extend"), pageOpen: ui.dialogOpen("page") })) {
      askedExtend = true;
      const yes = { label: t("ui.ext.yes"), act: "yes", primary: true, focus: true };
      const no = { label: t("ui.ext.no"), act: "no" };
      ui.dialog({ id: "extend", title: t("ui.ext.title"), body: tn("ui.ext.adds", sittingLength), buttons: prefs.leftHanded ? [no, yes] : [yes, no] });
    }
    if (wasm.phase !== "extend") ui.closeDialog("extend");
    mirror.set(mirrorState(mine && !busy));
    director.redecorate();
  }
  // The cards as the keyboard and the screen reader see them (3.10.1): the
  // hand and the covers, lit as `state.legal` has them.
  function mirrorState(live) {
    if (!S) return { hand: [], packets: [] };
    const ok = (code) => live && lit.has(code);
    const packets = S.model.packets[human].map((p, i) => ({ p, i })).filter(({ p }) => p.cover);
    return {
      hand: S.model.hand.map((c) => ({ code: c.code, lit: ok(c.code), chosen: c.code === sel.chosen })),
      packets: packets.map(({ p, i }) => ({ packet: i + 1, code: p.cover, hidden: p.hidden, lit: ok(p.cover), chosen: p.cover === sel.chosen })),
      focus,
    };
  }
  const legalNarrowed = () => lit.size < (wasm.deal?.own_hand?.length ?? 0) + (wasm.deal?.covers?.[wasm.human] ?? []).filter(Boolean).length;

  // A tap or a click on the table.
  // `at` is where the click fell. A card chosen has lifted from under the pointer, so a
  // second click at the same spot is the chosen card again, whatever lies there now (3.1.2).
  function tapped(slot, at) {
    if (director.busy() || pending > 0) {
      director.skip();
      return;
    }
    if (sel.chosen && slot?.code !== sel.chosen && sameSpot(sel.at, at)) slot = director.placement().find((m) => m.code === sel.chosen) ?? slot;
    ui.hideBanner();
    if (!slot || !slot.code || !(slot.zone === "your-hand" || slot.zone.endsWith("packets") || slot.zone === "trick" || slot.zone === "count")) {
      if (sel.chosen) {
        sel = { ...sel, chosen: null };
        refresh();
        director.rearrange();
      }
      return;
    }
    const code = slot.code;
    if (wasm.prompt !== "you" || !lit.has(code)) return askRefusal(code);
    if (sel.chosen === code) return commit(code);
    sel = { ...sel, chosen: code, at };
    ui.live(t("sr.chosen", { card: cardName(code) }));
    focus = null;
    ui.setRefusal(null);
    refresh();
    director.rearrange();
  }

  // A card that is not lit is sent anyway: the engine says why (3.2).
  function askRefusal(code) {
    const r = engine.send({ action: "play", card: code });
    if (r.ok) return advance(r); // the module allowed it after all: it is played
    showRefusal(r.refusal);
    shake(director.meshOf(code), { now: () => performance.now(), frame: requestAnimationFrame, render: () => stage.render(), reduced: calm() }); // 3.1.3
  }
  function showRefusal(refusal) {
    if (!refusal) return;
    const p = { ...refusal };
    if (p.led) p.led = t(`card.suit.${p.led}`);
    if (p.honour) p.honour = t(`card.honour.${p.honour}`);
    const text = t(`refusal.${refusal.key}`, p);
    ui.setRefusal(text);
    ui.live(t("sr.refusal", { text }));
    talk.onEvent({ type: "refused" }, { human });
    if (refusal.allowed) {
      hintCard = null;
      director.redecorate();
    }
  }

  // Send an action; a refusal is shown, an acceptance played out.
  function send(action) {
    const r = engine.send(action);
    if (!r.ok) {
      if (r.refusal) showRefusal(r.refusal);
      else if (r.error) ui.setRefusal(t(`error.${r.error}`));
      return false;
    }
    sel = { ...sel, chosen: null };
    focus = null; // the card in focus is played (or its row has changed under it): the next arrow starts again from the hand
    advance(r);
    return true;
  }

  function commit(code) {
    const honour = honourOf(code);
    const held = heldHonours();
    if (honour && held.has(honour)) {
      if (prefs.announceAuto) return send({ action: "play", card: code, announce: [honour] });
      ui.dialog({
        id: "announce",
        title: t(`ui.announce.prompt.${honour}`),
        body: t("ui.announce.note"),
        buttons: [
          { label: t("ui.announce.do"), act: `do:${code}:${honour}`, primary: true, focus: true },
          { label: t("ui.announce.dont"), act: `dont:${code}` },
        ],
        onClose: () => ui.closeDialog("announce"),
      });
      return false;
    }
    return send({ action: "play", card: code });
  }

  // The tutor's hint: the engine's own answer for this position, shown by the
  // card it names and one sentence from its reason's key. It never plays.
  function askHint() {
    if (!wasm || wasm.prompt !== "you" || director.busy() || pending > 0 || !prefs.hints) return;
    if (prefs.tutor && prefs.muted) return ui.setAid(t("hint.muted")); // "§ 13": only his tap un-mutes (6.3.4)
    const r = engine.hint();
    if (!r?.hint) return ui.setAid(t("hint.none"));
    hintCard = r.hint.action === "play" ? r.hint.card : null;
    const lines = hintLines(r, { t, cardName: romanName, thirds });
    ui.setAid([lines.lead, lines.reason].filter(Boolean).join(" "), lines.more);
    ui.live(hintSpoken(hintLines(r, { t, cardName, thirds })));
    director.redecorate();
  }

  // ---- buttons ---------------------------------------------------------------------------
  function onAct(act, data) {
    if (act.startsWith("dialog:")) return onDialog(act.split(":").slice(1));
    switch (act) {
      case "take":
      case "pass":
        return send({ action: act });
      case "announce": {
        const r = engine.send({ action: "announce", honour: data.honour });
        if (!r.ok) return showRefusal(r.refusal);
        return advance(r);
      }
      case "slate":
        return toggleSlate();
      case "settings":
        settings.open();
        return;
      case "hint":
        return askHint();
      case "tutor":
        return openHelp();
      case "mute":
        return tutor.toggleMute();
      case "review":
        return openReview();
      case "page":
        return tutor.help(data.page);
      default:
    }
  }
  function toggleSlate() {
    if (ui.slateRoot.hidden) {
      tutor.slateOpened();
      slate.open();
    }
    else slate.close();
    ui.setSlateOpen(!ui.slateRoot.hidden);
  }
  function onDialog([id, ...rest]) {
    if (id === "announce") {
      ui.closeDialog("announce");
      const [what, code, honour] = rest;
      if (what === "do") send({ action: "play", card: code, announce: [honour] });
      else send({ action: "play", card: code });
    } else if (id === "extend") {
      ui.closeDialog("extend");
      send({ action: "extend", accept: rest[0] === "yes" });
    } else if (id === "welcome") {
      ui.closeDialog("welcome");
      if (rest[0] === "continue") continueSitting();
      else askNew();
    } else if (id === "new") {
      const chosen = newScreen?.values() ?? {};
      ui.closeDialog("new");
      if (rest[0] === "deal") {
        for (const [key, value] of Object.entries(chosen)) {
          if (prefs[key] === value) continue;
          save({ [key]: value });
          settings.set(key, value);
        }
        newSitting({ touren: prefs.touren });
      }
    } else if (id === "over") {
      ui.closeDialog("over");
      askNew();
    } else if (id === "page") dismissPage(rest[0] === "remind");
    else if (id === "keys") ui.closeDialog("keys");
    else ui.closeDialog(id);
  }

  // ---- a sitting ----------------------------------------------------------------------------
  function resetTable() {
    director.cancelTimed();
    stepping = false;
    idleTimers.forEach(clearTimeout);
    idleTimers = [];
    pending = 0;
    speaker.stop();
    boxes.clear();
    ui.hush();
    ui.clearChips();
    ui.hideBanner();
    ui.closeDialog("over");
    for (const id of ["page", "help", "review"]) ui.closeDialog(id);
    ui.kibitzNote(null);
    tutor.reset();
    tutorBuf = [];
    reviewChip = null;
    showTutor();
    stage.unreveal();
    sittingOver = false;
    sel = { chosen: null, scroll: -1e9 };
    focus = null;
    wasm = null;
    S = null;
    log = [];
    restModel = null;
  }

  function newSitting({ touren = prefs.touren, seed = over.seed, humanSeat = over.human } = {}) {
    resetTable();
    const config = sittingConfig({ ...prefs, touren }, { seed: seed ?? undefined, human: humanSeat ?? Math.floor(Math.random() * 2) });
    if (params.has("level")) config.level = Number(params.get("level")); // for tests: the opponent's rung
    const r = engine.new(config);
    if (!r?.ok) {
      ui.dialog({ id: "error", title: t("error.bad-config"), body: "", buttons: [{ label: t("ui.close"), act: "close", focus: true }] });
      return;
    }
    human = r.state.human;
    settings.setSitting(true);
    ui.root.classList.toggle("left", prefs.leftHanded);
    talk.restore({});
    S = null;
    advance(r, { first: true });
  }

  // The new-sitting screen (3.5.1): what is chosen is kept as the next default (3.5.2).
  let newScreen = null;
  function askNew() {
    newScreen = createNewSitting(document, { t, prefs });
    ui.dialog({ id: "new", title: t("ui.new.title"), body: newScreen.node, buttons: [{ label: t("ui.new.deal"), act: "deal", primary: true, focus: true }, ...(wasm && !sittingOver ? [{ label: t("ui.new.cancel"), act: "cancel" }] : [])] });
  }

  // ---- keeping the sitting -----------------------------------------------------------------------
  const extras = () => ({ lang: getLang(), slate: slate.snapshot(), talk: talk.snapshot(), human, touren: sittingLength });
  let warned = false;
  function persist(reply) {
    if (TEST && !params.has("keep")) return;
    const over = reply.state.over;
    const kept = saveSitting(over ? null : engine.export(), extras(), { over });
    if (!kept.ok && !warned) {
      warned = true;
      ui.chip(t(kept.reason === "too-big" ? "chip.too-big" : "chip.unsaved"), { x: window.innerWidth / 2, y: 120, ms: 4000 });
    }
  }
  function continueSitting() {
    const kept = loadSitting();
    if (kept) restore(kept.export, kept.extras);
    else askNew();
  }
  function restore(exported, ex = {}) {
    const r = engine.import(exported);
    if (!r?.ok) {
      ui.dialog({ id: "error", title: t("error.bad-record"), body: "", buttons: [{ label: t("ui.close"), act: "close", focus: true }] });
      return false;
    }
    resetTable();
    human = r.state.human;
    wasm = r.state;
    log = [...r.state.events];
    const model = fromState(r.state);
    if (check(model, r.state).length) console.error("the restored model differs from the state", check(model, r.state));
    S = { model, events: log };
    if (ex.lang && ex.lang !== getLang()) changed("lang", ex.lang);
    if (ex.touren) sittingLength = ex.touren;
    else sittingLength = log.find((e) => e.type === "sitting_start")?.touren ?? r.state.touren_total;
    if (ex.slate) slate.restore(ex.slate);
    if (ex.talk) talk.restore(ex.talk);
    settings.setSitting(true);
    director.restart(S, { dealt: false });
    restModel = model;
    trickShown = r.state.deal?.playing ? r.state.deal.trick_number : 0;
    showTour();
    hud.reset({ hands: [], live: null });
    ui.root.classList.toggle("left", prefs.leftHanded);
    if (r.state.over) {
      const end = [...log].reverse().find((e) => e.type === "sitting_end");
      if (end) endSitting(end, { live: false });
      else {
        sittingOver = true;
        askNew();
      }
    }
    refresh();
    rested();
    return true;
  }

  function settingsAction(name) {
    if (name === "save" && wasm) {
      const text = fileText(engine.export(), extras());
      const url = URL.createObjectURL(new Blob([text], { type: "application/json" }));
      const link = Object.assign(document.createElement("a"), { href: url, download: "strohmandeln-sitting.json" });
      document.body.append(link);
      link.click();
      link.remove();
      setTimeout(() => URL.revokeObjectURL(url), 1000);
    } else if (name === "progress-export") {
      const url = URL.createObjectURL(new Blob([exportText(storage())], { type: "application/json" }));
      const link = Object.assign(document.createElement("a"), { href: url, download: "strohmandeln-progress.json" });
      document.body.append(link);
      link.click();
      link.remove();
      setTimeout(() => URL.revokeObjectURL(url), 1000);
    } else if (name === "progress-clear") {
      clearProgress(storage());
      pastSittings = [];
    } else if (name === "load") {
      const input = Object.assign(document.createElement("input"), { type: "file", accept: "application/json" });
      input.addEventListener("change", async () => {
        const file = input.files?.[0];
        if (!file) return;
        const got = parseFile(await file.text());
        if (!got.ok) return ui.setRefusal(t(`error.${got.error}`));
        restore(got.export, got.extras);
      });
      input.click();
    }
  }

  // ---- a setting changed ------------------------------------------------------------------------------
  function changed(key, value) {
    save({ [key]: value });
    switch (key) {
      case "lang":
        setLang(value);
        break;
      case "speed":
        director.setSpeed(dirSpeed());
        break;
      case "faces":
        redrawFaces(ALL_CODES);
        stage.render();
        break;
      case "talk":
        talk.setMode(value);
        break;
      case "tutor":
        if (value && prefs.muted) tutor.unmute(); // the setting un-mutes (6.1)
        showTutor();
        break;
      case "surface":
        stage.setSurface(chooseSurface({ chosen: value === "random" ? null : value, saved: null }));
        break;
      case "leftHanded":
        ui.root.classList.toggle("left", value);
        if (S) director.relayout();
        break;
      default:
    }
    if (wasm) refresh();
  }

  // Faces drawn again (a size chosen; the courts when the language changes).
  function redrawFaces(codes) {
    const fresh = faceTextures({ width, anisotropy, codes, lang: getLang(), size: faceSize() });
    const old = Object.keys(fresh).map((c) => textures.faces[c]);
    deck.setFaces(fresh);
    Object.assign(textures.faces, fresh);
    old.forEach((x) => x?.dispose?.());
  }

  // The language: every drawn string again, the faces' courts, the HUD rebuilt
  // with its numbers kept (7.5.2).
  onLang((lang) => {
    document.documentElement.lang = lang;
    document.title = t("ui.title");
    canvas.setAttribute("aria-label", t("ui.stage"));
    ui.relabel();
    mirror.relabel();
    redrawFaces(COURT_CODES);
    const totals = hud.totals();
    ui.hudSlot.replaceChildren();
    game = countGame(hudWords());
    hud = createHud(ui.hudSlot, game, { later: (ms, fn) => director.at(ms, fn), reduced: () => calm() });
    const ledger = countLedger(game, []);
    ledger.count = { you: totals.you, opp: totals.opp };
    hud.reset({ hands: [ledger], live: null });
    hud.node.classList.toggle("edge", edgeOf(totals).tick); // the tick (2.5.4) is on the old HUD's node
    if (wasm) {
      refresh();
      showTour();
      ui.closeDialog("extend"); // asked again, in the new language, by refresh
      ui.closeDialog("announce");
    }
    // An open page is drawn again in the new language; the lists are closed.
    ui.closeDialog("help");
    ui.closeDialog("review");
    if (tutor.current() && ui.dialogOpen("page")) openPage(tutor.current(), { help: pageHelp });
    stage.render();
  });
  document.documentElement.lang = getLang();
  document.title = t("ui.title");

  // ---- input -----------------------------------------------------------------------------------------------
  let afterDrag = false; // the click that follows a drag is not a tap
  canvas.addEventListener("click", (e) => {
    if (afterDrag) return (afterDrag = false);
    tapped(director.pick(e.clientX, e.clientY), { x: e.clientX, y: e.clientY });
  });
  document.getElementById("overlay").addEventListener("click", (e) => {
    if (e.target.closest("button, .dialog, .slate, .settings, .hud")) return;
    if (director.busy() || pending > 0) director.skip();
  });
  // A row of cards wider than the screen scrolls.
  let dragFrom = null;
  const clampScroll = (v) => {
    const range = handRange(S ? S.model.hand.length : 0, zonesNow().yourHand, view().fill);
    return Math.max(-range, Math.min(range, v));
  };
  canvas.addEventListener("pointerdown", (e) => {
    const slot = director.pick(e.clientX, e.clientY);
    dragFrom = { x: e.clientX, y: e.clientY, scroll: clampScroll(sel.scroll), code: slot?.zone === "your-hand" || slot?.zone === "your-packets" ? slot.code : null };
  });
  canvas.addEventListener("pointercancel", () => (dragFrom = null)); // the browser took the touch
  canvas.addEventListener("pointermove", (e) => {
    if (!dragFrom || zonesNow().yourHand.fan || director.busy()) return;
    const dx = e.clientX - dragFrom.x;
    if (dragKind(dx, e.clientY - dragFrom.y) !== "scroll") return;
    const cm = (dx / window.innerWidth) * 34;
    sel = { ...sel, scroll: clampScroll(dragFrom.scroll - cm), at: null }; // the row moved: the spot is no longer the card's
    director.rearrange();
  });
  // A lit card drawn up toward the trick is played (3.1.2).
  window.addEventListener("pointerup", (e) => {
    const from = dragFrom;
    dragFrom = null;
    if (!from?.code || !wasm || wasm.prompt !== "you" || wasm.phase !== "play" || director.busy() || pending > 0) return;
    const kind = dragKind(e.clientX - from.x, e.clientY - from.y);
    if (kind !== "tap") {
      afterDrag = true;
      setTimeout(() => (afterDrag = false), 80); // no click may follow on touch
    }
    if (kind === "play" && lit.has(from.code)) {
      sel = { ...sel, chosen: null };
      commit(from.code);
    }
  });
  canvas.addEventListener(
    "wheel",
    (e) => {
      if (zonesNow().yourHand.fan) return;
      sel = { ...sel, scroll: clampScroll(clampScroll(sel.scroll) + e.deltaY * 0.03 + e.deltaX * 0.03), at: null };
      director.rearrange();
    },
    { passive: true },
  );

  const handCodes = () => (S ? S.model.hand.map((c) => c.code) : []);
  const coverCodes = () => (S ? S.model.packets[human].map((p) => p.cover).filter(Boolean) : []);
  const focusList = (row) => (row === "hand" ? handCodes() : coverCodes());
  const focusedCode = () => (focus ? focusList(focus.row)[focus.index] ?? null : null);
  // The keyboard's focus is on a card of the mirror (Tab, or an arrow).
  function mirrorFocused(row, index) {
    if (focus?.row === row && focus.index === index) return;
    focus = { row, index };
    director.redecorate();
  }
  function moveFocus(row, by, absolute = null) {
    const list = focusList(row);
    if (!list.length) return;
    let index = focus?.row === row ? focus.index : -1;
    if (index < 0) {
      index = list.findIndex((c) => lit.has(c));
      if (index < 0) return; // nothing is lit: the focus does not land on a dimmed card
    } else index = absolute !== null ? Math.min(absolute, list.length - 1) : Math.max(0, Math.min(list.length - 1, index + by));
    focus = { row, index };
    mirror.focusItem(row, index); // the browser reads the card's label
    director.redecorate();
  }
  function chooseCover(n) {
    const code = S?.model.packets[human][n - 1]?.cover;
    if (code) moveFocus("packets", 0, coverCodes().indexOf(code));
  }
  // Enter, Space or a press on a card of the mirror: choose it, then play it.
  function pickCard(code) {
    if (!code || !wasm || wasm.phase !== "play" || director.busy() || pending > 0) return;
    if (!lit.has(code) || wasm.prompt !== "you") return askRefusal(code);
    if (sel.chosen === code) return commit(code);
    sel = { ...sel, chosen: code, at: null };
    ui.live(t("sr.chosen", { card: cardName(code) }));
    refresh();
    director.rearrange();
  }

  window.addEventListener("keydown", (e) => {
    if (e.defaultPrevented || e.ctrlKey || e.metaKey || e.altKey) return;
    if (e.key === "Escape") {
      if (ui.dialogOpen("page")) dismissPage(true);
      else if (ui.dialogOpen("help") || ui.dialogOpen("review")) {
        ui.closeDialog("help");
        ui.closeDialog("review");
      } else if (ui.dialogOpen("announce")) ui.closeDialog("announce");
      else if (ui.dialogOpen("keys")) ui.closeDialog("keys");
      else if (!ui.slateRoot.hidden) toggleSlate();
      else if (sel.chosen) {
        sel = { ...sel, chosen: null };
        refresh();
        director.rearrange();
      }
      return;
    }
    if (e.target instanceof Element && e.target.closest("input, select, textarea, .settings, .slate")) return;
    const key = e.key.length === 1 ? e.key.toLowerCase() : e.key;
    if ((key === "Enter" || key === " ") && e.target instanceof Element && e.target.closest("button")) return; // the button's own press
    if (key === "?") {
      ui.dialog({ id: "keys", title: t("keys.title"), body: ui.el("ul", { class: "keys" }, ...["play", "arrows", "packets", "announce", "take", "weiter", "slate", "hint", "mute", "where", "help", "esc"].map((k) => ui.el("li", {}, t(`keys.${k}`)))), buttons: [{ label: t("ui.close"), act: "close", focus: true }] });
      return;
    }
    if (ui.dialogOpen()) return;
    if (!wasm) return;
    if (key === "s") return toggleSlate();
    if (key === "h") return askHint();
    if (key === "m") return tutor.toggleMute();
    if (key === "i") return ui.live(where(wasm));
    const calm_ = director.busy() || pending > 0;
    if (key === "t" && !calm_ && wasm.legal.some((a) => a.action === "take")) return send({ action: "take" });
    if (key === "w" && !calm_ && wasm.legal.some((a) => a.action === "pass")) return send({ action: "pass" });
    if (key === "a" && !calm_ && wasm.phase === "play" && wasm.prompt === "you") {
      const first = [...heldHonours()][0];
      if (first) onAct("announce", { honour: first });
      return;
    }
    if (director.busy() || wasm.phase !== "play") return;
    if (key === "ArrowLeft") moveFocus(focus?.row ?? "hand", -1);
    else if (key === "ArrowRight") moveFocus(focus?.row ?? "hand", 1);
    else if (key === "Home") moveFocus(focus?.row ?? "hand", 0, 0);
    else if (key === "End") moveFocus(focus?.row ?? "hand", 0, 999);
    else if (["1", "2", "3"].includes(key)) chooseCover(Number(key));
    else if (key === "Enter" || key === " ") {
      const code = focusedCode();
      if (!code) return;
      e.preventDefault();
      pickCard(code);
    }
  });
  // The table is framed between the score and the controls (kit/js/framing.js).
  function fitStrips() {
    const hud = document.querySelector(".info").getBoundingClientRect();
    const controls = ui.controls.getBoundingClientRect();
    // Upright, the Kibitzer's tabs lie in a row above the controls (tutor.css).
    const tabs = window.matchMedia("(max-aspect-ratio: 85/100)").matches ? ui.root.querySelector(".tutor-row").getBoundingClientRect().top : Infinity;
    stage.setStrips({ top: hud.bottom + 6, foot: window.innerHeight - Math.min(controls.top, tabs) + 6, raised: 0 });
  }
  new ResizeObserver(() => {
    fitStrips();
    sel = { ...sel, at: null }; // cards move on a resize
    if (S) director.relayout();
  }).observe(document.getElementById("overlay"));
  fitStrips();
  stage.onReframe = () => S && director.relayout();

  stopped = () => ui.dialog({ id: "error", title: t("error.opponent-refused"), body: "", buttons: [{ label: t("ui.close"), act: "close", focus: true }] });
  if (info.notice === "unavailable") ui.chip(t("error.no-storage"), { x: window.innerWidth / 2, y: 100, ms: 5000 });
  if (info.notice === "newer") ui.chip(t("chip.newer"), { x: window.innerWidth / 2, y: 100, ms: 5000 });

  // ---- the start ---------------------------------------------------------------------------------------------
  const kept = TEST ? null : loadSitting();
  if (TEST) newSitting({ touren: Number(params.get("touren")) || prefs.touren });
  else if (kept) {
    ui.dialog({
      id: "welcome",
      title: t("ui.welcome.title"),
      body: "",
      buttons: [
        { label: t("ui.welcome.continue"), act: "continue", primary: true, focus: true },
        { label: t("ui.welcome.new"), act: "new" },
      ],
    });
  } else askNew();

  // ---- for the browser test --------------------------------------------------------------------------------------
  if (TEST || params.has("hook")) window.strohmandeln3d = {
    engine,
    state: () => wasm,
    model: () => S?.model,
    prefs: () => prefs,
    log: () => log,
    lit: () => [...lit],
    faces: () => director.placement().filter((m) => m.code).map((m) => m.code),
    shown: () => director.meshes.map((m) => m.userData.code),
    hidden: () => engine.audit().hidden,
    busy: () => director.busy() || pending > 0 || stage.reframing(),
    timing: () => lastTiming,
    debug: () => ({ pending, dbusy: director.busy(), reframing: stage.reframing(), held: director.held(), stepping, prompt: wasm?.prompt, events: log.length }),
    said: () => ui.said(),
    tutor: () => ({ page: tutor.current(), muted: prefs.muted, seen: prefs.seen, note: ui.root.querySelector(".tutor-note")?.dataset.text ?? "", chip: reviewChip }),
    openPage: (key) => tutor.help(key),
    live: () => ui.lived(),
    hud: () => ({ totals: hud.totals(), shown: hud.shown(), idle: hud.idle() }),
    zoneRects,
    cardRects: cardRectsOf,
    placement: () => director.placement().map((m) => ({ zone: m.zone, key: m.key, code: m.code, index: m.index, packet: m.packet })),
    screenPoint: (code) => director.screenPoint(code),
    pickAt: (x, y) => {
      const m = director.pick(x, y);
      return m ? { zone: m.zone, code: m.code } : null;
    },
    selection: () => sel.chosen,
    skip: () => director.skip(),
    tick: (ms) => director.tick(ms),
    clock: () => director.clock(),
    meshInfo: () => director.meshes.map((m) => ({ x: m.position.x, y: m.position.y, z: m.position.z, code: m.userData.code })),
    newSitting,
    act: (action) => send(action),
    speed: (name) => changed("speed", name),
    step,
    sittingOver: () => sittingOver,
    stage: () => ({ portrait: stage.portrait, strips: stage.strips() }),
  };
}

main().catch((error) => {
  console.error(error);
  const loading = document.getElementById("loading");
  if (loading) loading.textContent = t("error.bad-config");
});
