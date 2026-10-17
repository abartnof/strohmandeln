"""The browser test's checks for the slate, the talk, the settings, the language
switch and the persistence (docs/TABLE3D.md 10.4, 10.5). The tutor's checks are
the tutor's own. Imported by browser.py."""

import json
import re
import tempfile
from pathlib import Path

from browser import NO_DRAW, PAGE, ROOT, WORDS, STRINGS, check_offline, close_all, fail_if, intersects, js, rest, state, still
from browser_scenarios import (
    FAST,
    choose,
    count_run,
    drive,
    has,
    mrest,
    open_page,
    take_if_bidding,
    to_last_trick,
    with_prefs,
    until_over,
    watch_chips,
    chips_seen,
    wire,
)


def t(key, lang="en"):
    return STRINGS[key][lang]


CALLS = set(re.findall(r'"([a-z0-9-]+)"', re.search(r"export const CALLS = new Set\(\[(.*?)\]\)", (ROOT / "web3d/src/talk.js").read_text(), re.S).group(1)))
TEXT_KEYS = {lang: {} for lang in ("en", "de")}
for _key, _text in WORDS["texts"].items():
    for _lang in ("en", "de"):
        TEXT_KEYS[_lang].setdefault(_text[_lang], []).append(_key)
GROUP_OF = {}
for _group, _keys in WORDS["groups"].items():
    for _key in _keys:
        GROUP_OF.setdefault(_key, []).append(_group)


def groups_of(text, lang="en"):
    """The groups a spoken line may belong to."""
    return sorted({g for k in TEXT_KEYS[lang].get(text, []) for g in GROUP_OF.get(k, [])})


def watch_say(page):
    """Record each dialogue box as it is put up, with the table's clock and the deal count."""
    page.evaluate(
        """() => { window.__say = [];
        new MutationObserver((ms) => ms.forEach((m) => m.addedNodes.forEach((n) => {
          if (n.dataset && n.dataset.text) { const h = window.strohmandeln3d; const s = h.state();
            window.__say.push({ text: n.dataset.text, who: n.dataset.who, clock: h.clock(), deals: h.log().filter((e) => e.type === 'deal_start').length }); }
        }))).observe(document.querySelector('.say-layer'), { childList: true }); }"""
    )


def heard(page):
    return page.evaluate("window.__say")


def goto(page, query):
    page.goto(f"{PAGE.as_uri()}?{query}&lowfx=1&level=1", wait_until="commit", timeout=120_000)
    page.wait_for_function("window.strohmandeln3d !== undefined", timeout=180_000)


# Chips fade in a moment: record each as it is put up, from the page's start.
CHIP_WATCH = """window.__chips = [];
new MutationObserver((ms) => ms.forEach((m) => m.addedNodes.forEach((n) => n.classList && n.classList.contains('chip') && window.__chips.push(n.textContent)))).observe(document, { childList: true, subtree: true });"""


def slate_open(page):
    return page.evaluate("!document.querySelector('.slate-root').hidden")


# ---- 10.4 slate ------------------------------------------------------------------------------------


def check_slate_open_close(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)
    fail_if(slate_open(page), failures, "slate_open_close: the slate is open during play")
    page.locator(".slate-tab").click()
    fail_if(not slate_open(page), failures, "slate_open_close: the tab did not open the slate")
    fail_if(page.locator(".slate-tab").get_attribute("aria-expanded") != "true", failures, "slate_open_close: aria-expanded is not true when open")
    page.locator(".slate-tab").click(force=True)
    fail_if(slate_open(page), failures, "slate_open_close: the tab did not fold the slate")
    page.keyboard.press("s")
    fail_if(not slate_open(page), failures, "slate_open_close: S did not open the slate")
    page.keyboard.press("Escape")
    fail_if(slate_open(page), failures, "slate_open_close: Esc did not fold the slate")
    page.keyboard.press("s")
    box = page.locator(".slate-root").bounding_box()
    page.mouse.click(box["x"] + 2, box["y"] + 2)
    fail_if(slate_open(page), failures, "slate_open_close: a tap outside did not fold the slate")
    # It opens itself at a Tour's end (and, slateAuto being on, at nothing else): sample at every rest.
    page.keyboard.press("s") if slate_open(page) else None
    opened_in_play = []

    def until(s, log):
        if s["phase"] == "play" and slate_open(page):
            opened_in_play.append(s["deals_counted"])
        return any(e["type"] == "tour_end" for e in log)

    ok = drive(page, until)
    fail_if(not ok, failures, "slate_open_close: no Tour ended")
    fail_if(opened_in_play, failures, f"slate_open_close: the slate opened itself during play (deals {opened_in_play[:3]})")
    if ok:
        page.wait_for_function("!document.querySelector('.slate-root').hidden", timeout=30_000)
    check_offline(page, failures, "slate_open_close: ")


