//! Tests of M5r (`docs/HINTS.md` 5.2): the atoms, the classes, the search,
//! the sequential confirmation and the wording.

use crate::m5r::*;
use crate::measure::Precision;
use crate::testutil::{c, know};
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::types::Action;

const NO: [&str; 3] = ["", "", ""];
const HID: [&str; 3] = ["T2", "", ""];

fn holds(id: &str, k: &crate::know::Knowledge, focus: Option<Suit>) -> bool {
    (ATOMS[atom_index(id)].test)(k, focus)
}

#[test]
fn atoms_and_classes_are_well_formed() {
    assert!(ATOMS.len() <= 32, "a mask is a u32");
    let mut ids: Vec<&str> = ATOMS.iter().map(|a| a.id).collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), ATOMS.len());
    let mut cids: Vec<&str> = CLASSES.iter().map(|c| c.id).collect();
    cids.sort_unstable();
    cids.dedup();
    assert_eq!(cids.len(), CLASSES.len());
    for cl in CLASSES {
        assert!(cl.atoms.len() <= 12, "{}: at most 12 atoms", cl.id);
        for a in cl.atoms {
            assert!(ATOMS.iter().any(|x| x.id == *a), "{} lists {a}", cl.id);
        }
    }
    // at most 12 + 66 candidates per class
    for i in 0..CLASSES.len() {
        assert!(candidate_atoms(i).len() <= 78);
    }
}

#[test]
fn candidates_never_pair_two_atoms_of_one_group() {
    for i in 0..CLASSES.len() {
        for cand in candidate_atoms(i) {
            assert!((1..=2).contains(&cand.len()));
            if let [a, b] = cand[..] {
                assert_ne!(ATOMS[a].group, ATOMS[b].group, "{cand:?}");
                assert!(a < b);
            }
        }
    }
}

#[test]
fn tarock_count_atoms() {
    let mut k = know(&["T9", "T5", "T3", "Kh"], NO, HID, &[], None);
    // 22 tarocks (with the Sküs), 3 mine, none played: 19 unseen
    assert!(holds("ml5", &k, None));
    assert!(holds("ml7", &k, None));
    assert!(!holds("mt6", &k, None));
    assert!(!holds("tu8", &k, None));
    for i in 0..22u8 {
        if ![8, 4, 2].contains(&i) {
            k.unseen.remove(Card::from_index(i).unwrap());
        }
    }
    assert!(holds("tu0", &k, None));
    assert!(holds("tu8", &k, None));
}

#[test]
fn suit_atoms_need_a_focus_suit() {
    let k = know(&["Kh", "4h", "9c"], NO, ["Qh", "2h", ""], &[], Some("3h"));
    assert!(holds("master", &k, Some(Suit::Hearts)), "Kh is the top heart");
    assert!(!holds("master", &k, None));
    assert!(holds("hc1", &k, Some(Suit::Hearts)));
    assert!(holds("hc2", &k, Some(Suit::Hearts)));
    assert!(!holds("hc1", &k, Some(Suit::Clubs)));
    // seven hearts unplayed (the led 3h is played): far more than 13 thirds
    assert!(!holds("left13", &k, Some(Suit::Hearts)));
    assert!(!holds("left13", &k, None));
    let mut k = k;
    for p in 0..8u8 {
        if p != 7 {
            k.played.insert(Card::suit_card(Suit::Hearts, p));
        }
    }
    // only the king (13 thirds) is left
    assert!(holds("left13", &k, Some(Suit::Hearts)));
    assert!(holds("left26", &k, Some(Suit::Hearts)));
}

#[test]
fn the_skus_atom_is_unplayed_and_not_mine() {
    let mut k = know(&["T9", "Kh"], NO, HID, &[], None);
    assert!(holds("skus", &k, None));
    k.hand.insert(c("SK"));
    assert!(!holds("skus", &k, None));
    k.hand.remove(c("SK"));
    k.played.insert(c("SK"));
    assert!(!holds("skus", &k, None));
}

