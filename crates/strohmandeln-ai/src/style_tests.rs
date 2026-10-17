use super::*;
use crate::cardset::CardSet;
use crate::opponent::{Level, Opponent};
use crate::rung2::{best_card, bid_with, BidExtra, BidRule, BID_RULE};
use crate::testutil::{c, know};
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;

const NO: [&str; 3] = ["", "", ""];

fn set(codes: &[&str]) -> CardSet {
    CardSet::of(codes.iter().map(|s| c(s)))
}

fn cfg(style: Style) -> Config {
    Config::for_style(style, 0)
}

fn pick(style: Style, k: &crate::know::Knowledge) -> Card {
    best_card(k, &mut Rng::seeded(1), &cfg(style).weights).card
}

fn bids(style: Style, hand: &[&str]) -> bool {
    bid_with(set(hand), cfg(style).bid).0
}

#[test]
fn ids_round_trip_and_unknown_ones_are_refused() {
    for s in Style::ALL {
        assert_eq!(Style::parse(s.id()), Some(s));
    }
    assert_eq!(Style::parse("Normal"), None);
    assert_eq!(Style::parse(""), None);
    assert_eq!(Style::parse("bermann2"), None);
    assert_eq!(Style::default(), Style::Normal);
}

#[test]
fn random_is_drawn_from_the_pool_by_a_hash_of_the_seed() {
    let mut seen = std::collections::HashSet::new();
    for seed in 0..300 {
        let d = Style::drawn(seed);
        assert!(Style::POOL.contains(&d), "{d:?}");
        assert_eq!(d, Style::drawn(seed), "a function of the seed");
        assert_eq!(Style::Random.resolve(seed), d);
        assert_eq!(Style::Ulmann.resolve(seed), Style::Ulmann);
        seen.insert(d);
    }
    assert_eq!(seen.len(), Style::POOL.len(), "every member comes up");
    assert!(!Style::POOL.contains(&Style::Normal));
    assert!(!Style::POOL.contains(&Style::Random));
}

#[test]
fn normal_is_the_default_config_exactly() {
    assert_eq!(cfg(Style::Normal), Config::default());
    assert_eq!(cfg(Style::Normal).style, Style::Normal);
    // Random with a seed is a resolved style.
    let r = Config::for_style(Style::Random, 5);
    assert_eq!(r.style, Style::drawn(5));
}

#[test]
fn every_other_style_differs_from_normal_somewhere() {
    for s in Style::POOL {
        let c = cfg(s);
        assert_eq!(c.style, s);
        assert!(c != Config::default(), "{s:?}");
    }
}

#[test]
fn bermann_bids_on_sixty_thirds_with_queens_as_figures_and_any_tarocks() {
    // SK, Mond, two kings = 52; two lone queens: 20 more.
    let hand = [
        "SK", "T21", "T2", "Kh", "Kd", "Qc", "Qs", "1h", "2h", "3h", "4h", "1d", "2d", "3d", "4d",
    ];
    assert!(bids(Style::Bermann, &hand));
    assert!(!bids(Style::Normal, &hand), "Normal needs 5 tarocks");
    // Two tarocks and 52 thirds: under his 60.
    let thin = [
        "SK", "T21", "Kh", "Kd", "1h", "2h", "3h", "4h", "1d", "2d", "3d", "4d", "7c", "8c", "9c",
    ];
    assert!(!bids(Style::Bermann, &thin));
}

#[test]
fn ms_needs_six_tarocks_and_counts_the_pagat_always() {
    // Sküs, Kh, Kd, Kc, Pagat (13 x 5 = 65) with 4 tarocks: the Pagat counts.
    let hand = [
        "SK", "T1", "T5", "T6", "Kh", "Kd", "Kc", "1h", "2h", "3h", "1d", "2d", "3d", "7c", "8c",
    ];
    assert!(!bids(Style::Ms, &hand), "four tarocks are under six");
    let six = [
        "SK", "T1", "T5", "T6", "T7", "T8", "Kh", "Kd", "Kc", "1h", "2h", "1d", "2d", "7c", "8c",
    ];
    assert!(bids(Style::Ms, &six));
    // Six tarocks guard the Pagat for Normal too, so the figure is the same;
    // the rule's own fields are what differ.
    let rule = cfg(Style::Ms).bid;
    assert_eq!(rule.pagat_guard, 0);
    assert_eq!(rule.min_tarocks, 6);
    assert_eq!(rule.sure_thirds, 60);
}

