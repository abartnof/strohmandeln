//! The tutor's evidence (`docs/TUTOR.md` 2 to 4): what one deal says of the
//! ten skills of the ladder (`research/16`).
//!
//! For each unassisted decision of the human with a choice, the advisor
//! (rung 4, [`rung4::move_table`]) values the legal cards from the human's own
//! view alone ([`Knowledge`], never a hidden card), and each skill decides
//! whether the decision is a **clear chance** (the right class of cards beats
//! the next by more than the advisor's noise), whether the human **met** it
//! (played a card of the right class) and what a miss **cost**, in thirds of
//! the count margin. The classifiers are pure functions of the position and
//! the values ([`classify_play`], [`classify_bid`], [`classify_announce`]), so
//! they are tested on hand-built positions; [`decision_verdicts`] adds the
//! search, and [`evidence`] replays a deal record and tallies.
//!
//! **Provisional constants.** The values of [`MARGIN`], [`EXACT_MARGIN`],
//! [`BID_MARGIN`], [`THIRDS_PER_POINT`] and the split-suit share are
//! placeholders; each names the measurement (M1 to M8, `docs/TUTOR.md` 11)
//! that sets it. Changing any of them, or the advisor, bumps
//! [`EVIDENCE_VERSION`].

use crate::cardset::{leader_wins, Kind};
use crate::know::Knowledge;
use crate::rung2::Weights;
use crate::rung4::{self, Budget};
use crate::seed::view_seed;
use crate::sim::ULTIMO_THIRDS;
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::deal::TrickRecord;
use strohmandeln_core::dealrec::{DealRecord, ReplayError};
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::{Action, Seat};
use strohmandeln_core::view::{bare_view, View};

/// The version of what [`evidence`] computes. Bump it whenever a classifier,
/// a constant or the advisor changes, so that a stored summary made before is
/// recomputed from its record (`docs/TUTOR.md` 3).
pub const EVIDENCE_VERSION: u32 = 1;

/// The advisor's noise in thirds: a class must beat the next by more than
/// this to be a clear chance, and a spread card is met within it.
/// Provisional; M1 (advisor noise under independent seeds) sets it.
pub const MARGIN: f64 = 6.0;

/// The same in an exact position (no face-down packet card left, so the
/// search has no sampling noise): the least loss that counts. Provisional;
/// M1 sets it.
pub const EXACT_MARGIN: f64 = 1.0;

/// The margin of the bid in Points, times the deal's doubling. Provisional;
/// M1 sets it (the bid's expectation is over `Budget::bid_worlds` worlds).
pub const BID_MARGIN: f64 = 0.5;

/// Thirds in a Point, to price a bid or an announcement in the unit of the
/// rest. Provisional; M2 (final margin in thirds against net Points) sets it.
pub const THIRDS_PER_POINT: f64 = ULTIMO_THIRDS as f64;

/// A suit is split when my share of its unplayed value points lies in this
/// range. Provisional, as rung 2's 30% to 70%; M8 sets it.
pub const SPLIT_SHARE: (f64, f64) = (0.3, 0.7);

/// Unplayed value points a suit needs to be worth contesting. Provisional;
/// M8 sets it.
pub const SPLIT_MIN_POINTS: u32 = 6;

/// The stream the advisor's draws come from, mixed with the view.
const EVIDENCE_SEED: u64 = 0x7475_746f_7231;

pub const SKILLS: usize = 10;

/// The skills of `docs/TUTOR.md` 2, in stage order.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Skill {
    Worth,
    Cheap,
    Off,
    Run,
    Pagat,
    Void,
    Behind,
    Endgame,
    Bid,
    Announce,
}

impl Skill {
    pub const ALL: [Skill; SKILLS] = [
        Skill::Worth,
        Skill::Cheap,
        Skill::Off,
        Skill::Run,
        Skill::Pagat,
        Skill::Void,
        Skill::Behind,
        Skill::Endgame,
        Skill::Bid,
        Skill::Announce,
    ];

    /// The wire name.
    pub fn slug(self) -> &'static str {
        match self {
            Skill::Worth => "worth",
            Skill::Cheap => "cheap",
            Skill::Off => "off",
            Skill::Run => "run",
            Skill::Pagat => "pagat",
            Skill::Void => "void",
            Skill::Behind => "behind",
            Skill::Endgame => "endgame",
            Skill::Bid => "bid",
            Skill::Announce => "announce",
        }
    }

    pub fn index(self) -> usize {
        self as usize
    }

    /// The stage of the ladder, 1 to 5; the two calls are 0.
    pub fn stage(self) -> u8 {
        match self {
            Skill::Worth | Skill::Cheap => 1,
            Skill::Off => 2,
            Skill::Run | Skill::Pagat => 3,
            Skill::Void | Skill::Behind => 4,
            Skill::Endgame => 5,
            Skill::Bid | Skill::Announce => 0,
        }
    }

    /// The skills this one presupposes (`docs/TUTOR.md` 2).
    pub fn needs(self) -> &'static [Skill] {
        match self {
            Skill::Worth | Skill::Announce => &[],
            Skill::Cheap | Skill::Off | Skill::Void | Skill::Bid => &[Skill::Worth],
            Skill::Run => &[Skill::Cheap],
            Skill::Pagat => &[Skill::Run],
            Skill::Behind => &[Skill::Off, Skill::Void],
            Skill::Endgame => &[Skill::Cheap, Skill::Void],
        }
    }
}