def check_slate_rows(browser, failures):
    # A draw doubles the next deal.
    page = open_page(browser, "seed=2&speed=instant&human=1")
    ok = drive(page, has("doubling_applied"), policy="weiter")
    fail_if(not ok, failures, "slate_rows: no doubled deal in the seeded sitting")
    if ok:
        rest(page)
        log = js(page, "log()")
        scored = [e for e in log if e["type"] == "game_scored"]
        rows = page.locator(".slate [data-deal]")
        fail_if(not scored, failures, "slate_rows: no scored deal in the log, so no row was compared")
        fail_if(page.locator(".slate-draw .slate-note").count() < 1, failures, "slate_rows: the draw is not marked")
        fail_if(page.locator(".slate-row .slate-double").count() < 1, failures, "slate_rows: no x2 on the doubled deal")
        fail_if(rows.count() != len(scored) + 1, failures, f"slate_rows: {rows.count()} rows for {len(scored)} scored deals and one live")
        human = state(page)["human"]
        for i, e in enumerate(scored):
            row = rows.nth(i)
            causes = [c for c in e["credits"]]
            for c in causes:
                want = t("slate.entry").format(cause=t(f"slate.cause.{c['cause']}"), n=c["points"]).replace("{cause}", "")
            conto = row.locator(".slate-conto").all_inner_texts()
            want_conto = [str(e["contos"][human]), str(e["contos"][1 - human])]
            fail_if(conto != want_conto, failures, f"slate_rows: row {i + 1} Contos {conto} vs engine {want_conto}")
            texts = " ".join(row.locator(".slate-credit").all_inner_texts())
            for c in e["credits"]:
                fail_if(t(f"slate.cause.{c['cause']}") not in texts, failures, f"slate_rows: row {i + 1} lacks the cause {c['cause']}")
        foot = page.locator(".slate-balance").inner_text()
        s = state(page)
        balance = s["contos"][human] - s["contos"][1 - human]
        fail_if(f"{balance:+d}".replace("-", "−") not in foot and str(balance) not in foot, failures, f"slate_rows: the foot says {foot!r} for a balance of {balance}")
        fail_if(page.locator(".slate-board").get_attribute("title") != t("slate.tooltip"), failures, "slate_rows: the tooltip is not the reconstruction note")
        sheet = page.locator(".slate").inner_text()
        fail_if(sheet.count(t("slate.tooltip")) != 0, failures, "slate_rows: the borrowed note is printed in the sheet")
    close_all(browser)
    # A throw-in, in German: "eingeworfen".
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    context.add_init_script("try { localStorage.setItem('strohmandeln.prefs', JSON.stringify({ v: 1, both_pass: 'throw_in', lang: 'de' })); } catch (e) {}")
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=1&speed=instant&human=0")
    rest(page)
    ok = drive(page, has("thrown_in"), policy="weiter")
    fail_if(not ok, failures, "slate_rows: no throw-in")
    if ok:
        rest(page)
        fail_if(page.locator(".slate-thrown_in .slate-note").count() < 1 or page.locator(".slate-thrown_in .slate-note").first.inner_text() != t("slate.thrown-in", "de"), failures, "slate_rows: the thrown-in row does not say eingeworfen")


