//! The computations of the tutor's measurements (`docs/MEASUREMENTS.md`),
//! apart from the binary that runs them, so that each number a document relies
//! on is reproduced by a tested function.

use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung4::{Budget, MoveTable};
use crate::tutor::{
    advice_of, bid_estimate, classify_play_probed, play_table, Advice, Skill, SKILLS,
};
use strohmandeln_core::agents::Player;
use strohmandeln_core::cards::Card;
use strohmandeln_core::dealrec::DealRecord;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Action, Settings};
use strohmandeln_core::view::{bare_view, view as sitting_view, View};

// ---- small statistics ------------------------------------------------------

pub fn mean(x: &[f64]) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }
    x.iter().sum::<f64>() / x.len() as f64
}

/// The sample standard deviation; 0 for fewer than two values.
pub fn sd(x: &[f64]) -> f64 {
    let n = x.len();
    if n < 2 {
        return 0.0;
    }
    let m = mean(x);
    (x.iter().map(|v| (v - m).powi(2)).sum::<f64>() / (n - 1) as f64).sqrt()
}

/// The `p` quantile (0 to 1) by linear interpolation between order
/// statistics; NaN for no values.
pub fn quantile(x: &[f64], p: f64) -> f64 {
    if x.is_empty() {
        return f64::NAN;
    }
    let mut v = x.to_vec();
    v.sort_by(|a, b| a.total_cmp(b));
    let at = p.clamp(0.0, 1.0) * (v.len() - 1) as f64;
    let (lo, hi) = (at.floor() as usize, at.ceil() as usize);
    v[lo] + (v[hi] - v[lo]) * (at - lo as f64)
}

pub fn median(x: &[f64]) -> f64 {
    quantile(x, 0.5)
}

// ---- sittings and positions -------------------------------------------------

/// Plays a sitting of `touren` from `seed` between two players and gives the
/// record of every deal scored, in order. A prompt to extend is declined.
pub fn record_sitting(
    seed: u64,
    settings: Settings,
    touren: u32,
    players: &mut [&mut dyn Player; 2],
) -> Vec<DealRecord> {
    let mut s = Sitting::new(seed, settings, touren).expect("a sitting");
    let mut out = Vec::new();
    let mut counted = 0;
    while let Some(seat) = s.to_move() {
        let v = sitting_view(&s, seat);
        let a = if matches!(v.legal_actions.first(), Some(Action::Extend { .. })) {
            Action::Extend { accept: false }
        } else {
            players[seat.index()].choose(&v).expect("a legal action")
        };
        s.apply(seat, a).expect("a legal action");
        if s.deals_counted() > counted {
            counted = s.deals_counted();
            out.push(s.last_deal().expect("a scored deal").clone());
        }
    }
    out
}

/// The deals of one sitting of rung 4 against rung 4 (both seats, `config`'s
/// budget), seeded from `seed`.
pub fn rung4_deals(seed: u64, config: Config) -> Vec<DealRecord> {
    let mut a = Opponent::new(Level::from_halves(8), seed ^ 0xA).with_config(config);
    let mut b = Opponent::new(Level::from_halves(8), seed ^ 0xB).with_config(config);
    record_sitting(seed, Settings::default(), 3, &mut [&mut a, &mut b])
}

/// A decision with a choice, as the tutor sees it: the mover's view and what
/// was done.
#[derive(Clone, Debug)]
pub struct Position {
    pub view: View,
    pub action: Action,
}

/// Every decision of either seat in `record` in which the mover had a choice:
/// a play with at least two legal cards (`plays`) or a bid (`bids`).
pub fn decisions(record: &DealRecord) -> (Vec<Position>, Vec<Position>) {
    let (mut plays, mut bids) = (Vec::new(), Vec::new());
    record
        .replay(|_, deal, seat, action| {
            let view = bare_view(deal, seat);
            let Some(k) = Knowledge::from_view(&view) else {
                return;
            };
            let pos = || Position {
                view: view.clone(),
                action: action.clone(),
            };
            match action {
                Action::Play { .. } if k.playing && k.legal.len() >= 2 => plays.push(pos()),
                Action::Take | Action::Pass if !k.playing => bids.push(pos()),
                _ => {}
            }
        })
        .expect("a record of a played deal replays");
    (plays, bids)
}

// ---- M1: advisor noise -------------------------------------------------------

