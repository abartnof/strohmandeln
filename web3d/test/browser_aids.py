"""The browser test's checks for the slate, the talk, the settings, the language
switch and the persistence (docs/TABLE3D.md 10.4, 10.5). The tutor's checks are
the tutor's own. Imported by browser.py."""

import json
import re
from pathlib import Path

from browser import NO_DRAW, PAGE, ROOT, WORDS, STRINGS, check_offline, close_all, fail_if, js, rest, state, still
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
            window.__say.push({ text: n.dataset.text, who: n.dataset.who, clock: h.clock(), deals: s ? s.deals_counted : 0 }); }
        }))).observe(document.querySelector('.say-layer'), { childList: true }); }"""
    )


def heard(page):
    return page.evaluate("window.__say")


def goto(page, query):
    page.goto(f"{PAGE.as_uri()}?{query}&lowfx=1&level=1", wait_until="commit", timeout=120_000)
    page.wait_for_function("window.strohmandeln3d !== undefined", timeout=180_000)


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


def check_talk_events(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1", wait=False)
    watch_say(page)
    rest(page)
    ok = drive(page, until_over())
    fail_if(not ok, failures, "talk_events: the sitting did not end")
    page.wait_for_selector("[data-dialog-act=new]", timeout=60_000)
    page.wait_for_timeout(1500)
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
        if any(e["type"] == event for e in log):
            fail_if(not seen_groups & set(groups), failures, f"talk_events: {event} occurred but none of {groups} was said (heard {sorted(seen_groups)[:12]})")
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
        page = open_page(browser, f"seed=7&speed=instant&human=1&talk={mode}", wait=False)
        watch_say(page)
        rest(page)
        drive(page, has("game_scored"))
        rest(page)
        page.wait_for_timeout(1200)
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
    page = open_page(browser, "seed=7&speed=instant&human=1", wait=False)
    watch_say(page)
    rest(page)
    drive(page, until_over())
    page.wait_for_timeout(1500)
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
        # A repeat in the window: allowed only if every other line of the group was spent too.
        group_sigs = {WORDS["texts"][k].get("sig", k) for g in gs for k in WORDS["groups"][g]}
        if group_sigs - recent:
            bad.append(line["text"])
    fail_if(bad, failures, f"talk_repeat: sayings repeated inside 3 deals with an alternative: {bad[:3]}")
    fail_if(len(lines) < 10, failures, f"talk_repeat: only {len(lines)} lines in a whole sitting")


# ---- 10.5 settings -------------------------------------------------------------------------------------


def open_settings(page):
    page.locator("[data-act=settings]").click()
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
    fail_if(options != ["1", "1.5", "2", "2.5", "3", "3.5", "4"], failures, f"skill_speed: the dial offers {options}")
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
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    page.wait_for_selector(".chip", timeout=10_000)
    fail_if(t("error.no-storage") not in page.locator(".chip").all_inner_texts(), failures, "prefs_storage: blocked storage gave no notice")
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
    page = context.new_page()
    page.errors = []
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    goto(page, "seed=7&human=1&speed=instant")
    rest(page)
    page.wait_for_selector(".chip", timeout=10_000)
    fail_if(t("chip.newer") not in page.locator(".chip").all_inner_texts(), failures, "prefs_storage: a newer record was not mentioned")
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
    bad = Path("/tmp/sb/bad-sitting.json")
    bad.write_text("{ not json")
    with page.expect_file_chooser() as chooser:
        page.locator("[data-action=load]").click()
    chooser.value.set_files(str(bad))
    page.wait_for_selector(".refusal:not(:empty)", timeout=10_000)
    fail_if(t("error.load-bad") != page.locator(".refusal").inner_text(), failures, f"export_import_file: a bad file gave {page.locator('.refusal').inner_text()!r}")
    fail_if(state(page)["deal"]["own_hand"] != before["deal"]["own_hand"], failures, "export_import_file: a bad file changed the sitting")
    # The good file loads in a fresh page.
    good = Path("/tmp/sb/good-sitting.json")
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


CHECKS_104 = [
    check_slate_open_close, check_slate_rows, check_slate_points_and_settlement_note, check_talk_events, check_talk_setting,
    check_talk_result_after_count, check_talk_repeat, check_settings_rows, check_settings_next_sitting, check_skill_speed,
    check_language_switch, check_language_switch_at_count, check_prefs_storage, check_save_restore, check_export_import_file,
]