/// How a missed chance went wrong, for the words of the example.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    /// `worth`: won a poor trick.
    Greedy,
    /// `worth`: let a rich trick go.
    Timid,
    /// `off`: the better cover was a Dame or Cavall.
    Figure,
    /// `off`: the better cover was a pip.
    Pip,
    /// `pagat`: should have been played.
    Cash,
    /// `pagat`: should have been kept.
    Keep,
    /// `bid`: passed where taking up paid.
    Passed,
    /// `bid`: took up where passing paid.
    Took,
}

/// One skill's verdict on one decision, with what the brief's example needs.
#[derive(Clone, Debug, PartialEq)]
pub struct Verdict {
    pub skill: Skill,
    pub met: bool,
    /// Thirds lost by a miss; 0 when met.
    pub cost: f64,
    /// The best card of the right class (a miss).
    pub better: Option<Card>,
    /// A miss's direction, where the skill has them.
    pub direction: Option<Direction>,
    /// `worth`: the thirds in the trick (the card led and the card that won
    /// it or would have).
    pub trick_thirds: u32,
    /// `pagat`, `bid`: the player's tarocks.
    pub tarocks: u32,
    /// `bid`: the hand's sure winners in thirds.
    pub sure_thirds: u32,
}

impl Verdict {
    fn new(skill: Skill, met: bool, cost: f64) -> Verdict {
        Verdict {
            skill,
            met,
            cost: if met { 0.0 } else { cost },
            better: None,
            direction: None,
            trick_thirds: 0,
            tarocks: 0,
            sure_thirds: 0,
        }
    }

    /// The better card, when missed.
    fn better(mut self, card: Card) -> Verdict {
        if !self.met {
            self.better = Some(card);
        }
        self
    }

    /// The direction, when missed.
    fn toward(mut self, direction: Direction) -> Verdict {
        if !self.met {
            self.direction = Some(direction);
        }
        self
    }
}

/// Whether the advisor is exact: every packet card of both players is turned.
fn exact(k: &Knowledge) -> bool {
    k.playing && k.hidden_total() == 0
}

fn margin(k: &Knowledge) -> f64 {
    if exact(k) {
        EXACT_MARGIN
    } else {
        MARGIN
    }
}

/// The best (first of equals) of the values whose card is in `class`.
fn best(scores: &[(Card, f64)], class: impl Fn(Card) -> bool) -> Option<(Card, f64)> {
    scores
        .iter()
        .filter(|(c, _)| class(*c))
        .fold(None, |b, &(c, v)| match b {
            Some((_, bv)) if bv >= v => b,
            _ => Some((c, v)),
        })
}

fn worst(scores: &[(Card, f64)], class: impl Fn(Card) -> bool) -> Option<f64> {
    scores
        .iter()
        .filter(|(c, _)| class(*c))
        .map(|&(_, v)| v)
        .fold(None, |w, v| Some(w.map_or(v, |w: f64| w.min(v))))
}

/// The value points of the cards of `suit` among `cards`.
fn points_in(cards: impl IntoIterator<Item = Card>, suit: Suit) -> u32 {
    cards
        .into_iter()
        .filter(|c| c.suit() == Some(suit))
        .map(|c| c.value())
        .sum()
}

/// A suit the hands share about evenly: my share of its unplayed value points
/// is in [`SPLIT_SHARE`] (`research/16` 2, `behind`). What is not mine is
/// "his": his hand, his packets and my own face-down packet cards, which I
/// cannot tell from his.
fn is_split(k: &Knowledge, suit: Suit) -> bool {
    let mine = points_in(k.playable().iter(), suit);
    let theirs = points_in(k.opp_possible().iter(), suit);
    let total = mine + theirs;
    if total < SPLIT_MIN_POINTS {
        return false;
    }
    let share = f64::from(mine) / f64::from(total);
    (SPLIT_SHARE.0..=SPLIT_SHARE.1).contains(&share)
}

/// A suit he has shown he is out of: he could not follow, and no cover of his
/// and no card of his known from a pull is of it (the counter rung's own
/// inference, `Knowledge::opp_void`).
fn shown_void(k: &Knowledge, suit: Suit) -> bool {
    let kind = Kind::Suit(suit);
    k.opp_void[kind.index()]
        && !k
            .opp_covers
            .iter()
            .flatten()
            .copied()
            .chain(k.opp_known.iter())
            .any(|c| Kind::of(c) == kind)
}

