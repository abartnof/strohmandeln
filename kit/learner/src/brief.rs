//! The shape of the brief: what the person reads after a deal, "a few bullet
//! points at most". The words are the game's; this module decides which
//! bullets there are and in what order.
//!
//! 1. What went well, if anything: a skill just mastered, else the best of
//!    the deal's strengths, in one bullet.
//! 2. The focus: the game's own bullets for the skill to work on (its rule,
//!    this deal's example), as many as fit.
//! 3. Instead of 2, when there is no focus: that nothing stood out (with the
//!    next opponent up, if there is one), or that it is too early to say.
//!
//! Never praise and criticise the same skill; never more than
//! [`MAX_BULLETS`]; a short deal says so in one bullet.

use crate::{Ladder, Learner};

/// The most bullets a brief has.
pub const MAX_BULLETS: usize = 3;

/// A value a message template takes.
#[derive(Clone, Debug, PartialEq)]
pub enum Param {
    Int(i64),
    Num(f64),
    Text(String),
    /// A skill, by its index in the ladder; the table names it.
    Skill(usize),
}

/// One bullet: message keys with parameters, never prose, so that the table
/// can say it in any language. `lead` is the key of the bold lead (empty for
/// none), `key` the key of the rest.
#[derive(Clone, Debug, PartialEq)]
pub struct Bullet {
    pub lead: String,
    pub key: String,
    pub params: Vec<(String, Param)>,
}

impl Bullet {
    pub fn new(lead: &str, key: &str) -> Bullet {
        Bullet {
            lead: lead.into(),
            key: key.into(),
            params: Vec::new(),
        }
    }

    /// The bullet with a parameter `name` more.
    pub fn with(mut self, name: &str, value: Param) -> Bullet {
        self.params.push((name.into(), value));
        self
    }
}

/// A thing the person did well this deal, in the game's words.
#[derive(Clone, Debug, PartialEq)]
pub struct Strength {
    pub bullet: Bullet,
    /// The skills this praises: dropped when one of them is the focus.
    pub praises: Vec<usize>,
}

/// The keys the shape itself says.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Keys {
    /// A deal with too few decisions: its only bullet.
    pub short: &'static str,
    /// The lead of a praise.
    pub well_lead: &'static str,
    /// A skill just mastered, with the parameter `skill`.
    pub well_mastered: &'static str,
    /// Two skills just mastered, with the parameters `skill` and `skill2`
    /// (the first two of the ladder when more came at once).
    pub well_mastered_two: &'static str,
    /// No focus, too few skills seen to say that nothing stands out.
    pub none_early: &'static str,
    /// No focus, and no next opponent to suggest.
    pub none_clean: &'static str,
    /// No focus, and a next opponent up.
    pub none_up: &'static str,
}

/// The game's numbers for the shape.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BriefCfg {
    /// A deal with fewer decisions than this says so in one bullet.
    pub few: u32,
    /// The earlier deals a mastery needs before it is announced: after one
    /// deal a mastery is luck of the draw, not a skill shown over time.
    pub min_history: usize,
    /// The skills with enough chances behind them (their `min_chances`) to
    /// say that nothing stands out.
    pub enough_skills: usize,
    pub keys: Keys,
}

/// What the brief is made of, besides the game's words.
#[derive(Copy, Clone, Debug)]
pub struct Deal<'a> {
    /// The decisions with a choice that counted as evidence.
    pub decisions: u32,
    /// The learner over the earlier deals, and over them and this one.
    pub before: &'a Learner,
    pub after: &'a Learner,
    /// The deal's strengths, best first.
    pub strengths: &'a [Strength],
    /// Whether there is an opponent to suggest next (the top of the dial has
    /// none).
    pub next_opponent: bool,
}

/// The brief of one finished deal: one to [`MAX_BULLETS`] bullets.
///
/// `focus` is the game's bullets for the skill to work on (its rule and
/// example), called at most once, only when `after` has a focus; the ones
/// that do not fit are dropped. It must give at least one bullet for a focus;
/// if it gives none the brief says there is no focus, which can contradict
/// the learner.
pub fn brief(
    ladder: &Ladder,
    cfg: &BriefCfg,
    deal: &Deal,
    focus: impl FnOnce(usize) -> Vec<Bullet>,
) -> Vec<Bullet> {
    let keys = &cfg.keys;
    if deal.decisions < cfg.few {
        return vec![Bullet::new("", keys.short)];
    }
    let mut bullets = Vec::new();
    // What went well: skills just mastered (the first two of the ladder, as
    // Cassino names two), else the best strength that praises nothing the
    // focus is.
    let mastered: Vec<usize> = (0..ladder.skills.len())
        .filter(|&i| deal.after.mastered(i) && !deal.before.mastered(i))
        .filter(|_| deal.before.games >= cfg.min_history)
        .take(2)
        .collect();
    if let [skill] = mastered[..] {
        bullets.push(
            Bullet::new(keys.well_lead, keys.well_mastered).with("skill", Param::Skill(skill)),
        );
    } else if let [skill, skill2] = mastered[..] {
        bullets.push(
            Bullet::new(keys.well_lead, keys.well_mastered_two)
                .with("skill", Param::Skill(skill))
                .with("skill2", Param::Skill(skill2)),
        );
    } else if let Some(s) = deal
        .strengths
        .iter()
        .find(|s| deal.after.focus.is_none_or(|f| !s.praises.contains(&f)))
    {
        bullets.push(s.bullet.clone());
    }
    let said = deal.after.focus.map(focus).filter(|said| !said.is_empty());
    match said {
        Some(said) => bullets.extend(said.into_iter().take(MAX_BULLETS - bullets.len())),
        None => {
            let seen = deal
                .after
                .skills
                .iter()
                .zip(ladder.skills)
                .filter(|(state, spec)| spec.seen(state.chances))
                .count();
            let key = if seen < cfg.enough_skills {
                keys.none_early
            } else if deal.next_opponent {
                keys.none_up
            } else {
                keys.none_clean
            };
            bullets.push(Bullet::new("", key));
        }
    }
    bullets
}