#[test]
fn ulmann_takes_up_on_four_honours_at_once() {
    // Sküs, Mond, Kh, Kd: four honours, one more tarock only (T2).
    let four = [
        "SK", "T21", "T2", "Kh", "Kd", "1h", "2h", "3h", "1d", "2d", "3d", "7c", "8c", "9c", "10c",
    ];
    assert!(bids(Style::Ulmann, &four));
    assert!(bids(Style::Ulmann, &four));
    assert!(!bids(Style::Normal, &four));
    // Three honours and no queen: not enough (at most 39 thirds). U's
    // "especially as dealer" clause is not modelled: it could change nothing,
    // since Normal's thresholds already refuse such a hand.
    let three = [
        "SK", "T21", "Kh", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "1h", "2h", "1d", "7c", "8c",
    ];
    assert!(!bids(Style::Ulmann, &three));
    // The Pagat is an honour only with six tarocks (U 78): with three
    // tarocks SK, Mond, Pagat and a king are three honours, not four.
    let short = [
        "SK", "T21", "T1", "Kh", "1h", "2h", "3h", "1d", "2d", "3d", "7c", "8c", "9c", "10c", "7s",
    ];
    assert!(!bids(Style::Ulmann, &short));
    let long = [
        "SK", "T21", "T1", "Kh", "T2", "T3", "T4", "1h", "2h", "1d", "2d", "3d", "7c", "8c", "10c",
    ];
    assert!(bids(Style::Ulmann, &long), "six tarocks guard the Pagat");
}

#[test]
fn werner_wants_three_honours_six_tarocks_and_a_guarded_queen() {
    let good = [
        "SK", "T21", "Kh", "Qh", "T2", "T3", "T4", "T5", "T6", "1d", "2d", "3d", "7c", "8c", "7s",
    ];
    assert!(bids(Style::Werner, &good));
    // No queen with a guard: pass.
    let no_queen = [
        "SK", "T21", "Kh", "1h", "T2", "T3", "T4", "T5", "T6", "1d", "2d", "3d", "7c", "8c", "7s",
    ];
    assert!(!bids(Style::Werner, &no_queen));
    // Five tarocks: pass.
    let few = [
        "SK", "T21", "Kh", "Qh", "T2", "T3", "T4", "1d", "2d", "3d", "4d", "7c", "8c", "7s", "8s",
    ];
    assert!(!bids(Style::Werner, &few));
}

#[test]
fn temperaments_bid_by_their_own_thresholds() {
    let r = |s| cfg(s).bid;
    assert_eq!(
        (r(Style::Bold).sure_thirds, r(Style::Bold).min_tarocks),
        (36, 4)
    );
    assert_eq!(
        (
            r(Style::Cautious).sure_thirds,
            r(Style::Cautious).min_tarocks
        ),
        (60, 6)
    );
    assert_eq!(
        (r(Style::Hunter).sure_thirds, r(Style::Hunter).min_tarocks),
        (36, 7)
    );
    for s in [Style::Bold, Style::Cautious, Style::Hunter] {
        assert_eq!(r(s).extra, BidExtra::None);
    }
    let _: BidRule = BID_RULE;
}

#[test]
fn bermann_plays_a_lying_figure_even_when_two_low_cards_show() {
    // My Dame lies on a packet in hearts; he shows 2h and 3h. Every other card
    // of mine stands under a higher card of his. Normal keeps the Dame (his
    // two low cards); Bermann plays it at once (Be 148).
    let hand = ["7c", "8c", "7s", "8s"];
    let k = know(&hand, ["Qh", "", ""], ["2h", "Kc", "Ks"], &["3h"], None);
    assert_ne!(pick(Style::Normal, &k), c("Qh"));
    assert_eq!(pick(Style::Bermann, &k), c("Qh"));
}

#[test]
fn guard_mond_keeps_a_small_tarock_for_those_who_hold_the_mond_without_the_skus() {
    // Mond and one small tarock: Normal leads the small one (cheapest);
    // Bermann and MS keep it.
    let hand = ["T21", "T2", "T15", "T16", "T17", "T18", "7c", "8c", "9c"];
    let k = know(&hand, NO, NO, &[], None);
    assert_eq!(pick(Style::Normal, &k), c("T2"));
    for s in [Style::Bermann, Style::Ms] {
        let p = pick(s, &k);
        assert_ne!(p, c("T2"), "{s:?} keeps the small tarock");
        assert_ne!(p, c("T21"));
    }
    // With the Sküs beside the Mond there is nothing to guard.
    let hand = [
        "SK", "T21", "T2", "T15", "T16", "T17", "T18", "7c", "8c", "9c",
    ];
    let k = know(&hand, NO, NO, &[], None);
    assert_eq!(pick(Style::Bermann, &k), pick(Style::Normal, &k));
}

#[test]
fn cash_mond_plays_an_unescorted_mond_on_a_rich_suit_trick() {
    // He leads the king of hearts; I am void in hearts and hold the Mond and
    // a small tarock. Normal wins with the cheapest tarock; Bermann and
    // Ulmann cash the Mond.
    let hand = ["T21", "T5", "7c", "8c"];
    let k = know(&hand, NO, NO, &[], Some("Kh"));
    assert_eq!(pick(Style::Normal, &k), c("T5"));
    for s in [Style::Bermann, Style::Ulmann] {
        assert_eq!(pick(s, &k), c("T21"), "{s:?}");
    }
    // Beside the Sküs it is escorted: no cash.
    let hand = ["SK", "T21", "T5", "7c", "8c"];
    let k = know(&hand, NO, NO, &[], Some("Kh"));
    assert_ne!(pick(Style::Bermann, &k), c("T21"));
}

