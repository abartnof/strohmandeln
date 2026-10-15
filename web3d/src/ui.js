// The overlay: the 2D surfaces over the table (docs/TABLE3D.md 1, 3): the
// prompt and the refusal under the hand, the move bar (bid, play, announce,
// extension), the tags that follow cards (hidden counts, markers, public
// marks), chips that rise and fade, the result banner, dialogue boxes, the
// dialogs and the screen reader's live region.
//
// It draws what it is given and reports what is pressed (`onAct(act, data)`);
// it holds no rules and no literal text: every string is a key given to `t`.

const el = (tag, attrs = {}, ...children) => {
  const node = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs)) {
    if (v === null || v === undefined) continue;
    if (k === "class") node.className = v;
    else node.setAttribute(k, v);
  }
  node.append(...children.filter((c) => c !== null && c !== undefined && c !== false));
  return node;
};

export function createUi(root, { t, onAct }) {
  root.replaceChildren();
  const q = (s) => root.querySelector(s);
  root.append(
    el("div", { class: "tags", "aria-hidden": "true" }),
    el("div", { class: "floats", "aria-hidden": "true" }),
    el("div", { class: "info" }, el("div", { class: "hud-slot" }), el("p", { class: "tour-line" })),
    el("div", { class: "slate-root" }),
    el("div", { class: "settings-root" }),
    el("div", { class: "banner", role: "status", hidden: "" }),
    el("div", { class: "say-layer" }),
    el(
      "section",
      { class: "controls" },
      el("p", { class: "prompt", "data-prompt": "" }),
      el("p", { class: "refusal", role: "status" }),
      el("p", { class: "aid-line", hidden: "" }),
      el("div", { class: "move-bar", role: "group", "data-i18n-label": "ui.movebar", hidden: "" }),
    ),
    // Tab order (3.10.1): the move bar, the cards, then the slate tab and the settings.
    el(
      "div",
      { class: "top-right" },
      el("button", { class: "tab slate-tab", type: "button", "data-act": "slate", "aria-expanded": "false" }),
      el("button", { class: "tab cog", type: "button", "data-act": "settings" }),
    ),
    // The Kibitzer's tab (docs/TABLE3D.md 6): Help, the mute, the review chip, his note.
    el(
      "div",
      { class: "tutor" },
      el("p", { class: "tutor-note", role: "status", hidden: "" }),
      el(
        "div",
        { class: "tutor-row" },
        el("button", { class: "tutor-btn", type: "button", "data-act": "tutor" }),
        el("button", { class: "tutor-mute", type: "button", "data-act": "mute", "aria-pressed": "false" }),
        el("button", { class: "tutor-review", type: "button", "data-act": "review", hidden: "" }),
      ),
    ),
    el("p", { class: "sr-live", "aria-live": "polite", "aria-atomic": "true" }),
    el("div", { class: "dialogs" }),
  );
  const bar = q(".move-bar");
  const prompt = q(".prompt");
  const refusal = q(".refusal");
  const aid = q(".aid-line");
  const banner = q(".banner");
  const live = q(".sr-live");
  const floats = q(".floats");
  const tagLayer = q(".tags");
  const sayLayer = q(".say-layer");
  const dialogs = q(".dialogs");

  root.addEventListener("click", (e) => {
    const button = e.target instanceof Element ? e.target.closest("[data-act]") : null;
    if (!button || button.disabled || button.getAttribute("aria-disabled") === "true" && !button.dataset.ask) return;
    onAct(button.dataset.act, { ...button.dataset });
  });

  // Text set by key follows the language: relabel() redraws every static label.
  function relabel() {
    for (const node of root.querySelectorAll("[data-i18n-label]")) node.setAttribute("aria-label", t(node.dataset.i18nLabel));
    for (const node of root.querySelectorAll("[data-i18n]")) node.textContent = t(node.dataset.i18n);
    q(".slate-tab").textContent = t("ui.menu.slate");
    q(".cog").textContent = t("ui.menu.settings");
    drawTutor();
  }
  // The Kibitzer's tab, in the language: the tab opens Help, "§ 13" mutes him,
  // the chip offers the review of the deal or of the sitting.
  let tutor = { on: true, muted: false, review: null };
  function drawTutor() {
    const tab = q(".tutor-btn");
    tab.textContent = `✎ ${t("tutor.tab")}`;
    tab.title = t("tutor.tab.title");
    const mute = q(".tutor-mute");
    mute.hidden = !tutor.on;
    mute.textContent = t("tutor.mute.label");
    mute.title = t(tutor.muted ? "tutor.unmute.title" : "tutor.mute.title");
    mute.setAttribute("aria-label", mute.title);
    mute.setAttribute("aria-pressed", String(tutor.muted));
    const chip = q(".tutor-review");
    chip.hidden = !(tutor.on && !tutor.muted && tutor.review);
    chip.textContent = t(tutor.review === "sitting" ? "review.chip.sitting" : "review.chip");
    chip.title = t("review.offer");
  }
  relabel();

  const api = {
    root,
    hudSlot: q(".hud-slot"),
    slateRoot: q(".slate-root"),
    settingsRoot: q(".settings-root"),
    controls: q(".controls"),
    relabel,
    el,

    setTour(text) {
      q(".tour-line").textContent = text ?? "";
    },
    setPrompt(text) {
      if (prompt.textContent !== (text ?? "")) prompt.textContent = text ?? "";
    },
    setRefusal(text) {
      refusal.textContent = text ?? "";
      refusal.hidden = !text;
    },
    // The tutor's tab: `on` (the setting), `muted`, and the review chip,
    // `review`: null, "deal" or "sitting".
    setTutor(next) {
      tutor = { ...tutor, ...next };
      drawTutor();
    },
    // The Kibitzer's note by his tab: a ruled paper note, not the opponent's box.
    kibitzNote(text) {
      const note = q(".tutor-note");
      note.hidden = !text;
      note.textContent = text ?? "";
      note.dataset.text = text ?? "";
      return note;
    },
    // One line of help under the prompt; `more` adds lines under it (the hint's
    // rule and odds), each its own block so a long hint wraps tidily.
    setAid(text, more = []) {
      aid.hidden = !text;
      aid.replaceChildren(text ?? "", ...more.map((m) => el("span", { class: "aid-more" }, m)));
    },

    // The move bar. `groups`: [{ kind, buttons: [{ act, label, enabled, data, key, focus }] }].
    // Redrawn only when what it offers changes, so a button keeps the keyboard's focus.
    setBar(groups, { left = false } = {}) {
      const signature = JSON.stringify([groups, left]);
      bar.hidden = groups.length === 0;
      if (bar.dataset.sig === signature) return;
      bar.dataset.sig = signature;
      bar.classList.toggle("left", left);
      const ordered = left ? groups.map((g) => ({ ...g, buttons: [...g.buttons].reverse() })) : groups;
      bar.replaceChildren(
        ...ordered.map((g) =>
          el(
            "div",
            { class: `place ${g.kind}` },
            ...g.buttons.map((b) =>
              el("button", { class: `big ${b.kind ?? ""}`, type: "button", "data-act": b.act, "data-ask": b.ask ? "1" : null, ...Object.fromEntries(Object.entries(b.data ?? {}).map(([k, v]) => [`data-${k}`, v])), "aria-disabled": b.enabled ? null : "true", "data-key": b.key ?? null, title: b.title ?? null }, b.label),
            ),
          ),
        ),
      );
    },
    barButtons: () => [...bar.querySelectorAll("button")],

    // A chip that rises over a point of the screen and fades.
    chip(text, { x, y, kind = "", ms = 1600, stay = false, id = null }) {
      if (id) floats.querySelector(`[data-id="${id}"]`)?.remove();
      const chip = el("div", { class: `chip ${kind}`, "data-id": id }, text);
      chip.style.left = `${x}px`;
      chip.style.top = `${y}px`;
      floats.append(chip);
      if (!stay) setTimeout(() => chip.classList.add("gone"), ms);
      if (!stay) setTimeout(() => chip.remove(), ms + 600);
      return chip;
    },
    clearChips(kind = null) {
      for (const c of floats.querySelectorAll(".chip")) if (!kind || c.classList.contains(kind)) c.remove();
    },

    // Tags that follow cards: [{ id, text, x, y, kind, title }], the pool kept by id.
    tags(list) {
      const have = new Map([...tagLayer.children].map((n) => [n.dataset.id, n]));
      const keep = new Set();
      for (const tag of list) {
        keep.add(tag.id);
        let node = have.get(tag.id);
        if (!node) {
          node = el("div", { class: `tag ${tag.kind}`, "data-id": tag.id });
          tagLayer.append(node);
        }
        if (node.textContent !== (tag.text ?? "")) node.textContent = tag.text ?? "";
        node.style.transform = `translate(${tag.x.toFixed(1)}px, ${tag.y.toFixed(1)}px) translate(-50%, -50%)`;
        if (tag.size) node.style.fontSize = `${tag.size}px`;
        if (tag.width) {
          node.style.width = `${tag.width.toFixed(1)}px`;
          node.style.height = `${(tag.height ?? 3).toFixed(1)}px`;
        }
        if (tag.title) node.title = tag.title;
      }
      for (const [id, node] of have) if (!keep.has(id)) node.remove();
    },

    // The result banner: a title and rows.
    showBanner({ title, rows = [], foot = null }) {
      banner.hidden = false;
      banner.replaceChildren(el("h2", {}, title), ...rows.map((r) => el("p", { class: "row" }, el("span", {}, r[0]), el("b", {}, r[1]))), foot ? el("p", { class: "foot" }, foot) : null);
      banner.classList.add("in");
    },
    hideBanner() {
      banner.hidden = true;
      banner.classList.remove("in");
    },
    bannerShown: () => !banner.hidden,

    // A dialogue box by its speaker: `place` is the first of `places`
    // (kit/js/dialogue.js choosePlace) that covers nothing it must not.
    sayBox(who, text, place, key = who) {
      sayLayer.querySelector(`[data-who="${key}"]`)?.remove();
      const box = el("div", { class: `say ${who}`, "data-who": key, "data-text": text }, text);
      sayLayer.append(box);
      return box;
    },
    sayLayer,
    hush() {
      sayLayer.replaceChildren();
    },
    said: () => [...sayLayer.children].map((n) => n.dataset.text),

    // A dialog over the table. `body` is a node or a string; `buttons`: [{ label, act, primary }].
    dialog({ title, body, buttons = [], id = "dialog", onClose = null, wide = false }) {
      api.closeDialog(id);
      const box = el(
        "div",
        { class: `dialog ${wide ? "wide" : ""}`, role: "dialog", "aria-modal": "true", "aria-label": title, "data-dialog": id },
        el("h2", {}, title),
        typeof body === "string" ? el("p", {}, body) : body,
        el("div", { class: "actions" }, ...buttons.map((b) => el("button", { class: `big ${b.primary ? "primary" : ""}`, type: "button", "data-dialog-act": b.act, "data-focus": b.focus ? "1" : null }, b.label))),
      );
      const shade = el("div", { class: "shade", "data-shade": id }, box);
      shade.addEventListener("click", (e) => {
        const b = e.target instanceof Element ? e.target.closest("[data-dialog-act]") : null;
        if (b) onAct(`dialog:${id}:${b.dataset.dialogAct}`, {});
        else if (e.target === shade && onClose) onClose();
      });
      dialogs.append(shade);
      (box.querySelector("[data-focus]") ?? box.querySelector("button"))?.focus();
      return shade;
    },
    closeDialog(id = "dialog") {
      dialogs.querySelector(`[data-shade="${id}"]`)?.remove();
    },
    dialogOpen: (id = null) => Boolean(id ? dialogs.querySelector(`[data-shade="${id}"]`) : dialogs.children.length),

    // For the screen reader: one sentence at a time.
    live(text) {
      live.textContent = "";
      setTimeout(() => (live.textContent = text), 50);
    },
    lived: () => live.textContent,
    setSlateOpen(open) {
      q(".slate-tab").setAttribute("aria-expanded", String(open));
    },
  };
  return api;
}
