# The wire protocol

`strohmandeln-wasm` is the engine as a WebAssembly module with a JSON
protocol. A client (the web table, the tests, anything that replaces them)
holds no game logic: it creates a sitting, reads the state, sends the human's
actions and asks the module to step the opponent. Everything it receives is
the human's view (`docs/RULES.md` 13): the other hand and every face-down
packet card are never in it. Message keys (`docs/MESSAGES.md`) are the only
text; the client translates them.

Wire version: `protocol: 1` (`strohmandeln_wasm::PROTOCOL`). It is in every
reply and export.

## Building

```sh
cargo build --release -p strohmandeln-wasm --lib --target wasm32-unknown-unknown
# -> target/wasm32-unknown-unknown/release/strohmandeln_wasm.wasm
node crates/strohmandeln-wasm/tests/smoke.mjs      # golden and random sittings
```

`bin/gate` does both when `web3d/node_modules` exists. No `wasm-bindgen`: the
module has no imports.

## The ABI

Text goes in and out through linear memory, as in Cassino. `kit/js/engine.js`
(`loadWasm(bytes, "strohmandeln")`) wraps the first three exports.

| Export | Takes | Returns | Output |
|---|---|---|---|
| `strohmandeln_alloc(len)` | byte count | address of `len` writable bytes (valid until the next alloc) | none |
| `strohmandeln_out()` | | address of the last output | |
| `strohmandeln_out_len()` | | its length in bytes | |
| `strohmandeln_new(len)` | config JSON in the buffer | 1 made, 0 refused | opening reply, or an error |
| `strohmandeln_send(len)` | action JSON in the buffer | 1 accepted, 0 not | reply, with `refusal` when refused |
| `strohmandeln_step()` | | 1 the opponent moved, 0 not (not its turn) | reply with `stepped` |
| `strohmandeln_render()` | | | reply with the state and no events |
| `strohmandeln_legal()` | | | `{protocol, legal}` |
| `strohmandeln_export()` | | | the saved sitting |
| `strohmandeln_import(len)` | saved JSON in the buffer | 1 restored, 0 not (old sitting stays) | opening reply, or an error |
| `strohmandeln_hint()` | | | `{protocol, hint, reason, params, rule, odds}`: the tutor's move (one of `legal`, same shape) and why; on the opponent's turn or after the end `hint` is `null` and `reason` `"not-your-turn"`. Does not move the game. See below |
| `strohmandeln_review()` | | | `{protocol, review}`: the review of the last scored deal, or `review: null` before one is scored. Does not move the game. See "The review" |
| `strohmandeln_review_sitting()` | | | the same shape, over every deal scored so far |
| `strohmandeln_explain()` | | | `{protocol, explain}`: what the last trick showed, one bullet, or `explain: null` before a trick is won. See "The review" |
| `strohmandeln_tracker()` | | | `{protocol, tracker}`: where each of the 54 cards is from the human's view, and what is still out; `tracker: null` with no deal in progress. Does not move the game. See "The tracker" |
| `strohmandeln_audit()` | | | `{hidden: [codes]}`: test aid, see below |

To call: `at = alloc(n); copy the UTF-8 bytes to memory[at..at+n]; send(n)`,
then read `memory[out() .. out()+out_len()]` and parse it. Read
`memory.buffer` afresh after every call (it can grow). A panic in the module
traps; `guard` in `kit/js/engine.js` expects replies shaped `{ok, state}`,
which these are.

Seats are `0` and `1` throughout. Cards are codes (`docs/RULES.md` 1.2): `T1`
to `T21`, `SK` (the Sküs), `Kh Qh Ch Jh`, pip cards `1h`..`4h` (red suits) and
`7s`..`10s` (black); suits `h d c s`. Output is compact JSON with object keys in
alphabetical order.

## Creating a sitting

`strohmandeln_new`:

