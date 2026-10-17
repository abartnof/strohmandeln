//! Tests of the brief (`docs/TUTOR.md` 6, 12): constructed learners and
//! summaries for the shape, played deals for the whole.

use super::*;
use crate::lessons::{self, Gate};
use crate::opponent::{Config, Level, Opponent};
use crate::tutor::{Consequence, Direction, Facts, Miss, Skill, LADDER, PARAMS, SKILLS};
use kit_learner::{learn, Learner, Param, SkillState, Summary, Tally};
use strohmandeln_core::agents::Player;
use strohmandeln_core::cards::Honour;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Action, Settings};
use strohmandeln_core::view::view as sitting_view;

fn card(code: &str) -> Card {
    Card::parse(code).unwrap()
}

fn summary(lines: &[(Skill, u32, u32, f64)]) -> Summary {
    let mut s = Summary::new(&LADDER);
    for &(skill, chances, met, missed) in lines {
        s.tallies[skill.index()] = Tally {
            chances,
            met,
            missed,
        };
    }
    s
}

fn miss(skill: Skill, direction: Option<Direction>) -> Miss {
    Miss {
        skill,
        action: 3,
        trick: 14,
        played: Some(card("Qh")),
        better: Some(card("4h")),
        cost: 20.0,
        direction,
        led: Some(card("Kd")),
        trick_thirds: 22,
        tarocks: 5,
        sure_thirds: 61,
        consequence: None,
    }
}

fn nothing() -> [Option<(Miss, u32)>; SKILLS] {
    Default::default()
}

fn deal_of(
    s: Summary,
    decisions: u32,
    misses: [Option<(Miss, u32)>; SKILLS],
    facts: Facts,
) -> Played {
    Played {
        summary: s,
        decisions,
        facts: vec![facts],
        misses,
        settings: Settings::default(),
        variety: 0,
        // the tests admit the provisional rules; `until_a_rule_passes...` the real
        told: ADMIT,
    }
}

/// What the tests admit: the provisional rules too.
const ADMIT: &[Gate] = &[Gate::Provisional, Gate::Passed];

fn made(history: &[Summary], played: &Played, up: bool) -> Brief {
    let before = learn(&LADDER, &PARAMS, history);
    let mut all = history.to_vec();
    all.push(played.summary.clone());
    let after = learn(&LADDER, &PARAMS, &all);
    compose(&before, &after, played, up)
}

fn keys(b: &Brief) -> Vec<(&str, &str)> {
    b.bullets
        .iter()
        .map(|x| (x.lead.as_str(), x.key.as_str()))
        .collect()
}

fn param<'a>(b: &'a Bullet, name: &str) -> &'a Param {
    &b.params
        .iter()
        .find(|(k, _)| k == name)
        .unwrap_or_else(|| panic!("{name} in {b:?}"))
        .1
}

/// Deals in which `worth` is missed at most chances and cost a lot: the
/// learner names it.
fn bad_at_worth(n: usize) -> Vec<Summary> {
    (0..n)
        .map(|_| summary(&[(Skill::Worth, 6, 2, 60.0), (Skill::Cheap, 3, 3, 0.0)]))
        .collect()
}

#[test]
fn a_focus_with_a_miss_is_told_by_the_rule_of_its_direction_and_the_example() {
    let history = bad_at_worth(4);
    let played = deal_of(
        summary(&[(Skill::Worth, 6, 2, 60.0)]),
        12,
        {
            let mut m = nothing();
            m[0] = Some((miss(Skill::Worth, Some(Direction::Greedy)), 0));
            m
        },
        Facts::default(),
    );
    let b = made(&history, &played, true);
    assert_eq!(
        keys(&b),
        [
            // the rule for a greedy trick failed its measurement (M5): the fact
            ("tutor.lead.next", "tutor.fact.worth"),
            ("tutor.lead.deal", "tutor.miss.worth.poor")
        ],
        "{b:?}"
    );
    assert_eq!(param(&b.bullets[0], "skill"), &Param::Skill(0));
    assert_eq!(param(&b.bullets[1], "played"), &Param::Text("Qh".into()));
    assert_eq!(param(&b.bullets[1], "thirds"), &Param::Int(22));
    // the other direction, the other rule and example
    let mut m = nothing();
    m[0] = Some((miss(Skill::Worth, Some(Direction::Timid)), 0));
    let played = deal_of(
        summary(&[(Skill::Worth, 6, 2, 60.0)]),
        12,
        m,
        Facts::default(),
    );
    let b = made(&history, &played, true);
    assert_eq!(b.bullets[0].key, "tutor.rule.worth.rich");
    assert_eq!(b.bullets[1].key, "tutor.miss.worth.rich");
    assert_eq!(param(&b.bullets[1], "led"), &Param::Text("Kd".into()));
    // a timid miss on a trick led with a pip is outside what M5 measured:
    // the fact, not the rule
    let mut poor = miss(Skill::Worth, Some(Direction::Timid));
    poor.led = Some(card("3h"));
    let mut m = nothing();
    m[0] = Some((poor, 0));
    let played = deal_of(
        summary(&[(Skill::Worth, 6, 2, 60.0)]),
        12,
        m,
        Facts::default(),
    );
    let b = made(&history, &played, true);
    assert_eq!(b.bullets[0].key, "tutor.fact.worth", "{b:?}");
}

