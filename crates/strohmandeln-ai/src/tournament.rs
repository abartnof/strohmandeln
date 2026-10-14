//! Mirrored pairs: each seed's sitting played twice, the two players swapping
//! seats, so the luck of the deals cancels and what remains is play.
//!
//! The unit is the Points A takes from B over one sitting (3 Touren: 6
//! deals), summed over the pair: A's Conto less B's in the first sitting
//! (A in seat A), plus the same for the second (A in seat B). The seed fixes
//! the shuffles and the dealer, so both sittings start from the same cards;
//! the sittings then diverge by the bids and the play.

use strohmandeln_core::agents::{play_out, Player};
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Seat, Settings};

/// Makes a player for one game: the game's seed and the seat it sits in, so
/// its own choices are reproducible and independent of the cards.
pub type Factory<'a> = &'a dyn Fn(u64, Seat) -> Box<dyn Player>;

/// Plays a sitting of `touren` with `a` in seat A and `b` in seat B; seat A's
/// balance in Points (Conto A less Conto B).
pub fn play_sitting(
    seed: u64,
    settings: Settings,
    touren: u32,
    mut players: [Box<dyn Player>; 2],
) -> i32 {
    let mut s = Sitting::new(seed, settings, touren).expect("a sitting");
    play_out(&mut s, &mut players, 1_000_000).expect("a legal sitting");
    s.balance()
}

/// One mirrored pair from `seed`: A's Points less B's, over both seatings.
pub fn mirrored(seed: u64, settings: Settings, touren: u32, a: Factory, b: Factory) -> i32 {
    let first = play_sitting(seed, settings, touren, [a(seed, Seat::A), b(seed, Seat::B)]);
    let second = play_sitting(seed, settings, touren, [b(seed, Seat::A), a(seed, Seat::B)]);
    first - second
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opponent::{Level, Opponent};
    use crate::stats::{run_sequential, Plan, Verdict};

    fn at(halves: u8) -> impl Fn(u64, Seat) -> Box<dyn Player> {
        move |seed, seat| {
            Box::new(Opponent::new(
                Level::from_halves(halves),
                seed ^ ((seat.index() as u64 + 1) * 0x9e37),
            ))
        }
    }

    #[test]
    fn a_mirrored_pair_of_identical_deterministic_players_is_exactly_level() {
        // Both players decide from the view alone, with the same seed in
        // either seat: the pair must be exactly zero.
        let same =
            |seed, _| -> Box<dyn Player> { Box::new(Opponent::new(Level::from_halves(4), seed)) };
        for seed in 0..10 {
            assert_eq!(mirrored(seed, Default::default(), 3, &same, &same), 0);
        }
    }

    #[test]
    fn a_mirrored_pair_is_antisymmetric() {
        let (a, b) = (at(4), at(2));
        for seed in 0..10 {
            let ab = mirrored(seed, Default::default(), 3, &a, &b);
            let ba = mirrored(seed, Default::default(), 3, &b, &a);
            assert_eq!(ab, -ba, "seed {seed}");
        }
    }

    #[test]
    fn the_rules_of_thumb_beat_random_play_and_the_run_says_so_early() {
        let (a, b) = (at(4), at(2));
        let plan = Plan {
            batch: 30,
            looks: 4,
            negligible: 0.0,
        };
        let r = run_sequential(
            plan,
            |i| f64::from(mirrored(i as u64, Default::default(), 3, &a, &b)),
            |_| {},
        );
        match r.verdict {
            Verdict::Clear(l) => assert!(l.mean > 2.0 && l.look <= 2, "{l:?}"),
            v => panic!("{v:?}"),
        }
    }
}