/// The verdicts of the skills that judge a card played, given the advisor's
/// value of every legal card (`scores`, in thirds, higher is better for the
/// mover). A skill with no clear chance gives none.
pub fn classify_play(k: &Knowledge, card: Card, scores: &[(Card, f64)]) -> Vec<Verdict> {
    let mut out = Vec::new();
    if scores.len() < 2 {
        return out;
    }
    let Some(played) = scores.iter().find(|(c, _)| *c == card).map(|&(_, v)| v) else {
        return out;
    };
    let m = margin(k);

    if let Some(led) = k.lead {
        // The follower. A card beats the lead when the lead does not win.
        let beats = |c: Card| !leader_wins(led, c);
        // worth: win the trick, or let it go
        if let (Some(w), Some(l)) = (best(scores, beats), best(scores, |c| !beats(c))) {
            let gap = (w.1 - l.1).abs();
            if gap > m {
                let right_beats = w.1 > l.1;
                let met = beats(card) == right_beats;
                let (better, dir) = if right_beats {
                    (w.0, Direction::Timid)
                } else {
                    (l.0, Direction::Greedy)
                };
                let mut ch = Verdict::new(Skill::Worth, met, gap)
                    .better(better)
                    .toward(dir);
                // the trick as it went, or as it would have gone
                let taker = if right_beats && !met { better } else { card };
                ch.trick_thirds = led.thirds() + taker.thirds();
                out.push(ch);
            }
        }
        // cheap: among the cards that give the outcome played
        let same = |c: Card| beats(c) == beats(card);
        if scores.iter().filter(|(c, _)| same(*c)).count() >= 2 {
            let top = best(scores, same).expect("a card of the class");
            let low = worst(scores, same).expect("a card of the class");
            if top.1 - low > m {
                let lost = top.1 - played;
                out.push(Verdict::new(Skill::Cheap, lost <= m, lost).better(top.0));
            }
        }
    }

    // off: a cover or a hand card
    if let (Some(cover), Some(hand)) = (
        best(scores, |c| k.is_cover(c)),
        best(scores, |c| !k.is_cover(c)),
    ) {
        let gap = cover.1 - hand.1;
        if gap > m {
            let figure = cover.0.suit().is_some() && matches!(cover.0.strength(), 5 | 6);
            let dir = if figure {
                Direction::Figure
            } else {
                Direction::Pip
            };
            out.push(
                Verdict::new(Skill::Off, k.is_cover(card), gap)
                    .better(cover.0)
                    .toward(dir),
            );
        }
    }

    // run: which tarock to lead
    if k.lead.is_none() && card.is_tarock() {
        let top = best(scores, |_| true).expect("a card");
        let tarock = |c: Card| c.is_tarock();
        if top.0.is_tarock() && scores.iter().filter(|(c, _)| tarock(*c)).count() >= 2 {
            let t = best(scores, tarock).expect("a tarock");
            let low = worst(scores, tarock).expect("a tarock");
            if t.1 - low > m {
                let lost = t.1 - played;
                out.push(Verdict::new(Skill::Run, lost <= m, lost).better(t.0));
            }
        }
    }

    // pagat: play it now or keep it
    if let (Some(p), Some(o)) = (
        best(scores, |c| c == Card::PAGAT),
        best(scores, |c| c != Card::PAGAT),
    ) {
        let gap = (p.1 - o.1).abs();
        if gap > m {
            let play_it = p.1 > o.1;
            let met = (card == Card::PAGAT) == play_it;
            let (better, dir) = if play_it {
                (Card::PAGAT, Direction::Cash)
            } else {
                (o.0, Direction::Keep)
            };
            let mut ch = Verdict::new(Skill::Pagat, met, gap)
                .better(better)
                .toward(dir);
            ch.tarocks = k.my_tarocks() as u32;
            out.push(ch);
        }
    }

    if k.lead.is_none() {
        // void: a lead into a suit he has shown he is out of
        let into_void = |c: Card| c.suit().is_some_and(|s| shown_void(k, s));
        if let (Some(v), Some(o)) = (best(scores, into_void), best(scores, |c| !into_void(c))) {
            let gap = o.1 - v.1;
            if gap > m {
                out.push(Verdict::new(Skill::Void, !into_void(card), gap).better(o.0));
            }
        }
        // behind: opening a suit the hands share evenly
        let opens_split = |c: Card| c.suit().is_some_and(|s| is_split(k, s));
        if let (Some(s), Some(o)) = (best(scores, opens_split), best(scores, |c| !opens_split(c))) {
            let gap = o.1 - s.1;
            if gap > m {
                out.push(Verdict::new(Skill::Behind, !opens_split(card), gap).better(o.0));
            }
        }
    }

    // endgame: the position is known
    if exact(k) {
        let top = best(scores, |_| true).expect("a card");
        let low = worst(scores, |_| true).expect("a card");
        if top.1 - low > EXACT_MARGIN {
            let lost = top.1 - played;
            out.push(Verdict::new(Skill::Endgame, lost <= EXACT_MARGIN, lost).better(top.0));
        }
    }
    out
}

/// The verdict on a bid. `advantage` is the expectation in Points of taking
/// up against the simple game (`rung4::bid_advantage`, already times the
/// doubling); the margin scales with the doubling too.
pub fn classify_bid(k: &Knowledge, take: bool, advantage: f64) -> Option<Verdict> {
    if advantage.abs() <= BID_MARGIN * f64::from(k.multiplier) {
        return None;
    }
    let met = take == (advantage > 0.0);
    let dir = if take {
        Direction::Took
    } else {
        Direction::Passed
    };
    let mut ch = Verdict::new(Skill::Bid, met, advantage.abs() * THIRDS_PER_POINT).toward(dir);
    ch.tarocks = k.my_tarocks() as u32;
    ch.sure_thirds = crate::rung2::sure_thirds(k.hand).0;
    Some(ch)
}

