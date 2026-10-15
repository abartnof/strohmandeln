//! What the tutor's evidence costs, and how noisy it is, over a spread of
//! positions (`docs/OPPONENT.md`, "Hint evidence"). Native; run with `--release`.
//!
//! ```text
//! hintbench [--sittings N] [--cap-lead N] [--cap-follow N] [--cap-bid N]
//!           [--level HALVES] [--lead-ns 24,100,1000] [--odds-worlds 10,25,50,100]
//!           [--skip-odds] [--skip-noise]
//! ```
//!
//! Plays N seeded sittings with the opponent at `--level` (halves; 6 is the
//! browser's default skill 3) in both seats, takes every position where the
//! mover has more than one legal action, and samples up to the caps (leads,
//! follows, bids) by a seeded shuffle. Reports, in milliseconds (median, p95,
//! max):
//!
//! 1. the lead-rate evidence at each n: sampling `lead_sims`, then
//!    `lead_wins` for the hinted card (what `hint` does) and for every legal
//!    card (what `explain_play` could use);
//! 2. `odds_with` at each worlds count, with the node budget as it is (30 000
//!    shared by the worlds) and scaled to 3 000 per world (`odds_config`, what
//!    the hint uses), with how many worlds finished and how often odds are
//!    returned;
//! 3. the whole `hint` at levels 3 and 4 with the default config;
//! 4. the sampling noise of the lead-rate line: the 95% half-width of the
//!    proportion at each n, and how often two independent samples would
//!    differ in the wording (cheap: wins <= 20% vs rate) or in which side of
//!    one half the card falls, computed exactly from the binomial at the
//!    proportion measured over 2000 worlds.

use std::time::Instant;
use strohmandeln_ai::hint::{hint, odds_config, odds_with};
use strohmandeln_ai::know::Knowledge;
use strohmandeln_ai::opponent::{Config, Level, Opponent};
use strohmandeln_ai::rung3;
use strohmandeln_ai::seed::view_seed;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::{Action, Settings};
use strohmandeln_core::view::{view, View};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
    Lead,
    Follow,
    Bid,
}

struct Pos {
    view: View,
    kind: Kind,
    seed: u64,
}

struct Args {
    sittings: u64,
    cap: [usize; 3],
    level: u8,
    lead_ns: Vec<usize>,
    odds_worlds: Vec<usize>,
    odds: bool,
    noise: bool,
}

fn list(v: &str) -> Vec<usize> {
    v.split(',').map(|x| x.parse().expect("a number")).collect()
}

fn args() -> Args {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    let mut a = Args {
        sittings: 12,
        cap: [300, 300, 100],
        level: 6,
        lead_ns: vec![24, 100, 1000],
        odds_worlds: vec![10, 25, 50, 100],
        odds: true,
        noise: true,
    };
    let mut i = 0;
    while i < argv.len() {
        let v = argv.get(i + 1).cloned().unwrap_or_default();
        match argv[i].as_str() {
            "--sittings" => a.sittings = v.parse().expect("--sittings N"),
            "--cap-lead" => a.cap[0] = v.parse().expect("--cap-lead N"),
            "--cap-follow" => a.cap[1] = v.parse().expect("--cap-follow N"),
            "--cap-bid" => a.cap[2] = v.parse().expect("--cap-bid N"),
            "--level" => a.level = v.parse().expect("--level HALVES"),
            "--lead-ns" => a.lead_ns = list(&v),
            "--odds-worlds" => a.odds_worlds = list(&v),
            "--skip-odds" => {
                a.odds = false;
                i += 1;
                continue;
            }
            "--skip-noise" => {
                a.noise = false;
                i += 1;
                continue;
            }
            other => panic!("unknown {other}"),
        }
        i += 2;
    }
    a
}

