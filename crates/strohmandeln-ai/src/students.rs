//! Students with one skill knocked out (`docs/TUTOR.md` 11, M4): rung 4 whole,
//! except for one thing it is made not to do. The tests use them to check
//! that the evidence points at the skill that is missing (`tutor_tests.rs`),
//! the measurements to price each skill and to tune the learner (M4, M6).
//!
//! A knock-out removes the skill (it is never used), except `Pagat` and `Bid`,
//! which are halved: a coin decides, so a fair share of their chances is met
//! by luck. Total ones show in a few deals; partial ones, which show in tens,
//! are for M6. Each student decides from its own view only, as rung 4 does,
//! and is a pure function of the seed and the view.

use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung2::{self, play_card};
use crate::rung4;
use crate::seed::view_seed;
use crate::tutor::{is_split, shown_void, Skill};
use strohmandeln_core::agents::Player;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// What a student is made not to do.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Lesion {
    /// Rung 4 whole.
    None,
    /// `worth`: lets every trick go that it can, with the cheapest card.
    Worth,
    /// `cheap`: among the cards that give the outcome it chose, plays the
    /// dearest.
    Cheap,
    /// `off`: never plays a cover while a hand card is legal.
    Off,
    /// `run`: leads a tarock at random among its tarocks.
    Run,
    /// `pagat`: plays the Pagat or keeps it by a coin (halved, not removed),
    /// whenever it follows with a choice.
    Pagat,
    /// `void`: leads the best card of a suit he has shown he is out of, when
    /// it has one.
    Void,
    /// `behind`: leads the best card of a suit the hands share evenly, when
    /// it has one.
    Behind,
    /// `endgame`: plays rung 2 from the first position with nothing face down.
    Endgame,
    /// `bid`: takes up or passes by a coin (halved, not removed).
    Bid,
    /// `announce`: never announces.
    Announce,
    /// `bid`, one way: always passes.
    AlwaysPass,
    /// `bid`, the other way: takes up every hand.
    AlwaysTake,
}

impl Lesion {
    /// One knock-out per skill, in ladder order.
    pub const ALL: [Lesion; 10] = [
        Lesion::Worth,
        Lesion::Cheap,
        Lesion::Off,
        Lesion::Run,
        Lesion::Pagat,
        Lesion::Void,
        Lesion::Behind,
        Lesion::Endgame,
        Lesion::Bid,
        Lesion::Announce,
    ];

    /// The skill the knock-out removes.
    pub fn skill(self) -> Option<Skill> {
        Some(match self {
            Lesion::None => return None,
            Lesion::Worth => Skill::Worth,
            Lesion::Cheap => Skill::Cheap,
            Lesion::Off => Skill::Off,
            Lesion::Run => Skill::Run,
            Lesion::Pagat => Skill::Pagat,
            Lesion::Void => Skill::Void,
            Lesion::Behind => Skill::Behind,
            Lesion::Endgame => Skill::Endgame,
            Lesion::Bid | Lesion::AlwaysPass | Lesion::AlwaysTake => Skill::Bid,
            Lesion::Announce => Skill::Announce,
        })
    }
}

/// Rung 4 with one skill knocked out, or several (M4's pairs): each knock-out
/// changes what the one before it chose, in order.
#[derive(Clone, Debug)]
pub struct Student {
    opp: Opponent,
    lesions: Vec<Lesion>,
    seed: u64,
}

impl Student {
    /// A student at the top rung with the opponent's `config` (its budget is
    /// the student's search); `seed` makes its draws and its coin.
    pub fn new(lesion: Lesion, seed: u64, config: Config) -> Student {
        Student::knocked_out(&[lesion], seed, config)
    }

    /// A student with several knock-outs, applied in the order given.
    pub fn knocked_out(lesions: &[Lesion], seed: u64, config: Config) -> Student {
        Student {
            opp: Opponent::new(Level::from_halves(8), seed).with_config(config),
            lesions: lesions.to_vec(),
            seed,
        }
    }

    /// The first knock-out.
    pub fn lesion(&self) -> Lesion {
        self.lesions.first().copied().unwrap_or(Lesion::None)
    }

    pub fn lesions(&self) -> &[Lesion] {
        &self.lesions
    }

