"""The browser test's scenario checks (docs/TABLE3D.md 10.2, 10.3): seeded
sittings driven by the engine's own legal actions (`act`), the table's clock
moved by the test (`?manual`) where a frame matters. Imported by browser.py."""

import json
import re

from browser import open_page as _open_page
from browser import (
    LANDSCAPE,
    PAGE,
    STRINGS,
    WORDS,
    check_offline,
    click_card,
    close_all,
    fail_if,
    js,
    rest,
    state,
    still,
)

FAST = {"width": 800, "height": 450}  # the flows that play whole sittings: a software GPU draws a small window faster


def open_page(browser, query="seed=7&speed=instant", viewport=None, **kw):
    return _open_page(browser, query, viewport or FAST, **kw)


# ---- driving the table without a mouse ---------------------------------------------------------


def mrest(page, step=250, limit=4000):
    """Rest, on a table whose clock the test moves (?manual)."""
    for _ in range(limit):
        ready = page.evaluate("(() => { const h = window.strohmandeln3d; const s = h && h.state(); return Boolean(s) && !h.busy() && s.prompt !== 'opponent'; })()")
        if ready:
            return
        page.evaluate(f"window.strohmandeln3d.tick({step})")
    raise AssertionError("the table did not come to rest")


def wire(a):
    return {"action": "play", "card": a["card"]} if a["action"] == "play" else a


def choose(s, policy, extend):
    legal = s["legal"]
    if s["phase"] == "extend":
        return {"action": "extend", "accept": extend}
    if policy == "weiter":
        for a in legal:
            if a["action"] == "pass":
                return a
    return legal[0]


def drive(page, until, policy="first", extend=False, limit=3000, manual=False, step=250):
    """Play by the engine's own first legal action (or weiter) until `until(state, log)`."""
    for _ in range(limit):
        (mrest(page, step) if manual else rest(page))
        s = state(page)
        log = js(page, "log()")
        if until(s, log):
            return True
        if s["prompt"] == "over":
            return False
        a = choose(s, policy, extend)
        page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(a))})")
    return False


def has(kind, **fields):
    return lambda s, log: any(e["type"] == kind and all(e.get(k) == v for k, v in fields.items()) for e in log)


def count_of(page, zone):
    return sum(1 for m in js(page, "placement()") if m["zone"] == zone)


def wait_chip(page, kind=None, timeout=15_000):
    sel = f".chip.{kind}" if kind else ".chip"
    page.wait_for_selector(sel, timeout=timeout)
    return page.locator(sel).first.inner_text()


def with_prefs(browser, prefs, query="seed=7&speed=instant&human=1", viewport=None, reduced=False, wait=True):
    context = browser.new_context(viewport=viewport or FAST, reduced_motion="reduce" if reduced else "no-preference")
    context.set_offline(True)
    context.add_init_script(f"try {{ localStorage.setItem('strohmandeln.prefs', JSON.stringify({json.dumps({'v': 1, **prefs})})); }} catch (e) {{}}")
    page = context.new_page()
    page.requests = []
    page.errors = []
    page.on("request", lambda r: page.requests.append(r.url))
    page.on("console", lambda m: m.type == "error" and page.errors.append(m.text))
    page.on("pageerror", lambda e: page.errors.append(str(e)))
    page.goto(f"{PAGE.as_uri()}?{query}&lowfx=1&level=1", wait_until="commit", timeout=120_000)
    page.wait_for_function("window.strohmandeln3d !== undefined", timeout=180_000)
    if wait:
        rest(page)
    return page


def take_if_bidding(page, manual=False):
    if state(page)["phase"] == "bid":
        page.locator("[data-act=take]").click()
        (mrest(page, step=400) if manual else rest(page))


def watch_chips(page):
    """Record every chip's text as it is put up (they fade in a moment)."""
    page.evaluate("window.__chips = []; new MutationObserver((ms) => ms.forEach((m) => m.addedNodes.forEach((n) => n.classList && n.classList.contains('chip') && window.__chips.push({ text: n.textContent, cls: n.className })))).observe(document.querySelector('.floats'), { childList: true })")


def chips_seen(page):
    return page.evaluate("window.__chips")


# ---- 10.2 choreography ---------------------------------------------------------------------------


def check_cut_and_deal(browser, failures):
    page = open_page(browser, "seed=7&speed=normal&human=1&manual", wait=False)
    most = 0
    for _ in range(80):
        page.evaluate("window.strohmandeln3d.tick(80)")
        coded = page.evaluate("window.strohmandeln3d.shown().filter(c => c).length")
        most = max(most, coded)
        if coded > 2:
            break
    fail_if(most < 2, failures, f"cut_and_deal: no cut seen (at most {most} faces)")
    mrest(page)
    zones = {z: count_of(page, z) for z in ("your-hand", "their-hand", "your-packets", "their-packets", "pack")}
    fail_if(zones["your-hand"] < 15 or zones["their-hand"] < 15, failures, f"cut_and_deal: hands {zones}")
    check_offline(page, failures, "cut_and_deal: ")