/// What one skill's question said of one position under each of the streams.
#[derive(Clone, Debug, PartialEq)]
pub struct SkillRead {
    pub skill: Skill,
    /// Every stream's search solved the position.
    pub exact: bool,
    /// Per stream: the gap of the pair that stream chose (the classifier's own).
    pub gap: Vec<f64>,
    /// Per stream: the standard error the classifier would use for that pair.
    pub se: Vec<f64>,
    /// Per stream: the gap of the pair this stream chose, as the mean of the
    /// other streams read it.
    pub held_out: Vec<f64>,
    /// The gap of the pair that is best by the mean over all streams, in each
    /// stream, and the standard error each stream reports for it.
    pub fixed_gap: Vec<f64>,
    pub fixed_se: Vec<f64>,
}

type Pair = (Skill, Card, Card);

fn pairs_of(k: &Knowledge, card: Card, advice: &Advice) -> Vec<Pair> {
    let mut seen: Vec<Pair> = Vec::new();
    classify_play_probed(k, card, advice, &mut |s, a, b| seen.push((s, a, b)));
    seen
}

fn gap_of(advice: &Advice, a: Card, b: Card) -> f64 {
    advice.value(a) - advice.value(b)
}

/// The tables of the position under each stream, as advice; `None` for a
/// stream that finished no trick (the evidence would skip the decision).
pub fn play_tables(
    k: &Knowledge,
    view: &View,
    budget: Budget,
    streams: u64,
) -> Option<Vec<MoveTable>> {
    (0..streams)
        .map(|salt| play_table(k, view, budget, salt))
        .collect()
}

/// The reads of one play position, one per skill that applies under every
/// stream.
pub fn read_play(k: &Knowledge, card: Card, tables: &[MoveTable]) -> Vec<SkillRead> {
    let advices: Vec<Advice> = tables.iter().map(|t| advice_of(k, t)).collect();
    let probes: Vec<Vec<Pair>> = advices.iter().map(|a| pairs_of(k, card, a)).collect();
    // the pooled advice: the mean value of each card over the streams
    let first = &tables[0].scores;
    let mut pooled: Vec<(Card, f64)> = first
        .iter()
        .map(|&(c, _)| {
            let vals: Vec<f64> = advices.iter().map(|a| a.value(c)).collect();
            (c, mean(&vals))
        })
        .collect();
    pooled.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
    let pooled_advice = Advice {
        scores: &pooled,
        per_world: &[],
        exact: advices[0].exact,
        known: advices[0].known,
    };
    let pooled_pairs = pairs_of(k, card, &pooled_advice);
    let mut out = Vec::new();
    for skill in Skill::ALL {
        let find = |p: &[Pair]| p.iter().find(|x| x.0 == skill).copied();
        let own: Option<Vec<Pair>> = probes.iter().map(|p| find(p)).collect();
        let (Some(own), Some(fixed)) = (own, find(&pooled_pairs)) else {
            continue;
        };
        let n = advices.len();
        let gap: Vec<f64> = (0..n)
            .map(|s| gap_of(&advices[s], own[s].1, own[s].2))
            .collect();
        let se: Vec<f64> = (0..n)
            .map(|s| advices[s].std_error(own[s].1, own[s].2))
            .collect();
        let held_out: Vec<f64> = (0..n)
            .map(|s| {
                let others: Vec<f64> = (0..n)
                    .filter(|&t| t != s)
                    .map(|t| gap_of(&advices[t], own[s].1, own[s].2))
                    .collect();
                mean(&others)
            })
            .collect();
        let fixed_gap = advices
            .iter()
            .map(|a| gap_of(a, fixed.1, fixed.2))
            .collect();
        let fixed_se = advices
            .iter()
            .map(|a| a.std_error(fixed.1, fixed.2))
            .collect();
        out.push(SkillRead {
            skill,
            exact: advices.iter().all(|a| a.exact),
            gap,
            se,
            held_out,
            fixed_gap,
            fixed_se,
        });
    }
    out
}

/// The reads of one bid: the mean (Points per unit of doubling) and its
/// standard error under each stream.
pub fn read_bid(
    k: &Knowledge,
    view: &View,
    budget: Budget,
    streams: u64,
) -> Option<Vec<(f64, f64)>> {
    (0..streams)
        .map(|salt| bid_estimate(k, view, budget, salt))
        .collect()
}

/// What the reads of one skill say, over its positions.
#[derive(Clone, Debug, PartialEq)]
pub struct Noise {
    pub skill: Skill,
    pub positions: usize,
    /// The across-stream standard deviation of the gap, per position
    /// (positions not exact).
    pub spread: Vec<f64>,
    /// Exact positions, and the spread in each.
    pub exact_spread: Vec<f64>,
    pub calibration_median: f64,
    pub calibration_rms: f64,
    /// Positions whose reported error is infinite in some stream (one world).
    pub unbounded: usize,
    /// Mean of own-argmax gap less its held-out mean (thirds), in the
    /// direction of the gap's sign.
    pub selection: f64,
}

