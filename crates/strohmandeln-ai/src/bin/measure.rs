//! The tutor's measurements, M1 to M8 (`docs/MEASUREMENTS.md`). Each prints
//! its plan's numbers; the computations are in `strohmandeln_ai::measure`.
//!
//! ```text
//! measure m1 [--seed S] [--plays N] [--bids N] [--streams K]
//! ```

use std::time::Instant;
use strohmandeln_ai::know::Knowledge;
use strohmandeln_ai::measure::{self, Position};
use strohmandeln_ai::opponent::Config;
use strohmandeln_ai::rung4::Budget;
use strohmandeln_ai::tutor::Skill;
use strohmandeln_core::types::Action;

struct Args {
    rest: Vec<String>,
}

impl Args {
    fn num(&self, name: &str, default: u64) -> u64 {
        self.rest
            .iter()
            .position(|a| a == name)
            .map_or(default, |i| {
                self.rest
                    .get(i + 1)
                    .and_then(|v| v.parse().ok())
                    .unwrap_or_else(|| panic!("{name} N"))
            })
    }
}

fn main() {
    let mut argv: Vec<String> = std::env::args().skip(1).collect();
    if argv.is_empty() {
        panic!("measure m1|m2|...");
    }
    let which = argv.remove(0);
    let args = Args { rest: argv };
    match which.as_str() {
        "m1" => m1(&args),
        "m2" => m2(&args),
        other => panic!("unknown measurement {other}"),
    }
}

/// Every 7th play and every 2nd bid of the sittings from `first_seed`, until
/// the counts are reached.
fn collect(first_seed: u64, plays: usize, bids: usize) -> (Vec<Position>, Vec<Position>, u64) {
    let (mut p, mut b) = (Vec::new(), Vec::new());
    let (mut pi, mut bi) = (0usize, 0usize);
    let mut seed = first_seed;
    while p.len() < plays || b.len() < bids {
        for rec in measure::rung4_deals(seed, Config::default()) {
            let (ps, bs) = measure::decisions(&rec);
            for x in ps {
                if p.len() < plays && pi % 7 == 0 {
                    p.push(x);
                }
                pi += 1;
            }
            for x in bs {
                if b.len() < bids && bi % 2 == 0 {
                    b.push(x);
                }
                bi += 1;
            }
        }
        seed += 1;
    }
    (p, b, seed - first_seed)
}

fn m1(args: &Args) {
    let first = args.num("--seed", 10_000_000);
    let plays = args.num("--plays", 600) as usize;
    let bids = args.num("--bids", 200) as usize;
    let streams = args.num("--streams", 8);
    let budget = Budget::default();
    let started = Instant::now();
    let (pp, bp, sittings) = collect(first, plays, bids);
    println!(
        "M1: {} plays and {} bids from {sittings} sittings of rung 4 against rung 4 (seeds {first}..{}), {streams} streams, {:.0}s",
        pp.len(),
        bp.len(),
        first + sittings - 1,
        started.elapsed().as_secs_f64()
    );
    let mut reads = Vec::new();
    let mut starved = 0;
    for pos in &pp {
        let k = Knowledge::from_view(&pos.view).expect("a deal");
        let Action::Play { card, .. } = &pos.action else {
            continue;
        };
        match measure::play_tables(&k, &pos.view, budget, streams) {
            Some(t) => reads.push(measure::read_play(&k, *card, &t)),
            None => starved += 1,
        }
    }
    println!(
        "plays read: {} ({} starved), {:.0}s",
        reads.len(),
        starved,
        started.elapsed().as_secs_f64()
    );
    let mut bid_reads = Vec::new();
    for pos in &bp {
        let k = Knowledge::from_view(&pos.view).expect("a deal");
        if let Some(r) = measure::read_bid(&k, &pos.view, budget, streams) {
            bid_reads.push(r);
        }
    }
    println!(
        "bids read: {}, {:.0}s",
        bid_reads.len(),
        started.elapsed().as_secs_f64()
    );

    let split = measure::by_skill(&reads);
    let noises: Vec<_> = Skill::ALL
        .iter()
        .map(|&s| measure::noise_of(s, &split[s.index()]))
        .collect();
    println!("\nnoise: across-stream standard deviation of the gap, thirds (positions not exact)");
    println!(
        "{:8} {:>5} {:>5} {:>6} {:>6} {:>6} {:>7} {:>7} {:>5} {:>7} {:>8}",
        "skill",
        "pos",
        "exact",
        "median",
        "p90",
        "p95",
        "calMed",
        "calRMS",
        "noSE",
        "select",
        "exSpread"
    );
    for n in &noises {
        println!(
            "{:8} {:>5} {:>5} {:>6.2} {:>6.2} {:>6.2} {:>7.2} {:>7.2} {:>5} {:>7.2} {:>8.2}",
            n.skill.slug(),
            n.positions,
            n.exact_spread.len(),
            measure::median(&n.spread),
            measure::quantile(&n.spread, 0.9),
            measure::quantile(&n.spread, 0.95),
            n.calibration_median,
            n.calibration_rms,
            n.unbounded,
            n.selection,
            measure::quantile(&n.exact_spread, 0.95),
        );
    }
    let margin = measure::margin_rule(&noises);
    let exact = measure::exact_margin_rule(&noises);
    println!("\nMARGIN by the rule: {margin} thirds; EXACT_MARGIN: {exact} thirds");
    for k in [2.0, 2.5, 3.0] {
        println!("\nreplication at {k} standard errors beyond {margin} thirds:");
        println!(
            "{:8} {:>7} {:>8} {:>8} {:>9} {:>9}",
            "skill", "clear", "sameSign", "share", "again", "share"
        );
        for s in Skill::ALL {
            let (clear, same, again, pairs) = measure::replication(&split[s.index()], margin, k);
            if clear == 0 {
                continue;
            }
            println!(
                "{:8} {:>7} {:>8} {:>8.3} {:>9} {:>9.3}",
                s.slug(),
                clear,
                same,
                same as f64 / clear as f64,
                again,
                again as f64 / pairs as f64
            );
        }
    }
    println!("\nbids: mean (Points per unit of doubling) across {streams} streams");
    let sds: Vec<f64> = bid_reads
        .iter()
        .map(|r| measure::sd(&r.iter().map(|x| x.0).collect::<Vec<_>>()))
        .collect();
    let ses: Vec<f64> = bid_reads
        .iter()
        .map(|r| measure::mean(&r.iter().map(|x| x.1).collect::<Vec<_>>()))
        .collect();
    println!(
        "spread median {:.3} p90 {:.3} p95 {:.3}; reported error median {:.3}; ratio of medians {:.2}",
        measure::median(&sds),
        measure::quantile(&sds, 0.9),
        measure::quantile(&sds, 0.95),
        measure::median(&ses),
        measure::median(&sds) / measure::median(&ses)
    );
    println!(
        "BID_MARGIN by the rule: {:.2} Points",
        measure::bid_margin_rule(&bid_reads)
    );
    println!("\ntotal {:.0}s", started.elapsed().as_secs_f64());
}