def check_slate_points_and_settlement_note(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    ok = drive(page, has("game_scored"))
    fail_if(not ok, failures, "slate_points: no deal scored")
    if ok:
        rest(page)
        e = next(e for e in js(page, "log()") if e["type"] == "game_scored")
        human = state(page)["human"]
        row = page.locator(".slate [data-deal]").first
        for seat, cls in ((human, ".slate-cell:nth-of-type(2)"), (1 - human, ".slate-cell:nth-of-type(3)")):
            total = sum(c["points"] for c in e["credits"] if c["seat"] == seat)
            cell = row.locator(cls)
            text = cell.inner_text()
            if total:
                fail_if(f"{total * 2} kr" not in text, failures, f"slate_points: seat {seat}'s total {total} Pt should show {total * 2} kr; the cell says {text!r}")


# ---- 10.4 talk ----------------------------------------------------------------------------------------


def talk_page(browser, extra=""):
    """A sitting at Normal speed on the test's clock: the instant speed leaves the talk out."""
    page = open_page(browser, f"seed=7&speed=normal&human=1&manual{extra}", wait=False)
    watch_say(page)
    mrest(page, step=400)
    return page


def run_ticks(page, ms, step=250):
    page.evaluate(f"(async () => {{ for (let t = 0; t < {ms}; t += {step}) {{ window.strohmandeln3d.tick({step}); await new Promise((r) => setTimeout(r, 0)); }} }})()")


def check_talk_events(browser, failures):
    page = talk_page(browser)
    ok = drive(page, until_over(), manual=True, step=400)
    fail_if(not ok, failures, "talk_events: the sitting did not end")
    run_ticks(page, 10_000)
    lines = heard(page)
    log = js(page, "log()")
    seen_groups = set()
    for line in lines:
        gs = groups_of(line["text"])
        fail_if(not gs, failures, f"talk_events: {line['text']!r} is no line of the bank")
        seen_groups.update(gs)
    wants = [
        ("sitting_start", ["greet", "touren-3", "touren-6"]),
        ("deal_start", ["deal-mine", "deal-yours", "cut"]),
        ("game_scored", ["result-won-mine", "result-won-yours", "result-lost-mine", "result-lost-yours", "draw"]),
        ("sitting_end", ["farewell", "settle-even", "settle-opp-pays", "settle-player-pays"]),
    ]
    for event, groups in wants:
        fail_if(not any(e["type"] == event for e in log), failures, f"talk_events: the sitting never had a {event}, so its lines are untested")
        if any(e["type"] == event for e in log):
            fail_if(not seen_groups & set(groups), failures, f"talk_events: {event} occurred but none of {groups} was said (heard {sorted(seen_groups)[:12]}; last lines {[(l['deals'], l['text'][:20]) for l in lines[-4:]]})")
    fail_if(not (seen_groups - CALLS), failures, "talk_events: a whole sitting at All said no chatter")


def said_in(page, goal, **kw):
    watch_say(page)
    drive(page, goal, **kw)
    rest(page)
    page.wait_for_timeout(500)
    return heard(page)


def check_talk_setting(browser, failures):
    counts = {}
    for mode in ("all", "calls", "none"):
        page = talk_page(browser, f"&talk={mode}")
        drive(page, has("game_scored"), manual=True, step=400)
        run_ticks(page, 8000)
        lines = heard(page)
        counts[mode] = len(lines)
        if mode == "none":
            fail_if(lines, failures, f"talk_setting: None said {len(lines)} lines")
        if mode == "calls":
            chatter = [l["text"] for l in lines if not set(groups_of(l["text"])) & CALLS]
            fail_if(chatter, failures, f"talk_setting: Calls only said chatter: {chatter[:3]}")
        # The chips and badges still appear with None.
        if mode == "none":
            fail_if(page.locator(".tag.badge").count() == 0 and page.locator(".tag.declarer").count() == 0, failures, "talk_setting: no badge with None")
        close_all(browser)
    fail_if(counts["all"] < counts["calls"] or counts["calls"] == 0, failures, f"talk_setting: counts {counts}")
    # The setting applies now: switch to None through the menu, and the next deal is silent.
    page = open_page(browser, "seed=7&speed=instant&human=1&talk=all")
    page.locator("[data-act=settings]").click()
    page.locator("[data-row=talk] [data-value=none]").click()
    page.locator(".settings-close").click()
    fail_if(js(page, "prefs()")["talk"] != "none", failures, "talk_setting: the menu did not set the talk")


def check_talk_result_after_count(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&manual", wait=False)
    watch_say(page)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    ok = to_last_trick(page)
    fail_if(not ok, failures, "talk_result_after_count: the sitting did not reach a last trick")
    if ok:
        count_run(page)
        timing = js(page, "timing()")
        count = timing["count"]
        lines = heard(page)
        after = [l for l in lines if l["clock"] >= timing["base"] + count["end"] - 5]
        results = [l for l in after if {"result-won-mine", "result-won-yours", "result-lost-mine", "result-lost-yours", "draw"} & set(groups_of(l["text"]))]
        fail_if(not results, failures, f"talk_result_after_count: no result line after the count (lines {[l['text'][:20] for l in lines]})")
        inside = [l for l in lines if timing["base"] + count["lines"][0] < l["clock"] < timing["base"] + count["end"] - 5]
        fail_if(inside, failures, f"talk_result_after_count: {len(inside)} lines during the count")


def check_talk_repeat(browser, failures):
    page = talk_page(browser)
    drive(page, until_over(), manual=True, step=400)
    run_ticks(page, 10_000)
    lines = heard(page)
    sig_of = lambda text: next((WORDS["texts"][k].get("sig", k) for k in TEXT_KEYS["en"].get(text, [])), text)
    bad = []
    for i, line in enumerate(lines):
        gs = groups_of(line["text"])
        if not gs or set(gs) & CALLS:
            continue
        sig = sig_of(line["text"])
        recent = {sig_of(l["text"]) for l in lines[:i] if line["deals"] - l["deals"] < 3}
        if sig not in recent:
            continue
        # A repeat in the window: a fault only if, whichever group it came from, another saying of that group was free.
        free = [{WORDS["texts"][k].get("sig", k) for k in WORDS["groups"][g]} - recent for g in gs]
        if all(free):
            bad.append((line["deals"], line["text"], sig, gs))
    # The talk's own record also holds the lines the speaker dropped, which the page never shows, so a
    # repeat here can be the talk running out of unheard lines; talk.test.js holds the exact rule. A run
    # of repeats, though, means the control is off.
    fail_if(len(bad) > max(1, len(lines) // 10), failures, f"talk_repeat: {len(bad)} of {len(lines)} lines repeated a saying heard in the last 3 deals: {bad[:3]}")
    fail_if(len(lines) < 10, failures, f"talk_repeat: only {len(lines)} lines in a whole sitting")


# ---- 10.5 settings -------------------------------------------------------------------------------------


def open_settings(page):
    # Dispatched: with the slate open it covers the cog, and the language is switched with it open.
    page.locator("[data-act=settings]").dispatch_event("click")
    page.wait_for_selector(".settings-root:not([hidden])", timeout=10_000)


def check_settings_rows(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    open_settings(page)
    # The rule rows are greyed mid-sitting and say so; touching one changes nothing.
    for key in ("both_pass", "values", "ultimo", "touren"):
        row = page.locator(f"[data-row={key}]")
        fail_if("settings-locked" not in (row.get_attribute("class") or ""), failures, f"settings_rules: {key} is not greyed mid-sitting")
        fail_if(row.locator(".settings-when").count() != 1, failures, f"settings_rules: {key} does not say when it applies")
        before = js(page, "prefs()")[key]
        row.locator("button:not([aria-checked=true])").first.click(force=True)
        fail_if(js(page, "prefs()")[key] != before, failures, f"settings_rules: {key} changed mid-sitting")
    # Every row that applies now changes the prefs, and is kept.
    now_rows = {"speed": "fast", "faces": "jumbo", "reducedMotion": "off", "talk": "calls", "lang": "de"}
    for key, value in now_rows.items():
        page.locator(f"[data-row={key}] [data-value={value}]").click()
        got = js(page, "prefs()")[key]
        fail_if(got != value, failures, f"settings_rows: {key} -> {value} gave {got!r}")
    for key in ("leftHanded", "tutor", "hints", "explain", "announceAuto", "slateAuto"):
        before = js(page, "prefs()")[key]
        page.locator(f"[data-row={key}] [data-value=toggle]").click()
        fail_if(js(page, "prefs()")[key] == before, failures, f"settings_rows: the {key} switch did not turn")
    fail_if(not page.evaluate("document.getElementById('overlay').classList.contains('left') || document.querySelector('.left') !== null"), failures, "settings_rows: left-handed did not mirror the table")
    stored = page.evaluate("JSON.parse(localStorage.getItem('strohmandeln.prefs'))")
    for key, value in now_rows.items():
        fail_if(stored.get(key) != value, failures, f"settings_rows: {key} was not kept ({stored.get(key)!r})")
    surfaces = page.locator("[data-row=surface] button").count()
    fail_if(surfaces < 2, failures, "settings_rows: the table top offers no choice")
    check_offline(page, failures, "settings_rows: ")


def check_settings_next_sitting(browser, failures):
    prefs = json.dumps({"v": 1, "values": "1/2/3", "ultimo": 2, "both_pass": "throw_in", "touren": 6})
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    context.add_init_script(f"try {{ localStorage.setItem('strohmandeln.prefs', JSON.stringify({prefs})); }} catch (e) {{}}")
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&speed=instant&human=1")
    rest(page)
    start = next(e for e in js(page, "log()") if e["type"] == "sitting_start")
    settings = start["settings"]
    fail_if(settings["values"] != "1/2/3" or settings["ultimo"] != 2 or settings["both_pass"] != "throw_in", failures, f"settings_rules: the stored rules did not reach the sitting: {settings}")
    fail_if(start["touren"] != 6, failures, f"settings_rules: the stored length gave {start['touren']} Touren")
    page.locator(".slate-tab").click()
    rules = page.locator(".slate-rules").inner_text()
    fail_if("1/2/3" not in rules or "Ultimo 2" not in rules, failures, f"settings_rules: the slate header says {rules!r}")


def check_skill_speed(browser, failures):
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    open_settings(page)
    page.locator("[data-row=speed] [data-value=fast]").click()
    fail_if(js(page, "prefs()")["speed"] != "fast", failures, "skill_speed: the speed did not apply")
    skill = page.locator("[data-row=skill]")
    fail_if(skill.count() != 1, failures, "skill_speed: no skill dial")
    options = skill.locator("button").all_inner_texts()
    fail_if(options != ["1", "1½", "2", "2½", "3", "3½", "4"], failures, f"skill_speed: the dial offers {options}")
    # The dial is a next-sitting setting: locked here, so it is set and read before the sitting.
    goto(page, "seed=7&human=1")
    page.wait_for_function("window.strohmandeln3d.prefs().speed === 'fast'", timeout=30_000)
    fail_if(js(page, "prefs()")["speed"] != "fast", failures, "skill_speed: the speed was not kept across a reload")
    context.close()
    prefs = json.dumps({"v": 1, "skill": 2})
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    context.add_init_script(f"try {{ localStorage.setItem('strohmandeln.prefs', JSON.stringify({prefs})); }} catch (e) {{}}")
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    open_settings(page)
    selected = page.locator("[data-row=skill] [aria-checked=true]").all_inner_texts()
    fail_if(selected != ["2"], failures, f"skill_speed: the stored skill shows as {selected}")


# ---- language -------------------------------------------------------------------------------------------


def visible_strings(page):
    """The visible text of the page's own controls, line by line."""
    return page.evaluate(
        """() => { const out = []; const sel = '.info, .controls, .top-right, .banner, .dialog, .slate-root:not([hidden]), .settings:not([hidden])';
        document.querySelectorAll(sel).forEach((root) => { if (root.hidden) return;
          const w = document.createTreeWalker(root, NodeFilter.SHOW_TEXT);
          for (let n = w.nextNode(); n; n = w.nextNode()) { const s = n.textContent.trim(); if (s && n.parentElement.offsetParent !== null) out.push(s); }
          root.querySelectorAll('[aria-label], [title]').forEach((e) => { const l = e.getAttribute('aria-label') || e.getAttribute('title'); if (l) out.push(l); }); });
        return out; }"""
    )


def english_left(lines):
    """Strings that are an English text of the strings file and differ in German."""
    en = {}
    for k, v in STRINGS.items():
        if v["en"] != v["de"] and "{" not in v["en"]:
            en.setdefault(v["en"], []).append(k)
    return sorted({s for s in lines if s in en})


def set_lang(page, lang):
    open_settings(page)
    page.locator(f"[data-row=lang] [data-value={lang}]").click()
    page.locator(".settings-close").click()


def snapshot_state(page):
    s = state(page)
    m = js(page, "model()")
    return {"state": s, "hand": [c["code"] if isinstance(c, dict) else c for c in m["hand"]], "hud": js(page, "hud().totals"), "packets": m["packets"]}


def court_hand(page):
    return any(re.match(r"^[KQCJ][hdcs]?$", (c["code"] if isinstance(c, dict) else c)) for c in js(page, "model().hand"))


def check_language_switch(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1", nodraw=False)
    take_if_bidding(page)
    for _ in range(3):
        s = state(page)
        if s["prompt"] != "you":
            break
        page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(choose(s, 'first', False)))})")
        rest(page)
    before = snapshot_state(page)
    fail_if(not court_hand(page), failures, "language_switch: no court card in the hand to compare")
    en_image = page.locator("canvas#stage").screenshot()
    page.locator(".slate-tab").click()
    set_lang(page, "de")
    still(page)
    page.wait_for_timeout(500)
    after = snapshot_state(page)
    fail_if(before != after, failures, "language_switch: the state changed with the language")
    fail_if(page.evaluate("document.documentElement.lang") != "de", failures, "language_switch: the document's lang is not de")
    fail_if(page.locator(".slate-tab").inner_text() != t("ui.menu.slate", "de"), failures, "language_switch: the slate tab is not in German")
    fail_if(page.title() != t("ui.title", "de"), failures, "language_switch: the title did not change")
    fail_if(t("slate.title", "de") not in page.locator(".slate").inner_text(), failures, "language_switch: the open slate is not in German")
    left = english_left(visible_strings(page))
    fail_if(left, failures, f"language_switch: English left after the switch to German: {left[:5]}")
    de_image = page.locator("canvas#stage").screenshot()
    fail_if(de_image == en_image, failures, "language_switch: the canvas (the faces' court letters) did not change")
    live = js(page, "live()")
    fail_if(not state(page)["legal"], failures, "language_switch: nothing legal after the switch")
    set_lang(page, "en")
    still(page)
    page.wait_for_timeout(500)
    fail_if(snapshot_state(page) != before, failures, "language_switch: the state changed on the way back")
    fail_if(page.locator(".slate-tab").inner_text() != t("ui.menu.slate"), failures, "language_switch: the slate tab did not come back to English")
    back = page.locator("canvas#stage").screenshot()
    fail_if(back == de_image, failures, "language_switch: the court letters did not come back")
    check_offline(page, failures, "language_switch: ")


def check_language_switch_at_count(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    ok = to_last_trick(page)
    fail_if(not ok, failures, "language_switch_count: no last trick")
    if ok:
        s = state(page)
        a = choose(s, "first", False)
        page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(a))})")
        for _ in range(30):
            page.evaluate("window.strohmandeln3d.tick(100)")
        totals = js(page, "hud().totals")
        set_lang(page, "de")
        fail_if(js(page, "hud().totals") != totals, failures, f"language_switch_count: the HUD's numbers changed from {totals} to {js(page, 'hud().totals')}")
        samples = count_run(page)
        count = next(e for e in js(page, "log()") if e["type"] == "count")
        human = state(page)["human"]
        want = {"you": count["thirds"][human], "opp": count["thirds"][1 - human]}
        fail_if(want not in [x["totals"] for x in samples], failures, f"language_switch_count: the count did not finish at {want} after the switch")
        left = english_left(visible_strings(page))
        fail_if(left, failures, f"language_switch_count: English left at the count: {left[:5]}")
    check_offline(page, failures, "language_switch_count: ")


# ---- persistence ------------------------------------------------------------------------------------------


def check_prefs_storage(browser, failures):
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    open_settings(page)
    page.locator("[data-row=speed] [data-value=slow]").click()
    page.locator("[data-row=lang] [data-value=de]").click()
    page.locator("[data-row=talk] [data-value=calls]").click()
    goto(page, "seed=7&human=1")
    got = js(page, "prefs()")
    fail_if((got["speed"], got["lang"], got["talk"]) != ("slow", "de", "calls"), failures, f"prefs_storage: after a reload the prefs are {got['speed']}, {got['lang']}, {got['talk']}")
    context.close()
    # Blocked storage: defaults, a notice, and it still plays.
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    context.add_init_script("Object.defineProperty(window, 'localStorage', { get() { throw new Error('blocked'); } });")
    context.add_init_script(CHIP_WATCH)
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    page.wait_for_function("(window.__chips || []).length > 0", timeout=10_000)
    fail_if(t("error.no-storage") not in page.evaluate("window.__chips"), failures, "prefs_storage: blocked storage gave no notice")
    take_if_bidding(page)
    fail_if(state(page)["prompt"] not in ("you", "opponent"), failures, "prefs_storage: the page did not play with blocked storage")
    fail_if(page.errors, failures, f"prefs_storage: errors with blocked storage: {page.errors[:3]}")
    context.close()
    # A newer-version record is left as it is.
    newer = json.dumps({"v": 99, "speed": "slow", "lang": "de"})
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    context.add_init_script(f"try {{ if (!localStorage.getItem('strohmandeln.prefs')) localStorage.setItem('strohmandeln.prefs', {json.dumps(newer)}); }} catch (e) {{}}")
    context.add_init_script(CHIP_WATCH)
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    page.wait_for_function("(window.__chips || []).length > 0", timeout=10_000)
    fail_if(t("chip.newer") not in page.evaluate("window.__chips"), failures, "prefs_storage: a newer record was not mentioned")
    fail_if(js(page, "prefs()")["lang"] != "en", failures, "prefs_storage: a newer record was read")
    fail_if(page.evaluate("localStorage.getItem('strohmandeln.prefs')") != newer, failures, "prefs_storage: a newer record was overwritten")


def play_some(page, n):
    for _ in range(n):
        s = state(page)
        if s["prompt"] != "you":
            break
        page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(choose(s, 'first', False)))})")
        rest(page)


