//! The table as a protocol: JSON in, JSON out (`docs/PROTOCOL.md`).
//!
//! A client holds no game logic. It creates a sitting, reads the state, draws
//! it, and sends one action at a time; between its own moves it asks the
//! module to step the opponent. Every field comes from the human's view
//! (`strohmandeln_core::view`), so a client cannot show more than the person
//! at the table could know.
//!
//! Compiled to WebAssembly the crate exports a handful of functions (see
//! [`ffi`]), passing strings through linear memory with no `wasm-bindgen`
//! (the buffer pattern of Cassino's `cassino-wasm`).

use serde::Deserialize;
use serde_json::{json, Map, Value};
use strohmandeln_ai::cardset::Kind;
use strohmandeln_ai::explain::Param as HintParam;
use strohmandeln_ai::hint::hint as ai_hint;
use strohmandeln_ai::opponent::{Config as AiConfig, Level, Opponent};
use strohmandeln_ai::review::{explain_last, review_last, review_sitting, Bullet, Param, Review};
use strohmandeln_ai::tracker::{tracker as ai_tracker, Place};
use strohmandeln_core::agents::Player;
use strohmandeln_core::cards::{Card, Honour};
use strohmandeln_core::session::{Record, Sitting};
use strohmandeln_core::types::{Action, Event, Seat, Settings, SetupError, Source};
use strohmandeln_core::view::{filter_events, view, View};

/// Bumped whenever the wire format changes in a way a client would notice.
pub const PROTOCOL: u32 = 1;

/// Builds the opponent for a seat from the sitting's seed and the `level`
/// (1 to 4 in half steps, `docs/OPPONENT.md`) the host passed.
pub type OpponentFactory = fn(seed: u64, seat: Seat, level: f64) -> Box<dyn Player>;

/// The ladder's opponent (`strohmandeln-ai`) at the requested level. A level
/// outside 1 to 4 is clamped, one that is not a number is the top.
pub fn default_opponent(seed: u64, seat: Seat, level: f64) -> Box<dyn Player> {
    let seed = seed ^ (u64::from(seat.index() as u8) + 1).rotate_left(56);
    Box::new(Opponent::new(Level::from_f64(level), seed).with_config(browser_config()))
}

/// The tuning the browser plays with: the crate's defaults (the search budget
/// is capped there; `docs/OPPONENT.md`).
fn browser_config() -> AiConfig {
    AiConfig::default()
}

/// The level of the tutor's hint: always the top rung, whole.
const HINT_LEVEL: f64 = 4.0;

fn normal_level(x: f64) -> f64 {
    Level::from_f64(x).as_f64()
}

