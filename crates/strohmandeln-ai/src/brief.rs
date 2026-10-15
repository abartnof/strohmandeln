//! The brief: what the person reads after a deal and at the sitting's end,
//! at most three bullets of message keys (`docs/TUTOR.md` 6, 9;
//! `docs/MESSAGES.md` "Tutor keys"). The shape is the kit's
//! ([`kit_learner::brief`]): what went well, the focus with its rule and this
//! deal's example, or that nothing stood out. This module makes the game's
//! part of it: the strengths, the rule chosen by the direction of the
//! costliest miss, the example from the miss and the sitting's sum.

use crate::lessons::{self, Context, Gate};
use crate::rung4::Budget;
use crate::tutor::{
    self, evidence, Consequence, Direction, Facts, Miss, Skill, LADDER, PARAMS, SKILLS,
};
use kit_learner::{
    brief as shape, learn, BriefCfg, Bullet, Deal, Keys, Learner, Param, Strength, Summary,
};
use strohmandeln_core::cards::{Card, Honour};
use strohmandeln_core::dealrec::{DealRecord, ReplayError};
use strohmandeln_core::types::{Seat, Settings};

/// A deal with fewer decisions with a choice than this says so in one
/// bullet and nothing more. A deal has about fourteen; a thrown-in deal one
/// or two. Provisional (M3 has the chances a deal).
pub const FEW: u32 = 4;

/// The earlier deals a mastery needs before it is announced: after one a
/// mastery is luck of the draw (`docs/TUTOR.md` 6).
pub const MIN_HISTORY: usize = 2;

/// The skills with enough chances behind them to say that nothing stands
/// out, as Cassino's.
pub const ENOUGH_SKILLS: usize = 3;

/// The chances at a skill, all met, that are worth praising: one is luck.
pub const MIN_PRAISE_CHANCES: u32 = 2;

/// The top of the opponent dial (`docs/OPPONENT.md`): none above it.
pub const TOP_LEVEL: f64 = 4.0;

/// The key of the folded method.
pub const METHOD: &str = "tutor.method";

/// The keys the table says around the bullets: the prefix of a rule, the
/// fold's title and the method it opens.
pub const PAGE_KEYS: &[&str] = &["tutor.lead.rule", "tutor.how", METHOD];

/// The keys the brief says besides the lessons' and the examples': the
/// shape's (a short deal, what went well, nothing to work on) and the
/// strengths'.
pub const KEYS: &[&str] = &[
    "tutor.short",
    "tutor.lead.well",
    "tutor.well.mastered",
    "tutor.well.mastered-two",
    "tutor.well.met",
    "tutor.well.announced",
    "tutor.well.ultimo",
    "tutor.well.mond",
    "tutor.none.early",
    "tutor.none.clean",
    "tutor.none.up",
];

const CFG: BriefCfg = BriefCfg {
    few: FEW,
    min_history: MIN_HISTORY,
    enough_skills: ENOUGH_SKILLS,
    keys: Keys {
        short: "tutor.short",
        well_lead: "tutor.lead.well",
        well_mastered: "tutor.well.mastered",
        well_mastered_two: "tutor.well.mastered-two",
        none_early: "tutor.none.early",
        none_clean: "tutor.none.clean",
        none_up: "tutor.none.up",
    },
};

/// What a brief is made of: one deal, or the deals of a sitting as one game.
#[derive(Clone, Debug)]
pub struct Played {
    /// The deals' evidence summed.
    pub summary: Summary,
    /// The unassisted decisions with a choice.
    pub decisions: u32,
    /// The facts of each deal.
    pub facts: Vec<Facts>,
    /// The costliest miss at each skill, and the number of its deal in the
    /// sitting (0 for a single deal, which is "this deal").
    pub misses: [Option<(Miss, u32)>; SKILLS],
    /// The deals' settings, for the rule to be told under.
    pub settings: Settings,
    /// Chooses between a skill's two tries, a pure function of the deal.
    pub variety: usize,
    /// The gates a rule may have to be told: [`lessons::TOLD`] (tests admit
    /// more).
    pub told: &'static [Gate],
}

/// The brief and what it was worked out from.
#[derive(Clone, Debug, PartialEq)]
pub struct Brief {
    /// One to three bullets.
    pub bullets: Vec<Bullet>,
    /// The key of the folded method.
    pub method: &'static str,
    /// The deal's (or sitting's) evidence, which the table stores.
    pub summary: Summary,
    /// What the table keeps of a deal for the sitting's brief (a single
    /// deal's brief only).
    pub detail: Option<Detail>,
    /// The skill to work on, after this deal.
    pub focus: Option<Skill>,
}

