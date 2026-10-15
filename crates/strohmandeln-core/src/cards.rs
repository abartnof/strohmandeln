//! The pack: cards, codes, values and orders (RULES 1).

use serde::{Deserialize, Serialize};

/// A suit (RULES 1.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    pub const ALL: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];

    pub fn letter(self) -> char {
        match self {
            Suit::Hearts => 'h',
            Suit::Diamonds => 'd',
            Suit::Clubs => 'c',
            Suit::Spades => 's',
        }
    }

    /// Red suits rank pips `1 2 3 4` high to low; black `10 9 8 7` (RULES 1.5).
    fn is_red(self) -> bool {
        matches!(self, Suit::Hearts | Suit::Diamonds)
    }
}

/// One of the 54 cards. Identity is a number 0..54: `T1`..`T21` are 0..=20,
/// the Sküs is 21 (so a tarock's number is its strength, RULES 1.4), and the
/// 32 suit cards follow by suit then rank, lowest rank first (RULES 1.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Card(u8);

const SKUS: u8 = 21;
const FIRST_SUIT_CARD: u8 = 22;

/// The honours of RULES 11.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Honour {
    Trull,
    Kings,
}

impl Card {
    pub const COUNT: usize = 54;

    /// All 54 cards in canonical order (the order before shuffling).
    pub fn all() -> impl Iterator<Item = Card> {
        (0..Card::COUNT as u8).map(Card)
    }

    pub fn from_index(i: u8) -> Option<Card> {
        (usize::from(i) < Card::COUNT).then_some(Card(i))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }

    /// The tarock `T1`..`T21`.
    pub fn tarock(n: u8) -> Card {
        assert!((1..=21).contains(&n), "no tarock {n}");
        Card(n - 1)
    }

    pub const SKUS: Card = Card(SKUS);
    pub const PAGAT: Card = Card(0);
    pub const MOND: Card = Card(20);

    /// A suit card by suit and rank position (0 lowest .. 7 king).
    pub fn suit_card(suit: Suit, pos: u8) -> Card {
        assert!(pos < 8);
        Card(FIRST_SUIT_CARD + (suit as u8) * 8 + pos)
    }

    /// Tarocks include the Sküs (RULES 1.3).
    pub fn is_tarock(self) -> bool {
        self.0 < FIRST_SUIT_CARD
    }

    pub fn suit(self) -> Option<Suit> {
        if self.is_tarock() {
            None
        } else {
            Some(Suit::ALL[usize::from((self.0 - FIRST_SUIT_CARD) / 8)])
        }
    }

    /// Strength within the card's own order: tarock number order, or rank
    /// position in the suit (0 lowest .. 7 king).
    pub fn strength(self) -> u8 {
        if self.is_tarock() {
            self.0
        } else {
            (self.0 - FIRST_SUIT_CARD) % 8
        }
    }

    pub fn is_king(self) -> bool {
        !self.is_tarock() && self.strength() == 7
    }

    /// The Troule: `SK`, `T21`, `T1` (RULES 1.6).
    pub fn is_trull(self) -> bool {
        matches!(self.0, 0 | 20 | SKUS)
    }

    /// Card value 1..=5 (RULES 1.7).
    pub fn value(self) -> u32 {
        if self.is_tarock() {
            return if self.is_trull() { 5 } else { 1 };
        }
        match self.strength() {
            7 => 5,
            6 => 4,
            5 => 3,
            4 => 2,
            _ => 1,
        }
    }

    /// Thirds of a point (RULES 8.1): `3 x value - 2`.
    pub fn thirds(self) -> u32 {
        3 * self.value() - 2
    }

    /// The honours this card belongs to.
    pub fn honours(self) -> Vec<Honour> {
        let mut v = Vec::new();
        if self.is_trull() {
            v.push(Honour::Trull);
        }
        if self.is_king() {
            v.push(Honour::Kings);
        }
        v
    }

