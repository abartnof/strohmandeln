//! The computer opponent for Strohmandeln.
//!
//! A ladder of four rungs, each measured against the one below
//! (`docs/OPPONENT.md`). Every decision is a pure function of a seed and
//! the player's own view, so a record replays.

pub mod ask;
pub mod brief;
pub mod cardset;
pub mod explain;
#[cfg(test)]
mod golden_tests;
pub mod hint;
pub mod know;
pub mod lessons;
pub mod m5r;
pub mod m5r_rules;
#[cfg(test)]
mod m5r_tests;
pub mod measure;
pub mod opponent;
pub mod probe;
pub mod review;
pub mod rules;
pub mod rung1;
pub mod rung2;
pub mod rung3;
pub mod rung4;
pub mod sampler;
pub mod seed;
pub mod sim;
pub mod solver;
pub mod stats;
pub mod students;
pub mod style;
pub mod tell;
#[cfg(test)]
mod testutil;
pub mod tournament;
pub mod tracker;
pub mod tutor;
