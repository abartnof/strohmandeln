//! Rung 3, "Counter": remembers every card seen, tracks the suits each
//! player is void in, and draws worlds for the cards it cannot see
//! (`sampler.rs`), its own unturned packet cards included.
//!
//! **Leading** weighs each card by the sampled trick: in each of `WORLDS`
//! worlds the opponent answers as the rollout policy does, and the card is
//! scored by what the trick gives or takes, averaged. That prices what rung 2
//! can only guess: the chance a king is trumped or topped, with the voids he
//! has shown and the cards already gone counted in.
//! **Following** knows the trick exactly; what counting adds is the
//! tarocks it can still expect (the plan to draw trumps).
//! Both keep the maxims of rung 2 that are not a guess at the trick: the
//! burden of the lead, playing packets off, the Pagat for the last trick, the
//! packet kept for a threatened ultimo, the evenly split suit (now measured
//! on the cards still to be played).

use crate::cardset::{leader_wins, Kind};
use crate::know::Knowledge;
use crate::rung2::{
    common, cover_index, play_card, spend, BidRule, Choice, Tally, Weights, STRONG_TAROCKS,
};
use crate::sampler::sample;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// Worlds drawn for a lead.
pub const WORLDS: usize = 24;

/// The worlds a lead is weighed in, as search positions (none when the card
/// to play is forced, or when following: the trick is known).
pub fn lead_sims(k: &Knowledge, rng: &mut Rng, worlds: usize) -> Vec<crate::sim::Sim> {
    if k.lead.is_none() && k.legal.len() > 1 {
        (0..worlds).map(|_| sample(k, rng).sim(k)).collect()
    } else {
        Vec::new()
    }
}

/// In how many of `sims` the lead of `c` wins the trick, the opponent
/// answering as the rollout policy does.
pub fn lead_wins(sims: &[crate::sim::Sim], c: Card) -> usize {
    sims.iter()
        .filter(|sim| {
            let mut s = (*sim).clone();
            s.play(c);
            let reply = s.policy();
            s.play(reply).expect("a trick").0 == 0
        })
        .count()
}

