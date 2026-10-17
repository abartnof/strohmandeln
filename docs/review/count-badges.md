# Review: count badges (TABLE3D 2.5.2)

Scope: `git diff main...HEAD` (ui.js `countBadges`, counting.js `groupArithmetic`/`countBadges`,
strings.js `badgeName`, `hud.badge.*`, main.js placement, style.css, countbadges.test.js,
browser check `count_badges`, TABLE3D 2.5.2, tutorials).

Checked and sound
- Arithmetic matches RULES 8.1: card = 3 x value - 2 thirds, so a group is sum of values less
  2/3 a card (3 cards: less 2; pair: less 1 1/3; single: less 2/3). Tests cover the example, a
  pair, a single, the named tarocks, and en/de.
- Badge thirds sum to the engine's `count` thirds per side, per group of three with the last
  group 1-2 cards (unit test over simulated sittings; browser test against the log, normal and
  reduced motion).
- German parity: all `hud.badge.*` keys have en and de; sentence order is the same.
- Accessibility: `role=img`, `aria-label` = arithmetic, `tabindex=0` in counted order, tooltip
  `aria-hidden` (no double reading), visible focus ring, Escape hushes, tooltip nudged inside window.
- Reduced motion: badges appear at their line with no wait; no animation on the badges.
- Cleanup: `deal_start` clears `countShown`; the layer is reconciled on every render, so badges
  go with it; language switch re-renders and the text/aria-label update; resize re-renders.

Findings
1. Low, fixed: the badge and tooltip declared their own font stacks (`-apple-system ... SF Pro Text`,
   `system-ui`) instead of the page's global font. Now `font-family: inherit`; test added
   (`countbadges.test.js`).
2. Info, open (accepted): no badges at Instant speed (`calm()` path never schedules them) and none on a
   restored sitting (`countShown` is only set while the count plays). The HUD still shows each side's
   count; the arithmetic is unavailable there. Does not matter for correctness; revisit if the
   maintainer wants the arithmetic after a restore.
3. Info, open: the document-level Escape handler hushes badges without checking whether an
   overlay took the key; harmless (no preventDefault), the overlay still closes.

Tests: `node --test kit/test web3d/test` 576 pass; `kit/build.py --check` clean;
browser `--only=count_badges` ok.
