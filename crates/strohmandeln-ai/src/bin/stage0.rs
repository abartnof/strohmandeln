//! Stage 0 of `docs/STYLES.md` 4.1: do the styles differ in play, and how much
//! do they bid? Run it after `cargo build --release -p strohmandeln-ai --bin
//! stage0`.
//!
//! ```text
//! stage0 [--deals N] [--seed S] [--skills 3,4] [--styles a,b,...]
//!        [--calib-hands N] [--out FILE.csv]
//! ```
//!
//! For each style (default: Normal and every member of the Random pool) at each
//! skill (default 3 and 4), `--deals` deals (rounded up to whole sittings of 3
//! Touren) are played with the style in both seats; on every decision with a
//! choice Normal's choice on the same view is compared. Reported: divergence
//! from Normal (share of decisions whose card or bid differs), bid rate, and
//! at skill 4 the share of plays with a near tie inside Normal's band (0.34
//! thirds) and inside the style's. Then, once, the rung-4 bid-margin
//! calibration: for each style, the share of `--calib-hands` first-bidder hands
//! its rule takes up, and the advantage (Points) at which Normal's rung-4 sample
//! bids that share (`style.rs` `MARGINS`).
//!
//! `--out` writes one CSV row per deal and seat with the tells counted
//! (`probe::Features`), for the tells gate of 4.2. Normal's rows are its null
//! control: run again with another `--seed` for the disjoint-seed control.
//!
//! Single-threaded. Time per deal is printed for each style and skill.

use std::fs::File;
use std::io::Write;
use std::time::Instant;
use strohmandeln_ai::opponent::{Config, Level};
use strohmandeln_ai::probe::{csv_header, csv_line, run_style, styled, Calibration};
use strohmandeln_ai::style::Style;

const USAGE: &str = "stage0 [--deals N] [--seed S] [--skills 3,4] [--styles a,b,...] \
[--calib-hands N] [--out FILE.csv]";

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut deals = 1000u64;
    let mut seed = 5_000_000u64;
    let mut skills: Vec<f64> = vec![3.0, 4.0];
    let mut styles: Vec<Style> = std::iter::once(Style::Normal).chain(Style::POOL).collect();
    let mut calib_hands: Option<u64> = None;
    let mut out: Option<File> = None;
    let mut i = 0;
    while i < args.len() {
        let value = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--deals" => deals = value.parse().expect("--deals N"),
            "--seed" => seed = value.parse().expect("--seed S"),
            "--skills" => {
                skills = value
                    .split(',')
                    .map(|s| s.parse().expect("--skills 3,4"))
                    .collect()
            }
            "--styles" => {
                styles = value
                    .split(',')
                    .map(|s| {
                        Style::parse(s)
                            .filter(|s| *s != Style::Random)
                            .unwrap_or_else(|| panic!("a style (not random): {s}"))
                    })
                    .collect()
            }
            "--calib-hands" => calib_hands = Some(value.parse().expect("--calib-hands N")),
            "--out" => out = Some(File::create(&value).expect("--out FILE")),
            other => panic!("unknown {other}; {USAGE}"),
        }
        i += 2;
    }
    // A sitting of 3 Touren is 6 deals unless a deal is void.
    let sittings = deals.div_ceil(6).max(1);
    let base = Config::default();
    if let Some(f) = out.as_mut() {
        writeln!(f, "{}", csv_header()).expect("write");
    }
    println!(
        "stage 0: {deals} deals ({sittings} sittings of 3 Touren, a style in both seats), \
first seed {seed}"
    );
    println!(
        "style      skill   deals  decisions  divergence  bid rate  band .34  style band  s/deal"
    );
    for &skill in &skills {
        let level = Level::from_f64(skill);
        for &style in &styles {
            let started = Instant::now();
            let (t, rows) = run_style(level, style, seed..seed + sittings, &base);
            let per_deal = started.elapsed().as_secs_f64() / t.deals.max(1) as f64;
            let band = |x: f64| {
                if level.rung() == 4 {
                    format!("{:>7.1}%", 100.0 * x)
                } else {
                    "      -".to_string()
                }
            };
            println!(
                "{:<10} {:>5} {:>7} {:>10} {:>10.2}% {:>8.1}% {:>9} {:>11} {:>7.2}",
                style.id(),
                level.as_f64(),
                t.deals,
                t.decisions,
                100.0 * t.divergence(),
                100.0 * t.bid_rate(),
                band(t.normal_band()),
                band(t.style_band()),
                per_deal,
            );
            if let Some(f) = out.as_mut() {
                for r in &rows {
                    writeln!(f, "{}", csv_line(style, level, r)).expect("write");
                }
            }
        }
    }
    let hands = calib_hands.unwrap_or(deals.min(1000));
    let started = Instant::now();
    let cal = Calibration::sample(seed..seed + hands, base.budget.bid_worlds);
    println!(
        "\nrung-4 bid margin: {hands} first-bidder hands, {} worlds each ({:.1}s)",
        base.budget.bid_worlds,
        started.elapsed().as_secs_f64()
    );
    println!(
        "Normal's rung-4 sample bids {:.1}% at margin 0; its rule bids {:.1}%",
        100.0 * cal.share_at(0.0),
        100.0 * cal.share(base.bid)
    );
    println!("style      rule bids   margin (Points)");
    for &style in &styles {
        let share = cal.share(styled(&base, style).bid);
        println!(
            "{:<10} {:>8.1}% {:>+10.3}",
            style.id(),
            100.0 * share,
            cal.margin(share)
        );
    }
}