/// Every position with a choice, from sittings played by the opponent.
fn positions(a: &Args) -> Vec<Pos> {
    let mut all = Vec::new();
    for seed in 0..a.sittings {
        let mut s = Sitting::new(seed, Settings::default(), 3).expect("a sitting");
        let opp = |seat: usize| {
            Opponent::new(
                Level::from_halves(a.level),
                seed ^ ((seat as u64 + 1) * 0x9e37),
            )
        };
        let players = [opp(0), opp(1)];
        while let Some(seat) = s.to_move() {
            let v = view(&s, seat);
            let action = players[seat.index()].decide(&v);
            let choice = v.legal_actions.len() > 1
                && !matches!(v.legal_actions.first(), Some(Action::Extend { .. }));
            if choice {
                if let Some(k) = Knowledge::from_view(&v) {
                    let kind = if !k.playing {
                        Kind::Bid
                    } else if k.lead.is_none() {
                        Kind::Lead
                    } else {
                        Kind::Follow
                    };
                    let seed = all.len() as u64;
                    all.push(Pos {
                        view: v,
                        kind,
                        seed,
                    });
                }
            }
            s.apply(seat, action).expect("a legal move");
        }
    }
    let mut rng = Rng::seeded(7);
    rng.shuffle(&mut all);
    let mut taken = [0usize; 3];
    all.retain(|p| {
        let i = p.kind as usize;
        taken[i] += 1;
        taken[i] <= a.cap[i]
    });
    all
}

fn ms<T>(f: impl FnOnce() -> T) -> (T, f64) {
    let t = Instant::now();
    let r = f();
    (r, t.elapsed().as_secs_f64() * 1e3)
}

fn row(name: &str, mut xs: Vec<f64>) {
    if xs.is_empty() {
        println!("| {name} | 0 | | | |");
        return;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| xs[((xs.len() - 1) as f64 * p).round() as usize];
    println!(
        "| {name} | {} | {:.2} | {:.2} | {:.2} |",
        xs.len(),
        q(0.5),
        q(0.95),
        xs[xs.len() - 1]
    );
}

fn header(title: &str) {
    println!("\n### {title}\n");
    println!("| what | positions | median ms | p95 ms | max ms |");
    println!("|---|---|---|---|---|");
}

/// ln of n! for n up to `n`.
fn ln_fact(n: usize) -> Vec<f64> {
    let mut f = vec![0.0; n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] + (i as f64).ln();
    }
    f
}

/// P(Bin(n, p) <= k) and P(Bin(n, p) > n / 2).
fn binomial_tails(n: usize, p: f64, k: usize, lf: &[f64]) -> (f64, f64) {
    let pmf = |i: usize| -> f64 {
        if p <= 0.0 {
            return f64::from(i == 0);
        }
        if p >= 1.0 {
            return f64::from(i == n);
        }
        (lf[n] - lf[i] - lf[n - i] + i as f64 * p.ln() + (n - i) as f64 * (1.0 - p).ln()).exp()
    };
    let low: f64 = (0..=k).map(pmf).sum();
    let half: f64 = (0..=n).filter(|i| 2 * i > n).map(pmf).sum();
    // The sum of a whole range can round to just above 1.
    (low.min(1.0), half.min(1.0))
}

