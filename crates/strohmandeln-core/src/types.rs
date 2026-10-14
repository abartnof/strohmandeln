//! Seats, settings, actions, refusals and events: the plain data of the
//! engine (RULES 16). Everything here is serde-serialisable for the wasm layer.

use crate::cards::{Card, Honour};
use serde::{Deserialize, Serialize};

/// A seat id: 0 or 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub enum Seat {
    A,
    B,
}

impl Seat {
    pub const BOTH: [Seat; 2] = [Seat::A, Seat::B];
    pub fn other(self) -> Seat {
        match self {
            Seat::A => Seat::B,
            Seat::B => Seat::A,
        }
    }
    pub fn index(self) -> usize {
        self as usize
    }
    pub fn from_index(i: usize) -> Seat {
        if i == 0 {
            Seat::A
        } else {
            Seat::B
        }
    }
}

impl From<Seat> for u8 {
    fn from(s: Seat) -> u8 {
        s as u8
    }
}

impl TryFrom<u8> for Seat {
    type Error = String;
    fn try_from(n: u8) -> Result<Seat, String> {
        match n {
            0 => Ok(Seat::A),
            1 => Ok(Seat::B),
            _ => Err(format!("no seat {n}")),
        }
    }
}

/// Setting `both_pass` (RULES 3.4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BothPass {
    Play,
    ThrowIn,
}

/// Setting `values` (RULES 9.1): simple / declarer wins / declarer loses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Values {
    #[serde(rename = "2/3/4")]
    V234,
    #[serde(rename = "1/2/3")]
    V123,
    #[serde(rename = "1/2/4")]
    V124,
}

impl Values {
    /// (simple game won, declarer wins, declarer loses)
    pub fn triple(self) -> (u32, u32, u32) {
        match self {
            Values::V234 => (2, 3, 4),
            Values::V123 => (1, 2, 3),
            Values::V124 => (1, 2, 4),
        }
    }
}

/// The three settings of RULES 0 ("Authority and settings").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    pub both_pass: BothPass,
    pub values: Values,
    /// Pagat ultimo, 1 or 2 Points (RULES 12.3).
    pub ultimo: u32,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            both_pass: BothPass::Play,
            values: Values::V234,
            ultimo: 1,
        }
    }
}

/// Where a played card came from (RULES 5.7). Serialised `hand` or `packet n`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Source {
    Hand,
    /// Packet number 1 to 3.
    Packet(u8),
}

impl From<Source> for String {
    fn from(s: Source) -> String {
        match s {
            Source::Hand => "hand".into(),
            Source::Packet(n) => format!("packet {n}"),
        }
    }
}

impl TryFrom<String> for Source {
    type Error = String;
    fn try_from(s: String) -> Result<Source, String> {
        if s == "hand" {
            return Ok(Source::Hand);
        }
        match s.strip_prefix("packet ").and_then(|n| n.parse::<u8>().ok()) {
            Some(n @ 1..=3) => Ok(Source::Packet(n)),
            _ => Err(format!("not a source: {s:?}")),
        }
    }
}

/// What a seat may do. `Take` and `Pass` are the bids (RULES 3); `Play`
/// carries the optional announcements of RULES 11.4 made just before the card;
/// `Announce` is the same announcement on its own; `Extend` answers the offer
/// of RULES 15.6.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum Action {
    Take,
    Pass,
    Announce {
        honour: Honour,
    },
    Play {
        card: Card,
        #[serde(default)]
        announce: Vec<Honour>,
    },
    Extend {
        accept: bool,
    },
}

impl Action {
    pub fn play(card: Card) -> Action {
        Action::Play {
            card,
            announce: Vec::new(),
        }
    }
}