```json
{"seed": 7, "touren": 3, "human": 1, "level": 1,
 "settings": {"both_pass": "play", "values": "2/3/4", "ultimo": 1}}
```

| Field | Meaning |
|---|---|
| `seed` | integer 0 to 2^53 (a u64 in the module); the deals and the first dealer follow from it |
| `touren` | `3` or `6` (15.2) |
| `human` | the seat the human takes, `0` (default) or `1` |
| `level` | the opponent's level, 1 to 4 in half steps (default 1; outside the range is clamped, a fraction rounds to the nearest half). `docs/OPPONENT.md`; the module carries it through export and import |
| `settings` | optional; any of `both_pass` (`"play"`, `"throw_in"`), `values` (`"2/3/4"`, `"1/2/3"`, `"1/2/4"`), `ultimo` (`1`, `2`); missing keys take the defaults of RULES 0 |

Errors (return 0; output `{"ok":false,"error":"bad-config", ...}` with either
`"field"` (`config` unparsable, `human` not 0 or 1, `settings` bad) or
`"setup"`, the core's `SetupError` as a key with parameters, e.g.
`{"key":"bad-touren","touren":4}`; keys in `docs/MESSAGES.md`). No error
carries prose.

## Replies

Every call that changes the sitting answers

```json
{"protocol": 1, "ok": true,  "state": {...}, "events": [...]}
{"protocol": 1, "ok": false, "state": {...}, "events": [], "refusal": {"key": "not-your-turn", "speaker": 0}}
```

- `events`: the events since the last reply, in order, filtered for the human
  (the opponent's `dealt` is dropped). Each event is sent once. The opening
  reply carries the sitting's first events.
- A refused action changes nothing; `events` is empty.
- `step` adds `"stepped": true`. If it is not the opponent's turn it answers
  `ok: false, stepped: false` (and returns 0).

Errors that are not rules refusals have no `state`:
`{"protocol":1,"ok":false,"error":CODE, ...parameters}` with `CODE` one of
`bad-config`, `bad-action` (the action JSON does not parse or names an unknown
action or card), `bad-record` (import: `field` or `setup` as for
`bad-config`), `opponent-refused` (a bug in the opponent: it chose an action
the engine refused; carries its `refusal`), `no-action` (the opponent found
nothing to choose from; nothing moved), `no-sitting` (any call before
`new`).

## The state

`state` is the view of `strohmandeln-core::view` for the human, plus the
module's fields. Example (the opening of seed 7, human at seat 1; the deal's
parts are shown in the next section):

```json
{
  "protocol": 1, "human": 1, "seat": 1, "level": 1,
  "prompt": "you", "phase": "bid", "to_move": 1, "over": false,
  "contos": [0, 0], "balance": 0, "touren_total": 3, "deals_counted": 0,
  "pending_doubling": false,
  "legal": [{"action": "take"}, {"action": "pass"}],
  "deal": { ... },
  "events": [ {"type":"deal_start", ...}, {"type":"dealt", ...} ]
}
```

| Field | Meaning |
|---|---|
| `prompt` | what the host does next: `"you"` (the human is to act), `"opponent"` (call `step`), `"over"` (the sitting is settled) |
| `phase` | `"bid"`, `"play"`, `"extend"` (the offer of 15.6) or `"over"` |
| `human`, `seat` | the human's seat (the same number twice: `seat` is the view's) |
| `to_move` | seat to act, or `null` when over |
| `over` | the sitting is settled |
| `contos` | each seat's Conto (running points), by seat |
| `balance` | `contos[0] - contos[1]` (15.5), whoever the human is |
| `touren_total` | Touren in the sitting now (6 or 12 after an extension) |
| `deals_counted` | deals scored so far, void and thrown-in deals not counted |
| `pending_doubling` | the next deal will be doubled (14) |
| `legal` | the human's legal actions, empty unless `prompt` is `"you"` (below) |
| `deal` | the deal in progress, or `null` when there is none |
| `events` | the events of the deal in progress, filtered; lets a client rebuild after a reload |
| `level` | the opponent's level after clamping to half steps (a number, e.g. `2.5`) |

