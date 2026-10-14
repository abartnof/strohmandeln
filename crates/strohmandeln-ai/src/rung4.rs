//! Rung 4, "Searcher": perfect-information Monte Carlo (Gams and Lustrek
//! 2003; Long, Sturtevant, Buro and Furtak 2010; `research/07` 3.3).
//!
//! Draw `worlds` worlds from rung 3's sampler (the opponent's hidden cards and
//! all twelve packets' face-down cards, its own included), solve each as a
//! game with every card known by the double-dummy solver (`solver.rs`), and
//! play the card with the best average margin in thirds. The same worlds
//! serve every candidate, so luck in the draw does not separate them.
//!
//! The search is iterative deepening with a **node budget per decision**:
//! each world gets its share, deepens trick by trick while the share lasts,
//! and falls back to the last depth completed (a deal's end is solved
//! exactly once it is near enough). Nodes, not seconds, so a record replays;
//! the budget is the knob for a slow phone.
//!
//! **Bidding** is by the sampled expectation: in each of `BID_WORLDS` worlds,
//! revealed, the deal is played out by the cheap policy, and the opponent's
//! take-up is valued against the simple game.

use crate::know::Knowledge;
use crate::rung2::{best_card as rung2_best, play_card};
use crate::rung3;
use crate::sampler::sample;
use crate::solver::Solver;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// What a decision may spend.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Budget {
    /// Worlds drawn for a play.
    pub worlds: usize,
    /// Nodes in all, split evenly over the worlds.
    pub nodes: u64,
    /// Worlds drawn for a bid.
    pub bid_worlds: usize,
}

impl Default for Budget {
    fn default() -> Budget {
        Budget {
            worlds: 10,
            nodes: 30_000,
            bid_worlds: 40,
        }
    }
}

/// The average margin in thirds of each legal card over the sampled worlds,
/// best first, and the depth (in tricks) the searches reached on average.
pub fn move_scores(k: &Knowledge, rng: &mut Rng, budget: Budget) -> (Vec<(Card, f64)>, f64) {
    let mut sums: Vec<(Card, f64)> = k.legal.iter().map(|&c| (c, 0.0)).collect();
    let share = (budget.nodes / budget.worlds.max(1) as u64).max(1);
    let mut solver = Solver::new(14);
    let mut counted = 0usize;
    let mut depth_sum = 0u32;
    for _ in 0..budget.worlds {
        let world = sample(k, rng);
        let sim = world.sim(k);
        solver.clear();
        solver.set_limit(share);
        let remaining = 28 - u32::from(sim.trick);
        let mut best: Option<(Vec<(Card, i32)>, u32)> = None;
        for depth in 1..=remaining {
            match solver.move_values(&sim, depth) {
                Some(values) => best = Some((values, depth)),
                None => break,
            }
        }
        let Some((values, depth)) = best else {
            continue;
        };
        for (c, v) in values {
            sums.iter_mut()
                .find(|(x, _)| *x == c)
                .expect("a candidate")
                .1 += f64::from(v);
        }
        depth_sum += depth;
        counted += 1;
    }
    if counted == 0 {
        // The budget was too small to finish even one trick: rung 3's choice.
        let scored = rung3::score_cards(k, rng, rung3::WORLDS);
        return (scored.iter().map(|c| (c.card, c.score)).collect(), 0.0);
    }
    let n = counted as f64;
    let mut out: Vec<(Card, f64)> = sums.into_iter().map(|(c, s)| (c, s / n)).collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    (out, f64::from(depth_sum) / n)
}

/// The card to play, ties (and near ties) settled by rung 2's maxims.
pub fn best_card(k: &Knowledge, rng: &mut Rng, budget: Budget) -> Card {
    if k.legal.len() == 1 {
        return k.legal[0];
    }
    let (scores, _) = move_scores(k, rng, budget);
    let top = scores[0].1;
    // Within a hair of the best (a third of a third), the maxims decide.
    let near: Vec<Card> = scores
        .iter()
        .filter(|(_, v)| top - v < 0.34)
        .map(|(c, _)| *c)
        .collect();
    if near.len() == 1 {
        return near[0];
    }
    let mut k2 = k.clone();
    k2.legal = near;
    rung2_best(&k2, rng).card
}

/// The expected payment (in Points) of taking up, against playing the simple
/// game, over sampled worlds. A positive difference means take up.
pub fn bid_advantage(k: &Knowledge, rng: &mut Rng, worlds: usize) -> f64 {
    let (mut win, mut lose, mut n) = (0.0, 0.0, 0.0);
    for _ in 0..worlds {
        let world = sample(k, rng);
        let thirds = world.sim(k).rollout_thirds();
        n += 1.0;
        if thirds >= 107 {
            win += 1.0;
        }
        if thirds <= 103 {
            lose += 1.0;
        }
    }
    let (p, q) = (win / n, lose / n);
    // Take up: 3 if I reach 107 thirds, 4 paid if not. Simple game: 2 each
    // way, nothing for a draw.
    let take = 3.0 * p - 4.0 * (1.0 - p);
    let simple = 2.0 * p - 2.0 * q;
    take - simple
}

/// A decision at rung 4.
pub fn decide(
    view: &View,
    rng: &mut Rng,
    budget: Budget,
    bid_margin: Option<f64>,
    rule: crate::rung2::BidRule,
) -> Action {
    if matches!(view.legal_actions.first(), Some(Action::Extend { .. })) {
        return Action::Extend { accept: false };
    }
    let k = Knowledge::from_view(view).expect("a deal is in progress");
    if !k.playing {
        let take = match bid_margin {
            Some(m) => bid_advantage(&k, rng, budget.bid_worlds) > m,
            None => crate::rung2::bid(&k, rule).0,
        };
        return if take { Action::Take } else { Action::Pass };
    }
    play_card(&k, best_card(&k, rng, budget))
}

#[cfg(test)]
#[path = "rung4_tests.rs"]
mod tests;