/// The sign-replication and repeat counts of the clear cases at `k` standard
/// errors and `margin` thirds: (clear cases, same sign held out, clear again
/// under another stream with the same sign, pairs of streams compared).
pub fn replication(reads: &[&SkillRead], margin: f64, k: f64) -> (usize, usize, usize, usize) {
    let (mut clear, mut same, mut again, mut pairs) = (0, 0, 0, 0);
    for r in reads.iter().filter(|r| !r.exact) {
        let n = r.gap.len();
        let is_clear = |s: usize| r.gap[s].abs() > margin && r.gap[s].abs() > k * r.se[s];
        for s in 0..n {
            if !is_clear(s) {
                continue;
            }
            clear += 1;
            if r.held_out[s] * r.gap[s] > 0.0 {
                same += 1;
            }
            for t in (0..n).filter(|&t| t != s) {
                pairs += 1;
                if is_clear(t) && r.gap[t] * r.gap[s] > 0.0 {
                    again += 1;
                }
            }
        }
    }
    (clear, same, again, pairs)
}

pub fn noise_of(skill: Skill, reads: &[&SkillRead]) -> Noise {
    let mut spread = Vec::new();
    let mut exact_spread = Vec::new();
    let (mut sd_fixed, mut se_fixed) = (Vec::new(), Vec::new());
    let (mut sq_sd, mut sq_se) = (0.0, 0.0);
    let mut unbounded = 0;
    let mut selection = Vec::new();
    for r in reads {
        if r.exact {
            exact_spread.push(sd(&r.gap));
            continue;
        }
        spread.push(sd(&r.gap));
        let s = sd(&r.fixed_gap);
        let e = mean(&r.fixed_se);
        sd_fixed.push(s);
        se_fixed.push(e);
        // a stream that finished one world of many reports no error (infinite,
        // `Advice::std_error`): it stays in the medians, out of the root mean
        // square
        if e.is_finite() {
            sq_sd += s * s;
            sq_se += e * e;
        } else {
            unbounded += 1;
        }
        for (g, h) in r.gap.iter().zip(&r.held_out) {
            selection.push(g.signum() * (g - h));
        }
    }
    Noise {
        skill,
        positions: reads.len(),
        spread,
        exact_spread,
        calibration_median: median(&sd_fixed) / median(&se_fixed),
        calibration_rms: (sq_sd / sq_se).sqrt(),
        unbounded,
        selection: mean(&selection),
    }
}

/// `MARGIN` by the rule of M1: twice the median across-stream spread pooled
/// over the play skills (positions not exact), rounded to the nearest third,
/// at least 2.
pub fn margin_rule(noises: &[Noise]) -> f64 {
    let all: Vec<f64> = noises
        .iter()
        .flat_map(|n| n.spread.iter().copied())
        .collect();
    (2.0 * median(&all)).round().max(2.0)
}

/// `EXACT_MARGIN` by the rule of M1: 1 third if no exact position has a
/// spread, else twice the 95th percentile of the spread, rounded up.
pub fn exact_margin_rule(noises: &[Noise]) -> f64 {
    let all: Vec<f64> = noises
        .iter()
        .flat_map(|n| n.exact_spread.iter().copied())
        .collect();
    let top = quantile(&all, 0.95);
    if all.is_empty() || top <= 0.0 {
        1.0
    } else {
        (2.0 * top).ceil()
    }
}

/// `BID_MARGIN` by the rule of M1: twice the median across-stream standard
/// deviation of the bid mean, rounded to 0.05 Point.
pub fn bid_margin_rule(reads: &[Vec<(f64, f64)>]) -> f64 {
    let sds: Vec<f64> = reads
        .iter()
        .map(|r| sd(&r.iter().map(|x| x.0).collect::<Vec<_>>()))
        .collect();
    (2.0 * median(&sds) / 0.05).round() * 0.05
}

/// Splits the reads of all positions by skill.
pub fn by_skill(reads: &[Vec<SkillRead>]) -> Vec<Vec<&SkillRead>> {
    let mut out: Vec<Vec<&SkillRead>> = vec![Vec::new(); SKILLS];
    for position in reads {
        for r in position {
            out[r.skill.index()].push(r);
        }
    }
    out
}

// ---- M2: thirds per Point ----------------------------------------------------

/// A least-squares line with a cluster-robust standard error of its slope.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ols {
    pub n: usize,
    pub slope: f64,
    pub intercept: f64,
    pub se_slope: f64,
    pub correlation: f64,
}