#[test]
fn led_and_phase_and_mond_atoms() {
    let mut k = know(&["Kh", "T21"], NO, HID, &[], Some("Qh"));
    assert!(holds("led10", &k, None) && !holds("led13", &k, None));
    assert!(holds("mond", &k, None));
    k.trick_number = 10;
    assert!(holds("ph2", &k, None) && !holds("ph1", &k, None));
    let lead = know(&["Kh"], NO, HID, &[], None);
    assert!(!holds("led1", &lead, None), "no card led");
}

#[test]
fn the_mond_class_fires_when_the_mond_wins_and_another_card_is_legal() {
    let k = know(&["T21", "T9"], NO, HID, &[], Some("T5"));
    let cl = &CLASSES[class_index("mond/mond")];
    assert_eq!((cl.check)(&k, &Action::play(c("T21"))), Some(true));
    assert_eq!((cl.check)(&k, &Action::play(c("T9"))), Some(false));
    // no other card legal: no choice
    let k = know(&["T21", "Kh"], NO, HID, &[], Some("T5"));
    assert_eq!((cl.check)(&k, &Action::play(c("T21"))), None);
    // a suit led and I am out of it: the Mond trumps
    let k = know(&["T21", "T9"], NO, HID, &[], Some("Kh"));
    assert_eq!((cl.check)(&k, &Action::play(c("T9"))), Some(false));
}

#[test]
fn the_trumps_classes_split_a_lead_between_tarock_and_suit() {
    let k = know(&["T9", "Kh"], NO, HID, &[], None);
    let t = &CLASSES[class_index("trumps/tarock")];
    let s = &CLASSES[class_index("trumps/suit")];
    assert_eq!((t.check)(&k, &Action::play(c("T9"))), Some(true));
    assert_eq!((s.check)(&k, &Action::play(c("T9"))), Some(false));
    assert_eq!((s.check)(&k, &Action::play(c("Kh"))), Some(true));
    let only = know(&["T9", "T8"], NO, HID, &[], None);
    assert_eq!((t.check)(&only, &Action::play(c("T9"))), None);
}

#[test]
fn the_pagat_classes_have_no_threshold_of_their_own() {
    let k = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("3d"));
    let keep = &CLASSES[class_index("pagat/keep")];
    let cash = &CLASSES[class_index("pagat/cash")];
    assert_eq!((keep.check)(&k, &Action::play(c("T9"))), Some(true));
    assert_eq!((cash.check)(&k, &Action::play(c("T1"))), Some(true));
}

#[test]
fn the_log_records_each_firing_class_with_its_atom_mask() {
    let k = know(&["T9", "Kh"], NO, HID, &[], None);
    let mut log = Log::new();
    log.record(0, &k, &Action::play(c("T9")));
    let t = class_index("trumps/tarock");
    assert_eq!(log.by_class[t].len(), 1);
    let e = log.by_class[t][0];
    assert!(e.ok);
    assert_ne!(e.mask & (1 << atom_index("ml5")), 0);
    assert_eq!(log.by_class[class_index("mond/mond")].len(), 0);
    assert_eq!(log.ndeals, 1);
}

#[test]
fn the_log_survives_its_text_form() {
    let k = know(&["T9", "Kh"], NO, HID, &[], None);
    let mut log = Log::new();
    log.record(0, &k, &Action::play(c("T9")));
    log.record(1, &k, &Action::play(c("Kh")));
    let back = Log::from_tsv(&log.to_tsv());
    assert_eq!(back.ndeals, 2);
    for (a, b) in log.by_class.iter().zip(&back.by_class) {
        assert_eq!(a, b);
    }
}

#[test]
fn appending_a_log_shifts_its_deals() {
    let k = know(&["T9", "Kh"], NO, HID, &[], None);
    let mut a = Log::new();
    a.record(0, &k, &Action::play(c("T9")));
    let mut b = Log::new();
    b.record(0, &k, &Action::play(c("T9")));
    a.append(&b);
    let t = class_index("trumps/tarock");
    assert_eq!(a.ndeals, 2);
    assert_eq!(a.by_class[t][1].deal, 1);
}

