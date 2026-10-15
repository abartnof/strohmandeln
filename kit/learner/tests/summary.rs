//! The summary: counts, text form, versions. Ported from Cassino's learner
//! tests (`a_summary_counts_chances_met_and_the_gaps_of_those_missed`,
//! `a_summary_round_trips_through_its_text`, `a_stale_summary_is_not_counted`)
//! onto indexes and a ladder.

mod common;

use common::*;
use kit_learner::{stale, Chance, Ladder, Params, SkillSpec, Summary, Tally};

#[test]
fn a_summary_counts_chances_met_and_the_gaps_of_those_missed() {
    let chance = |skill, met, gap| Chance { skill, met, gap };
    let s = Summary::from_chances(
        &LADDER,
        &[
            chance(PAIRS, true, 1.0),
            chance(PAIRS, false, 0.75),
            chance(PAIRS, false, 0.5),
            chance(BUILDING, true, 2.0),
        ],
    );
    assert_eq!(
        s.of(PAIRS),
        Tally {
            chances: 3,
            met: 1,
            missed: 1.25
        }
    );
    assert_eq!(s.of(BUILDING).missed, 0.0);
    assert_eq!(s.of(SUMS), Tally::default());
    assert!(s.is_current(&LADDER));
    assert_eq!(s.tallies.len(), 8);
}

#[test]
fn a_summary_round_trips_through_its_text() {
    let s = summary(&[
        (PAIRS, 12, 11, 0.35),
        (BUILDING, 3, 0, 4.0 / 3.0),
        (TRAILING, 7, 7, 0.0),
    ]);
    let text = s.to_text(&LADDER);
    assert!(
        text.starts_with("cassino evidence v1\npairs 12 11 0.35\n"),
        "{text}"
    );
    assert_eq!(Summary::parse(&LADDER, &text), Ok(s.clone()));
    assert_eq!(Summary::parse(&LADDER, &format!("{text}\n")), Ok(s));
    for bad in [
        "",
        "cassino evidence v1",
        "cassino evidence v1\npairs 1 2 0\n",
        &text.replace("pairs", "pears"),
        &text.replace("12 11", "x 11"),
        &text.replace("0.35", "-1"),
        &text.replace("cassino", "piquet"),
        &format!("{text}\nextra 1 1 0"),
    ] {
        assert!(Summary::parse(&LADDER, bad).is_err(), "{bad}");
    }
}

#[test]
fn a_stale_summary_is_listed() {
    let mut old = summary(&[(PAIRS, 20, 0, 40.0)]);
    old.version += 1;
    let fresh = summary(&[]);
    assert!(!old.is_current(&LADDER));
    assert_eq!(stale(&LADDER, &[fresh.clone(), old, fresh.clone()]), [1]);
    assert!(stale(&LADDER, &[]).is_empty());
    // A summary of another ladder's size is as stale as one of another version.
    let mut short = fresh.clone();
    short.tallies.pop();
    assert_eq!(stale(&LADDER, &[fresh, short]), [1]);
}

#[test]
fn a_gap_that_is_not_a_finite_loss_counts_as_none() {
    let chance = |gap| Chance {
        skill: SUMS,
        met: false,
        gap,
    };
    let s = Summary::from_chances(
        &LADDER,
        &[
            chance(f64::NAN),
            chance(-2.0),
            chance(f64::INFINITY),
            chance(f64::NEG_INFINITY),
            chance(1.5),
        ],
    );
    assert_eq!(
        s.of(SUMS),
        Tally {
            chances: 5,
            met: 0,
            missed: 1.5
        }
    );
    // What it wrote it can read back, and the skill stays learnable.
    let text = s.to_text(&LADDER);
    assert_eq!(Summary::parse(&LADDER, &text), Ok(s.clone()));
    let l = kit_learner::learn(&LADDER, &PARAMS, &[s]);
    assert!(l.state(SUMS).cost.is_finite() && l.state(SUMS).cost > 0.0);
}

