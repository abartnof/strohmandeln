//! Rung 3's sampler: worlds consistent with everything a seat knows.
//!
//! A world fixes the cards the seat cannot see: the opponent's dealt hand
//! cards still in his hand, and the face-down cards of all six packets,
//! **its own included**: the owner of a packet does not know its unturned
//! cards either (RULES 13.3). They are drawn from the cards the seat cannot
//! place (`Knowledge::unseen`), never from a card it has seen, so the sampler
//! cannot reach a hidden card by any route but the view.
//!
//! Consistent means: the opponent's dealt cards avoid every kind he has
//! shown himself void in (a player who could not follow a suit held none of
//! it, and one who did not trump a suit held no tarock, RULES 5.4); the
//! packets have the sizes the table shows. Among the consistent worlds the
//! draw is uniform: the deal was a uniform shuffle and the only public facts
//! about the hidden cards are the voids.

use crate::cardset::{CardSet, Kind};
use crate::know::Knowledge;
use crate::sim::Sim;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World {
    /// The opponent's dealt (private) hand cards.
    pub opp_hand: Vec<Card>,
    /// The face-down cards of every packet, top first: `[0]` mine, `[1]` his.
    pub packets: [[Vec<Card>; 3]; 2],
}

/// Whether `world` could be the truth for `k`: right sizes, only unseen
/// cards, each exactly once, and the opponent's dealt cards void-free.
pub fn is_consistent(k: &Knowledge, w: &World) -> bool {
    let mut all = w.opp_hand.clone();
    for p in &w.packets {
        for pk in p {
            all.extend(pk.iter().copied());
        }
    }
    let set = CardSet::of(all.iter().copied());
    if set.len() != all.len() || set != k.unseen {
        return false;
    }
    if w.opp_hand.len() != k.opp_private() {
        return false;
    }
    if w.opp_hand.iter().any(|&c| k.opp_void[Kind::of(c).index()]) {
        return false;
    }
    (0..3).all(|i| {
        w.packets[0][i].len() == usize::from(k.my_hidden[i])
            && w.packets[1][i].len() == usize::from(k.opp_hidden[i])
    })
}

/// Draws a world uniformly from the consistent ones.
pub fn sample(k: &Knowledge, rng: &mut Rng) -> World {
    let mut pool = k.unseen.to_vec();
    let n_hand = k.opp_private();
    let allowed = |c: Card| !k.opp_void[Kind::of(c).index()];
    // Choose the opponent's hand among the cards he may hold; if the voids
    // leave too few (a view that cannot occur) fall back to any.
    let n_allowed = pool.iter().filter(|&&c| allowed(c)).count();
    let respect = n_allowed >= n_hand;
    let mut opp_hand = Vec::with_capacity(n_hand);
    while opp_hand.len() < n_hand {
        let candidates: Vec<usize> = (0..pool.len())
            .filter(|&i| !respect || allowed(pool[i]))
            .collect();
        let pick = candidates[rng.below(candidates.len() as u64) as usize];
        opp_hand.push(pool.swap_remove(pick));
    }
    rng.shuffle(&mut pool);
    let mut rest = pool.into_iter();
    let mut take = |n: u8| -> Vec<Card> { rest.by_ref().take(usize::from(n)).collect() };
    let packets = [
        std::array::from_fn(|i| take(k.my_hidden[i])),
        std::array::from_fn(|i| take(k.opp_hidden[i])),
    ];
    World { opp_hand, packets }
}

