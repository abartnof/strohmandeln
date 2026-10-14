#!/usr/bin/env python3
"""Harvest ANNO (Austrian National Library newspapers) hits and snippets.

Uses the JSON service behind https://anno.onb.ac.at/anno-suche/ :
  /anno-suche/rest/search/simple?query=..&from=N&facets=true   (10 hits a page)
  /anno-suche/rest/search/snippet?documentId=..&query=..        (page + text)

The page-text view (annoshow) sits behind a bot check and is not used.

    python3 tools/anno.py OUT.jsonl TERM [TERM ...]

Each line of OUT.jsonl is one document: term, docId, title, date, and its
snippets (page, text with <br/> kept as newlines, image crop URL, open URL).
Already-harvested (term, docId) pairs are skipped, so a run can be resumed.
"""
import html
import json
import re
import sys
import time
import urllib.parse
import urllib.request

BASE = "https://anno.onb.ac.at/anno-suche/rest/search"


def get(path, **params):
    url = f"{BASE}/{path}?{urllib.parse.urlencode(params)}"
    for attempt in range(4):
        try:
            with urllib.request.urlopen(url, timeout=60) as r:
                return json.load(r)
        except Exception as e:  # network hiccup: back off and retry
            if attempt == 3:
                raise
            print(f"  retry {url}: {e}", file=sys.stderr)
            time.sleep(2 ** attempt)


def clean(snippet):
    s = snippet.replace("<br/>", "\n")
    s = re.sub(r"<[^>]+>", "", s)
    return html.unescape(s)


def harvest(term, out, seen):
    first = get("simple", query=term, **{"from": 1}, facets="true")
    total = first["totalHits"]
    print(f"{term}: {total} documents", file=sys.stderr)
    start, page = 1, first
    while True:
        for d in page["documents"]:
            key = (term, d["docId"])
            if key in seen:
                continue
            sn = get("snippet", documentId=d["docId"], query=term)
            rec = {
                "term": term,
                "docId": d["docId"],
                "title": d["displayTitle"],
                "date": (d["uid"].get("annoDate") or {}).get("date") or d["uid"].get("year"),
                "uid": d["uid"],
                "type": d.get("type"),
                "places": d.get("places"),
                "snippets": [
                    {
                        "page": s["pageLabel"],
                        "text": clean(s["text"]),
                        "image": s.get("imageURL"),
                        "open": s.get("openURL"),
                    }
                    for p in sn.get("snippetPages", [])
                    for s in p["snippets"]
                ],
            }
            out.write(json.dumps(rec, ensure_ascii=False) + "\n")
            out.flush()
            seen.add(key)
            time.sleep(0.3)
        start += 10
        if start > total:
            break
        page = get("simple", query=term, **{"from": start}, facets="true")


def main():
    path, terms = sys.argv[1], sys.argv[2:]
    seen = set()
    try:
        with open(path, encoding="utf-8") as f:
            for line in f:
                r = json.loads(line)
                seen.add((r["term"], r["docId"]))
    except FileNotFoundError:
        pass
    with open(path, "a", encoding="utf-8") as out:
        for t in terms:
            harvest(t, out, seen)


if __name__ == "__main__":
    main()