#[test]
fn a_summary_that_cannot_be_true_is_not_current() {
    let fresh = summary(&[(PAIRS, 3, 3, 0.0)]);
    assert!(fresh.is_current(&LADDER));
    let mut bad = fresh.clone();
    bad.tallies[PAIRS].met = 4;
    assert!(!bad.is_current(&LADDER), "met above chances");
    for missed in [f64::NAN, -1.0, f64::INFINITY] {
        let mut bad = fresh.clone();
        bad.tallies[PAIRS].missed = missed;
        assert!(!bad.is_current(&LADDER), "missed {missed}");
    }
    assert_eq!(stale(&LADDER, &[fresh, bad_met()]), [1]);
    // ...and so is not counted by the learner.
    assert_eq!(kit_learner::learn(&LADDER, &PARAMS, &[bad_met()]).games, 0);
}

fn bad_met() -> Summary {
    let mut s = summary(&[(PAIRS, 3, 3, 0.0)]);
    s.tallies[PAIRS].met = 4;
    s
}

#[test]
fn a_summary_of_a_newer_version_keeps_its_version_through_the_text() {
    let mut s = summary(&[(SUMS, 4, 2, 1.5)]);
    s.version = 7;
    let text = s.to_text(&LADDER);
    assert!(text.starts_with("cassino evidence v7\n"), "{text}");
    assert_eq!(Summary::parse(&LADDER, &text), Ok(s));
}

#[test]
fn a_ladder_is_checked() {
    static A: [SkillSpec; 2] = [
        SkillSpec {
            slug: "a",
            needs: &[],
            top_rate: 0.9,
            top_cost: 0.1,
            min_chances: 4.0,
            may_focus: true,
        },
        SkillSpec {
            slug: "b",
            needs: &[0],
            top_rate: 0.9,
            top_cost: 0.1,
            min_chances: 4.0,
            may_focus: true,
        },
    ];
    let ladder = |skills| Ladder {
        name: "game",
        version: 1,
        skills,
    };
    assert_eq!(ladder(&A).check(), Ok(()));
    assert_eq!(LADDER.check(), Ok(()));
    assert_eq!(ladder(&A).index_of("b"), Some(1));
    assert_eq!(ladder(&A).index_of("c"), None);

    static SELF: [SkillSpec; 1] = [SkillSpec {
        needs: &[0],
        ..A[0]
    }];
    static OUT: [SkillSpec; 1] = [SkillSpec {
        needs: &[3],
        ..A[0]
    }];
    static SPACE: [SkillSpec; 1] = [SkillSpec {
        slug: "a b",
        ..A[0]
    }];
    static RATE: [SkillSpec; 1] = [SkillSpec {
        top_rate: 1.5,
        ..A[0]
    }];
    static DUP: [SkillSpec; 2] = [A[0], A[0]];
    for (what, skills) in [
        ("itself", &SELF[..]),
        ("outside", &OUT[..]),
        ("whitespace", &SPACE[..]),
        ("rate", &RATE[..]),
        ("duplicate", &DUP[..]),
        ("empty", &[][..]),
    ] {
        assert!(ladder(skills).check().is_err(), "{what}");
    }
    let mut bad_name = ladder(&A);
    bad_name.name = "two words";
    assert!(bad_name.check().is_err());
}

#[test]
fn mastery_asks_a_fraction_of_the_strongest_rate_and_loss_a_margin_below() {
    let p = Params {
        master_of_top: 0.5,
        lose_below: 0.2,
        ..PARAMS
    };
    let top = &SKILLS[PAIRS];
    assert!((p.master(top) - 0.487).abs() < 1e-12);
    assert!((p.lose(top) - 0.287).abs() < 1e-12);
    // The loss bar never goes below nothing.
    let p = Params {
        master_of_top: 0.1,
        lose_below: 0.5,
        ..PARAMS
    };
    assert_eq!(p.lose(top), 0.0);
}
