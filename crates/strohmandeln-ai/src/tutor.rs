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
use kit_learner::{Chance, Ladder, Params, SkillSpec, Summary};
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::deal::TrickRecord;
use strohmandeln_core::dealrec::{DealRecord, ReplayError};
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::{Action, Seat};
use strohmandeln_core::view::{bare_view, View};

/// The version of what [`evidence`] computes. Bump it whenever a classifier,
/// a constant or the advisor changes, so that a stored summary made before is
/// recomputed from its record (`docs/TUTOR.md` 3).
pub const EVIDENCE_VERSION: u32 = 3;

/// The resolution of the evidence: a player who loses less than this on a
/// decision is perfect in it, and a miss costs at least this (so `top_cost` is
/// conditional on it). The least gap in thirds that can be a clear chance: a
/// class must beat the next by more than this and by more than twice the
/// standard error of the per-world difference ([`Advice::std_error`]), and a
/// spread card is met unless it is worse than the best by as much.
///
/// **Measured, M1** (`docs/MEASUREMENTS.md`): twice the median across-stream
/// standard deviation of the gap, 5 thirds, over 596 positions each re-drawn
/// under 8 independent streams. The two-standard-error rule alone is honest
/// (calibration 0.97 to 1.19, sign replication 0.97 to 1.00).
pub const MARGIN: f64 = 10.0;

/// The same where the search solved the position (nothing face down and the
/// search reached the last trick, [`search_complete`]): the least loss that
/// counts. **Measured, M1**: an exact position has no spread across streams
/// (68 exact reads of position and skill, all zero), so it is the resolution of the
/// solver, one third.
pub const EXACT_MARGIN: f64 = 1.0;

/// The least expectation in Points, per unit of doubling, that can make a bid
/// a clear chance, besides twice its standard error over the sampled worlds
/// (`rung4::bid_samples`). **Measured, M1**: twice the median across-stream
/// standard deviation of the bid mean (0.217 Point), 200 bids under 8 streams.
pub const BID_MARGIN: f64 = 0.45;

/// Thirds in a Point, to price a bid or an announcement in the unit of the
/// rest. **Measured, M2** (`docs/MEASUREMENTS.md`): the reciprocal of the slope
/// of a deal's net Points (per unit of doubling) on its count margin in
/// thirds, 1,800 deals of rung 4 against rung 4: 0.0426 Point a third, 23.5
/// thirds a Point (95% interval 22.7 to 24.3). The solver's own weight of the
/// ultimo (`sim::ULTIMO_THIRDS`, 12) is lower and is not changed here: it is
/// the opponent's.
pub const THIRDS_PER_POINT: f64 = 23.0;

/// A suit is split when my share (my value points over mine and everything
/// unseen, of the suit) lies in this range. **Measured, M8**
/// (`docs/MEASUREMENTS.md`): over 36,665 suit-leads of 100 sittings, rung 4
/// surely prefers another lead for shares up to 0.6 (the mean gap crosses 0
/// there), and opens the suit in at least a tenth of the cases from 0.1 (below
/// it the suit is his). Rung 2 uses 30% to 70% of the suit's whole 18 points,
/// a different quantity.
pub const SPLIT_SHARE: (f64, f64) = (0.1, 0.6);

/// Unplayed value points a suit needs to be worth contesting. **Measured,
/// M8**: from 6 points unplayed, with the share in the band, another lead is
/// clearly better (beyond `MARGIN`) in at least a quarter of the cases (26%,
/// 35%, 52% for 6 to 9, 9 to 12, 12 and more; 15% and 19% below 6).
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
    /// `pagat`, `bid`: the player's tarocks now (hand and covers, as the
    /// sources count "neben nur wenigen Taroks"); the brief tells a `pagat`
    /// rule only inside its sourced zones, the direction is not tied to one.
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

/// Whether a search that reached `depth` tricks (on average over its worlds)
/// solved the position: nothing is face down, so the other hand follows from
/// the log, and the search went to the last trick (`rung4::move_table` deepens
/// trick by trick while its node budget lasts).
pub fn search_complete(k: &Knowledge, depth: f64) -> bool {
    k.playing && k.hidden_total() == 0 && depth + 1e-9 >= f64::from(28 - k.trick_number)
}

/// What the advisor said of the legal cards of one decision.
#[derive(Copy, Clone, Debug)]
pub struct Advice<'a> {
    /// The average value in thirds of each legal card over the sampled
    /// worlds, for the mover (higher is better).
    pub scores: &'a [(Card, f64)],
    /// The value of each legal card in each world (`MoveTable::per_world`):
    /// what gives a gap its standard error. Empty: no spread known.
    pub per_world: &'a [Vec<(Card, i32)>],
    /// The position is solved exactly ([`search_complete`]).
    pub exact: bool,
    /// Nothing is face down: the opponent's hand follows from the log. Such a
    /// position is searched in one world by design, so a single world is no
    /// sign of a starved budget there.
    pub known: bool,
}