/// The verdict on announcing: a held, unannounced honour is announced with
/// the action that announces it, or with the play of its first card; played
/// silent, it is forfeited (RULES 11.3, 11.5): a Point, doubled with the
/// deal. Exact, no search.
pub fn classify_announce(k: &Knowledge, action: &Action) -> Option<Verdict> {
    let met = match action {
        Action::Announce { honour } if k.announceable.contains(honour) => true,
        Action::Play { card, announce } => {
            let h = *k.announceable.iter().find(|h| h.cards().contains(card))?;
            announce.contains(&h)
        }
        _ => return None,
    };
    Some(Verdict::new(
        Skill::Announce,
        met,
        THIRDS_PER_POINT * f64::from(k.multiplier),
    ))
}

/// Every verdict on `action`, taken in `view` by the seat to move: the
/// advisor runs on that seat's view alone (the same draws for every card, a
/// pure function of the view and `budget`).
pub fn decision_verdicts(view: &View, action: &Action, budget: Budget) -> Vec<Verdict> {
    let Some(k) = Knowledge::from_view(view) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    if !k.playing {
        let take = match action {
            Action::Take => true,
            Action::Pass => false,
            _ => return out,
        };
        let mut rng = Rng::stream(EVIDENCE_SEED, view_seed(view) ^ 0x7002);
        let adv = rung4::bid_advantage(&k, &mut rng, budget.bid_worlds);
        out.extend(classify_bid(&k, take, adv));
        return out;
    }
    out.extend(classify_announce(&k, action));
    if let Action::Play { card, .. } = action {
        if k.legal.len() >= 2 {
            let mut budget = budget;
            if exact(&k) {
                // Nothing is hidden: every world is the same one, so spend
                // the nodes on it.
                budget.worlds = 1;
            }
            let mut rng = Rng::stream(EVIDENCE_SEED, view_seed(view) ^ 0x7001);
            let table = rung4::move_table(&k, &mut rng, budget, &Weights::default());
            out.extend(classify_play(&k, *card, &table.scores));
        }
    }
    out
}

/// A clear chance in the shape `kit/learner` takes (`learner::Chance`):
/// `skill` indexes [`Skill::ALL`], `gap` is the thirds a miss cost (0 when
/// met), in thirds of the count margin (1 Point = [`THIRDS_PER_POINT`]).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Chance {
    pub skill: usize,
    pub met: bool,
    pub gap: f64,
}

impl Verdict {
    pub fn chance(&self) -> Chance {
        Chance {
            skill: self.skill.index(),
            met: self.met,
            gap: self.cost,
        }
    }
}

/// What one deal showed of one skill: the clear chances, how many were met,
/// and the thirds the missed ones cost. Field names and types are those of
/// `kit/learner`'s `Tally` (Cassino's).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Tally {
    pub chances: u32,
    pub met: u32,
    pub missed: f64,
}

/// What one deal showed of each skill, in [`Skill::ALL`] order (as
/// `learner::Summary`).
#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    pub version: u32,
    pub tallies: Vec<Tally>,
}

impl Summary {
    /// As `learner::Summary::from_chances` for this ladder.
    pub fn from_chances(chances: &[Chance]) -> Summary {
        let mut tallies = vec![Tally::default(); SKILLS];
        for c in chances {
            let t = &mut tallies[c.skill];
            t.chances += 1;
            if c.met {
                t.met += 1;
            } else {
                t.missed += c.gap;
            }
        }
        Summary {
            version: EVIDENCE_VERSION,
            tallies,
        }
    }

    pub fn tally(&self, skill: Skill) -> Tally {
        self.tallies[skill.index()]
    }
}

/// What a figure that was not played at once came to.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Consequence {
    /// The better card, played later, fell under a higher card of its suit.
    FellUnder { by: Card, trick: u32 },
}

/// A missed chance, for the brief's example.
#[derive(Clone, Debug, PartialEq)]
pub struct Miss {
    pub skill: Skill,
    /// The action's number in the deal.
    pub action: usize,
    /// The trick, 1 to 27; 0 for a bid.
    pub trick: u32,
    pub played: Option<Card>,
    pub better: Option<Card>,
    pub cost: f64,
    pub direction: Option<Direction>,
    /// The card led to the trick, when the player followed.
    pub led: Option<Card>,
    pub trick_thirds: u32,
    pub tarocks: u32,
    pub sure_thirds: u32,
    pub consequence: Option<Consequence>,
}

/// One deal's evidence: the summary and, for each skill, its costliest miss.
#[derive(Clone, Debug, PartialEq)]
pub struct Evidence {
    /// Every clear chance of the deal, in order.
    pub chances: Vec<Chance>,
    pub summary: Summary,
    pub costliest: [Option<Miss>; SKILLS],
}