fn m2(args: &Args) {
    let first = args.num("--seed", 20_000_000);
    let sittings = args.num("--sittings", 300);
    let started = Instant::now();
    let (mut x, mut y, mut cluster) = (Vec::new(), Vec::new(), Vec::new());
    let mut thrown_in = 0;
    for i in 0..sittings {
        for rec in measure::rung4_deals(first + i, Config::default()) {
            match measure::deal_margin_points(&rec) {
                Some((a, b)) => {
                    x.push(a);
                    y.push(b);
                    cluster.push(i as usize);
                }
                None => thrown_in += 1,
            }
        }
    }
    println!(
        "M2: {} played deals ({} thrown in) from {sittings} sittings of rung 4 against rung 4 (seeds {first}..{}), {:.0}s",
        x.len(),
        thrown_in,
        first + sittings - 1,
        started.elapsed().as_secs_f64()
    );
    let o = measure::ols_clustered(&x, &y, &cluster);
    let (lo, hi) = (o.slope - 1.96 * o.se_slope, o.slope + 1.96 * o.se_slope);
    println!(
        "net Points per third: slope {:.5} +- {:.5} (cluster), intercept {:.3}, correlation {:.3}",
        o.slope, o.se_slope, o.intercept, o.correlation
    );
    println!(
        "thirds per Point = 1/slope = {:.2} (95% interval {:.2} to {:.2}); rounded {}",
        1.0 / o.slope,
        1.0 / hi,
        1.0 / lo,
        (1.0 / o.slope).round()
    );
    let rev = measure::ols_clustered(&y, &x, &cluster);
    println!(
        "reverse regression (thirds on Points): {:.2} thirds per Point",
        rev.slope
    );
    println!("\nmean net Points by decile of the margin (thirds):");
    let mut order: Vec<usize> = (0..x.len()).collect();
    order.sort_by(|&a, &b| x[a].total_cmp(&x[b]));
    for d in 0..10 {
        let part = &order[d * order.len() / 10..(d + 1) * order.len() / 10];
        let xs: Vec<f64> = part.iter().map(|&i| x[i]).collect();
        let ys: Vec<f64> = part.iter().map(|&i| y[i]).collect();
        println!(
            "  margin {:>6.1} to {:>6.1}: mean {:>6.1}, points {:>6.2}",
            xs[0],
            xs[xs.len() - 1],
            measure::mean(&xs),
            measure::mean(&ys)
        );
    }
}