#[test]
fn a_focus_without_a_miss_this_deal_gets_a_try_and_no_rule_for_a_directional_skill() {
    let history = bad_at_worth(4);
    // this deal had no chance at worth: the focus stands from the history
    let played = deal_of(summary(&[]), 12, nothing(), Facts::default());
    let b = made(&history, &played, true);
    assert_eq!(
        keys(&b),
        [
            ("tutor.lead.next", "tutor.fact.worth"),
            ("tutor.lead.try", "tutor.try.worth.1")
        ]
    );
    // a single-rule skill that passed is told its rule with no miss at all
    let announce: Vec<Summary> = (0..4)
        .map(|_| summary(&[(Skill::Announce, 6, 1, 60.0)]))
        .collect();
    let b = made(
        &announce,
        &deal_of(summary(&[]), 12, nothing(), Facts::default()),
        true,
    );
    assert_eq!(b.bullets[0].key, "tutor.rule.announce", "{b:?}");
    let mut played = deal_of(summary(&[]), 12, nothing(), Facts::default());
    played.variety = 1;
    let b = made(&announce, &played, true);
    assert_eq!(
        b.bullets[1].key, "tutor.try.announce.2",
        "the second try for variety"
    );
}

#[test]
fn a_focus_whose_rule_does_not_apply_is_told_as_a_fact_and_the_example() {
    // the Pagat at nine tarocks: the books differ
    let history: Vec<Summary> = (0..4)
        .map(|_| {
            summary(&[
                (Skill::Worth, 6, 6, 0.0),
                (Skill::Cheap, 6, 6, 0.0),
                (Skill::Run, 6, 6, 0.0),
                (Skill::Pagat, 6, 1, 90.0),
            ])
        })
        .collect();
    let mut m = nothing();
    let mut pagat = miss(Skill::Pagat, Some(Direction::Cash));
    pagat.tarocks = 9;
    m[Skill::Pagat.index()] = Some((pagat, 0));
    let played = deal_of(
        summary(&[(Skill::Pagat, 2, 0, 20.0)]),
        12,
        m,
        Facts::default(),
    );
    let b = made(&history, &played, true);
    assert_eq!(b.bullets[0].key, "tutor.fact.pagat", "{b:?}");
    assert_eq!(b.bullets[1].key, "tutor.miss.pagat.cash");
    assert_eq!(param(&b.bullets[1], "n"), &Param::Int(9));
}

#[test]
fn nothing_stood_out_says_so_and_points_up_only_below_the_top() {
    let clean: Vec<Summary> = (0..6)
        .map(|_| {
            summary(&[
                (Skill::Worth, 8, 8, 0.0),
                (Skill::Cheap, 6, 6, 0.0),
                (Skill::Void, 5, 5, 0.0),
            ])
        })
        .collect();
    let played = deal_of(
        summary(&[(Skill::Worth, 8, 8, 0.0), (Skill::Cheap, 6, 6, 0.0)]),
        12,
        nothing(),
        Facts::default(),
    );
    let up = made(&clean, &played, true);
    assert_eq!(keys(&up).last().unwrap().1, "tutor.none.up", "{up:?}");
    let top = made(&clean, &played, false);
    assert_eq!(keys(&top).last().unwrap().1, "tutor.none.clean");
    assert_eq!(top.focus, None);
}