/// `y` on `x` with an intercept; the slope's standard error is the
/// cluster-robust one (the sum over clusters of the squared sum of
/// `(x - mean x) * residual`, over the squared `Sxx`, times G / (G - 1)).
pub fn ols_clustered(x: &[f64], y: &[f64], cluster: &[usize]) -> Ols {
    let n = x.len();
    assert!(n == y.len() && n == cluster.len() && n >= 3, "paired data");
    let (mx, my) = (mean(x), mean(y));
    let sxx: f64 = x.iter().map(|v| (v - mx).powi(2)).sum();
    let syy: f64 = y.iter().map(|v| (v - my).powi(2)).sum();
    let sxy: f64 = x.iter().zip(y).map(|(a, b)| (a - mx) * (b - my)).sum();
    let slope = sxy / sxx;
    let intercept = my - slope * mx;
    let mut sums: std::collections::BTreeMap<usize, f64> = Default::default();
    for i in 0..n {
        let e = y[i] - intercept - slope * x[i];
        *sums.entry(cluster[i]).or_default() += (x[i] - mx) * e;
    }
    let g = sums.len() as f64;
    let var = sums.values().map(|v| v * v).sum::<f64>() / (sxx * sxx) * g / (g - 1.0).max(1.0);
    Ols {
        n,
        slope,
        intercept,
        se_slope: var.sqrt(),
        correlation: sxy / (sxx * syy).sqrt(),
    }
}

/// A played deal's observation for seat A: the count margin in thirds (A's
/// less B's, the ultimo excluded) and A's net Points per unit of doubling.
/// `None` for a deal thrown in.
pub fn deal_margin_points(record: &DealRecord) -> Option<(f64, f64)> {
    use strohmandeln_core::deal::DealEnd;
    use strohmandeln_core::types::Seat;
    let deal = record.replay_to_end(|_, _, _, _| {}).ok()?;
    let DealEnd::Played(score) = deal.end()? else {
        return None;
    };
    let x = f64::from(score.thirds[Seat::A.index()]) - f64::from(score.thirds[Seat::B.index()]);
    let net: f64 = score
        .credits
        .iter()
        .map(|c| {
            let p = f64::from(c.points);
            if c.seat == Seat::A {
                p
            } else {
                -p
            }
        })
        .sum();
    Some((x, net / f64::from(score.multiplier)))
}

// ---- M8: the split suit --------------------------------------------------------

/// The mean of `values` and its standard error with the values grouped in
/// clusters (the cluster sums of deviations, G / (G - 1)).
pub fn clustered_mean(values: &[f64], cluster: &[usize]) -> (f64, f64) {
    assert_eq!(values.len(), cluster.len());
    let n = values.len() as f64;
    let m = mean(values);
    let mut sums: std::collections::BTreeMap<usize, f64> = Default::default();
    for (v, c) in values.iter().zip(cluster) {
        *sums.entry(*c).or_default() += v - m;
    }
    let g = sums.len() as f64;
    let var = sums.values().map(|v| v * v).sum::<f64>() * g / (g - 1.0).max(1.0) / (n * n);
    (m, var.sqrt())
}

/// One suit of one lead decision: my share of its unplayed points, their
/// total, and how the best lead in the suit compares with the best lead
/// outside it (thirds; negative: another lead is better).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeadRow {
    pub share: f64,
    pub total: u32,
    pub gap: f64,
    /// The lead actually made was in this suit.
    pub opened: bool,
    pub cluster: usize,
}

/// The rows of a lead decision, one per suit that has a legal lead and
/// leaves another kind of lead to compare with.
pub fn lead_rows(k: &Knowledge, card: Card, advice: &Advice, cluster: usize) -> Vec<LeadRow> {
    use crate::tutor::suit_share;
    use strohmandeln_core::cards::Suit;
    if k.lead.is_some() {
        return Vec::new();
    }
    let best = |class: &dyn Fn(Card) -> bool| {
        advice
            .scores
            .iter()
            .filter(|(c, _)| class(*c))
            .map(|&(_, v)| v)
            .fold(f64::NEG_INFINITY, f64::max)
    };
    let mut out = Vec::new();
    for suit in Suit::ALL {
        let (inside, outside) = (
            best(&|c| c.suit() == Some(suit)),
            best(&|c| c.suit() != Some(suit)),
        );
        if inside == f64::NEG_INFINITY || outside == f64::NEG_INFINITY {
            continue;
        }
        let (mine, total) = suit_share(k, suit);
        if total == 0 {
            continue;
        }
        out.push(LeadRow {
            share: f64::from(mine) / f64::from(total),
            total,
            gap: inside - outside,
            opened: card.suit() == Some(suit),
            cluster,
        });
    }
    out
}

