//! Sets of cards as a 54-bit mask, and the small facts about the pack that
//! every rung uses (who wins a trick, what kind of card it is).

use strohmandeln_core::cards::{Card, Suit};

/// A set of cards: bit `i` is the card of index `i`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Default, Debug)]
pub struct CardSet(pub u64);

impl CardSet {
    pub const EMPTY: CardSet = CardSet(0);

    pub fn of(cards: impl IntoIterator<Item = Card>) -> CardSet {
        let mut s = CardSet::EMPTY;
        for c in cards {
            s.insert(c);
        }
        s
    }

    pub fn full() -> CardSet {
        CardSet((1u64 << Card::COUNT) - 1)
    }

    pub fn contains(self, c: Card) -> bool {
        self.0 >> c.index() & 1 == 1
    }

    pub fn insert(&mut self, c: Card) {
        self.0 |= 1 << c.index();
    }

    pub fn remove(&mut self, c: Card) {
        self.0 &= !(1 << c.index());
    }

    pub fn len(self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn union(self, o: CardSet) -> CardSet {
        CardSet(self.0 | o.0)
    }

    pub fn minus(self, o: CardSet) -> CardSet {
        CardSet(self.0 & !o.0)
    }

    pub fn intersect(self, o: CardSet) -> CardSet {
        CardSet(self.0 & o.0)
    }

    /// The cards in index order.
    pub fn iter(self) -> impl Iterator<Item = Card> {
        let mut bits = self.0;
        std::iter::from_fn(move || {
            if bits == 0 {
                return None;
            }
            let i = bits.trailing_zeros() as u8;
            bits &= bits - 1;
            Card::from_index(i)
        })
    }

    pub fn to_vec(self) -> Vec<Card> {
        self.iter().collect()
    }
}

/// What a card leads: a suit, or trump (RULES 5.3). `index()` is 0 to 3 for
/// the suits in `Suit::ALL` order and 4 for trump.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Kind {
    Suit(Suit),
    Trump,
}

impl Kind {
    pub const ALL: [Kind; 5] = [
        Kind::Suit(Suit::Hearts),
        Kind::Suit(Suit::Diamonds),
        Kind::Suit(Suit::Clubs),
        Kind::Suit(Suit::Spades),
        Kind::Trump,
    ];

    pub fn of(c: Card) -> Kind {
        match c.suit() {
            Some(s) => Kind::Suit(s),
            None => Kind::Trump,
        }
    }

    pub fn index(self) -> usize {
        match self {
            Kind::Suit(s) => s as usize,
            Kind::Trump => 4,
        }
    }

    pub fn from_index(i: usize) -> Kind {
        Kind::ALL[i]
    }

    /// Every card of this kind.
    pub fn cards(self) -> CardSet {
        CardSet::of(Card::all().filter(|&c| Kind::of(c) == self))
    }
}

/// Whether the card led beats the card followed (RULES 7.1): the highest
/// trump wins, else the highest card of the suit led; other suits never win.
pub fn leader_wins(led: Card, reply: Card) -> bool {
    match (led.is_tarock(), reply.is_tarock()) {
        (true, true) => led.strength() > reply.strength(),
        (true, false) => true,
        (false, true) => false,
        (false, false) => led.suit() != reply.suit() || led.strength() > reply.strength(),
    }
}

/// Thirds of a point in a trick of two cards.
pub fn trick_thirds(a: Card, b: Card) -> u32 {
    a.thirds() + b.thirds()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn c(s: &str) -> Card {
        Card::parse(s).unwrap()
    }

    #[test]
    fn a_set_holds_cards_and_lists_them_in_index_order() {
        let mut s = CardSet::of([c("Kh"), c("T1"), c("SK")]);
        assert_eq!(s.len(), 3);
        assert!(s.contains(c("T1")) && !s.contains(c("T2")));
        s.remove(c("T1"));
        assert_eq!(s.to_vec(), vec![c("SK"), c("Kh")]);
        assert_eq!(CardSet::full().len(), 54);
        assert_eq!(CardSet::full().iter().count(), 54);
        assert_eq!(s.minus(CardSet::of([c("SK")])).to_vec(), vec![c("Kh")]);
    }

    #[test]
    fn kinds_partition_the_pack() {
        let total: usize = Kind::ALL.iter().map(|k| k.cards().len()).sum();
        assert_eq!(total, 54);
        assert_eq!(Kind::Trump.cards().len(), 22);
        for (i, k) in Kind::ALL.iter().enumerate() {
            assert_eq!(Kind::from_index(i), *k);
            assert_eq!(k.index(), i);
        }
    }

    #[test]
    fn rules_7_1_the_winner_of_a_trick() {
        assert!(leader_wins(c("Kh"), c("Qh")));
        assert!(!leader_wins(c("Qh"), c("Kh")));
        assert!(!leader_wins(c("Qh"), c("T1")), "any tarock beats a suit");
        assert!(leader_wins(c("Qh"), c("Ks")), "another suit never wins");
        assert!(leader_wins(c("T1"), c("Kh")));
        assert!(!leader_wins(c("T20"), c("SK")));
        assert!(leader_wins(c("SK"), c("T21")));
        assert!(!leader_wins(c("T3"), c("T4")));
        // red pips: 1 is high
        assert!(leader_wins(c("1h"), c("4h")));
    }
}