/// Whether `figure`, played in the deal's tricks by `human`, was taken by a
/// higher card of its suit.
fn fell_under(tricks: &[TrickRecord], figure: Card, human: Seat) -> Option<Consequence> {
    tricks.iter().enumerate().find_map(|(i, t)| {
        let at = t.cards.iter().position(|&c| c == figure)?;
        let other = t.cards[1 - at];
        (t.winner != human && other.suit() == figure.suit() && other.strength() > figure.strength())
            .then_some(Consequence::FellUnder {
                by: other,
                trick: i as u32 + 1,
            })
    })
}

/// The evidence of one scored deal for `human`: the record replayed, and at
/// every decision of the human's that is not in `assisted` (the numbers of
/// the actions a hint or a nudge was shown for, counted from 0 over both
/// seats' actions) the verdicts of [`decision_verdicts`].
pub fn evidence(
    record: &DealRecord,
    human: Seat,
    assisted: &[usize],
    budget: Budget,
) -> Result<Evidence, ReplayError> {
    let mut chances = Vec::new();
    let mut costliest: [Option<Miss>; SKILLS] = Default::default();
    let deal = record.replay(|index, deal, seat, action| {
        if seat != human || assisted.contains(&index) {
            return;
        }
        let view = bare_view(deal, seat);
        for ch in decision_verdicts(&view, action, budget) {
            chances.push(ch.chance());
            if ch.met {
                continue;
            }
            let slot = &mut costliest[ch.skill.index()];
            if slot.as_ref().is_none_or(|m| ch.cost > m.cost) {
                let d = view.deal.as_ref();
                *slot = Some(Miss {
                    skill: ch.skill,
                    action: index,
                    trick: d.map_or(0, |d| if d.playing { d.trick_number } else { 0 }),
                    played: match action {
                        Action::Play { card, .. } => Some(*card),
                        _ => None,
                    },
                    better: ch.better,
                    cost: ch.cost,
                    direction: ch.direction,
                    led: d.and_then(|d| d.lead.as_ref().map(|l| l.card)),
                    trick_thirds: ch.trick_thirds,
                    tarocks: ch.tarocks,
                    sure_thirds: ch.sure_thirds,
                    consequence: None,
                });
            }
        }
    })?;
    // After the last trick every card is public: what became of a figure.
    for miss in costliest.iter_mut().flatten() {
        if let (Some(Direction::Figure), Some(figure)) = (miss.direction, miss.better) {
            miss.consequence = fell_under(deal.tricks(), figure, human);
        }
    }
    let summary = Summary::from_chances(&chances);
    Ok(Evidence {
        chances,
        summary,
        costliest,
    })
}