/// Scores every legal card, best first.
pub fn score_cards(k: &Knowledge, rng: &mut Rng, worlds: usize, wt: &Weights) -> Vec<Choice> {
    let sims = lead_sims(k, rng, worlds);
    let mut out: Vec<Choice> = k
        .legal
        .iter()
        .map(|&c| {
            let t = match k.lead {
                Some(led) => follow(k, led, c, wt),
                None => lead(k, c, &sims, wt),
            };
            Choice {
                card: c,
                score: t.total,
                reason: t.best.1,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap()
            .then(a.card.cmp(&b.card))
    });
    out
}

fn follow(k: &Knowledge, led: Card, c: Card, wt: &Weights) -> Tally {
    let mut t = Tally::new("follow-cheaply");
    if !leader_wins(led, c) {
        t.add(
            f64::from(led.thirds()),
            if led.thirds() >= 7 {
                "take-rich-trick"
            } else {
                "take-trick"
            },
        );
        t.add(-wt.burden, "finesse");
        t.add(-spend(c), "follow-cheaply");
    } else {
        t.add(-f64::from(c.thirds()), "follow-cheaply");
        t.add(-0.01 * f64::from(c.strength()), "follow-cheaply");
    }
    common(k, c, &mut t, k.legal.len() > 1, wt);
    t
}

/// Tarocks I hold against the tarocks I expect him to hold.
fn trump_edge(k: &Knowledge) -> f64 {
    let unseen_t = k.unseen_tarocks() as f64;
    let share = k.opp_private() as f64 / k.unseen.len().max(1) as f64;
    let known_t = k.opp_known.iter().filter(|c| c.is_tarock()).count() as f64;
    let expected = unseen_t * share + known_t;
    k.my_tarocks() as f64 - expected
}

/// My share of the points still to be played in `suit`: mine over mine plus
/// what he may hold.
fn suit_share(k: &Knowledge, kind: Kind) -> (u32, f64) {
    let mine: u32 = k
        .playable()
        .iter()
        .filter(|&c| Kind::of(c) == kind)
        .map(|c| c.value())
        .sum();
    let theirs: u32 = k
        .opp_possible()
        .iter()
        .filter(|&c| Kind::of(c) == kind)
        .map(|c| c.value())
        .sum();
    (mine, f64::from(mine) / f64::from((mine + theirs).max(1)))
}

fn lead(k: &Knowledge, c: Card, sims: &[crate::sim::Sim], wt: &Weights) -> Tally {
    let mut t = Tally::new("lead-low");
    if !sims.is_empty() {
        let mut sum = 0.0;
        let mut wins = 0;
        for sim in sims {
            let mut s = sim.clone();
            s.play(c);
            let reply = s.policy();
            let (winner, _) = s.play(reply).expect("a trick");
            if winner == 0 {
                sum += f64::from(reply.thirds()) - wt.burden;
                wins += 1;
            } else {
                sum -= f64::from(c.thirds());
            }
        }
        let mean = sum / sims.len() as f64;
        let reason = if wins * 10 >= sims.len() * 8 {
            "lead-safe"
        } else if wins * 10 <= sims.len() * 2 {
            "lead-low"
        } else {
            "lead-sampled"
        };
        t.add(mean, reason);
    }
    match c.suit() {
        None => {
            t.add(-spend(c), "keep-tarocks");
            let edge = trump_edge(k);
            if edge >= 2.0 && k.my_tarocks() >= STRONG_TAROCKS - 2 {
                t.add(wt.draw_trumps, "draw-trumps");
                let neighbour = |d: i32| {
                    let s = i32::from(c.strength()) + d;
                    (0..=21).contains(&s)
                        && k.playable()
                            .iter()
                            .any(|x| x.is_tarock() && i32::from(x.strength()) == s)
                };
                if neighbour(-1) || neighbour(1) {
                    t.add(wt.solid_run, "lead-solid-run");
                }
                t.add(-0.05 * f64::from(c.strength()), "lead-solid-run");
            } else if edge <= -1.5 {
                t.add(-wt.keep_tarocks, "keep-tarocks");
            }
        }
        Some(_) => {
            let (mine, share) = suit_share(k, Kind::of(c));
            if mine >= 3 && (0.35..=0.65).contains(&share) {
                t.add(-wt.split_suit, "dont-lead-split-suit");
            }
            if cover_index(k, c).is_some() && (c.strength() == 6 || c.strength() == 5) {
                t.add(wt.exposed_figure, "play-exposed-figure");
            }
        }
    }
    common(k, c, &mut t, k.legal.len() > 1, wt);
    t
}

/// The best-scoring card, ties broken by `rng`.
pub fn best_card(k: &Knowledge, rng: &mut Rng, worlds: usize, wt: &Weights) -> Choice {
    let scored = score_cards(k, rng, worlds, wt);
    let top = scored[0].score;
    let tied: Vec<&Choice> = scored.iter().filter(|c| top - c.score < 1e-9).collect();
    *tied[rng.below(tied.len() as u64) as usize]
}

/// A decision at rung 3. It bids as rung 2 does: counting has nothing to
/// count before the first card.
pub fn decide_with(view: &View, rng: &mut Rng, rule: BidRule, wt: &Weights) -> Action {
    if matches!(view.legal_actions.first(), Some(Action::Extend { .. })) {
        return Action::Extend { accept: false };
    }
    let k = Knowledge::from_view(view).expect("a deal is in progress");
    if !k.playing {
        return if crate::rung2::bid(&k, rule).0 {
            Action::Take
        } else {
            Action::Pass
        };
    }
    play_card(&k, best_card(&k, rng, WORLDS, wt).card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cardset::CardSet;
    use crate::rung2::BID_RULE;
    use crate::testutil::{c, know, walk};
    use strohmandeln_core::view::view;

    const NO: [&str; 3] = ["", "", ""];

    #[test]
    fn it_always_chooses_a_legal_action_and_is_a_function_of_the_view() {
        let mut moves = 0;
        for seed in 0..12 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let a = decide_with(&v, &mut Rng::seeded(seed), BID_RULE, &Weights::default());
                assert_eq!(
                    a,
                    decide_with(&v, &mut Rng::seeded(seed), BID_RULE, &Weights::default())
                );
                match &a {
                    Action::Play { card, .. } => {
                        assert!(v.legal_actions.contains(&Action::play(*card)))
                    }
                    Action::Extend { accept: false } => {}
                    other => assert!(v.legal_actions.contains(other), "{other:?}"),
                }
                moves += 1;
            });
        }
        assert!(moves > 2000);
    }

    #[test]
    fn hidden_cards_never_change_a_decision() {
        let twins = crate::testutil::blind_check(
            &|v| decide_with(v, &mut Rng::seeded(5), BID_RULE, &Weights::default()),
            25,
        );
        assert!(twins > 100, "only {twins} twins");
    }

    #[test]
    fn a_king_is_not_led_into_a_known_void_with_a_trump_in_his_hand() {
        let hand = ["Kh", "7c", "8c", "9c", "10c"];
        // He has shown himself void in hearts and holds the known T5: the
        // king will be trumped.
        let mut trumped = know(&hand, NO, NO, &["T5"], None);
        trumped.opp_void[Kind::Suit(strohmandeln_core::cards::Suit::Hearts).index()] = true;
        let scored = score_cards(&trumped, &mut Rng::seeded(1), 24, &Weights::default());
        assert_ne!(scored[0].card, c("Kh"), "{scored:?}");
        let king_trumped = scored.iter().find(|s| s.card == c("Kh")).unwrap().score;
        // The same void with no trump possible: the king is safe.
        let mut safe = know(&hand, NO, NO, &[], None);
        safe.opp_void[Kind::Suit(strohmandeln_core::cards::Suit::Hearts).index()] = true;
        safe.opp_void[Kind::Trump.index()] = true;
        let scored = score_cards(&safe, &mut Rng::seeded(1), 24, &Weights::default());
        let king_safe = scored.iter().find(|s| s.card == c("Kh")).unwrap().score;
        assert!(king_safe > king_trumped + 8.0, "{king_safe} {king_trumped}");
    }

    #[test]
    fn its_samples_include_its_own_unturned_packet_cards() {
        let k = know(&["Kh", "7c"], ["2d", "", ""], NO, &[], None);
        let w = sample(&k, &mut Rng::seeded(1));
        assert_eq!(w.packets[0][0].len(), 3, "the cards under my own cover");
        let known: CardSet = k.hand;
        for card in &w.packets[0][0] {
            assert!(!known.contains(*card));
        }
    }
}
