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

/// Why an action was refused: a kind for programs and a sentence for people.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Refusal {
    pub kind: RefusalKind,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalKind {
    NotYourTurn,
    WrongPhase,
    NotPlayable,
    MustFollowSuit,
    MustTrump,
    NotHeld,
    AlreadyAnnounced,
    Finished,
}

impl Refusal {
    pub fn new(kind: RefusalKind, reason: impl Into<String>) -> Refusal {
        Refusal {
            kind,
            reason: reason.into(),
        }
    }
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reason)
    }
}

impl std::error::Error for Refusal {}

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
