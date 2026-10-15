//! The learner: the fold over the deals, mastery with hysteresis, the focus.

use crate::beta::beta_quantile;
use crate::{Ladder, Params, Summary};

/// What the deals say of one skill.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SkillState {
    pub mastered: bool,
    /// Discounted chances and chances met.
    pub chances: f64,
    pub met: f64,
    /// Discounted loss a deal to this skill, in the game's score unit.
    pub cost: f64,
    /// What mastering it would gain: the cost less the skill's `top_cost`,
    /// at least nothing. What the focus is chosen on.
    pub gain: f64,
}

/// What the deals say, after the latest.
#[derive(Clone, Debug, PartialEq)]
pub struct Learner {
    /// How many deals (summaries of the present version) it was learnt from.
    pub games: usize,
    /// In ladder order.
    pub skills: Vec<SkillState>,
    /// The skills (indexes) that became mastered with the latest deal.
    pub just_mastered: Vec<usize>,
    /// The skill to work on next.
    pub focus: Option<usize>,
}

impl Learner {
    pub fn state(&self, skill: usize) -> SkillState {
        self.skills[skill]
    }

    pub fn mastered(&self, skill: usize) -> bool {
        self.skills[skill].mastered
    }
}

/// The learner after `games`, oldest first. Summaries of another version of
/// the evidence (see [`crate::stale`]) are skipped: a client recomputes them
/// first. The ladder and the constants must pass [`Ladder::check`] and
/// [`Params::check`]: this asserts both in debug builds (and does not in
/// release, where a game calls them once at start).
///
/// **Mastery.** Each deal's counts are weighed `discount` times as much as
/// the deal after it. The met rate has a Beta(1, 1) prior, so after
/// discounted `c` chances and `m` met its posterior is Beta(1 + m, 1 + c - m).
/// A skill becomes mastered when the `lower` quantile of the posterior
/// reaches [`Params::master`], and once mastered is lost only when the
/// `upper` quantile falls below [`Params::lose`]. Between the two the state
/// stays as it was, so progress does not flicker. With fewer than the
/// skill's `min_chances` discounted chances nothing changes.
///
/// **Focus.** Of the skills not mastered that may be the focus
/// ([`crate::SkillSpec::may_focus`]), whose prerequisites are *cleared*
/// (mastered, a skill that may not be the focus, or gaining less than `min_gain`, which covers a skill whose
/// chances do not come up, or not shown to be short), and whose met rate is
/// surely short of mastery (the `upper` quantile below [`Params::master`]),
/// or which costs a great deal (a gain of `costly_gain` over `costly_chances`
/// chances), the one with the largest gain: what a deal its misses cost
/// (discounted) less its `top_cost`. Exact ties go to the earlier skill
/// in the ladder. `None` when no skill gains `min_gain`.
pub fn learn(ladder: &Ladder, p: &Params, games: &[Summary]) -> Learner {
    debug_assert!(ladder.check().is_ok(), "{:?}", ladder.check());
    debug_assert!(p.check().is_ok(), "{:?}", p.check());
    let n = ladder.skills.len();
    let mut mastered = vec![false; n];
    let mut before = vec![false; n];
    let (mut chances, mut met, mut missed) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    let mut weight = 0.0;
    let current = || games.iter().filter(|g| g.is_current(ladder));
    for g in current() {
        before.clone_from(&mastered);
        weight = weight * p.discount + 1.0;
        for (i, spec) in ladder.skills.iter().enumerate() {
            let t = g.of(i);
            chances[i] = chances[i] * p.discount + f64::from(t.chances);
            met[i] = met[i] * p.discount + f64::from(t.met);
            missed[i] = missed[i] * p.discount + t.missed;
            if !spec.seen(chances[i]) {
                continue;
            }
            let (a, b) = (1.0 + met[i], 1.0 + chances[i] - met[i]);
            if mastered[i] {
                if beta_quantile(p.upper, a, b) < p.lose(spec) {
                    mastered[i] = false;
                }
            } else if beta_quantile(p.lower, a, b) >= p.master(spec) {
                mastered[i] = true;
            }
        }
    }
    let skills: Vec<SkillState> = ladder
        .skills
        .iter()
        .enumerate()
        .map(|(i, spec)| {
            let cost = if weight > 0.0 {
                missed[i] / weight
            } else {
                0.0
            };
            SkillState {
                mastered: mastered[i],
                chances: chances[i],
                met: met[i],
                cost,
                gain: (cost - spec.top_cost).max(0.0),
            }
        })
        .collect();
    let seen = |i: usize| ladder.skills[i].seen(skills[i].chances);
    // Surely short: enough chances, and the rate shown below mastery.
    let short = |i: usize| {
        seen(i)
            && beta_quantile(p.upper, 1.0 + met[i], 1.0 + chances[i] - met[i])
                < p.master(&ladder.skills[i])
    };
    // Cleared: mastered, or costing too little to matter, or not shown to be
    // short (a prerequisite half-proven either way must not hold back its
    // dependents).
    let cleared = |i: usize| {
        mastered[i] || !ladder.skills[i].may_focus || skills[i].gain < p.min_gain || !short(i)
    };
    let open = |i: usize| {
        !mastered[i]
            && ladder.skills[i].may_focus
            && skills[i].gain >= p.min_gain
            && ladder.skills[i].needs.iter().all(|&need| cleared(need))
    };
    // Eligible: surely short by rate, or costing a great deal over enough
    // chances although the rate is not shown short (a rate of 0.8 at a
    // skill that costs a great deal a miss).
    let eligible = |i: usize| {
        open(i)
            && (short(i)
                || (skills[i].chances >= p.costly_chances && skills[i].gain >= p.costly_gain))
    };
    // The earlier skill keeps an exact tie.
    let mut focus: Option<usize> = None;
    for i in (0..n).filter(|&i| eligible(i)) {
        if focus.is_none_or(|f| skills[i].gain.total_cmp(&skills[f].gain).is_gt()) {
            focus = Some(i);
        }
    }
    let just_mastered = (0..n).filter(|&i| mastered[i] && !before[i]).collect();
    Learner {
        games: current().count(),
        skills,
        just_mastered,
        focus,
    }
}
