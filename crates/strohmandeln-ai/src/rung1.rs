//! Rung 1, "Legal": a random legal move.

use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// Uniformly one of the legal actions; an extension is declined.
pub fn decide(view: &View, rng: &mut Rng) -> Action {
    assert!(!view.legal_actions.is_empty(), "nothing to choose");
    if matches!(view.legal_actions[0], Action::Extend { .. }) {
        return Action::Extend { accept: false };
    }
    view.legal_actions[rng.below(view.legal_actions.len() as u64) as usize].clone()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    #[test]
    fn it_chooses_a_legal_action_and_varies() {
        let mut rng = Rng::seeded(1);
        let mut bids = std::collections::HashSet::new();
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let a = decide(&v, &mut rng);
                if matches!(a, Action::Take | Action::Pass) {
                    bids.insert(format!("{a:?}"));
                }
                assert!(
                    v.legal_actions.contains(&a) || matches!(a, Action::Extend { accept: false }),
                    "{a:?}"
                );
            });
        }
        assert_eq!(bids.len(), 2, "both bids come up");
    }
}
