//! The comparisons at their exact thresholds: `>=` or `>`, `<` or `<=`. A
//! mutation of each of them survived the rest of the suite (review finding 4).
//! The inputs sit exactly on the line; the answers are Cassino's choices, which
//! `fixtures/cassino_learner.txt` fixes where Cassino's own ladder can reach the
//! line (the `edge-*` cases, run through Cassino's code by the generator).
//! Where only another ladder can (a mastery at exactly `min_chances`, a
//! quantile equal to the bar), the line is reached here by making the bar the
//! quantile itself (`master_of_top` 1, `top_rate` the same call's result), so
//! the floats are equal by construction.

mod common;

use common::*;
use kit_learner::beta::beta_quantile;
use kit_learner::{learn, Ladder, Params, SkillSpec, Summary};

fn run(games: &[Summary]) -> kit_learner::Learner {
    learn(&LADDER, &PARAMS, games)
}

// ---- With Cassino's ladder: answering-builds has top_cost 0 and top_rate 1,
// so a gain is the miss itself, and it needs only sums, which has no chances.

#[test]
fn a_gain_of_exactly_min_gain_over_exactly_min_chances_names_the_focus() {
    // 4 chances (= min_chances), none met, 0.5 lost (= min_gain).
    let at = run(&[summary(&[(ANSWERING_BUILDS, 4, 0, 0.5)])]);
    assert_eq!(at.state(ANSWERING_BUILDS).gain, 0.5);
    assert_eq!(at.focus, Some(ANSWERING_BUILDS));
    // A shade less gain, or one chance fewer, names none.
    let less = run(&[summary(&[(ANSWERING_BUILDS, 4, 0, 0.499)])]);
    assert_eq!(less.focus, None);
    let fewer = run(&[summary(&[(ANSWERING_BUILDS, 3, 0, 0.5)])]);
    assert_eq!(fewer.focus, None);
}

#[test]
fn a_costly_skill_needs_exactly_costly_chances_and_exactly_costly_gain() {
    // Met 7 of 8 (not surely short of 0.85), 2.0 lost: both lines exactly.
    let at = run(&[summary(&[(ANSWERING_BUILDS, 8, 7, 2.0)])]);
    assert_eq!(at.state(ANSWERING_BUILDS).chances, 8.0);
    assert_eq!(at.state(ANSWERING_BUILDS).gain, 2.0);
    assert_eq!(at.focus, Some(ANSWERING_BUILDS));
    // One chance fewer, or a shade less gain: no longer costly.
    assert_eq!(
        run(&[summary(&[(ANSWERING_BUILDS, 7, 6, 2.0)])]).focus,
        None
    );
    assert_eq!(
        run(&[summary(&[(ANSWERING_BUILDS, 8, 7, 1.999)])]).focus,
        None
    );
}

// ---- With a ladder of one skill, to reach the lines Cassino's cannot.

static ONE: [SkillSpec; 1] = [SkillSpec {
    slug: "one",
    needs: &[],
    top_rate: 0.5,
    top_cost: 0.0,
    min_chances: 4.0,
    may_focus: true,
}];

fn one(skills: &'static [SkillSpec]) -> Ladder {
    Ladder {
        name: "one",
        version: 1,
        skills,
    }
}

fn deal(ladder: &Ladder, chances: u32, met: u32) -> Summary {
    let mut s = Summary::new(ladder);
    s.tallies[0].chances = chances;
    s.tallies[0].met = met;
    s
}

#[test]
fn a_rate_counts_from_exactly_min_chances() {
    let l = one(&ONE);
    // Mastery needs a rate of 0.425; 4 of 4 clears it, and 4.0 chances count.
    let at = learn(&l, &PARAMS, &[deal(&l, 4, 4)]);
    assert!(at.mastered(0));
    // 3 of 3 would clear it too, but 3.0 chances do not count yet.
    let below = learn(&l, &PARAMS, &[deal(&l, 3, 3)]);
    assert!(!below.mastered(0));
}