/// A bin of rows: how many, the mean gap with its cluster standard error, the
/// share in which another lead is clearly better (the gap below minus
/// `margin`), and the share that opened the suit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bin {
    pub n: usize,
    pub mean: f64,
    pub se: f64,
    pub other_clearly_better: f64,
    pub opened: f64,
}

pub fn bin_of(rows: &[&LeadRow], margin: f64) -> Bin {
    let gaps: Vec<f64> = rows.iter().map(|r| r.gap).collect();
    let cl: Vec<usize> = rows.iter().map(|r| r.cluster).collect();
    let (m, se) = if rows.len() >= 2 {
        clustered_mean(&gaps, &cl)
    } else {
        (mean(&gaps), f64::INFINITY)
    };
    let n = rows.len() as f64;
    Bin {
        n: rows.len(),
        mean: m,
        se,
        other_clearly_better: rows.iter().filter(|r| r.gap < -margin).count() as f64 / n,
        opened: rows.iter().filter(|r| r.opened).count() as f64 / n,
    }
}

impl Bin {
    /// Another lead is surely better on average (the upper end of the 95%
    /// interval of the mean gap is below 0) and clearly so often enough: in
    /// at least a quarter of the rows the gap is below minus the margin.
    pub fn prefers_other(&self) -> bool {
        self.n >= 2 && self.mean + 1.96 * self.se < 0.0 && self.other_clearly_better >= 0.25
    }
}

/// The M8 band: the contiguous run of bins (of equal width over the share,
/// from 0) that prefer another lead, around the bin of the lowest mean gap,
/// without the lowest bins in which rung 4 itself opens the suit in less
/// than `min_opened` of the rows (a suit it has no mind to open is his, not
/// contested); its edges as (low, high) shares. `None` if the lowest bin does
/// not prefer another lead.
pub fn split_band(bins: &[Bin], min_opened: f64) -> Option<(f64, f64)> {
    let width = 1.0 / bins.len() as f64;
    let lowest = (0..bins.len())
        .filter(|&i| bins[i].n >= 2)
        .min_by(|&a, &b| bins[a].mean.total_cmp(&bins[b].mean))?;
    if !bins[lowest].prefers_other() {
        return None;
    }
    let (mut lo, mut hi) = (lowest, lowest);
    while lo > 0 && bins[lo - 1].prefers_other() {
        lo -= 1;
    }
    while hi + 1 < bins.len() && bins[hi + 1].prefers_other() {
        hi += 1;
    }
    while lo < hi && bins[lo].opened < min_opened {
        lo += 1;
    }
    Some((lo as f64 * width, (hi + 1) as f64 * width))
}

/// The classes of the suit's total points M8 compares: [0,3), [3,6), [6,9),
/// [9,12), [12, ...).
pub const TOTAL_CLASSES: [(u32, u32); 5] = [(0, 3), (3, 6), (6, 9), (9, 12), (12, u32::MAX)];

