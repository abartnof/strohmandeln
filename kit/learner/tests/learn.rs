//! The learner. Ported from Cassino's learner tests (the generic ones: a
//! skill is mastered and lost, the state depends on the state before, the
//! focus is the costliest cleared skill, a few misses among few chances name
//! no focus, older games count for less, a stale summary is not counted),
//! onto indexes and Cassino's ladder; the tie rule and the other ladders are
//! new.

mod common;

use common::*;
use kit_learner::{learn, Ladder, SkillSpec, Summary};

fn close(a: f64, b: f64) {
    assert!((a - b).abs() < 1e-9, "{a} != {b}");
}

fn run(games: &[Summary]) -> kit_learner::Learner {
    learn(&LADDER, &PARAMS, games)
}

#[test]
fn a_stale_summary_is_not_counted() {
    let mut old = summary(&[(PAIRS, 20, 0, 40.0)]);
    old.version += 1;
    let l = run(&[old]);
    assert_eq!(l.focus, None);
    assert!(!l.mastered(PAIRS));
    assert_eq!(l.games, 0);
    assert_eq!(run(&[summary(&[]), summary(&[])]).games, 2);
}

#[test]
fn a_skill_is_mastered_when_its_rate_is_surely_high_and_lost_when_surely_low() {
    let mut games = Vec::new();
    let mut states = Vec::new();
    for _ in 0..3 {
        games.push(game(PAIRS, 6, 6, 1.0));
        states.push(run(&games).mastered(PAIRS));
    }
    assert_eq!(states, [false, true, true], "first too few, then sure");
    assert_eq!(run(&games[..2]).just_mastered, [PAIRS]);
    assert!(
        run(&games).just_mastered.is_empty(),
        "only the game it came with"
    );
    // One bad game does not lose it; a run of them does.
    games.push(game(PAIRS, 6, 3, 1.0));
    assert!(run(&games).mastered(PAIRS), "hysteresis");
    assert!(run(&games).just_mastered.is_empty());
    for _ in 0..4 {
        games.push(game(PAIRS, 6, 1, 1.0));
    }
    assert!(!run(&games).mastered(PAIRS));
}

#[test]
fn the_state_after_a_game_depends_on_the_state_before() {
    // The same recent games, a different past: a rate between losing
    // and gaining mastery keeps whichever state it came to.
    let middling = game(PAIRS, 30, 25, 0.5);
    let mut up: Vec<Summary> = (0..3).map(|_| game(PAIRS, 30, 30, 0.5)).collect();
    let mut down: Vec<Summary> = (0..3).map(|_| game(PAIRS, 30, 15, 0.5)).collect();
    for _ in 0..30 {
        up.push(middling.clone());
        down.push(middling.clone());
    }
    let (up, down) = (run(&up), run(&down));
    assert!(up.mastered(PAIRS));
    assert!(!down.mastered(PAIRS));
}

#[test]
fn the_focus_is_the_costliest_skill_whose_prerequisites_are_cleared() {
    // Sums costs most, but needs Pairs; Pairs is not mastered (it is
    // missed, cheaply), so Pairs is the focus.
    let games = vec![summary(&[
        (PAIRS, 10, 4, 3.0),
        (SUMS, 10, 2, 16.0),
        (VALUABLES, 5, 3, 1.0),
    ])];
    let l = run(&games);
    assert_eq!(l.focus, Some(PAIRS));
    close(l.state(SUMS).cost, 16.0);
    // With Pairs cleared (mastered), Sums is the next.
    let games: Vec<Summary> = (0..3)
        .map(|_| {
            summary(&[
                (PAIRS, 20, 20, 0.0),
                (SUMS, 10, 2, 16.0),
                (VALUABLES, 5, 3, 1.0),
            ])
        })
        .collect();
    assert_eq!(run(&games).focus, Some(SUMS));
    // A prerequisite that costs nothing is cleared without being
    // mastered (too few chances to say).
    let games = vec![summary(&[(SUMS, 10, 2, 16.0)])];
    assert_eq!(run(&games).focus, Some(SUMS));
}

