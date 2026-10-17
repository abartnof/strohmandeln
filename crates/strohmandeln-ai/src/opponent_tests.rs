use super::*;
use crate::testutil::{c, know, walk};
use strohmandeln_core::view::view;

const NO: [&str; 3] = ["", "", ""];

fn small() -> Config {
    let mut config = Config::default();
    config.budget.nodes = 2_000;
    config.budget.worlds = 3;
    config.budget.bid_worlds = 4;
    config
}

#[test]
fn the_dial_is_one_two_three_three_and_a_half_four() {
    let dial: Vec<f64> = DIAL
        .iter()
        .map(|&h| Level::from_halves(h).as_f64())
        .collect();
    assert_eq!(dial, [1.0, 2.0, 3.0, 3.5, 4.0]);
    assert_eq!(Level::from_halves(2).rung(), 2);
    assert_eq!(Level::from_halves(4).rung(), 2);
    assert_eq!(Level::from_halves(6).rung(), 3);
    assert_eq!(Level::from_halves(7).rung(), 4);
    assert_eq!(Level::from_halves(8).rung(), 4);
    assert_eq!(Level::from_halves(7).rungs(), (4, None), "no slips");
}

#[test]
fn a_half_snaps_to_the_nearest_dial_position_ties_down() {
    for (x, want) in [
        (1.0, 1.0),
        (1.5, 1.0),
        (1.6, 2.0),
        (2.4, 2.0),
        (2.5, 2.0),
        (2.6, 3.0),
        (3.0, 3.0),
        (3.25, 3.0),
        (3.26, 3.5),
        (3.5, 3.5),
        (3.75, 3.5),
        (3.76, 4.0),
        (4.0, 4.0),
        (9.0, 4.0),
        (0.0, 1.0),
        (-3.0, 1.0),
        (f64::NAN, 4.0),
        (f64::INFINITY, 4.0),
        (f64::NEG_INFINITY, 1.0),
    ] {
        assert_eq!(Level::from_f64(x).as_f64(), want, "{x}");
    }
    // The halves of the old dial: 1.5 and 2.5 lose their slips and snap down.
    assert_eq!(Level::from_halves(3).halves(), 2);
    assert_eq!(Level::from_halves(5).halves(), 4);
}

#[test]
fn level_one_keeps_burden_only_and_bids_on_sixty_thirds_with_any_tarocks() {
    let one = Config::default().at_level_one();
    let w = one.weights;
    assert_eq!(w.burden, 3.0);
    for zero in [
        w.cover,
        w.exposed_figure,
        w.split_suit,
        w.keep_packet,
        w.hold_pagat,
        w.cash_king,
        w.draw_trumps,
        w.solid_run,
        w.keep_tarocks,
        w.high_trump_lead,
        w.coverless_suit,
        w.guard_mond,
        w.cash_mond,
        w.spare_lead,
    ] {
        assert_eq!(zero, 0.0);
    }
    assert_eq!((one.bid.sure_thirds, one.bid.min_tarocks), (60, 0));
    // A style's bid, or one a measurement set, is kept.
    let styled = Config::for_style(Style::Bold, 0).at_level_one();
    assert_eq!(
        (styled.bid.sure_thirds, styled.bid.min_tarocks),
        (36, 4),
        "at level 1 a style shows in the bid"
    );
    assert_eq!(styled.weights.split_suit, 0.0);
    // Level 2 is Normal's config as it stands.
    let two = Opponent::new(Level::from_halves(4), 1);
    assert_eq!(two.config, Config::default());
}

#[test]
fn level_one_takes_a_rich_trick_and_pays_with_the_cheapest() {
    // The `worth` and `cheap` maxims (research/16 s2): the king takes the
    // queen, and a poor trick is lost with the cheapest card.
    let o = Opponent::new(Level::from_halves(2), 1);
    let take = |hand: &[&str], lead: &str| {
        let k = know(hand, NO, NO, &[], Some(lead));
        crate::rung2::best_card(&k, &mut Rng::seeded(1), &o.config.at_level_one().weights).card
    };
    assert_eq!(take(&["Kh", "4h", "9c"], "Qh"), c("Kh"));
    assert_eq!(take(&["Kh", "4h", "9c"], "2h"), c("4h"));
}

#[test]
fn level_one_leads_the_cheapest_card_and_plays_no_cover_off() {
    let w = Config::default().at_level_one().weights;
    // A cavall and pips: level 1 leads a pip, and does not prefer a cover to
    // a hand card.
    let k = know(&["Ch", "7c", "8c"], NO, NO, &[], None);
    let p = crate::rung2::best_card(&k, &mut Rng::seeded(1), &w).card;
    assert!(p == c("7c") || p == c("8c"), "{p:?}");
    let k = know(&["4d", "3d", "2d"], ["4h", "", ""], NO, &[], Some("9s"));
    let scored = crate::rung2::score_cards(&k, &w);
    let cover = scored.iter().find(|s| s.card == c("4h")).unwrap().score;
    let hand = scored.iter().find(|s| s.card == c("4d")).unwrap().score;
    assert_eq!(cover, hand, "no `cover` weight");
}