impl Advice<'_> {
    /// The least gap that can count: the margin, the exact one only where
    /// the search solved the position.
    fn margin(&self) -> f64 {
        if self.exact {
            EXACT_MARGIN
        } else {
            MARGIN
        }
    }

    /// The advisor's value of `card` in thirds (NaN if it is not a legal card).
    pub fn value(&self, card: Card) -> f64 {
        self.scores
            .iter()
            .find(|(c, _)| *c == card)
            .map_or(f64::NAN, |&(_, v)| v)
    }

    /// The standard error of the mean of `a` less `b` over the worlds, the
    /// pairs taken world by world (as `hint.rs` `odds_for` does): the same
    /// worlds serve both cards, so what they share cancels. Fewer than two
    /// worlds say nothing of the spread: infinity (nothing clears the noise
    /// rule) for an open position, where it means a budget that finished one
    /// world of many; 0 for a known position, which is searched in one world
    /// by design and where the margin is all there is.
    pub fn std_error(&self, a: Card, b: Card) -> f64 {
        let diffs: Vec<f64> = self
            .per_world
            .iter()
            .filter_map(|row| {
                let v = |c: Card| {
                    row.iter()
                        .find(|(x, _)| *x == c)
                        .map(|&(_, v)| f64::from(v))
                };
                Some(v(a)? - v(b)?)
            })
            .collect();
        let n = diffs.len() as f64;
        if n < 2.0 {
            return if self.known { 0.0 } else { f64::INFINITY };
        }
        let mean = diffs.iter().sum::<f64>() / n;
        let var = diffs.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / (n - 1.0);
        (var / n).sqrt()
    }

    /// Whether `hi` is clearly better than `lo`: by more than the margin and
    /// than twice the standard error of the difference.
    fn clearly_better(&self, hi: Card, lo: Card) -> bool {
        let gap = self.value(hi) - self.value(lo);
        gap > self.margin() && gap > 2.0 * self.std_error(hi, lo)
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

/// The worst (first of equals) of the values whose card is in `class`.
fn worst(scores: &[(Card, f64)], class: impl Fn(Card) -> bool) -> Option<(Card, f64)> {
    scores
        .iter()
        .filter(|(c, _)| class(*c))
        .fold(None, |w, &(c, v)| match w {
            Some((_, wv)) if wv <= v => w,
            _ => Some((c, v)),
        })
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
/// cannot tell from his (so the share is biased low by about my hidden
/// fraction of the suit).
pub(crate) fn is_split(k: &Knowledge, suit: Suit) -> bool {
    let (mine, total) = suit_share(k, suit);
    if total < SPLIT_MIN_POINTS {
        return false;
    }
    let share = f64::from(mine) / f64::from(total);
    (SPLIT_SHARE.0..=SPLIT_SHARE.1).contains(&share)
}

/// My value points of `suit` among the unplayed cards, and all of them (mine
/// and "his", see [`is_split`]).
pub fn suit_share(k: &Knowledge, suit: Suit) -> (u32, u32) {
    let mine = points_in(k.playable().iter(), suit);
    let theirs = points_in(k.opp_possible().iter(), suit);
    (mine, mine + theirs)
}

/// A suit he has shown he is out of: he could not follow, and no cover of his
/// and no card of his known from a pull is of it (the counter rung's own
/// inference, `Knowledge::opp_void`).
pub(crate) fn shown_void(k: &Knowledge, suit: Suit) -> bool {
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
/// values of every legal card. A skill with no clear chance gives none. A
/// chance needs the right class to beat the next by more than the margin and
/// than twice the standard error of the difference ([`Advice::std_error`]);
/// a spread card is met unless it is clearly worse than the best of its kind.
pub fn classify_play(k: &Knowledge, card: Card, advice: &Advice) -> Vec<Verdict> {
    classify_play_probed(k, card, advice, &mut |_, _, _| {})
}

/// As [`classify_play`], and `probe(skill, a, b)` hears the pair of cards each
/// skill compares whenever its question applies, clear or not: `a` the best
/// card of the class the skill's rule favours, `b` the best of the other (for
/// the spread skills, the best and the worst of the kind). The gap
/// `value(a) - value(b)` is what the margin is tested against; `measure.rs`
/// (M1) re-draws it under other streams.
pub fn classify_play_probed(
    k: &Knowledge,
    card: Card,
    advice: &Advice,
    probe: &mut dyn FnMut(Skill, Card, Card),
) -> Vec<Verdict> {
    let scores = advice.scores;
    let mut out = Vec::new();
    if scores.len() < 2 {
        return out;
    }
    let played = advice.value(card);
    if played.is_nan() {
        return out;
    }
    let better = |hi: Card, lo: Card| advice.clearly_better(hi, lo);
    // The gap lost by `card` against the best of its kind `top`, if clearly.
    let lost_to =
        |top: Card| (top != card && better(top, card)).then(|| advice.value(top) - played);

    if let Some(led) = k.lead {
        // The follower. A card beats the lead when the lead does not win.
        let beats = |c: Card| !leader_wins(led, c);
        // worth: win the trick, or let it go
        if let (Some(w), Some(l)) = (best(scores, beats), best(scores, |c| !beats(c))) {
            let (hi, lo) = if w.1 > l.1 { (w, l) } else { (l, w) };
            probe(Skill::Worth, w.0, l.0);
            if better(hi.0, lo.0) {
                let right_beats = w.1 > l.1;
                let met = beats(card) == right_beats;
                let dir = if right_beats {
                    Direction::Timid
                } else {
                    Direction::Greedy
                };
                let mut ch = Verdict::new(Skill::Worth, met, hi.1 - lo.1)
                    .better(hi.0)
                    .toward(dir);
                // the trick as it went, or as it would have gone
                let taker = if right_beats && !met { hi.0 } else { card };
                ch.trick_thirds = led.thirds() + taker.thirds();
                out.push(ch);
            }
        }
        // cheap: among the cards that give the outcome played
        let same = |c: Card| beats(c) == beats(card);
        if let (Some(top), Some(low)) = (best(scores, same), worst(scores, same)) {
            if top.0 != low.0 {
                probe(Skill::Cheap, top.0, low.0);
            }
            if better(top.0, low.0) {
                let lost = lost_to(top.0);
                out.push(
                    Verdict::new(Skill::Cheap, lost.is_none(), lost.unwrap_or(0.0)).better(top.0),
                );
            }
        }
    }

    // off: a cover or a hand card
    if let (Some(cover), Some(hand)) = (
        best(scores, |c| k.is_cover(c)),
        best(scores, |c| !k.is_cover(c)),
    ) {
        probe(Skill::Off, cover.0, hand.0);
        if better(cover.0, hand.0) {
            let figure = cover.0.suit().is_some() && matches!(cover.0.strength(), 5 | 6);
            let dir = if figure {
                Direction::Figure
            } else {
                Direction::Pip
            };
            out.push(
                Verdict::new(Skill::Off, k.is_cover(card), cover.1 - hand.1)
                    .better(cover.0)
                    .toward(dir),
            );
        }
    }

    // run: which tarock to lead. Deliberately silent when a suit card was led
    // where a tarock lead is best: only *which* tarock is a skill here
    // (`research/16`, 2).
    if k.lead.is_none() && card.is_tarock() {
        let top = best(scores, |_| true).expect("a card");
        let tarock = |c: Card| c.is_tarock();
        if let (true, Some(t), Some(low)) = (
            top.0.is_tarock(),
            best(scores, tarock),
            worst(scores, tarock),
        ) {
            if t.0 != low.0 {
                probe(Skill::Run, t.0, low.0);
            }
            if t.0 != low.0 && better(t.0, low.0) {
                let lost = lost_to(t.0);
                out.push(Verdict::new(Skill::Run, lost.is_none(), lost.unwrap_or(0.0)).better(t.0));
            }
        }
    }

    // pagat: play it now or keep it. Only when following. Leading the Pagat
    // is almost always worse than a tarock by the margin, so keeping it is
    // trivial: measured at b3ffb01 (before the gate; the leader row of
    // `print_pagat_variants` at that commit, 9 sittings, default budget), rung 4
    // met 107 of 107 leads and a random player 83 of 91. Not reproducible at
    // HEAD, where a lead is no chance. Following, at HEAD and reproducible
    // (`cargo test -p strohmandeln-ai --release print_pagat_variants --
    // --ignored --nocapture`, 9 sittings): rung 4 32 of 34, random 15 of 29.
    if let (true, Some(p), Some(o)) = (
        k.lead.is_some(),
        best(scores, |c| c == Card::PAGAT),
        best(scores, |c| c != Card::PAGAT),
    ) {
        let (hi, lo) = if p.1 > o.1 { (p, o) } else { (o, p) };
        probe(Skill::Pagat, p.0, o.0);
        if better(hi.0, lo.0) {
            let play_it = p.1 > o.1;
            let met = (card == Card::PAGAT) == play_it;
            let dir = if play_it {
                Direction::Cash
            } else {
                Direction::Keep
            };
            let mut ch = Verdict::new(Skill::Pagat, met, hi.1 - lo.1)
                .better(hi.0)
                .toward(dir);
            ch.tarocks = k.my_tarocks() as u32;
            out.push(ch);
        }
    }

    if k.lead.is_none() {
        // void: a lead into a suit he has shown he is out of
        let into_void = |c: Card| c.suit().is_some_and(|s| shown_void(k, s));
        if let (Some(v), Some(o)) = (best(scores, into_void), best(scores, |c| !into_void(c))) {
            probe(Skill::Void, o.0, v.0);
            if better(o.0, v.0) {
                out.push(Verdict::new(Skill::Void, !into_void(card), o.1 - v.1).better(o.0));
            }
        }
        // behind: opening a suit the hands share evenly
        let opens_split = |c: Card| c.suit().is_some_and(|s| is_split(k, s));
        if let (Some(s), Some(o)) = (best(scores, opens_split), best(scores, |c| !opens_split(c))) {
            probe(Skill::Behind, o.0, s.0);
            if better(o.0, s.0) {
                out.push(Verdict::new(Skill::Behind, !opens_split(card), o.1 - s.1).better(o.0));
            }
        }
    }

    // endgame: the position is known and solved
    if advice.exact {
        if let (Some(top), Some(low)) = (best(scores, |_| true), worst(scores, |_| true)) {
            if top.0 != low.0 {
                probe(Skill::Endgame, top.0, low.0);
            }
            if better(top.0, low.0) {
                let lost = lost_to(top.0);
                out.push(
                    Verdict::new(Skill::Endgame, lost.is_none(), lost.unwrap_or(0.0)).better(top.0),
                );
            }
        }
    }
    out
}

/// The verdict on a bid. `mean` is the expectation in Points of taking up
/// against the simple game over the sampled worlds, per unit of doubling
/// (`rung4::bid_samples`), and `se` its standard error: a bid is a chance
/// when `mean` is beyond [`BID_MARGIN`] and twice `se`. The cost is in thirds
/// of the count, not scaled by the doubling.
pub fn classify_bid(k: &Knowledge, take: bool, mean: f64, se: f64) -> Option<Verdict> {
    if mean.abs() <= BID_MARGIN || mean.abs() <= 2.0 * se {
        return None;
    }
    let met = take == (mean > 0.0);
    let dir = if take {
        Direction::Took
    } else {
        Direction::Passed
    };
    let mut ch = Verdict::new(Skill::Bid, met, mean.abs() * THIRDS_PER_POINT).toward(dir);
    ch.tarocks = k.my_tarocks() as u32;
    ch.sure_thirds = crate::rung2::sure_thirds(k.hand).0;
    Some(ch)
}

/// The verdict on announcing: a held, unannounced honour is announced with
/// the action that announces it, or with the play of its first card; played
/// silent, it is forfeited (RULES 11.3, 11.5): a Point, [`THIRDS_PER_POINT`]
/// thirds, whatever the doubling. Exact, no search.
pub fn classify_announce(k: &Knowledge, action: &Action) -> Option<Verdict> {
    let met = match action {
        Action::Announce { honour } if k.announceable.contains(honour) => true,
        Action::Play { card, announce } => {
            let h = *k.announceable.iter().find(|h| h.cards().contains(card))?;
            announce.contains(&h)
        }
        _ => return None,
    };
    Some(Verdict::new(Skill::Announce, met, THIRDS_PER_POINT))
}

/// The stream of the advisor's draws for stream number `salt` of `view`: the
/// evidence's own for salt 0, an independent one for any other (M1 re-draws a
/// position under several).
fn advisor_rng(view: &View, salt: u64, purpose: u64) -> Rng {
    Rng::stream(
        EVIDENCE_SEED ^ salt.wrapping_mul(0x9e37_79b9_7f4a_7c15),
        view_seed(view) ^ purpose,
    )
}

/// The advisor's table for a play in `view` (`k` its knowledge), drawn from
/// stream `salt`; `None` when the budget finished no trick, so that the
/// values are not in thirds. A position with nothing face down is the same
/// world every time, so the nodes are spent on one.
pub fn play_table(
    k: &Knowledge,
    view: &View,
    budget: Budget,
    salt: u64,
) -> Option<rung4::MoveTable> {
    let mut budget = budget;
    if k.hidden_total() == 0 {
        budget.nodes *= budget.worlds as u64;
        budget.worlds = 1;
    }
    let mut rng = advisor_rng(view, salt, 0x7001);
    let table = rung4::move_table(k, &mut rng, budget, &Weights::default());
    (!table.per_world.is_empty()).then_some(table)
}

/// The advice a table gives (`k` its position).
pub fn advice_of<'a>(k: &Knowledge, table: &'a rung4::MoveTable) -> Advice<'a> {
    Advice {
        scores: &table.scores,
        per_world: &table.per_world,
        exact: search_complete(k, table.depth),
        known: k.hidden_total() == 0,
    }
}

/// The expectation in Points of taking up, per unit of doubling, over the
/// sampled worlds of stream `salt`, and its standard error; `None` for fewer
/// than two worlds.
pub fn bid_estimate(k: &Knowledge, view: &View, budget: Budget, salt: u64) -> Option<(f64, f64)> {
    let mut rng = advisor_rng(view, salt, 0x7002);
    let samples = rung4::bid_samples(k, &mut rng, budget.bid_worlds);
    let n = samples.len() as f64;
    if n < 2.0 {
        return None;
    }
    let mean = samples.iter().sum::<f64>() / n;
    let var = samples.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    Some((mean, (var / n).sqrt()))
}

/// Every verdict on `action`, taken in `view` by the seat to move: the
/// advisor runs on that seat's view alone (the same draws for every card, a
/// pure function of the view and `budget`). A decision whose search finished
/// no trick (a budget too small, `MoveTable` then holds rung 3's scale) gives
/// no play verdict.
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
        if let Some((mean, se)) = bid_estimate(&k, view, budget, 0) {
            out.extend(classify_bid(&k, take, mean, se));
        }
        return out;
    }
    out.extend(classify_announce(&k, action));
    if let Action::Play { card, .. } = action {
        if k.legal.len() >= 2 {
            if let Some(table) = play_table(&k, view, budget, 0) {
                out.extend(classify_play(&k, *card, &advice_of(&k, &table)));
            }
        }
    }
    out
}