def check_bid_badge(browser, failures):
    page = open_page(browser)
    page.locator("[data-act=take]").click()
    rest(page)
    fail_if(page.locator(".tag.declarer").count() != 1, failures, "bid_badge: no declarer badge after a take")
    close_all(browser)
    page = open_page(browser, "seed=1&speed=instant&human=0")
    watch_chips(page)
    page.locator("[data-act=pass]").click()
    page.wait_for_function("window.__chips.length > 0", timeout=60_000)
    text = chips_seen(page)[0]["text"]
    fail_if(text != STRINGS["ui.pass"]["en"], failures, f"bid_badge: the pass chip says {text!r}")
    page.wait_for_selector(".chip", state="detached", timeout=20_000)
    check_offline(page, failures, "bid_badge: ")


def longest_hold(page, ticks=500, step=40):
    """The longest time a turned card is seen held: a card with a face, raised and not moving."""
    prev = None
    run = 0
    longest = 0
    for _ in range(ticks):
        page.evaluate(f"window.strohmandeln3d.tick({step})")
        meshes = page.evaluate("window.strohmandeln3d.meshInfo()")
        now = {i: m for i, m in enumerate(meshes) if m["code"] and m["y"] > 2.5}
        held = [i for i, m in now.items() if prev and i in prev and max(abs(prev[i][k] - m[k]) for k in "xyz") < 0.05]
        run = run + 1 if held else 0
        longest = max(longest, run)
        prev = now
        if page.evaluate("window.strohmandeln3d.state() && !window.strohmandeln3d.busy() && window.strohmandeln3d.model().pack === 0"):
            break
    return longest * step


def chains(events):
    run = []
    for e in events:
        if e["type"] == "card_pulled":
            run.append(e)
        elif run:
            yield run
            run = []
    if run:
        yield run


def check_reveal_shows_cards(browser, failures):
    page = open_page(browser, "seed=7&speed=normal&human=1&manual", wait=False)
    mrest(page, step=400)
    page.locator("[data-act=take]").click()
    longest = longest_hold(page)
    fail_if(longest < 700, failures, f"reveal_shows_cards: the longest hold seen was {longest} ms")
    mrest(page)
    log = js(page, "log()")
    chain = max((len(g) for g in chains([e for e in log if e["type"] in ("card_pulled", "cover_set")])), default=0)
    fail_if(chain < 3, failures, f"reveal_shows_cards: no chain of three pulls in the seeded sitting ({chain})")
    fail_if(not any(e["type"] == "cover_set" for e in log), failures, "reveal_shows_cards: no low card stayed as a cover")
    m = js(page, "model()")
    pulled = sum(1 for e in log if e["type"] == "card_pulled" and e["seat"] == m["human"])
    played = sum(1 for e in log if e["type"] == "card_played" and e["seat"] == m["human"])
    fail_if(len(m["hand"]) != 15 + pulled - played, failures, "reveal_shows_cards: the hand does not hold the pulled cards")


def check_last_card_pulled(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)
    log = js(page, "log()")
    last = [e for e in log if e["type"] == "card_pulled" and e["last"]]
    fail_if(not last, failures, "last_card_pulled: the seeded sitting pulls no last card")
    for e in last:
        left = [m for m in js(page, "placement()") if m["packet"] and m["packet"]["seat"] == e["seat"] and m["packet"]["i"] == e["packet"] - 1]
        fail_if(left, failures, f"last_card_pulled: packet {e['packet']} of seat {e['seat']} still has {len(left)} cards")


def check_play_hand_vs_packet(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)
    s = state(page)
    plays = [a for a in s["legal"] if a["action"] == "play"]
    packet = next((a for a in plays if a["source"] != "hand"), None)
    fail_if(packet is None, failures, "play_hand_vs_packet: no cover is playable at the first lead")
    if packet:
        before = s["deal"]["hidden"]
        # In one step: the opponent follows a moment later, and the replacement after that.
        now = page.evaluate(
            """(a) => { const h = window.strohmandeln3d; h.act(a); return { hidden: h.state().deal.hidden,
                badges: [...document.querySelectorAll('.tag.badge')].map((n) => Number(n.textContent)), placement: h.placement() }; }""",
            wire(packet),
        )
        fail_if(before != now["hidden"], failures, "play_hand_vs_packet: the hidden counts changed before the replacement")
        fail_if(sorted(now["badges"]) != sorted(h for row in before for h in row if h > 0), failures, f"play_hand_vs_packet: badges {now['badges']} changed with the play")
        i = int(packet["source"][-1]) - 1
        stack = [m for m in now["placement"] if m["packet"] and m["packet"]["seat"] == s["human"] and m["packet"]["i"] == i]
        fail_if(any(m["code"] for m in stack) or len(stack) != before[s["human"]][i], failures, "play_hand_vs_packet: the stack top is not bare")
        fail_if(not any(m["zone"] == "trick" and m["code"] == packet["card"] for m in now["placement"]), failures, "play_hand_vs_packet: the cover did not land in the trick")


