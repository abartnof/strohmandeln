#!/usr/bin/env python3
"""The table in a real browser, offline (docs/TABLE3D.md section 10).

    python3 kit/build.py
    <python with Playwright> web3d/test/browser.py [--quick] [--only=a,b] [screenshot-dir]

Needs Playwright's Python package and the system Chromium (/usr/bin/chromium,
from apt). On the development VM piquet's venv has Playwright:
`~/piquet/.venv/bin/python web3d/test/browser.py`. WebGL runs on SwiftShader.
The page is opened from a file:// URL with the network off; every check runs
in a fresh context and fails on a console error or a network request.

From cassino web3d/test/browser.py (its helpers). The page's debug hook,
`window.strohmandeln3d`, is what the checks read; the cards are found on the
screen with its `screenPoint` and clicked, as a person would.
"""

import io
import json
import re
import sys
from pathlib import Path

from PIL import Image
from playwright.sync_api import sync_playwright

ROOT = Path(__file__).resolve().parents[2]
PAGE = ROOT / "web3d" / "strohmandeln.html"
SHOTS = next((Path(a) for a in sys.argv[1:] if not a.startswith("--")), None)
LOCAL = ("file:", "data:", "blob:")
STRINGS = json.loads((ROOT / "web3d" / "strings.json").read_text())
WORDS = json.loads((ROOT / "web3d" / "words.json").read_text())
LAUNCH = dict(executable_path="/usr/bin/chromium", args=["--no-sandbox", "--use-angle=swiftshader", "--use-gl=angle", "--enable-unsafe-swiftshader", "--disable-gpu-compositing"])
LANDSCAPE = {"width": 1280, "height": 720}
PORTRAIT = {"width": 390, "height": 844}
TABLET = {"width": 1024, "height": 768}


def shot(page, name):
    if SHOTS:
        SHOTS.mkdir(parents=True, exist_ok=True)
        page.screenshot(path=str(SHOTS / f"{name}.png"))


def close_all(browser):
    for context in list(browser.contexts):
        context.close()


# The software GPU spends about a second on each frame; a flow that reads no pixels
# (a whole sitting played by the engine's actions) turns the draw calls into no-ops.
NO_DRAW = """for (const C of [window.WebGLRenderingContext, window.WebGL2RenderingContext]) { if (!C) continue;
  for (const n of ['drawElements', 'drawArrays', 'drawElementsInstanced', 'drawArraysInstanced', 'drawRangeElements']) if (C.prototype[n]) C.prototype[n] = function () {}; }"""


def open_page(browser, query="seed=7&speed=instant", viewport=None, reduced=False, wait=True, nodraw=False):
    """A fresh context, offline, recording every request and console error."""
    context = browser.new_context(viewport=viewport or LANDSCAPE, reduced_motion="reduce" if reduced else "no-preference")
    context.set_offline(True)
    if nodraw:
        context.add_init_script(NO_DRAW)
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


def js(page, expr):
    return page.evaluate(f"window.strohmandeln3d.{expr}")


def rest(page, timeout=180_000):
    """Wait for the cards to come to rest and the opponent to have moved: the
    human's turn, or the end."""
    page.wait_for_function(
        "(() => { const h = window.strohmandeln3d; const s = h && h.state(); return Boolean(s) && !h.busy() && s.prompt !== 'opponent'; })()",
        timeout=timeout,
    )


def still(page, timeout=180_000):
    """Wait for the cards to come to rest, whoever's turn it is."""
    page.wait_for_function("(() => { const h = window.strohmandeln3d; return Boolean(h && h.state()) && !h.busy(); })()", timeout=timeout)


def state(page):
    return js(page, "state()")


def click_card(page, code):
    point = js(page, f"screenPoint('{code}')")
    assert point, f"{code} is not on the table"
    page.mouse.click(point["x"], point["y"])


def canvas_image(page):
    return Image.open(io.BytesIO(page.locator("canvas#stage").screenshot())).convert("RGB")


def dialog_buttons(page):
    return page.locator(".dialog [data-dialog-act]")


def play_one(page, take=True, extend=False, pick=None):
    """One decision of the human, by clicking. Returns False at the end."""
    s = state(page)
    if s["prompt"] == "over":
        return False
    if s["phase"] == "bid":
        page.locator(f"[data-act={'take' if take else 'pass'}]").click()
    elif s["phase"] == "extend":
        page.locator(f"[data-dialog-act={'yes' if extend else 'no'}]").click()
    else:
        plays = [a for a in s["legal"] if a["action"] == "play"]
        a = (pick or (lambda p: p[0]))(plays)
        click_card(page, a["card"])
        still(page)
        page.locator("[data-act=play]").click()
        if page.locator("[data-dialog-act^=dont]").count():
            page.locator("[data-dialog-act^=dont]").click()
    rest(page)
    return True


