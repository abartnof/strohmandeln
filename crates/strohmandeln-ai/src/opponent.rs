//! The opponent a player chooses: a rung of the ladder, or a step between
//! two rungs (`level` in halves, 1 to 4).
//!
//! **A half step is erraticism**, as in Cassino. On each decision the opponent
//! at level L.5 slips down one rung with probability one half: level 2.5 plays
//! between rungs 2 and 3, level 3.5 between 3 and 4. A slip is one rung only:
//! Cassino measured that letting slips repeat down to the random rung costs
//! more than half a rung. The slips are drawn from the opponent's own seeded
//! stream, a hash of the view, so every decision is a pure function of
//! (seed, view): asked twice it answers the same, and a record replays.

use crate::rung2::{BidRule, Weights};
use crate::seed::view_seed;
use crate::{rung1, rung2, rung3, rung4};
use strohmandeln_core::agents::Player;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

pub const TOP: u8 = 4;

/// The dial, in halves: 2 is level 1, 3 is level 1.5, ... 8 is level 4.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Level(u8);

impl Level {
    /// `halves` from 2 (level 1) to 8 (level 4).
    pub fn from_halves(halves: u8) -> Level {
        assert!((2..=2 * TOP).contains(&halves), "level 1 to {TOP}");
        Level(halves)
    }

    /// The nearest half step to `x`, clamped to 1..=4; not a number is the top.
    pub fn from_f64(x: f64) -> Level {
        if x.is_nan() {
            return Level(2 * TOP);
        }
        let x = x.clamp(1.0, f64::from(TOP));
        Level((x * 2.0).round() as u8)
    }

    pub fn halves(self) -> u8 {
        self.0
    }

    pub fn as_f64(self) -> f64 {
        f64::from(self.0) / 2.0
    }

    /// The rung it plays at, and the rung it may slip down to.
    pub fn rungs(self) -> (u8, Option<u8>) {
        let top = self.0.div_ceil(2);
        if self.0 % 2 == 1 {
            (top, Some(top - 1))
        } else {
            (top, None)
        }
    }
}

/// Everything that tunes the rungs, so a measurement can vary it.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Config {
    pub bid: BidRule,
    /// The weights of the maxims (rungs 2 to 4).
    pub weights: Weights,
    /// Rung 4's search budget.
    pub budget: rung4::Budget,
    /// Rung 4 takes up when the sampled advantage exceeds this (Points);
    /// `None` bids by rung 2's rule.
    pub bid_margin: Option<f64>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bid: rung2::BID_RULE,
            weights: Weights::default(),
            budget: rung4::Budget::default(),
            bid_margin: Some(0.0),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Opponent {
    pub level: Level,
    pub seed: u64,
    pub config: Config,
}

impl Opponent {
    pub fn new(level: Level, seed: u64) -> Opponent {
        Opponent {
            level,
            seed,
            config: Config::default(),
        }
    }

    pub fn with_config(mut self, config: Config) -> Opponent {
        self.config = config;
        self
    }

    /// The draws for a decision in `view`.
    fn draws(&self, view: &View) -> Rng {
        Rng::stream(self.seed, view_seed(view))
    }

    /// The rung a decision in `view` is made at.
    pub fn rung_for(&self, view: &View) -> u8 {
        let (top, slip) = self.level.rungs();
        match slip {
            Some(low) => {
                let mut d = self.draws(view);
                if d.next_u64() & 1 == 1 {
                    low
                } else {
                    top
                }
            }
            None => top,
        }
    }

    /// The decision, a pure function of the seed and the view.
    pub fn decide(&self, view: &View) -> Action {
        let mut draws = self.draws(view);
        draws.next_u64(); // the slip's draw, made in `rung_for`
        let mut rng = Rng::seeded(draws.next_u64());
        match self.rung_for(view) {
            1 => rung1::decide(view, &mut rng),
            3 => rung3::decide_with(view, &mut rng, self.config.bid, &self.config.weights),
            4 => rung4::decide(
                view,
                &mut rng,
                self.config.budget,
                self.config.bid_margin,
                self.config.bid,
                &self.config.weights,
            ),
            _ => rung2::decide_with(view, &mut rng, self.config.bid, &self.config.weights),
        }
    }
}

