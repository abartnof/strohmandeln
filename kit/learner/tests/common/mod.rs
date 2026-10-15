//! Cassino's skill ladder and constants, written out as a [`Ladder`] and
//! [`Params`]: the fixture of the ported tests and of the equivalence test.
//! Values: `cassino-core/src/{tutor,learner}.rs` @ 73f0cd7 (`Skill::needs`,
//! `top_rate`, `top_cost`, and the constants).
#![allow(dead_code)]

use kit_learner::{Ladder, Params, SkillSpec, Summary, Tally};

pub const PAIRS: usize = 0;
pub const SUMS: usize = 1;
pub const BUILDING: usize = 2;
pub const SAFE_BUILDS: usize = 3;
pub const ANSWERING_BUILDS: usize = 4;
pub const NO_SWEEP: usize = 5;
pub const VALUABLES: usize = 6;
pub const TRAILING: usize = 7;

const fn spec(
    slug: &'static str,
    needs: &'static [usize],
    top_rate: f64,
    top_cost: f64,
) -> SkillSpec {
    SkillSpec {
        slug,
        needs,
        top_rate,
        top_cost,
        min_chances: 4.0,
    }
}

pub static SKILLS: [SkillSpec; 8] = [
    spec("pairs", &[], 0.974, 0.10),
    spec("sums", &[PAIRS], 0.965, 0.07),
    spec("building", &[SUMS], 0.938, 0.22),
    spec("safe-builds", &[BUILDING], 0.966, 0.10),
    spec("answering-builds", &[SUMS], 1.0, 0.0),
    spec("sweeps", &[], 0.956, 0.09),
    spec("valuables", &[], 0.994, 0.04),
    spec("trailing", &[], 0.949, 0.46),
];

pub static LADDER: Ladder = Ladder {
    name: "cassino",
    version: 1,
    skills: &SKILLS,
};

/// Written out, not taken from `Params::CASSINO`, so that a change to the
/// crate's defaults cannot move the fixture.
pub const PARAMS: Params = Params {
    discount: 0.8,
    min_gain: 0.5,
    costly_gain: 2.0,
    costly_chances: 8.0,
    lower: 0.2,
    upper: 0.9,
    master_of_top: 0.85,
    lose_below: 0.2,
};

/// A summary with these `(skill, chances, met, missed)` lines, the rest empty.
pub fn summary(lines: &[(usize, u32, u32, f64)]) -> Summary {
    let mut s = Summary::new(&LADDER);
    for &(skill, chances, met, missed) in lines {
        s.tallies[skill] = Tally {
            chances,
            met,
            missed,
        };
    }
    s
}

/// A deal of `chances` at `skill`, `met` of them met, each miss costing `gap`.
pub fn game(skill: usize, chances: u32, met: u32, gap: f64) -> Summary {
    summary(&[(skill, chances, met, f64::from(chances - met) * gap)])
}
