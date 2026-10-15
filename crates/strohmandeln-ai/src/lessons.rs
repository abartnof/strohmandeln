//! What the tutor says of each skill: its name, its rule, its fact for when
//! no rule applies, its two tries and its nudge, all as message keys
//! (`docs/TUTOR.md` 9, `docs/MESSAGES.md` "Tutor keys"). The words are the
//! table's; the sources of each rule are in `research/16`.
//!
//! **Rules are claims, and are provisional.** A rule ships only if it passes
//! its measurement (`docs/TUTOR.md` 11, M5: how often the strongest rung's
//! choice matches when the rule's trigger fires, a lower bound of 0.80
//! confirmed on fresh seeds). That has not been run, so every rule is
//! [`Gate::Provisional`] and **not told**: the skill's fact is, in its place
//! ([`TOLD`]; decision of 15 Oct 2026, `docs/TUTOR.md` 6). M5 sets each
//! rule's gate to [`Gate::Passed`] (told) or [`Gate::Failed`] (never told)
//! and the settings it was confirmed under ([`Rule::under`]); until then the
//! `bid` and `pagat` rules are bounded to the default settings.

use crate::tutor::{Direction, Skill, SKILLS};
use strohmandeln_core::types::Settings;

/// Whether a rule has been measured.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Gate {
    /// Not yet measured: told, as the books' advice.
    Provisional,
    /// Measured and passed.
    Passed,
    /// Measured and failed: never told.
    Failed,
}

/// What a rule is told for: the facts of a missed chance (or none, when the
/// focus came from the history alone).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Context {
    /// The direction of the deal's costliest miss at the skill.
    pub direction: Option<Direction>,
    /// The tarocks the player held at that miss (`pagat`, `bid`).
    pub tarocks: u32,
}

/// One rule of a skill.
#[derive(Copy, Clone, Debug)]
pub struct Rule {
    /// The message key; shown after `tutor.lead.rule`.
    pub key: &'static str,
    pub gate: Gate,
    /// The rule speaks to this miss (its trigger, `docs/TUTOR.md` 4).
    pub applies: fn(&Context) -> bool,
    /// The sitting's settings the rule was confirmed under.
    pub under: fn(&Settings) -> bool,
}

