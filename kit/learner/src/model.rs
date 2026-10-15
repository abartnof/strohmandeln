//! The ladder, the constants and the summary of one deal.

/// One skill of a game's ladder. Skills are indexes into
/// [`Ladder::skills`]; a game keeps its own enum and maps it.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SkillSpec {
    /// The wire name: one word, no whitespace (the summary's text form).
    pub slug: &'static str,
    /// The skills this one needs first, as indexes into the ladder.
    pub needs: &'static [usize],
    /// The met rate the strongest player reaches at this skill (measured).
    pub top_rate: f64,
    /// What the strongest player itself loses to this skill in a deal, in the
    /// game's score unit.
    /// A student's cost beyond it is what mastering the skill would gain.
    pub top_cost: f64,
    /// The discounted chances the skill needs before its rate counts. Below
    /// it the state does not change.
    pub min_chances: f64,
}

/// A game's skills, in ladder order (exact ties in the focus go to the
/// earlier skill), and the version of the evidence that tallies them.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ladder {
    /// The first word of a summary's text form ("<name> evidence v1").
    pub name: &'static str,
    /// Bump it whenever the clear chances change (the advisor, the margin,
    /// the classifier of what a move shows), so that a stored summary made
    /// before is recomputed from its record.
    pub version: u32,
    pub skills: &'static [SkillSpec],
}

impl Ladder {
    /// Whether the ladder is well formed: a name of one word, skills with
    /// distinct one-word slugs, prerequisites inside the ladder and not the
    /// skill itself and not twice, no cycle among them, rates in [0, 1].
    pub fn check(&self) -> Result<(), String> {
        let one_word = |w: &str| !w.is_empty() && !w.contains(char::is_whitespace);
        if !one_word(self.name) {
            return Err(format!(
                "the ladder's name is not one word: {:?}",
                self.name
            ));
        }
        if self.skills.is_empty() {
            return Err("a ladder with no skills".into());
        }
        for (i, s) in self.skills.iter().enumerate() {
            if !one_word(s.slug) {
                return Err(format!("skill {i}: slug {:?} is not one word", s.slug));
            }
            if self.skills[..i].iter().any(|t| t.slug == s.slug) {
                return Err(format!("skill {i}: slug {:?} is used twice", s.slug));
            }
            if let Some(n) = s.needs.iter().find(|&&n| n >= self.skills.len() || n == i) {
                return Err(format!(
                    "{}: needs {n}, which is itself or not in the ladder",
                    s.slug
                ));
            }
            if let Some(k) = (1..s.needs.len()).find(|&k| s.needs[..k].contains(&s.needs[k])) {
                return Err(format!("{}: needs {} twice", s.slug, s.needs[k]));
            }
            if !(0.0..=1.0).contains(&s.top_rate) || !s.top_cost.is_finite() || s.top_cost < 0.0 {
                return Err(format!("{}: top_rate or top_cost out of range", s.slug));
            }
            if !s.min_chances.is_finite() || s.min_chances < 0.0 {
                return Err(format!("{}: min_chances out of range", s.slug));
            }
        }
        // A cycle in the prerequisites would keep every skill on it from
        // being open for ever: no focus, silently.
        if let Some(cycle) = self.cycle() {
            let names: Vec<&str> = cycle.iter().map(|&i| self.skills[i].slug).collect();
            return Err(format!(
                "prerequisites form a cycle: {}",
                names.join(" -> ")
            ));
        }
        Ok(())
    }

    /// A cycle of prerequisites as indexes, the first repeated at the end, or
    /// `None`. Depth first from each skill in ladder order; assumes every
    /// `needs` index is inside the ladder.
    fn cycle(&self) -> Option<Vec<usize>> {
        // 0: not seen, 1: on the path, 2: done.
        let mut state = vec![0u8; self.skills.len()];
        let mut path: Vec<usize> = Vec::new();
        for start in 0..self.skills.len() {
            if state[start] != 0 {
                continue;
            }
            // (skill, next need to look at)
            let mut stack = vec![(start, 0usize)];
            state[start] = 1;
            path.push(start);
            while let Some(&(i, k)) = stack.last() {
                let Some(&n) = self.skills[i].needs.get(k) else {
                    state[i] = 2;
                    path.pop();
                    stack.pop();
                    continue;
                };
                stack.last_mut().expect("a frame").1 += 1;
                match state[n] {
                    0 => {
                        state[n] = 1;
                        path.push(n);
                        stack.push((n, 0));
                    }
                    1 => {
                        let from = path.iter().position(|&p| p == n).expect("on the path");
                        let mut cycle = path[from..].to_vec();
                        cycle.push(n);
                        return Some(cycle);
                    }
                    _ => {}
                }
            }
        }
        None
    }