#[test]
fn too_early_says_so() {
    let played = deal_of(
        summary(&[(Skill::Worth, 2, 2, 0.0)]),
        12,
        nothing(),
        Facts::default(),
    );
    let b = made(&[], &played, true);
    assert_eq!(keys(&b).last().unwrap().1, "tutor.none.early");
}

#[test]
fn a_short_deal_says_so_in_one_bullet() {
    let played = deal_of(
        summary(&[(Skill::Worth, 6, 0, 90.0)]),
        FEW - 1,
        nothing(),
        Facts::default(),
    );
    let b = made(&bad_at_worth(4), &played, true);
    assert_eq!(keys(&b), [("", "tutor.short")]);
    let enough = deal_of(
        summary(&[(Skill::Worth, 6, 0, 90.0)]),
        FEW,
        nothing(),
        Facts::default(),
    );
    assert!(made(&bad_at_worth(4), &enough, true).bullets.len() > 1);
}

#[test]
fn skills_mastered_are_the_first_thing_that_went_well_but_not_after_one_deal() {
    // two earlier deals of a little, then a deal that clears two skills
    let history = vec![
        summary(&[(Skill::Worth, 1, 1, 0.0), (Skill::Cheap, 1, 1, 0.0)]),
        summary(&[(Skill::Worth, 1, 1, 0.0), (Skill::Cheap, 1, 1, 0.0)]),
    ];
    let s = summary(&[(Skill::Worth, 12, 12, 0.0), (Skill::Cheap, 12, 12, 0.0)]);
    let b = made(
        &history,
        &deal_of(s.clone(), 20, nothing(), Facts::default()),
        true,
    );
    assert_eq!(
        keys(&b)[0],
        ("tutor.lead.well", "tutor.well.mastered-two"),
        "{b:?}"
    );
    assert_eq!(param(&b.bullets[0], "skill"), &Param::Skill(0));
    assert_eq!(param(&b.bullets[0], "skill2"), &Param::Skill(1));
    // after one earlier deal a mastery is luck of the draw
    let b = made(
        &history[..1],
        &deal_of(s, 20, nothing(), Facts::default()),
        true,
    );
    assert!(
        keys(&b)
            .iter()
            .all(|(_, k)| !k.starts_with("tutor.well.mastered")),
        "{b:?}"
    );
    // one skill alone
    let history = vec![
        summary(&[(Skill::Worth, 1, 1, 0.0)]),
        summary(&[(Skill::Worth, 1, 1, 0.0)]),
    ];
    let s = summary(&[(Skill::Worth, 12, 12, 0.0)]);
    let b = made(&history, &deal_of(s, 20, nothing(), Facts::default()), true);
    assert_eq!(keys(&b)[0].1, "tutor.well.mastered");
}

#[test]
fn what_went_well_is_the_best_strength_and_luck_is_only_praise() {
    let history = bad_at_worth(0);
    let all_met = summary(&[
        (Skill::Cheap, 3, 3, 0.0),
        (Skill::Off, 5, 5, 0.0),
        (Skill::Void, 4, 3, 10.0),
    ]);
    let b = made(
        &history,
        &deal_of(all_met, 20, nothing(), Facts::default()),
        true,
    );
    // the skill with the most chances met
    assert_eq!(b.bullets[0].key, "tutor.well.met");
    assert_eq!(
        param(&b.bullets[0], "skill"),
        &Param::Skill(Skill::Off.index())
    );
    assert_eq!(param(&b.bullets[0], "chances"), &Param::Int(5));
    // no skill all met: the honour, then the ultimo, then the Mond
    let facts = Facts {
        ultimo_won: true,
        mond_caught: Some(9),
        announced: vec![Honour::Trull],
    };
    let s = summary(&[(Skill::Announce, 1, 1, 0.0)]);
    let b = made(&history, &deal_of(s, 20, nothing(), facts.clone()), true);
    assert_eq!(b.bullets[0].key, "tutor.well.announced");
    assert_eq!(param(&b.bullets[0], "honour"), &Param::Text("trull".into()));
    let b = made(
        &history,
        &deal_of(
            summary(&[]),
            20,
            nothing(),
            Facts {
                announced: vec![],
                ..facts.clone()
            },
        ),
        true,
    );
    assert_eq!(b.bullets[0].key, "tutor.well.ultimo");
    let b = made(
        &history,
        &deal_of(
            summary(&[]),
            20,
            nothing(),
            Facts {
                ultimo_won: false,
                announced: vec![],
                ..facts
            },
        ),
        true,
    );
    assert_eq!(b.bullets[0].key, "tutor.well.mond");
    assert_eq!(param(&b.bullets[0], "trick"), &Param::Int(9));
    // a single chance met is nothing to praise
    let b = made(
        &history,
        &deal_of(
            summary(&[(Skill::Cheap, 1, 1, 0.0)]),
            20,
            nothing(),
            Facts::default(),
        ),
        true,
    );
    assert!(b.bullets.iter().all(|x| x.key != "tutor.well.met"));
}