fn int(n: u32) -> Param {
    Param::Int(i64::from(n))
}

fn text(c: Card) -> Param {
    Param::Text(c.code())
}

type Params = Vec<(String, Param)>;

fn named(list: Vec<(&str, Param)>) -> Params {
    list.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

/// A missed chance told in one sentence: the key of the example and its
/// parameters, or `None` where the miss lacks what the sentence needs. After
/// the last trick every card is public, so the cards named hold nothing the
/// person could not have seen by now (`docs/TUTOR.md` 6).
pub fn example(m: &Miss) -> Option<(String, Params)> {
    let trick = int(m.trick);
    let (played, better) = (m.played, m.better);
    let suit = || {
        m.played
            .and_then(Card::suit)
            .map(|s| Param::Text(s.letter().to_string()))
    };
    let (key, params) = match (m.skill, m.direction) {
        (Skill::Worth, Some(Direction::Greedy)) => (
            "tutor.miss.worth.poor",
            vec![
                ("trick", trick),
                ("played", text(played?)),
                ("better", text(better?)),
                ("thirds", int(m.trick_thirds)),
            ],
        ),
        (Skill::Worth, Some(Direction::Timid)) => (
            "tutor.miss.worth.rich",
            vec![
                ("trick", trick),
                ("led", text(m.led?)),
                ("better", text(better?)),
                ("thirds", int(m.trick_thirds)),
            ],
        ),
        (Skill::Cheap, _) => (
            "tutor.miss.cheap",
            vec![
                ("trick", trick),
                ("played", text(played?)),
                ("better", text(better?)),
            ],
        ),
        (Skill::Off, Some(Direction::Pip)) => (
            "tutor.miss.off.pip",
            vec![("trick", trick), ("better", text(better?))],
        ),
        (Skill::Off, Some(Direction::Figure)) => match m.consequence {
            Some(Consequence::FellUnder { by, trick: late }) => (
                "tutor.miss.off.figure",
                vec![
                    ("card", text(better?)),
                    ("king", text(by)),
                    ("late", int(late)),
                ],
            ),
            None => (
                "tutor.miss.off.figure-plain",
                vec![("trick", trick), ("better", text(better?))],
            ),
        },
        (Skill::Run, _) => (
            "tutor.miss.run",
            vec![
                ("trick", trick),
                ("played", text(played?)),
                ("better", text(better?)),
            ],
        ),
        (Skill::Pagat, Some(Direction::Cash)) => (
            "tutor.miss.pagat.cash",
            vec![("trick", trick), ("n", int(m.tarocks))],
        ),
        (Skill::Pagat, Some(Direction::Keep)) => (
            "tutor.miss.pagat.keep",
            vec![("trick", trick), ("n", int(m.tarocks))],
        ),
        (Skill::Void, _) => (
            "tutor.miss.void",
            vec![
                ("trick", trick),
                ("played", text(played?)),
                ("suit", suit()?),
            ],
        ),
        (Skill::Behind, _) => (
            "tutor.miss.behind",
            vec![
                ("trick", trick),
                ("suit", suit()?),
                ("better", text(better?)),
            ],
        ),
        (Skill::Endgame, _) => (
            "tutor.miss.endgame",
            vec![
                ("trick", trick),
                ("played", text(played?)),
                ("better", text(better?)),
            ],
        ),
        (Skill::Bid, Some(Direction::Passed)) => (
            "tutor.miss.bid.pass",
            vec![("sure", int(m.sure_thirds)), ("n", int(m.tarocks))],
        ),
        (Skill::Bid, Some(Direction::Took)) => (
            "tutor.miss.bid.take",
            vec![("sure", int(m.sure_thirds)), ("n", int(m.tarocks))],
        ),
        (Skill::Announce, _) => {
            let card = played?;
            let honour = [Honour::Trull, Honour::Kings]
                .into_iter()
                .find(|h| h.cards().contains(&card))?;
            let name = match honour {
                Honour::Trull => "trull",
                Honour::Kings => "kings",
            };
            (
                "tutor.miss.announce",
                vec![("card", text(card)), ("honour", Param::Text(name.into()))],
            )
        }
        _ => return None,
    };
    Some((key.to_string(), named(params)))
}

/// What went well, best first: every chance at a skill met (the skill with
/// the most chances first), an honour announced, the Pagat ultimo won, the
/// Mond caught. The last two carry luck and are only ever praise; a skill
/// praised is dropped when it is the focus (the kit does that).
fn strengths(summary: &Summary, facts: &[Facts]) -> Vec<Strength> {
    let well = |key: &str| Bullet::new("tutor.lead.well", key);
    let mut out = Vec::new();
    let mut all_met: Vec<usize> = (0..SKILLS)
        .filter(|&i| i != Skill::Announce.index())
        .filter(|&i| {
            let t = summary.of(i);
            t.chances >= MIN_PRAISE_CHANCES && t.met == t.chances
        })
        .collect();
    all_met.sort_by_key(|&i| (std::cmp::Reverse(summary.of(i).chances), i));
    for i in all_met {
        out.push(Strength {
            bullet: well("tutor.well.met")
                .with("chances", int(summary.of(i).chances))
                .with("skill", Param::Skill(i)),
            praises: vec![i],
        });
    }
    let say = summary.of(Skill::Announce.index());
    let honour = facts.iter().flat_map(|f| f.announced.iter()).next();
    if let (true, Some(h)) = (say.chances >= 1 && say.met == say.chances, honour) {
        let name = match h {
            Honour::Trull => "trull",
            Honour::Kings => "kings",
        };
        out.push(Strength {
            bullet: well("tutor.well.announced").with("honour", Param::Text(name.into())),
            praises: vec![Skill::Announce.index()],
        });
    }
    if facts.iter().any(|f| f.ultimo_won) {
        out.push(Strength {
            bullet: well("tutor.well.ultimo"),
            praises: vec![Skill::Pagat.index()],
        });
    }
    // The trick of a Mond is a trick of one deal: said of a deal only.
    if let (1, Some(trick)) = (facts.len(), facts.iter().find_map(|f| f.mond_caught)) {
        out.push(Strength {
            bullet: well("tutor.well.mond").with("trick", int(trick)),
            praises: vec![],
        });
    }
    out
}

/// The bullets for the focus: its name with the rule or else the fact, then
/// this deal's example, or else one of its two tries.
fn focus_bullets(skill: usize, played: &Played) -> Vec<Bullet> {
    let skill_ = Skill::ALL[skill];
    let lesson = lessons::lesson(skill_);
    let found = played.misses[skill].as_ref();
    let ctx = Context {
        direction: found.and_then(|(m, _)| m.direction),
        tarocks: found.map_or(0, |(m, _)| m.tarocks),
        led_thirds: found.and_then(|(m, _)| m.led).map_or(0, |c| c.thirds()),
    };
    let said =
        lessons::taught(skill_, &ctx, &played.settings, played.told).map_or(lesson.fact, |r| r.key);
    let next = Bullet::new("tutor.lead.next", said).with("skill", Param::Skill(skill));
    let second = match found.and_then(|(m, n)| example(m).map(|e| (e, *n))) {
        Some(((key, params), n)) => {
            let mut b = if n == 0 {
                Bullet::new("tutor.lead.deal", &key)
            } else {
                Bullet::new("tutor.lead.deal-n", &key).with("deal", int(n))
            };
            b.params.extend(params);
            b
        }
        None => Bullet::new("tutor.lead.try", lesson.tries[played.variety % 2]),
    };
    vec![next, second]
}

/// The brief of `played`, from the learner before it and after it. Pure.
pub fn compose(before: &Learner, after: &Learner, played: &Played, next_opponent: bool) -> Brief {
    let strengths = strengths(&played.summary, &played.facts);
    let deal = Deal {
        decisions: played.decisions,
        before,
        after,
        strengths: &strengths,
        next_opponent,
    };
    let bullets = shape(&LADDER, &CFG, &deal, |i| focus_bullets(i, played));
    Brief {
        bullets,
        method: METHOD,
        summary: played.summary.clone(),
        detail: None,
        focus: after.focus.map(|i| Skill::ALL[i]),
    }
}

fn higher_than_top(level: f64) -> bool {
    level < TOP_LEVEL - 1e-9
}

/// What a scored deal leaves for the briefs, worked out once from its record
/// when it is scored and kept with it: the sitting's brief is made from these
/// and does not run the advisor again (`docs/TUTOR.md` 8, review finding 4).
#[derive(Clone, Debug, PartialEq)]
pub struct Detail {
    pub summary: Summary,
    /// The unassisted decisions with a choice.
    pub decisions: u32,
    pub facts: Facts,
    /// The costliest miss at each skill.
    pub misses: [Option<Miss>; SKILLS],
    /// The deal's settings, for the rule to be told under.
    pub settings: Settings,
    /// Chooses between a skill's two tries.
    pub variety: usize,
}

/// The evidence of a record, as a [`Detail`].
pub fn detail_of(
    record: &DealRecord,
    human: Seat,
    assisted: &[usize],
    budget: Budget,
) -> Result<Detail, ReplayError> {
    let e = evidence(record, human, assisted, budget)?;
    Ok(Detail {
        summary: e.summary,
        decisions: e.decisions,
        facts: e.facts,
        misses: e.costliest,
        settings: record.settings,
        variety: record.deck.first().map_or(0, |c| c.index()),
    })
}

/// The brief of the deals `parts` (a deal's number in the sitting, 0 for a
/// single deal, and its detail), told against `history`, the summaries before
/// them. A sitting is its deals summed as one game for the counts and the
/// example, and learned deal by deal: a mastery earned in it is announced in
/// the brief of the deal that earned it, not again in the sitting's.
fn brief_from(parts: &[(u32, &Detail)], history: &[Summary], level: f64, sitting: bool) -> Brief {
    let first = parts.first().map(|(_, d)| *d).expect("a deal");
    let mut played = Played {
        summary: Summary::new(&LADDER),
        decisions: 0,
        facts: Vec::new(),
        misses: Default::default(),
        settings: first.settings,
        variety: first.variety,
        told: lessons::TOLD,
    };
    let mut all = history.to_vec();
    for (number, d) in parts {
        for (sum, t) in played.summary.tallies.iter_mut().zip(&d.summary.tallies) {
            // a detail comes from storage: sums that cannot overflow
            sum.chances = sum.chances.saturating_add(t.chances);
            sum.met = sum.met.saturating_add(t.met);
            sum.missed = (sum.missed + t.missed).min(f64::MAX);
        }
        played.decisions = played.decisions.saturating_add(d.decisions);
        played.facts.push(d.facts.clone());
        for (slot, m) in played.misses.iter_mut().zip(&d.misses) {
            if let Some(m) = m {
                if slot.as_ref().is_none_or(|(w, _)| m.cost > w.cost) {
                    *slot = Some((m.clone(), *number));
                }
            }
        }
        all.push(d.summary.clone());
    }
    let after = learn(&LADDER, &PARAMS, &all);
    let before = if sitting {
        after.clone()
    } else {
        learn(&LADDER, &PARAMS, history)
    };
    compose(&before, &after, &played, higher_than_top(level))
}

/// The brief of one scored deal: its evidence is worked out here, once, and
/// is the brief's `summary` and `detail`. `history` holds the earlier deals'
/// summaries, oldest first (stale ones are skipped); `level` is the
/// opponent's.
pub fn brief_of_deal(
    record: &DealRecord,
    human: Seat,
    assisted: &[usize],
    history: &[Summary],
    level: f64,
    budget: Budget,
) -> Result<Brief, ReplayError> {
    let detail = detail_of(record, human, assisted, budget)?;
    let mut brief = brief_from(&[(0, &detail)], history, level, false);
    brief.detail = Some(detail);
    Ok(brief)
}

/// One deal of a sitting: the number the table shows for it, and what was
/// kept when it was scored.
#[derive(Copy, Clone, Debug)]
pub struct SittingDeal<'a> {
    pub number: u32,
    pub detail: &'a Detail,
}

/// The brief of a sitting (`docs/TUTOR.md` 6) from the deals' kept details;
/// the advisor does not run. `history` holds the summaries of the deals
/// before the sitting. `None` for an empty sitting.
pub fn brief_of_sitting(deals: &[SittingDeal], history: &[Summary], level: f64) -> Option<Brief> {
    let parts: Vec<(u32, &Detail)> = deals.iter().map(|d| (d.number, d.detail)).collect();
    (!parts.is_empty()).then(|| brief_from(&parts, history, level, true))
}

/// The learner over stored summaries, oldest first.
pub fn learner(history: &[Summary]) -> Learner {
    tutor::learner(history)
}

#[cfg(test)]
#[path = "brief_tests.rs"]
mod tests;