    /// The index of the skill with this slug.
    pub fn index_of(&self, slug: &str) -> Option<usize> {
        self.skills.iter().position(|s| s.slug == slug)
    }
}

/// The constants of the learner. A game sets its own from its strongest
/// player's deals and lesioned students (`docs/TUTOR.md` section 11); the
/// crate has no defaults. Costs and gains are in the game's score unit.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Params {
    /// The weight of each deal relative to the one after it.
    pub discount: f64,
    /// The least a skill must cost per deal to be worth working on.
    pub min_gain: f64,
    /// The gain at which an unmastered skill is the focus although its rate
    /// is not shown to be short ([`Params::upper`]).
    pub costly_gain: f64,
    /// The discounted chances a costly skill needs.
    pub costly_chances: f64,
    /// The quantile of the met rate that must reach [`Params::master`], for
    /// mastery: the rate is at least that high with `1 - lower` confidence.
    pub lower: f64,
    /// The quantile of the met rate that must fall below [`Params::lose`]
    /// for a mastered skill to be lost, and below [`Params::master`] for a
    /// skill to be the focus. Stricter than `lower` on purpose.
    pub upper: f64,
    /// What mastery asks, as a fraction of the strongest player's own rate.
    pub master_of_top: f64,
    /// How far below the mastery rate the rate must be shown to be to lose
    /// a skill.
    pub lose_below: f64,
}

impl Params {
    /// Whether the constants can work: all finite; `discount` in (0, 1];
    /// `lower` and `upper` quantile levels in (0, 1) with `lower < upper`;
    /// `master_of_top` in (0, 1]; the gains, chances and `lose_below` at
    /// least 0. [`crate::learn`] asserts it in debug builds; a game also
    /// calls it once at start.
    pub fn check(&self) -> Result<(), String> {
        let finite = [
            ("discount", self.discount),
            ("min_gain", self.min_gain),
            ("costly_gain", self.costly_gain),
            ("costly_chances", self.costly_chances),
            ("lower", self.lower),
            ("upper", self.upper),
            ("master_of_top", self.master_of_top),
            ("lose_below", self.lose_below),
        ];
        for (name, v) in finite {
            if !v.is_finite() {
                return Err(format!("{name} is not finite: {v}"));
            }
        }
        if !(self.discount > 0.0 && self.discount <= 1.0) {
            return Err(format!("discount must be in (0, 1]: {}", self.discount));
        }
        for (name, v) in [("lower", self.lower), ("upper", self.upper)] {
            if !(v > 0.0 && v < 1.0) {
                return Err(format!("{name} must be a quantile level in (0, 1): {v}"));
            }
        }
        if self.lower >= self.upper {
            return Err(format!(
                "lower ({}) must be below upper ({})",
                self.lower, self.upper
            ));
        }
        if !(self.master_of_top > 0.0 && self.master_of_top <= 1.0) {
            return Err(format!(
                "master_of_top must be in (0, 1]: {}",
                self.master_of_top
            ));
        }
        for (name, v) in [
            ("min_gain", self.min_gain),
            ("costly_gain", self.costly_gain),
            ("costly_chances", self.costly_chances),
            ("lose_below", self.lose_below),
        ] {
            if v < 0.0 {
                return Err(format!("{name} must be at least 0: {v}"));
            }
        }
        Ok(())
    }

    /// The met rate that makes `skill` mastered.
    pub fn master(&self, skill: &SkillSpec) -> f64 {
        self.master_of_top * skill.top_rate
    }

    /// The met rate a mastered `skill` is lost below.
    pub fn lose(&self, skill: &SkillSpec) -> f64 {
        (self.master(skill) - self.lose_below).max(0.0)
    }
}

/// One skill's counts over one deal: whole numbers. The learner's discounted
/// counts are in [`crate::SkillState`].
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Tally {
    pub chances: u32,
    /// At most `chances`.
    pub met: u32,
    /// The sum of the gaps of the chances missed, in the game's score unit:
    /// finite, at least 0.
    pub missed: f64,
}

