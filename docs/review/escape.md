# Review: Escape closes the topmost overlay (536501c)

Scope: `web3d/src/overlays.js`, the Escape handler and `closeLayer` in `main.js`, `dialogIds()` and focus return in `ui.js`, the removed handlers in `settings.js` and `slate.js`, TABLE3D 3.10.5, tests `overlays.test.js` and browser check `escape_closes`.

Run: `node --test kit/test web3d/test` 543 pass, 0 fail; `kit/build.py --check` clean; `browser.py --only=escape_closes` ok.

## Verified

- Order (dialogs, last opened first > settings > slate > HUD list > tracker > selection) is what `escapeTarget` does; a press closes one layer and returns, `preventDefault` only when something closed.
- No double close: the only other `keydown` listener is `mirror.js` (arrows, Enter, Space, 1-3; Escape unused). Nothing in `kit/` or `web3d/src` uses native `<dialog>`, `showModal` or a `cancel` event; dialogs are `div.shade` boxes. The Escape check runs before the `.settings`/`.slate` input guard, so Escape reaches it from inside those panels.
- The public-mark note is not a dialog (a card mark and the inline Kibitzer note, `.tutor-note`), so it is rightly not a layer.
- Dialog ids in use: page, help, review, over, error, extend, announce, new, clear, keys, welcome. All are covered by the tests or the answer-required set.

## Findings

1. Medium. Native `<select>` in the settings menu (another branch adds one). The handler is on `window` and does not look at the target. Chrome swallows Escape while the popup is open, but Firefox and Safari behaviour is not established and headless Chromium cannot test it. If the key reaches the page, one press closes the select and the whole settings menu. Fix: track whether a select is open (set on its `mousedown`, `click` or opening keys; clear on `change`, `blur` and Escape) and have the Escape branch return early for an open select; add a browser check when that select lands.
2. Medium. Required dialogs (`extend`, `over`, `welcome`, `new` without Cancel) give a dead key. Refusing to choose for the player is right for `extend` (changes the sitting) and `over`. `welcome` is the one that could safely be dismissed: its default is Continue and nothing is lost. Fix: on a blocked press, `preventDefault` and move focus to the dialog's `[data-focus]` button (this also rescues a keyboard user whose focus is lost). If the maintainer wants "any dialog" literally, make Escape on `welcome` act as Continue and keep `extend`/`over` focus-only. Record the choice in 3.10.5.
3. Low. Focus return when the opener is gone or hidden is silent. `ui.closeDialog` skips it when `opener.isConnected` is false (a played hand card, a settings row redrawn by a language switch), and a hidden opener takes `focus()` as a no-op. `settings.open` records `document.activeElement`, which is `body` after a mouse click in Safari (buttons are not focused by click), so the cog is not focused as 3.10.5 promises. Fix: fall back to the cog when the opener is `body`, disconnected or hidden; for dialogs fall back to the existing `settleFocus()`.
4. Low (tests). `overlays.test.js` is meaningful for the order and the answer-required set, but covers the pure function only. The wiring is untested: `page` closing as "Remind me", `announce` closing without playing and leaving the card chosen, `new` Cancel with and without a sitting, `extend`/`over`/`welcome` swallowing Escape, selection clearing. `escape_closes` covers help, clear, keys, settings, slate, HUD and tracker. Add browser steps for page and announce at least. The settings unit test now asserts Escape no longer closes the menu, which pins the removal and is fine.
5. Low (process). The author states the pure test was written with the code, not first. The tests would have failed before (module absent) and were not weakened, so no rework; recorded as a lapse against CLAUDE.md.
6. Info. `ui.dialogIds()` reads DOM order, which is open order because `dialog()` appends and a re-opened id is removed first, so it moves to the top, as it should.

## Fixes

- Finding 1: `selectGuard` (overlays.js) tracks an open native select (mousedown, Alt+Arrow, Space, F4 open it; change, focusout, Escape clear it) and the Escape branch returns early for that press. Node test only: main has no `<select>` yet, so no browser step; add one when the settings split lands.
- Finding 2: `welcome` acts as Continue. `extend`, `over` and `new` without Cancel: `escapeTarget` returns `{kind: "focus"}`, the handler prevents default and focuses the dialog's `[data-focus]` button.
- Finding 3: focus falls back from a gone, hidden or body opener to the cog (settings) or, for dialogs, the move bar's first button then the cog. `settleFocus()` is the tutor's focus decision, not a DOM focus, so it is not used.
- Finding 4: `escape_dialogs` covers a tutor page (closes, Help stays), the announce prompt (closes, nothing played, card still in hand) and the extension prompt (stays, focus on the dialog). `new` Cancel and `over` are covered by the pure test only.
- The public-mark note (`note`, now a dialog on main) is a plain dismissible layer; the ids on main are page, help, review, over, error, extend, announce, new, clear, keys, welcome, note, all handled.