#[test]
fn a_skill_is_never_praised_and_worked_on_in_one_brief() {
    // worth is the focus from the history, and this deal met all its chances
    let history = bad_at_worth(4);
    let played = deal_of(
        summary(&[(Skill::Worth, 5, 5, 0.0), (Skill::Cheap, 3, 3, 0.0)]),
        20,
        nothing(),
        Facts::default(),
    );
    let b = made(&history, &played, true);
    assert_eq!(b.focus, Some(Skill::Worth));
    for x in &b.bullets {
        if x.key == "tutor.well.met" {
            assert_ne!(
                param(x, "skill"),
                &Param::Skill(Skill::Worth.index()),
                "{b:?}"
            );
        }
    }
    // the next strength is praised instead
    assert_eq!(b.bullets[0].key, "tutor.well.met");
    assert_eq!(
        param(&b.bullets[0], "skill"),
        &Param::Skill(Skill::Cheap.index())
    );
    // the Pagat ultimo won praises the Pagat: dropped when it is the focus
    let pagat: Vec<Summary> = (0..4)
        .map(|_| {
            summary(&[
                (Skill::Worth, 6, 6, 0.0),
                (Skill::Cheap, 6, 6, 0.0),
                (Skill::Run, 6, 6, 0.0),
                (Skill::Pagat, 6, 1, 90.0),
            ])
        })
        .collect();
    let facts = Facts {
        ultimo_won: true,
        ..Facts::default()
    };
    let b = made(&pagat, &deal_of(summary(&[]), 20, nothing(), facts), true);
    assert_eq!(b.focus, Some(Skill::Pagat));
    assert!(
        b.bullets.iter().all(|x| x.key != "tutor.well.ultimo"),
        "{b:?}"
    );
}

#[test]
fn a_sitting_names_the_deal_of_its_example() {
    let history = bad_at_worth(4);
    let mut m = nothing();
    m[0] = Some((miss(Skill::Worth, Some(Direction::Greedy)), 4));
    let mut played = deal_of(
        summary(&[(Skill::Worth, 6, 2, 60.0)]),
        30,
        m,
        Facts::default(),
    );
    played.facts.push(Facts::default());
    let b = made(&history, &played, true);
    assert_eq!(b.bullets[1].lead, "tutor.lead.deal-n");
    assert_eq!(param(&b.bullets[1], "deal"), &Param::Int(4));
}

#[test]
fn the_example_of_a_figure_names_a_later_trick_only_where_the_log_shows_the_fall() {
    let mut m = miss(Skill::Off, Some(Direction::Figure));
    m.better = Some(card("Qh"));
    let (key, params) = example(&m).unwrap();
    assert_eq!(key, "tutor.miss.off.figure-plain");
    assert!(params.iter().any(|(k, _)| k == "better"));
    m.consequence = Some(Consequence::FellUnder {
        by: card("Kh"),
        trick: 14,
    });
    let (key, params) = example(&m).unwrap();
    assert_eq!(key, "tutor.miss.off.figure");
    let get = |n: &str| params.iter().find(|(k, _)| k == n).map(|(_, v)| v.clone());
    assert_eq!(get("card"), Some(Param::Text("Qh".into())));
    assert_eq!(get("king"), Some(Param::Text("Kh".into())));
    assert_eq!(get("late"), Some(Param::Int(14)));
}

