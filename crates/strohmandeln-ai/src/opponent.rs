//! The opponent a player chooses: a point on the skill dial and a style
//! (`docs/STYLES.md`, `docs/OPPONENT.md`).
//!
//! **The dial** is 1, 2, 3, 3.5, 4. Nothing is played at random to be weak: a
//! known tendency can be learnt and beaten, a random slip cannot. Level 1 is
//! rung 2 with the maxims `worth` and `cheap` only (`Config::at_level_one`);
//! levels 2, 3 and 4 are rungs 2, 3 and 4; level 3.5 is rung 4 with 3 worlds
//! and 8 000 nodes. Rung 1 (random) is the test baseline and not on the dial.
//! Every decision is a pure function of (seed, view): asked twice it answers
//! the same, and a record replays. The seed breaks ties only.

use crate::rung2::{BidRule, Weights, BID_RULE};
use crate::seed::view_seed;
use crate::style::Style;
use crate::{rung2, rung3, rung4};
use strohmandeln_core::agents::Player;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

pub const TOP: u8 = 4;

/// The positions of the dial, in halves: 2 is level 1, 4 is 2, 6 is 3, 7 is
/// 3.5, 8 is 4.
const DIAL: [u8; 5] = [2, 4, 6, 7, 8];

/// Level 3.5's search: 3 worlds and 8 000 nodes (`docs/STYLES.md` 1.1).
const HALF_WORLDS: usize = 3;
const HALF_NODES: u64 = 8_000;

/// A position on the dial, held in halves.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Level(u8);

impl Level {
    /// `halves` from 2 (level 1) to 8 (level 4); a position that is not on
    /// the dial snaps to the nearest one, ties down (3 is level 1, 5 is 2).
    pub fn from_halves(halves: u8) -> Level {
        assert!((2..=2 * TOP).contains(&halves), "level 1 to {TOP}");
        Level::from_f64(f64::from(halves) / 2.0)
    }

    /// The nearest dial position to `x`, ties down (1.5 is 1, 2.5 is 2, 3.25
    /// is 3), clamped to 1..=4; not a number is the top.
    pub fn from_f64(x: f64) -> Level {
        if x.is_nan() {
            return Level(2 * TOP);
        }
        let x = x.clamp(1.0, f64::from(TOP));
        let mut best = DIAL[0];
        for h in DIAL {
            if (x - f64::from(h) / 2.0).abs() < (x - f64::from(best) / 2.0).abs() {
                best = h;
            }
        }
        Level(best)
    }

    pub fn halves(self) -> u8 {
        self.0
    }

    pub fn as_f64(self) -> f64 {
        f64::from(self.0) / 2.0
    }

    /// The rung it plays at: 2 for levels 1 and 2, 3 for 3, 4 for 3.5 and 4.
    pub fn rung(self) -> u8 {
        match self.0 {
            2 | 4 => 2,
            6 => 3,
            _ => 4,
        }
    }

    /// The rung it plays at, and the rung it may slip down to: none, since
    /// the dial has no slips (kept for the hint's rung test).
    pub fn rungs(self) -> (u8, Option<u8>) {
        (self.rung(), None)
    }

    pub fn is_one(self) -> bool {
        self.0 == 2
    }

    pub fn is_three_and_a_half(self) -> bool {
        self.0 == 7
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
    /// The style the other fields were set for (resolved: never `Random`).
    pub style: Style,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            bid: rung2::BID_RULE,
            weights: Weights::default(),
            budget: rung4::Budget::default(),
            bid_margin: Some(0.0),
            style: Style::Normal,
        }
    }
}

impl Config {
    /// This tuning as level 1 plays it (`docs/STYLES.md` 1.1, E: no text gives
    /// a beginner's play). Of the maxims only `burden`, the finesse half of
    /// `worth`, is kept: a lead is the cheapest card and a tarock costs
    /// `spend`. The constants of `Weights` are unused (every weight that reads
    /// them is zero) and the style's own maxims are off. The bid takes up with
    /// at least 60 sure thirds and any tarocks, unless the config has a bid of
    /// its own (a style's, or one a measurement set).
    pub fn at_level_one(&self) -> Config {
        let w = Weights {
            burden: self.weights.burden,
            cover: 0.0,
            exposed_figure: 0.0,
            split_suit: 0.0,
            keep_packet: 0.0,
            hold_pagat: 0.0,
            cash_king: 0.0,
            draw_trumps: 0.0,
            solid_run: 0.0,
            keep_tarocks: 0.0,
            high_trump_lead: 0.0,
            coverless_suit: 0.0,
            stable_ties: true,
            ..Weights::default()
        };
        let bid = if self.bid == BID_RULE {
            BidRule {
                sure_thirds: 60,
                min_tarocks: 0,
                ..BID_RULE
            }
        } else {
            self.bid
        };
        Config {
            weights: w,
            bid,
            ..*self
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

    /// The opponent of `style` (`Random` resolved by the seed) at `level`.
    pub fn styled(level: Level, style: Style, seed: u64) -> Opponent {
        Opponent::new(level, seed).with_config(Config::for_style(style, seed))
    }

    /// The draws for a decision in `view`.
    fn draws(&self, view: &View) -> Rng {
        Rng::stream(self.seed, view_seed(view))
    }

    /// Rung 4's budget at this level: level 3.5 searches 3 worlds and 8 000
    /// nodes at most, whatever the config allows beyond.
    pub fn budget(&self) -> rung4::Budget {
        let b = self.config.budget;
        if self.level.is_three_and_a_half() {
            rung4::Budget {
                worlds: b.worlds.min(HALF_WORLDS),
                nodes: b.nodes.min(HALF_NODES),
                ..b
            }
        } else {
            b
        }
    }

    /// The decision, a pure function of the seed and the view.
    pub fn decide(&self, view: &View) -> Action {
        let mut draws = self.draws(view);
        draws.next_u64(); // spare: the draw the retired half-step slip took
        let mut rng = Rng::seeded(draws.next_u64());
        let config = if self.level.is_one() {
            self.config.at_level_one()
        } else {
            self.config
        };
        match self.level.rung() {
            3 => rung3::decide_with(view, &mut rng, config.bid, &config.weights),
            4 => rung4::decide(
                view,
                &mut rng,
                self.budget(),
                config.bid_margin,
                config.bid,
                &config.weights,
            ),
            _ => rung2::decide_with(view, &mut rng, config.bid, &config.weights),
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
#[path = "opponent_tests.rs"]
mod tests;
