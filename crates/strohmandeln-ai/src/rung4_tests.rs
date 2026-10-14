use super::*;
use crate::rung2::BID_RULE;
use crate::sim::Sim;
use crate::testutil::{blind_check, c, know, walk, Scene};
use strohmandeln_core::types::Seat;
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
