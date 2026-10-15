//! What a ladder and the constants must satisfy to be learnt from.

use kit_learner::{Ladder, SkillSpec};

const fn s(slug: &'static str, needs: &'static [usize]) -> SkillSpec {
    SkillSpec {
        slug,
        needs,
        top_rate: 0.9,
        top_cost: 0.1,
        min_chances: 4.0,
        may_focus: true,
    }
}

fn ladder(skills: &'static [SkillSpec]) -> Ladder {
    Ladder {
        name: "game",
        version: 1,
        skills,
    }
}

#[test]
fn a_cycle_in_needs_is_refused() {
    static TWO: [SkillSpec; 2] = [s("a", &[1]), s("b", &[0])];
    static LONGER: [SkillSpec; 4] = [s("a", &[]), s("b", &[2]), s("c", &[3]), s("d", &[1])];
    // The message names the cycle, from its first skill in ladder order.
    assert_eq!(
        ladder(&TWO).check(),
        Err("prerequisites form a cycle: a -> b -> a".to_string())
    );
    // `a` is off the cycle and not named.
    assert_eq!(
        ladder(&LONGER).check(),
        Err("prerequisites form a cycle: b -> c -> d -> b".to_string())
    );
}

#[test]
fn a_skill_is_seen_from_exactly_its_min_chances() {
    let spec = s("a", &[]);
    assert!(spec.seen(4.0));
    assert!(spec.seen(7.5));
    assert!(!spec.seen(3.999));
}

#[test]
fn shared_prerequisites_are_not_a_cycle_but_a_repeated_one_is_a_slip() {
    static DIAMOND: [SkillSpec; 4] = [s("a", &[]), s("b", &[0]), s("c", &[0]), s("d", &[1, 2])];
    assert_eq!(ladder(&DIAMOND).check(), Ok(()));
    static TWICE: [SkillSpec; 2] = [s("a", &[]), s("b", &[0, 0])];
    assert!(ladder(&TWICE).check().is_err());
}