def check_offline(page, failures, where=""):
    remote = [u for u in page.requests if not u.startswith(LOCAL)]
    if remote:
        failures.append(f"{where}the page asked the network for {len(remote)} things: {remote[:5]}")
    pages = {u.split("?")[0] for u in page.requests if u.startswith("file:")}
    if len(pages) != 1:
        failures.append(f"{where}expected one file loaded, got {pages}")
    if page.errors:
        failures.append(f"{where}console errors: {page.errors[:5]}")


def fail_if(cond, failures, message):
    if cond:
        failures.append(message)


def secret(page):
    """Faces shown that the engine says are hidden from the human."""
    return sorted(set(c for c in js(page, "shown()") if c) & set(js(page, "hidden()")))


def intersects(a, b):
    return a["left"] < b["right"] - 1 and b["left"] < a["right"] - 1 and a["top"] < b["bottom"] - 1 and b["top"] < a["bottom"] - 1


# ---- 10.1 page and layout ------------------------------------------------------------------

def check_offline_opens(browser, failures):
    page = open_page(browser)
    small = canvas_image(page).resize((160, 100))
    colours = small.getcolors(maxcolors=160 * 100)
    fail_if(len(colours) < 4, failures, f"offline_opens: the canvas is nearly flat ({len(colours)} colours)")
    fail_if(sum(n for n, (r, g, b) in colours if r + g + b < 60) > 8000, failures, "offline_opens: the canvas is mostly black")
    fail_if(js(page, "meshInfo()").__len__() != 54, failures, "offline_opens: not 54 cards")
    check_offline(page, failures, "offline_opens: ")
    shot(page, "landscape")


def layout_checks(browser, failures, name, query, viewport):
    page = open_page(browser, query, viewport)
    rects = js(page, "zoneRects()")
    names = [z for z in rects if z in ("your-hand", "their-hand", "your-packets", "their-packets", "trick", "your-won", "their-won")]
    for i, a in enumerate(names):
        for b in names[i + 1:]:
            fail_if(intersects(rects[a], rects[b]), failures, f"{name}: zones {a} and {b} overlap: {rects[a]} {rects[b]}")
    # Each badge equals hidden (1.5.4).
    s = state(page)
    hidden = [h for row in s["deal"]["hidden"] for h in row]
    badges = [int(t) for t in page.locator(".tag.badge").all_inner_texts()]
    fail_if(sorted(badges) != sorted(h for h in hidden if h > 0), failures, f"{name}: badges {badges} vs hidden {hidden}")
    # A cover's index is not covered by the badge (1.5.2): the badge sits on the opposite corner.
    covers = js(page, "cardRects('your-packets')") + js(page, "cardRects('their-packets')")
    for box in page.locator(".tag.badge").all():
        b = box.bounding_box()
        for c in covers:
            top_left = {"left": c["left"], "right": c["left"] + (c["right"] - c["left"]) * 0.45, "top": c["top"], "bottom": c["top"] + (c["bottom"] - c["top"]) * 0.4}
            fail_if(intersects({"left": b["x"], "right": b["x"] + b["width"], "top": b["y"], "bottom": b["y"] + b["height"]}, top_left), failures, f"{name}: a badge covers a cover's index")
    check_offline(page, failures, f"{name}: ")
    shot(page, name)
    return page


def check_layout_landscape(browser, failures):
    layout_checks(browser, failures, "layout_landscape", "seed=7&speed=instant&human=1", LANDSCAPE)


def check_layout_portrait(browser, failures):
    layout_checks(browser, failures, "layout_portrait", "seed=7&speed=instant&human=1", PORTRAIT)


def check_layout_touch(browser, failures):
    layout_checks(browser, failures, "layout_touch", "seed=7&speed=instant&human=1&touch=1", {"width": 768, "height": 1024})