def check_save_restore(browser, failures):
    context = browser.new_context(viewport=FAST)
    context.set_offline(True)
    context.add_init_script(NO_DRAW)
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant&keep=1")
    rest(page)
    # Past the first game, so the slate has a row and the talk has heard something.
    ok = drive(page, has("game_scored"))
    fail_if(not ok, failures, "save_restore: no game scored")
    rest(page)
    take_if_bidding(page)
    play_some(page, 3)
    rest(page)
    before = state(page)
    keep = page.evaluate("JSON.parse(localStorage.getItem('strohmandeln.sitting'))")
    fail_if(not keep, failures, "save_restore: nothing was kept")
    rows_before = page.locator(".slate [data-deal]").count()
    hand_before = sorted(c["code"] if isinstance(c, dict) else c for c in js(page, "model().hand"))
    fail_if(not keep.get("extras", {}).get("talk", {}).get("spoken"), failures, "save_restore: the talk's record was not kept")
    fail_if(not keep.get("extras", {}).get("slate", {}).get("ledger", {}).get("rows"), failures, "save_restore: the slate's record was not kept")
    goto(page, "human=1&hook=1")
    page.wait_for_selector("[data-dialog=welcome]", timeout=60_000)
    page.locator("[data-dialog-act=continue]").click()
    rest(page)
    after = state(page)
    for field in ("contos", "deals_counted", "phase", "touren_total", "human"):
        fail_if(before[field] != after[field], failures, f"save_restore: {field} was {before[field]!r}, restored {after[field]!r}")
    fail_if(before["deal"]["own_hand"] != after["deal"]["own_hand"] or before["deal"]["hidden"] != after["deal"]["hidden"] or before["deal"]["trick_number"] != after["deal"]["trick_number"], failures, "save_restore: the deal differs after the restore")
    fail_if(sorted(c["code"] if isinstance(c, dict) else c for c in js(page, "model().hand")) != hand_before, failures, "save_restore: the drawn hand differs")
    fail_if(page.locator(".slate [data-deal]").count() != rows_before, failures, f"save_restore: the slate has {page.locator('.slate [data-deal]').count()} rows, was {rows_before}")
    # The opponent goes on, and the record keeps growing from the restored talk.
    play_some(page, 2)
    keep2 = page.evaluate("JSON.parse(localStorage.getItem('strohmandeln.sitting'))")
    old = {tuple(k) if isinstance(k, list) else k for k, _ in keep["extras"]["talk"]["spoken"]}
    new = {tuple(k) if isinstance(k, list) else k for k, _ in keep2["extras"]["talk"]["spoken"]}
    fail_if(not old <= new, failures, "save_restore: the talk forgot what it had said")
    fail_if(len(js(page, "log()")) <= len(keep["export"]["record"]["moves"]) and state(page)["prompt"] == "opponent", failures, "save_restore: the opponent did not continue")
    fail_if(page.errors, failures, f"save_restore: errors {page.errors[:3]}")
    # A corrupt record is refused with the old sitting kept.
    bad = json.dumps({"v": 1, "export": {"protocol": 1, "human": 1, "level": 1, "record": {"version": 1, "moves": [{"nonsense": True}]}}, "extras": {}})
    page.evaluate(f"localStorage.setItem('strohmandeln.sitting', {json.dumps(bad)})")
    snap = state(page)["deal"]["own_hand"]
    goto(page, "human=1&hook=1")
    page.wait_for_selector("[data-dialog=welcome]", timeout=60_000)
    page.locator("[data-dialog-act=continue]").click()
    page.wait_for_selector("[data-dialog=error], [data-dialog=new]", timeout=30_000)
    fail_if(page.locator("[data-dialog=error]").count() == 0 and page.locator("[data-dialog=new]").count() == 0, failures, "save_restore: a corrupt record was neither refused nor replaced by a new-sitting offer")
    del snap