/// A log of `deals` deals for one class: deal d has one firing carrying
/// `mask(d)` and satisfied when `ok(d)`.
fn synthetic(class: &str, deals: usize, f: impl Fn(usize) -> (u32, bool)) -> Log {
    let mut log = Log::new();
    log.ndeals = deals;
    let i = class_index(class);
    for d in 0..deals {
        let (mask, ok) = f(d);
        log.by_class[i].push(Entry {
            deal: d as u32,
            mask,
            ok,
        });
    }
    log
}

#[test]
fn selection_takes_the_widest_eligible_candidate() {
    let a = 1u32 << atom_index("mt8");
    let b = 1u32 << atom_index("skus");
    // mt8 holds in 300 deals (all satisfied); skus in 100 of them; the
    // other 300 deals are half satisfied.
    let log = synthetic("trumps/tarock", 600, |d| {
        if d < 100 {
            (a | b, true)
        } else if d < 300 {
            (a, true)
        } else {
            (0, d % 2 == 0)
        }
    });
    let sel = select(&log);
    let s = sel
        .iter()
        .find(|s| CLASSES[s.class].id == "trumps/tarock")
        .unwrap();
    let best = s.best.as_ref().expect("an eligible candidate");
    assert_eq!(best.atoms, vec![atom_index("mt8")], "the wider of the two");
    assert_eq!(best.prec.fired, 300);
    assert!(best.prec.lower >= 0.80);
}

#[test]
fn selection_needs_an_effective_sample_of_60() {
    let a = 1u32 << atom_index("mt8");
    let log = synthetic("trumps/tarock", 200, |d| (if d < 40 { a } else { 0 }, d < 40));
    let sel = select(&log);
    let s = sel
        .iter()
        .find(|s| CLASSES[s.class].id == "trumps/tarock")
        .unwrap();
    assert!(s.best.is_none(), "40 firings are not enough");
}

#[test]
fn an_atom_true_at_every_firing_is_not_a_candidate() {
    let a = 1u32 << atom_index("mt8");
    let log = synthetic("trumps/tarock", 100, |_| (a, true));
    let sel = select(&log);
    let s = sel
        .iter()
        .find(|s| CLASSES[s.class].id == "trumps/tarock")
        .unwrap();
    assert!(s.best.is_none(), "it adds nothing to the idea");
}

#[test]
fn reach_is_the_share_of_deals_with_a_firing() {
    let a = 1u32 << atom_index("mt8");
    let log = synthetic("trumps/tarock", 100, |d| (if d % 4 == 0 { a } else { 0 }, true));
    let (p, reach) = evaluate(&log, class_index("trumps/tarock"), &[atom_index("mt8")]);
    assert_eq!(p.fired, 25);
    assert!((reach - 0.25).abs() < 1e-9);
}

#[test]
fn the_boundaries_are_the_plans() {
    let plan = crate::stats::Plan {
        batch: 100,
        looks: 4,
        negligible: 0.0,
    };
    for (b, p) in BOUNDARIES.iter().zip(plan.boundaries()) {
        assert!((b - p).abs() < 0.01, "{b} {p}");
    }
}

fn prec(p: f64, n: f64) -> Precision {
    Precision {
        fired: n as u32,
        satisfied: (p * n) as u32,
        p,
        n_eff: n,
        lower: crate::measure::wilson_lower(p, n),
    }
}