### `deal`

```json
{
  "dealer": 0, "forehand": 1, "multiplier": 1, "values": "2/3/4", "both_pass": "play",
  "declarer": 1, "playing": true,
  "trick_number": 3,
  "own_hand": [{"card": "T5", "public": false}, {"card": "1d", "public": true}],
  "opponent_hand_size": 15,
  "opponent_known": ["T18"],
  "covers": [[null, "Qs", "T7"], ["1d", null, "Jc"]],
  "hidden": [[3, 0, 2], [4, 4, 1]],
  "tricks": [{"leader": 0, "cards": ["Jd", "1d"], "sources": ["hand", "packet 2"], "winner": 0}],
  "lead": {"seat": 0, "card": "Jd", "source": "hand"},
  "take_thirds": [12, 9],
  "announcements": [{"seat": 1, "honour": "trull"}],
  "forfeited": []
}
```

| Field | Meaning |
|---|---|
| `dealer`, `forehand`, `multiplier`, `declarer` | as in RULES 2, 3, 14; `declarer` is `null` before a bid is taken or if nobody took the game |
| `values`, `both_pass` | the sitting's settings (RULES 9.1, 3.4): `"2/3/4"`, `"1/2/3"` or `"1/2/4"`; `"play"` or `"throw_in"`. Public; they price a bid |
| `playing` | the bidding is over |
| `trick_number` | the current trick, 1 to 27 |
| `own_hand` | the human's hand cards in dealt order, each with `public`: pulled packet cards and announced honour cards the opponent knows (13.4) |
| `opponent_hand_size` | number of cards in the opponent's hand (15 plus those pulled, minus those played) |
| `opponent_known` | the opponent's hand cards the human knows: pulled and still unplayed, and those of an announced honour (13.4, 13.5) |
| `covers` | the face-up card of each packet: `covers[owner seat][packet - 1]`, or `null` when none |
| `hidden` | face-down cards left in each packet: `hidden[owner seat][packet - 1]` |
| `tricks` | completed tricks of this deal: leader, the two cards in play order, where each came from, the winner |
| `lead` | the card led to the open trick, or `null` |
| `take_thirds` | the running count of each seat's won cards in thirds, by seat (the pack counts 70 points, 210 thirds; the threshold of the `count` event is 107, RULES 8) |
| `announcements`, `forfeited` | honours announced (both seats), and the human's own honours lost by playing a card of them unannounced (the opponent's are never shown) |

A packet card played from a packet leaves a `null` cover until the
replacement events refill it (6.1).

### `legal`

The human's legal actions, one object each, in the wire form of the actions
they are sent back as, with `source` added to plays:

```json
[{"action": "take"}, {"action": "pass"}]
[{"action": "announce", "honour": "trull"},
 {"action": "play", "card": "T5", "source": "hand"},
 {"action": "play", "card": "1d", "source": "packet 2"}]
[{"action": "extend", "accept": true}, {"action": "extend", "accept": false}]
```

`source` is `"hand"` or `"packet 1"` to `"packet 3"`: whether the card is a
hand card or the uncovered card of that packet (a packet's cover is playable;
5.2). `strohmandeln_legal()` returns the same list on its own.

## Actions

Sent with `strohmandeln_send`, always as the human. These are
`strohmandeln_core::types::Action`:

| Action | JSON | When |
|---|---|---|
| take the game | `{"action":"take"}` | bidding (3) |
| pass | `{"action":"pass"}` | bidding |
| announce an honour | `{"action":"announce","honour":"trull"}` (or `"kings"`) | before playing a card (11.3) |
| play a card | `{"action":"play","card":"T5"}` | play (5); the card names itself: the engine finds whether it is a hand or packet card |
| play and announce | `{"action":"play","card":"T5","announce":["trull"]}` | the announcements of 11.4 made just before the card |
| answer the extension | `{"action":"extend","accept":true}` | `phase: "extend"` (15.6); both players must accept |

