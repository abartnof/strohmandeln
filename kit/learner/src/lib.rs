//! The learner of the series, game-independent: what the games a person has
//! played say about the skills of a ladder, and the one skill to work on next
//! (`docs/TUTOR.md` section 13).
//!
//! A game supplies its [`Ladder`] (skills as indexes, with their
//! prerequisites and the rate and cost of the strongest player) and its
//! [`Params`], computes a [`Summary`] per scored deal from its own evidence,
//! and calls [`learn`]. The words are the game's: [`brief`] only puts the
//! bullets in order.
//!
//! No dependencies. Vendored from Cassino's `learner.rs` and `brief.rs`
//! (`kit/STAMP`).

pub mod beta;
mod model;

pub use model::{stale, Chance, Ladder, Params, SkillSpec, Summary, Tally};