// ---- the words: every bullet a brief can hold, in both languages ----------

fn strings() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../web3d/strings.json")).unwrap()
}

/// A bullet's words with the parameters put in as the table would (a card is
/// as long as the table's longest name).
fn say(table: &serde_json::Value, lang: &str, b: &Bullet, long_cards: bool) -> String {
    let get = |key: &str| -> String {
        table[key][lang]
            .as_str()
            .unwrap_or_else(|| panic!("{key} {lang}"))
            .to_string()
    };
    let fill = |text: String| -> String {
        let mut out = text;
        for (name, p) in &b.params {
            let value = match p {
                Param::Skill(i) if !long_cards => {
                    // a name is one word where the limit of a key is held; it has its own
                    let _ = i;
                    "S".to_string()
                }
                Param::Skill(i) => get(&format!("tutor.name.{}", Skill::ALL[*i].slug())),
                Param::Int(n) => n.to_string(),
                Param::Num(n) => n.to_string(),
                Param::Text(s) if s.len() == 2 || s.starts_with('T') => match (long_cards, lang) {
                    (true, "en") => "king of hearts".to_string(),
                    (true, _) => "Herz-König".to_string(),
                    (false, _) => "X".to_string(),
                },
                Param::Text(s) => s.clone(),
            };
            out = out.replace(&format!("{{{name}}}"), &value);
        }
        out
    };
    let lead = if b.lead.is_empty() {
        String::new()
    } else {
        fill(get(&b.lead))
    };
    format!("{lead} {}", fill(get(&b.key))).trim().to_string()
}

/// "About 45 words" (`docs/TUTOR.md` 6) with the leads, a card named in three
/// words, a skill in full and the "If I may:" of a rule left out. The longest
/// brief that can be made is measured below; this is the bound it must keep.
const MAX_BRIEF_WORDS: usize = 62;

fn count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Every focus bullet pair the lessons can make, with the worst companions.
fn every_brief() -> Vec<Vec<Bullet>> {
    let mut out = Vec::new();
    let directions = [
        None,
        Some(Direction::Greedy),
        Some(Direction::Timid),
        Some(Direction::Figure),
        Some(Direction::Pip),
        Some(Direction::Cash),
        Some(Direction::Keep),
        Some(Direction::Passed),
        Some(Direction::Took),
    ];
    for skill in Skill::ALL {
        for d in directions {
            for consequence in [
                None,
                Some(Consequence::FellUnder {
                    by: card("Kh"),
                    trick: 22,
                }),
            ] {
                for tarocks in [3, 9, 14] {
                    for (with_miss, variety, n) in
                        [(true, 0, 0), (true, 0, 3), (false, 0, 0), (false, 1, 0)]
                    {
                        let mut m = miss(skill, d);
                        m.tarocks = tarocks;
                        m.consequence = consequence;
                        if skill == Skill::Announce {
                            m.played = Some(card("T21"));
                        }
                        let mut misses = nothing();
                        if with_miss {
                            misses[skill.index()] = Some((m, n));
                        }
                        let played = Played {
                            variety,
                            ..deal_of(summary(&[]), 20, misses, Facts::default())
                        };
                        let mut before = learner_with_focus(None);
                        before.games = 5;
                        let after = learner_with_focus(Some(skill.index()));
                        let b = compose(&before, &after, &played, true);
                        out.push(b.bullets);
                    }
                }
            }
        }
    }
    out
}

fn learner_with_focus(focus: Option<usize>) -> Learner {
    Learner {
        games: 6,
        skills: vec![
            SkillState {
                mastered: false,
                chances: 20.0,
                met: 10.0,
                cost: 10.0,
                gain: 5.0
            };
            SKILLS
        ],
        just_mastered: vec![],
        focus,
    }
}