def check_export_import_file(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)
    play_some(page, 2)
    before = state(page)
    open_settings(page)
    with page.expect_download() as info:
        page.locator("[data-action=save]").click()
    path = info.value.path()
    text = Path(path).read_text()
    saved = json.loads(text)
    fail_if(saved.get("v") != 1 or "export" not in saved or "extras" not in saved, failures, "export_import_file: the file lacks v, export or extras")
    # A bad file is refused and changes nothing.
    files = Path(tempfile.mkdtemp(prefix="sitting-"))
    bad = files / "bad-sitting.json"
    bad.write_text("{ not json")
    with page.expect_file_chooser() as chooser:
        page.locator("[data-action=load]").click()
    chooser.value.set_files(str(bad))
    page.wait_for_selector(".refusal:not(:empty)", timeout=10_000)
    fail_if(t("error.load-bad") != page.locator(".refusal").inner_text(), failures, f"export_import_file: a bad file gave {page.locator('.refusal').inner_text()!r}")
    fail_if(state(page)["deal"]["own_hand"] != before["deal"]["own_hand"], failures, "export_import_file: a bad file changed the sitting")
    # The good file loads in a fresh page.
    good = files / "good-sitting.json"
    good.write_text(text)
    close_all(browser)
    page = open_page(browser, "seed=3&speed=instant&human=1")
    open_settings(page)
    with page.expect_file_chooser() as chooser:
        page.locator("[data-action=load]").click()
    chooser.value.set_files(str(good))
    page.wait_for_function("window.strohmandeln3d.state().deals_counted !== undefined", timeout=10_000)
    rest(page)
    after = state(page)
    fail_if(after["deal"]["own_hand"] != before["deal"]["own_hand"] or after["contos"] != before["contos"], failures, "export_import_file: the loaded sitting differs from the saved one")