impl Verdict {
    /// The clear chance in the learner's shape: the skill's index in
    /// [`LADDER`] and the thirds a miss cost.
    pub fn chance(&self) -> Chance {
        Chance {
            skill: self.skill.index(),
            met: self.met,
            gap: self.cost,
        }
    }
}

/// The ladder the learner folds (`docs/TUTOR.md` 13). Prerequisites are those
/// of [`Skill::needs`] (a test holds them equal).
///
/// **Measured, M3** (`docs/MEASUREMENTS.md`): `top_rate` and `top_cost` are
/// what rung 4 met and lost per deal against rung 3, both seats, 1,200 deals
/// (200 sittings) at the default settings, after the noise rule and the margin
/// of M1; `top_cost` is conditional on the resolution (a miss costs at least
/// `MARGIN`). `min_chances` is half the chances a skill offers in six deals,
/// rounded down, between 2 and 4.
pub const LADDER: Ladder = Ladder {
    name: "strohmandeln",
    version: EVIDENCE_VERSION,
    skills: &[
        spec("worth", &[], 0.99, 0.32, 4.0),
        spec("cheap", &[0], 0.96, 1.35, 4.0),
        spec("off", &[0], 0.92, 0.72, 2.0),
        spec("run", &[1], 0.95, 0.93, 3.0),
        spec("pagat", &[3], 0.98, 0.22, 2.0),
        spec("void", &[0], 0.96, 0.62, 2.0),
        spec("behind", &[2, 5], 0.96, 1.42, 4.0),
        spec("endgame", &[1, 5], 0.93, 0.86, 3.0),
        spec("bid", &[0], 0.99, 0.09, 2.0),
        spec("announce", &[], 1.0, 0.0, 2.0),
    ],
};