#[test]
fn a_few_misses_among_few_chances_name_no_focus() {
    // One miss of five, costing 3 points: 3 points a game, but the
    // rate may still be high.
    let one = vec![summary(&[(VALUABLES, 5, 4, 3.0)])];
    assert_eq!(run(&one).focus, None);
    // Two met of five in each of six games: surely short.
    let six = vec![summary(&[(VALUABLES, 5, 2, 3.0)]); 6];
    assert_eq!(run(&six).focus, Some(VALUABLES));
}

#[test]
fn one_missed_chance_names_no_focus() {
    // A single chance, missed, with a large gap: no evidence of a rate.
    let one = vec![summary(&[(VALUABLES, 1, 0, 3.0)])];
    assert_eq!(run(&one).focus, None);
}

#[test]
fn a_prerequisite_not_shown_short_does_not_block_what_is_surely_short() {
    // Pairs is met 7 of 8 (a miss costs 2 points): not mastered, but not
    // surely short either. Sums is met 1 of 6: surely short, and it
    // should be named, not left behind its half-proven prerequisite.
    let games = vec![summary(&[(PAIRS, 8, 7, 2.0), (SUMS, 6, 1, 25.0)]); 20];
    let l = run(&games);
    assert!(!l.mastered(PAIRS));
    assert_eq!(l.focus, Some(SUMS));
}

#[test]
fn a_costly_skill_with_a_middling_rate_is_the_focus() {
    // A random player's Valuables: met 39 of 48 (0.81, not surely short
    // of 0.845) but 6 points a game lost.
    let games = vec![summary(&[(VALUABLES, 12, 10, 14.0)]); 8];
    let l = run(&games);
    assert!(l.state(VALUABLES).gain > PARAMS.costly_gain);
    assert_eq!(l.focus, Some(VALUABLES));
}

#[test]
fn a_skill_costing_a_lot_is_the_focus_even_when_not_surely_short() {
    // Met 5 of 6 each game, a miss costing 12 points: not surely short
    // (the rate may be 0.85) but a great deal is being lost.
    let games = vec![summary(&[(VALUABLES, 6, 5, 12.0)]); 10];
    let l = run(&games);
    assert!(!l.mastered(VALUABLES));
    assert_eq!(l.focus, Some(VALUABLES));
}

#[test]
fn no_focus_when_nothing_costs_enough() {
    let games = vec![summary(&[(PAIRS, 10, 9, 0.2), (VALUABLES, 5, 4, 0.1)])];
    assert_eq!(run(&games).focus, None);
    assert_eq!(run(&[]).focus, None);
}

#[test]
fn older_games_count_for_less() {
    let bad = game(VALUABLES, 10, 0, 1.0);
    let fine = game(VALUABLES, 10, 10, 1.0);
    let recent_bad = run(&[fine.clone(), bad.clone()]);
    let recent_fine = run(&[bad, fine]);
    assert!(recent_bad.state(VALUABLES).cost > recent_fine.state(VALUABLES).cost);
    close(
        recent_fine.state(VALUABLES).cost,
        10.0 / (1.0 + PARAMS.discount) * PARAMS.discount,
    );
}

// New: the tie rule and what the parameters move.

static TWINS: [SkillSpec; 3] = [
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
        needs: &[],
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: true,
    },
    SkillSpec {
        slug: "c",
        needs: &[0],
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: true,
    },
];

fn twins() -> Ladder {
    Ladder {
        name: "twins",
        version: 3,
        skills: &TWINS,
    }
}

fn twin_games(lines: &[(usize, u32, u32, f64)]) -> Vec<Summary> {
    let mut s = Summary::new(&twins());
    for &(i, chances, met, missed) in lines {
        s.tallies[i].chances = chances;
        s.tallies[i].met = met;
        s.tallies[i].missed = missed;
    }
    vec![s; 6]
}

#[test]
fn exact_ties_in_gain_go_to_the_earlier_skill() {
    let games = twin_games(&[(0, 6, 1, 10.0), (1, 6, 1, 10.0)]);
    let l = learn(&twins(), &PARAMS, &games);
    close(l.state(0).gain, l.state(1).gain);
    assert_eq!(l.focus, Some(0));
    // A larger gain later in the ladder still wins.
    let games = twin_games(&[(0, 6, 1, 10.0), (1, 6, 1, 10.5)]);
    assert_eq!(learn(&twins(), &PARAMS, &games).focus, Some(1));
}

