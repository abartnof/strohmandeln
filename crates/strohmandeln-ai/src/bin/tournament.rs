//! Measures one rung against another: mirrored pairs, run sequentially with
//! O'Brien-Fleming looks and a futility stop, the plan fixed before the first
//! batch (CLAUDE.md, "Simulations stop early when the signal is clear").
//!
//! ```text
//! tournament A B [--touren 3|6] [--batch N] [--looks K] [--negligible X]
//!                [--seed S] [--out FILE.tsv]
//! ```
//!
//! A and B are specs: a level (`1` to `4`, or in halves `2.5`), optionally
//! followed by `@` and comma-separated settings, such as `2@bid=40/5` (the
//! rung-2 bid rule: sure-winner thirds / least tarocks). The unit of
//! observation is the Points A takes from B over one mirrored pair of
//! sittings (each of `--touren` Touren, so 6 or 12 deals a seating).

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use strohmandeln_ai::opponent::{Config, Level, Opponent};
use strohmandeln_ai::rung2::BidRule;
use strohmandeln_ai::stats::{run_sequential, Plan, Verdict};
use strohmandeln_ai::tournament::mirrored;
use strohmandeln_core::agents::Player;
use strohmandeln_core::types::Seat;

fn parse_spec(spec: &str) -> (Level, Config) {
    let (level, settings) = spec.split_once('@').unwrap_or((spec, ""));
    let level = Level::from_f64(level.parse().unwrap_or_else(|_| panic!("level in {spec}")));
    let mut config = Config::default();
    for kv in settings.split(',').filter(|s| !s.is_empty()) {
        let (k, v) = kv
            .split_once('=')
            .unwrap_or_else(|| panic!("key=value in {kv}"));
        match k {
            "bid" => {
                let (t, n) = v.split_once('/').expect("bid=THIRDS/TAROCKS");
                config.bid = BidRule {
                    sure_thirds: t.parse().expect("thirds"),
                    min_tarocks: n.parse().expect("tarocks"),
                };
            }
            other => panic!("unknown setting {other}"),
        }
    }
    (level, config)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "tournament A B [--touren 3|6] [--batch N] [--looks K] [--negligible X] [--seed S] [--out FILE]";
    if args.len() < 2 {
        panic!("{usage}");
    }
    let (spec_a, spec_b) = (args[0].clone(), args[1].clone());
    let (la, ca) = parse_spec(&spec_a);
    let (lb, cb) = parse_spec(&spec_b);
    let mut touren = 3u32;
    let mut plan = Plan {
        batch: 50,
        looks: 4,
        negligible: 0.0,
    };
    let mut first_seed = 1_000_000u64;
    let mut out: Option<File> = None;
    let mut i = 2;
    while i < args.len() {
        let value = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--touren" => touren = value.parse().expect("--touren 3|6"),
            "--batch" => plan.batch = value.parse().expect("--batch N"),
            "--looks" => plan.looks = value.parse().expect("--looks K"),
            "--negligible" => plan.negligible = value.parse().expect("--negligible X"),
            "--seed" => first_seed = value.parse().expect("--seed S"),
            "--out" => out = Some(File::create(&value).expect("--out FILE")),
            other => panic!("unknown {other}; {usage}"),
        }
        i += 2;
    }
    let factory = |level: Level, config: Config, salt: u64| {
        move |seed: u64, seat: Seat| -> Box<dyn Player> {
            Box::new(
                Opponent::new(level, seed ^ salt ^ ((seat.index() as u64 + 1) * 0x9e37))
                    .with_config(config),
            )
        }
    };
    let (fa, fb) = (factory(la, ca, 0xA), factory(lb, cb, 0xB));
    println!("{spec_a} against {spec_b}: Points per mirrored pair of sittings of {touren} Touren");
    println!(
        "plan: batches of {}, at most {} looks, boundaries {:?}, negligible +-{}, first seed {first_seed}",
        plan.batch,
        plan.looks,
        plan.boundaries()
            .iter()
            .map(|z| (z * 100.0).round() / 100.0)
            .collect::<Vec<_>>(),
        plan.negligible
    );
    let started = Instant::now();
    let report = run_sequential(
        plan,
        |i| {
            let seed = first_seed + i as u64;
            let v = mirrored(seed, Default::default(), touren, &fa, &fb);
            if let Some(f) = out.as_mut() {
                writeln!(f, "{seed}\t{v}").expect("write");
            }
            f64::from(v)
        },
        |look| {
            println!(
                "look {}: {} pairs, {:+.3} +- {:.3} (z {:+.2}, boundary {:.2}), {:.1}s",
                look.look,
                look.n,
                look.mean,
                look.se,
                look.z,
                look.boundary,
                started.elapsed().as_secs_f64()
            );
        },
    );
    let l = report.verdict.last();
    let interval = format!("95% interval {:+.3} to {:+.3}", l.low(), l.high());
    match report.verdict {
        Verdict::Clear(_) => println!(
            "CLEAR at look {} ({} pairs): {spec_a} {} {spec_b} by {:.3} +- {:.3}; {interval}",
            l.look,
            l.n,
            if l.mean > 0.0 { "beats" } else { "loses to" },
            l.mean.abs(),
            l.se
        ),
        Verdict::Negligible(_) => println!(
            "NEGLIGIBLE at look {} ({} pairs): {:+.3}; {interval}",
            l.look, l.n, l.mean
        ),
        Verdict::Inconclusive(_) => println!(
            "NO CLEAR DIFFERENCE after {} pairs: {:+.3}; {interval}",
            l.n, l.mean
        ),
    }
}