#[test]
fn every_key_a_brief_can_hold_has_both_languages_and_the_limits_hold() {
    let table = strings();
    let mut long = Vec::new();
    let mut most = (0, 0);
    for bullets in every_brief() {
        assert!((1..=3).contains(&bullets.len()), "{bullets:?}");
        for lang in ["en", "de"] {
            let lines: Vec<String> = bullets.iter().map(|b| say(&table, lang, b, true)).collect();
            let total: usize = lines.iter().map(|l| count(l)).sum();
            let plain: usize = bullets
                .iter()
                .map(|b| count(&say(&table, lang, b, false)))
                .sum();
            most = (most.0.max(total), most.1.max(plain));
            if total > MAX_BRIEF_WORDS {
                long.push(format!("{lang} {total}: {lines:?}"));
            }
            for (b, l) in bullets.iter().zip(&lines) {
                assert!(!l.contains('{'), "an unfilled slot: {l}");
                // leads and the lesson's text each within their limit
                let text = say_key(&table, lang, b);
                let _ = l;
                let limit = if b.key.starts_with("tutor.miss.") {
                    18
                } else if b.key.starts_with("tutor.rule.") || b.key.starts_with("tutor.fact.") {
                    lessons::MAX_RULE_WORDS
                } else if b.key.starts_with("tutor.try.") {
                    lessons::MAX_TRY_WORDS
                } else if b.key.starts_with("tutor.well.") {
                    12
                } else {
                    20
                };
                if count(&text) > limit {
                    long.push(format!(
                        "{lang} {}: {} words > {limit}: {text}",
                        b.key,
                        count(&text)
                    ));
                }
            }
        }
    }
    long.sort();
    long.dedup();
    assert!(long.is_empty(), "{long:#?}");
    // measured: the longest brief, cards named in full and in one word
    eprintln!(
        "longest focus pair: {} words, {} with a card a word",
        most.0, most.1
    );
}

/// What went well comes first when there is something: the longest of those
/// bullets in front of the longest focus pair is still about 45 words, as
/// the brief is promised (`docs/TUTOR.md` 6).
#[test]
fn a_whole_brief_of_three_bullets_stays_about_forty_five_words() {
    let table = strings();
    let well: Vec<Bullet> = {
        let b = |key: &str| Bullet::new("tutor.lead.well", key);
        let mut out = vec![
            b("tutor.well.announced").with("honour", Param::Text("kings".into())),
            b("tutor.well.ultimo"),
            b("tutor.well.mond").with("trick", Param::Int(24)),
        ];
        for i in 0..SKILLS {
            for j in 0..SKILLS {
                out.push(
                    b("tutor.well.mastered-two")
                        .with("skill", Param::Skill(i))
                        .with("skill2", Param::Skill(j)),
                );
            }
            out.push(
                b("tutor.well.met")
                    .with("chances", Param::Int(12))
                    .with("skill", Param::Skill(i)),
            );
        }
        out
    };
    let mut worst = (0, String::new());
    for pair in every_brief() {
        for lang in ["en", "de"] {
            let pair_words: usize = pair
                .iter()
                .map(|b| count(&say(&table, lang, b, true)))
                .sum();
            for w in &well {
                let total = pair_words + count(&say(&table, lang, w, true));
                if total > worst.0 {
                    worst = (
                        total,
                        format!("{lang}: {} / {pair:?}", say(&table, lang, w, true)),
                    );
                }
            }
        }
    }
    eprintln!("longest whole brief: {} words", worst.0);
    assert!(worst.0 <= MAX_BRIEF_WORDS, "{worst:?}");
}

/// The key's text alone, with its parameters put in.
fn say_key(table: &serde_json::Value, lang: &str, b: &Bullet) -> String {
    let bare = Bullet {
        lead: String::new(),
        ..b.clone()
    };
    say(table, lang, &bare, false)
}