#[test]
fn a_prerequisite_gaining_exactly_min_gain_is_not_cleared() {
    // a is surely short (0 of 6) and gains exactly 0.5; b needs a and is
    // surely short and gains more. a is not "gaining less than min_gain", so
    // it is not cleared and b waits.
    static TWO: [SkillSpec; 2] = [
        SkillSpec {
            slug: "a",
            needs: &[],
            top_rate: 0.9,
            top_cost: 0.0,
            min_chances: 4.0,
            may_focus: true,
        },
        SkillSpec {
            slug: "b",
            needs: &[0],
            top_rate: 0.9,
            top_cost: 0.0,
            min_chances: 4.0,
            may_focus: true,
        },
    ];
    let l = Ladder {
        name: "two",
        version: 1,
        skills: &TWO,
    };
    let game = |a_missed: f64| {
        let mut s = Summary::new(&l);
        s.tallies[0].chances = 6;
        s.tallies[0].missed = a_missed;
        s.tallies[1].chances = 6;
        s.tallies[1].missed = 9.0;
        s
    };
    assert_eq!(learn(&l, &PARAMS, &[game(0.5)]).focus, Some(0));
    // A shade under, a is cleared by costing too little, and b is the focus
    // (a is no longer open: its gain is below min_gain).
    assert_eq!(learn(&l, &PARAMS, &[game(0.499)]).focus, Some(1));
}

/// A ladder of one skill whose bar (master, with `master_of_top` 1 and no
/// `lose_below`) is exactly `bar`.
fn bar(bar: f64) -> (&'static Ladder, Params) {
    let skills: &'static [SkillSpec] = Box::leak(Box::new([SkillSpec {
        top_rate: bar,
        ..ONE[0]
    }]));
    let ladder: &'static Ladder = Box::leak(Box::new(one(skills)));
    let p = Params {
        master_of_top: 1.0,
        lose_below: 0.0,
        ..PARAMS
    };
    (ladder, p)
}

#[test]
fn mastery_comes_at_a_lower_quantile_exactly_equal_to_the_bar() {
    // 10 of 10: Beta(11, 1).
    let (l, p) = bar(beta_quantile(p0().lower, 11.0, 1.0));
    assert!(
        learn(l, &p, &[deal(l, 10, 10)]).mastered(0),
        "equal is enough"
    );
    // A bar one step above the quantile is not reached.
    let (l, p) = bar(beta_quantile(p0().lower, 11.0, 1.0) * (1.0 + 1e-12));
    assert!(!learn(l, &p, &[deal(l, 10, 10)]).mastered(0));
}

#[test]
fn a_rate_is_surely_short_only_when_the_upper_quantile_is_below_the_bar() {
    // 6 of 10: Beta(7, 5), and 1.5 lost a deal, so only "surely short" can
    // name the focus (the gain is below costly_gain).
    let upper = beta_quantile(p0().upper, 7.0, 5.0);
    let s = |l: &Ladder| {
        let mut s = deal(l, 10, 6);
        s.tallies[0].missed = 1.5;
        s
    };
    let (l, p) = bar(upper * (1.0 + 1e-12));
    assert_eq!(learn(l, &p, &[s(l)]).focus, Some(0), "below the bar");
    let (l, p) = bar(upper);
    assert_eq!(learn(l, &p, &[s(l)]).focus, None, "equal is not below");
}

#[test]
fn a_mastery_is_lost_only_when_the_upper_quantile_is_below_the_bar() {
    // Master on 20 of 20, then a deal of 4 chances, 1 met. The counts the
    // learner then holds are worked out here with its own expressions, and the
    // bar is the upper quantile of that posterior.
    let d = p0().discount;
    let chances = 20.0 * d + 4.0;
    let met = 20.0 * d + 1.0;
    let upper = beta_quantile(p0().upper, 1.0 + met, 1.0 + chances - met);
    let games = |l: &Ladder| vec![deal(l, 20, 20), deal(l, 4, 1)];
    // The first deal alone masters it at these bars (the lower quantile of
    // 20 of 20 is above the upper quantile of the second state).
    let (l, p) = bar(upper * (1.0 + 1e-12));
    assert!(learn(l, &p, &games(l)[..1]).mastered(0));
    assert!(!learn(l, &p, &games(l)).mastered(0), "below the bar: lost");
    let (l, p) = bar(upper);
    assert!(learn(l, &p, &games(l)).mastered(0), "equal is kept");
}

fn p0() -> Params {
    PARAMS
}