/// Why an action was refused: a stable message key and structured
/// parameters, never a sentence. The table translates the key (see
/// `docs/MESSAGES.md`); `Display` gives an English rendering for tests and the
/// terminal client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refusal {
    #[serde(rename = "key")]
    pub kind: RefusalKind,
    /// The card the player tried to play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    /// The suit letter led (`h d c s`) or `trump`, for the follow rules.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub led: Option<String>,
    /// The cards that would have been allowed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed: Vec<Card>,
    /// The honour cards missing from the hand, for a false announcement.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<Card>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub honour: Option<Honour>,
    /// Whose turn it is, for out-of-turn refusals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub speaker: Option<Seat>,
}

/// The message keys of refusals (kebab-case on the wire).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RefusalKind {
    NotYourTurn,
    WrongPhase,
    NotPlayable,
    MustFollowSuit,
    MustTrump,
    NotHeld,
    AlreadyAnnounced,
    HonourForfeited,
    DuplicateAnnouncement,
    Finished,
}

impl Refusal {
    pub fn of(kind: RefusalKind) -> Refusal {
        Refusal {
            kind,
            card: None,
            led: None,
            allowed: Vec::new(),
            missing: Vec::new(),
            honour: None,
            speaker: None,
        }
    }
    pub fn with_card(mut self, card: Card) -> Refusal {
        self.card = Some(card);
        self
    }
    pub fn with_honour(mut self, honour: Honour) -> Refusal {
        self.honour = Some(honour);
        self
    }
    pub fn with_speaker(mut self, seat: Seat) -> Refusal {
        self.speaker = Some(seat);
        self
    }
    pub fn with_led(mut self, led: &str) -> Refusal {
        self.led = Some(led.to_string());
        self
    }
    pub fn with_allowed(mut self, cards: Vec<Card>) -> Refusal {
        self.allowed = cards;
        self
    }
    pub fn with_missing(mut self, cards: Vec<Card>) -> Refusal {
        self.missing = cards;
        self
    }
}

fn codes(cards: &[Card]) -> String {
    cards
        .iter()
        .map(|c| c.code())
        .collect::<Vec<_>>()
        .join(", ")
}

impl std::fmt::Display for Refusal {
    /// An English rendering, for tests and the terminal client only.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let card = self.card.map(|c| c.code()).unwrap_or_default();
        let honour = match self.honour {
            Some(Honour::Trull) => "the Troule",
            Some(Honour::Kings) => "the four kings",
            None => "the honour",
        };
        match self.kind {
            RefusalKind::NotYourTurn => write!(f, "It is not your turn."),
            RefusalKind::WrongPhase => write!(f, "That cannot be done now."),
            RefusalKind::NotPlayable => write!(
                f,
                "{card} is not among your hand cards or uncovered packet cards."
            ),
            RefusalKind::MustFollowSuit => write!(
                f,
                "You must follow suit ({} was led); {card} is not allowed; play one of: {}.",
                self.led.as_deref().unwrap_or("?"),
                codes(&self.allowed)
            ),
            RefusalKind::MustTrump => write!(
                f,
                "You hold nothing of the suit led, so you must play a tarock; {card} is not allowed; play one of: {}.",
                codes(&self.allowed)
            ),
            RefusalKind::NotHeld => write!(
                f,
                "You cannot announce {honour}: you do not hold {} in your hand now.",
                codes(&self.missing)
            ),
            RefusalKind::AlreadyAnnounced => write!(f, "You have already announced {honour}."),
            RefusalKind::HonourForfeited => write!(
                f,
                "You played a card of {honour} without announcing it, so it is lost for this deal."
            ),
            RefusalKind::DuplicateAnnouncement => {
                write!(f, "You listed {honour} twice.")
            }
            RefusalKind::Finished => write!(f, "This deal is over."),
        }
    }
}

impl std::error::Error for Refusal {}