    /// The code of RULES 1.2.
    pub fn code(self) -> String {
        match self.suit() {
            None if self.0 == SKUS => "SK".to_string(),
            None => format!("T{}", self.0 + 1),
            Some(s) => {
                let rank = match (s.is_red(), self.strength()) {
                    (_, 7) => "K",
                    (_, 6) => "Q",
                    (_, 5) => "C",
                    (_, 4) => "J",
                    (true, p) => return format!("{}{}", 4 - p, s.letter()),
                    (false, p) => return format!("{}{}", 7 + p, s.letter()),
                };
                format!("{rank}{}", s.letter())
            }
        }
    }

    pub fn parse(code: &str) -> Option<Card> {
        if code == "SK" {
            return Some(Card::SKUS);
        }
        if let Some(n) = code.strip_prefix('T') {
            // digits only: `T01` and `T+1` are not codes
            let ok = !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()) && !n.starts_with('0');
            return n
                .parse::<u8>()
                .ok()
                .filter(|_| ok)
                .and_then(|n| (1..=21).contains(&n).then(|| Card::tarock(n)));
        }
        let suit = Suit::ALL
            .into_iter()
            .find(|s| code.ends_with(s.letter()) && code.len() >= 2)?;
        let rank = &code[..code.len() - 1];
        let pos = match (suit.is_red(), rank) {
            (_, "K") => 7,
            (_, "Q") => 6,
            (_, "C") => 5,
            (_, "J") => 4,
            (true, "1") => 3,
            (true, "2") => 2,
            (true, "3") => 1,
            (true, "4") => 0,
            (false, "10") => 3,
            (false, "9") => 2,
            (false, "8") => 1,
            (false, "7") => 0,
            _ => return None,
        };
        Some(Card::suit_card(suit, pos))
    }
}

impl Honour {
    /// The cards of the honour (RULES 1.6).
    pub fn cards(self) -> Vec<Card> {
        match self {
            Honour::Trull => vec![Card::SKUS, Card::MOND, Card::PAGAT],
            Honour::Kings => Suit::ALL.iter().map(|&s| Card::suit_card(s, 7)).collect(),
        }
    }
}

impl From<Card> for String {
    fn from(c: Card) -> String {
        c.code()
    }
}

impl TryFrom<String> for Card {
    type Error = String;
    fn try_from(s: String) -> Result<Card, String> {
        Card::parse(&s).ok_or_else(|| format!("not a card code: {s:?}"))
    }
}

impl Serialize for Card {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.code())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Card, D::Error> {
        let s = String::deserialize(d)?;
        Card::try_from(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(code: &str) -> Card {
        Card::parse(code).unwrap_or_else(|| panic!("bad code {code}"))
    }

    #[test]
    fn rules_1_1_the_pack_has_54_cards_22_of_them_tarocks() {
        assert_eq!(Card::all().count(), 54);
        assert_eq!(Card::all().filter(|c| c.is_tarock()).count(), 22);
        for s in Suit::ALL {
            assert_eq!(Card::all().filter(|c| c.suit() == Some(s)).count(), 8);
        }
    }

    #[test]
    fn rules_1_2_codes_are_unique_and_round_trip() {
        let codes: Vec<String> = Card::all().map(|c| c.code()).collect();
        let mut sorted = codes.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted.len(), 54);
        for card in Card::all() {
            assert_eq!(Card::parse(&card.code()), Some(card));
        }
        for code in [
            "T1", "T21", "SK", "Kh", "Qd", "Cc", "Js", "10s", "7c", "1h", "4d", "9c", "8s", "2h",
            "3d", "Ks",
        ] {
            assert_eq!(c(code).code(), code);
        }
        assert_eq!(Card::parse("sk"), None);
        assert_eq!(Card::parse("T22"), None);
        assert_eq!(Card::parse("10h"), None);
        assert_eq!(Card::parse("1c"), None);
    }

    #[test]
    fn rules_1_2_only_the_54_codes_parse_and_malformed_strings_do_not() {
        for bad in [
            "", "T", "T0", "T01", "T+1", "T-1", "T22", "T256", "h", "1", "Kx", "KH", "SKs", "sk",
            "Sk", "0h", "11h", "10", "7h", "1c", "10h", "Ké", "é", " Kh", "Kh ", "T1 ",
        ] {
            assert_eq!(Card::parse(bad), None, "{bad:?}");
        }
        let parsed = ["SK", "T1", "T21", "Kh", "10s", "4d", "1h"]
            .into_iter()
            .filter_map(Card::parse)
            .count();
        assert_eq!(parsed, 7);
    }