#[test]
fn every_parameter_a_template_names_is_supplied_by_the_brief() {
    let table = strings();
    for bullets in every_brief() {
        for b in &bullets {
            for key in [&b.lead, &b.key] {
                if key.is_empty() {
                    continue;
                }
                for lang in ["en", "de"] {
                    let text = table[key.as_str()][lang].as_str().unwrap();
                    for slot in text
                        .split('{')
                        .skip(1)
                        .map(|r| r.split('}').next().unwrap())
                    {
                        assert!(
                            b.params.iter().any(|(k, _)| k == slot),
                            "{key} {lang}: {{{slot}}} is not supplied in {b:?}"
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn every_key_of_the_brief_is_documented() {
    let doc = include_str!("../../../docs/MESSAGES.md");
    let section = doc.split("\n## Tutor keys").nth(1).expect("section");
    let documented: Vec<&str> = section
        .lines()
        .filter(|l| l.starts_with("| `tutor."))
        .map(|l| l.split('|').nth(1).unwrap().trim().trim_matches('`'))
        .collect();
    for bullets in every_brief() {
        for b in bullets {
            for k in [b.lead.as_str(), b.key.as_str()] {
                assert!(k.is_empty() || documented.contains(&k), "{k}");
            }
        }
    }
    for k in KEYS {
        assert!(documented.contains(k), "{k}");
    }
    // and the other way: every documented key is one the code can say
    let mut said: Vec<String> = every_brief()
        .into_iter()
        .flatten()
        .flat_map(|b| [b.lead, b.key])
        .collect();
    said.extend(lessons::keys().iter().map(|k| k.to_string()));
    said.extend(
        KEYS.iter()
            .chain(PAGE_KEYS)
            .chain(crate::tell::KEYS)
            .map(|k| k.to_string()),
    );
    for k in documented {
        // rule notes are said by the table (web3d/src/main.js), not the brief
        if k.starts_with("tutor.note.") {
            continue;
        }
        assert!(
            said.iter().any(|s| s == k),
            "{k} is documented but never said"
        );
    }
}

#[test]
fn the_learners_ladder_and_constants_can_work() {
    LADDER.check().unwrap();
    PARAMS.check().unwrap();
}

#[test]
fn nobody_is_named_in_a_brief() {
    // the kindness rule, on the English and German of every tutor key
    let table = strings();
    let bad = [
        "stupid",
        "foolish",
        "blunder",
        "mistake",
        "careless",
        "bad player",
        "fool",
        "lazy",
        "dumm",
        "blöd",
        "Fehler",
        "Anfänger",
        "schlecht",
    ];
    for (key, v) in table.as_object().unwrap() {
        if !key.starts_with("tutor.") {
            continue;
        }
        for lang in ["en", "de"] {
            let text = v[lang].as_str().unwrap().to_lowercase();
            for w in bad {
                assert!(!text.contains(&w.to_lowercase()), "{key} {lang}: {text}");
            }
        }
    }
}

// ---- played deals -----------------------------------------------------------

fn config() -> Config {
    Config::default()
}

/// The records of the scored deals of a short sitting: seat A takes rung 4's
/// part with a different seed than seat B's, so both play a sound game.
fn deals_of(seed: u64, n: usize) -> Vec<DealRecord> {
    let mut a = Opponent::new(Level::from_halves(8), seed).with_config(config());
    let mut b = Opponent::new(Level::from_halves(8), seed + 1000).with_config(config());
    let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
    let mut out = Vec::new();
    let mut counted = 0;
    while let Some(seat) = s.to_move() {
        let v = sitting_view(&s, seat);
        let act = if matches!(v.legal_actions.first(), Some(Action::Extend { .. })) {
            Action::Extend { accept: false }
        } else if seat == Seat::A {
            a.choose(&v).unwrap()
        } else {
            b.choose(&v).unwrap()
        };
        s.apply(seat, act).unwrap();
        if s.deals_counted() > counted {
            counted = s.deals_counted();
            out.push(s.last_deal().unwrap().clone());
            if out.len() == n {
                break;
            }
        }
    }
    out
}

#[test]
fn a_played_deals_brief_is_pure_short_and_made_of_known_keys() {
    let recs = deals_of(5, 3);
    assert_eq!(recs.len(), 3);
    let budget = Budget::default();
    let table = strings();
    for rec in &recs {
        let one = brief_of_deal(rec, Seat::A, &[], &[], 3.0, budget).unwrap();
        let two = brief_of_deal(rec, Seat::A, &[], &[], 3.0, budget).unwrap();
        assert_eq!(one, two, "the same record gives the same brief");
        assert!((1..=3).contains(&one.bullets.len()), "{one:?}");
        for b in &one.bullets {
            for k in [&b.lead, &b.key] {
                assert!(k.is_empty() || table[k.as_str()]["en"].is_string(), "{k}");
            }
        }
        // the summary is the evidence's
        let e = crate::tutor::evidence(rec, Seat::A, &[], budget).unwrap();
        assert_eq!(one.summary, e.summary);
    }
}

#[test]
fn a_brief_after_a_history_counts_the_history_and_leaves_it_alone() {
    let recs = deals_of(6, 3);
    let budget = Budget::default();
    let hist: Vec<Summary> = recs[..2]
        .iter()
        .map(|r| {
            crate::tutor::evidence(r, Seat::A, &[], budget)
                .unwrap()
                .summary
        })
        .collect();
    let with = brief_of_deal(&recs[2], Seat::A, &[], &hist, 3.0, budget).unwrap();
    let without = brief_of_deal(&recs[2], Seat::A, &[], &[], 3.0, budget).unwrap();
    assert_eq!(with.summary, without.summary);
    assert!(with.bullets.len() <= 3);
}

#[test]
fn a_sitting_brief_is_made_of_what_was_kept_and_sums_its_deals() {
    let recs = deals_of(7, 3);
    let budget = Budget::default();
    let details: Vec<Detail> = recs
        .iter()
        .map(|r| detail_of(r, Seat::A, &[], budget).unwrap())
        .collect();
    let deals: Vec<SittingDeal> = details
        .iter()
        .enumerate()
        .map(|(i, d)| SittingDeal {
            number: i as u32 + 1,
            detail: d,
        })
        .collect();
    let b = brief_of_sitting(&deals, &[], 3.0).unwrap();
    assert!(b.detail.is_none());
    let singles: Vec<Summary> = details.iter().map(|d| d.summary.clone()).collect();
    for i in 0..SKILLS {
        let sum = singles.iter().fold(Tally::default(), |t, s| Tally {
            chances: t.chances + s.of(i).chances,
            met: t.met + s.of(i).met,
            missed: t.missed + s.of(i).missed,
        });
        assert_eq!(b.summary.of(i), sum, "{}", Skill::ALL[i].slug());
    }
    assert!((1..=3).contains(&b.bullets.len()));
    for x in &b.bullets {
        if x.lead == "tutor.lead.deal-n" {
            let Param::Int(n) = param(x, "deal") else {
                panic!()
            };
            assert!((1..=3).contains(n));
        }
        assert_ne!(x.lead, "tutor.lead.deal", "a sitting names the deal");
    }
}

#[test]
fn a_mastery_is_announced_in_the_brief_of_its_deal_not_again_in_the_sitting() {
    let d = |s: Summary| Detail {
        summary: s,
        decisions: 20,
        facts: Facts::default(),
        misses: Default::default(),
        settings: Settings::default(),
        variety: 0,
    };
    let early = vec![
        summary(&[(Skill::Worth, 1, 1, 0.0)]),
        summary(&[(Skill::Worth, 1, 1, 0.0)]),
    ];
    let clears = d(summary(&[(Skill::Worth, 12, 12, 0.0)]));
    // told as a deal it is announced ...
    let b = brief_from(&[(0, &clears)], &early, 3.0, false);
    assert_eq!(b.bullets[0].key, "tutor.well.mastered", "{b:?}");
    // ... and not again as part of the sitting
    let deals = [SittingDeal {
        number: 3,
        detail: &clears,
    }];
    let s = brief_of_sitting(&deals, &early, 3.0).unwrap();
    assert!(
        s.bullets
            .iter()
            .all(|x| !x.key.starts_with("tutor.well.mastered")),
        "{s:?}"
    );
    assert_eq!(brief_of_sitting(&[], &early, 3.0), None);
}

#[test]
fn until_a_rule_passes_every_focus_is_told_as_its_fact() {
    let history = bad_at_worth(4);
    let mut m = nothing();
    m[0] = Some((miss(Skill::Worth, Some(Direction::Greedy)), 0));
    let played = Played {
        told: lessons::TOLD,
        ..deal_of(
            summary(&[(Skill::Worth, 6, 2, 60.0)]),
            12,
            m,
            Facts::default(),
        )
    };
    let b = made(&history, &played, true);
    assert_eq!(b.bullets[0].key, "tutor.fact.worth", "{b:?}");
    assert_eq!(
        b.bullets[1].key, "tutor.miss.worth.poor",
        "the example is told"
    );
}

#[test]
fn an_unfinished_deal_is_an_error_not_a_brief() {
    let mut rec = deals_of(8, 1).remove(0);
    rec.actions.truncate(rec.actions.len() / 2);
    assert!(brief_of_deal(&rec, Seat::A, &[], &[], 3.0, Budget::default()).is_err());
}
