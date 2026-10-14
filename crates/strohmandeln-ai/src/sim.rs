//! A compact copy of the game of tricks for search: two hands as bit masks,
//! six packets, the takes. It plays RULES 4 to 7 (legal plays, tricks, packet
//! replacement) with no allocation, and is checked against the engine's own
//! `Deal` in the tests.
//!
//! Players are numbered 0 and 1 here, not by seat: 0 is the player the search
//! is for (the seat that decides), 1 the opponent.

use crate::cardset::{leader_wins, CardSet, Kind};
use std::sync::OnceLock;
use strohmandeln_core::cards::Card;
use strohmandeln_core::deal::Deal;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Seat;

const NONE: u8 = 255;

/// Thirds a Pagat ultimo is worth in the final margin: a Point of the
/// payment, set against card thirds (RULES 12).
pub const ULTIMO_THIRDS: i32 = 12;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Packet {
    /// The face-down cards, top first.
    cards: [u8; 4],
    len: u8,
    next: u8,
    cover: u8,
}

impl Packet {
    const EMPTY: Packet = Packet {
        cards: [NONE; 4],
        len: 0,
        next: 0,
        cover: NONE,
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sim {
    hand: [CardSet; 2],
    packets: [[Packet; 3]; 2],
    /// Thirds taken so far.
    pub take: [u32; 2],
    pub leader: u8,
    /// 1 to 27; 28 when the deal is over.
    pub trick: u8,
    led: u8,
    /// Packets whose cover was played to the open trick, per player (bit i
    /// is packet i), awaiting the replacement after the trick (RULES 6.1).
    pending: [u8; 2],
    /// Pagat ultimo, from player 0's side: +1 won, -1 lost, 0 none.
    pub ultimo: i8,
}

impl Sim {
    /// The packets whose cover was played to the open trick and is waiting to
    /// be replaced: no cover, cards beneath (RULES 4.8: otherwise a
    /// non-empty packet always has its cover).
    pub fn vacated(covers: &[[Option<Card>; 3]; 2], face_down: &[[Vec<Card>; 3]; 2]) -> [u8; 2] {
        let mut pending = [0u8; 2];
        for p in 0..2 {
            for i in 0..3 {
                if covers[p][i].is_none() && !face_down[p][i].is_empty() {
                    pending[p] |= 1 << i;
                }
            }
        }
        pending
    }

    /// Builds a position. `face_down[p][i]` are the face-down cards of
    /// player p's packet i, top first; `covers` the covers; `pending` the
    /// packets whose cover was just played to the open trick.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hands: [CardSet; 2],
        covers: [[Option<Card>; 3]; 2],
        face_down: [[Vec<Card>; 3]; 2],
        take: [u32; 2],
        leader: u8,
        trick: u8,
        led: Option<Card>,
        pending: [u8; 2],
    ) -> Sim {
        let mut packets = [[Packet::EMPTY; 3]; 2];
        for p in 0..2 {
            for i in 0..3 {
                let pk = &mut packets[p][i];
                for (j, c) in face_down[p][i].iter().enumerate() {
                    pk.cards[j] = c.index() as u8;
                }
                pk.len = face_down[p][i].len() as u8;
                pk.cover = covers[p][i].map_or(NONE, |c| c.index() as u8);
            }
        }
        Sim {
            hand: hands,
            packets,
            take,
            leader,
            trick,
            led: led.map_or(NONE, |c| c.index() as u8),
            pending,
            ultimo: 0,
        }
    }

    /// Turns the first card of every packet that has none up (the opening
    /// reveal, RULES 4): for a world drawn before the reveal.
    pub fn reveal(&mut self) {
        self.pending = [0; 2];
        for p in 0..2 {
            for i in 0..3 {
                self.advance(p, i);
            }
        }
    }

    /// The sim as seen by an omniscient player 0 = `me`, from a real deal in
    /// play (tests and measurements).
    pub fn from_deal(deal: &Deal, me: Seat) -> Sim {
        let you = me.other();
        let side = |s: Seat| {
            let hand = CardSet::of(deal.hand_cards(s));
            let covers: [Option<Card>; 3] = std::array::from_fn(|i| deal.cover(s, i as u8 + 1));
            let down: [Vec<Card>; 3] = std::array::from_fn(|i| {
                let all = deal.packet_cards(s, i as u8 + 1);
                all.into_iter()
                    .skip(usize::from(covers[i].is_some()))
                    .collect()
            });
            (hand, covers, down)
        };
        let (h0, c0, d0) = side(me);
        let (h1, c1, d1) = side(you);
        let led = deal.led_card();
        let leader = if deal.leader() == me { 0 } else { 1 };
        let covers = [c0, c1];
        let down = [d0, d1];
        let pending = Sim::vacated(&covers, &down);
        Sim::new(
            [h0, h1],
            covers,
            down,
            [
                deal.take(me).iter().map(|c| c.thirds()).sum(),
                deal.take(you).iter().map(|c| c.thirds()).sum(),
            ],
            leader,
            deal.trick_number() as u8,
            led.map(|(_, c, _)| c),
            pending,
        )
    }

    /// Covers and remaining face-down cards of every packet, for comparing
    /// positions reached by different routes.
    #[cfg(test)]
    fn canonical_packets(&self) -> Vec<(u8, Vec<u8>)> {
        self.packets
            .iter()
            .flatten()
            .map(|pk| {
                (
                    pk.cover,
                    pk.cards[usize::from(pk.next)..usize::from(pk.len)].to_vec(),
                )
            })
            .collect()
    }

    pub fn is_over(&self) -> bool {
        self.trick > 27
    }

    /// The player to play.
    pub fn mover(&self) -> u8 {
        if self.led == NONE {
            self.leader
        } else {
            1 - self.leader
        }
    }

    pub fn led(&self) -> Option<Card> {
        (self.led != NONE).then(|| card_of(self.led))
    }

    pub fn hand(&self, p: u8) -> CardSet {
        self.hand[usize::from(p)]
    }

    pub fn cover(&self, p: u8, i: usize) -> Option<Card> {
        let c = self.packets[usize::from(p)][i].cover;
        (c != NONE).then(|| card_of(c))
    }

    /// The playable set of `p`: hand and covers.
    pub fn playable(&self, p: u8) -> CardSet {
        let mut s = self.hand[usize::from(p)];
        for pk in &self.packets[usize::from(p)] {
            if pk.cover != NONE {
                s.0 |= 1 << pk.cover;
            }
        }
        s
    }

    /// The legal plays of the mover (RULES 5.3, 5.4).
    pub fn legal(&self) -> CardSet {
        let set = self.playable(self.mover());
        if self.led == NONE {
            return set;
        }
        let led = card_of(self.led);
        let kind = Kind::of(led);
        let following = set.intersect(kind_mask(kind));
        if !following.is_empty() {
            return following;
        }
        if kind != Kind::Trump {
            let trumps = set.intersect(kind_mask(Kind::Trump));
            if !trumps.is_empty() {
                return trumps;
            }
        }
        set
    }

    /// Plays `card` for the mover. Returns `Some((winner, thirds))` when it
    /// completes a trick.
    pub fn play(&mut self, card: Card) -> Option<(u8, u32)> {
        let p = usize::from(self.mover());
        let idx = card.index() as u8;
        if self.hand[p].contains(card) {
            self.hand[p].remove(card);
        } else {
            let i = self.packets[p]
                .iter()
                .position(|pk| pk.cover == idx)
                .expect("a card played is held");
            self.packets[p][i].cover = NONE;
            self.pending[p] |= 1 << i;
        }
        if self.led == NONE {
            self.led = idx;
            return None;
        }
        let led = card_of(self.led);
        self.led = NONE;
        let leader = self.leader;
        let winner = if leader_wins(led, card) {
            leader
        } else {
            1 - leader
        };
        let thirds = led.thirds() + card.thirds();
        self.take[usize::from(winner)] += thirds;
        if self.trick == 27 {
            let pagat = |c: Card| c == Card::PAGAT;
            let by = if pagat(led) {
                Some(leader)
            } else if pagat(card) {
                Some(1 - leader)
            } else {
                None
            };
            if let Some(by) = by {
                let won = by == winner;
                let from0 = if by == 0 { 1 } else { -1 };
                self.ultimo = if won { from0 } else { -from0 };
            }
        }
        // Replacement, the leader's packets first (RULES 6.3).
        for p in [leader, 1 - leader] {
            let bits = std::mem::take(&mut self.pending[usize::from(p)]);
            for i in 0..3 {
                if bits >> i & 1 == 1 {
                    self.advance(usize::from(p), i);
                }
            }
        }
        self.leader = winner;
        self.trick += 1;
        Some((winner, thirds))
    }

    /// Turns cards of a packet until a cover is set or it is empty (RULES
    /// 4.4 to 4.7, 6.1).
    fn advance(&mut self, p: usize, i: usize) {
        loop {
            let pk = &mut self.packets[p][i];
            if pk.next >= pk.len {
                return;
            }
            let idx = pk.cards[usize::from(pk.next)];
            pk.next += 1;
            let card = card_of(idx);
            let last = pk.next == pk.len;
            if last || card.is_tarock() || card.is_king() {
                self.hand[p].insert(card);
                if last {
                    return;
                }
            } else {
                pk.cover = idx;
                return;
            }
        }
    }

    /// The final margin for `p` in thirds, with the ultimo weighed in. Only
    /// meaningful when the deal is over.
    pub fn margin(&self, p: u8) -> i32 {
        let p = usize::from(p);
        let thirds = self.take[p] as i32 - self.take[1 - p] as i32;
        let u = i32::from(self.ultimo) * ULTIMO_THIRDS;
        thirds + if p == 0 { u } else { -u }
    }

    /// A hash of the position at the start of a trick, for the table.
    pub fn key(&self) -> u64 {
        debug_assert_eq!(self.led, NONE);
        let z = zobrist();
        let mut h = z.leader[usize::from(self.leader)];
        for p in 0..2 {
            for c in self.hand[p].iter() {
                h ^= z.hand[p][c.index()];
            }
            for (i, pk) in self.packets[p].iter().enumerate() {
                h ^= z.next[p][i][usize::from(pk.next)];
                if pk.cover != NONE {
                    h ^= z.cover[p][i][usize::from(pk.cover)];
                }
            }
        }
        h
    }

    /// A deterministic rollout to the end of the deal: the cheap policy
    /// below for both players. Returns the final margin for player 0.
    pub fn rollout(self) -> i32 {
        self.rollout_for(0)
    }

    /// The thirds player 0 takes when the deal is played out by the policy.
    pub fn rollout_thirds(mut self) -> u32 {
        while !self.is_over() {
            let card = self.policy();
            self.play(card);
        }
        self.take[0]
    }

    /// As `rollout`, the final margin for player `p`.
    pub fn rollout_for(mut self, p: u8) -> i32 {
        while !self.is_over() {
            let card = self.policy();
            self.play(card);
        }
        self.margin(p)
    }

    /// The rollout policy. Following: the cheapest winner when the trick is
    /// worth at least a Jack, else the cheapest loser. Leading: the cheapest
    /// card, packet covers first. It is not good; it is the same for both
    /// sides and costs nothing.
    pub fn policy(&self) -> Card {
        let legal = self.legal();
        let me = self.mover();
        let is_cover = |c: Card| {
            self.packets[usize::from(me)]
                .iter()
                .any(|pk| pk.cover == c.index() as u8)
        };
        let cost = |c: Card| {
            (
                c.thirds(),
                u8::from(!is_cover(c)),
                u8::from(c.is_tarock()),
                c.strength(),
            )
        };
        if let Some(led) = self.led() {
            if led.thirds() >= 4 {
                let winner = legal
                    .iter()
                    .filter(|&c| !leader_wins(led, c))
                    .min_by_key(|&c| (c.is_tarock(), c.strength()));
                if let Some(w) = winner {
                    return w;
                }
            }
            let loser = legal
                .iter()
                .filter(|&c| leader_wins(led, c))
                .min_by_key(|&c| cost(c));
            if let Some(l) = loser {
                return l;
            }
        }
        legal.iter().min_by_key(|&c| cost(c)).expect("a legal card")
    }

    /// A uniformly random legal card.
    pub fn random_move(&self, rng: &mut Rng) -> Card {
        let legal = self.legal().to_vec();
        legal[rng.below(legal.len() as u64) as usize]
    }
}

fn card_of(idx: u8) -> Card {
    Card::from_index(idx).expect("a card index")
}

fn kind_mask(kind: Kind) -> CardSet {
    static MASKS: OnceLock<[CardSet; 5]> = OnceLock::new();
    MASKS.get_or_init(|| std::array::from_fn(|i| Kind::from_index(i).cards()))[kind.index()]
}

struct Zobrist {
    hand: [[u64; 54]; 2],
    cover: [[[u64; 54]; 3]; 2],
    next: [[[u64; 5]; 3]; 2],
    leader: [u64; 2],
}

fn zobrist() -> &'static Zobrist {
    static Z: OnceLock<Zobrist> = OnceLock::new();
    Z.get_or_init(|| {
        let mut rng = Rng::seeded(0x57_0841_4d41_4e44);
        let mut z = Zobrist {
            hand: [[0; 54]; 2],
            cover: [[[0; 54]; 3]; 2],
            next: [[[0; 5]; 3]; 2],
            leader: [0; 2],
        };
        for p in 0..2 {
            z.hand[p].iter_mut().for_each(|x| *x = rng.next_u64());
            z.leader[p] = rng.next_u64();
            for i in 0..3 {
                z.cover[p][i].iter_mut().for_each(|x| *x = rng.next_u64());
                z.next[p][i].iter_mut().for_each(|x| *x = rng.next_u64());
            }
        }
        z
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::Scene;

    fn random_scene(seed: u64) -> Scene {
        let mut rng = Rng::seeded(seed);
        let mut deck: Vec<Card> = Card::all().collect();
        rng.shuffle(&mut deck);
        let mut s = Scene::new(&deck);
        s.apply(Seat::A, strohmandeln_core::types::Action::Take)
            .unwrap();
        s
    }

    #[test]
    fn the_sim_plays_exactly_as_the_engine_does() {
        let mut cards_checked = 0;
        for seed in 0..60 {
            let mut scene = random_scene(seed);
            let mut rng = Rng::seeded(seed + 1000);
            while let Some(seat) = scene.deal.to_move() {
                let sim = Sim::from_deal(&scene.deal, seat);
                assert_eq!(sim.mover(), 0);
                // legal moves agree
                let engine: CardSet =
                    CardSet::of(scene.deal.legal_plays(seat).into_iter().map(|(c, _)| c));
                assert_eq!(sim.legal(), engine, "seed {seed}");
                // play a random legal card in both and compare the result
                let legal = engine.to_vec();
                let card = legal[rng.below(legal.len() as u64) as usize];
                let mut sim2 = sim.clone();
                sim2.play(card);
                scene
                    .apply(seat, strohmandeln_core::types::Action::play(card))
                    .unwrap();
                if let Some(next) = scene.deal.to_move() {
                    let check = Sim::from_deal(&scene.deal, seat);
                    assert_eq!(sim2.take, check.take, "seed {seed}");
                    // after a full trick the states are the same too
                    if scene.deal.led_card().is_none() {
                        let want = Sim::from_deal(&scene.deal, seat);
                        assert_eq!(sim2.hand, want.hand, "seed {seed}");
                        assert_eq!(
                            sim2.canonical_packets(),
                            want.canonical_packets(),
                            "seed {seed}"
                        );
                        assert_eq!(sim2.leader, want.leader);
                        assert_eq!(sim2.trick, want.trick);
                    }
                    let _ = next;
                }
                cards_checked += 1;
            }
        }
        assert!(cards_checked > 3000);
    }

    #[test]
    fn a_full_playout_matches_the_engine_score() {
        for seed in 0..30 {
            let mut scene = random_scene(seed);
            let mut sim = Sim::from_deal(&scene.deal, Seat::A);
            let mut rng = Rng::seeded(seed);
            while let Some(seat) = scene.deal.to_move() {
                let card = sim.random_move(&mut rng);
                scene
                    .apply(seat, strohmandeln_core::types::Action::play(card))
                    .unwrap();
                sim.play(card);
            }
            assert!(sim.is_over());
            let a: u32 = scene.deal.take(Seat::A).iter().map(|c| c.thirds()).sum();
            assert_eq!(sim.take[0], a);
            assert_eq!(sim.take[0] + sim.take[1], 210);
        }
    }

    #[test]
    fn the_pagat_ultimo_is_scored_for_the_player_who_plays_it_last() {
        // Hand-built last trick: player 0 leads the Pagat and wins against a
        // suit card; or loses it to a tarock.
        let build = |opp: &str| {
            let mut sim = Sim::new(
                [
                    CardSet::of([Card::PAGAT]),
                    CardSet::of([Card::parse(opp).unwrap()]),
                ],
                [[None; 3]; 2],
                Default::default(),
                [100, 100],
                0,
                27,
                None,
                [0, 0],
            );
            sim.play(Card::PAGAT);
            sim.play(Card::parse(opp).unwrap());
            sim
        };
        assert_eq!(build("Kh").ultimo, 1);
        assert_eq!(build("T2").ultimo, -1);
        assert!(build("Kh").margin(0) > build("T2").margin(0));
        assert_eq!(build("Kh").margin(1), -build("Kh").margin(0));
    }

    #[test]
    fn rollouts_are_deterministic_and_end_the_deal() {
        let scene = random_scene(3);
        let sim = Sim::from_deal(&scene.deal, Seat::A);
        assert_eq!(sim.clone().rollout(), sim.clone().rollout());
    }
}
