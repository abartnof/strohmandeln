//! The brief's shape, after Cassino's brief tests that concern it: at most
//! three bullets, well / next / none in that order, never praise and work on
//! the same skill, mastery only after earlier deals, "enough skills seen",
//! a short deal says so in one bullet.

mod common;

use common::*;
use kit_learner::{
    brief, learn, BriefCfg, Bullet, Deal, Keys, Learner, Param, Strength, Summary, MAX_BULLETS,
};

const CFG: BriefCfg = BriefCfg {
    few: 5,
    min_history: 2,
    enough_skills: 3,
    keys: Keys {
        short: "t.short",
        well_lead: "t.lead.well",
        well_mastered: "t.well.mastered",
        none_early: "t.none.early",
        none_clean: "t.none.clean",
        none_up: "t.none.up",
    },
};

fn learner(games: &[Summary]) -> Learner {
    learn(&LADDER, &PARAMS, games)
}

fn strength(key: &str, praises: &[usize]) -> Strength {
    Strength {
        bullet: Bullet::new("t.lead.well", key),
        praises: praises.to_vec(),
    }
}

fn keys(b: &[Bullet]) -> Vec<&str> {
    b.iter().map(|b| b.key.as_str()).collect()
}

/// The brief of a deal `game` after `history`.
fn made(
    decisions: u32,
    history: &[Summary],
    game: &Summary,
    strengths: &[Strength],
    next_opponent: bool,
) -> Vec<Bullet> {
    let mut all = history.to_vec();
    all.push(game.clone());
    let (before, after) = (learner(history), learner(&all));
    brief(
        &LADDER,
        &CFG,
        &Deal {
            decisions,
            before: &before,
            after: &after,
            strengths,
            next_opponent,
        },
        |skill| {
            vec![
                Bullet::new("t.lead.next", "t.rule").with("skill", Param::Skill(skill)),
                Bullet::new("t.lead.deal", "t.example"),
                Bullet::new("", "t.extra"),
            ]
        },
    )
}

/// Six clean chances at three independent skills: three seen, none mastered.
fn clean() -> Summary {
    summary(&[
        (PAIRS, 6, 6, 0.0),
        (NO_SWEEP, 6, 6, 0.0),
        (VALUABLES, 6, 6, 0.0),
    ])
}

#[test]
fn the_focus_comes_with_the_games_bullets_and_no_more_than_three() {
    let game = summary(&[(BUILDING, 10, 0, 10.0)]);
    let b = made(40, &[], &game, &[], true);
    assert_eq!(keys(&b), ["t.rule", "t.example", "t.extra"]);
    assert_eq!(b[0].params, [("skill".to_string(), Param::Skill(BUILDING))]);
    assert_eq!(b[0].lead, "t.lead.next");
    // With a praise first, the game's bullets are cut to fit.
    let strengths = [strength("t.s.cards", &[])];
    let b = made(40, &[], &game, &strengths, true);
    assert_eq!(keys(&b), ["t.s.cards", "t.rule", "t.example"]);
    assert_eq!(b.len(), MAX_BULLETS);
}

#[test]
fn a_short_deal_says_so_in_one_bullet() {
    let game = summary(&[(BUILDING, 10, 0, 10.0)]);
    let strengths = [strength("t.s.cards", &[])];
    let b = made(4, &[], &game, &strengths, true);
    assert_eq!(keys(&b), ["t.short"]);
    assert_eq!(made(5, &[], &game, &[], true).len(), 3, "FEW is the least");
}

#[test]
fn a_mastery_is_announced_only_after_earlier_deals() {
    let hist: Vec<Summary> = (0..2).map(|_| game(PAIRS, 3, 3, 1.0)).collect();
    let g = game(PAIRS, 6, 6, 1.0);
    assert!(
        !learner(&hist).mastered(PAIRS),
        "two small deals are not yet enough"
    );
    // After two earlier deals: announced, in place of any strength.
    let strengths = [strength("t.s.cards", &[])];
    let b = made(40, &hist, &g, &strengths, false);
    assert_eq!(keys(&b)[0], "t.well.mastered");
    assert_eq!(b[0].lead, "t.lead.well");
    assert_eq!(b[0].params, [("skill".to_string(), Param::Skill(PAIRS))]);
    assert!(learner(&hist).just_mastered.is_empty());
    // After one earlier deal the learner may master it but the brief is quiet.
    let b = made(40, &hist[..1], &g, &strengths, false);
    assert!(learner(&[hist[0].clone(), g.clone()]).mastered(PAIRS));
    assert_eq!(keys(&b)[0], "t.s.cards");
}

#[test]
fn a_skill_is_never_praised_and_worked_on_in_one_brief() {
    let game = summary(&[(BUILDING, 10, 0, 10.0)]);
    let strengths = [
        strength("t.s.building", &[BUILDING]),
        strength("t.s.taking", &[PAIRS, SUMS]),
    ];
    let b = made(40, &[], &game, &strengths, true);
    assert_eq!(
        keys(&b)[0],
        "t.s.taking",
        "the praise of the focus is skipped"
    );
    let only = [strength("t.s.building", &[BUILDING])];
    let b = made(40, &[], &game, &only, true);
    assert_eq!(keys(&b), ["t.rule", "t.example", "t.extra"]);
}

#[test]
fn with_no_focus_the_brief_says_nothing_stood_out_or_that_it_is_early() {
    let g = clean();
    assert_eq!(learner(std::slice::from_ref(&g)).focus, None);
    // Three skills seen: nothing stood out, and the next opponent if any.
    assert_eq!(keys(&made(40, &[], &g, &[], false)), ["t.none.clean"]);
    assert_eq!(keys(&made(40, &[], &g, &[], true)), ["t.none.up"]);
    // Two skills seen: too early.
    let two = summary(&[(PAIRS, 6, 6, 0.0), (NO_SWEEP, 6, 6, 0.0)]);
    assert_eq!(keys(&made(40, &[], &two, &[], true)), ["t.none.early"]);
    // A strength still goes first.
    let s = [strength("t.s.cards", &[])];
    assert_eq!(
        keys(&made(40, &[], &g, &s, true)),
        ["t.s.cards", "t.none.up"]
    );
}

#[test]
fn a_focus_with_nothing_to_say_is_no_focus() {
    let game = summary(&[(BUILDING, 10, 0, 10.0)]);
    let (before, after) = (learner(&[]), learner(std::slice::from_ref(&game)));
    assert_eq!(after.focus, Some(BUILDING));
    let b = brief(
        &LADDER,
        &CFG,
        &Deal {
            decisions: 40,
            before: &before,
            after: &after,
            strengths: &[],
            next_opponent: false,
        },
        |_| vec![],
    );
    assert_eq!(keys(&b), ["t.none.early"]);
}

#[test]
fn the_focus_is_asked_for_only_when_there_is_one() {
    let g = clean();
    let (before, after) = (learner(&[]), learner(std::slice::from_ref(&g)));
    let _ = brief(
        &LADDER,
        &CFG,
        &Deal {
            decisions: 40,
            before: &before,
            after: &after,
            strengths: &[],
            next_opponent: false,
        },
        |_| panic!("no focus, no call"),
    );
}
