use super::*;
use crate::rung2::BID_RULE;
use crate::sim::Sim;
use crate::testutil::{blind_check, c, know, walk, Scene};
use strohmandeln_core::types::{BothPass, Seat, Settings, Values};
use strohmandeln_core::view::view;

const SMALL: Budget = Budget {
    worlds: 3,
    nodes: 3_000,
    bid_worlds: 6,
};

fn go(v: &View, seed: u64) -> Action {
    decide(
        v,
        &mut Rng::seeded(seed),
        SMALL,
        Some(0.0),
        BID_RULE,
        &Weights::default(),
    )
}

#[test]
fn it_always_chooses_a_legal_action_and_is_a_function_of_the_view() {
    let mut moves = 0;
    for seed in 0..6 {
        walk(seed, |s, seat| {
            let v = view(s, seat);
            let a = go(&v, seed);
            assert_eq!(a, go(&v, seed));
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
    assert!(moves > 1000);
}

#[test]
fn hidden_cards_never_change_a_decision() {
    let twins = blind_check(&|v| go(v, 5), 12);
    assert!(twins > 50, "only {twins} twins");
}

#[test]
fn with_nothing_hidden_it_plays_the_double_dummy_best_card() {
    let mut checked = 0;
    for seed in 0..600u64 {
        let mut rng = Rng::seeded(seed);
        let mut deck: Vec<Card> = Card::all().collect();
        rng.shuffle(&mut deck);
        let mut scene = Scene::new(&deck);
        scene.apply(Seat::A, Action::Take).unwrap();
        while scene.deal.trick_number() < 24 {
            let Some(seat) = scene.deal.to_move() else {
                break;
            };
            let legal = scene.deal.legal_plays(seat);
            let (card, _) = legal[rng.below(legal.len() as u64) as usize];
            scene.apply(seat, Action::play(card)).unwrap();
        }
        let Some(seat) = scene.deal.to_move() else {
            continue;
        };
        let k = Knowledge::from_view(&scene.view(seat)).unwrap();
        // Nothing face down anywhere: the one world the sampler can draw is
        // the truth.
        if k.hidden_total() > 0 || k.opp_private() == 0 || k.legal.len() < 2 {
            continue;
        }
        let budget = Budget {
            worlds: 4,
            nodes: 4_000_000,
            bid_worlds: 1,
        };
        let chosen = best_card(&k, &mut Rng::seeded(1), budget, &Weights::default());
        let sim = Sim::from_deal(&scene.deal, seat);
        let mut sv = Solver::new(14);
        let values = sv.move_values(&sim, 30).unwrap();
        let best = values.iter().map(|&(_, v)| v).max().unwrap();
        let got = values.iter().find(|&&(c, _)| c == chosen).unwrap().1;
        assert_eq!(got, best, "seed {seed}");
        checked += 1;
    }
    assert!(checked >= 5, "{checked}");
}

#[test]
fn a_bigger_budget_searches_deeper() {
    let mut s = strohmandeln_core::session::Sitting::new(3, Default::default(), 3).unwrap();
    let seat = s.to_move().unwrap();
    s.apply(seat, Action::Take).unwrap();
    let seat = s.to_move().unwrap();
    let k = Knowledge::from_view(&view(&s, seat)).unwrap();
    let depth = |nodes| {
        let b = Budget {
            worlds: 4,
            nodes,
            bid_worlds: 1,
        };
        move_scores(&k, &mut Rng::seeded(1), b, &Weights::default()).1
    };
    assert!(depth(40_000) > depth(2_000));
}

fn bidding(hand: &[&str]) -> Knowledge {
    let mut k = know(hand, ["", "", ""], ["", "", ""], &[], None);
    k.playing = false;
    k.opp_hand_size = 15;
    k.my_hidden = [4; 3];
    k.opp_hidden = [4; 3];
    k.forehand = k.me;
    k
}

#[test]
fn the_sampled_bid_takes_a_dream_hand_and_passes_a_poor_one() {
    let dream = bidding(&[
        "SK", "T21", "T1", "T20", "T19", "T18", "T17", "T16", "Kh", "Kd", "Kc", "Ks", "Qh", "Qd",
        "Qc",
    ]);
    assert!(bid_advantage(&dream, &mut Rng::seeded(1), 40) > 0.0);
    let poor = bidding(&[
        "T2", "T3", "T4", "T5", "T6", "Jh", "Jd", "7c", "8c", "9c", "10c", "7s", "8s", "9s", "10s",
    ]);
    assert!(bid_advantage(&poor, &mut Rng::seeded(1), 40) < 0.0);
    let _ = c("T1");
}

#[test]
fn the_bid_estimate_follows_the_sitting_values_and_the_doubling() {
    let hand = [
        "SK", "T21", "T1", "T20", "T19", "T18", "T17", "T16", "Kh", "Kd", "Kc", "Ks", "Qh", "Qd",
        "Qc",
    ];
    let poor = [
        "T2", "T3", "T4", "T5", "T6", "Jh", "Jd", "7c", "8c", "9c", "10c", "7s", "8s", "9s", "10s",
    ];
    let adv_of = |hand: &[&str], values: Values, both_pass: BothPass, multiplier: u32| {
        let mut k = bidding(hand);
        k.values = values;
        k.both_pass = both_pass;
        k.multiplier = multiplier;
        bid_advantage(&k, &mut Rng::seeded(3), 40)
    };
    let adv = |values, both_pass, multiplier| adv_of(&hand, values, both_pass, multiplier);
    // (At the extremes some triples agree: 3-2 and 2-1 are both 1.)
    assert_ne!(
        adv_of(&poor, Values::V234, BothPass::Play, 1),
        adv_of(&poor, Values::V124, BothPass::Play, 1)
    );
    let base = adv(Values::V234, BothPass::Play, 1);
    assert_ne!(base, adv(Values::V234, BothPass::ThrowIn, 1));
    assert!((adv(Values::V234, BothPass::Play, 2) - 2.0 * base).abs() < 1e-9);
}

#[test]
fn the_knowledge_carries_the_settings_and_multiplier_of_the_view() {
    let settings = Settings {
        values: Values::V124,
        both_pass: BothPass::ThrowIn,
        ..Settings::default()
    };
    let s = strohmandeln_core::session::Sitting::new(1, settings, 3).unwrap();
    let seat = s.to_move().unwrap();
    let k = Knowledge::from_view(&view(&s, seat)).unwrap();
    assert_eq!(
        (k.values, k.both_pass, k.multiplier),
        (Values::V124, BothPass::ThrowIn, 1)
    );
}

#[test]
fn the_bid_samples_are_the_per_world_payoffs_of_the_bid_estimate() {
    // The estimate is the doubling times the mean of the per-world payoffs,
    // the same worlds in the same order (the tutor takes their spread).
    let dream = bidding(&[
        "SK", "T21", "T1", "T20", "T19", "T18", "T17", "T16", "Kh", "Kd", "Kc", "Ks", "Qh", "Qd",
        "Qc",
    ]);
    for multiplier in [1, 2] {
        let mut k = dream.clone();
        k.multiplier = multiplier;
        let samples = bid_samples(&k, &mut Rng::seeded(4), 40);
        assert_eq!(samples.len(), 40);
        let mean = samples.iter().sum::<f64>() / 40.0;
        let adv = bid_advantage(&k, &mut Rng::seeded(4), 40);
        assert!(
            (f64::from(multiplier) * mean - adv).abs() < 1e-9,
            "{mean} {adv}"
        );
    }
}