impl Tally {
    /// Whether the counts can be true: `met` within `chances`, `missed` a
    /// finite loss.
    pub fn is_valid(&self) -> bool {
        self.met <= self.chances && self.missed.is_finite() && self.missed >= 0.0
    }
}

/// One clear chance, as the game's evidence finds it.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chance {
    /// The skill's index in the ladder.
    pub skill: usize,
    pub met: bool,
    /// What a miss cost, in the game's score unit; unused when met. Meant
    /// finite and at least 0; anything else counts as 0, so that one bad
    /// number cannot poison a skill for ever.
    pub gap: f64,
}

/// What one deal showed of each skill, in ladder order.
#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    pub version: u32,
    pub tallies: Vec<Tally>,
}

impl Summary {
    /// The summary of no chances, at the ladder's version.
    pub fn new(ladder: &Ladder) -> Summary {
        Summary {
            version: ladder.version,
            tallies: vec![Tally::default(); ladder.skills.len()],
        }
    }

    /// The summary of these chances. Panics on a skill outside the ladder.
    pub fn from_chances(ladder: &Ladder, chances: &[Chance]) -> Summary {
        let mut s = Summary::new(ladder);
        for c in chances {
            let t = &mut s.tallies[c.skill];
            t.chances += 1;
            if c.met {
                t.met += 1;
            } else {
                if c.gap.is_finite() && c.gap > 0.0 {
                    t.missed += c.gap;
                }
            }
        }
        s
    }

    /// The tally of skill `i`.
    pub fn of(&self, i: usize) -> Tally {
        self.tallies[i]
    }

    /// Whether this was computed by the ladder's present evidence (its
    /// version, and a tally for each skill) and can be true (each tally
    /// [`Tally::is_valid`]); anything else is stale and [`crate::learn`]
    /// skips it.
    pub fn is_current(&self, ladder: &Ladder) -> bool {
        self.version == ladder.version
            && self.tallies.len() == ladder.skills.len()
            && self.tallies.iter().all(Tally::is_valid)
    }

    /// The text form: a header, then a line a skill, `<slug> <chances>
    /// <met> <missed>`.
    pub fn to_text(&self, ladder: &Ladder) -> String {
        let mut lines = vec![format!("{} evidence v{}", ladder.name, self.version)];
        for (spec, t) in ladder.skills.iter().zip(&self.tallies) {
            lines.push(format!(
                "{} {} {} {}",
                spec.slug, t.chances, t.met, t.missed
            ));
        }
        lines.join("\n")
    }

    /// Reads the text form of this ladder; refuses anything else.
    pub fn parse(ladder: &Ladder, text: &str) -> Result<Summary, String> {
        let mut lines = text.lines().map(str::trim).filter(|l| !l.is_empty());
        let head = lines.next().ok_or("an empty summary")?;
        let prefix = format!("{} evidence v", ladder.name);
        let version: u32 = head
            .strip_prefix(prefix.as_str())
            .and_then(|v| v.parse().ok())
            .ok_or(format!("expected \"{prefix}<n>\", found {head:?}"))?;
        let mut tallies = Vec::with_capacity(ladder.skills.len());
        for spec in ladder.skills {
            let line = lines
                .next()
                .ok_or(format!("the summary ends before {}", spec.slug))?;
            let w: Vec<&str> = line.split_whitespace().collect();
            let bad = || format!("not a tally of {}: {line:?}", spec.slug);
            if w.len() != 4 || w[0] != spec.slug {
                return Err(bad());
            }
            let t = Tally {
                chances: w[1].parse().map_err(|_| bad())?,
                met: w[2].parse().map_err(|_| bad())?,
                missed: w[3].parse().map_err(|_| bad())?,
            };
            if !t.is_valid() {
                return Err(bad());
            }
            tallies.push(t);
        }
        if let Some(extra) = lines.next() {
            return Err(format!("more than the ladder's skills: {extra:?}"));
        }
        Ok(Summary { version, tallies })
    }
}

/// The indexes of the summaries that [`crate::learn`] would skip as stale
/// (made by another version of the evidence): a client recomputes those from
/// their records before asking, so that no deal is silently dropped.
pub fn stale(ladder: &Ladder, games: &[Summary]) -> Vec<usize> {
    games
        .iter()
        .enumerate()
        .filter(|(_, g)| !g.is_current(ladder))
        .map(|(i, _)| i)
        .collect()
}