impl World {
    /// The search position this world makes: player 0 is the seat the
    /// knowledge belongs to. Before the reveal (bidding) the packets are
    /// turned first.
    pub fn sim(&self, k: &Knowledge) -> Sim {
        let covers = [k.covers, k.opp_covers];
        let hands = [
            k.hand,
            k.opp_known
                .union(CardSet::of(self.opp_hand.iter().copied())),
        ];
        let (leader, trick) = if k.playing {
            (if k.lead.is_some() { 1 } else { 0 }, k.trick_number as u8)
        } else {
            (if k.forehand == k.me { 0 } else { 1 }, 1)
        };
        let pending = if k.playing {
            Sim::vacated(&covers, &self.packets)
        } else {
            [0, 0]
        };
        let mut sim = Sim::new(
            hands,
            covers,
            self.packets.clone(),
            [k.take[k.me.index()], k.take[k.opp.index()]],
            leader,
            trick,
            k.lead,
            pending,
        );
        if !k.playing {
            sim.reveal();
        }
        sim
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    fn truth(
        s: &strohmandeln_core::session::Sitting,
        seat: strohmandeln_core::types::Seat,
    ) -> World {
        let deal = s.deal().unwrap();
        let k = Knowledge::from_view(&view(s, seat)).unwrap();
        let opp = seat.other();
        let opp_hand = deal
            .hand(opp)
            .iter()
            .map(|h| h.card)
            .filter(|&c| !k.opp_known.contains(c))
            .collect();
        let down = |owner, p: u8| -> Vec<Card> {
            let skip = usize::from(deal.cover(owner, p).is_some());
            deal.packet_cards(owner, p).into_iter().skip(skip).collect()
        };
        World {
            opp_hand,
            packets: [
                std::array::from_fn(|i| down(seat, i as u8 + 1)),
                std::array::from_fn(|i| down(opp, i as u8 + 1)),
            ],
        }
    }

    #[test]
    fn the_true_world_is_consistent_and_so_are_the_samples() {
        let mut rng = Rng::seeded(11);
        let mut checked = 0;
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let Some(k) = Knowledge::from_view(&view(s, seat)) else {
                    return;
                };
                if s.deal().is_none() {
                    return;
                }
                assert!(is_consistent(&k, &truth(s, seat)), "the truth, seed {seed}");
                for _ in 0..2 {
                    let w = sample(&k, &mut rng);
                    assert!(
                        is_consistent(&k, &w),
                        "seed {seed} trick {}",
                        k.trick_number
                    );
                    checked += 1;
                }
            });
        }
        assert!(checked > 2000);
    }

    #[test]
    fn a_perturbed_world_is_caught_by_the_check() {
        let s = strohmandeln_core::session::Sitting::new(2, Default::default(), 3).unwrap();
        let seat = s.to_move().unwrap();
        let k = Knowledge::from_view(&view(&s, seat)).unwrap();
        let mut w = truth(&s, seat);
        assert!(is_consistent(&k, &w));
        // a card the seat holds itself, put among the hidden ones
        w.opp_hand[0] = k.hand.iter().next().unwrap();
        assert!(!is_consistent(&k, &w));
        // a void broken
        let mut k2 = k.clone();
        let mut w = truth(&s, seat);
        k2.opp_void[Kind::of(w.opp_hand[0]).index()] = true;
        w.opp_hand.swap(0, 0);
        assert!(!is_consistent(&k2, &w));
    }

    #[test]
    fn the_world_sim_has_the_legal_moves_the_view_shows() {
        let mut rng = Rng::seeded(5);
        let mut n = 0;
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                if !k.playing || k.legal.is_empty() {
                    return;
                }
                let sim = sample(&k, &mut rng).sim(&k);
                assert_eq!(sim.mover(), 0);
                assert_eq!(sim.legal(), CardSet::of(k.legal.iter().copied()));
                n += 1;
            });
        }
        assert!(n > 1000);
    }

    #[test]
    fn the_sampler_draws_every_unseen_card_for_every_place_about_equally() {
        // At the start of a deal the opponent holds 15 of the 39 unseen cards.
        let s = strohmandeln_core::session::Sitting::new(9, Default::default(), 3).unwrap();
        let seat = s.to_move().unwrap();
        let k = Knowledge::from_view(&view(&s, seat)).unwrap();
        assert_eq!(k.unseen.len(), 39);
        let mut rng = Rng::seeded(2);
        let n = 4000;
        let mut in_hand = [0u32; 54];
        let mut in_my_packets = [0u32; 54];
        for _ in 0..n {
            let w = sample(&k, &mut rng);
            for c in &w.opp_hand {
                in_hand[c.index()] += 1;
            }
            for pk in &w.packets[0] {
                for c in pk {
                    in_my_packets[c.index()] += 1;
                }
            }
        }
        for c in k.unseen.iter() {
            let h = f64::from(in_hand[c.index()]) / f64::from(n);
            let m = f64::from(in_my_packets[c.index()]) / f64::from(n);
            assert!((h - 15.0 / 39.0).abs() < 0.04, "{c:?} in his hand {h}");
            assert!((m - 12.0 / 39.0).abs() < 0.04, "{c:?} in my packets {m}");
        }
    }

    #[test]
    fn the_sampler_uses_only_what_the_view_holds() {
        // Twin deals that differ only in the hidden cards give the same
        // knowledge, hence the same samples from the same stream.
        let knows = |v: &strohmandeln_core::view::View| {
            let k = Knowledge::from_view(v).unwrap();
            let mut rng = Rng::seeded(3);
            (0..3).map(|_| sample(&k, &mut rng)).collect::<Vec<_>>()
        };
        let twins = crate::testutil::blind_check_values(&knows, 25);
        assert!(twins > 100, "{twins}");
    }

    #[test]
    fn voids_are_respected_even_when_they_are_many() {
        // A position deep in a deal with voids: the samples keep them.
        let mut rng = Rng::seeded(8);
        let mut with_voids = 0;
        for seed in 0..30 {
            walk(seed, |s, seat| {
                let Some(k) = Knowledge::from_view(&view(s, seat)) else {
                    return;
                };
                if k.opp_void.iter().filter(|&&v| v).count() >= 2 && k.playing {
                    for _ in 0..3 {
                        let w = sample(&k, &mut rng);
                        for c in &w.opp_hand {
                            assert!(!k.opp_void[Kind::of(*c).index()]);
                        }
                    }
                    with_voids += 1;
                }
            });
        }
        assert!(with_voids > 50, "{with_voids}");
    }
}
