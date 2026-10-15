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
use crate::rung2::{best_card as rung2_best, play_card, Weights};
use crate::rung3;
use crate::sampler::sample;
use crate::solver::Solver;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::{Action, BothPass};
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
pub fn move_scores(
    k: &Knowledge,
    rng: &mut Rng,
    budget: Budget,
    wt: &Weights,
) -> (Vec<(Card, f64)>, f64) {
    let t = move_table(k, rng, budget, wt);
    (t.scores, t.depth)
}

/// The search's working: the averages of [`move_scores`] and, for each world
/// searched, every legal card's margin there (the tutor compares two cards
/// world by world, `hint.rs`).
pub struct MoveTable {
    pub scores: Vec<(Card, f64)>,
    pub depth: f64,
    pub per_world: Vec<Vec<(Card, i32)>>,
}

pub fn move_table(k: &Knowledge, rng: &mut Rng, budget: Budget, wt: &Weights) -> MoveTable {
    let mut per_world = Vec::new();
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
        per_world.push(values.clone());
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
        let scored = rung3::score_cards(k, rng, rung3::WORLDS, wt);
        return MoveTable {
            scores: scored.iter().map(|c| (c.card, c.score)).collect(),
            depth: 0.0,
            per_world: Vec::new(),
        };
    }
    let n = counted as f64;
    let mut out: Vec<(Card, f64)> = sums.into_iter().map(|(c, s)| (c, s / n)).collect();
    out.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap().then(a.0.cmp(&b.0)));
    MoveTable {
        scores: out,
        depth: f64::from(depth_sum) / n,
        per_world,
    }
}

/// The card to play, ties (and near ties) settled by rung 2's maxims.
pub fn best_card(k: &Knowledge, rng: &mut Rng, budget: Budget, wt: &Weights) -> Card {
    if k.legal.len() == 1 {
        return k.legal[0];
    }
    let (scores, _) = move_scores(k, rng, budget, wt);
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
    rung2_best(&k2, rng, wt).card
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
    // Take up: the win value if I reach 107 thirds, the loss value paid if
    // not. Simple game (both pass): the simple value each way, nothing for a
    // draw; under throw-in nothing at all (RULES 3.4, 9.1). All doubled by
    // the deal's multiplier (RULES 14).
    let (simple_v, win_v, lose_v) = k.values.triple();
    let take = f64::from(win_v) * p - f64::from(lose_v) * (1.0 - p);
    let simple = match k.both_pass {
        BothPass::Play => f64::from(simple_v) * (p - q),
        BothPass::ThrowIn => 0.0,
    };
    f64::from(k.multiplier) * (take - simple)
}

/// The payoff in Points, per sampled world, of taking up against the simple
/// game, not yet times the doubling: [`bid_advantage`] is the doubling times
/// the mean of these, over the same worlds in the same order. The tutor takes
/// their spread to tell a clear bid from noise (`tutor.rs`).
pub fn bid_samples(k: &Knowledge, rng: &mut Rng, worlds: usize) -> Vec<f64> {
    let (simple_v, win_v, lose_v) = k.values.triple();
    (0..worlds)
        .map(|_| {
            let thirds = sample(k, rng).sim(k).rollout_thirds();
            let take = if thirds >= 107 {
                f64::from(win_v)
            } else {
                -f64::from(lose_v)
            };
            let simple = match k.both_pass {
                BothPass::Play => {
                    f64::from(simple_v)
                        * (f64::from(u8::from(thirds >= 107)) - f64::from(u8::from(thirds <= 103)))
                }
                BothPass::ThrowIn => 0.0,
            };
            take - simple
        })
        .collect()
}

/// A decision at rung 4.
pub fn decide(
    view: &View,
    rng: &mut Rng,
    budget: Budget,
    bid_margin: Option<f64>,
    rule: crate::rung2::BidRule,
    wt: &Weights,
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
    play_card(&k, best_card(&k, rng, budget, wt))
}

#[cfg(test)]
#[path = "rung4_tests.rs"]
mod tests;
