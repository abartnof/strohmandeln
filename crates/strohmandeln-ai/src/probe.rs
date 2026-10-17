//! Stage 0 of `docs/STYLES.md` 4.1: do the styles differ in play, and how much
//! do they bid? The `stage0` binary drives this module.
//!
//! A style plays both seats of a sitting (self-play: every deal gives two
//! seats' decisions). At each decision with a choice the same view is put to
//! Normal at the same skill, with the same seed, so a difference is the
//! style's. **Divergence** is the share of such decisions whose action (the
//! card, or the bid) differs. **Bid rate** is the share of bid decisions that
//! take up. At rung 4 the search is run once and both choose from its scores
//! (the tests hold that equal to `Opponent::decide`), and the share of
//! decisions with a near tie is counted: within Normal's band of 0.34 thirds
//! and within the style's own. **Calibration**: rung 4 bids by the sampled
//! advantage, not by `BidRule`; a style's rung-4 margin is the advantage at
//! which Normal's sample bids the share of hands the style's rule bids
//! (`Calibration`).
//!
//! Per deal and seat the features of the tells gate (`docs/STYLES.md` 4.2)
//! are counted from the decisions, for stage 1.

use crate::cardset::CardSet;
use crate::cardset::Kind;
use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung2::{bid_as, cover_index, my_value_in, opp_visible, play_card, BidRule};
use crate::rung4;
use crate::style::Style;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Action, Seat, Settings};
use strohmandeln_core::view::{view, View};

/// The tells of one seat in one deal, counted from its decisions.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Features {
    /// A tarock led while holding at least 5 tarocks.
    pub tarock_lead_ge5: u32,
    /// A tarock led before trick 5.
    pub tarock_lead_early: u32,
    /// Bid decisions made (0 for the dealer when the forehand took up).
    pub bid_decisions: u32,
    pub bid_taken: u32,
    /// A bid taken with at most 5 tarocks in hand.
    pub bid_taken_le5: u32,
    /// The Pagat played before trick 20 holding 7 or 8 tarocks, and holding 10
    /// or more.
    pub pagat_early_78: u32,
    pub pagat_early_10: u32,
    /// A lying Dame or Cavall led while fewer than two low cards of its suit
    /// show on his side ("blank"), and while two or more show.
    pub figure_blank: u32,
    pub figure_two_low: u32,
    /// A suit led from a split holding (3 or more points, 0.3 to 0.7 of the
    /// suit's 18).
    pub split_open: u32,
    /// The Mond played before trick 14.
    pub mond_early: u32,
    /// Decisions with a choice, and those where Normal would have chosen
    /// otherwise.
    pub decisions: u32,
    pub diverged: u32,
}

impl Features {
    /// The columns of a CSV row, in order.
    pub const COLUMNS: [&'static str; 13] = [
        "tarock_lead_ge5",
        "tarock_lead_early",
        "bid_decisions",
        "bid_taken",
        "bid_taken_le5",
        "pagat_early_78",
        "pagat_early_10",
        "figure_blank",
        "figure_two_low",
        "split_open",
        "mond_early",
        "decisions",
        "diverged",
    ];

    pub fn values(&self) -> [u32; 13] {
        [
            self.tarock_lead_ge5,
            self.tarock_lead_early,
            self.bid_decisions,
            self.bid_taken,
            self.bid_taken_le5,
            self.pagat_early_78,
            self.pagat_early_10,
            self.figure_blank,
            self.figure_two_low,
            self.split_open,
            self.mond_early,
            self.decisions,
            self.diverged,
        ]
    }