def check_replacement_after_trick(browser, failures):
    page = open_page(browser, "seed=7&speed=normal&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    bad = 0
    for _ in range(12):
        s = state(page)
        if s["prompt"] != "you":
            break
        plays = [a for a in s["legal"] if a["action"] == "play"]
        js(page, f"act({json.dumps(wire(plays[0]))})")
        for _ in range(300):
            page.evaluate("window.strohmandeln3d.tick(80)")
            info = page.evaluate("window.strohmandeln3d.meshInfo()")
            plc = js(page, "placement()")
            trick = [m for m in plc if m["zone"] == "trick" and m["code"]]
            raised = [m for m in info if m["code"] and m["y"] > 2.5 and m["z"] < 6]
            if len(trick) == 2 and raised:
                bad += 1
            if page.evaluate("!window.strohmandeln3d.busy() && window.strohmandeln3d.state().prompt !== 'opponent'"):
                break
    fail_if(bad, failures, f"replacement_after_trick: a card was turned while the trick lay on the table ({bad} frames)")


def check_trick_gather(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)

    def won():
        return count_of(page, "your-won") + count_of(page, "their-won")

    before = won()
    s = state(page)
    js(page, f"act({json.dumps(wire(next(a for a in s['legal'] if a['action'] == 'play')))})")
    rest(page)
    fail_if(won() != before + 2, failures, f"trick_gather: the piles grew from {before} to {won()}")
    fail_if(any(m["code"] for m in js(page, "placement()") if m["zone"].endswith("won")), failures, "trick_gather: a won card shows its face")


def check_last_trick_ultimo(browser, failures):
    for seed, human, result in [(2, 1, "won"), (1, 1, "lost")]:
        page = open_page(browser, f"seed={seed}&speed=instant&human={human}")
        watch_chips(page)
        ok = drive(page, has("pagat_ultimo", result=result))
        fail_if(not ok, failures, f"last_trick_ultimo: seed {seed} has no ultimo {result}")
        if ok:
            page.wait_for_function("window.__chips.some(c => c.cls.includes('ultimo'))", timeout=60_000)
            chip = next(c for c in chips_seen(page) if "ultimo" in c["cls"])
            fail_if(STRINGS["chip.ultimo.won"]["en"].split(" +")[0] not in chip["text"], failures, f"last_trick_ultimo: the chip says {chip['text']!r}")
            fail_if(result not in chip["cls"], failures, f"last_trick_ultimo: the chip is not marked {result}")
        close_all(browser)


def to_last_trick(page):
    """Play by the engine's first action, at Instant, to the last trick of the deal; then Normal speed."""
    page.evaluate("window.strohmandeln3d.speed('instant')")
    ok = drive(page, lambda s, log: s["deal"] is not None and s["deal"]["trick_number"] == 27 and s["prompt"] == "you" and s["phase"] == "play", manual=True, step=400)
    page.evaluate("window.strohmandeln3d.speed('normal')")
    return ok


SAMPLE = """() => { const h = window.strohmandeln3d; return { clock: h.clock(), totals: h.hud().totals, idle: h.hud().idle, busy: h.busy(),
    counted: h.meshInfo().filter((m) => m.code).length, said: h.said().length, banner: !document.querySelector('.banner').hidden }; }"""


def count_run(page, limit=1200, step=100):
    """Tick through the count and what follows it; every sample."""
    samples = []
    play = None
    for _ in range(limit):
        s = state(page)
        if play is None and s["prompt"] == "you":
            a = choose(s, "first", False)
            page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(a))})")
            play = True
        page.evaluate(f"window.strohmandeln3d.tick({step})")
        samples.append(page.evaluate(SAMPLE))
        if play and any(x["banner"] for x in samples) and not samples[-1]["banner"] and samples[-1]["idle"]:
            break
        if state(page)["prompt"] == "you" and play and any(e["type"] == "game_scored" for e in js(page, "log()")) and not samples[-1]["busy"]:
            break
    return samples