fn main() {
    let a = args();
    let pos = positions(&a);
    let count = |k: Kind| pos.iter().filter(|p| p.kind == k).count();
    println!(
        "{} positions from {} sittings at level {} halves: {} leads, {} follows, {} bids",
        pos.len(),
        a.sittings,
        a.level,
        count(Kind::Lead),
        count(Kind::Follow),
        count(Kind::Bid)
    );
    let config = Config::default();

    // 3. The whole hint (also gives the hinted card for the rest).
    header("3. whole hint(), default config");
    let mut hinted: Vec<Option<Card>> = vec![None; pos.len()];
    for (name, halves) in [("level 3", 6u8), ("level 4", 8)] {
        for kind in [Kind::Lead, Kind::Follow, Kind::Bid] {
            let mut times = Vec::new();
            for (i, p) in pos.iter().enumerate().filter(|(_, p)| p.kind == kind) {
                let (h, t) = ms(|| hint(&p.view, p.seed, Level::from_halves(halves), config));
                if halves == 8 {
                    if let Action::Play { card, .. } = &h.action {
                        hinted[i] = Some(*card);
                    }
                }
                times.push(t);
            }
            row(&format!("{name}, {kind:?}"), times);
        }
    }

    // 1. The lead-rate evidence.
    header("1. lead-rate evidence at n worlds (leads, level-4 hinted card)");
    let leads: Vec<usize> = (0..pos.len())
        .filter(|&i| pos[i].kind == Kind::Lead && hinted[i].is_some())
        .collect();
    for &n in &a.lead_ns {
        let (mut sample, mut one, mut every) = (Vec::new(), Vec::new(), Vec::new());
        for &i in &leads {
            let k = Knowledge::from_view(&pos[i].view).unwrap();
            let mut rng = Rng::stream(pos[i].seed, view_seed(&pos[i].view) ^ 0x4813);
            let (sims, ts) = ms(|| rung3::lead_sims(&k, &mut rng, n));
            let card = hinted[i].unwrap();
            let (_, t1) = ms(|| rung3::lead_wins(&sims, card));
            let (_, tall) = ms(|| {
                k.legal
                    .iter()
                    .map(|&c| rung3::lead_wins(&sims, c))
                    .sum::<usize>()
            });
            sample.push(ts);
            one.push(ts + t1);
            every.push(ts + tall);
        }
        row(&format!("n={n}: sampling only"), sample);
        row(&format!("n={n}: sampling + hinted card"), one);
        row(&format!("n={n}: sampling + every legal card"), every);
    }

    // 2. The odds.
    if a.odds {
        let all: Vec<usize> = (0..pos.len())
            .filter(|&i| pos[i].kind != Kind::Bid && hinted[i].is_some())
            .collect();
        for scaled in [false, true] {
            header(&format!(
                "2. odds_with, nodes {} (all play positions)",
                if scaled {
                    "3000 per world"
                } else {
                    "30000 shared"
                }
            ));
            for &w in &a.odds_worlds {
                let cfg = if scaled {
                    odds_config(config, w)
                } else {
                    config
                };
                let (mut times, mut got, mut finished) = (Vec::new(), 0usize, 0u32);
                for &i in &all {
                    let k = Knowledge::from_view(&pos[i].view).unwrap();
                    let mut rng = Rng::stream(pos[i].seed, view_seed(&pos[i].view) ^ 0x4812);
                    let (o, t) = ms(|| odds_with(&k, hinted[i].unwrap(), &mut rng, cfg, w));
                    times.push(t);
                    if let Some(o) = o {
                        got += 1;
                        finished += o.worlds;
                    }
                }
                row(&format!("worlds={w}"), times);
                println!(
                    "|  | odds returned {:.0}%, worlds finished {:.1} of {w} | | | |",
                    100.0 * got as f64 / all.len().max(1) as f64,
                    f64::from(finished) / got.max(1) as f64
                );
            }
        }
    }

    // 4. Noise.
    if a.noise {
        println!("\n### 4. lead-rate sampling noise (leads, level-4 hinted card)\n");
        let nmax = a.lead_ns.iter().copied().max().unwrap_or(1000).max(2000);
        let lf = ln_fact(nmax);
        let mut ps = Vec::new();
        for &i in &leads {
            let k = Knowledge::from_view(&pos[i].view).unwrap();
            let mut rng = Rng::stream(pos[i].seed, view_seed(&pos[i].view) ^ 0x4814);
            let sims = rung3::lead_sims(&k, &mut rng, 2000);
            ps.push(rung3::lead_wins(&sims, hinted[i].unwrap()) as f64 / sims.len() as f64);
        }
        let mut sorted = ps.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let q = |p: f64| sorted[((sorted.len() - 1) as f64 * p).round() as usize];
        println!(
            "proportion p of the hinted lead over 2000 worlds, {} leads: p10 {:.2}, p25 {:.2}, median {:.2}, p75 {:.2}, p90 {:.2}; <=0.2 in {:.0}%, >0.5 in {:.0}%\n",
            ps.len(),
            q(0.1),
            q(0.25),
            q(0.5),
            q(0.75),
            q(0.9),
            100.0 * ps.iter().filter(|&&p| p <= 0.2).count() as f64 / ps.len() as f64,
            100.0 * ps.iter().filter(|&&p| p > 0.5).count() as f64 / ps.len() as f64,
        );
        println!("| n | half-width at p=0.5 | mean half-width over these leads | wording flips (cheap vs rate) | side of 1/2 flips | any displayed count differs by more than 10 points |");
        println!("|---|---|---|---|---|---|");
        for &n in &a.lead_ns {
            let hw = |p: f64| 1.96 * (p * (1.0 - p) / n as f64).sqrt();
            let mean_hw = ps.iter().map(|&p| hw(p)).sum::<f64>() / ps.len() as f64;
            let (mut cheap, mut side, mut far) = (0.0, 0.0, 0.0);
            for &p in &ps {
                let (low, half) = binomial_tails(n, p, n * 2 / 10, &lf);
                cheap += 2.0 * low * (1.0 - low);
                side += 2.0 * half * (1.0 - half);
                // Two independent counts differ by more than 10 points: the
                // difference of proportions has sd sqrt(2) * sd; normal approx.
                let sd = (2.0 * p * (1.0 - p) / n as f64).sqrt();
                far += if sd == 0.0 {
                    0.0
                } else {
                    erfc(0.1 / (sd * std::f64::consts::SQRT_2))
                };
            }
            let m = ps.len() as f64;
            println!(
                "| {n} | {:.3} | {:.3} | {:.1}% | {:.1}% | {:.1}% |",
                hw(0.5),
                mean_hw,
                100.0 * cheap / m,
                100.0 * side / m,
                100.0 * far / m
            );
        }
    }
}

