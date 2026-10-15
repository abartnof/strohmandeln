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

/// Why a record does not replay. A record comes from storage, so nothing
/// in it is trusted: each of these is an error, never a panic.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "key", rename_all = "kebab-case")]
pub enum ReplayError {
    /// Not the 54 cards once each, or a hand with no tarock (RULES 2.6: the
    /// engine redeals those, so no record holds one).
    BadDeck,
    /// The doubling in force is 1 or 2 (RULES 14).
    BadMultiplier { multiplier: u32 },
    /// The ultimo setting is 1 or 2 Points (RULES 12.3).
    BadSettings { ultimo: u32 },
    /// The engine refused the action of this number.
    Refused { index: usize, refusal: Refusal },
    /// The actions end before the deal does (`replay_to_end`).
    Unfinished,
}

impl DealRecord {
    /// Whether the record can be dealt: the checks `Deal::new` would
    /// otherwise answer with a panic.
    pub fn check(&self) -> Result<(), ReplayError> {
        let mut seen = [false; Card::COUNT];
        for c in &self.deck {
            seen[c.index()] = true;
        }
        if self.deck.len() != Card::COUNT || seen.iter().any(|s| !s) {
            return Err(ReplayError::BadDeck);
        }
        if !(1..=2).contains(&self.multiplier) {
            return Err(ReplayError::BadMultiplier {
                multiplier: self.multiplier,
            });
        }
        if !(1..=2).contains(&self.settings.ultimo) {
            return Err(ReplayError::BadSettings {
                ultimo: self.settings.ultimo,
            });
        }
        if !self.start_unchecked().void_seats().is_empty() {
            return Err(ReplayError::BadDeck);
        }
        Ok(())
    }

    fn start_unchecked(&self) -> Deal {
        Deal::new(&self.deck, self.dealer, self.multiplier, self.settings)
    }

    /// The deal as dealt, before any action.
    pub fn start(&self) -> Result<Deal, ReplayError> {
        self.check()?;
        Ok(self.start_unchecked())
    }

    /// Plays the record through, as far as it goes. Before each action
    /// `before(index, deal, seat, action)` sees the deal as it stands, so a
    /// caller can take the view of any seat (`view::deal_view`). Returns the
    /// deal after the last action, finished or not.
    pub fn replay(
        &self,
        mut before: impl FnMut(usize, &Deal, Seat, &Action),
    ) -> Result<Deal, ReplayError> {
        let mut deal = self.start()?;
        let mut events = Vec::new();
        for (index, m) in self.actions.iter().enumerate() {
            before(index, &deal, m.seat, &m.action);
            deal.apply(m.seat, m.action.clone(), &mut events)
                .map_err(|refusal| ReplayError::Refused { index, refusal })?;
        }
        Ok(deal)
    }