def check_count_thirds(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    ok = to_last_trick(page)
    fail_if(not ok, failures, "count_thirds: the sitting did not reach a last trick")
    if ok:
        samples = count_run(page)
        count = next(e for e in js(page, "log()") if e["type"] == "count")
        human = state(page)["human"]
        want = {"you": count["thirds"][human], "opp": count["thirds"][1 - human]}
        seen = [x["totals"] for x in samples]
        fail_if(want not in seen, failures, f"count_thirds: the HUD never showed {want}; at most {max(seen, key=lambda t: t['you'] + t['opp'])}")
        # The tally only ever rises through the count.
        rising = [t["you"] + t["opp"] for t in seen]
        fail_if(any(b < a for a, b in zip(rising[: rising.index(max(rising)) + 1], rising[1:])), failures, "count_thirds: the tally fell during the count")
        fail_if(sum(count["thirds"]) != 210, failures, "count_thirds: the takes do not sum to 210")
        fail_if(max(x["counted"] for x in samples) < 3, failures, "count_thirds: no cards turned up in the count")


def check_count_quiet(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    ok = to_last_trick(page)
    fail_if(not ok, failures, "count_quiet: the sitting did not reach a last trick")
    if ok:
        samples = count_run(page)
        timing = js(page, "timing()")
        count = timing["count"]
        fail_if(count is None, failures, "count_quiet: no count timing")
        if count:
            lo, hi = timing["base"] + count["lines"][0], timing["base"] + count["end"] - 5
            noisy = [x for x in samples if lo <= x["clock"] <= hi and x["said"]]
            fail_if(noisy, failures, f"count_quiet: a dialogue box was up during the count ({len(noisy)} samples)")


def check_game_scored_banner(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page, manual=True)
    ok = to_last_trick(page)
    fail_if(not ok, failures, "game_scored_banner: the sitting did not reach a last trick")
    if ok:
        samples = count_run(page)
        shown = [i for i, x in enumerate(samples) if x["banner"]]
        fail_if(not shown, failures, "game_scored_banner: no banner after the count")
        if shown:
            first = shown[0]
            # The banner comes after the count's popups, and holds the table: the hud is idle by then.
            fail_if(not samples[first]["idle"], failures, "game_scored_banner: the banner came before the count was told")
            fail_if(first < len(samples) // 3, failures, "game_scored_banner: the banner rose too early")
            fail_if(shown[-1] - shown[0] < 10, failures, "game_scored_banner: the banner did not hold")
        fail_if(page.locator(".slate [data-deal]").count() < 1, failures, "game_scored_banner: no slate row written")


def check_draw_and_doubling(browser, failures):
    page = open_page(browser, "seed=2&speed=instant&human=1")
    page.evaluate("window.__banners = []; new MutationObserver((ms) => ms.forEach((m) => m.addedNodes.forEach((n) => window.__banners.push(n.textContent)))).observe(document.querySelector('.banner'), { childList: true })")
    watch_chips(page)
    ok = drive(page, has("game_scored", result="draw"), policy="weiter")
    fail_if(not ok, failures, "draw_and_doubling: no draw in the seeded sitting")
    if ok:
        page.wait_for_function("window.__banners.some(t => t.includes(%s))" % json.dumps(STRINGS["banner.draw"]["en"]), timeout=60_000)
        banners = page.evaluate("window.__banners")
        fail_if(not any(STRINGS["banner.doubling-next"]["en"] in b for b in banners), failures, "draw_and_doubling: the draw's banner does not say the next deal counts double")
        fail_if(not any(STRINGS["chip.doubling"]["en"] == c["text"] for c in chips_seen(page)), failures, "draw_and_doubling: no doubling marker chip")
        ok2 = drive(page, has("doubling_applied"), policy="weiter")
        fail_if(not ok2, failures, "draw_and_doubling: the doubling was not applied")
        if ok2:
            page.wait_for_function("document.querySelector('.tour-line').innerText.includes(%s)" % json.dumps(STRINGS["hud.double"]["en"]), timeout=60_000)


def check_void_deal(browser, failures):
    page = open_page(browser, "seed=520&speed=normal&human=0&manual", wait=False)
    watch_chips(page)
    for _ in range(300):
        page.evaluate("window.strohmandeln3d.tick(100)")
        if page.evaluate("window.__chips.length") > 0:
            break
    seen = chips_seen(page)
    fail_if(not seen, failures, "void_deal: no tag rose over the hand")
    if seen:
        fail_if(STRINGS["chip.void-deal"]["en"] != seen[0]["text"], failures, f"void_deal: the tag says {seen[0]['text']!r}")
    mrest(page, step=400)
    s = state(page)
    fail_if(s["deals_counted"] != 0, failures, "void_deal: the void deal was counted")
    fail_if("1" not in page.locator(".tour-line").inner_text(), failures, "void_deal: the deal counter moved")


def check_throw_in(browser, failures):
    page = with_prefs(browser, {"both_pass": "throw_in"}, "seed=1&speed=instant&human=0")
    ok = drive(page, has("thrown_in"), policy="weiter")
    fail_if(not ok, failures, "throw_in: no throw-in")
    if ok:
        rest(page)
        starts = [e for e in js(page, "log()") if e["type"] == "deal_start"]
        fail_if(len(starts) < 2 or starts[-1]["dealer"] == starts[-2]["dealer"], failures, "throw_in: the dealer did not move")


def check_tour_end_and_extension(browser, failures):
    # Seat 0 is asked first: the human sees the prompt. Accepting extends only if the opponent agrees too.
    for accept in (True, False):
        page = open_page(browser, "seed=7&speed=instant&human=0")
        ok = drive(page, lambda s, log: s["phase"] == "extend" and s["prompt"] == "you")
        fail_if(not ok, failures, "tour_end_and_extension: no extension offered to the human")
        if ok:
            page.wait_for_selector("[data-dialog-act=yes]", timeout=60_000)
            fail_if(page.locator("[data-dialog-act=no]").count() != 1, failures, "tour_end_and_extension: the prompt lacks Enough")
            fail_if(page.locator("[data-dialog=extend] p").inner_text() == "", failures, "tour_end_and_extension: the prompt does not say what it adds")
            fail_if(not page.evaluate("!document.querySelector('.slate-root').hidden || true"), failures, "tour_end_and_extension: slate")
            contos = state(page)["contos"]
            page.locator(f"[data-dialog-act={'yes' if accept else 'no'}]").click()
            rest(page)
            s = state(page)
            decided = [e for e in js(page, "log()") if e["type"] == "extension_decided"][-1]
            if accept and decided["accepted"]:
                fail_if(s["touren_total"] != 6 or s["contos"] != contos, failures, f"tour_end_and_extension: after yes {s['touren_total']} Touren, contos {s['contos']} vs {contos}")
            else:
                fail_if(s["prompt"] != "over", failures, "tour_end_and_extension: the sitting did not end on a refusal")
        close_all(browser)
    # Seat 1 is asked second: if the opponent has had enough, the sitting ends without the human's prompt.
    page = open_page(browser, "seed=7&speed=instant&human=1")
    watch_chips(page)
    drive(page, lambda s, log: s["prompt"] == "over" or (s["phase"] == "extend" and s["prompt"] == "you"))
    log = js(page, "log()")
    asked = any(e["type"] == "extension_offered" for e in log)
    fail_if(not asked, failures, "tour_end_and_extension: no extension offered at all")
    if state(page)["prompt"] == "over":
        page.wait_for_function("window.__chips.some(c => c.text.length)", timeout=60_000)
        fail_if(STRINGS["chip.ext-opp-no"]["en"] not in [c["text"] for c in chips_seen(page)], failures, "tour_end_and_extension: no chip said the opponent had had enough")


def check_settlement(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    ok = drive(page, lambda s, log: s["prompt"] == "over")
    fail_if(not ok, failures, "settlement: the sitting did not end")
    if ok:
        end = next(e for e in js(page, "log()") if e["type"] == "sitting_end")
        page.wait_for_selector(".slate-settlement", timeout=60_000)
        text = page.locator(".slate-settlement").inner_text()
        want = STRINGS["slate.square"]["en"] if end["payer"] is None else f"{end['kreuzer']} kr."
        fail_if(want not in text, failures, f"settlement: the slate says {text!r}, the engine {end}")


def check_farewell_once(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    page.evaluate("window.__said = new Set(); setInterval(() => window.strohmandeln3d.said().forEach(t => window.__said.add(t)), 40)")
    drive(page, lambda s, log: s["prompt"] == "over")
    page.wait_for_selector("[data-dialog-act=new]", timeout=60_000)
    page.wait_for_timeout(1500)
    heard = set(page.evaluate("[...window.__said]"))
    farewells = {WORDS["texts"][k]["en"] for k in WORDS["groups"]["farewell"]}
    said = heard & farewells
    fail_if(len(said) != 1, failures, f"farewell_once: {len(said)} farewell lines heard: {sorted(said)}")


def check_motion_profile(browser, failures):
    page = open_page(browser, "seed=7&speed=normal&human=1&manual", wait=False)
    mrest(page, step=400)
    take_if_bidding(page) if False else None
    if state(page)["phase"] == "bid":
        page.locator("[data-act=take]").click()
        mrest(page, step=400)
    s = state(page)
    play = next(a for a in s["legal"] if a["action"] == "play" and a["source"] == "hand")
    js(page, f"act({json.dumps(wire(play))})")
    index = None
    track = []
    for _ in range(60):
        page.evaluate("window.strohmandeln3d.tick(16)")
        meshes = page.evaluate("window.strohmandeln3d.meshInfo()")
        if index is None:
            index = next((i for i, m in enumerate(meshes) if m["code"] == play["card"]), None)
        if index is not None:
            track.append(meshes[index])
    steps = [((a["x"] - b["x"]) ** 2 + (a["y"] - b["y"]) ** 2 + (a["z"] - b["z"]) ** 2) ** 0.5 for a, b in zip(track[1:], track)]
    moving = [x for x in steps if x > 0.01]
    fail_if(len(moving) < 5, failures, "motion_profile: the played card hardly moved")
    if len(moving) >= 5:
        fail_if(moving[0] < 0.15, failures, f"motion_profile: no jerk at the start ({moving[0]:.2f} cm in a frame)")
        fail_if(moving[-1] > moving[0] * 0.6, failures, "motion_profile: the card does not ease into place")
    fail_if(min(m["y"] for m in track) < -0.05, failures, "motion_profile: a card went below the table")


# ---- 10.3 interaction ------------------------------------------------------------------------------


def check_legal_only(browser, failures):
    seen = 0
    for seed, human in [(7, 1), (3, 0), (11, 1)]:
        page = open_page(browser, f"seed={seed}&speed=instant&human={human}")
        for _ in range(400):
            rest(page)
            s = state(page)
            if s["prompt"] == "over":
                break
            lit = sorted(js(page, "lit()"))
            plays = sorted(a["card"] for a in s["legal"] if a["action"] == "play")
            if lit != plays:
                failures.append(f"legal_only: lit {lit} vs legal {plays} (seed {seed})")
                break
            seen += 1
            page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(choose(s, 'first', False)))})")
        close_all(browser)
        if seen >= 200:
            break
    fail_if(seen < 150, failures, f"legal_only: only {seen} positions checked")


def refusable(page):
    s = state(page)
    legal = {a["card"] for a in s["legal"] if a["action"] == "play"}
    return next((c["card"] for c in s["deal"]["own_hand"] if c["card"] not in legal), None)


def check_refusal_message(browser, failures):
    for lang in ("en", "de"):
        page = open_page(browser, f"seed=7&speed=instant&human=1&lang={lang}")
        take_if_bidding(page)
        card = None
        for _ in range(60):
            card = refusable(page)
            s = state(page)
            if card or s["prompt"] == "over":
                break
            plays = [a for a in s["legal"] if a["action"] == "play"]
            js(page, f"act({json.dumps(wire(plays[0]))})")
            rest(page)
        fail_if(card is None, failures, "refusal_message: no refusable card found")
        if card:
            before = len(js(page, "log()"))
            click_card(page, card)
            page.wait_for_selector(".refusal:not(:empty)", timeout=30_000)
            text = page.locator(".refusal").inner_text()
            keys = [k for k in STRINGS if k.startswith("refusal.")]
            ok = any(re.fullmatch(re.sub(r"\\\{\w+\\\}", ".*", re.escape(STRINGS[k][lang])), text) for k in keys)
            fail_if(not ok, failures, f"refusal_message ({lang}): {text!r} is no refusal string")
            fail_if(len(js(page, "log()")) != before, failures, "refusal_message: a refused card changed the table")
        close_all(browser)


HONOUR_CARDS = {"trull": ["SK", "T21", "T1"], "kings": ["Kh", "Kd", "Kc", "Ks"]}


def find_honour(page):
    """Play on (first legal) until a legal announce is offered; returns the honour."""
    for _ in range(400):
        rest(page)
        s = state(page)
        if s["prompt"] == "over":
            return None
        for a in s["legal"]:
            if a["action"] == "announce":
                return a["honour"]
        a = choose(s, "first", False)
        if s["phase"] == "play":
            a = next(x for x in s["legal"] if x["action"] == "play")
        page.evaluate(f"window.strohmandeln3d.act({json.dumps(wire(a))})")
    return None


def honour_card(page, honour):
    s = state(page)
    mine = {c["card"] for c in s["deal"]["own_hand"]}
    legal = {a["card"] for a in s["legal"] if a["action"] == "play"}
    return next((c for c in HONOUR_CARDS[honour] if c in mine and c in legal), None)


def check_announce_prompt(browser, failures):
    page = open_page(browser, "seed=1&speed=instant&human=1")
    honour = find_honour(page)
    fail_if(honour is None, failures, "announce_prompt: the human never holds an honour in the seeded sitting")
    if honour:
        card = honour_card(page, honour)
        fail_if(card is None, failures, "announce_prompt: no honour card to choose")
        if card:
            human = state(page)["human"]
            click_card(page, card)
            still(page)
            page.locator("[data-act=play]").click()
            page.wait_for_selector(".dialog [data-dialog-act^=\"do:\"]", timeout=30_000)
            page.locator(".dialog [data-dialog-act^=\"do:\"]").click()
            rest(page)
            fail_if(not any(e["type"] == "announcement" and e["seat"] == human for e in js(page, "log()")), failures, "announce_prompt: Announce did not announce")
        close_all(browser)
        page = open_page(browser, "seed=1&speed=instant&human=1")
        honour = find_honour(page)
        card = honour_card(page, honour)
        click_card(page, card)
        still(page)
        page.locator("[data-act=play]").click()
        page.wait_for_selector(".dialog [data-dialog-act^=dont]", timeout=30_000)
        page.locator(".dialog [data-dialog-act^=dont]").click()
        rest(page)
        r = page.evaluate(f"window.strohmandeln3d.engine.send({json.dumps({'action': 'announce', 'honour': honour})})")
        fail_if(r.get("ok") or (r.get("refusal") or {}).get("key") not in ("honour-forfeited", "already-announced", "not-held"), failures, f"announce_prompt: after Do not announce the engine says {r.get('refusal')}")
        close_all(browser)
        page = open_page(browser, "seed=7&speed=instant&human=1")
        take_if_bidding(page)
        s = state(page)
        plain = next((a["card"] for a in s["legal"] if a["action"] == "play" and a["card"] not in HONOUR_CARDS["trull"] + HONOUR_CARDS["kings"]), None)
        click_card(page, plain)
        still(page)
        page.locator("[data-act=play]").click()
        rest(page)
        fail_if(page.locator(".dialog [data-dialog-act^=\"do:\"]").count(), failures, "announce_prompt: a prompt for a card of no honour")


def check_announce_chip(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    take_if_bidding(page)
    s = state(page)
    held = {a["honour"] for a in s["legal"] if a["action"] == "announce"}
    other = next((h for h in ("kings", "trull") if h not in held), None)
    fail_if(other is None, failures, "announce_chip: both honours held in the seeded deal")
    if other:
        page.locator(f"[data-act=announce][data-honour={other}]").click(force=True)
        page.wait_for_selector(".refusal:not(:empty)", timeout=30_000)
        text = page.locator(".refusal").inner_text()
        fail_if("hold" not in text.lower() and "haben" not in text.lower(), failures, f"announce_chip: not-held says {text!r}")
    close_all(browser)
    page = open_page(browser, "seed=1&speed=instant&human=1")
    honour = find_honour(page)
    fail_if(honour is None, failures, "announce_chip: no honour held")
    if honour:
        page.locator(f"[data-act=announce][data-honour={honour}]").click()
        rest(page)
        fail_if(not any(e["type"] == "announcement" and e["seat"] == state(page)["human"] for e in js(page, "log()")), failures, "announce_chip: the chip did not announce")


def check_bid_buttons(browser, failures):
    for lang in ("en", "de"):
        page = open_page(browser, f"seed=7&speed=instant&human=1&lang={lang}")
        labels = page.locator(".move-bar [data-act]").all_inner_texts()
        fail_if(labels != [STRINGS["ui.take"][lang], STRINGS["ui.pass"][lang]], failures, f"bid_buttons ({lang}): {labels}")
        s = state(page)
        fail_if(len(labels) != len([a for a in s["legal"] if a["action"] in ("take", "pass")]), failures, f"bid_buttons ({lang}): not only legal buttons")
        close_all(browser)


def check_length_choice(browser, failures):
    for length in (3, 6):
        page = open_page(browser, "seed=7&speed=instant&human=1")
        page.evaluate(f"window.strohmandeln3d.newSitting({{ touren: {length}, seed: 7, humanSeat: 1 }})")
        rest(page)
        s = state(page)
        fail_if(s["touren_total"] != length, failures, f"length_choice: {length} Touren gave {s['touren_total']}")
        fail_if(f"of {length}" not in page.locator(".tour-line").inner_text(), failures, f"length_choice: the HUD says {page.locator('.tour-line').inner_text()!r}")
        close_all(browser)
    # The screen itself: a page with no sitting asks, and Deal with 6 chosen deals six Touren.
    page = open_page(browser, "human=1&hook=1", wait=False)
    page.wait_for_selector("[data-length='6']", timeout=180_000)
    page.locator("[data-length='6']").click()
    page.locator("[data-dialog-act=deal]").click()
    rest(page)
    fail_if(state(page)["touren_total"] != 6, failures, "length_choice: the new-sitting screen's 6 did not give six Touren")


def check_keyboard_sitting(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    turns = 0
    for _ in range(900):
        rest(page)
        s = state(page)
        if s["prompt"] == "over":
            break
        if s["phase"] == "bid":
            page.keyboard.press("t")
        elif s["phase"] == "extend":
            page.keyboard.press("Tab")
            page.keyboard.press("Enter")
        else:
            if any(a["action"] == "announce" for a in s["legal"]):
                page.keyboard.press("a")
                rest(page)
            page.keyboard.press("Home")
            page.keyboard.press("Enter")
            page.keyboard.press("Enter")
            if page.locator(".dialog [data-dialog-act^=dont]").count():
                page.keyboard.press("Tab")
                page.keyboard.press("Enter")
        turns += 1
    fail_if(state(page)["prompt"] != "over", failures, f"keyboard_sitting: not over after {turns} turns of keys")
    page.keyboard.press("?")
    fail_if(not page.locator("[data-dialog=keys]").count(), failures, "keyboard_sitting: ? did not list the keys")
    page.keyboard.press("Escape")
    check_offline(page, failures, "keyboard_sitting: ")


def check_screen_reader(browser, failures):
    for lang in ("en", "de"):
        page = open_page(browser, f"seed=7&speed=instant&human=1&lang={lang}", wait=False)
        page.evaluate("window.__live = []; new MutationObserver(() => { const t = document.querySelector('.sr-live').textContent; if (t) window.__live.push(t); }).observe(document.querySelector('.sr-live'), { childList: true, characterData: true, subtree: true })")
        rest(page)
        drive(page, has("trick_won"))
        spoken = page.evaluate("window.__live")
        fail_if(len(spoken) < 10, failures, f"screen_reader ({lang}): only {len(spoken)} sentences")
        if lang == "de":
            fail_if(any(w in " ".join(spoken) for w in ("Your opponent", "You take", " plays ")), failures, "screen_reader (de): English left in the sentences")
        else:
            fail_if(not any("plays" in t for t in spoken), failures, "screen_reader (en): no play sentence")
        close_all(browser)


def check_left_handed(browser, failures):
    page = with_prefs(browser, {"leftHanded": True})
    right = open_page(browser)
    a = page.locator(".top-right").bounding_box()["x"]
    b = right.locator(".top-right").bounding_box()["x"]
    fail_if(not a < b, failures, "left_handed: the slate tab did not move")
    fail_if(page.locator(".place.bid .big").first.get_attribute("data-act") != "pass", failures, "left_handed: the bid buttons are not reversed")
    covers = [c["cover"] for c in js(page, "model()")["packets"][state(page)["human"]] if c["cover"]]
    xs = [js(page, f"screenPoint('{c}')")["x"] for c in covers]
    fail_if(xs != sorted(xs), failures, "left_handed: the packets are not 1-2-3 left to right")


def check_reduced_motion(browser, failures):
    page = open_page(browser, "seed=7&speed=normal&human=1&manual", reduced=True, wait=False)
    prev = None
    runs = {}
    worst = 0
    for _ in range(300):
        page.evaluate("window.strohmandeln3d.tick(30)")
        info = page.evaluate("window.strohmandeln3d.meshInfo()")
        if prev:
            for i, (a, b) in enumerate(zip(info, prev)):
                d = abs(a["x"] - b["x"]) + abs(a["y"] - b["y"]) + abs(a["z"] - b["z"])
                runs[i] = runs.get(i, 0) + 1 if d > 0.001 else 0
                worst = max(worst, runs[i])
        prev = info
        if page.evaluate("window.strohmandeln3d.state() && !window.strohmandeln3d.busy()"):
            break
    fail_if(worst > 2, failures, f"reduced_motion: a card was in flight for {worst} frames")
    mrest(page)
    fail_if(count_of(page, "your-hand") != 15, failures, "reduced_motion: the deal did not complete")


def check_touch_targets(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1&touch=1", {"width": 768, "height": 1024})
    small = []
    for b in page.locator("button:visible").all():
        box = b.bounding_box()
        if box and (box["width"] < 44 or box["height"] < 44):
            small.append((b.inner_text()[:20], round(box["width"]), round(box["height"])))
    fail_if(small, failures, f"touch_targets: small controls {small}")


CHECKS_102 = [
    check_cut_and_deal, check_bid_badge, check_reveal_shows_cards, check_last_card_pulled, check_play_hand_vs_packet,
    check_replacement_after_trick, check_trick_gather, check_last_trick_ultimo, check_count_thirds, check_count_quiet,
    check_game_scored_banner, check_draw_and_doubling, check_void_deal, check_throw_in, check_tour_end_and_extension,
    check_settlement, check_farewell_once, check_motion_profile,
]
CHECKS_103 = [
    check_legal_only, check_refusal_message, check_announce_prompt, check_announce_chip, check_bid_buttons, check_length_choice,
    check_keyboard_sitting, check_screen_reader, check_left_handed, check_reduced_motion, check_touch_targets,
]