Anything else is `{"error":"bad-action"}`.

## Refusals

`refusal` is `{"key": ..., <parameters>}` (absent parameters are omitted).
Keys and parameters are those of `docs/MESSAGES.md`; examples:

```json
{"key": "not-your-turn", "speaker": 0}
{"key": "not-playable", "card": "T5"}
{"key": "must-follow-suit", "card": "T5", "led": "h", "allowed": ["3h", "Kh"]}
{"key": "must-trump", "card": "3h", "allowed": ["T5", "T7"]}
{"key": "not-held", "honour": "trull", "missing": ["T21"]}
```

`led` is a suit letter or `"trump"`. The other keys: `wrong-phase`, `finished`,
`already-announced`, `honour-forfeited`, `duplicate-announcement`. A
client that offers only `legal` actions will not see rule refusals, only
`not-your-turn` if it sends out of turn.

## Events

Records `{"type": ..., ...}` with the fields of `docs/RULES.md` 16. Seats are
numbers, cards are codes, `source` is `"hand"` or `"packet n"`. Real
examples (field order is alphabetical on the wire):

```json
{"type":"sitting_start","first_dealer":0,"touren":3,"settings":{"both_pass":"play","ultimo":1,"values":"2/3/4"}}
{"type":"deal_start","deal_number":1,"tour":1,"dealer":0,"forehand":1,"multiplier":1}
{"type":"dealt","seat":1,"cards":["T5","Qd","Ch","3h","Cc","10c","T3","8s","T1","Js","T16","T20","T2","10s","T8"]}
{"type":"void_deal","seats":[0]}
{"type":"bid","seat":0}
{"type":"pass","seat":0}
{"type":"both_pass","mode":"play"}
{"type":"thrown_in"}
{"type":"reveal_start","seat":0}
{"type":"packet_turned","seat":1,"packet":2,"card":"T18"}
{"type":"card_pulled","seat":1,"packet":2,"card":"T18","last":false}
{"type":"cover_set","seat":1,"packet":3,"card":"Qs"}
{"type":"packet_empty","seat":1,"packet":2}
{"type":"announcement","seat":1,"honour":"trull","cards":["SK","T21","T1"]}
{"type":"honour_forfeited","seat":1,"honour":"kings"}
{"type":"card_played","seat":1,"card":"1d","source":"packet 2","trick":1}
{"type":"trick_won","winner":0,"cards":["1d","Jd"],"trick":1}
{"type":"pagat_ultimo","seat":0,"result":"won"}
{"type":"valat","seat":1}
{"type":"count","thirds":[66,144],"declarer":1,"threshold":107}
{"type":"game_scored","result":"declarer_win","credits":[{"seat":1,"cause":"game","points":3},{"seat":1,"cause":"trull","points":1}],"multiplier":1,"contos":[0,4]}
{"type":"doubling_set"}
{"type":"doubling_applied","multiplier":2}
{"type":"doubling_lapsed"}
{"type":"tour_end","tour":1,"contos":[1,8],"balance":-7}
{"type":"extension_offered"}
{"type":"extension_decided","accepted":false,"touren_added":0}
{"type":"sitting_end","contos":[9,15],"balance":-6,"payer":0,"kreuzer":12}
```

Notes:

- `dealt` appears only for the human's seat. The opponent's 15 cards are never
  sent. Likewise `honour_forfeited` appears only for the human's own forfeit:
  the opponent's is private (RULES 11.5, 13.2), and `deal.forfeited` lists only
  the human's. A card appears in the stream only when the human may know it.
- `credits[].cause` is one of `game trull kings ultimo_won ultimo_lost valat`;
  `result` one of `declarer_win declarer_loss simple_win draw`.