def check_cover_legibility(browser, failures):
    for viewport, query in [(LANDSCAPE, "seed=7&speed=instant&human=1"), (PORTRAIT, "seed=7&speed=instant&human=1")]:
        page = open_page(browser, query, viewport)
        # A few plays so that covers exist: play until the opening reveal is done (it is, at rest).
        covers = js(page, "cardRects('your-packets')")
        hand = js(page, "cardRects('your-hand')")
        fail_if(not covers or not hand, failures, "cover_legibility: no covers or hand")
        if covers and hand:
            ch = sorted((c["bottom"] - c["top"]) for c in covers)[len(covers) // 2]
            hh = min((c["bottom"] - c["top"]) for c in hand)  # the middle of a fan is turned least
            fail_if(abs(ch - hh) > 0.15 * hh + 1, failures, f"cover_legibility at {viewport['width']}: a cover {ch:.0f} px tall, a hand card {hh:.0f}")
        close_all(browser)


def check_no_face_leak(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1", {"width": 800, "height": 450})
    made = 0
    while True:
        bad = secret(page)
        fail_if(bad, failures, f"no_face_leak: faces {bad} shown that the view does not give, after {made} decisions")
        if bad or not play_one(page, extend=False):
            break
        made += 1
        if made % 20 == 0:
            print(f"  no_face_leak: {made} decisions, prompt {state(page)['prompt']}, trick {(state(page)['deal'] or {}).get('trick_number')}", flush=True)
        if made > 600:
            failures.append("no_face_leak: the sitting did not end")
            break
    fail_if(state(page)["prompt"] != "over", failures, "no_face_leak: the sitting did not end by clicking")
    check_offline(page, failures, "no_face_leak: ")


def check_phone_row(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1", PORTRAIT)
    hand = js(page, "cardRects('your-hand')")
    ys = {round(c["top"]) for c in hand}
    fail_if(max(ys) - min(ys) > 4, failures, f"phone_row: the hand is not one row: tops {sorted(ys)}")
    fail_if(len(hand) != 15, failures, f"phone_row: {len(hand)} cards")
    # It scrolls: wheel over the hand until the last card is on the screen.
    seen = set()
    for step in [300] * 6 + [-300] * 12:
        for code in js(page, "model().hand.map(c => c.code)"):
            p = js(page, f"screenPoint('{code}')")
            if p and 10 <= p["x"] <= PORTRAIT["width"] - 10:
                seen.add(code)
        page.mouse.move(195, 640)
        page.mouse.wheel(0, step)
        page.wait_for_timeout(250)
    for code in js(page, "model().hand.map(c => c.code)"):
        p = js(page, f"screenPoint('{code}')")
        if p and 10 <= p["x"] <= PORTRAIT["width"] - 10:
            seen.add(code)
    fail_if(len(seen) < 15, failures, f"phone_row: only {len(seen)} of 15 cards reached by scrolling")


def check_window_resize(browser, failures):
    page = open_page(browser, "seed=7&speed=instant&human=1")
    before = sorted(js(page, "faces()"))
    page.set_viewport_size(PORTRAIT)
    page.wait_for_timeout(600)
    still(page)
    page.set_viewport_size(LANDSCAPE)
    page.wait_for_timeout(600)
    still(page)
    after = sorted(js(page, "faces()"))
    fail_if(before != after, failures, "window_resize: the cards changed across a resize")
    fail_if(len(js(page, "meshInfo()")) != 54, failures, "window_resize: a card was lost")
    check_offline(page, failures, "window_resize: ")


CHECKS_101 = [check_offline_opens, check_layout_landscape, check_layout_portrait, check_layout_touch, check_cover_legibility, check_no_face_leak, check_phone_row, check_window_resize]
ALL = list(CHECKS_101)


def run(checks, failures):
    with sync_playwright() as p:
        browser = p.chromium.launch(**LAUNCH)
        for check in checks:
            before = len(failures)
            try:
                check(browser, failures)
            except Exception as error:  # a check that cannot run is a failure, named
                failures.append(f"{check.__name__}: {type(error).__name__}: {str(error)[:300]}")
            print(f"{'ok  ' if len(failures) == before else 'FAIL'} {check.__name__}", flush=True)
            for f in failures[before:]:
                print(f"       {f}"[:600], flush=True)
            close_all(browser)
        browser.close()


def main() -> int:
    import browser_aids as aids
    import browser_scenarios as scenarios

    failures = []
    names = next((a.split("=", 1)[1].split(",") for a in sys.argv[1:] if a.startswith("--only=")), None)
    every = {c.__name__: c for c in CHECKS_101 + scenarios.CHECKS_102 + scenarios.CHECKS_103 + aids.CHECKS_104}
    if names:
        checks = [every[n if n.startswith("check_") else f"check_{n}"] for n in names]
    elif "--quick" in sys.argv:
        checks = [check_offline_opens, check_layout_landscape]
    else:
        checks = list(every.values())
    run(checks, failures)
    for f in failures:
        print("FAIL:", f)
    print("browser: ok" if not failures else f"browser: {len(failures)} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