/// What the tutor says of one skill.
#[derive(Copy, Clone, Debug)]
pub struct Lesson {
    pub skill: Skill,
    pub name: &'static str,
    /// The rules, at most one of which is told.
    pub rules: &'static [Rule],
    /// Told when no rule applies.
    pub fact: &'static str,
    pub tries: [&'static str; 2],
    pub nudge: &'static str,
}

/// The words of a rule and a fact: at most this many.
pub const MAX_RULE_WORDS: usize = 15;
pub const MAX_TRY_WORDS: usize = 14;
pub const MAX_NUDGE_WORDS: usize = 10;
/// A skill's name: it is the lead's `{skill}` and the praise's.
pub const MAX_NAME_WORDS: usize = 8;

fn any(_: &Context) -> bool {
    true
}
fn anywhere(_: &Settings) -> bool {
    true
}
fn toward(d: Direction) -> impl Fn(&Context) -> bool {
    move |c| c.direction == Some(d)
}
// `fn` pointers, not closures, so that the table can be a constant.
fn timid(c: &Context) -> bool {
    toward(Direction::Timid)(c)
}
fn greedy(c: &Context) -> bool {
    toward(Direction::Greedy)(c)
}
fn pip(c: &Context) -> bool {
    toward(Direction::Pip)(c)
}
fn figure(c: &Context) -> bool {
    toward(Direction::Figure)(c)
}
/// `pagat`: the sources agree at seven tarocks or fewer (U 74, MS) ...
fn few_tarocks(c: &Context) -> bool {
    c.direction == Some(Direction::Cash) && c.tarocks <= 7
}
/// ... and at twelve or more (MS; U says ten to twelve); between them they
/// differ and the fact is told (`research/16`, 3.4).
fn many_tarocks(c: &Context) -> bool {
    c.direction == Some(Direction::Keep) && c.tarocks >= 12
}
/// `bid`: the rule says when to take up, so it speaks to a pass.
fn passed(c: &Context) -> bool {
    toward(Direction::Passed)(c)
}

const fn rule(key: &'static str, applies: fn(&Context) -> bool) -> Rule {
    Rule {
        key,
        gate: Gate::Provisional,
        applies,
        under: anywhere,
    }
}

/// A rule the sources state for the standard game only (`bid`, `pagat`): not
/// told under another setting until M5 has checked it there.
const fn standard_rule(key: &'static str, applies: fn(&Context) -> bool) -> Rule {
    Rule {
        under: standard,
        ..rule(key, applies)
    }
}

fn standard(s: &Settings) -> bool {
    *s == Settings::default()
}

/// The gates a rule may have to be told. A rule that has not been measured
/// is not told: its skill's fact is (`docs/TUTOR.md` 6: "where no rule has
/// passed its gate, one plain fact in its place"). M5 sets the gates.
pub const TOLD: &[Gate] = &[Gate::Passed];

/// The ten lessons, in ladder order.
pub const LESSONS: [Lesson; SKILLS] = [
    Lesson {
        skill: Skill::Worth,
        name: "tutor.name.worth",
        rules: &[
            rule("tutor.rule.worth.rich", timid),
            rule("tutor.rule.worth.poor", greedy),
        ],
        fact: "tutor.fact.worth",
        tries: ["tutor.try.worth.1", "tutor.try.worth.2"],
        nudge: "tutor.nudge.worth",
    },
    Lesson {
        skill: Skill::Cheap,
        name: "tutor.name.cheap",
        rules: &[rule("tutor.rule.cheap", any)],
        fact: "tutor.fact.cheap",
        tries: ["tutor.try.cheap.1", "tutor.try.cheap.2"],
        nudge: "tutor.nudge.cheap",
    },
    Lesson {
        skill: Skill::Off,
        name: "tutor.name.off",
        rules: &[
            rule("tutor.rule.off.pip", pip),
            rule("tutor.rule.off.figure", figure),
        ],
        fact: "tutor.fact.off",
        tries: ["tutor.try.off.1", "tutor.try.off.2"],
        nudge: "tutor.nudge.off",
    },
    Lesson {
        skill: Skill::Run,
        name: "tutor.name.run",
        rules: &[rule("tutor.rule.run", any)],
        fact: "tutor.fact.run",
        tries: ["tutor.try.run.1", "tutor.try.run.2"],
        nudge: "tutor.nudge.run",
    },
    Lesson {
        skill: Skill::Pagat,
        name: "tutor.name.pagat",
        rules: &[
            standard_rule("tutor.rule.pagat.cash", few_tarocks),
            standard_rule("tutor.rule.pagat.keep", many_tarocks),
        ],
        fact: "tutor.fact.pagat",
        tries: ["tutor.try.pagat.1", "tutor.try.pagat.2"],
        nudge: "tutor.nudge.pagat",
    },
    Lesson {
        skill: Skill::Void,
        name: "tutor.name.void",
        rules: &[rule("tutor.rule.void", any)],
        fact: "tutor.fact.void",
        tries: ["tutor.try.void.1", "tutor.try.void.2"],
        nudge: "tutor.nudge.void",
    },
    // `behind` has no rule: Werner says none can be given and Bermann has a
    // case where the leader wins (`research/16`, 3.1).
    Lesson {
        skill: Skill::Behind,
        name: "tutor.name.behind",
        rules: &[],
        fact: "tutor.fact.behind",
        tries: ["tutor.try.behind.1", "tutor.try.behind.2"],
        nudge: "tutor.nudge.behind",
    },
    Lesson {
        skill: Skill::Endgame,
        name: "tutor.name.endgame",
        rules: &[rule("tutor.rule.endgame", any)],
        fact: "tutor.fact.endgame",
        tries: ["tutor.try.endgame.1", "tutor.try.endgame.2"],
        nudge: "tutor.nudge.endgame",
    },
    Lesson {
        skill: Skill::Bid,
        name: "tutor.name.bid",
        rules: &[standard_rule("tutor.rule.bid", passed)],
        fact: "tutor.fact.bid",
        tries: ["tutor.try.bid.1", "tutor.try.bid.2"],
        nudge: "tutor.nudge.bid",
    },
    Lesson {
        skill: Skill::Announce,
        name: "tutor.name.announce",
        rules: &[rule("tutor.rule.announce", any)],
        fact: "tutor.fact.announce",
        tries: ["tutor.try.announce.1", "tutor.try.announce.2"],
        nudge: "tutor.nudge.announce",
    },
];

pub fn lesson(skill: Skill) -> &'static Lesson {
    &LESSONS[skill.index()]
}