/// The complementary error function (Abramowitz and Stegun 7.1.26).
fn erfc(x: f64) -> f64 {
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = t
        * (0.254_829_592
            + t * (-0.284_496_736
                + t * (1.421_413_741 + t * (-1.453_152_027 + t * 1.061_405_429))));
    poly * (-x * x).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_binomial_tails_are_exact_on_small_cases() {
        let lf = ln_fact(10);
        // Bin(4, 1/2): P(<= 0) = 1/16, P(> 2) = 5/16.
        let (low, half) = binomial_tails(4, 0.5, 0, &lf);
        assert!((low - 1.0 / 16.0).abs() < 1e-12, "{low}");
        assert!((half - 5.0 / 16.0).abs() < 1e-12, "{half}");
        // p = 0 never wins, p = 1 always does.
        assert_eq!(binomial_tails(4, 0.0, 0, &lf), (1.0, 0.0));
        assert_eq!(binomial_tails(4, 1.0, 0, &lf), (0.0, 1.0));
    }

    #[test]
    fn a_tail_is_a_probability_even_when_the_sum_rounds_up() {
        // The whole range sums to 1 + rounding; a flip chance 2q(1 - q) must
        // not come out negative (it printed -0.0%).
        let lf = ln_fact(1000);
        for p in [0.1, 0.3, 0.5, 0.7, 0.9, 0.123_456] {
            let (low, half) = binomial_tails(1000, p, 1000, &lf);
            assert!(low <= 1.0 && half <= 1.0, "{p}: {low} {half}");
            assert!(low * (1.0 - low) >= 0.0);
        }
    }

    #[test]
    fn erfc_matches_known_values() {
        assert!((erfc(0.0) - 1.0).abs() < 1e-6);
        assert!((erfc(1.0) - 0.157_299_2).abs() < 1e-6);
    }
}