#[cfg(test)]
#[path = "tutor_tests.rs"]
mod evidence_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{c, know};
    use strohmandeln_core::cards::Honour;

    const NO: [&str; 3] = ["", "", ""];
    /// An opponent cover keeps the position from being exact.
    const HID: [&str; 3] = ["T2", "", ""];

    fn sc(v: &[(&str, f64)]) -> Vec<(Card, f64)> {
        let mut s: Vec<(Card, f64)> = v.iter().map(|(x, s)| (c(x), *s)).collect();
        s.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        s
    }

    fn one(k: &Knowledge, played: &str, scores: &[(&str, f64)], skill: Skill) -> Option<Verdict> {
        classify_play(k, c(played), &sc(scores))
            .into_iter()
            .find(|x| x.skill == skill)
    }

    // ---- worth ----------------------------------------------------------

    #[test]
    fn worth_a_rich_trick_let_go_is_timid_and_costs_the_gap() {
        let k = know(&["Kh", "4h", "9c"], NO, HID, &[], Some("3h"));
        let s = [("Kh", 20.0), ("4h", 5.0)];
        let met = one(&k, "Kh", &s, Skill::Worth).unwrap();
        assert!(met.met && met.cost == 0.0);
        let missed = one(&k, "4h", &s, Skill::Worth).unwrap();
        assert!(!missed.met);
        assert_eq!(missed.cost, 15.0);
        assert_eq!(missed.direction, Some(Direction::Timid));
        assert_eq!(missed.better, Some(c("Kh")));
    }

    #[test]
    fn worth_a_poor_trick_won_with_the_king_is_greedy() {
        let k = know(&["Kh", "4h", "9c"], NO, HID, &[], Some("3h"));
        let s = [("Kh", 5.0), ("4h", 20.0)];
        let missed = one(&k, "Kh", &s, Skill::Worth).unwrap();
        assert!(!missed.met);
        assert_eq!(missed.direction, Some(Direction::Greedy));
        assert_eq!(missed.better, Some(c("4h")));
        assert_eq!(missed.cost, 15.0);
        // the trick it won: the lead and the king
        assert_eq!(missed.trick_thirds, c("3h").thirds() + c("Kh").thirds());
        assert!(one(&k, "4h", &s, Skill::Worth).unwrap().met);
    }

    #[test]
    fn worth_is_no_chance_inside_the_margin_or_for_a_leader() {
        let k = know(&["Kh", "4h", "9c"], NO, HID, &[], Some("3h"));
        assert!(one(&k, "Kh", &[("Kh", 10.0), ("4h", 6.0)], Skill::Worth).is_none());
        let lead = know(&["Kh", "4h", "9c"], NO, HID, &[], None);
        assert!(one(
            &lead,
            "Kh",
            &[("Kh", 30.0), ("4h", 0.0), ("9c", 0.0)],
            Skill::Worth
        )
        .is_none());
    }

    #[test]
    fn worth_needs_a_card_on_each_side() {
        // both beat the lead: one class only
        let k = know(&["Kh", "Qh"], NO, HID, &[], Some("3h"));
        assert!(one(&k, "Kh", &[("Kh", 30.0), ("Qh", 0.0)], Skill::Worth).is_none());
    }

    // ---- cheap ----------------------------------------------------------

    #[test]
    fn cheap_the_spread_among_the_cards_that_win() {
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], Some("3h"));
        let s = [("Kh", 20.0), ("Qh", 12.0)];
        assert!(one(&k, "Kh", &s, Skill::Cheap).unwrap().met);
        let m = one(&k, "Qh", &s, Skill::Cheap).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 8.0);
        assert_eq!(m.better, Some(c("Kh")));
        assert!(one(&k, "Qh", &[("Kh", 20.0), ("Qh", 17.0)], Skill::Cheap).is_none());
    }

    #[test]
    fn cheap_judges_the_class_of_the_card_played() {
        // Two cards do not beat the lead; the player gave the dearer.
        let k = know(&["Jh", "4h", "2d"], NO, HID, &[], Some("Qh"));
        let s = [("Jh", -10.0), ("4h", 0.0)];
        let m = one(&k, "Jh", &s, Skill::Cheap).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 10.0);
        assert!(one(&k, "4h", &s, Skill::Cheap).unwrap().met);
    }

    #[test]
    fn cheap_needs_two_cards_of_the_kind() {
        let k = know(&["Kh", "4h"], NO, HID, &[], Some("3h"));
        assert!(one(&k, "Kh", &[("Kh", 20.0), ("4h", 0.0)], Skill::Cheap).is_none());
    }

    // ---- off ------------------------------------------------------------

    #[test]
    fn off_a_cover_that_is_best_by_the_margin_is_a_chance() {
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        let s = [("Qh", 20.0), ("Kh", 10.0), ("4h", 5.0)];
        assert!(one(&k, "Qh", &s, Skill::Off).unwrap().met);
        let m = one(&k, "Kh", &s, Skill::Off).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 10.0);
        assert_eq!(m.better, Some(c("Qh")));
        assert_eq!(m.direction, Some(Direction::Figure));
    }

    #[test]
    fn off_a_low_cover_is_a_pip_miss() {
        let k = know(&["Kh", "Qh"], ["3h", "", ""], HID, &[], None);
        let m = one(
            &k,
            "Kh",
            &[("3h", 20.0), ("Kh", 5.0), ("Qh", 4.0)],
            Skill::Off,
        )
        .unwrap();
        assert_eq!(m.direction, Some(Direction::Pip));
    }

    #[test]
    fn off_nothing_is_shown_where_a_hand_card_is_best() {
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        let s = [("Kh", 20.0), ("Qh", 10.0), ("4h", 5.0)];
        assert!(one(&k, "Kh", &s, Skill::Off).is_none());
        assert!(
            one(&k, "Qh", &s, Skill::Off).is_none(),
            "no miss, no credit"
        );
        // near tie
        let s = [("Qh", 12.0), ("Kh", 10.0), ("4h", 5.0)];
        assert!(one(&k, "Kh", &s, Skill::Off).is_none());
    }

    // ---- run ------------------------------------------------------------

    #[test]
    fn run_which_tarock_to_lead() {
        let k = know(&["T10", "T9", "T3", "Kh"], NO, HID, &[], None);
        let s = [("T10", 12.0), ("T9", 11.0), ("T3", 2.0), ("Kh", 0.0)];
        assert!(
            one(&k, "T9", &s, Skill::Run).unwrap().met,
            "within the margin"
        );
        let m = one(&k, "T3", &s, Skill::Run).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 10.0);
        assert_eq!(m.better, Some(c("T10")));
    }

    #[test]
    fn run_is_no_chance_when_a_suit_lead_is_best_or_a_suit_was_led() {
        let k = know(&["T10", "T9", "T3", "Kh"], NO, HID, &[], None);
        let s = [("T10", 12.0), ("T9", 11.0), ("T3", 2.0), ("Kh", 20.0)];
        assert!(one(&k, "T3", &s, Skill::Run).is_none());
        let s = [("T10", 12.0), ("T9", 11.0), ("T3", 2.0), ("Kh", 0.0)];
        assert!(
            one(&k, "Kh", &s, Skill::Run).is_none(),
            "he did not lead a tarock"
        );
        let f = know(&["T10", "T9", "T3"], NO, HID, &[], Some("T5"));
        assert!(one(
            &f,
            "T10",
            &[("T10", 12.0), ("T9", 1.0), ("T3", 0.0)],
            Skill::Run
        )
        .is_none());
    }

    // ---- pagat ----------------------------------------------------------

    #[test]
    fn pagat_cash_it_or_keep_it() {
        let k = know(&["T1", "T9", "Kh"], NO, HID, &[], None);
        let cash = [("T1", 20.0), ("T9", 5.0), ("Kh", 3.0)];
        assert!(one(&k, "T1", &cash, Skill::Pagat).unwrap().met);
        let m = one(&k, "T9", &cash, Skill::Pagat).unwrap();
        assert!(!m.met);
        assert_eq!(m.direction, Some(Direction::Cash));
        assert_eq!(m.better, Some(c("T1")));
        assert_eq!(m.cost, 15.0);
        assert_eq!(m.tarocks, 2);
        let keep = [("T1", 3.0), ("T9", 10.0), ("Kh", 1.0)];
        let m = one(&k, "T1", &keep, Skill::Pagat).unwrap();
        assert!(!m.met);
        assert_eq!(m.direction, Some(Direction::Keep));
        assert_eq!(m.better, Some(c("T9")));
        assert!(one(&k, "T9", &keep, Skill::Pagat).unwrap().met);
        let tie = [("T1", 8.0), ("T9", 10.0), ("Kh", 1.0)];
        assert!(one(&k, "T1", &tie, Skill::Pagat).is_none());
    }

    #[test]
    fn pagat_needs_the_pagat_and_another_legal_card() {
        let k = know(&["T9", "Kh"], NO, HID, &[], None);
        assert!(one(&k, "T9", &[("T9", 20.0), ("Kh", 0.0)], Skill::Pagat).is_none());
        let k = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("T5"));
        // must follow with a tarock; the Pagat and T9 are both legal, Kh is not
        assert!(one(&k, "T9", &[("T1", 0.0), ("T9", 20.0)], Skill::Pagat).is_some());
    }

    // ---- void -----------------------------------------------------------

    #[test]
    fn void_leading_a_suit_he_has_shown_he_is_out_of() {
        let mut k = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        k.opp_void[0] = true; // hearts
        let s = [("Qd", 10.0), ("T3", 4.0), ("Kh", 0.0)];
        let m = one(&k, "Kh", &s, Skill::Void).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 10.0);
        assert_eq!(m.better, Some(c("Qd")));
        assert!(one(&k, "Qd", &s, Skill::Void).unwrap().met);
        assert!(one(&k, "T3", &s, Skill::Void).unwrap().met);
    }

    #[test]
    fn void_is_not_shown_when_a_cover_or_a_pulled_card_of_his_is_of_the_suit() {
        let s = [("Qd", 10.0), ("T3", 4.0), ("Kh", 0.0)];
        let mut k = know(&["Kh", "Qd", "T3"], NO, ["4h", "", ""], &[], None);
        k.opp_void[0] = true;
        assert!(one(&k, "Kh", &s, Skill::Void).is_none());
        let mut k = know(&["Kh", "Qd", "T3"], NO, HID, &["3h"], None);
        k.opp_void[0] = true;
        assert!(one(&k, "Kh", &s, Skill::Void).is_none());
        // and a trump void is not a lead into a suit he would trump
        let mut k = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        k.opp_void[4] = true;
        assert!(one(
            &k,
            "T3",
            &[("Qd", 10.0), ("T3", 0.0), ("Kh", 4.0)],
            Skill::Void
        )
        .is_none());
    }

    #[test]
    fn void_does_not_apply_to_a_follower_or_inside_the_margin() {
        let mut k = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        k.opp_void[0] = true;
        assert!(one(
            &k,
            "Kh",
            &[("Qd", 5.0), ("T3", 4.0), ("Kh", 0.0)],
            Skill::Void
        )
        .is_none());
        let mut f = know(&["Kh", "Qd", "T3"], NO, HID, &[], Some("Jd"));
        f.opp_void[0] = true;
        assert!(one(&f, "Qd", &[("Qd", 10.0), ("T3", 0.0)], Skill::Void).is_none());
    }

    // ---- behind ---------------------------------------------------------

    #[test]
    fn behind_opening_a_suit_the_hands_share_evenly() {
        // hearts: mine K+Q = 9 of the 18 points, the rest unseen: split.
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], None);
        let s = [("9c", 10.0), ("Kh", 0.0), ("Qh", 1.0)];
        let m = one(&k, "Kh", &s, Skill::Behind).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 9.0);
        assert_eq!(m.better, Some(c("9c")));
        assert!(one(&k, "9c", &s, Skill::Behind).unwrap().met);
    }

    #[test]
    fn behind_a_suit_that_is_mostly_mine_or_his_is_not_split() {
        // all four honours of hearts and more: 16 of 18 points are mine
        let k = know(&["Kh", "Qh", "Ch", "Jh", "9c"], NO, HID, &[], None);
        let s = [
            ("9c", 10.0),
            ("Kh", 0.0),
            ("Qh", 1.0),
            ("Ch", 0.0),
            ("Jh", 0.0),
        ];
        assert!(one(&k, "Kh", &s, Skill::Behind).is_none());
        // a lone 9 of clubs: 1 of 18, his suit
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], None);
        assert!(one(
            &k,
            "9c",
            &[("9c", 0.0), ("Kh", 10.0), ("Qh", 1.0)],
            Skill::Behind
        )
        .is_none());
    }

    #[test]
    fn behind_a_follower_does_not_open_anything() {
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], Some("Jh"));
        assert!(one(&k, "Kh", &[("Kh", 0.0), ("Qh", 10.0)], Skill::Behind).is_none());
    }

    // ---- endgame --------------------------------------------------------

    #[test]
    fn endgame_in_an_exact_position_the_margin_is_small() {
        let k = know(&["Kh", "4h", "T3"], NO, NO, &[], None);
        let s = [("Kh", 5.0), ("T3", 4.0), ("4h", 3.5)];
        assert!(one(&k, "Kh", &s, Skill::Endgame).unwrap().met);
        let m = one(&k, "4h", &s, Skill::Endgame).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 1.5);
        assert_eq!(m.better, Some(c("Kh")));
        // within the exact margin of the best: met
        assert!(one(&k, "T3", &s, Skill::Endgame).unwrap().met);
        assert!(one(
            &k,
            "4h",
            &[("Kh", 4.0), ("T3", 4.0), ("4h", 3.5)],
            Skill::Endgame
        )
        .is_none());
    }

    #[test]
    fn endgame_is_not_judged_while_a_packet_card_is_face_down() {
        let k = know(&["Kh", "4h", "T3"], NO, HID, &[], None);
        let s = [("Kh", 50.0), ("T3", 4.0), ("4h", 3.5)];
        assert!(one(&k, "4h", &s, Skill::Endgame).is_none());
        let k = know(&["Kh", "4h", "T3"], ["Qd", "", ""], NO, &[], None);
        assert!(one(&k, "4h", &s, Skill::Endgame).is_none());
    }

    #[test]
    fn the_margin_is_the_exact_one_only_when_the_position_is_exact() {
        // A 3 thirds gap: a chance when exact, noise otherwise.
        let s = [("Kh", 8.0), ("4h", 5.0)];
        let exact = know(&["Kh", "4h"], NO, NO, &[], Some("3h"));
        assert!(one(&exact, "4h", &s, Skill::Worth).is_some());
        let open = know(&["Kh", "4h"], NO, HID, &[], Some("3h"));
        assert!(one(&open, "4h", &s, Skill::Worth).is_none());
    }

    // ---- bid ------------------------------------------------------------

    #[test]
    fn bid_the_expectation_in_points_decides_which_call_is_right() {
        let k = know(&["Kh"], NO, HID, &[], None);
        let m = classify_bid(&k, false, 3.0).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 36.0);
        assert_eq!(m.direction, Some(Direction::Passed));
        assert!(classify_bid(&k, true, 3.0).unwrap().met);
        let m = classify_bid(&k, true, -2.0).unwrap();
        assert!(!m.met);
        assert_eq!(m.direction, Some(Direction::Took));
        assert_eq!(m.cost, 24.0);
        assert!(classify_bid(&k, false, -2.0).unwrap().met);
        assert!(classify_bid(&k, true, 0.2).is_none());
        assert!(classify_bid(&k, false, -0.2).is_none());
    }

    #[test]
    fn bid_the_margin_scales_with_the_doubling() {
        let mut k = know(&["Kh"], NO, HID, &[], None);
        k.multiplier = 2;
        assert!(classify_bid(&k, false, 0.8).is_none());
        assert!(classify_bid(&k, false, 1.5).is_some());
    }

    // ---- announce -------------------------------------------------------

    #[test]
    fn announce_playing_a_card_of_a_held_honour_without_announcing_is_a_miss() {
        let mut k = know(&["SK", "T21", "T1"], NO, HID, &[], None);
        k.announceable = vec![Honour::Trull];
        let silent = classify_announce(&k, &Action::play(c("T21"))).unwrap();
        assert!(!silent.met);
        assert_eq!(silent.cost, 12.0);
        assert_eq!(silent.skill, Skill::Announce);
        let said = Action::Play {
            card: c("T21"),
            announce: vec![Honour::Trull],
        };
        assert!(classify_announce(&k, &said).unwrap().met);
        let first = Action::Announce {
            honour: Honour::Trull,
        };
        assert!(classify_announce(&k, &first).unwrap().met);
    }

    #[test]
    fn announce_no_chance_without_a_held_honour_or_for_another_card() {
        let mut k = know(&["SK", "T21", "Kh"], NO, HID, &[], None);
        assert!(
            classify_announce(&k, &Action::play(c("T21"))).is_none(),
            "not held"
        );
        k.announceable = vec![Honour::Trull];
        assert!(classify_announce(&k, &Action::play(c("Kh"))).is_none());
        assert!(classify_announce(&k, &Action::Take).is_none());
    }

    #[test]
    fn announce_a_point_is_doubled_with_the_doubling() {
        let mut k = know(&["SK", "T21", "T1"], NO, HID, &[], None);
        k.announceable = vec![Honour::Trull];
        k.multiplier = 2;
        assert_eq!(
            classify_announce(&k, &Action::play(c("T1"))).unwrap().cost,
            24.0
        );
    }
}