/// The first of `rules` whose gate is among `told`, that applies to `ctx` and
/// was confirmed under `settings`. A failed rule is never told, whatever
/// `told` says.
pub fn pick<'a>(
    rules: &'a [Rule],
    ctx: &Context,
    settings: &Settings,
    told: &[Gate],
) -> Option<&'a Rule> {
    rules.iter().find(|r| {
        r.gate != Gate::Failed && told.contains(&r.gate) && (r.applies)(ctx) && (r.under)(settings)
    })
}

/// The rule told for `skill` at a miss like `ctx` under `settings`, if one
/// applies; otherwise the skill's fact is told (`Lesson::fact`). `told` is
/// [`TOLD`] outside the tests.
pub fn taught(
    skill: Skill,
    ctx: &Context,
    settings: &Settings,
    told: &[Gate],
) -> Option<&'static Rule> {
    pick(lesson(skill).rules, ctx, settings, told)
}

/// Every key the lessons carry, in document order: names, rules, facts,
/// tries, nudges.
pub fn keys() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = LESSONS.iter().map(|l| l.name).collect();
    out.extend(LESSONS.iter().flat_map(|l| l.rules.iter().map(|r| r.key)));
    out.extend(LESSONS.iter().map(|l| l.fact));
    out.extend(LESSONS.iter().flat_map(|l| l.tries));
    out.extend(LESSONS.iter().map(|l| l.nudge));
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tutor::Skill;

    fn ctx(direction: Option<Direction>, tarocks: u32) -> Context {
        Context { direction, tarocks }
    }
    /// What would be told were the provisional rules admitted.
    const ADMIT: &[Gate] = &[Gate::Provisional, Gate::Passed];
    fn told(skill: Skill, c: &Context) -> Option<&'static str> {
        taught(skill, c, &Settings::default(), ADMIT).map(|r| r.key)
    }

    #[test]
    fn the_lessons_are_in_ladder_order_with_slug_keys() {
        for (i, l) in LESSONS.iter().enumerate() {
            assert_eq!(l.skill.index(), i);
            let slug = l.skill.slug();
            assert_eq!(l.name, format!("tutor.name.{slug}"));
            assert_eq!(l.fact, format!("tutor.fact.{slug}"));
            assert_eq!(l.tries[0], format!("tutor.try.{slug}.1"));
            assert_eq!(l.tries[1], format!("tutor.try.{slug}.2"));
            assert_eq!(l.nudge, format!("tutor.nudge.{slug}"));
            for r in l.rules {
                assert!(
                    r.key.starts_with(&format!("tutor.rule.{slug}")),
                    "{}",
                    r.key
                );
            }
        }
    }

    #[test]
    fn every_rule_is_provisional_until_the_measurement_sets_it() {
        for l in &LESSONS {
            for r in l.rules {
                assert_eq!(r.gate, Gate::Provisional, "{}", r.key);
            }
        }
    }

    #[test]
    fn until_a_rule_passes_its_measurement_its_fact_is_told() {
        // docs/TUTOR.md 6 and 9: only a rule that passed its gate is told.
        assert_eq!(TOLD, &[Gate::Passed]);
        let s = Settings::default();
        for l in &LESSONS {
            for direction in [
                None,
                Some(Direction::Greedy),
                Some(Direction::Timid),
                Some(Direction::Figure),
                Some(Direction::Pip),
                Some(Direction::Cash),
                Some(Direction::Keep),
                Some(Direction::Passed),
            ] {
                for tarocks in [3, 9, 14] {
                    let c = ctx(direction, tarocks);
                    assert!(
                        taught(l.skill, &c, &s, TOLD).is_none(),
                        "{}",
                        l.skill.slug()
                    );
                }
            }
        }
    }

    #[test]
    fn a_rule_is_told_only_if_it_passed_and_never_if_it_failed() {
        let mut rules = [rule("a", any), rule("b", any)];
        let s = Settings::default();
        let c = ctx(None, 0);
        assert!(
            pick(&rules, &c, &s, TOLD).is_none(),
            "provisional is not told"
        );
        assert_eq!(pick(&rules, &c, &s, ADMIT).unwrap().key, "a");
        rules[0].gate = Gate::Failed;
        assert_eq!(pick(&rules, &c, &s, ADMIT).unwrap().key, "b");
        rules[1].gate = Gate::Failed;
        assert!(pick(&rules, &c, &s, ADMIT).is_none());
        rules[1].gate = Gate::Passed;
        assert_eq!(pick(&rules, &c, &s, TOLD).unwrap().key, "b");
        rules[1].gate = Gate::Failed;
        let all = [Gate::Provisional, Gate::Passed, Gate::Failed];
        assert!(pick(&rules, &c, &s, &all).is_none(), "failed is never told");
    }

    #[test]
    fn the_bid_and_pagat_rules_are_told_only_under_the_default_settings() {
        // The sources state them for the standard game; no other setting has
        // been checked (M5).
        let two = Settings {
            ultimo: 2,
            ..Settings::default()
        };
        let cash = ctx(Some(Direction::Cash), 5);
        let bid = ctx(Some(Direction::Passed), 6);
        assert!(taught(Skill::Pagat, &cash, &Settings::default(), ADMIT).is_some());
        assert!(taught(Skill::Pagat, &cash, &two, ADMIT).is_none());
        assert!(taught(Skill::Bid, &bid, &Settings::default(), ADMIT).is_some());
        assert!(taught(Skill::Bid, &bid, &two, ADMIT).is_none());
        // the rest do not depend on them
        assert!(taught(Skill::Cheap, &ctx(None, 0), &two, ADMIT).is_some());
    }

    #[test]
    fn a_rule_is_told_only_under_the_settings_it_was_confirmed_under() {
        fn one_point(s: &Settings) -> bool {
            s.ultimo == 1
        }
        let r = [Rule {
            under: one_point,
            ..rule("a", any)
        }];
        let two = Settings {
            ultimo: 2,
            ..Settings::default()
        };
        assert!(pick(&r, &ctx(None, 0), &Settings::default(), ADMIT).is_some());
        assert!(pick(&r, &ctx(None, 0), &two, ADMIT).is_none());
    }

    #[test]
    fn worth_is_told_by_the_direction_of_the_miss_and_not_without_one() {
        assert_eq!(
            told(Skill::Worth, &ctx(Some(Direction::Timid), 0)),
            Some("tutor.rule.worth.rich")
        );
        assert_eq!(
            told(Skill::Worth, &ctx(Some(Direction::Greedy), 0)),
            Some("tutor.rule.worth.poor")
        );
        assert_eq!(told(Skill::Worth, &ctx(None, 0)), None);
    }

    #[test]
    fn off_is_told_by_the_kind_of_the_better_cover() {
        assert_eq!(
            told(Skill::Off, &ctx(Some(Direction::Figure), 0)),
            Some("tutor.rule.off.figure")
        );
        assert_eq!(
            told(Skill::Off, &ctx(Some(Direction::Pip), 0)),
            Some("tutor.rule.off.pip")
        );
    }

    #[test]
    fn the_pagat_rule_is_told_only_where_the_sources_agree() {
        let cash = |n| told(Skill::Pagat, &ctx(Some(Direction::Cash), n));
        let keep = |n| told(Skill::Pagat, &ctx(Some(Direction::Keep), n));
        assert_eq!(cash(7), Some("tutor.rule.pagat.cash"));
        assert_eq!(cash(3), Some("tutor.rule.pagat.cash"));
        assert_eq!(cash(8), None, "U says 7 to 8, MS 7 or fewer");
        assert_eq!(keep(12), Some("tutor.rule.pagat.keep"));
        assert_eq!(keep(11), None, "between the sources: the fact");
        assert_eq!(keep(7), None, "the direction must fit the zone");
        assert_eq!(cash(12), None);
        assert_eq!(told(Skill::Pagat, &ctx(None, 5)), None);
    }

    #[test]
    fn behind_has_no_rule_and_bid_only_speaks_to_a_pass() {
        for d in [None, Some(Direction::Cash), Some(Direction::Pip)] {
            assert_eq!(told(Skill::Behind, &ctx(d, 5)), None);
        }
        assert_eq!(
            told(Skill::Bid, &ctx(Some(Direction::Passed), 6)),
            Some("tutor.rule.bid")
        );
        assert_eq!(told(Skill::Bid, &ctx(Some(Direction::Took), 6)), None);
        assert_eq!(told(Skill::Bid, &ctx(None, 6)), None);
    }

    #[test]
    fn the_single_rule_skills_are_told_with_or_without_a_miss() {
        for (skill, key) in [
            (Skill::Cheap, "tutor.rule.cheap"),
            (Skill::Run, "tutor.rule.run"),
            (Skill::Void, "tutor.rule.void"),
            (Skill::Endgame, "tutor.rule.endgame"),
            (Skill::Announce, "tutor.rule.announce"),
        ] {
            assert_eq!(told(skill, &ctx(None, 0)), Some(key));
        }
    }

    // ---- the words: both languages, the limits, the document --------------

    fn strings() -> serde_json::Value {
        serde_json::from_str(include_str!("../../../web3d/strings.json")).unwrap()
    }

    /// The words of a template: a `{slot}` counts as one word.
    fn words(s: &str) -> usize {
        s.split_whitespace().count()
    }

    #[test]
    fn every_key_is_in_both_languages_within_its_word_limit() {
        let table = strings();
        let mut long = Vec::new();
        for l in &LESSONS {
            let mut check = |key: &str, max: usize| {
                for lang in ["en", "de"] {
                    let text = table[key][lang]
                        .as_str()
                        .unwrap_or_else(|| panic!("{key} {lang}"));
                    if words(text) > max {
                        long.push(format!("{key} {lang}: {} words: {text}", words(text)));
                    }
                }
            };
            check(l.name, MAX_NAME_WORDS);
            check(l.fact, MAX_RULE_WORDS);
            for r in l.rules {
                check(r.key, MAX_RULE_WORDS);
            }
            for t in l.tries {
                check(t, MAX_TRY_WORDS);
            }
            check(l.nudge, MAX_NUDGE_WORDS);
        }
        assert!(long.is_empty(), "{long:#?}");
    }

    #[test]
    fn the_keys_of_the_lessons_are_the_keys_of_the_document() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let section = doc.split("\n## Tutor keys").nth(1).expect("section");
        let documented: Vec<&str> = section
            .lines()
            .filter(|l| l.starts_with("| `tutor."))
            .map(|l| l.split('|').nth(1).unwrap().trim().trim_matches('`'))
            .collect();
        for k in keys() {
            assert!(documented.contains(&k), "{k} is not in docs/MESSAGES.md");
        }
        let lessons = keys();
        for k in documented {
            let owned = lessons.contains(&k)
                || k.starts_with("tutor.lead.")
                || k.starts_with("tutor.well.")
                || k.starts_with("tutor.none.")
                || k.starts_with("tutor.miss.")
                || ["tutor.short", "tutor.how", "tutor.method"].contains(&k);
            assert!(owned, "{k} is documented but nothing carries it");
        }
    }
}