def check_tutor_pages(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&tutor=1", wait=False)
    page.wait_for_selector("[data-dialog=page]", timeout=180_000)
    fail_if(js(page, "busy()"), failures, "tutor_pages: the first page opened while the cards moved")
    title = page.locator("[data-dialog=page] h2").inner_text()
    fail_if(title != t("tutor.page.the-cards"), failures, f"tutor_pages: the first page is {title!r}, not 'The cards'")
    # Pages due at one rest follow one at a time, in the table's order (TABLE3D 6.2.2): the cards, the packets, bidding.
    titles = [title]
    for _ in range(6):
        fail_if(page.locator("[data-dialog=page]").count() != 1, failures, "tutor_pages: not one page at a time")
        page.locator("[data-dialog-act=notnow]").click()
        page.wait_for_timeout(300)
        if not page.locator("[data-dialog=page]").count():
            break
        titles.append(page.locator("[data-dialog=page] h2").inner_text())
    fail_if(page.locator("[data-dialog=page]").count(), failures, "tutor_pages: Not now did not close the page")
    fail_if(titles != [t("tutor.page.the-cards"), t("tutor.page.packets"), t("tutor.page.bidding")], failures, f"tutor_pages: the pages came as {titles}, not the cards, the packets, bidding")
    seen = js(page, "prefs()")["seen"]
    fail_if(not {"the-cards", "packets", "bidding"} <= set(seen), failures, f"tutor_pages: Not now did not mark the pages seen ({seen})")
    rest(page)
    page.keyboard.press("m")
    fail_if(not js(page, "prefs()")["muted"], failures, "tutor_pages: M did not mute the Kibitzer")
    page.keyboard.press("m")
    fail_if(js(page, "prefs()")["muted"], failures, "tutor_pages: M again did not unmute")
    check_offline(page, failures, "tutor_pages: ")


# ---- 10.4 the cards-out tracker (6.6) ----------------------------------------------------------------

TRACKER_SIZES = [("1280x720", {"width": 1280, "height": 720}), ("1024x768", {"width": 1024, "height": 768})]  # a phone layout is deferred (6.6.6)
BOXES = """(() => { const box = (e) => { const r = e.getBoundingClientRect(); return { left: r.left, right: r.right, top: r.top, bottom: r.bottom }; };
  const shown = (sel) => [...document.querySelectorAll(sel)].filter((e) => !e.hidden && e.getBoundingClientRect().width > 0).map(box);
  const panel = document.querySelector('.tracker');
  return { panel: panel ? box(panel) : null, bar: shown('.move-bar button'), prompt: shown('.prompt, .aid-line, .refusal'), tabs: shown('.tutor-row > button'), hud: shown('.info .hud'),
    hand: window.strohmandeln3d.cardRects('your-hand') }; })()"""


def tracker_page(browser, size, prefs=None, query="seed=7&speed=instant&human=0", unfold=True):
    # The panel starts folded (6.6.2); unfold it by its tab, where the window shows one.
    page = with_prefs(browser, {"hints": True, "slateAuto": False, **(prefs or {})}, query, viewport=size)
    take_if_bidding(page)
    drive(page, lambda s, log: sum(e["type"] == "trick_won" for e in log) >= 3 or s["prompt"] == "over")
    rest(page)
    tab = page.locator("[data-act=tracker]:not([hidden])")
    if unfold and tab.count() and tab.is_visible():
        tab.click()
    return page


def check_tracker_clear(browser, failures):
    # Open at rest, the engine's counts in the panel, clear of the hand, the bar, the prompt, the tabs and the HUD.
    for name, size in TRACKER_SIZES:
        page = tracker_page(browser, size)
        where = f"tracker_clear {name}: "
        boxes = page.evaluate(BOXES)
        panel = boxes["panel"]
        fail_if(panel is None, failures, f"{where}no panel with Hints on")
        if panel:
            fail_if(panel["left"] < 0 or panel["right"] > size["width"] or panel["top"] < 0 or panel["bottom"] > size["height"], failures, f"{where}the panel runs off the window: {panel}")
            for what in ("bar", "prompt", "tabs", "hud", "hand"):
                for other in boxes[what]:
                    fail_if(intersects(panel, other), failures, f"{where}the panel {panel} covers {what} {other}")
            data = js(page, "tracker()")["data"]
            fail_if(not data or data["trick"] != state(page)["deal"]["trick_number"], failures, f"{where}the panel is not of the trick at rest ({data and data['trick']})")
            lines = page.locator(".tracker-lines li").count()
            fail_if(not 2 <= lines <= 4, failures, f"{where}{lines} lines")
            fail_if(page.locator(".tracker-grid .tracker-cell").count() != 54, failures, f"{where}the grid has not 54 cells")
            fail_if(page.locator(".tracker-grid").get_attribute("aria-hidden") != "true", failures, f"{where}the grid is not hidden from the screen reader")
            fail_if(page.locator(".tracker-sr li").count() != 5, failures, f"{where}the screen reader has not 5 sentences")
            first = page.locator(".tracker-lines li").first.inner_text()
            fail_if(not first.startswith(("Tarocks still out", "No tarock is out")), failures, f"{where}the first line is {first!r}")
            fail_if(page.locator("[data-act=tracker]").get_attribute("aria-expanded") != "true", failures, f"{where}the tab does not say it is open")
            scroll = page.evaluate("(() => { const p = document.querySelector('.tracker'); return p.scrollHeight - p.clientHeight; })()")
            fail_if(scroll > 1, failures, f"{where}the panel scrolls by {scroll} px: the legend is cut off")
        check_offline(page, failures, where)
        close_all(browser)


def check_tracker_fold_and_settings(browser, failures):
    where = "tracker_fold_and_settings: "
    page = tracker_page(browser, TRACKER_SIZES[0][1])
    tab = page.locator("[data-act=tracker]")
    # A tap on the tab folds it; another opens it. A fresh page starts folded, at this large window too.
    fresh = tracker_page(browser, TRACKER_SIZES[0][1], unfold=False)
    fail_if(fresh.locator(".tracker").count(), failures, f"{where}the panel starts open")
    fail_if(fresh.locator("[data-act=tracker]").get_attribute("aria-expanded") != "false", failures, f"{where}the fresh tab says open")
    fresh.close()
    tab.click()
    fail_if(page.locator(".tracker").count(), failures, f"{where}the tab did not fold the panel")
    fail_if(tab.get_attribute("aria-expanded") != "false", failures, f"{where}the tab says open while folded")
    tab.click()
    fail_if(page.locator(".tracker").count() != 1, failures, f"{where}the tab did not open the panel")
    page.locator("[data-act=tracker-fold]").click()
    fail_if(page.locator(".tracker").count(), failures, f"{where}Fold did not fold the panel")
    tab.click()
    # It follows the cards: after the human's next play and the reply, the panel is of the new trick.
    before = js(page, "tracker()")["data"]["trick"]
    drive(page, lambda s, log: (s["deal"] or {}).get("trick_number", 0) > before or s["prompt"] == "over")
    rest(page)
    if state(page)["prompt"] != "over":
        fail_if(page.locator(".tracker").get_attribute("data-trick") != str(state(page)["deal"]["trick_number"]), failures, f"{where}the panel was not updated at rest")
    # Hints off: no tab, no panel; back on, and the tracker's own setting off: the same.
    page.locator("[data-act=settings]").dispatch_event("click")
    page.wait_for_selector(".settings-root:not([hidden])", timeout=10_000)
    page.locator("[data-row=hints] [data-value=toggle]").click()
    fail_if(page.locator("[data-act=tracker]:not([hidden])").count() or page.locator(".tracker").count(), failures, f"{where}Hints off left the tracker")
    page.locator("[data-row=hints] [data-value=toggle]").click()
    fail_if(page.locator(".tracker").count() != 1, failures, f"{where}Hints on did not bring the panel back")
    page.locator("[data-row=tracker] [data-value=toggle]").click()
    fail_if(page.locator("[data-act=tracker]:not([hidden])").count() or page.locator(".tracker").count(), failures, f"{where}the tracker setting off left it")
    page.locator("[data-row=tracker] [data-value=toggle]").click()
    # The Kibitzer's mute takes it away; unmuting brings it back.
    page.keyboard.press("Escape")
    page.keyboard.press("m")
    fail_if(page.locator(".tracker").count(), failures, f"{where}the muted Kibitzer kept the panel")
    page.keyboard.press("m")
    fail_if(page.locator(".tracker").count() != 1, failures, f"{where}unmuting did not bring the panel back")
    # The next deal of the sitting starts folded again, though the person had it open.
    drive(page, lambda s, log: sum(e["type"] == "deal_start" for e in log) >= 2)
    rest(page)
    fail_if(page.locator(".tracker").count(), failures, f"{where}the next deal kept the panel open")
    fail_if(page.locator("[data-act=tracker]").get_attribute("aria-expanded") != "false", failures, f"{where}the next deal's tab says open")
    check_offline(page, failures, where)


def check_tracker_languages(browser, failures):
    # In German the tab, the lines and the court labels are German; upright the tracker is out of the way.
    where = "tracker_languages: "
    page = tracker_page(browser, TRACKER_SIZES[0][1], {"lang": "de"})
    fail_if(page.locator("[data-act=tracker]").inner_text() != t("tracker.tab", "de"), failures, f"{where}the tab is not German")
    first = page.locator(".tracker-lines li").first.inner_text()
    fail_if(not first.startswith(("Tarock noch draußen", "Kein Tarock")), failures, f"{where}the first German line is {first!r}")
    labels = page.locator("[data-row=h] .tracker-cell").all_inner_texts()[:4]
    fail_if(labels != ["K", "D", "C", "B"], failures, f"{where}the German court labels are {labels}")
    scroll = page.evaluate("(() => { const p = document.querySelector('.tracker'); return p.scrollHeight - p.clientHeight; })()")
    fail_if(scroll > 1, failures, f"{where}the German panel scrolls by {scroll} px: the legend is cut off at 1280x720")
    fail_if(page.locator(".tracker-legend li").count() != 3, failures, f"{where}the legend has not three entries")
    close_all(browser)
    # Upright, the tracker is not there at all (deferred, 6.6.6): it cannot break the page.
    upright = tracker_page(browser, {"width": 390, "height": 844})
    fail_if(upright.locator(".tracker-root").is_visible() or upright.locator("[data-act=tracker]").is_visible(), failures, f"{where}the tracker shows on a phone")
    fail_if(js(upright, "tracker()")["data"] is not None, failures, f"{where}the engine was asked for a tracker nobody sees")
    check_offline(upright, failures, f"{where}upright: ")


# ---- 10.4 the brief after a scored deal (6.4) ----------------------------------------------------------

BRIEF_KEPT = "(() => { const b = window.strohmandeln3d.briefing(); return b.kept.length >= 1 && b.kept.every((d) => d.summary) && b.owed === 0; })()"


def brief_page(browser, prefs, query="seed=7&speed=instant&human=1"):
    page = with_prefs(browser, {"hints": True, "slateAuto": False, **prefs}, query)
    return page


def scored_deal(page):
    """Play on by the first legal action to the end of a deal and let the tutor settle."""
    ok = drive(page, has("game_scored"))
    rest(page)
    page.wait_for_function(BRIEF_KEPT, timeout=60_000)
    return ok


def check_deal_brief(browser, failures):
    page = brief_page(browser, {})
    # a hint asked at the first decision marks it assisted: its number is the engine's, counted over both seats
    page.wait_for_selector("[data-act=hint]", timeout=30_000)
    number = state(page)["deal_actions"]
    page.locator("[data-act=hint]").click()
    page.wait_for_function("window.strohmandeln3d.briefing().assisted.length > 0", timeout=10_000)
    fail_if(js(page, "briefing()")["assisted"] != [number], failures, f"deal_brief: the hint's number is {js(page, 'briefing()')['assisted']}, not [{number}]")
    fail_if(not scored_deal(page), failures, "deal_brief: no deal was scored")
    kept = js(page, "briefing()")["kept"]
    fail_if(len(kept) != 1 or number not in kept[0]["assisted"], failures, f"deal_brief: the kept deal does not hold the assisted number: {kept}")
    fail_if(js(page, "briefing()")["assisted"] != [], failures, "deal_brief: the assisted list was not cleared for the next deal")
    # the chip opens the dialog: the count, at most three bullets, the folded method
    page.wait_for_selector(".tutor-review:not([hidden])", timeout=10_000)
    page.locator(".tutor-review").click()
    page.wait_for_selector("[data-dialog=review]", timeout=10_000)
    head = page.locator("[data-dialog=review] .tutor-count").inner_text()
    fail_if(not head.startswith("You counted"), failures, f"deal_brief: the header is {head!r}")
    bullets = page.locator("[data-dialog=review] li")
    fail_if(bullets.count() > 3, failures, f"deal_brief: {bullets.count()} bullets")
    for i in range(bullets.count()):
        lead = bullets.nth(i).locator("b").inner_text()
        fail_if(not lead.endswith(":") and not lead.endswith("."), failures, f"deal_brief: the lead {lead!r}")
    fail_if(page.locator("[data-dialog=review] summary").inner_text() != t("tutor.how"), failures, "deal_brief: no folded method")
    fail_if(page.locator("[data-dialog=review] details[open]").count(), failures, "deal_brief: the method is not folded")
    page.locator("[data-dialog=review] [data-dialog-act=close]").click()
    page.wait_for_function("!document.querySelector('[data-dialog=review]')", timeout=10_000)
    # muted: no chip, no dialog
    page.keyboard.press("m")
    fail_if(page.locator(".tutor-review:not([hidden])").count(), failures, "deal_brief: the chip shows while muted")
    page.evaluate("window.strohmandeln3d.openReview()")
    fail_if(page.locator("[data-dialog=review]").count(), failures, "deal_brief: the brief opened while muted")
    # unmuted, the next deal brings the nudge at the first clear chance at the focus (one a deal);
    # that decision then counts as assisted
    page.keyboard.press("m")
    focus = js(page, "briefing()")["focus"]
    fail_if(focus is None, failures, "deal_brief: the first deal named no focus (the seed is chosen for one)")
    if focus:
        before = len(js(page, "briefing()")["kept"])
        drive(page, lambda st, log: bool(js(page, "briefing()")["assisted"]) or sum(e["type"] == "game_scored" for e in log) > before or st["prompt"] == "over")
        assisted = js(page, "briefing()")["assisted"]
        if assisted:
            note = js(page, "tutor()")["note"]
            words = t(f"tutor.nudge.{focus}")
            fail_if(note != f"{t('tutor.lead.rule')} {words[0].lower()}{words[1:]}", failures, f"deal_brief: the nudge says {note!r}, not the focus's ({focus})")
            fail_if(len(assisted) != 1, failures, f"deal_brief: the nudged decision is not assisted: {assisted}")
    # clearing what the Kibitzer keeps asks first, says it is done, and the focus does not come back
    kept_before = len(js(page, "briefing()")["kept"])
    fail_if(kept_before < 1, failures, "deal_brief: nothing kept")
    page.locator("[data-act=settings]").click()
    page.locator("[data-action=progress-clear]").click()
    page.wait_for_selector("[data-dialog=clear]", timeout=10_000)
    page.locator("[data-dialog=clear] [data-dialog-act=no]").click()
    fail_if(len(js(page, "briefing()")["kept"]) != kept_before, failures, "deal_brief: Cancel cleared what is kept")
    page.locator("[data-action=progress-clear]").click()
    page.wait_for_selector("[data-dialog=clear]", timeout=10_000)
    page.locator("[data-dialog=clear] [data-dialog-act=yes]").click()
    page.wait_for_function("window.strohmandeln3d.briefing().kept.length === 0", timeout=10_000)
    fail_if(js(page, "briefing()")["focus"] is not None, failures, "deal_brief: the focus survived the clearing")
    page.keyboard.press("Escape")
    check_offline(page, failures, "deal_brief: ")
    # the German, and the tips off: the count and what went well, nothing to work on
    for prefs, check in (
        ({"lang": "de"}, lambda h, n: (h.startswith("Sie zählten"), "German header")),
        ({"tips": False}, lambda h, n: (n <= 1, "one bullet with the tips off")),
    ):
        other = brief_page(browser, prefs)
        take_if_bidding(other)
        scored_deal(other)
        other.wait_for_selector(".tutor-review:not([hidden])", timeout=10_000)
        other.locator(".tutor-review").click()
        other.wait_for_selector("[data-dialog=review]", timeout=10_000)
        ok, what = check(other.locator("[data-dialog=review] .tutor-count").inner_text(), other.locator("[data-dialog=review] li").count())
        fail_if(not ok, failures, f"deal_brief: {what} ({prefs})")
        if "tips" in prefs:
            texts = other.locator("[data-dialog=review] li").all_inner_texts()
            fail_if(any(x.startswith("Next:") for x in texts), failures, f"deal_brief: advice with the tips off: {texts}")
        check_offline(other, failures, "deal_brief: ")


CHECKS_104 = [
    check_slate_open_close, check_slate_rows, check_slate_points_and_settlement_note, check_talk_events, check_talk_setting,
    check_talk_result_after_count, check_talk_repeat, check_settings_rows, check_settings_next_sitting, check_skill_speed,
    check_language_switch, check_language_switch_at_count, check_prefs_storage, check_save_restore, check_export_import_file,
    check_tutor_pages, check_tracker_clear, check_tracker_fold_and_settings, check_tracker_languages, check_deal_brief,
]