- `sitting_end` is the settlement: `payer` is the losing seat (`null` if
  square), `kreuzer` what they pay (`|balance|` times 2, 15.7).
- After `trick_won` come the replacement events for each cover used (6.1),
  leader first.
- A `deal_start` may be followed by `dealt`, `void_deal`, and a new
  `deal_start` (a hand without a tarock is dealt again, 2.6). The `void_deal`
  is in the `events` of the reply that carries it, where the table shows it
  (it names the seats whose hand had no tarock). It is not in `state.events`,
  which starts at the last `deal_start` and so never holds an annulled deal;
  a client that rebuilds from a saved state does not see it again.

## Saving and restoring

`strohmandeln_export()`:

```json
{"protocol": 1, "human": 1, "level": 1,
 "record": {"version": 1, "seed": 7, "settings": {"both_pass":"play","ultimo":1,"values":"2/3/4"}, "touren": 3,
            "moves": [{"seat": 1, "action": {"action":"take"}},
                      {"seat": 1, "action": {"action":"play","card":"1d","announce":[]}},
                      {"seat": 0, "action": {"action":"play","card":"Jd","announce":[]}}]}}
```

`record` is the core's `Record` (`strohmandeln_core::session::Record`): the
seed, settings, Touren and every move by either seat, in order. The sitting is
rebuilt move by move; it is exact for any sitting. `strohmandeln_import`
replays it and answers the opening shape (`ok`, `state`, and `events: []`:
the client rebuilds from `state`, whose `events` hold the deal in progress).
Errors: `bad-record` (unparsable, wrong `protocol`, record `version` other
than 1, or a move refused on replay). The opponent's own moves are recorded;
what it would do next is not (an imported sitting continues with a freshly
seeded opponent).

## The audit export

`strohmandeln_audit()` returns `{"hidden": [codes]}`: the cards the human must
not be able to learn in the current deal (the opponent's private hand cards
and every face-down packet card, both sides). `tests/smoke.mjs` and
`tests/golden.rs` check after every move that none appears in the state or in
the deal's events. It is a test aid; a page does not call it.

## Golden sittings

`crates/strohmandeln-wasm/tests/golden.txt` records, for fixed seeds, the
FNV-1a hash of the raw bytes of every reply (opening reply, then each send or
step) of a sitting played by a fixed policy: the human takes the first legal
action, declining any extension; the opponent is the default. A change to
anything on the wire, or to the engine's play, shows as a mismatch in
`cargo test -p strohmandeln-wasm --test golden` and in `smoke.mjs`; re-record
with `UPDATE_GOLDEN=1` once the change is intended.

## The hint

`strohmandeln_hint` answers for the human's seat at the top rung (level 4,
whole), whatever the opponent's level. It is a function of the position, so
asking twice gives the same answer, and asking changes nothing.

```json
{"protocol": 1, "hint": {"action": "play", "card": "T19", "source": "hand"},
 "reason": "win-cheapest",
 "params": {"card": "T19", "led": "T15", "alt": "T21", "trick_thirds": 2},
 "rule": null,
 "odds": {"alt": "T21", "better": 35, "worlds": 50, "gain_thirds": 4}}
```

`hint` is one of the entries of `state.legal` (send its card alone, as for any
play). `reason` is a message key from the hint table in `docs/MESSAGES.md` and
`params` its parameters, named there: a card is a code, a name ending
`_thirds` is thirds of a point, the rest are counts. `rule` is a key of the
rule table there, or `null`: the maxim the move follows. `odds` is how the
search ranks the card against the best other legal card over freshly sampled
deals, or `null` (not asked at a level below the top, a forced card, or the
sample does not agree with the choice). The opponent's `step` and the hint
share their rules: a hint is what the level-4 opponent would play; the
explanation never changes the move. Without a move to make, `hint`, `rule` and
`odds` are `null` and `reason` is `not-your-turn`.

