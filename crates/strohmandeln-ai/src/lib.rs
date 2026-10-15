//! The computer opponent for Strohmandeln.
//!
//! A ladder of four rungs, each measured against the one below
//! (`docs/OPPONENT.md`). Every decision is a pure function of a seed and
//! the player's own view, so a record replays.

pub mod brief;
pub mod cardset;
pub mod explain;
pub mod hint;
pub mod know;
pub mod lessons;
pub mod measure;
pub mod opponent;
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
#[cfg(test)]
mod testutil;
pub mod tournament;
pub mod tracker;
pub mod tutor;