/// Why a sitting could not be begun or replayed: a message key and
/// parameters, as for a [`Refusal`] (`docs/MESSAGES.md`).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "key", rename_all = "kebab-case")]
pub enum SetupError {
    /// A sitting is 3 or 6 Touren (RULES 15.2).
    BadTouren { touren: u32 },
    /// The ultimo setting is 1 or 2 Points (RULES 12.3).
    BadUltimo { ultimo: u32 },
    /// The record is of another version.
    RecordVersion { found: u32, expected: u32 },
    /// A move of the record was refused on replay.
    ReplayRefused { index: usize, refusal: Refusal },
}

impl std::fmt::Display for SetupError {
    /// An English rendering, for tests and the terminal client only.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetupError::BadTouren { touren } => {
                write!(f, "a sitting is 3 or 6 Touren, not {touren}")
            }
            SetupError::BadUltimo { ultimo } => {
                write!(f, "the ultimo is 1 or 2 Points, not {ultimo}")
            }
            SetupError::RecordVersion { found, expected } => {
                write!(f, "record version {found} is not {expected}")
            }
            SetupError::ReplayRefused { index, refusal } => {
                write!(f, "move {index} refused on replay: {refusal}")
            }
        }
    }
}

impl std::error::Error for SetupError {}

/// How a deal came out (RULES 8, 9.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameResult {
    DeclarerWin,
    DeclarerLoss,
    SimpleWin,
    Draw,
}

/// Why points were credited (RULES 15.9).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cause {
    Game,
    Trull,
    Kings,
    UltimoWon,
    UltimoLost,
    Valat,
}

/// One credit of points to a seat's Conto, already doubled if a doubling applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credit {
    pub seat: Seat,
    pub cause: Cause,
    pub points: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UltimoResult {
    Won,
    Lost,
}