    /// As [`replay`](DealRecord::replay), but the deal must be over: a scored
    /// deal's record, not a deal in progress.
    pub fn replay_to_end(
        &self,
        before: impl FnMut(usize, &Deal, Seat, &Action),
    ) -> Result<Deal, ReplayError> {
        let deal = self.replay(before)?;
        if deal.end().is_none() {
            return Err(ReplayError::Unfinished);
        }
        Ok(deal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Honour;
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
        assert!(
            matches!(err, ReplayError::Refused { index, .. } if index == first_play + 1),
            "{err:?}"
        );
    }

    #[test]
    fn the_sitting_counts_the_actions_of_the_deal_in_progress() {
        // The number the tutor's assisted list refers to (docs/TUTOR.md 3):
        // both seats' actions, from 0 at each deal, the same after a reload.
        let mut s = Sitting::new(8, Settings::default(), 3).unwrap();
        let mut rng = Rng::stream(8, purpose::agent(0));
        let (mut n, mut deals_seen) = (0, 0);
        while let Some(seat) = s.to_move() {
            assert_eq!(s.deal_action_count(), n);
            if n % 7 == 3 {
                let again = Sitting::replay(&s.record()).unwrap();
                assert_eq!(again.deal_action_count(), n, "after a reload");
            }
            let legal = s.legal_actions();
            let a = match legal[0] {
                Action::Extend { .. } => Action::Extend { accept: false },
                _ => legal[rng.below(legal.len() as u64) as usize].clone(),
            };
            let was = s.deals_counted();
            s.apply(seat, a).unwrap();
            n += 1;
            if s.deals_counted() > was {
                assert_eq!(s.last_deal().unwrap().actions.len(), n);
                n = 0;
                deals_seen += 1;
            }
        }
        assert!(deals_seen >= 6);
    }

    fn first() -> DealRecord {
        deals(5).remove(0).0
    }

    #[test]
    fn a_malformed_record_is_an_error_and_never_a_panic() {
        let rec = first();
        let mut short = rec.clone();
        short.deck.pop();
        assert_eq!(
            short.replay(|_, _, _, _| {}).unwrap_err(),
            ReplayError::BadDeck
        );
        let mut twice = rec.clone();
        twice.deck[1] = twice.deck[0];
        assert_eq!(
            twice.replay(|_, _, _, _| {}).unwrap_err(),
            ReplayError::BadDeck
        );
        let mut empty = rec.clone();
        empty.deck.clear();
        assert_eq!(
            empty.replay(|_, _, _, _| {}).unwrap_err(),
            ReplayError::BadDeck
        );
        for multiplier in [0, 3, u32::MAX] {
            let mut m = rec.clone();
            m.multiplier = multiplier;
            assert_eq!(
                m.replay(|_, _, _, _| {}).unwrap_err(),
                ReplayError::BadMultiplier { multiplier }
            );
        }
        for ultimo in [0, 3] {
            let mut u = rec.clone();
            u.settings.ultimo = ultimo;
            assert_eq!(
                u.replay(|_, _, _, _| {}).unwrap_err(),
                ReplayError::BadSettings { ultimo }
            );
        }
    }

    #[test]
    fn a_deck_that_leaves_a_hand_without_a_tarock_is_not_a_deal() {
        // the engine redeals these (RULES 2.6): a record never holds one
        let suit: Vec<Card> = Card::all().filter(|c| !c.is_tarock()).collect();
        let tar: Vec<Card> = Card::all().filter(|c| c.is_tarock()).collect();
        let mut deck: Vec<Option<Card>> = vec![None; 54];
        for (k, i) in [0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24]
            .iter()
            .enumerate()
        {
            deck[*i] = Some(suit[k]);
        }
        let mut rest = suit[15..].iter().chain(tar.iter());
        let deck: Vec<Card> = deck
            .into_iter()
            .map(|c| c.unwrap_or_else(|| *rest.next().unwrap()))
            .collect();
        let mut rec = first();
        rec.deck = deck;
        rec.actions.clear();
        assert_eq!(
            rec.replay(|_, _, _, _| {}).unwrap_err(),
            ReplayError::BadDeck
        );
    }

    #[test]
    fn a_record_that_stops_short_replays_but_is_not_finished() {
        let mut rec = first();
        rec.actions.truncate(10);
        assert!(rec.replay(|_, _, _, _| {}).is_ok(), "a prefix replays");
        assert_eq!(
            rec.replay_to_end(|_, _, _, _| {}).unwrap_err(),
            ReplayError::Unfinished
        );
        let full = first();
        assert!(full.replay_to_end(|_, _, _, _| {}).is_ok());
    }

    #[test]
    fn mutated_records_never_panic() {
        // Deck, action list, dealer, and the contents of an action (its seat,
        // its card, its announcements, its kind) are all changed at random;
        // each mutant replays or is refused, none panics.
        let base = first();
        let mut rng = Rng::stream(9, 1);
        let (mut replayed, mut refused) = (0, 0);
        for _ in 0..3000 {
            let mut r = base.clone();
            for _ in 0..1 + rng.below(3) {
                let at = rng.below(r.actions.len().max(1) as u64) as usize;
                match rng.below(10) {
                    0 => r.deck.swap(rng.below(54) as usize, rng.below(54) as usize),
                    1 => {
                        let at = rng.below(r.deck.len() as u64) as usize;
                        r.deck[at] = Card::from_index(rng.below(54) as u8).unwrap();
                    }
                    2 => r
                        .actions
                        .truncate(rng.below(r.actions.len() as u64 + 1) as usize),
                    3 if !r.actions.is_empty() => {
                        r.actions.remove(at);
                    }
                    4 if !r.actions.is_empty() => {
                        let b = rng.below(r.actions.len() as u64) as usize;
                        r.actions.swap(at, b);
                    }
                    5 if !r.actions.is_empty() => r.actions[at].seat = r.actions[at].seat.other(),
                    6 if !r.actions.is_empty() => {
                        let card = Card::from_index(rng.below(54) as u8).unwrap();
                        r.actions[at].action = Action::play(card);
                    }
                    7 if !r.actions.is_empty() => {
                        let honour = if rng.below(2) == 0 {
                            Honour::Trull
                        } else {
                            Honour::Kings
                        };
                        r.actions[at].action = match r.actions[at].action.clone() {
                            Action::Play { card, .. } => Action::Play {
                                card,
                                announce: vec![honour, honour],
                            },
                            _ => Action::Announce { honour },
                        };
                    }
                    8 if !r.actions.is_empty() => {
                        r.actions[at].action = match rng.below(3) {
                            0 => Action::Take,
                            1 => Action::Pass,
                            _ => Action::Extend { accept: true },
                        };
                    }
                    _ => r.dealer = r.dealer.other(),
                }
            }
            match r.replay_to_end(|_, d, seat, _| {
                let _ = bare_view(d, seat);
            }) {
                Ok(_) => replayed += 1,
                Err(_) => refused += 1,
            }
        }
        assert!(replayed > 0 && refused > 1000, "{replayed} {refused}");
    }

    #[test]
    fn a_record_survives_json() {
        let (rec, _, _) = deals(2).remove(0);
        let text = serde_json::to_string(&rec).unwrap();
        assert_eq!(serde_json::from_str::<DealRecord>(&text).unwrap(), rec);
    }
}
