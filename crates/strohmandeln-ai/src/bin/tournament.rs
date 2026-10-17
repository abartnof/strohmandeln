//! Measures one rung against another: mirrored pairs, run sequentially with
//! O'Brien-Fleming looks and a futility stop, the plan fixed before the first
//! batch (CLAUDE.md, "Simulations stop early when the signal is clear").
//!
//! ```text
//! tournament A B [--touren 3|6] [--batch N] [--looks K] [--negligible X]
//!                [--seed S] [--out FILE.tsv] [--baseline C]
//! ```
//!
//! With `--baseline C` the unit is the exploit's D (`docs/STYLES.md` 4.3):
//! the Points A takes from B over a mirrored pair, less the Points C takes
//! from B on the same deals. `counter=ID` in a spec sets Normal's counter to
//! the style `ID` (`Config::counter`), the rest then override it.
//!
//! A and B are specs: a level on the dial (`1`, `2`, `3`, `3.5`, `4`; a half
//! snaps to the nearest, ties down), optionally
//! followed by `@` and comma-separated settings, such as `2@bid=40/5` (the
//! rung-2 bid rule: sure-winner thirds / least tarocks); `worlds=`, `nodes=`,
//! `bidworlds=` and `bidmargin=` set rung 4's search; `w.burden=2` and the
//! like set a weight of the maxims (`rung2::Weights`); `style=bermann` and the
//! like set a style first (`style::Style`, not `random`), the rest then
//! override it. The unit of
//! observation is the Points A takes from B over one mirrored pair of
//! sittings (each of `--touren` Touren, so 6 or 12 deals a seating).

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use strohmandeln_ai::opponent::{Config, Level, Opponent};
use strohmandeln_ai::rung2::BidRule;
use strohmandeln_ai::stats::{run_sequential, Plan, Verdict};
use strohmandeln_ai::style::Style;
use strohmandeln_ai::tournament::{mirrored, paired_gain};
use strohmandeln_core::agents::Player;
use strohmandeln_core::types::Seat;

fn parse_spec(spec: &str) -> (Level, Config) {
    let (level, settings) = spec.split_once('@').unwrap_or((spec, ""));
    let level = Level::from_f64(level.parse().unwrap_or_else(|_| panic!("level in {spec}")));
    // A `style=` setting is applied first; the others then override it.
    let style = settings
        .split(',')
        .find_map(|kv| kv.strip_prefix("style="))
        .map(|id| {
            Style::parse(id)
                .filter(|s| *s != Style::Random)
                .unwrap_or_else(|| panic!("a style (not random) in {spec}"))
        })
        .unwrap_or_default();
    let counter = settings
        .split(',')
        .find_map(|kv| kv.strip_prefix("counter="))
        .map(|id| Style::parse(id).unwrap_or_else(|| panic!("a style in {spec}")));
    let mut config = match counter {
        Some(c) => Config::counter(c),
        None => Config::for_style(style, 0),
    };
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
                    ..config.bid
                };
            }
            "style" | "counter" => {}
            "worlds" => config.budget.worlds = v.parse().expect("worlds"),
            "nodes" => config.budget.nodes = v.parse().expect("nodes"),
            "bidworlds" => config.budget.bid_worlds = v.parse().expect("bidworlds"),
            "bidmargin" => config.bid_margin = Some(v.parse().expect("bidmargin")),
            "bidrule" => config.bid_margin = None,
            w if w.starts_with("w.") => {
                let x: f64 = v.parse().expect("weight");
                let wt = &mut config.weights;
                match &w[2..] {
                    "burden" => wt.burden = x,
                    "cover" => wt.cover = x,
                    "exposed_figure" => wt.exposed_figure = x,
                    "split_suit" => wt.split_suit = x,
                    "keep_packet" => wt.keep_packet = x,
                    "hold_pagat" => wt.hold_pagat = x,
                    "cash_king" => wt.cash_king = x,
                    "draw_trumps" => wt.draw_trumps = x,
                    "solid_run" => wt.solid_run = x,
                    "keep_tarocks" => wt.keep_tarocks = x,
                    "high_trump_lead" => wt.high_trump_lead = x,
                    "coverless_suit" => wt.coverless_suit = x,
                    other => panic!("unknown weight {other}"),
                }
            }
            other => panic!("unknown setting {other}"),
        }
    }
    (level, config)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "tournament A B [--touren 3|6] [--batch N] [--looks K] [--negligible X] [--seed S] [--out FILE] [--baseline C]";
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
    let mut baseline: Option<(String, Level, Config)> = None;
    let mut i = 2;
    while i < args.len() {
        let value = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--touren" => touren = value.parse().expect("--touren 3|6"),
            "--batch" => plan.batch = value.parse().expect("--batch N"),
            "--looks" => plan.looks = value.parse().expect("--looks K"),
            "--negligible" => plan.negligible = value.parse().expect("--negligible X"),
            "--seed" => first_seed = value.parse().expect("--seed S"),
            "--baseline" => {
                let (l, c) = parse_spec(&value);
                baseline = Some((value.clone(), l, c));
            }
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
    match &baseline {
        Some((spec_c, ..)) => println!(
            "D = Points {spec_a} takes from {spec_b} less Points {spec_c} takes from {spec_b}, per mirrored pair of sittings of {touren} Touren"
        ),
        None => println!(
            "{spec_a} against {spec_b}: Points per mirrored pair of sittings of {touren} Touren"
        ),
    }
    let fc = baseline
        .as_ref()
        .map(|(_, l, c)| factory(*l, *c, 0xA));
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
            let v = match &fc {
                Some(fc) => {
                    paired_gain(seed, Default::default(), touren, &fa, fc, &fb)
                }
                None => mirrored(seed, Default::default(), touren, &fa, &fb),
            };
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