#[test]
fn a_ladder_of_its_own_is_learnt_by_its_own_numbers() {
    // c needs a: while a is short and costly, c waits, though it costs more.
    let games = twin_games(&[(0, 6, 1, 10.0), (2, 6, 1, 30.0)]);
    let l = learn(&twins(), &PARAMS, &games);
    assert_eq!(l.focus, Some(0));
    assert_eq!(l.skills.len(), 3);
    // A summary of Cassino's ladder is stale on it (another version and size).
    assert_eq!(learn(&twins(), &PARAMS, &[summary(&[])]).games, 0);
}

#[test]
fn a_skill_with_its_own_min_chances_counts_from_there() {
    // Three chances in one deal, none met, a miss costing 4 points: seen
    // from 2 chances (the rare skill's own bar), not from 4.
    static RARE: [SkillSpec; 1] = [SkillSpec {
        slug: "rare",
        needs: &[],
        top_rate: 0.95,
        top_cost: 0.0,
        min_chances: 2.0,
        may_focus: true,
    }];
    static COMMON: [SkillSpec; 1] = [SkillSpec {
        min_chances: 4.0,
        may_focus: true,
        ..RARE[0]
    }];
    let ladder = |skills| Ladder {
        name: "rare",
        version: 1,
        skills,
    };
    for (skills, focus) in [(&RARE[..], Some(0)), (&COMMON[..], None)] {
        let ladder = ladder(skills);
        let mut s = Summary::new(&ladder);
        s.tallies[0].chances = 3;
        s.tallies[0].missed = 12.0;
        assert_eq!(learn(&ladder, &PARAMS, &[s]).focus, focus);
    }
}

// New: a skill that may not be the focus (the game has not shown it worth anything).

static MUTE: [SkillSpec; 3] = [
    SkillSpec {
        slug: "a",
        needs: &[],
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: false,
    },
    SkillSpec {
        slug: "b",
        needs: &[0],
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: true,
    },
    SkillSpec {
        slug: "c",
        needs: &[],
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: true,
    },
];

fn mute() -> Ladder {
    Ladder {
        name: "mute",
        version: 3,
        skills: &MUTE,
    }
}

fn mute_games(lines: &[(usize, u32, u32, f64)]) -> Vec<Summary> {
    let mut s = Summary::new(&mute());
    for &(i, chances, met, missed) in lines {
        s.tallies[i].chances = chances;
        s.tallies[i].met = met;
        s.tallies[i].missed = missed;
    }
    vec![s; 6]
}

#[test]
fn a_skill_that_may_not_be_the_focus_never_is_however_much_it_costs() {
    let games = mute_games(&[(0, 6, 0, 60.0)]);
    let l = learn(&mute(), &PARAMS, &games);
    assert!(l.state(0).gain > PARAMS.costly_gain, "it does cost");
    assert_eq!(l.focus, None);
    // the others are still chosen among
    let games = mute_games(&[(0, 6, 0, 60.0), (2, 6, 1, 10.0)]);
    assert_eq!(learn(&mute(), &PARAMS, &games).focus, Some(2));
}

#[test]
fn a_skill_that_may_not_be_the_focus_does_not_hold_back_what_needs_it() {
    // b needs a; a is short and costly, but it cannot be the focus, so b is
    // not made to wait for it.
    let games = mute_games(&[(0, 6, 0, 60.0), (1, 6, 1, 10.0)]);
    assert_eq!(learn(&mute(), &PARAMS, &games).focus, Some(1));
}

#[test]
fn a_skill_that_may_not_be_the_focus_is_still_counted_and_mastered() {
    let one = mute_games(&[(0, 6, 6, 0.0)]).remove(0);
    let games = vec![one; 4];
    let l = learn(&mute(), &PARAMS, &games);
    assert!(l.mastered(0));
    assert!(l.state(0).chances > 0.0);
}