    #[test]
    fn rules_1_2_a_code_ends_in_a_lowercase_suit_letter_iff_a_suit_card() {
        for card in Card::all() {
            let last = card.code().chars().last().unwrap();
            assert_eq!(last.is_ascii_lowercase(), !card.is_tarock());
        }
    }

    #[test]
    fn rules_1_3_the_skus_is_the_highest_tarock() {
        assert!(c("SK").is_tarock());
        assert!(c("SK").strength() > c("T21").strength());
        assert!(c("T21").strength() > c("T2").strength());
        assert!(c("T2").strength() > c("T1").strength());
    }

    #[test]
    fn rules_1_5_black_suits_rank_k_q_c_j_10_9_8_7() {
        for s in ["c", "s"] {
            let order = ["K", "Q", "C", "J", "10", "9", "8", "7"];
            for w in order.windows(2) {
                let hi = c(&format!("{}{s}", w[0]));
                let lo = c(&format!("{}{s}", w[1]));
                assert!(
                    hi.strength() > lo.strength(),
                    "{} over {}",
                    hi.code(),
                    lo.code()
                );
            }
        }
    }

    #[test]
    fn rules_1_5_and_17_2_red_suits_rank_k_q_c_j_1_2_3_4() {
        for s in ["h", "d"] {
            let order = ["K", "Q", "C", "J", "1", "2", "3", "4"];
            for w in order.windows(2) {
                let hi = c(&format!("{}{s}", w[0]));
                let lo = c(&format!("{}{s}", w[1]));
                assert!(
                    hi.strength() > lo.strength(),
                    "{} over {}",
                    hi.code(),
                    lo.code()
                );
            }
        }
    }

    #[test]
    fn rules_1_6_kings_and_the_troule() {
        let kings: Vec<_> = Card::all().filter(|c| c.is_king()).collect();
        assert_eq!(kings.len(), 4);
        assert_eq!(Honour::Kings.cards().len(), 4);
        assert!(Honour::Kings.cards().iter().all(|c| c.is_king()));
        assert_eq!(Honour::Trull.cards(), vec![c("SK"), c("T21"), c("T1")]);
        assert_eq!(Card::all().filter(|c| c.is_trull()).count(), 3);
    }

    #[test]
    fn rules_1_7_card_values() {
        for k in ["Kh", "Kd", "Kc", "Ks", "SK", "T21", "T1"] {
            assert_eq!(c(k).value(), 5, "{k}");
        }
        for q in ["Qh", "Qd", "Qc", "Qs"] {
            assert_eq!(c(q).value(), 4);
        }
        for q in ["Ch", "Cd", "Cc", "Cs"] {
            assert_eq!(c(q).value(), 3);
        }
        for q in ["Jh", "Jd", "Jc", "Js"] {
            assert_eq!(c(q).value(), 2);
        }
        for q in ["1h", "4d", "7c", "10s", "T2", "T20", "T10"] {
            assert_eq!(c(q).value(), 1, "{q}");
        }
        assert_eq!(Card::all().map(|c| c.value()).sum::<u32>(), 106);
    }

    #[test]
    fn rules_8_1_a_card_counts_three_times_value_less_two_thirds() {
        assert_eq!(c("Kh").thirds(), 13);
        assert_eq!(c("Qh").thirds(), 10);
        assert_eq!(c("Ch").thirds(), 7);
        assert_eq!(c("Jh").thirds(), 4);
        assert_eq!(c("T5").thirds(), 1);
        // The whole pack is 70 points exactly.
        assert_eq!(Card::all().map(|c| c.thirds()).sum::<u32>(), 210);
    }

    #[test]
    fn rules_1_2_cards_serialise_as_their_code() {
        let json = serde_json::to_string(&c("T21")).unwrap();
        assert_eq!(json, "\"T21\"");
        let back: Card = serde_json::from_str("\"Kh\"").unwrap();
        assert_eq!(back, c("Kh"));
        assert!(serde_json::from_str::<Card>("\"Zz\"").is_err());
    }
}
