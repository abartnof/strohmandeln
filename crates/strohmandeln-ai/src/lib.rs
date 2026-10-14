//! The computer opponent for Strohmandeln.
//!
//! A ladder of four rungs, each measured against the one below
//! (`measurements/README.md`). Every decision is a pure function of a seed and
//! the player's own view, so a record replays.

pub mod cardset;
pub mod know;
pub mod opponent;
pub mod rung1;
pub mod rung2;
pub mod seed;
pub mod stats;
#[cfg(test)]
mod testutil;
pub mod tournament;
