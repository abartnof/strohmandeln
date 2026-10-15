// The browser suite covers docs/TABLE3D.md section 10, or says where it does not:
// every check named there is registered in the suite or listed in NOT_YET.
import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";

const read = (p) => readFileSync(new URL(p, import.meta.url), "utf8");

// Named in section 10 and not yet written (docs/review/browser-suite.md). Remove a name when its check lands.
const NOT_YET = new Set([
  "slate_reconstruction_note",
  "talk_timing",
  "talk_cross_group", // the suite's check_talk_repeat bounds repeats instead
  "opponent_unnamed",
  "tutor_mute",
  "hint_and_explain",
  "deal_review",
  "strings_complete",
  "single_file",
  "full_sitting_3",
  "full_sitting_6_extended",
  "phone_sitting",
  "watch_reload",
]);

// A spec name that a registered check covers under another name.
const ALIASES = { settings_rules: "settings_rows" };

function specNames() {
  const doc = read("../../docs/TABLE3D.md");
  const start = doc.indexOf("## 10.");
  const end = doc.indexOf("## 11.");
  assert.ok(start > 0 && end > start, "section 10 not found");
  const section = doc.slice(start, end);
  return [...section.matchAll(/^- `([a-z0-9_]+)`/gm)].map((m) => m[1]);
}

function registered() {
  const names = new Set();
  for (const f of ["browser.py", "browser_scenarios.py", "browser_aids.py"]) {
    for (const block of read(f).matchAll(/^CHECKS_10\d = \[([^\]]*)\]/gms)) {
      for (const m of block[1].matchAll(/check_([a-z0-9_]+)/g)) names.add(m[1]);
    }
  }
  return names;
}

test("section 10 names a good number of checks", () => {
  assert.ok(specNames().length >= 40);
});

test("every check named in section 10 is registered or in NOT_YET", () => {
  const have = registered();
  const missing = specNames().filter((n) => !have.has(ALIASES[n] ?? n) && !NOT_YET.has(n));
  assert.deepEqual(missing, [], "named in TABLE3D section 10 but neither registered nor in NOT_YET");
});

test("NOT_YET lists only names the spec has and the suite lacks", () => {
  const have = registered();
  const spec = new Set(specNames());
  for (const n of NOT_YET) {
    assert.ok(spec.has(n), `${n} is not named in section 10`);
    assert.ok(!have.has(n), `${n} is registered now: remove it from NOT_YET`);
  }
});
