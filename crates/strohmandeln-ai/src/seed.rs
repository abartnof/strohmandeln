//! A stable seed from what a view shows, so that every decision is a pure
//! function of (seed, view): asked twice it answers the same, and a record
//! replays into the same game.

use strohmandeln_core::view::View;

/// FNV-1a over what the view shows. Stable on every platform and build.
pub fn view_seed(view: &View) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |x: u64| {
        for b in x.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
    };
    eat(view.seat.index() as u64);
    eat(u64::from(view.deals_counted));
    eat(u64::from(view.touren_total));
    eat(u64::from(view.pending_doubling));
    eat(view.legal_actions.len() as u64);
    if let Some(d) = &view.deal {
        eat(d.dealer.index() as u64);
        eat(u64::from(d.multiplier));
        eat(u64::from(d.trick_number));
        eat(d.declarer.map_or(2, |s| s.index() as u64));
        let mut hand = 0u64;
        for h in &d.own_hand {
            hand |= 1 << h.card.index();
        }
        eat(hand);
        for owner in &d.covers {
            for c in owner {
                eat(c.map_or(99, |c| c.index() as u64));
            }
        }
        for owner in &d.hidden {
            for n in owner {
                eat(u64::from(*n));
            }
        }
        eat(d.opponent_hand_size as u64);
        for c in &d.opponent_known {
            eat(c.index() as u64);
        }
        for t in &d.tricks {
            eat(t.cards[0].index() as u64 | (t.cards[1].index() as u64) << 8);
        }
        if let Some(l) = &d.lead {
            eat(l.card.index() as u64);
        }
        for t in d.take_thirds {
            eat(u64::from(t));
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use std::collections::HashSet;
    use strohmandeln_core::view::view;

    #[test]
    fn the_seed_is_a_function_of_the_view() {
        let mut seen = HashSet::new();
        let mut n = 0;
        for seed in 0..6 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                assert_eq!(view_seed(&v), view_seed(&v.clone()));
                seen.insert(view_seed(&v));
                n += 1;
            });
        }
        // Distinct positions give distinct seeds, nearly always.
        assert!(seen.len() * 100 > n * 97, "{} of {n}", seen.len());
    }
}