#[test]
fn a_look_passes_at_its_boundary_and_the_minimum_sample() {
    // z = (p - .8) / sqrt(.16 / n) = 0.5 * (p - .8) * sqrt(n) / .2 ...
    assert_eq!(step(1, &prec(1.0, 400.0)), Step::Pass, "z = 10");
    assert_eq!(step(1, &prec(0.95, 100.0)), Step::Continue, "z = 3.75 < 4.05");
    assert_eq!(step(2, &prec(0.95, 100.0)), Step::Pass, "3.75 > 2.86");
    assert_eq!(step(2, &prec(1.0, 50.0)), Step::Continue, "n below 60");
}

#[test]
fn futility_from_the_second_look_and_the_last_look_without_a_pass() {
    assert_eq!(step(1, &prec(0.5, 200.0)), Step::Continue, "no futility at look 1");
    assert_eq!(step(2, &prec(0.5, 200.0)), Step::Fail);
    assert_eq!(step(2, &prec(0.80, 200.0)), Step::Continue);
    assert_eq!(step(4, &prec(0.80, 200.0)), Step::Fail, "no pass by the last look");
    let none = Precision {
        fired: 0,
        satisfied: 0,
        p: f64::NAN,
        n_eff: 0.0,
        lower: 0.0,
    };
    assert_eq!(step(2, &none), Step::Continue);
}

#[test]
fn a_sentence_is_the_atoms_joined_to_the_action() {
    let en = sentence(&["tu0"], "pagat/keep", false);
    assert_eq!(en, "When no tarock is out, keep the Pagat.");
    let de = sentence(&["tu0", "mt8"], "pagat/keep", true);
    assert!(de.starts_with("Wenn kein Tarock draußen ist und "), "{de}");
    assert!(de.ends_with(": den Pagat behalten."), "{de}");
    let (cw, aw) = word_counts(&["tu0", "mt8"], "pagat/keep");
    assert_eq!((cw, aw), (4 + 6, 3));
}

#[test]
fn every_template_is_short() {
    for a in ATOMS {
        assert!(a.en.split_whitespace().count() <= 8, "{}", a.id);
        assert!(!a.de.is_empty());
    }
    for cl in CLASSES {
        assert!(cl.en.split_whitespace().count() <= 10, "{}", cl.id);
        assert!(!cl.de.is_empty());
    }
}

#[test]
fn the_passed_rules_are_consistent_with_their_classes_and_atoms() {
    use crate::m5r_rules::PASSED;
    for r in PASSED {
        assert!(r.lower >= 0.80, "{}", r.id);
        assert!((1..=2).contains(&r.atoms.len()), "{}", r.id);
        class_index(r.class);
        for a in r.atoms {
            atom_index(a);
        }
        let (cw, aw) = word_counts(r.atoms, r.class);
        assert_eq!(r.fits_budget, cw <= 8 && aw <= 10, "{}: {cw} + {aw} words", r.id);
        assert_eq!(r.too_rare, r.reach < 1.0 / 40.0, "{}", r.id);
        assert!((0.0..=1.0).contains(&r.reach));
    }
}

#[test]
fn a_matching_rule_needs_its_atoms_and_the_action_in_its_class() {
    let r = PassedRule {
        id: "pagat/keep:ml5",
        class: "pagat/keep",
        atoms: &["ml5"],
        fired: 100,
        precision: 0.9,
        n_eff: 100.0,
        lower: 0.85,
        reach: 0.1,
        too_rare: false,
        settings_ok: true,
        fits_budget: true,
    };
    let k = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("3d"));
    assert!(rule_matches(&r, &k, &Action::play(c("T9"))));
    assert!(!rule_matches(&r, &k, &Action::play(c("T1"))), "not in the class");
    let many = know(
        &["T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "T9"],
        NO,
        HID,
        &[],
        Some("T15"),
    );
    assert!(!rule_matches(&r, &many, &Action::play(c("T2"))), "ml5 fails");
}

#[test]
fn collecting_a_sitting_logs_decisions() {
    let log = collect(95_000_000, 1, 0, strohmandeln_core::types::Settings::default());
    assert!(log.ndeals >= 6, "{}", log.ndeals);
    assert!(log.by_class.iter().any(|v| !v.is_empty()));
}
