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
        sq_sd += s * s;
        sq_se += e * e;
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
}
