use super::*;
use crate::cardset::CardSet;
use crate::testutil::{c, Scene};
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::{Action, Seat};

/// The value for the mover of playing `card`, by plain recursion.
fn naive_child(s: &Sim, card: Card) -> i32 {
    let mover = s.mover();
    let mut s2 = s.clone();
    match s2.play(card) {
        None => -naive(&s2),
        Some((winner, t)) => {
            let sign = if winner == mover { 1 } else { -1 };
            let x = if s2.is_over() {
                let u = i32::from(s2.ultimo) * ULTIMO_THIRDS;
                if s2.leader == 0 {
                    u
                } else {
                    -u
                }
            } else {
                naive(&s2)
            };
            sign * (t as i32 + x)
        }
    }
}

/// Plain minimax over plies, with nothing clever: the oracle.
fn naive(s: &Sim) -> i32 {
    s.legal()
        .iter()
        .map(|card| naive_child(s, card))
        .max()
        .expect("a move")
}

fn hands(p0: &[&str], p1: &[&str]) -> Sim {
    let n = p0.len() as u8;
    assert_eq!(p0.len(), p1.len());
    Sim::new(
        [
            CardSet::of(p0.iter().map(|s| c(s))),
            CardSet::of(p1.iter().map(|s| c(s))),
        ],
        [[None; 3]; 2],
        Default::default(),
        [0, 0],
        0,
        28 - n,
        None,
        [0, 0],
    )
}

#[test]
fn one_trick_left_is_just_the_trick() {
    let mut sv = Solver::new(10);
    // Player 0 leads the king of hearts; player 1 has no heart and no tarock.
    let s = hands(&["Kh"], &["7c"]);
    assert_eq!(sv.solve(&s, 5), Some(13 + 1));
    // Player 1 trumps with the Pagat in the last trick and wins it: the
    // trick (13 + 13) and the ultimo are his.
    let s = hands(&["Kh"], &["T1"]);
    assert_eq!(sv.solve(&s, 5), Some(-(13 + 13) - ULTIMO_THIRDS));
}

#[test]
fn two_tricks_solved_by_hand() {
    let mut sv = Solver::new(10);
    // Player 0 holds the top two hearts: both tricks are his whatever player
    // 1 plays: 13 + 10 + 4 + 1 = 28.
    let s = hands(&["Kh", "Qh"], &["Jh", "4h"]);
    assert_eq!(sv.solve(&s, 5), Some(28));
    // Player 0 leads a low heart into the king and queen: he loses both
    // tricks, 4h+Kh = 14 and 2h+Qh = 11 to player 1: -25.
    let s = hands(&["4h", "2h"], &["Kh", "Qh"]);
    assert_eq!(sv.solve(&s, 5), Some(-25));
}

#[test]
fn the_solver_agrees_with_plain_minimax_on_random_endgames() {
    let mut rng = Rng::seeded(99);
    let mut sv = Solver::new(14);
    for case in 0..300 {
        let n = 1 + (case % 4);
        let mut deck: Vec<Card> = Card::all().collect();
        rng.shuffle(&mut deck);
        let a = CardSet::of(deck[..n].iter().copied());
        let b = CardSet::of(deck[n..2 * n].iter().copied());
        let s = Sim::new(
            [a, b],
            [[None; 3]; 2],
            Default::default(),
            [0, 0],
            (case % 2) as u8,
            (28 - n) as u8,
            None,
            [0, 0],
        );
        sv.clear();
        sv.set_limit(u64::MAX);
        assert_eq!(sv.solve(&s, 30), Some(naive(&s)), "case {case}");
        let values = sv.move_values(&s, 30).unwrap();
        for (card, v) in values {
            assert_eq!(v, naive_child(&s, card), "case {case} {card:?}");
        }
    }
}

#[test]
fn packets_and_covers_are_searched_right_too() {
    // Late positions of real deals, all cards known, a few tricks to go.
    let mut checked = 0;
    for seed in 0..60u64 {
        let mut rng = Rng::seeded(seed);
        let mut deck: Vec<Card> = Card::all().collect();
        rng.shuffle(&mut deck);
        let mut scene = Scene::new(&deck);
        scene.apply(Seat::A, Action::Take).unwrap();
        while scene.deal.trick_number() < 25 {
            let Some(seat) = scene.deal.to_move() else {
                break;
            };
            let legal = scene.deal.legal_plays(seat);
            let (card, _) = legal[rng.below(legal.len() as u64) as usize];
            scene.apply(seat, Action::play(card)).unwrap();
        }
        if scene.deal.trick_number() != 25 || scene.deal.led_card().is_some() {
            continue;
        }
        let seat = scene.deal.to_move().unwrap();
        let s = Sim::from_deal(&scene.deal, seat);
        let mut sv = Solver::new(12);
        assert_eq!(sv.solve(&s, 30), Some(naive(&s)), "seed {seed}");
        checked += 1;
    }
    assert!(checked >= 10, "{checked}");
}

#[test]
fn the_budget_stops_the_search_and_says_so() {
    let mut rng = Rng::seeded(4);
    let mut deck: Vec<Card> = Card::all().collect();
    rng.shuffle(&mut deck);
    let mut scene = Scene::new(&deck);
    scene.apply(Seat::A, Action::Take).unwrap();
    let s = Sim::from_deal(&scene.deal, Seat::A);
    let mut sv = Solver::new(12);
    sv.set_limit(200);
    assert_eq!(sv.solve(&s, 30), None);
    assert!(sv.exhausted);
    // a fresh budget works again
    sv.set_limit(u64::MAX);
    assert!(sv.solve(&s, 1).is_some());
}

#[test]
fn a_search_is_deterministic() {
    let mut rng = Rng::seeded(12);
    let mut deck: Vec<Card> = Card::all().collect();
    rng.shuffle(&mut deck);
    let mut scene = Scene::new(&deck);
    scene.apply(Seat::A, Action::Take).unwrap();
    let s = Sim::from_deal(&scene.deal, Seat::A);
    let run = || {
        let mut sv = Solver::new(14);
        sv.set_limit(50_000);
        (sv.solve(&s, 2), sv.nodes)
    };
    assert_eq!(run(), run());
}