/// The events of RULES 16, in emission order. Plain data.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
    SittingStart {
        first_dealer: Seat,
        touren: u32,
        settings: Settings,
    },
    DealStart {
        deal_number: u32,
        tour: u32,
        dealer: Seat,
        forehand: Seat,
        multiplier: u32,
    },
    /// One seat's own 15 cards. Only that seat sees it.
    Dealt {
        seat: Seat,
        cards: Vec<Card>,
    },
    VoidDeal {
        seats: Vec<Seat>,
    },
    Bid {
        seat: Seat,
    },
    Pass {
        seat: Seat,
    },
    BothPass {
        mode: BothPass,
    },
    ThrownIn,
    RevealStart {
        seat: Seat,
    },
    PacketTurned {
        seat: Seat,
        packet: u8,
        card: Card,
    },
    CardPulled {
        seat: Seat,
        packet: u8,
        card: Card,
        last: bool,
    },
    CoverSet {
        seat: Seat,
        packet: u8,
        card: Card,
    },
    PacketEmpty {
        seat: Seat,
        packet: u8,
    },
    Announcement {
        seat: Seat,
        honour: Honour,
        cards: Vec<Card>,
    },
    HonourForfeited {
        seat: Seat,
        honour: Honour,
    },
    CardPlayed {
        seat: Seat,
        card: Card,
        source: Source,
        trick: u32,
    },
    TrickWon {
        winner: Seat,
        cards: [Card; 2],
        trick: u32,
    },
    PagatUltimo {
        seat: Seat,
        result: UltimoResult,
    },
    Valat {
        seat: Seat,
    },
    Count {
        thirds: [u32; 2],
        declarer: Option<Seat>,
        threshold: u32,
    },
    GameScored {
        result: GameResult,
        credits: Vec<Credit>,
        multiplier: u32,
        contos: [u32; 2],
    },
    DoublingSet,
    DoublingApplied {
        multiplier: u32,
    },
    DoublingLapsed,
    TourEnd {
        tour: u32,
        contos: [u32; 2],
        balance: i32,
    },
    ExtensionOffered,
    ExtensionDecided {
        accepted: bool,
        touren_added: u32,
    },
    SittingEnd {
        contos: [u32; 2],
        balance: i32,
        payer: Option<Seat>,
        kreuzer: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_16_events_are_plain_tagged_data() {
        let e = Event::CardPlayed {
            seat: Seat::B,
            card: Card::PAGAT,
            source: Source::Packet(2),
            trick: 27,
        };
        let json = serde_json::to_string(&e).unwrap();
        assert_eq!(
            json,
            r#"{"type":"card_played","seat":1,"card":"T1","source":"packet 2","trick":27}"#
        );
        let back: Event = serde_json::from_str(&json).unwrap();
        assert_eq!(back, e);
    }

    #[test]
    fn refusals_are_message_keys_with_parameters_not_sentences() {
        let r = Refusal::of(RefusalKind::MustFollowSuit)
            .with_card(Card::PAGAT)
            .with_led("h")
            .with_allowed(vec![Card::SKUS]);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"key":"must-follow-suit","card":"T1","led":"h","allowed":["SK"]}"#
        );
        let r = Refusal::of(RefusalKind::NotYourTurn).with_speaker(Seat::A);
        assert_eq!(
            serde_json::to_string(&r).unwrap(),
            r#"{"key":"not-your-turn","speaker":0}"#
        );
    }

    #[test]
    fn docs_messages_md_lists_exactly_the_keys_the_code_can_emit() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let documented: std::collections::BTreeSet<&str> = doc
            .lines()
            .filter_map(|l| l.strip_prefix("| `"))
            .filter_map(|l| l.split('`').next())
            .collect();
        let refusals = [
            RefusalKind::NotYourTurn,
            RefusalKind::WrongPhase,
            RefusalKind::NotPlayable,
            RefusalKind::MustFollowSuit,
            RefusalKind::MustTrump,
            RefusalKind::NotHeld,
            RefusalKind::AlreadyAnnounced,
            RefusalKind::HonourForfeited,
            RefusalKind::DuplicateAnnouncement,
            RefusalKind::Finished,
        ];
        let mut emitted: std::collections::BTreeSet<String> = refusals
            .iter()
            .map(|k| {
                serde_json::to_value(Refusal::of(*k)).unwrap()["key"]
                    .as_str()
                    .unwrap()
                    .to_string()
            })
            .collect();
        let setup = [
            SetupError::BadTouren { touren: 0 },
            SetupError::BadUltimo { ultimo: 0 },
            SetupError::RecordVersion {
                found: 0,
                expected: 0,
            },
            SetupError::ReplayRefused {
                index: 0,
                refusal: Refusal::of(RefusalKind::Finished),
            },
        ];
        emitted.extend(setup.iter().map(|e| {
            serde_json::to_value(e).unwrap()["key"]
                .as_str()
                .unwrap()
                .to_string()
        }));
        let documented: std::collections::BTreeSet<String> =
            documented.into_iter().map(String::from).collect();
        // `Key` is the table's header row in backticks-free form, so absent
        assert_eq!(documented, emitted);
    }

    #[test]
    fn rules_9_1_value_triples() {
        assert_eq!(Values::V234.triple(), (2, 3, 4));
        assert_eq!(Values::V123.triple(), (1, 2, 3));
        assert_eq!(Values::V124.triple(), (1, 2, 4));
        assert_eq!(serde_json::to_string(&Values::V124).unwrap(), "\"1/2/4\"");
    }

    #[test]
    fn rules_0_defaults_are_play_234_and_ultimo_1() {
        let s = Settings::default();
        assert_eq!(s.both_pass, BothPass::Play);
        assert_eq!(s.values, Values::V234);
        assert_eq!(s.ultimo, 1);
    }

    #[test]
    fn actions_round_trip_through_json() {
        for a in [
            Action::Take,
            Action::Pass,
            Action::Announce {
                honour: Honour::Kings,
            },
            Action::play(Card::SKUS),
            Action::Extend { accept: true },
        ] {
            let j = serde_json::to_string(&a).unwrap();
            assert_eq!(serde_json::from_str::<Action>(&j).unwrap(), a);
        }
    }
}