#[test]
fn level_one_is_readable_not_random() {
    // The card chosen is the same under five seeds in at least 95% of 1 000
    // views with a choice: the seed breaks ties only (`docs/STYLES.md` 1.1).
    let mut views = Vec::new();
    'all: for seed in 0..40 {
        let mut found = Vec::new();
        walk(seed, |s, seat| {
            let v = view(s, seat);
            if v.legal_actions.len() > 1 {
                found.push(v);
            }
        });
        for v in found.into_iter().step_by(3) {
            views.push(v);
            if views.len() == 1000 {
                break 'all;
            }
        }
    }
    assert_eq!(views.len(), 1000);
    let same = views
        .iter()
        .filter(|v| {
            let first = Opponent::new(Level::from_halves(2), 1).decide(v);
            (2..=5).all(|s| Opponent::new(Level::from_halves(2), s).decide(v) == first)
        })
        .count();
    assert!(same >= 950, "{same} of 1000 views decided alike");
}

#[test]
fn level_two_and_one_differ_in_play_and_every_level_decides_legally() {
    let mut differ = 0;
    let mut n = 0;
    for halves in [2u8, 4, 6, 7, 8] {
        let o = Opponent::new(Level::from_halves(halves), 3).with_config(small());
        let one = Opponent::new(Level::from_halves(2), 3).with_config(small());
        for seed in 0..2 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let a = o.decide(&v);
                assert_eq!(a, o.decide(&v), "asked twice, level {halves}");
                match &a {
                    Action::Play { card, .. } => {
                        assert!(v.legal_actions.contains(&Action::play(*card)), "{halves}")
                    }
                    Action::Extend { accept: false } => {}
                    other => assert!(v.legal_actions.contains(other), "{other:?}"),
                }
                if halves == 4 {
                    n += 1;
                    differ += usize::from(a != one.decide(&v));
                }
            });
        }
    }
    assert!(
        differ * 100 > n * 2,
        "level 1 and 2 differ in {differ} of {n}"
    );
}

#[test]
fn level_three_and_a_half_searches_three_worlds_and_eight_thousand_nodes() {
    let half = Opponent::new(Level::from_halves(7), 1);
    let b = half.budget();
    assert_eq!((b.worlds, b.nodes), (3, 8_000));
    assert_eq!(b.bid_worlds, rung4::Budget::default().bid_worlds);
    let four = Opponent::new(Level::from_halves(8), 1).budget();
    assert_eq!(four, rung4::Budget::default());
    // A smaller budget a caller sets (the browser's) still binds.
    let mut tight = Config::default();
    tight.budget.worlds = 2;
    tight.budget.nodes = 1_000;
    let b = half.with_config(tight).budget();
    assert_eq!((b.worlds, b.nodes), (2, 1_000));
}

#[test]
fn the_opponent_decides_from_its_view_alone_at_every_level() {
    // Twin deals that differ only in the cards the seat cannot see (the
    // other hand, every face-down packet card) give it the same view; the
    // answer must be the same at every level, bids included.
    let mut twins = 0;
    for halves in DIAL {
        let o = Opponent::new(Level::from_halves(halves), 5).with_config(small());
        twins += crate::testutil::blind_check(&|v| o.decide(v), 6);
    }
    assert!(twins > 300, "{twins}");
}

#[test]
fn a_decision_is_a_function_of_the_seed_and_the_view() {
    for halves in [2u8, 4, 6] {
        for seed in 0..4 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let a = Opponent::new(Level::from_halves(halves), 7);
                let b = Opponent::new(Level::from_halves(halves), 7);
                assert_eq!(a.decide(&v), a.decide(&v), "asked twice");
                assert_eq!(a.decide(&v), b.decide(&v));
            });
        }
    }
}

#[test]
fn it_plays_a_sitting_to_its_end_at_every_level() {
    use strohmandeln_core::agents::play_out;
    use strohmandeln_core::session::Sitting;
    for halves in DIAL {
        let mut s = Sitting::new(u64::from(halves), Default::default(), 3).unwrap();
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(Opponent::new(Level::from_halves(halves), 1).with_config(small())),
            Box::new(Opponent::new(Level::from_halves(halves), 2).with_config(small())),
        ];
        play_out(&mut s, &mut players, 100_000).unwrap();
        assert!(s.is_over());
    }
}

#[test]
fn a_styled_opponent_carries_its_style_and_random_is_resolved_by_the_seed() {
    let o = Opponent::styled(Level::from_halves(4), Style::Bermann, 9);
    assert_eq!(o.config.style, Style::Bermann);
    let r = Opponent::styled(Level::from_halves(4), Style::Random, 9);
    assert_eq!(r.config.style, Style::drawn(9));
    assert_eq!(
        Opponent::styled(Level::from_halves(4), Style::Normal, 9).config,
        Config::default()
    );
}