/// FNV-1a, 64 bits: the hash of the golden games (`tests/golden.txt`).
pub fn fnv(data: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in data {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// The request that creates a sitting.
#[derive(Deserialize)]
struct Config {
    seed: u64,
    touren: u32,
    /// Overrides of the defaults, any of `both_pass`, `values`, `ultimo`.
    #[serde(default)]
    settings: Map<String, Value>,
    /// The seat the human takes, 0 or 1.
    #[serde(default)]
    human: u8,
    /// The opponent's level, 1 to 4 in half steps (default 1).
    #[serde(default = "one")]
    level: f64,
}

fn one() -> f64 {
    1.0
}

/// What a saved sitting holds.
#[derive(Deserialize)]
struct Saved {
    protocol: u32,
    human: u8,
    #[serde(default = "one")]
    level: f64,
    record: Record,
}

/// Why a call that is not a rules refusal failed. On the wire it is a code
/// plus parameters, never prose (`docs/MESSAGES.md`).
#[derive(Debug)]
pub enum Fail {
    /// The input did not parse or does not fit: the field that was wrong.
    Field(&'static str),
    /// The core refused to begin or replay the sitting.
    Setup(SetupError),
}

impl From<SetupError> for Fail {
    fn from(e: SetupError) -> Fail {
        Fail::Setup(e)
    }
}

impl Fail {
    /// The error reply, under the given code (`bad-config`, `bad-record`).
    fn reply(&self, code: &str) -> Value {
        match self {
            Fail::Field(f) => error(code, json!({"field": f})),
            Fail::Setup(e) => error(code, json!({"setup": e})),
        }
    }
}

fn settings_of(overrides: Map<String, Value>) -> Result<Settings, Fail> {
    let mut base = match serde_json::to_value(Settings::default()) {
        Ok(Value::Object(m)) => m,
        _ => Map::new(),
    };
    for (k, v) in overrides {
        base.insert(k, v);
    }
    serde_json::from_value(Value::Object(base)).map_err(|_| Fail::Field("settings"))
}

/// One sitting against one opponent, from the human's chair.
pub struct Table {
    sitting: Sitting,
    human: Seat,
    level: f64,
    opponent: Box<dyn Player>,
    /// How many of the sitting's events the host has been sent.
    sent: usize,
}

/// A review or explanation bullet on the wire.
fn bullet_json(b: Bullet) -> Value {
    let params: Map<String, Value> = b
        .params
        .into_iter()
        .map(|(k, v)| {
            let v = match v {
                Param::Num(n) => json!(n),
                Param::Text(s) => json!(s),
            };
            (k.to_string(), v)
        })
        .collect();
    json!({"lead": b.lead, "key": b.key, "params": params})
}

/// An error reply: the code, and the parameters (an object) beside it.
fn error(code: &str, params: Value) -> Value {
    let mut r = json!({"protocol": PROTOCOL, "ok": false, "error": code});
    if let (Value::Object(m), Value::Object(p)) = (&mut r, params) {
        m.extend(p);
    }
    r
}

fn seat_of(n: u8, field: &'static str) -> Result<Seat, Fail> {
    Seat::try_from(n).map_err(|_| Fail::Field(field))
}

impl Table {
    fn build(
        sitting: Sitting,
        human: Seat,
        level: f64,
        factory: OpponentFactory,
        sent: usize,
    ) -> Table {
        // A restored sitting gets a different opponent stream than the first
        // run would have had; the record holds moves, not the opponent's mind.
        let seed = sitting.seed() ^ (sitting.record().moves.len() as u64).rotate_left(32);
        let level = normal_level(level);
        Table {
            opponent: factory(seed, human.other(), level),
            sitting,
            human,
            level,
            sent,
        }
    }

    /// Creates a sitting from the JSON of `strohmandeln_new`.
    pub fn create(config: &str, factory: OpponentFactory) -> Result<Table, Fail> {
        let c: Config = serde_json::from_str(config).map_err(|_| Fail::Field("config"))?;
        let human = seat_of(c.human, "human")?;
        let sitting = Sitting::new(c.seed, settings_of(c.settings)?, c.touren)?;
        Ok(Table::build(sitting, human, c.level, factory, 0))
    }

    /// Restores a sitting from the JSON of `export`.
    pub fn restore(saved: &str, factory: OpponentFactory) -> Result<Table, Fail> {
        let s: Saved = serde_json::from_str(saved).map_err(|_| Fail::Field("saved"))?;
        if s.protocol != PROTOCOL {
            return Err(Fail::Field("protocol"));
        }
        let human = seat_of(s.human, "human")?;
        let sitting = Sitting::replay(&s.record)?;
        let sent = sitting.events().len();
        Ok(Table::build(sitting, human, s.level, factory, sent))
    }

    pub fn human(&self) -> Seat {
        self.human
    }

    pub fn sitting(&self) -> &Sitting {
        &self.sitting
    }

    fn my_view(&self) -> View {
        view(&self.sitting, self.human)
    }

    /// Where a card the human may play comes from.
    fn source_of(&self, card: Card) -> Option<Source> {
        let deal = self.sitting.deal()?;
        if deal.hand_cards(self.human).contains(&card) {
            return Some(Source::Hand);
        }
        (1..=3u8)
            .find(|&p| deal.cover(self.human, p) == Some(card))
            .map(Source::Packet)
    }

    /// The human's legal actions, each play with where its card comes from.
    pub fn legal(&self) -> Vec<Value> {
        if self.sitting.to_move() != Some(self.human) {
            return Vec::new();
        }
        self.sitting
            .legal_actions()
            .into_iter()
            .map(|a| match a {
                Action::Play { card, .. } => {
                    let source = String::from(self.source_of(card).unwrap_or(Source::Hand));
                    json!({"action": "play", "card": card, "source": source})
                }
                other => serde_json::to_value(other).unwrap_or(Value::Null),
            })
            .collect()
    }

    /// What the host should do next: `you`, `opponent` or `over`.
    pub fn prompt(&self) -> &'static str {
        match self.sitting.to_move() {
            None => "over",
            Some(s) if s == self.human => "you",
            Some(_) => "opponent",
        }
    }

    fn phase(&self) -> &'static str {
        if self.sitting.is_over() {
            return "over";
        }
        let actions = self.sitting.legal_actions();
        match actions.first() {
            Some(Action::Take | Action::Pass) => "bid",
            Some(Action::Extend { .. }) => "extend",
            _ => "play",
        }
    }

    /// The current view of the human, with the module's own additions.
    pub fn state(&self) -> Value {
        let mut v = serde_json::to_value(self.my_view()).unwrap_or(Value::Null);
        if let Value::Object(m) = &mut v {
            m.insert("protocol".into(), json!(PROTOCOL));
            m.insert("prompt".into(), json!(self.prompt()));
            m.insert("phase".into(), json!(self.phase()));
            m.insert("human".into(), json!(self.human));
            m.insert("level".into(), json!(self.level));
            // The actions made in this deal, both seats': the number the next
            // action will have in the deal's record (`docs/TUTOR.md` 3).
            m.insert(
                "deal_actions".into(),
                json!(self.sitting.deal_action_count()),
            );
            // `legal` replaces the view's own list: the same actions, with the
            // source of each playable card.
            m.remove("legal_actions");
            m.insert("legal".into(), Value::Array(self.legal()));
        }
        v
    }

    /// The events not yet sent, as the human may see them.
    fn take_events(&mut self) -> Vec<Event> {
        let all = self.sitting.events();
        let out = filter_events(&all[self.sent.min(all.len())..], self.human);
        self.sent = all.len();
        out
    }

    fn reply(&mut self, ok: bool, refusal: Option<Value>) -> Value {
        let events = self.take_events();
        let mut r = json!({
            "protocol": PROTOCOL,
            "ok": ok,
            "state": self.state(),
            "events": events,
        });
        if let (Some(refusal), Value::Object(m)) = (refusal, &mut r) {
            m.insert("refusal".into(), refusal);
        }
        r
    }

    /// The reply that opens a sitting: its first events and state.
    pub fn open(&mut self) -> Value {
        self.reply(true, None)
    }

    /// The human's action, as JSON.
    pub fn send(&mut self, action: &str) -> Value {
        let action: Action = match serde_json::from_str(action) {
            Ok(a) => a,
            Err(_) => return error("bad-action", json!({})),
        };
        match self.sitting.apply(self.human, action) {
            Ok(_) => self.reply(true, None),
            Err(refusal) => {
                let refusal = serde_json::to_value(refusal).unwrap_or(Value::Null);
                self.reply(false, Some(refusal))
            }
        }
    }

    /// One move of the opponent, if it is the opponent's turn.
    pub fn step(&mut self) -> Result<Value, Value> {
        let opp = self.human.other();
        if self.sitting.to_move() != Some(opp) {
            let mut r = self.reply(false, None);
            if let Value::Object(m) = &mut r {
                m.insert("stepped".into(), json!(false));
            }
            return Err(r);
        }
        let v = view(&self.sitting, opp);
        let Some(action) = self.opponent.choose(&v) else {
            return Err(error("no-action", json!({})));
        };
        if let Err(refusal) = self.sitting.apply(opp, action.clone()) {
            return Err(error("opponent-refused", json!({"refusal": refusal})));
        }
        let mut r = self.reply(true, None);
        if let Value::Object(m) = &mut r {
            m.insert("stepped".into(), json!(true));
        }
        Ok(r)
    }

    /// The sitting as the host should save it.
    pub fn export(&self) -> Value {
        json!({
            "protocol": PROTOCOL,
            "human": self.human,
            "level": self.level,
            "record": self.sitting.record(),
        })
    }

    /// The tutor's hint for the human's move: the move the top rung would
    /// make, with a reason as a message key (`docs/MESSAGES.md`). Without a
    /// move to make (not the human's turn, or the sitting is over) the hint is
    /// `null` and the reason `not-your-turn`.
    pub fn hint(&self) -> Value {
        let Some(legal) = self.my_hint_legal() else {
            return json!({
                "protocol": PROTOCOL,
                "action": null,
                "hint": null,
                "reason": "not-your-turn",
                "params": {},
                "rule": null,
                "odds": null,
            });
        };
        let v = self.my_view();
        let seed = self.sitting.seed() ^ 0x7417_0000_0000_0001;
        let h = ai_hint(&v, seed, Level::from_f64(HINT_LEVEL), AiConfig::default());
        let action = match &h.action {
            Action::Play { card, .. } => legal
                .iter()
                .find(|a| a["action"] == "play" && a["card"] == json!(card))
                .cloned()
                .unwrap_or(Value::Null),
            other => serde_json::to_value(other).unwrap_or(Value::Null),
        };
        let params: serde_json::Map<String, Value> = h
            .params
            .iter()
            .map(|(name, p)| {
                let v = match p {
                    HintParam::Card(c) => json!(c),
                    HintParam::Int(n) => json!(n),
                };
                ((*name).to_string(), v)
            })
            .collect();
        let odds = h.odds.as_ref().map(|o| {
            json!({"alt": o.alt, "better": o.better, "worlds": o.worlds, "gain_thirds": o.gain_thirds})
        });
        json!({
            "protocol": PROTOCOL,
            "action": self.sitting.deal_action_count(),
            "hint": action,
            "reason": h.key,
            "params": params,
            "rule": h.rule,
            "odds": odds,
        })
    }

    /// The review of the last scored deal, or of the whole sitting so far,
    /// from the human's filtered events (so it holds nothing the human's view
    /// does not): `{"review": {"deal", "bullets": [{"lead", "key",
    /// "params"}]}}`, or `"review": null` when no deal was scored yet.
    pub fn review(&self, whole_sitting: bool) -> Value {
        let events = filter_events(self.sitting.events(), self.human);
        let r = if whole_sitting {
            review_sitting(&events, self.human)
        } else {
            review_last(&events, self.human)
        };
        let review = r.map(|Review { deal, bullets }| {
            let bullets: Vec<Value> = bullets.into_iter().map(bullet_json).collect();
            json!({"deal": deal, "bullets": bullets})
        });
        json!({"protocol": PROTOCOL, "review": review})
    }

    /// What the last trick showed, for the line under the prompt
    /// (`{"explain": {"lead", "key", "params"}}`, or `null` before a trick is
    /// won). The trick is the last of the human's filtered events, so a trick
    /// of an earlier deal is explained until the next is won.
    pub fn explain(&self) -> Value {
        let events = filter_events(self.sitting.events(), self.human);
        let bullet = explain_last(&events, self.human).map(bullet_json);
        json!({"protocol": PROTOCOL, "explain": bullet})
    }

    /// The card tracker for the human's seat (`docs/PROTOCOL.md`): where each
    /// of the 54 cards is from the human's view, and the counts of what is
    /// still out, as data and keys. `null` when no deal is in progress.
    pub fn tracker(&self) -> Value {
        let t = ai_tracker(&self.my_view()).map(|t| {
            let cards: Map<String, Value> = t
                .cards
                .iter()
                .map(|(c, place)| {
                    let v = match place {
                        Place::Hand => json!({"place": "hand"}),
                        Place::Cover => json!({"place": "cover"}),
                        Place::OppCover => json!({"place": "opp_cover"}),
                        Place::OppHand => json!({"place": "opp_hand"}),
                        Place::Unseen => json!({"place": "unseen"}),
                        Place::Played { by, trick } => {
                            json!({"place": "played", "by": by, "trick": trick})
                        }
                    };
                    (c.code(), v)
                })
                .collect();
            let groups: Vec<Value> = t
                .groups
                .iter()
                .map(|g| {
                    let kind = match g.kind {
                        Kind::Suit(s) => s.letter().to_string(),
                        Kind::Trump => "trump".to_string(),
                    };
                    json!({
                        "kind": kind,
                        "mine": g.mine,
                        "out": g.out,
                        "unseen": g.unseen,
                        "void": g.void,
                        "best": g.best,
                        "top_out": g.top_out,
                        "master": g.master,
                    })
                })
                .collect();
            json!({
                "human": t.me,
                "trick": t.trick,
                "cards": cards,
                "groups": groups,
                "out_count": t.out_count,
                "out_thirds": t.out_thirds,
                "unseen_count": t.unseen_count,
                "unseen_thirds": t.unseen_thirds,
            })
        });
        json!({"protocol": PROTOCOL, "tracker": t})
    }

    fn my_hint_legal(&self) -> Option<Vec<Value>> {
        let legal = self.legal();
        (!legal.is_empty()).then_some(legal)
    }

    /// For tests and the smoke test only: every card the human must not be
    /// able to learn in this deal (the opponent's private hand cards and every
    /// face-down packet card), as codes.
    pub fn audit(&self) -> Value {
        let mut hidden: Vec<String> = Vec::new();
        if let Some(deal) = self.sitting.deal() {
            let opp = self.human.other();
            let announced: Vec<Card> = [Honour::Trull, Honour::Kings]
                .into_iter()
                .filter(|&h| {
                    deal.honour_state(opp, h) == strohmandeln_core::deal::HonourState::Announced
                })
                .flat_map(Honour::cards)
                .collect();
            for h in deal.hand(opp) {
                if !h.public && !announced.contains(&h.card) {
                    hidden.push(h.card.code());
                }
            }
            for seat in Seat::BOTH {
                for p in 1..=3u8 {
                    let all = deal.packet_cards(seat, p);
                    let skip = usize::from(deal.cover(seat, p).is_some());
                    hidden.extend(all.iter().skip(skip).map(|c| c.code()));
                }
            }
        }
        json!({"hidden": hidden})
    }

    /// Replaces the opponent (the ladder, or a test's).
    pub fn set_opponent(&mut self, opponent: Box<dyn Player>) {
        self.opponent = opponent;
    }
}

/// The module's state between calls: the table and the last output. Native
/// tests use it as the host would, through the same strings.
#[derive(Default)]
pub struct Host {
    table: Option<Table>,
    out: Vec<u8>,
}

impl Host {
    pub fn out(&self) -> &[u8] {
        &self.out
    }

    fn give(&mut self, v: &Value) {
        self.out = v.to_string().into_bytes();
    }

    fn missing(&mut self) {
        self.give(&error("no-sitting", json!({})));
    }

    /// `strohmandeln_new`: 1 if a sitting was created (the output is its
    /// opening reply), else 0 and an error.
    pub fn new_sitting(&mut self, config: &str, factory: OpponentFactory) -> u32 {
        match Table::create(config, factory) {
            Ok(mut t) => {
                let r = t.open();
                self.table = Some(t);
                self.give(&r);
                1
            }
            Err(why) => {
                self.give(&why.reply("bad-config"));
                0
            }
        }
    }

    /// `strohmandeln_send`: 1 if the action was accepted.
    pub fn send(&mut self, action: &str) -> u32 {
        let Some(t) = self.table.as_mut() else {
            self.missing();
            return 0;
        };
        let r = t.send(action);
        let ok = r["ok"] == Value::Bool(true);
        self.give(&r);
        u32::from(ok)
    }

    /// `strohmandeln_step`: 1 if the opponent moved.
    pub fn step(&mut self) -> u32 {
        let Some(t) = self.table.as_mut() else {
            self.missing();
            return 0;
        };
        match t.step() {
            Ok(r) => {
                self.give(&r);
                1
            }
            Err(r) => {
                self.give(&r);
                0
            }
        }
    }

    /// Writes `f` of the table, or the no-sitting error.
    pub fn show(&mut self, f: impl FnOnce(&mut Table) -> Value) {
        match self.table.as_mut() {
            Some(t) => {
                let v = f(t);
                self.give(&v);
            }
            None => self.missing(),
        }
    }

    /// `strohmandeln_render`: the state, with no events.
    pub fn render(&mut self) {
        self.show(|t| t.reply_state());
    }

    /// `strohmandeln_import`: 1 if the saved sitting fitted; otherwise the old
    /// sitting stays.
    pub fn import(&mut self, saved: &str, factory: OpponentFactory) -> u32 {
        match Table::restore(saved, factory) {
            Ok(mut t) => {
                let r = t.open();
                self.table = Some(t);
                self.give(&r);
                1
            }
            Err(why) => {
                self.give(&why.reply("bad-record"));
                0
            }
        }
    }
}

impl Table {
    /// The state alone, in the shape of a reply with no events.
    pub fn reply_state(&self) -> Value {
        json!({"protocol": PROTOCOL, "ok": true, "state": self.state(), "events": []})
    }
}

/// The WebAssembly exports. Every call that has something to say leaves JSON
/// for the host to read at `strohmandeln_out()`, `strohmandeln_out_len()`.
pub mod ffi {
    use super::{default_opponent, Host};
    use std::cell::RefCell;

    thread_local! {
        static HOST: RefCell<Host> = RefCell::new(Host::default());
        static IN: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    }

    fn input(len: usize) -> String {
        IN.with(|b| {
            let b = b.borrow();
            String::from_utf8_lossy(&b[..len.min(b.len())]).into_owned()
        })
    }

    /// A buffer of `len` bytes for the host to write a request into. Valid
    /// until the next call to this function.
    #[no_mangle]
    pub extern "C" fn strohmandeln_alloc(len: usize) -> *mut u8 {
        IN.with(|b| {
            let mut b = b.borrow_mut();
            b.clear();
            b.resize(len, 0);
            b.as_mut_ptr()
        })
    }

    /// The address of the last output.
    #[no_mangle]
    pub extern "C" fn strohmandeln_out() -> *const u8 {
        HOST.with(|h| h.borrow().out().as_ptr())
    }

    /// The length in bytes of the last output.
    #[no_mangle]
    pub extern "C" fn strohmandeln_out_len() -> usize {
        HOST.with(|h| h.borrow().out().len())
    }

    /// Starts a sitting from the config JSON in the `len` bytes just written
    /// (`{"seed", "touren", "settings"?, "human"?, "level"?}`), discarding any
    /// old one; 1 if it was made. The output is the opening reply.
    #[no_mangle]
    pub extern "C" fn strohmandeln_new(len: usize) -> u32 {
        let config = input(len);
        HOST.with(|h| h.borrow_mut().new_sitting(&config, default_opponent))
    }

    /// Sends the human's action JSON in the `len` bytes just written; 1 if
    /// accepted. The output is the reply (a refusal when it was not).
    #[no_mangle]
    pub extern "C" fn strohmandeln_send(len: usize) -> u32 {
        let action = input(len);
        HOST.with(|h| h.borrow_mut().send(&action))
    }

    /// One move of the opponent; 1 if it moved (and the output is the reply).
    #[no_mangle]
    pub extern "C" fn strohmandeln_step() -> u32 {
        HOST.with(|h| h.borrow_mut().step())
    }

    /// Renders the current state (no events).
    #[no_mangle]
    pub extern "C" fn strohmandeln_render() {
        HOST.with(|h| h.borrow_mut().render())
    }

    /// Renders the human's legal actions: `{"legal": [...]}`.
    #[no_mangle]
    pub extern "C" fn strohmandeln_legal() {
        HOST.with(|h| {
            h.borrow_mut()
                .show(|t| serde_json::json!({"protocol": super::PROTOCOL, "legal": t.legal()}))
        })
    }

    /// Renders the saved form of the sitting.
    #[no_mangle]
    pub extern "C" fn strohmandeln_export() {
        HOST.with(|h| h.borrow_mut().show(|t| t.export()))
    }

    /// Restores a sitting from the export JSON in the `len` bytes just
    /// written; 1 if it fitted, otherwise the old sitting stays.
    #[no_mangle]
    pub extern "C" fn strohmandeln_import(len: usize) -> u32 {
        let saved = input(len);
        HOST.with(|h| h.borrow_mut().import(&saved, default_opponent))
    }

    /// Renders the tutor's hint for the human's move
    /// (`{"hint": <a legal action>, "reason": <key>, "params": {..}, "rule":
    /// <key or null>, "odds": <object or null>}`).
    #[no_mangle]
    pub extern "C" fn strohmandeln_hint() {
        HOST.with(|h| h.borrow_mut().show(|t| t.hint()))
    }

    /// Renders the review of the last scored deal
    /// (`{"review": {"deal", "bullets"}}` or `"review": null`).
    #[no_mangle]
    pub extern "C" fn strohmandeln_review() {
        HOST.with(|h| h.borrow_mut().show(|t| t.review(false)))
    }

    /// Renders the review of the sitting so far (the same shape).
    #[no_mangle]
    pub extern "C" fn strohmandeln_review_sitting() {
        HOST.with(|h| h.borrow_mut().show(|t| t.review(true)))
    }

    /// Renders the explanation of the last trick
    /// (`{"explain": {"lead", "key", "params"}}` or `"explain": null`).
    #[no_mangle]
    pub extern "C" fn strohmandeln_explain() {
        HOST.with(|h| h.borrow_mut().show(|t| t.explain()))
    }

    /// Renders the card tracker (`{"tracker": {...}}` or `"tracker": null`).
    #[no_mangle]
    pub extern "C" fn strohmandeln_tracker() {
        HOST.with(|h| h.borrow_mut().show(|t| t.tracker()))
    }

    /// A test aid: the codes of the cards the human must not see.
    #[no_mangle]
    pub extern "C" fn strohmandeln_audit() {
        HOST.with(|h| h.borrow_mut().show(|t| t.audit()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONFIG: &str = r#"{"seed":7,"touren":3,"human":0}"#;

    fn host(config: &str) -> Host {
        let mut h = Host::default();
        assert_eq!(h.new_sitting(config, default_opponent), 1);
        h
    }

    fn out(h: &Host) -> Value {
        serde_json::from_slice(h.out()).unwrap()
    }

    #[test]
    fn a_new_sitting_opens_with_its_events_and_state() {
        let h = host(CONFIG);
        let r = out(&h);
        assert_eq!(r["ok"], true);
        assert_eq!(r["events"][0]["type"], "sitting_start");
        assert_eq!(r["state"]["human"], 0);
        assert!(r["state"]["deal"]["own_hand"].as_array().unwrap().len() == 15);
    }

    #[test]
    fn the_state_and_the_hint_carry_the_number_of_the_next_action_of_the_deal() {
        // `deal_actions` counts both seats' actions of the deal in progress;
        // a hint says which action it is for (docs/TUTOR.md 3).
        let mut h = host(CONFIG);
        assert_eq!(out(&h)["state"]["deal_actions"], 0);
        let mut seen = 0;
        for _ in 0..40 {
            h.render();
            let s = out(&h)["state"].clone();
            if s["prompt"] == "you" {
                h.show(|t| t.hint());
                let r = out(&h);
                assert_eq!(r["action"], s["deal_actions"], "{r}");
                let a = r["hint"].to_string();
                assert_eq!(h.send(&a), 1);
                assert_eq!(out(&h)["state"]["deal_actions"], seen + 1);
            } else if s["prompt"] == "opponent" {
                assert_eq!(h.step(), 1);
                assert_eq!(out(&h)["state"]["deal_actions"], seen + 1);
            } else {
                break;
            }
            seen += 1;
        }
        assert!(seen >= 20);
        // not the human's turn: the hint has no action number
        let mut h = host(r#"{"seed":7,"touren":3,"human":1}"#);
        h.show(|t| t.hint());
        let r = out(&h);
        if r["reason"] == "not-your-turn" {
            assert_eq!(r["action"], Value::Null);
        }
    }

    #[test]
    fn bad_configs_are_errors_not_panics() {
        for bad in [
            "nonsense",
            r#"{"seed":1,"touren":4}"#,
            r#"{"seed":1,"touren":3,"human":2}"#,
            r#"{"seed":1,"touren":3,"settings":{"ultimo":"x"}}"#,
        ] {
            let mut h = Host::default();
            assert_eq!(h.new_sitting(bad, default_opponent), 0, "{bad}");
            assert_eq!(out(&h)["error"], "bad-config");
        }
    }

    #[test]
    fn setup_errors_reach_the_wire_as_a_key_with_parameters() {
        let mut h = Host::default();
        assert_eq!(
            h.new_sitting(r#"{"seed":1,"touren":4}"#, default_opponent),
            0
        );
        let r = out(&h);
        assert_eq!(r["error"], "bad-config");
        assert_eq!(r["setup"], json!({"key": "bad-touren", "touren": 4}));
        assert!(r.get("detail").is_none(), "no prose on the wire");

        let mut h = host(CONFIG);
        h.show(|t| t.export());
        let mut saved = out(&h);
        saved["record"]["version"] = json!(9);
        assert_eq!(h.import(&saved.to_string(), default_opponent), 0);
        let r = out(&h);
        assert_eq!(r["error"], "bad-record");
        assert_eq!(
            r["setup"],
            json!({"key": "record-version", "found": 9, "expected": 1})
        );
    }

    #[test]
    fn settings_overrides_are_merged_over_the_defaults() {
        let h = host(r#"{"seed":1,"touren":3,"settings":{"values":"1/2/3"}}"#);
        let e = &out(&h)["events"][0];
        assert_eq!(e["settings"]["values"], "1/2/3");
        assert_eq!(e["settings"]["ultimo"], 1);
    }

    #[test]
    fn calls_before_a_sitting_say_so() {
        let mut h = Host::default();
        assert_eq!(h.send("{}"), 0);
        assert_eq!(out(&h)["error"], "no-sitting");
        assert_eq!(h.step(), 0);
        h.render();
        assert_eq!(out(&h)["error"], "no-sitting");
    }

    #[test]
    fn an_action_out_of_turn_is_a_refusal_with_a_key() {
        // The human sits where it is not their turn to bid: sit at the seat
        // that is not first.
        let mut h = host(CONFIG);
        if out(&h)["state"]["prompt"] == "you" {
            h = host(r#"{"seed":7,"touren":3,"human":1}"#);
        }
        assert_eq!(out(&h)["state"]["prompt"], "opponent");
        assert_eq!(h.send(r#"{"action":"take"}"#), 0);
        let r = out(&h);
        assert_eq!(r["ok"], false);
        assert_eq!(r["refusal"]["key"], "not-your-turn");
        assert!(r["refusal"]["speaker"].is_number());
        assert_eq!(r["events"], json!([]));
    }

    #[test]
    fn malformed_actions_are_errors() {
        let mut h = host(CONFIG);
        assert_eq!(h.send("{\"action\":\"fold\"}"), 0);
        assert_eq!(out(&h)["error"], "bad-action");
    }

    /// Every `dealt` event of a reply (its events and the state's), as the
    /// seats they name.
    fn dealt_seats(r: &Value) -> Vec<u64> {
        r["events"]
            .as_array()
            .into_iter()
            .chain(r["state"]["events"].as_array())
            .flatten()
            .filter(|e| e["type"] == "dealt")
            .map(|e| e["seat"].as_u64().unwrap())
            .collect()
    }

    #[test]
    fn the_opponents_dealt_never_reaches_the_human() {
        for human in [0u64, 1] {
            let mut h = host(&format!(r#"{{"seed":9,"touren":3,"human":{human}}}"#));
            let mut told = dealt_seats(&out(&h));
            let mut moves = 0;
            loop {
                let s = out(&h)["state"].clone();
                match s["prompt"].as_str().unwrap() {
                    "over" => break,
                    "opponent" => assert_eq!(h.step(), 1),
                    _ => {
                        let a = s["legal"][0].clone();
                        let a = if a["action"] == "extend" {
                            json!({"action":"extend","accept":false})
                        } else {
                            a
                        };
                        assert_eq!(h.send(&a.to_string()), 1);
                    }
                }
                told.extend(dealt_seats(&out(&h)));
                moves += 1;
            }
            assert!(moves > 100 && told.len() > 3, "several deals were told");
            assert!(told.iter().all(|&s| s == human), "human {human}: {told:?}");
            // ...although the sitting itself dealt to both seats.
            let table = h.table.as_ref().unwrap();
            let both = table
                .sitting()
                .events()
                .iter()
                .filter(|e| matches!(e, Event::Dealt { seat, .. } if seat.index() as u64 != human))
                .count();
            assert!(both > 0, "the unfiltered stream holds the opponent's hands");
        }
    }

    #[test]
    fn a_void_deal_is_surfaced_to_the_table_in_the_reply_events() {
        // Seed 520 deals a hand without a tarock first (found by search): the
        // core's view starts at the last deal_start, so the void_deal is in
        // the reply's events but not in the state's list of the deal's.
        let h = host(r#"{"seed":520,"touren":3,"human":0}"#);
        let r = out(&h);
        let types: Vec<&str> = r["events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| e["type"].as_str().unwrap())
            .collect();
        let void = types.iter().position(|&t| t == "void_deal").expect("told");
        assert_eq!(types.iter().filter(|&&t| t == "deal_start").count(), 2);
        assert!(
            types[..void].contains(&"deal_start"),
            "after the deal it voids"
        );
        assert!(types[void..].contains(&"deal_start"), "and then a new deal");
        let seats = r["events"][void]["seats"].as_array().unwrap();
        assert!(!seats.is_empty());
        let in_state = r["state"]["events"].as_array().unwrap();
        assert!(!in_state.iter().any(|e| e["type"] == "void_deal"));
    }

    #[test]
    fn an_opponent_with_no_action_is_an_error_not_a_panic() {
        struct Stuck;
        impl Player for Stuck {
            fn choose(&mut self, _: &View) -> Option<Action> {
                None
            }
        }
        let mut h = host(CONFIG);
        while out(&h)["state"]["prompt"] != "opponent" {
            let a = out(&h)["state"]["legal"][0].clone();
            assert_eq!(h.send(&a.to_string()), 1);
        }
        h.table.as_mut().unwrap().set_opponent(Box::new(Stuck));
        assert_eq!(h.step(), 0);
        assert_eq!(out(&h)["error"], "no-action");
        h.render();
        assert_eq!(out(&h)["state"]["prompt"], "opponent", "nothing moved");
    }

    #[test]
    fn stepping_out_of_turn_moves_nobody() {
        let mut h = host(CONFIG);
        while out(&h)["state"]["prompt"] != "you" {
            assert_eq!(h.step(), 1);
        }
        assert_eq!(h.step(), 0);
        assert_eq!(out(&h)["stepped"], false);
    }

    #[test]
    fn legal_plays_name_their_source() {
        let mut h = host(CONFIG);
        let mut saw_packet = false;
        let mut n = 0;
        loop {
            let s = out(&h)["state"].clone();
            match s["prompt"].as_str().unwrap() {
                "over" => break,
                "opponent" => {
                    h.step();
                }
                _ => {
                    for a in s["legal"].as_array().unwrap() {
                        if a["action"] == "play" {
                            let src = a["source"].as_str().unwrap();
                            assert!(src == "hand" || src.starts_with("packet "), "{src}");
                            saw_packet |= src.starts_with("packet");
                        }
                    }
                    let a = s["legal"][n % s["legal"].as_array().unwrap().len()].clone();
                    n += 1;
                    let a = if a["action"] == "extend" {
                        json!({"action":"extend","accept":false})
                    } else {
                        a
                    };
                    assert_eq!(h.send(&a.to_string()), 1, "{a}");
                }
            }
        }
        assert!(saw_packet, "a packet card became playable");
    }

    #[test]
    fn export_then_import_restores_the_same_state() {
        let mut h = host(CONFIG);
        for _ in 0..30 {
            let s = out(&h)["state"].clone();
            match s["prompt"].as_str().unwrap() {
                "opponent" => {
                    h.step();
                }
                "you" => {
                    let a = s["legal"][0].clone();
                    h.send(&a.to_string());
                }
                _ => break,
            }
        }
        h.show(|t| t.export());
        let saved = out(&h);
        assert_eq!(saved["protocol"], 1);
        h.render();
        let before = out(&h)["state"].clone();
        let mut g = Host::default();
        assert_eq!(g.import(&saved.to_string(), default_opponent), 1);
        let r = out(&g);
        assert_eq!(r["state"], before);
        assert_eq!(r["events"], json!([]));
    }

    #[test]
    fn a_bad_import_leaves_the_old_sitting() {
        let mut h = host(CONFIG);
        assert_eq!(h.import("{}", default_opponent), 0);
        assert_eq!(out(&h)["error"], "bad-record");
        h.render();
        assert_eq!(out(&h)["ok"], true);
    }

    #[test]
    fn the_hint_is_a_legal_action_with_a_known_reason_key() {
        for level in [1, 2] {
            let mut h = host(&format!(r#"{{"seed":7,"touren":3,"level":{level}}}"#));
            let mut n = 0;
            loop {
                h.render();
                if out(&h)["state"]["prompt"] == "over" || n >= 40 {
                    break;
                }
                n += 1;
                let s = out(&h)["state"].clone();
                if s["prompt"] == "opponent" {
                    h.show(|t| t.hint());
                    assert_eq!(out(&h)["hint"], Value::Null, "not the human's turn");
                    assert_eq!(out(&h)["reason"], "not-your-turn");
                    h.step();
                    continue;
                }
                h.show(|t| t.hint());
                let r = out(&h);
                assert!(s["legal"].as_array().unwrap().contains(&r["hint"]), "{r}");
                let key = r["reason"].as_str().unwrap();
                assert!(strohmandeln_ai::hint::KEYS.contains(&key), "{key}");
                assert!(r["params"].is_object());
                assert!(r["rule"].is_null() || r["rule"].is_string());
                if r["hint"]["action"] == "play" {
                    assert_eq!(r["params"]["card"], r["hint"]["card"], "{r}");
                }
                let a = if r["hint"]["action"] == "play" {
                    json!({"action":"play","card":r["hint"]["card"]})
                } else {
                    r["hint"].clone()
                };
                assert_eq!(h.send(&a.to_string()), 1, "the hint is playable: {a}");
            }
        }
    }

    /// The hint never falls outside `state.legal` (a card the search chose
    /// that `legal` does not offer would come back `null`): over seeded
    /// positions at every opponent level, the hint is always one of `legal`.
    #[test]
    fn the_hint_is_never_outside_the_legal_actions() {
        let mut hinted = 0;
        for level in ["1", "1.5", "2", "2.5", "3", "3.5", "4"] {
            for seed in [3, 11] {
                let mut h = host(&format!(r#"{{"seed":{seed},"touren":3,"level":{level}}}"#));
                for _ in 0..30 {
                    h.render();
                    let s = out(&h)["state"].clone();
                    if s["prompt"] == "over" {
                        break;
                    }
                    if s["prompt"] == "opponent" {
                        h.step();
                        continue;
                    }
                    h.show(|t| t.hint());
                    let r = out(&h);
                    assert!(
                        s["legal"].as_array().unwrap().contains(&r["hint"]),
                        "level {level} seed {seed}: {r}"
                    );
                    hinted += 1;
                    let a = if r["hint"]["action"] == "play" {
                        json!({"action":"play","card":r["hint"]["card"]})
                    } else {
                        r["hint"].clone()
                    };
                    assert_eq!(h.send(&a.to_string()), 1);
                }
            }
        }
        assert!(hinted > 150, "only {hinted} hints");
    }

    /// Plays the human's first legal action and steps the opponent for up to
    /// `n` moves, calling `f` with the state, the human's tracker reply and
    /// the module's audit (the cards he must not learn) before each.
    fn walk_tracker(h: &mut Host, n: usize, mut f: impl FnMut(&Value, &Value, &Value)) {
        for _ in 0..n {
            h.render();
            let s = out(h)["state"].clone();
            h.show(|t| t.tracker());
            let tr = out(h);
            h.show(|t| t.audit());
            let audit = out(h);
            f(&s, &tr, &audit);
            if s["prompt"] == "over" {
                break;
            }
            if s["prompt"] == "opponent" {
                h.step();
            } else {
                let a = match &s["legal"][0] {
                    a if a["action"] == "play" => json!({"action":"play","card":a["card"]}),
                    a => a.clone(),
                };
                assert_eq!(h.send(&a.to_string()), 1);
            }
        }
    }

    #[test]
    fn the_tracker_places_every_card_and_names_no_hidden_one() {
        let mut h = host(CONFIG);
        let mut looked = 0;
        walk_tracker(&mut h, 120, |s, tr, audit| {
            let t = &tr["tracker"];
            assert_eq!(tr["protocol"], PROTOCOL);
            if s["deal"].is_null() {
                assert!(t.is_null());
                return;
            }
            assert_eq!(t["cards"].as_object().unwrap().len(), 54);
            assert_eq!(t["groups"].as_array().unwrap().len(), 5);
            for code in audit["hidden"].as_array().unwrap() {
                let place = &t["cards"][code.as_str().unwrap()]["place"];
                assert_eq!(place, "unseen", "{code} must only be unseen");
                looked += 1;
            }
            for c in s["deal"]["own_hand"].as_array().unwrap() {
                assert_eq!(t["cards"][c["card"].as_str().unwrap()]["place"], "hand");
            }
            for p in t["cards"].as_object().unwrap().values() {
                if p["place"] == "played" {
                    assert!(p["by"] == 0 || p["by"] == 1);
                    assert!(p["trick"].as_u64().unwrap() >= 1);
                }
            }
            assert_eq!(t["human"], s["human"]);
            assert!(t["out_thirds"].as_u64().unwrap() <= 210);
        });
        assert!(looked > 500, "{looked}");
    }

    #[test]
    fn the_tracker_needs_a_sitting_and_does_not_move_the_game() {
        let mut h = Host::default();
        h.show(|t| t.tracker());
        assert_eq!(out(&h)["error"], "no-sitting");
        let mut h = host(CONFIG);
        h.render();
        let before = out(&h);
        h.show(|t| t.tracker());
        h.render();
        assert_eq!(out(&h), before);
    }

    #[test]
    fn the_hint_does_not_move_the_game() {
        let mut h = host(CONFIG);
        while out(&h)["state"]["prompt"] != "you" {
            h.step();
        }
        h.render();
        let before = out(&h);
        h.show(|t| t.hint());
        h.render();
        assert_eq!(out(&h), before);
    }

    #[test]
    fn the_level_is_clamped_to_half_steps_and_echoed() {
        for (given, shown) in [("3", 3.0), ("2.4", 2.5), ("9", 4.0), ("0", 1.0)] {
            let h = host(&format!(r#"{{"seed":7,"touren":3,"level":{given}}}"#));
            assert_eq!(out(&h)["state"]["level"], shown, "{given}");
        }
        assert_eq!(out(&host(CONFIG))["state"]["level"], 1.0, "default");
    }

    #[test]
    fn every_level_plays_a_sitting_through() {
        for level in ["1", "2.5", "3"] {
            let mut h = host(&format!(r#"{{"seed":3,"touren":3,"level":{level}}}"#));
            for _ in 0..5000 {
                let s = out(&h)["state"].clone();
                match s["prompt"].as_str().unwrap() {
                    "over" => break,
                    "opponent" => assert_eq!(h.step(), 1, "level {level}"),
                    _ => {
                        let a = s["legal"][0].clone();
                        let a = if a["action"] == "extend" {
                            json!({"action":"extend","accept":false})
                        } else {
                            a
                        };
                        assert_eq!(h.send(&a.to_string()), 1);
                    }
                }
            }
            assert_eq!(out(&h)["state"]["prompt"], "over", "level {level}");
        }
    }

    /// Plays the first legal action for the human until `stop` holds.
    fn play_until(h: &mut Host, stop: impl Fn(&Value) -> bool) {
        h.render();
        for _ in 0..5000 {
            let s = out(h)["state"].clone();
            if stop(&s) || s["prompt"] == "over" {
                return;
            }
            if s["prompt"] == "opponent" {
                assert_eq!(h.step(), 1);
                continue;
            }
            let a = s["legal"][0].clone();
            let a = if a["action"] == "extend" {
                json!({"action":"extend","accept":false})
            } else if a["action"] == "play" {
                json!({"action":"play","card":a["card"]})
            } else {
                a
            };
            assert_eq!(h.send(&a.to_string()), 1);
        }
    }

    #[test]
    fn the_review_is_null_before_a_deal_is_scored() {
        let mut h = host(CONFIG);
        h.show(|t| t.review(false));
        assert_eq!(out(&h)["review"], Value::Null);
        h.show(|t| t.review(true));
        assert_eq!(out(&h)["review"], Value::Null);
    }

    #[test]
    fn the_review_follows_the_last_scored_deal_with_known_keys() {
        for seed in [3u64, 7, 11] {
            let mut h = host(&format!(r#"{{"seed":{seed},"touren":3,"human":1}}"#));
            play_until(&mut h, |s| s["deals_counted"].as_u64().unwrap_or(0) >= 1);
            h.show(|t| t.review(false));
            let r = out(&h);
            let review = &r["review"];
            assert!(review.is_object(), "{r}");
            assert_eq!(review["bullets"][0]["lead"], "count");
            for b in review["bullets"].as_array().unwrap() {
                let key = b["key"].as_str().unwrap();
                assert!(strohmandeln_ai::review::KEYS.contains(&key), "{key}");
                assert!(b["params"].is_object());
            }
            assert!(review["deal"].as_u64().unwrap() >= 1);
            play_until(&mut h, |_| false);
            h.show(|t| t.review(true));
            let r = out(&h);
            assert_eq!(r["review"]["bullets"].as_array().unwrap().len(), 3, "{r}");
            assert_eq!(r["review"]["bullets"][0]["key"], "sitting-tally");
        }
    }

    #[test]
    fn the_explanation_follows_each_trick_with_a_listed_key() {
        let mut h = host(r#"{"seed":5,"touren":3,"human":0}"#);
        h.show(|t| t.explain());
        assert_eq!(out(&h)["explain"], Value::Null, "no trick yet");
        let mut seen = 0;
        h.render();
        for _ in 0..400 {
            let s = out(&h)["state"].clone();
            if s["prompt"] == "over" {
                break;
            }
            let won = out(&h)["events"]
                .as_array()
                .is_some_and(|e| e.iter().any(|e| e["type"] == "trick_won"));
            if won {
                h.show(|t| t.explain());
                let r = out(&h);
                let key = r["explain"]["key"].as_str().unwrap();
                assert!(strohmandeln_ai::review::EXPLAIN_KEYS.contains(&key), "{r}");
                assert_eq!(r["explain"]["lead"], "explain");
                seen += 1;
                h.render();
            }
            if s["prompt"] == "opponent" {
                h.step();
            } else {
                let a = s["legal"][0].clone();
                let a = if a["action"] == "extend" {
                    json!({"action":"extend","accept":false})
                } else if a["action"] == "play" {
                    json!({"action":"play","card":a["card"]})
                } else {
                    a
                };
                h.send(&a.to_string());
            }
        }
        assert!(seen > 20, "only {seen} tricks explained");
    }

    #[test]
    fn a_review_does_not_move_the_game() {
        let mut h = host(CONFIG);
        play_until(&mut h, |s| s["deals_counted"].as_u64().unwrap_or(0) >= 1);
        h.render();
        let before = out(&h);
        h.show(|t| t.review(false));
        h.render();
        assert_eq!(out(&h), before);
    }
}
