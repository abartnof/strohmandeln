#!/usr/bin/env python3
"""Find browser-check seeds whose precondition holds, at ?level=1.

Browser seeds depend on the opponent at level 1: a change to the opponent or to
the deal makes some seeded preconditions fail ("no ultimo lost", "no draw in
the seeded sitting", "no play sentence", the first deal naming no focus).

Rerun after an opponent change (each seed plays a sitting in the page, no
drawing; about 10 to 90 s a seed). The ultimo and draw preconditions are scanned
much faster, in Node without a browser, by web3d/test/seed_scan.mjs FIRST LAST
[first|weiter] (a few hundred seeds in a minute); use this script for `sentences`
and `focus`, and to confirm a seed in the page:

    ~/piquet/.venv/bin/python web3d/test/seed_probe.py MODE FIRST LAST

MODE is one of
    ultimo                    play the sitting out and print the results of its pagat_ultimo events, `won` and
                              `lost` (last_trick_ultimo needs one seed with each; human=1); prints ok when both occur
    draw                      a draw, then the doubling applied (draw_and_doubling, slate_rows; human=1, "weiter")
    sentences                 an English screen-reader sentence with "plays" and 10 or more sentences
                              by the first trick won (screen_reader; human=1)
    focus                     the first scored deal names a focus and every bullet has a bold lead
                              (deal_brief; the hint is asked at the first decision)
Each line printed is `seed ok|no`. Put a seed that is ok in the check and note in
its comment that level 1 is deterministic, so the seed needs this rescan after an opponent change.
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

from playwright.sync_api import sync_playwright

import browser as B
from browser_aids import brief_page, scored_deal
from browser_scenarios import drive, has, until_over
from browser import js, open_page, rest, state


def probe(br, mode, seed):
    query = f"seed={seed}&speed=instant&human=1"
    if mode == "focus":
        page = brief_page(br, {}, query=query)
        page.wait_for_selector("[data-act=hint]", timeout=30_000)
        page.locator("[data-act=hint]").click()
        page.wait_for_function("window.strohmandeln3d.briefing().assisted.length > 0", timeout=10_000)
        scored_deal(page)
        page.wait_for_selector(".tutor-review:not([hidden])", timeout=10_000)
        page.locator(".tutor-review").click()
        page.wait_for_selector("[data-dialog=review]", timeout=10_000)
        bullets = page.locator("[data-dialog=review] li")
        return js(page, "briefing()")["focus"] is not None and all(bullets.nth(i).locator("b").count() for i in range(bullets.count()))
    page = open_page(br, query, wait=mode != "sentences", nodraw=True)
    if mode == "ultimo":
        drive(page, until_over())
        seen = sorted({e["result"] for e in js(page, "log()") if e["type"] == "pagat_ultimo"})
        print(f"  {seed}: {seen}", flush=True)
        return len(seen) == 2
    if mode == "draw":
        return drive(page, has("game_scored", result="draw"), policy="weiter") and drive(page, has("doubling_applied"), policy="weiter")
    if mode == "sentences":
        page.evaluate("window.__live = []; new MutationObserver(() => { const t = document.querySelector('.sr-live').textContent; if (t) window.__live.push(t); }).observe(document.querySelector('.sr-live'), { childList: true, characterData: true, subtree: true })")
        rest(page)
        drive(page, has("trick_won"))
        spoken = page.evaluate("window.__live")
        return len(spoken) >= 10 and any("plays" in t for t in spoken)
    raise SystemExit(f"unknown mode {mode}")


if __name__ == "__main__":
    mode, first, last = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
    with sync_playwright() as p:
        br = p.chromium.launch(**B.LAUNCH)
        for seed in range(first, last + 1):
            try:
                ok = probe(br, mode, seed)
            except Exception as e:  # a seed that cannot be played is not a candidate
                ok, _ = False, print(f"  {seed}: {type(e).__name__}", flush=True)
            print(seed, "ok" if ok else "no", flush=True)
            B.close_all(br)
