//! A deal record: what replays one deal without its sitting (`docs/TUTOR.md`
//! 3). The 54 cards in deal order, the dealer, the doubling in force, the
//! settings and the deal's actions in order, both seats'. The tutor replays a
//! scored deal from it to judge the human's decisions.
//!
//! Actions are numbered from 0 in the order they were made, both seats' (the
//! numbers of the tutor's assisted list refer to them).

use crate::cards::Card;
use crate::deal::Deal;
use crate::session::Move;
use crate::types::{Action, Refusal, Seat, Settings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DealRecord {
    /// The 54 cards, index 0 the first dealt.
    pub deck: Vec<Card>,
    pub dealer: Seat,
    /// The doubling in force: 1 or 2.
    pub multiplier: u32,
    pub settings: Settings,
    pub actions: Vec<Move>,
}

/// An action of a record the engine refused.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReplayError {
    pub index: usize,
    pub refusal: Refusal,
}

impl DealRecord {
    /// The deal as dealt, before any action.
    pub fn start(&self) -> Deal {
        Deal::new(&self.deck, self.dealer, self.multiplier, self.settings)
    }

    /// Plays the record through. Before each action `before(index, deal,
    /// seat, action)` sees the deal as it stands, so a caller can take the
    /// view of any seat (`view::deal_view`). Returns the finished deal.
    pub fn replay(
        &self,
        mut before: impl FnMut(usize, &Deal, Seat, &Action),
    ) -> Result<Deal, ReplayError> {
        let mut deal = self.start();
        let mut events = Vec::new();
        for (index, m) in self.actions.iter().enumerate() {
            before(index, &deal, m.seat, &m.action);
            deal.apply(m.seat, m.action.clone(), &mut events)
                .map_err(|refusal| ReplayError { index, refusal })?;
        }
        Ok(deal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::{purpose, Rng};
    use crate::session::Sitting;
    use crate::types::*;
    use crate::view::{bare_view, deal_view, DealView};

    type Played = (DealRecord, Vec<(Seat, DealView)>, Vec<Event>);

    /// Plays a sitting by random legal actions; for every deal, the record
    /// the sitting kept, the views it gave before each move and the events
    /// of the move that ended the deal.
    fn deals(seed: u64) -> Vec<Played> {
        let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
        let mut rng = Rng::stream(seed, purpose::agent(0));
        let mut out = Vec::new();
        let mut views = Vec::new();
        while let Some(seat) = s.to_move() {
            let in_deal = s.deal().is_some_and(|d| d.end().is_none());
            if in_deal {
                views.push((seat, deal_view(s.deal().unwrap(), seat)));
            }
            let legal = s.legal_actions();
            let a = match legal[0] {
                Action::Extend { .. } => Action::Extend { accept: false },
                _ => legal[rng.below(legal.len() as u64) as usize].clone(),
            };
            let ev = s.apply(seat, a).unwrap();
            let ended = ev
                .iter()
                .any(|e| matches!(e, Event::GameScored { .. } | Event::ThrownIn));
            if in_deal && ended {
                let rec = s.last_deal().expect("the deal just scored").clone();
                out.push((rec, std::mem::take(&mut views), ev));
            }
        }
        out
    }

    #[test]
    fn a_scored_deal_replays_from_its_record_with_the_same_views_and_score() {
        let mut checked = 0;
        for seed in 0..6 {
            for (rec, views, ev) in deals(seed) {
                assert_eq!(rec.deck.len(), 54);
                let mut seen = Vec::new();
                let deal = rec
                    .replay(|_, d, seat, _| seen.push((seat, deal_view(d, seat))))
                    .unwrap();
                assert_eq!(seen, views, "seed {seed}");
                match deal.end().unwrap() {
                    crate::deal::DealEnd::ThrownIn => {
                        assert!(ev.iter().any(|e| matches!(e, Event::ThrownIn)))
                    }
                    crate::deal::DealEnd::Played(score) => assert!(ev.iter().any(|e| matches!(
                        e,
                        Event::GameScored { credits, .. } if *credits == score.credits
                    ))),
                }
                checked += 1;
            }
        }
        assert!(checked >= 20, "{checked}");
    }

    #[test]
    fn a_bare_view_is_the_seats_view_of_the_deal_with_its_legal_actions() {
        let (rec, _, _) = deals(3).remove(0);
        let mut n = 0;
        rec.replay(|_, d, seat, _| {
            let v = bare_view(d, seat);
            assert_eq!(v.seat, seat);
            assert_eq!(v.to_move, d.to_move());
            assert_eq!(v.deal, Some(deal_view(d, seat)));
            assert_eq!(v.legal_actions, d.legal_actions(), "the mover's view");
            assert!(bare_view(d, seat.other()).legal_actions.is_empty());
            n += 1;
        })
        .unwrap();
        assert!(n > 40);
    }

    #[test]
    fn a_record_that_cannot_replay_says_which_action_failed() {
        let (mut rec, _, _) = deals(1).remove(0);
        let first_play = rec
            .actions
            .iter()
            .position(|m| matches!(m.action, Action::Play { .. }))
            .unwrap();
        // The same play twice in a row: the second is not the player's turn
        // or not a card in the hand.
        let dup = rec.actions[first_play].clone();
        rec.actions.insert(first_play + 1, dup);
        let err = rec.replay(|_, _, _, _| {}).unwrap_err();
        assert_eq!(err.index, first_play + 1);
    }

    #[test]
    fn a_record_survives_json() {
        let (rec, _, _) = deals(2).remove(0);
        let text = serde_json::to_string(&rec).unwrap();
        assert_eq!(serde_json::from_str::<DealRecord>(&text).unwrap(), rec);
    }
}