/// `SPLIT_MIN_POINTS` by the rule of M8: the start of the lowest class from
/// which that class and every higher class with at least `min_rows` rows
/// prefer another lead; `None` if no class does.
pub fn min_points_rule(classes: &[Bin], min_rows: usize) -> Option<u32> {
    for start in 0..classes.len() {
        let rest = &classes[start..];
        let seen: Vec<&Bin> = rest.iter().filter(|b| b.n >= min_rows).collect();
        if !seen.is_empty()
            && classes[start].n >= min_rows
            && seen.iter().all(|b| b.prefers_other())
        {
            return Some(TOTAL_CLASSES[start].0);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mean_sd_and_quantiles() {
        let x = [2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        assert_eq!(mean(&x), 5.0);
        assert!((sd(&x) - 2.138_089_935).abs() < 1e-8);
        assert_eq!(median(&x), 4.5);
        assert_eq!(quantile(&x, 0.0), 2.0);
        assert_eq!(quantile(&x, 1.0), 9.0);
        assert!((quantile(&x, 0.9) - 7.6).abs() < 1e-12);
        assert_eq!(sd(&[3.0]), 0.0);
        assert!(median(&[]).is_nan());
    }

    fn read(gap: &[f64], se: &[f64], held: &[f64], exact: bool) -> SkillRead {
        SkillRead {
            skill: Skill::Off,
            exact,
            gap: gap.to_vec(),
            se: se.to_vec(),
            held_out: held.to_vec(),
            fixed_gap: gap.to_vec(),
            fixed_se: se.to_vec(),
        }
    }

    #[test]
    fn replication_counts_clear_cases_their_sign_and_their_repeats() {
        // three streams: the first two are clear and positive, the third is
        // inside the margin; the first's held-out mean is positive, the
        // second's negative.
        let r = read(
            &[10.0, 9.0, 3.0],
            &[1.0, 1.0, 1.0],
            &[8.0, -1.0, 8.0],
            false,
        );
        let (clear, same, again, pairs) = replication(&[&r], 6.0, 2.0);
        assert_eq!(clear, 2);
        assert_eq!(same, 1);
        // each clear case is compared with the 2 other streams
        assert_eq!(pairs, 4);
        assert_eq!(again, 2, "stream 0 against 1, stream 1 against 0");
        // a larger multiple of the error removes the cases
        let wide = read(&[10.0, 9.0], &[4.0, 4.0], &[8.0, 8.0], false);
        assert_eq!(replication(&[&wide], 6.0, 2.0).0, 2);
        assert_eq!(replication(&[&wide], 6.0, 3.0).0, 0);
        // an exact position is not counted
        let ex = read(&[10.0, 10.0], &[0.0, 0.0], &[10.0, 10.0], true);
        assert_eq!(replication(&[&ex], 6.0, 2.0).0, 0);
    }

    #[test]
    fn replication_respects_the_sign_of_a_negative_gap() {
        let r = read(&[-10.0, -10.0], &[1.0, 1.0], &[-9.0, 4.0], false);
        let (clear, same, again, _) = replication(&[&r], 6.0, 2.0);
        assert_eq!((clear, same, again), (2, 1, 2));
    }

    #[test]
    fn the_noise_of_a_skill_separates_exact_positions_and_measures_calibration() {
        let steady = read(&[8.0, 8.0, 8.0], &[1.0, 1.0, 1.0], &[8.0, 8.0, 8.0], false);
        let wild = read(
            &[2.0, 8.0, 14.0],
            &[1.0, 1.0, 1.0],
            &[11.0, 8.0, 5.0],
            false,
        );
        let exact = read(&[5.0, 5.0], &[0.0, 0.0], &[5.0, 5.0], true);
        let n = noise_of(Skill::Off, &[&steady, &wild, &exact]);
        assert_eq!(n.positions, 3);
        assert_eq!(n.spread, vec![0.0, 6.0]);
        assert_eq!(n.exact_spread, vec![0.0]);
        // medians of the fixed-pair spread (0 and 6) and the mean error (1, 1)
        assert_eq!(n.calibration_median, 3.0);
        assert!((n.calibration_rms - 18.0_f64.sqrt()).abs() < 1e-12);
        assert_eq!(n.unbounded, 0);
        // a position whose error is infinite stays out of the root mean square
        let blind = read(&[2.0, 8.0], &[f64::INFINITY, 1.0], &[8.0, 2.0], false);
        let m = noise_of(Skill::Off, &[&steady, &wild, &blind]);
        assert_eq!(m.unbounded, 1);
        assert!((m.calibration_rms - 18.0_f64.sqrt()).abs() < 1e-12);
        // selection: wild's gaps 2 and 8 and 14 less 11, 8 and 5, signs positive
        assert!((n.selection - (0.0 + 0.0 + 0.0 + (-9.0) + 0.0 + 9.0) / 6.0).abs() < 1e-12);
    }

    #[test]
    fn the_rules_for_the_margins() {
        let mk = |spread: &[f64], exact: &[f64]| Noise {
            skill: Skill::Off,
            positions: spread.len(),
            spread: spread.to_vec(),
            exact_spread: exact.to_vec(),
            calibration_median: 1.0,
            calibration_rms: 1.0,
            unbounded: 0,
            selection: 0.0,
        };
        // median 2.6: twice is 5.2, rounded to 5
        assert_eq!(
            margin_rule(&[mk(&[1.0, 2.6, 9.0], &[]), mk(&[2.6, 4.0], &[])]),
            5.0
        );
        // never below 2
        assert_eq!(margin_rule(&[mk(&[0.1, 0.2], &[])]), 2.0);
        assert_eq!(exact_margin_rule(&[mk(&[], &[0.0, 0.0])]), 1.0);
        assert_eq!(exact_margin_rule(&[mk(&[], &[0.0, 0.7, 0.0])]), 2.0);
        assert_eq!(exact_margin_rule(&[mk(&[1.0], &[])]), 1.0);
        // bids: sds 0.5, 1.5 and 1.0 give median 1.0, twice is 2.0
        let reads = vec![
            vec![(0.0, 0.1), (1.0, 0.1)],
            vec![(0.0, 0.1), (3.0, 0.1)],
            vec![(0.0, 0.1), (2.0, 0.1)],
        ];
        let m = bid_margin_rule(&reads);
        let expect =
            (2.0 * median(&[0.707_106_781, 2.121_320_343, 1.414_213_562]) / 0.05).round() * 0.05;
        assert!((m - expect).abs() < 1e-9, "{m} {expect}");
    }

    fn tiny() -> Config {
        Config {
            budget: Budget {
                worlds: 4,
                nodes: 4_000,
                bid_worlds: 8,
            },
            ..Config::default()
        }
    }

    #[test]
    fn a_sitting_gives_its_decisions_and_each_is_read_under_every_stream() {
        let recs = rung4_deals(10_000_000, tiny());
        assert_eq!(recs.len(), 6);
        let (plays, bids) = decisions(&recs[0]);
        assert!(plays.len() >= 5, "{}", plays.len());
        assert!(!bids.is_empty());
        for p in plays.iter().take(8) {
            let k = Knowledge::from_view(&p.view).unwrap();
            assert!(k.playing && k.legal.len() >= 2);
            let Action::Play { card, .. } = &p.action else {
                panic!("a play")
            };
            let tables = play_tables(&k, &p.view, Budget::default(), 3).unwrap();
            assert_eq!(tables.len(), 3);
            for r in read_play(&k, *card, &tables) {
                assert_eq!(r.gap.len(), 3);
                assert_eq!(r.held_out.len(), 3);
                assert_eq!(r.fixed_gap.len(), 3);
                // stream 0 is the evidence's own: its own pair's gap is the
                // classifier's, and the held-out mean is of the other two
                assert!(r.gap.iter().all(|g| g.is_finite()), "{r:?}");
            }
        }
        for b in &bids {
            let k = Knowledge::from_view(&b.view).unwrap();
            let r = read_bid(&k, &b.view, Budget::default(), 3).unwrap();
            assert_eq!(r.len(), 3);
        }
    }

    #[test]
    fn a_line_is_recovered_and_its_cluster_error_is_zero_when_it_is_exact() {
        let x: Vec<f64> = (0..12).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|v| 0.5 * v + 2.0).collect();
        let cl: Vec<usize> = (0..12).map(|i| i / 3).collect();
        let o = ols_clustered(&x, &y, &cl);
        assert!((o.slope - 0.5).abs() < 1e-12 && (o.intercept - 2.0).abs() < 1e-12);
        assert!(o.se_slope < 1e-12 && (o.correlation - 1.0).abs() < 1e-12);
    }

    #[test]
    fn the_cluster_error_has_its_hand_computed_value() {
        // slope 0.5, intercept 0; the residual sums by cluster give 0.1,
        // and by single point 0.26726 (computed by hand)
        let x = [0.0, 1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0];
        let y = [1.0, 1.0, 2.0, 2.0, -1.0, 0.0, 0.0, 1.0];
        let together = ols_clustered(&x, &y, &[0, 0, 0, 0, 1, 1, 1, 1]);
        let apart = ols_clustered(&x, &y, &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert!((together.slope - 0.5).abs() < 1e-12 && together.intercept.abs() < 1e-12);
        assert!((together.se_slope - 0.1).abs() < 1e-12, "{together:?}");
        assert!((apart.se_slope - 0.267_261_241_9).abs() < 1e-9, "{apart:?}");
    }

    #[test]
    fn a_played_deal_gives_its_margin_and_net_points() {
        let recs = rung4_deals(20_000_000, tiny());
        let mut seen = 0;
        for r in &recs {
            if let Some((x, y)) = deal_margin_points(r) {
                seen += 1;
                assert!(x.abs() <= 214.0 && y.abs() <= 30.0, "{x} {y}");
            }
        }
        assert!(seen >= 3);
    }

    fn row(share: f64, total: u32, gap: f64, cluster: usize) -> LeadRow {
        LeadRow {
            share,
            total,
            gap,
            opened: gap > 0.0,
            cluster,
        }
    }

    #[test]
    fn a_clustered_mean_has_its_hand_computed_error() {
        // clusters {1, 3} and {5, 7}: means 2 and 6, overall 4; cluster sums
        // of deviations -0 (1-4 + 3-4 = -4)... computed: -4 and +4, so
        // var = (16 + 16) * 2 / 1 / 16 = 4
        let (m, se) = clustered_mean(&[1.0, 3.0, 5.0, 7.0], &[0, 0, 1, 1]);
        assert_eq!(m, 4.0);
        assert!((se - 2.0).abs() < 1e-12, "{se}");
    }

    #[test]
    fn bins_say_where_another_lead_is_surely_better() {
        let bad: Vec<LeadRow> = (0..40)
            .map(|i| row(0.5, 10, -12.0 - (i % 3) as f64, i))
            .collect();
        let good: Vec<LeadRow> = (0..40)
            .map(|i| row(0.9, 10, 6.0 + (i % 3) as f64, i))
            .collect();
        let b = bin_of(&bad.iter().collect::<Vec<_>>(), 10.0);
        let g = bin_of(&good.iter().collect::<Vec<_>>(), 10.0);
        assert!(b.prefers_other() && !g.prefers_other());
        assert_eq!(b.other_clearly_better, 1.0);
        assert_eq!(g.opened, 1.0);
        // ten bins: 0.4 to 0.7 prefer another lead (bins 4, 5, 6)
        let mut bins = vec![g; 10];
        for i in 4..7 {
            bins[i] = b;
        }
        bins[5].mean = -20.0;
        let (lo, hi) = split_band(&bins, 0.0).unwrap();
        assert!(
            (lo - 0.4).abs() < 1e-12 && (hi - 0.7).abs() < 1e-12,
            "{lo} {hi}"
        );
        // no band when the lowest bin does not prefer another lead
        assert_eq!(split_band(&vec![g; 10], 0.0), None);
        // the lowest bins in which the suit is hardly ever opened are left out
        bins[4].opened = 0.05;
        bins[5].opened = 0.3;
        bins[6].opened = 0.3;
        let (lo, _) = split_band(&bins, 0.1).unwrap();
        assert!((lo - 0.5).abs() < 1e-12, "{lo}");
    }

    #[test]
    fn the_least_total_is_where_the_effect_holds_from_there_up() {
        let bad = |n| Bin {
            n,
            mean: -9.0,
            se: 1.0,
            other_clearly_better: 0.8,
            opened: 0.2,
        };
        let flat = |n| Bin {
            n,
            mean: 1.0,
            se: 1.0,
            other_clearly_better: 0.1,
            opened: 0.5,
        };
        // [0,3) flat, from [3,6) up bad
        let classes = [flat(100), bad(100), bad(100), bad(100), bad(100)];
        assert_eq!(min_points_rule(&classes, 30), Some(3));
        // a class with too few rows is neither for nor against
        let classes = [flat(100), bad(100), flat(5), bad(100), bad(100)];
        assert_eq!(min_points_rule(&classes, 30), Some(3));
        // a flat class above a bad one breaks the run from there down
        let classes = [bad(100), bad(100), flat(100), bad(100), bad(100)];
        assert_eq!(min_points_rule(&classes, 30), Some(9));
        let none = [flat(100); 5];
        assert_eq!(min_points_rule(&none, 30), None);
    }

    #[test]
    fn a_lead_gives_a_row_for_each_suit_with_something_else_to_lead() {
        use crate::testutil::{c, know};
        let k = know(
            &["Kh", "Qh", "9c", "T3"],
            ["", "", ""],
            ["T2", "", ""],
            &[],
            None,
        );
        let scores = vec![
            (c("9c"), 10.0),
            (c("T3"), 4.0),
            (c("Kh"), 0.0),
            (c("Qh"), 1.0),
        ];
        let per_world: Vec<Vec<(Card, i32)>> = Vec::new();
        let advice = Advice {
            scores: &scores,
            per_world: &per_world,
            exact: false,
            known: false,
        };
        let rows = lead_rows(&k, c("Kh"), &advice, 7);
        // hearts and clubs have legal leads; diamonds and spades have none
        assert_eq!(rows.len(), 2);
        let hearts = rows.iter().find(|r| r.opened).unwrap();
        assert_eq!(hearts.gap, 1.0 - 10.0, "best heart 1 against best other 10");
        assert_eq!(hearts.cluster, 7);
        let clubs = rows.iter().find(|r| !r.opened).unwrap();
        assert_eq!(clubs.gap, 10.0 - 4.0);
        // mine: K+Q of hearts; the share is of mine and everything unseen
        assert!(hearts.share > 0.0 && hearts.share < 1.0 && hearts.total >= 9);
        // a follower has no lead to compare
        let f = know(&["Kh", "Qh"], ["", "", ""], ["T2", "", ""], &[], Some("3h"));
        assert!(lead_rows(&f, c("Kh"), &advice, 0).is_empty());
    }
}