impl Player for Opponent {
    fn choose(&mut self, view: &View) -> Option<Action> {
        if view.legal_actions.is_empty() {
            return None;
        }
        Some(self.decide(view))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    #[test]
    fn the_dial_runs_in_halves_from_one_to_four() {
        assert_eq!(Level::from_halves(2).rungs(), (1, None));
        assert_eq!(Level::from_halves(3).rungs(), (2, Some(1)));
        assert_eq!(Level::from_halves(7).rungs(), (4, Some(3)));
        assert_eq!(Level::from_halves(8).rungs(), (4, None));
        assert_eq!(Level::from_f64(2.5).halves(), 5);
        assert_eq!(Level::from_f64(2.4).halves(), 5);
        assert_eq!(Level::from_f64(9.0).halves(), 8, "clamped");
        assert_eq!(Level::from_f64(0.0).halves(), 2, "clamped");
        assert_eq!(
            Level::from_f64(f64::NAN).halves(),
            8,
            "not a number: the top"
        );
        assert_eq!(Level::from_f64(3.0).as_f64(), 3.0);
    }

    #[test]
    fn the_opponent_decides_from_its_view_alone_at_every_level() {
        // Twin deals that differ only in the cards the seat cannot see (the
        // other hand, every face-down packet card) give it the same view; the
        // answer must be the same at every level, slips and bids included.
        let mut config = Config::default();
        config.budget.nodes = 2_000;
        config.budget.worlds = 3;
        config.budget.bid_worlds = 4;
        let mut twins = 0;
        for halves in 2..=8 {
            let o = Opponent::new(Level::from_halves(halves), 5).with_config(config);
            twins += crate::testutil::blind_check(&|v| o.decide(v), 6);
        }
        assert!(twins > 300, "{twins}");
    }

    #[test]
    fn a_decision_is_a_function_of_the_seed_and_the_view() {
        for halves in 2..=6 {
            for seed in 0..4 {
                walk(seed, |s, seat| {
                    let v = view(s, seat);
                    let a = Opponent::new(Level::from_halves(halves), 7);
                    let b = Opponent::new(Level::from_halves(halves), 7);
                    assert_eq!(a.decide(&v), a.decide(&v), "asked twice");
                    assert_eq!(a.decide(&v), b.decide(&v));
                });
            }
        }
    }

    #[test]
    fn a_half_step_slips_one_rung_about_half_the_time() {
        let o = Opponent::new(Level::from_halves(5), 3);
        let (mut low, mut n) = (0, 0);
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let r = o.rung_for(&view(s, seat));
                assert!(r == 2 || r == 3);
                low += usize::from(r == 2);
                n += 1;
            });
        }
        let p = low as f64 / n as f64;
        assert!((p - 0.5).abs() < 0.04, "{p}");
        let whole = Opponent::new(Level::from_halves(6), 3);
        walk(1, |s, seat| assert_eq!(whole.rung_for(&view(s, seat)), 3));
    }

    #[test]
    fn it_plays_a_sitting_to_its_end_at_every_level() {
        use strohmandeln_core::agents::play_out;
        use strohmandeln_core::session::Sitting;
        for halves in 2..=8 {
            let mut s = Sitting::new(halves as u64, Default::default(), 3).unwrap();
            let mut players: [Box<dyn Player>; 2] = [
                Box::new(Opponent::new(Level::from_halves(halves), 1)),
                Box::new(Opponent::new(Level::from_halves(halves), 2)),
            ];
            play_out(&mut s, &mut players, 100_000).unwrap();
            assert!(s.is_over());
        }
    }
}