    /// Counts what `action` shows of the tells, in the position `k`.
    pub fn observe(&mut self, k: &Knowledge, action: &Action) {
        let tarocks = k.my_tarocks();
        match action {
            Action::Take | Action::Pass => {
                self.bid_decisions += 1;
                if *action == Action::Take {
                    self.bid_taken += 1;
                    let in_hand = k.hand.iter().filter(|c| c.is_tarock()).count();
                    self.bid_taken_le5 += u32::from(in_hand <= 5);
                }
            }
            Action::Play { card, .. } => {
                let card = *card;
                let leading = k.lead.is_none();
                if leading && card.is_tarock() {
                    self.tarock_lead_ge5 += u32::from(tarocks >= 5);
                    self.tarock_lead_early += u32::from(k.trick_number < 5);
                }
                if card == Card::PAGAT && k.trick_number < 20 {
                    self.pagat_early_78 += u32::from((7..=8).contains(&tarocks));
                    self.pagat_early_10 += u32::from(tarocks >= 10);
                }
                if card == Card::MOND && k.trick_number < 14 {
                    self.mond_early += 1;
                }
                if leading {
                    if let Some(suit) = card.suit() {
                        let visible = opp_visible(k, Kind::of(card));
                        if cover_index(k, card).is_some() && matches!(card.strength(), 5 | 6) {
                            let low = visible.iter().filter(|v| v.thirds() == 1).count();
                            if low < 2 {
                                self.figure_blank += 1;
                            } else {
                                self.figure_two_low += 1;
                            }
                        }
                        let mine = my_value_in(k, suit);
                        if mine >= 3 && (0.3..=0.7).contains(&(f64::from(mine) / 18.0)) {
                            self.split_open += 1;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// What the decisions of a run add up to.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Tally {
    /// Decisions with a choice (a bid, or a play with more than one legal
    /// card), and those whose action differs from Normal's.
    pub decisions: u64,
    pub diverged: u64,
    pub bids: u64,
    pub takes: u64,
    /// Rung 4 plays with a choice, and those with a near tie inside Normal's
    /// band and inside the style's band.
    pub searched: u64,
    pub in_normal_band: u64,
    pub in_style_band: u64,
    pub deals: u64,
}

impl Tally {
    fn share(n: u64, d: u64) -> f64 {
        if d == 0 {
            0.0
        } else {
            n as f64 / d as f64
        }
    }
    pub fn divergence(&self) -> f64 {
        Tally::share(self.diverged, self.decisions)
    }
    pub fn bid_rate(&self) -> f64 {
        Tally::share(self.takes, self.bids)
    }
    pub fn normal_band(&self) -> f64 {
        Tally::share(self.in_normal_band, self.searched)
    }
    pub fn style_band(&self) -> f64 {
        Tally::share(self.in_style_band, self.searched)
    }
}

/// One seat's row for stage 1.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Row {
    pub seed: u64,
    /// The deal's number in its sitting, from 0.
    pub deal: u32,
    pub seat: u8,
    pub features: Features,
}

/// A decision of the style and the one Normal makes of the same view, and
/// where a rung-4 play stood against the bands.
pub struct Both {
    pub action: Action,
    pub normal: Action,
    /// For a rung-4 play with a choice: a near tie within Normal's band, and
    /// within the style's.
    pub bands: Option<(bool, bool)>,
}

/// The style's decision in `view` with the seed `seed`, and Normal's (config
/// `normal`) at the same skill and seed.
pub fn decide_both(level: Level, style: &Config, normal: &Config, seed: u64, v: &View) -> Both {
    let s = Opponent::new(level, seed).with_config(*style);
    let n = Opponent::new(level, seed).with_config(*normal);
    let search = level.rung() == 4
        && matches!(v.legal_actions.first(), Some(Action::Play { .. }))
        && v.legal_actions.len() > 1;
    if !search {
        return Both {
            action: s.decide(v),
            normal: n.decide(v),
            bands: None,
        };
    }
    // One search serves both: the worlds do not depend on the weights.
    let k = Knowledge::from_view(v).expect("a deal is in progress");
    let mut rng = s.decision_rng(v);
    let (scores, depth) = rung4::move_scores(&k, &mut rng, s.budget(), &style.weights);
    if depth == 0.0 {
        // The budget finished no trick: rung 3's choice stands in, and it does
        // read the weights, so the two are decided apart.
        return Both {
            action: s.decide(v),
            normal: n.decide(v),
            bands: None,
        };
    }
    let mut rng_n = rng.clone();
    let card = rung4::choose(&k, &mut rng, &scores, &style.weights);
    let card_n = rung4::choose(&k, &mut rng_n, &scores, &normal.weights);
    let bands = (
        rung4::near(&scores, normal.weights.band).len() > 1,
        rung4::near(&scores, style.weights.band).len() > 1,
    );
    Both {
        action: play_card(&k, card),
        normal: play_card(&k, card_n),
        bands: Some(bands),
    }
}

/// Whether `a` and `b` are the same decision: the card, or the bid.
fn same(a: &Action, b: &Action) -> bool {
    match (a, b) {
        (Action::Play { card: x, .. }, Action::Play { card: y, .. }) => x == y,
        _ => a == b,
    }
}

/// The style's `Config` over `base` (the budget and the rest are `base`'s).
pub fn styled(base: &Config, style: Style) -> Config {
    let mut c = *base;
    style.apply(&mut c);
    c
}

/// Plays the sittings of `seeds` (3 Touren each) with `style` in both seats at
/// `level`, against Normal's reference on every view. A tally and one row per
/// deal and seat.
pub fn run_style(
    level: Level,
    style: Style,
    seeds: std::ops::Range<u64>,
    base: &Config,
) -> (Tally, Vec<Row>) {
    let cfg = styled(base, style);
    let mut tally = Tally::default();
    let mut rows = Vec::new();
    for seed in seeds {
        let mut s = Sitting::new(seed, Settings::default(), 3).expect("a sitting");
        let first_row = rows.len();
        let mut deal = None::<u32>;
        while let Some(seat) = s.to_move() {
            if s.deal_action_count() == 0 {
                let d = deal.map_or(0, |d| d + 1);
                deal = Some(d);
                for seat in Seat::BOTH {
                    rows.push(Row {
                        seed,
                        deal: d,
                        seat: seat.index() as u8,
                        features: Features::default(),
                    });
                }
                tally.deals += 1;
            }
            let v = view(&s, seat);
            let who = seed ^ ((seat.index() as u64 + 1) * 0x9e37);
            let both = decide_both(level, &cfg, base, who, &v);
            let row = &mut rows[first_row + 2 * deal.unwrap() as usize + seat.index()];
            if let Some(k) = Knowledge::from_view(&v) {
                row.features.observe(&k, &both.action);
            }
            let bid = matches!(both.action, Action::Take | Action::Pass);
            let choice = bid || v.legal_actions.len() > 1;
            if matches!(both.action, Action::Extend { .. }) {
                // No tell in an extension.
            } else if choice {
                tally.decisions += 1;
                row.features.decisions += 1;
                if !same(&both.action, &both.normal) {
                    tally.diverged += 1;
                    row.features.diverged += 1;
                }
                if bid {
                    tally.bids += 1;
                    tally.takes += u64::from(both.action == Action::Take);
                }
                if let Some((n, st)) = both.bands {
                    tally.searched += 1;
                    tally.in_normal_band += u64::from(n);
                    tally.in_style_band += u64::from(st);
                }
            }
            s.apply(seat, both.action).expect("a legal action");
        }
    }
    (tally, rows)
}

/// The hands the first bidder of each sitting holds, and the sampled
/// advantage of taking up that rung 4 puts on each.
pub struct Calibration {
    pub hands: Vec<CardSet>,
    pub advantages: Vec<f64>,
}

impl Calibration {
    /// Samples `seeds` (the first bidder's hand of each sitting), `worlds`
    /// worlds for each advantage.
    pub fn sample(seeds: std::ops::Range<u64>, worlds: usize) -> Calibration {
        let mut hands = Vec::new();
        let mut advantages = Vec::new();
        for seed in seeds {
            let s = Sitting::new(seed, Settings::default(), 3).expect("a sitting");
            let seat = s.to_move().expect("a bid to make");
            let k = Knowledge::from_view(&view(&s, seat)).expect("a deal");
            let mut rng = Rng::seeded(seed ^ 0xca11);
            advantages.push(rung4::bid_advantage(&k, &mut rng, worlds));
            hands.push(k.hand);
        }
        Calibration { hands, advantages }
    }

    /// The share of the hands `rule` takes up on (as forehand).
    pub fn share(&self, rule: BidRule) -> f64 {
        let n = self
            .hands
            .iter()
            .filter(|&&h| bid_as(h, rule, false).0)
            .count();
        n as f64 / self.hands.len().max(1) as f64
    }

    /// The share of the hands with an advantage above `margin`: Normal's
    /// rung-4 bid rate at that margin.
    pub fn share_at(&self, margin: f64) -> f64 {
        let n = self.advantages.iter().filter(|&&a| a > margin).count();
        n as f64 / self.advantages.len().max(1) as f64
    }

    /// The margin (Points) at which rung 4 bids on `share` of the hands: the
    /// midpoint between the advantages either side of the cut; above the
    /// largest for a share of 0, below the smallest for a share of 1.
    pub fn margin(&self, share: f64) -> f64 {
        let mut a = self.advantages.clone();
        a.sort_by(|x, y| y.partial_cmp(x).expect("a number"));
        let n = a.len();
        let take = ((share * n as f64).round() as usize).min(n);
        match (take.checked_sub(1).map(|i| a[i]), a.get(take)) {
            (Some(hi), Some(lo)) => (hi + lo) / 2.0,
            (None, Some(lo)) => *lo + 1.0,
            (Some(hi), None) => hi - 1.0,
            (None, None) => 0.0,
        }
    }
}

/// The header of the CSV `rows` are written under.
pub fn csv_header() -> String {
    let mut h = String::from("style,skill,seed,deal,seat");
    for c in Features::COLUMNS {
        h.push(',');
        h.push_str(c);
    }
    h
}

/// A CSV line for `row`, played by `style` at `level`.
pub fn csv_line(style: Style, level: Level, row: &Row) -> String {
    let mut l = format!(
        "{},{},{},{},{}",
        style.id(),
        level.as_f64(),
        row.seed,
        row.deal,
        row.seat
    );
    for v in row.features.values() {
        l.push(',');
        l.push_str(&v.to_string());
    }
    l
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod tests;