    /// The student's move: rung 4's, changed by the knock-outs.
    pub fn decide(&self, view: &View) -> Action {
        let mut action = self.opp.decide(view);
        let Some(k) = Knowledge::from_view(view) else {
            return action;
        };
        for (i, &lesion) in self.lesions.iter().enumerate() {
            // the first knock-out keeps the stream it always had
            let salt = (i as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
            action = self.apply(lesion, salt, view, &k, action);
        }
        action
    }

    /// One knock-out's change to `action`.
    fn apply(
        &self,
        lesion: Lesion,
        salt: u64,
        view: &View,
        k: &Knowledge,
        action: Action,
    ) -> Action {
        let mut rng = Rng::stream(self.seed ^ 0x5757_d3e7 ^ salt, view_seed(view));
        if !k.playing {
            return match (lesion, &action) {
                (Lesion::Bid, Action::Take | Action::Pass) => {
                    if rng.below(2) == 0 {
                        Action::Take
                    } else {
                        Action::Pass
                    }
                }
                (Lesion::AlwaysPass, Action::Take | Action::Pass) => Action::Pass,
                (Lesion::AlwaysTake, Action::Take | Action::Pass) => Action::Take,
                _ => action,
            };
        }
        let Action::Play { card, .. } = action else {
            return action;
        };
        let pick = match lesion {
            Lesion::Announce => return Action::play(card),
            Lesion::Worth => self.worth(k, card),
            Lesion::Cheap => dearest_of_its_kind(k, card),
            Lesion::Off => self.without(k, card, &mut rng, |c| !k.is_cover(c)),
            Lesion::Run => random_tarock(k, card, &mut rng),
            Lesion::Pagat => coin_pagat(k, card, &mut rng),
            Lesion::Void if k.lead.is_none() => self.without(k, card, &mut rng, |c| {
                c.suit().is_some_and(|s| shown_void(k, s))
            }),
            Lesion::Behind if k.lead.is_none() => self.without(k, card, &mut rng, |c| {
                c.suit().is_some_and(|s| is_split(k, s))
            }),
            Lesion::Endgame if k.hidden_total() == 0 => {
                let mut rng = Rng::stream(self.seed ^ 0x2 ^ salt, view_seed(view));
                match rung2::decide(view, &mut rng) {
                    Action::Play { card, .. } => card,
                    other => return other,
                }
            }
            _ => card,
        };
        play_card(k, pick)
    }

    /// Lets the trick go if it can, with the cheapest card that does.
    fn worth(&self, k: &Knowledge, card: Card) -> Card {
        let Some(led) = k.lead else { return card };
        k.legal
            .iter()
            .copied()
            .filter(|&c| crate::cardset::leader_wins(led, c))
            .min_by_key(|c| (c.thirds(), c.strength()))
            .unwrap_or(card)
    }

    /// The best card by rung 4's own values among the legal cards `keep`
    /// admits; the student's own choice if it is one, or none is admitted.
    /// (The knock-outs that make the student do the one thing it should not.)
    fn without(
        &self,
        k: &Knowledge,
        card: Card,
        rng: &mut Rng,
        keep: impl Fn(Card) -> bool,
    ) -> Card {
        let admitted: Vec<Card> = k.legal.iter().copied().filter(|&c| keep(c)).collect();
        if admitted.is_empty() || keep(card) {
            return card;
        }
        let (scores, _) =
            rung4::move_scores(k, rng, self.opp.config.budget, &self.opp.config.weights);
        scores
            .iter()
            .find(|(c, _)| keep(*c))
            .map_or(admitted[0], |&(c, _)| c)
    }
}

/// The dearest card (by thirds, then strength) that gives the same outcome
/// as `card` against the card led; a lead is left as it is.
fn dearest_of_its_kind(k: &Knowledge, card: Card) -> Card {
    let Some(led) = k.lead else { return card };
    let beats = |c: Card| !crate::cardset::leader_wins(led, c);
    k.legal
        .iter()
        .copied()
        .filter(|&c| beats(c) == beats(card))
        .max_by_key(|c| (c.thirds(), c.strength()))
        .unwrap_or(card)
}

/// A tarock lead is a random tarock.
fn random_tarock(k: &Knowledge, card: Card, rng: &mut Rng) -> Card {
    if k.lead.is_some() || !card.is_tarock() {
        return card;
    }
    let tarocks: Vec<Card> = k.legal.iter().copied().filter(|c| c.is_tarock()).collect();
    tarocks[rng.below(tarocks.len() as u64) as usize]
}

/// Following with the Pagat among the legal cards: a coin says play it or not.
fn coin_pagat(k: &Knowledge, card: Card, rng: &mut Rng) -> Card {
    if k.lead.is_none() || !k.legal.contains(&Card::PAGAT) || k.legal.len() < 2 {
        return card;
    }
    if rng.below(2) == 0 {
        return Card::PAGAT;
    }
    if card != Card::PAGAT {
        return card;
    }
    let others: Vec<Card> = k
        .legal
        .iter()
        .copied()
        .filter(|&c| c != Card::PAGAT)
        .collect();
    others[rng.below(others.len() as u64) as usize]
}

impl Player for Student {
    fn choose(&mut self, view: &View) -> Option<Action> {
        if view.legal_actions.is_empty() {
            return None;
        }
        Some(self.decide(view))
    }
}