/// The learner's constants. **Provisional** except `discount`: a window of
/// about a sitting of six deals, 1 - 1/6 (M3; M6 tunes it); the gains are set
/// by M4 and the quantiles and mastery by M6.
pub const PARAMS: Params = Params {
    discount: 0.83,
    min_gain: 1.0,
    costly_gain: 4.0,
    costly_chances: 8.0,
    lower: 0.2,
    upper: 0.9,
    master_of_top: 0.85,
    lose_below: 0.2,
};

const fn spec(
    slug: &'static str,
    needs: &'static [usize],
    top_rate: f64,
    top_cost: f64,
    min_chances: f64,
) -> SkillSpec {
    SkillSpec {
        slug,
        needs,
        top_rate,
        top_cost,
        min_chances,
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

/// The actions a hint or a nudge was shown for, and the rest of the turn they
/// were shown in: a hint is for the position, and the human may announce
/// (any number of times) before the card the hint was about, so an
/// assisted `Announce` also covers the actions of the same seat that follow
/// it up to and including the card played.
fn assisted_actions(record: &DealRecord, assisted: &[usize]) -> Vec<bool> {
    let mut covered = vec![false; record.actions.len()];
    for &first in assisted {
        let mut at = first;
        while let Some(m) = record.actions.get(at) {
            covered[at] = true;
            let next = record.actions.get(at + 1);
            if matches!(m.action, Action::Announce { .. }) && next.is_some_and(|n| n.seat == m.seat)
            {
                at += 1;
            } else {
                break;
            }
        }
    }
    covered
}

/// The evidence of one scored deal for `human`: the record replayed, and at
/// every decision of the human's that is not assisted (`assisted` holds the
/// numbers of the actions a hint or a nudge was shown for, counted from 0
/// over both seats' actions, `Sitting::deal_action_count`; see
/// [`assisted_actions`]) the verdicts of [`decision_verdicts`].
pub fn evidence(
    record: &DealRecord,
    human: Seat,
    assisted: &[usize],
    budget: Budget,
) -> Result<Evidence, ReplayError> {
    let mut chances = Vec::new();
    let mut costliest: [Option<Miss>; SKILLS] = Default::default();
    let covered = assisted_actions(record, assisted);
    let deal = record.replay_to_end(|index, deal, seat, action| {
        if seat != human || covered[index] {
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
    let summary = Summary::from_chances(&LADDER, &chances);
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

    /// The tests' open-position values are written for a margin of 5; this
    /// brings them to `MARGIN` (a test holds the fit).
    const OPEN: f64 = 2.0;
    const NO: [&str; 3] = ["", "", ""];
    /// An opponent cover keeps the position from being exact.
    const HID: [&str; 3] = ["T2", "", ""];

    fn sc(v: &[(&str, f64)]) -> Vec<(Card, f64)> {
        let mut s: Vec<(Card, f64)> = v.iter().map(|(x, s)| (c(x), *s)).collect();
        s.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        s
    }

    /// The per-world values of the cards: each world moves a card by
    /// `noise` up or down, alternately by position, so that two cards'
    /// difference has a spread of about twice `noise` and the means stay.
    fn rows(scores: &[(Card, f64)], noise: f64) -> Vec<Vec<(Card, i32)>> {
        (0..10)
            .map(|i| {
                scores
                    .iter()
                    .enumerate()
                    .map(|(j, &(card, v))| {
                        let sign = if (i >> j) & 1 == 1 { 1.0 } else { -1.0 };
                        (card, (v + sign * noise).round() as i32)
                    })
                    .collect()
            })
            .collect()
    }

    fn one_with(
        k: &Knowledge,
        played: &str,
        scores: &[(&str, f64)],
        noise: f64,
        exact: bool,
        skill: Skill,
    ) -> Option<Verdict> {
        // the values are written for a margin of 5 and doubled where the
        // search is not exact (`OPEN`); an exact position keeps them
        let scale = if exact { 1.0 } else { OPEN };
        let scores: Vec<(Card, f64)> = sc(scores)
            .into_iter()
            .map(|(c, v)| (c, v * scale))
            .collect();
        let per_world = rows(&scores, noise);
        let advice = Advice {
            scores: &scores,
            per_world: &per_world,
            exact,
            known: k.hidden_total() == 0,
        };
        classify_play(k, c(played), &advice)
            .into_iter()
            .find(|x| x.skill == skill)
    }

    /// A position with no packet card face down is taken as exactly solved.
    fn one(k: &Knowledge, played: &str, scores: &[(&str, f64)], skill: Skill) -> Option<Verdict> {
        one_with(k, played, scores, 0.0, k.hidden_total() == 0, skill)
    }

    // ---- the advisor's noise and its exactness ---------------------------

    fn advice<'a>(
        scores: &'a [(Card, f64)],
        per_world: &'a [Vec<(Card, i32)>],
        known: bool,
    ) -> Advice<'a> {
        Advice {
            scores,
            per_world,
            exact: false,
            known,
        }
    }

    #[test]
    fn the_standard_error_is_that_of_the_paired_differences() {
        let (a, b) = (c("Kh"), c("4h"));
        let scores = [(a, 10.0), (b, 9.0)];
        // differences 0 and 2: mean 1, sample variance 2, error sqrt(2 / 2)
        let rows = vec![vec![(a, 10), (b, 10)], vec![(a, 10), (b, 8)]];
        assert!((advice(&scores, &rows, false).std_error(a, b) - 1.0).abs() < 1e-12);
        // the sign of the pair does not matter
        assert!((advice(&scores, &rows, false).std_error(b, a) - 1.0).abs() < 1e-12);
        // a world that lacks one of the cards is dropped
        let mut with_gap = rows.clone();
        with_gap.push(vec![(a, 99)]);
        assert!((advice(&scores, &with_gap, false).std_error(a, b) - 1.0).abs() < 1e-12);
        // a card against itself has no spread
        assert_eq!(advice(&scores, &rows, false).std_error(a, a), 0.0);
        // steady worlds: no error
        let steady = vec![vec![(a, 10), (b, 5)]; 6];
        assert_eq!(advice(&scores, &steady, false).std_error(a, b), 0.0);
    }

    #[test]
    fn one_counted_world_clears_nothing_unless_the_position_is_known() {
        let (a, b) = (c("Kh"), c("4h"));
        let scores = [(a, 40.0), (b, 0.0)];
        let one = vec![vec![(a, 40), (b, 0)]];
        // a budget that finished one world of ten: the noisiest estimate
        // may not get the loosest test
        let open = advice(&scores, &one, false);
        assert!(open.std_error(a, b).is_infinite());
        assert!(!open.clearly_better(a, b));
        // a known position is searched in one world by design: the margin
        // alone, the error being the horizon's
        let known = advice(&scores, &one, true);
        assert_eq!(known.std_error(a, b), 0.0);
        assert!(known.clearly_better(a, b));
        // no worlds at all
        assert!(advice(&scores, &[], false).std_error(a, b).is_infinite());
    }

    #[test]
    fn a_gap_inside_twice_its_standard_error_is_no_chance() {
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        let s = [("Qh", 20.0), ("Kh", 10.0), ("4h", 5.0)];
        // the same means: steady worlds make a clear chance, wild ones do not
        assert!(one_with(&k, "Kh", &s, 0.0, false, Skill::Off).is_some());
        assert!(one_with(&k, "Kh", &s, 2.0, false, Skill::Off).is_some());
        assert!(one_with(&k, "Kh", &s, 30.0 * OPEN, false, Skill::Off).is_none());
        assert!(one_with(&k, "Qh", &s, 30.0 * OPEN, false, Skill::Off).is_none());
        // worth, cheap, pagat, void, behind, run are held to the same rule
        let f = know(&["Kh", "Qh", "4h"], NO, HID, &[], Some("3h"));
        let w = [("Kh", 20.0), ("Qh", 12.0), ("4h", 0.0)];
        assert!(one_with(&f, "4h", &w, 0.0, false, Skill::Worth).is_some());
        assert!(one_with(&f, "4h", &w, 30.0 * OPEN, false, Skill::Worth).is_none());
        assert!(one_with(&f, "Qh", &w, 0.0, false, Skill::Cheap).is_some());
        assert!(one_with(&f, "Qh", &w, 30.0 * OPEN, false, Skill::Cheap).is_none());
        let p = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("3d"));
        let ps = [("T1", 20.0), ("T9", 5.0), ("Kh", 3.0)];
        assert!(one_with(&p, "T9", &ps, 0.0, false, Skill::Pagat).is_some());
        assert!(one_with(&p, "T9", &ps, 30.0 * OPEN, false, Skill::Pagat).is_none());
        let r = know(&["T10", "T9", "T3", "Kh"], NO, HID, &[], None);
        let rs = [("T10", 12.0), ("T9", 11.0), ("T3", 2.0), ("Kh", 0.0)];
        assert!(one_with(&r, "T3", &rs, 0.0, false, Skill::Run).is_some());
        assert!(one_with(&r, "T3", &rs, 30.0 * OPEN, false, Skill::Run).is_none());
        let mut v = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        v.opp_void[0] = true;
        let vs = [("Qd", 10.0), ("T3", 4.0), ("Kh", 0.0)];
        assert!(one_with(&v, "Kh", &vs, 0.0, false, Skill::Void).is_some());
        assert!(one_with(&v, "Kh", &vs, 30.0 * OPEN, false, Skill::Void).is_none());
        let b = know(&["Kh", "Qh", "9c"], NO, HID, &[], None);
        let bs = [("9c", 10.0), ("Kh", 0.0), ("Qh", 1.0)];
        assert!(one_with(&b, "Kh", &bs, 0.0, false, Skill::Behind).is_some());
        assert!(one_with(&b, "Kh", &bs, 30.0 * OPEN, false, Skill::Behind).is_none());
    }

    #[test]
    fn a_spread_card_is_met_unless_it_is_clearly_worse_than_the_best() {
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], Some("3h"));
        let s = [("Kh", 20.0), ("Qh", 5.0), ("9c", 0.0)];
        // Qh is 15 below the best: a miss when the worlds agree, met when not
        let steady = one_with(&k, "Qh", &s, 0.0, false, Skill::Cheap).unwrap();
        assert!(!steady.met);
        assert!(one_with(&k, "Qh", &s, 30.0 * OPEN, false, Skill::Cheap).is_none());
    }

    #[test]
    fn a_position_not_searched_to_its_end_has_the_open_margin_and_no_endgame() {
        // Nothing face down, but the search stopped short: a 3 thirds gap is
        // noise, and the endgame is not the known position.
        let k = know(&["Kh", "4h", "T3"], NO, NO, &[], None);
        let s = [("Kh", 8.0), ("T3", 6.0), ("4h", 5.0)];
        assert!(one_with(&k, "4h", &s, 0.0, true, Skill::Endgame).is_some());
        assert!(one_with(&k, "4h", &s, 0.0, false, Skill::Endgame).is_none());
        let f = know(&["Kh", "4h"], NO, NO, &[], Some("3h"));
        let w = [("Kh", 8.0), ("4h", 5.0)];
        assert!(one_with(&f, "4h", &w, 0.0, true, Skill::Worth).is_some());
        assert!(one_with(&f, "4h", &w, 0.0, false, Skill::Worth).is_none());
    }

    #[test]
    fn a_search_is_complete_when_it_reached_the_last_trick_and_nothing_is_face_down() {
        let mut k = know(&["Kh", "4h"], NO, NO, &[], None);
        k.trick_number = 20;
        assert!(search_complete(&k, 8.0));
        assert!(!search_complete(&k, 7.9));
        k.trick_number = 27;
        assert!(search_complete(&k, 1.0));
        let open = know(&["Kh", "4h"], NO, HID, &[], None);
        assert!(!search_complete(&open, 28.0), "a packet card is face down");
    }

    #[test]
    fn the_test_values_fit_the_margin() {
        // near ties are written as gaps of 4 or less, clear chances as 10 or more
        assert!(4.0 * OPEN < MARGIN && MARGIN < 10.0 * OPEN, "{MARGIN}");
    }

    // ---- the probe (M1) ----------------------------------------------------

    #[test]
    fn the_probe_names_each_skills_pair_clear_or_not() {
        let probed = |k: &Knowledge, played: &str, scores: &[(&str, f64)]| {
            let scores: Vec<(Card, f64)> =
                sc(scores).into_iter().map(|(c, v)| (c, v * OPEN)).collect();
            let per_world = rows(&scores, 0.0);
            let advice = Advice {
                scores: &scores,
                per_world: &per_world,
                exact: false,
                known: k.hidden_total() == 0,
            };
            let mut seen = Vec::new();
            let verdicts =
                classify_play_probed(k, c(played), &advice, &mut |s, a, b| seen.push((s, a, b)));
            // the same verdicts as classify_play
            assert_eq!(verdicts, classify_play(k, c(played), &advice));
            (seen, verdicts)
        };
        // off: the best cover against the best hand card
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        let (seen, v) = probed(&k, "Kh", &[("Qh", 20.0), ("Kh", 10.0), ("4h", 5.0)]);
        assert!(seen.contains(&(Skill::Off, c("Qh"), c("Kh"))), "{seen:?}");
        assert!(v.iter().any(|x| x.skill == Skill::Off));
        // inside the margin the pair is still named, and there is no verdict
        let (seen, v) = probed(&k, "Kh", &[("Qh", 12.0), ("Kh", 10.0), ("4h", 5.0)]);
        assert!(seen.contains(&(Skill::Off, c("Qh"), c("Kh"))), "{seen:?}");
        assert!(v.iter().all(|x| x.skill != Skill::Off));
        // worth: the best card that beats the lead against the best that does not
        let f = know(&["Kh", "Qh", "4h"], NO, HID, &[], Some("3h"));
        let (seen, _) = probed(&f, "4h", &[("Kh", 20.0), ("Qh", 12.0), ("4h", 0.0)]);
        assert!(seen.contains(&(Skill::Worth, c("Kh"), c("4h"))), "{seen:?}");
        // cheap: the best and the worst of the kind played
        let (seen, _) = probed(&f, "Qh", &[("Kh", 20.0), ("Qh", 12.0), ("4h", 0.0)]);
        assert!(seen.contains(&(Skill::Cheap, c("Kh"), c("Qh"))), "{seen:?}");
        // void and behind: the lead that is not into the void / the split suit,
        // then the one that is
        let mut v = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        v.opp_void[0] = true;
        let (seen, _) = probed(&v, "Kh", &[("Qd", 10.0), ("T3", 4.0), ("Kh", 0.0)]);
        assert!(seen.contains(&(Skill::Void, c("Qd"), c("Kh"))), "{seen:?}");
        // a skill that does not apply is not probed
        let (seen, _) = probed(&k, "Kh", &[("Qh", 12.0), ("Kh", 10.0), ("4h", 5.0)]);
        assert!(seen.iter().all(|(s, _, _)| *s != Skill::Pagat));
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
        assert_eq!(missed.cost, 15.0 * OPEN);
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
        assert_eq!(missed.cost, 15.0 * OPEN);
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
        assert_eq!(m.cost, 8.0 * OPEN);
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
        assert_eq!(m.cost, 10.0 * OPEN);
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
        assert_eq!(m.cost, 10.0 * OPEN);
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
        assert_eq!(m.cost, 10.0 * OPEN);
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
        // he led a diamond and I have none: a tarock must go
        let k = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("3d"));
        let cash = [("T1", 20.0), ("T9", 5.0), ("Kh", 3.0)];
        assert!(one(&k, "T1", &cash, Skill::Pagat).unwrap().met);
        let m = one(&k, "T9", &cash, Skill::Pagat).unwrap();
        assert!(!m.met);
        assert_eq!(m.direction, Some(Direction::Cash));
        assert_eq!(m.better, Some(c("T1")));
        assert_eq!(m.cost, 15.0 * OPEN);
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
    fn pagat_is_no_chance_for_a_leader() {
        // keeping the Pagat back from a lead is trivial (see classify_play)
        let k = know(&["T1", "T9", "Kh"], NO, HID, &[], None);
        let s = [("T1", 20.0), ("T9", 5.0), ("Kh", 3.0)];
        assert!(one(&k, "T9", &s, Skill::Pagat).is_none());
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
        assert_eq!(m.cost, 10.0 * OPEN);
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
        assert_eq!(m.cost, 9.0 * OPEN);
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
        let m = classify_bid(&k, false, 3.0, 0.0).unwrap();
        assert!(!m.met);
        assert_eq!(m.cost, 3.0 * THIRDS_PER_POINT);
        assert_eq!(m.direction, Some(Direction::Passed));
        assert!(classify_bid(&k, true, 3.0, 0.0).unwrap().met);
        let m = classify_bid(&k, true, -2.0, 0.0).unwrap();
        assert!(!m.met);
        assert_eq!(m.direction, Some(Direction::Took));
        assert_eq!(m.cost, 2.0 * THIRDS_PER_POINT);
        assert!(classify_bid(&k, false, -2.0, 0.0).unwrap().met);
        assert!(classify_bid(&k, true, 0.2, 0.0).is_none());
        assert!(classify_bid(&k, false, -0.2, 0.0).is_none());
    }

    #[test]
    fn bid_a_gap_inside_twice_its_standard_error_is_no_chance() {
        let k = know(&["Kh"], NO, HID, &[], None);
        assert!(classify_bid(&k, false, 0.8, 0.6).is_none());
        assert!(classify_bid(&k, false, 0.8, 0.1).is_some());
        assert!(classify_bid(&k, true, -0.8, 0.6).is_none());
    }

    #[test]
    fn bid_costs_are_in_thirds_of_the_count_whatever_the_doubling() {
        // the estimate is per unit of doubling; a doubled deal costs the
        // same thirds (the skill is the play, not the stakes)
        let mut k = know(&["Kh"], NO, HID, &[], None);
        let one = classify_bid(&k, false, 2.0, 0.0).unwrap();
        k.multiplier = 2;
        assert_eq!(classify_bid(&k, false, 2.0, 0.0).unwrap(), one);
    }

    // ---- announce -------------------------------------------------------

    #[test]
    fn announce_playing_a_card_of_a_held_honour_without_announcing_is_a_miss() {
        let mut k = know(&["SK", "T21", "T1"], NO, HID, &[], None);
        k.announceable = vec![Honour::Trull];
        let silent = classify_announce(&k, &Action::play(c("T21"))).unwrap();
        assert!(!silent.met);
        assert_eq!(silent.cost, THIRDS_PER_POINT);
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
    fn announce_costs_a_point_whatever_the_doubling() {
        let mut k = know(&["SK", "T21", "T1"], NO, HID, &[], None);
        k.announceable = vec![Honour::Trull];
        k.multiplier = 2;
        assert_eq!(
            classify_announce(&k, &Action::play(c("T1"))).unwrap().cost,
            THIRDS_PER_POINT
        );
    }
}