## The review

`strohmandeln_review` and `strohmandeln_review_sitting` answer for the end-of-deal
dialog (`docs/TABLE3D.md` 6.4). The module reads the human's filtered events
(the public log of `docs/RULES.md` 13.6 and the human's own `dealt`), so a
review holds nothing the human's view does not; after the last trick every card,
the opponent's included, is in that log. Asking changes nothing.

```json
{"protocol": 1, "review": {"deal": 3, "bullets": [
  {"lead": "count", "key": "count", "params": {"mine": 120, "theirs": 90}},
  {"lead": "decided", "key": "trick-lost-rich", "params": {"mine": "Qs", "theirs": "Ks", "thirds": 23, "trick": 4}},
  {"lead": "try", "key": "try-announce", "params": {"honour": "trull"}}]}}
```

`review` is `null` when no deal has been scored (a void deal and a throw-in are
not scored). `deal` is the deal's number, `0` for the sitting. A bullet's `lead`
is `count`, `decided` or `try`; `key` and `params` are those of the "Review
keys" table of `docs/MESSAGES.md`, which gives each key's English and German
text and the rule it rests on. Thirds are integers (107 is 35⅔); cards are
codes. The sitting's review has three bullets: `sitting-tally`, the `decided`
bullet of the deal that went worst (else best), with its `deal`, and the most
frequent `try`. The review is a function of the events (`strohmandeln_ai::review`).

`strohmandeln_explain` answers the line under the prompt after a trick (the
setting "Explanations", `docs/TABLE3D.md` 6.3.3): one bullet
`{"lead": "explain", "key": "explain-trump-opp", "params": {"suit": "h"}}` from the
"Explanation keys" table of `docs/MESSAGES.md`. It deduces from the rule of
following only what the rule gives (a trump on a suit led: none of that suit in
hand or showing), from the human's filtered events.

## The tracker

`strohmandeln_tracker` answers for the table's card tracker (`docs/TABLE3D.md`
6.6). It is built from the human's view alone (`strohmandeln_ai::tracker`, over
`Knowledge::from_view`), so it holds nothing the view does not: an unseen card
is `unseen` and no more. Asking changes nothing.

```json
{"protocol": 1, "tracker": {
  "human": 0, "trick": 5,
  "cards": {"T21": {"place": "hand"}, "T1": {"place": "played", "by": 1, "trick": 3},
            "Kh": {"place": "unseen"}, "...": "all 54 codes"},
  "groups": [{"kind": "h", "mine": ["Qh"], "out": ["Kh", "Ch", "2h"], "unseen": 2,
              "void": false, "best": "Qh", "top_out": "Kh", "master": false},
             "... d, c, s, then trump"],
  "out_count": 31, "out_thirds": 110, "unseen_count": 27, "unseen_thirds": 96}}
```

`place` is one of `hand` and `cover` (mine), `opp_cover` (his face-up packet
card), `opp_hand` (in his hand and shown: pulled and not yet played, or part of
an announced honour), `played` (with `by`, a seat, and `trick`, 1 to 27, the
open trick included) and `unseen` (his private hand cards and every face-down
packet card, both players'). `groups` are the four suits `h d c s` and
`trump`, each with: `mine` (my hand and covers of the kind) and `out` (the
cards of the kind that are not mine and not played; `opp_cover`, `opp_hand` and
`unseen` together), both high to low; `unseen`, how many of `out` are strictly
unseen; `void`, the opponent failed to follow the kind and no card of it has
been shown in his hand or on his covers since (a packet can refill a hand);
`best`, my highest card of the kind; `top_out`, the highest card out; `master`,
I hold the kind and nothing out is higher. `out_count` and `out_thirds` cover
the cards out, `unseen_count` and `unseen_thirds` the unseen ones; thirds of a
point, so the whole pack is 210 (70 points, RULES 8.1). `trick` stops at 27 once
the last trick is won.