#[test]
fn spare_lead_opens_a_wholly_split_suit_with_the_highest_spare_card() {
    // Be 150-151: I hold the king, cavall, two and four of hearts; his queen
    // and jack are out. Normal does not open a split suit; Bermann leads 2h.
    let hand = ["Kh", "Ch", "2h", "4h", "7c", "8c", "7s"];
    let k = know(&hand, NO, NO, &[], None);
    assert_ne!(
        pick(Style::Normal, &k).suit(),
        Some(strohmandeln_core::cards::Suit::Hearts)
    );
    assert_eq!(pick(Style::Bermann, &k), c("2h"));
}

#[test]
fn ulmann_and_ms_draw_trumps_only_from_ten_and_twelve() {
    // Eight tarocks: Normal draws trumps (rung 2, from 7).
    let hand = ["T9", "T8", "T7", "T6", "T5", "T4", "T3", "T2", "9c", "8c"];
    let k = know(&hand, NO, NO, &[], None);
    let draws = |s: Style| {
        crate::rung2::score_cards(&k, &cfg(s).weights)
            .iter()
            .any(|x| x.reason == "draw-trumps")
    };
    assert!(draws(Style::Normal));
    assert!(!draws(Style::Ulmann), "10 or more");
    assert!(!draws(Style::Ms), "12 or more");
    assert_eq!(cfg(Style::Ulmann).weights.draw_from, Some(10));
    assert_eq!(cfg(Style::Ms).weights.draw_from, Some(12));
}

#[test]
fn the_pagat_hunter_holds_the_pagat_from_six_and_the_cautious_cashes_it_sooner() {
    assert_eq!(cfg(Style::Hunter).weights.strong_tarocks, 6);
    assert_eq!(cfg(Style::Hunter).weights.draw_from, Some(8));
    assert_eq!(cfg(Style::Cautious).weights.strong_tarocks, 10);
    // Six tarocks with the Pagat, a rich trick led: Normal cashes the Pagat;
    // the hunter holds it.
    let hand = ["T1", "T9", "T10", "T11", "T12", "T13", "9c"];
    let k = know(&hand, NO, NO, &[], Some("Kh"));
    assert_eq!(pick(Style::Normal, &k), c("T1"));
    assert_ne!(pick(Style::Hunter, &k), c("T1"));
}

#[test]
fn temperaments_scale_their_weights() {
    let n = Weights::default();
    let w = |s| cfg(s).weights;
    assert_eq!(w(Style::Bold).split_suit, n.split_suit * 0.5);
    assert_eq!(w(Style::Bold).exposed_figure, n.exposed_figure * 1.5);
    assert_eq!(w(Style::Cautious).split_suit, n.split_suit * 1.5);
    assert_eq!(w(Style::Cautious).keep_packet, n.keep_packet * 1.5);
}

#[test]
fn a_styled_opponent_is_a_function_of_the_view_and_decides_legally() {
    use crate::testutil::walk;
    use strohmandeln_core::types::Action;
    use strohmandeln_core::view::view;
    for s in Style::POOL {
        let o = Opponent::new(Level::from_halves(4), 3).with_config(cfg(s));
        for seed in 0..3 {
            walk(seed, |st, seat| {
                let v = view(st, seat);
                let a = o.decide(&v);
                assert_eq!(a, o.decide(&v));
                match &a {
                    Action::Play { card, .. } => {
                        assert!(v.legal_actions.contains(&Action::play(*card)))
                    }
                    Action::Extend { accept: false } => {}
                    other => assert!(v.legal_actions.contains(other), "{s:?} {other:?}"),
                }
            });
        }
    }
}

#[test]
fn a_style_carries_its_calibrated_rung_four_margin() {
    assert_eq!(Style::Normal.rung4_margin(), 0.0);
    for s in Style::POOL {
        assert_eq!(cfg(s).bid_margin, Some(s.rung4_margin()), "{s:?}");
    }
    assert_eq!(cfg(Style::Normal).bid_margin, Some(0.0));
}

#[test]
fn the_margins_are_stage_0_figures_relative_to_normals_and_ordered_by_the_rules_shares() {
    // Stage 0 (docs/STYLES.md 9): calibrated margins, less Normal's 0.100 so
    // that Normal's own margin stays 0.
    for (s, m) in [
        (Style::Bermann, 0.362),
        (Style::Ulmann, -0.075),
        (Style::Ms, 0.450),
        (Style::Werner, 0.188),
        (Style::Bold, -0.575),
        (Style::Cautious, 0.450),
        (Style::Hunter, 0.0),
    ] {
        assert!((s.rung4_margin() - m).abs() < 1e-9, "{s:?}");
    }
    // A rule that takes up on fewer hands has the higher margin.
    let cal = crate::probe::Calibration::sample(0..300, 1);
    for a in Style::POOL {
        for b in Style::POOL {
            let (sa, sb) = (cal.share(cfg(a).bid), cal.share(cfg(b).bid));
            if sa + 0.02 < sb {
                assert!(a.rung4_margin() >= b.rung4_margin(), "{a:?} {b:?}");
            }
        }
    }
}
