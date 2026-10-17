//! M5r, the rule refinement of `docs/HINTS.md` 5.2 (`docs/MEASUREMENTS.md`,
//! "M5r"). The computations are in `strohmandeln_ai::m5r`.
//!
//! ```text
//! m5r select   [--seed 60000000] [--sittings 400] [--out DIR]     fit: log, table, selection.tsv
//! m5r confirm  [--seed 65000000] [--batch 100] [--cands FILE] [--out DIR]
//!                                                                  sequential confirmation: confirmed.tsv
//! m5r settings [--sittings 200] [--rules FILE] [--out DIR]         each other setting: settings.tsv
//! m5r emit     [--rules FILE] [--settings FILE]                    prints m5r_rules.rs
//! ```

use std::io::Write;
use std::time::Instant;
use strohmandeln_ai::m5r::*;
use strohmandeln_core::types::{BothPass, Settings, Values};

fn arg(rest: &[String], name: &str, default: &str) -> String {
    rest.iter()
        .position(|a| a == name)
        .and_then(|i| rest.get(i + 1).cloned())
        .unwrap_or_else(|| default.to_string())
}

fn num(rest: &[String], name: &str, default: u64) -> u64 {
    arg(rest, name, &default.to_string())
        .parse()
        .unwrap_or_else(|_| panic!("{name} N"))
}

fn names(atoms: &[usize]) -> String {
    atoms.iter().map(|&a| ATOMS[a].id).collect::<Vec<_>>().join(",")
}

fn atoms_of(text: &str) -> Vec<usize> {
    text.split(',').filter(|s| !s.is_empty()).map(atom_index).collect()
}

/// The settings of the settings run, with their seed blocks (HINTS 5.2).
const SETTINGS: [(&str, u64); 4] = [
    ("v123", 66_000_000),
    ("v124", 67_000_000),
    ("throwin", 68_000_000),
    ("ult2", 69_000_000),
];

fn setting_named(name: &str) -> Settings {
    match name {
        "v123" => Settings {
            values: Values::V123,
            ..Settings::default()
        },
        "v124" => Settings {
            values: Values::V124,
            ..Settings::default()
        },
        "throwin" => Settings {
            both_pass: BothPass::ThrowIn,
            ..Settings::default()
        },
        "ult2" => Settings {
            ultimo: 2,
            ..Settings::default()
        },
        other => panic!("unknown setting {other}"),
    }
}

fn main() {
    let mut argv: Vec<String> = std::env::args().skip(1).collect();
    assert!(!argv.is_empty(), "m5r select|confirm|settings|emit");
    let which = argv.remove(0);
    match which.as_str() {
        "select" => select_cmd(&argv),
        "confirm" => confirm_cmd(&argv),
        "settings" => settings_cmd(&argv),
        "emit" => emit_cmd(&argv),
        other => panic!("unknown command {other}"),
    }
}

fn select_cmd(rest: &[String]) {
    let started = Instant::now();
    let first = num(rest, "--seed", 60_000_000);
    let sittings = num(rest, "--sittings", 400);
    let out = arg(rest, "--out", ".");
    let log = collect(first, sittings, 0, Settings::default());
    println!(
        "M5r selection: {} deals ({sittings} sittings from {first}), {:.0}s",
        log.ndeals,
        started.elapsed().as_secs_f64()
    );
    std::fs::write(format!("{out}/selection.log.tsv"), log.to_tsv()).expect("the log");
    let sel = select(&log);
    let mut chosen = String::new();
    println!(
        "{:20} {:>6} {:>6} {:>6} {:>5} {:>4}  best candidate (fired, p, n_eff, lower, reach)",
        "class", "fired", "base", "baseLB", "cand", "elig"
    );
    for s in &sel {
        let cl = &CLASSES[s.class];
        let best = s.best.as_ref().map_or("none".to_string(), |b| {
            format!(
                "{}  {} {:.3} {:.0} {:.3} {:.3}",
                names(&b.atoms),
                b.prec.fired,
                b.prec.p,
                b.prec.n_eff,
                b.prec.lower,
                b.reach
            )
        });
        println!(
            "{:20} {:>6} {:>6.3} {:>6.3} {:>5} {:>4}  {best}",
            cl.id, s.base.fired, s.base.p, s.base.lower, s.ncand, s.eligible
        );
        if let Some(b) = &s.best {
            chosen.push_str(&format!(
                "{}\t{}\t{}\t{:.4}\t{:.1}\t{:.4}\t{:.4}\n",
                cl.id,
                names(&b.atoms),
                b.prec.fired,
                b.prec.p,
                b.prec.n_eff,
                b.prec.lower,
                b.reach
            ));
        }
    }
    std::fs::write(format!("{out}/selection.tsv"), chosen).expect("the selection");
    println!("total {:.0}s", started.elapsed().as_secs_f64());
}

fn confirm_cmd(rest: &[String]) {
    let started = Instant::now();
    let first = num(rest, "--seed", 65_000_000);
    let batch = num(rest, "--batch", 100);
    let out = arg(rest, "--out", ".");
    let cands = std::fs::read_to_string(arg(rest, "--cands", &format!("{out}/selection.tsv")))
        .expect("the selection");
    let mut live: Vec<(usize, Vec<usize>, String)> = cands
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            (class_index(f[0]), atoms_of(f[1]), l.to_string())
        })
        .collect();
    let mut log = Log::new();
    let mut verdicts = String::new();
    let mut lookfile = std::fs::File::create(format!("{out}/confirm.log")).expect("a log file");
    for look in 1..=BOUNDARIES.len() {
        let part = collect(first + (look as u64 - 1) * batch, batch, (look as u64 - 1) * batch, Settings::default());
        log.append(&part);
        let mut still = Vec::new();
        for (class, atoms, line) in live {
            let (p, reach) = evaluate(&log, class, &atoms);
            let st = step(look, &p);
            let msg = format!(
                "look {look} ({} sittings, {:.0}s) {} [{}] fired {} p {:.3} n_eff {:.0} lower {:.3} z {:.2} (boundary {:.2}) reach {:.3}: {:?}",
                look as u64 * batch,
                started.elapsed().as_secs_f64(),
                CLASSES[class].id,
                names(&atoms),
                p.fired,
                p.p,
                p.n_eff,
                p.lower,
                z_of(&p),
                BOUNDARIES[look - 1],
                reach,
                st
            );
            println!("{msg}");
            writeln!(lookfile, "{msg}").expect("log");
            match st {
                Step::Continue => still.push((class, atoms, line)),
                Step::Pass | Step::Fail => verdicts.push_str(&format!(
                    "{}\t{}\t{}\t{look}\t{}\t{:.4}\t{:.1}\t{:.4}\t{:.4}\n",
                    CLASSES[class].id,
                    names(&atoms),
                    if st == Step::Pass { "pass" } else { "fail" },
                    p.fired,
                    p.p,
                    p.n_eff,
                    p.lower,
                    reach
                )),
            }
        }
        live = still;
        if live.is_empty() {
            break;
        }
    }
    std::fs::write(format!("{out}/confirmed.tsv"), verdicts).expect("verdicts");
    println!("total {:.0}s", started.elapsed().as_secs_f64());
}

fn passed(rest: &[String], out: &str) -> Vec<(String, String, Vec<String>)> {
    let text = std::fs::read_to_string(arg(rest, "--rules", &format!("{out}/confirmed.tsv")))
        .expect("confirmed.tsv");
    text.lines()
        .map(|l| l.split('\t').map(str::to_string).collect::<Vec<_>>())
        .filter(|f| f[2] == "pass")
        .map(|f| (f[0].clone(), f[1].clone(), f))
        .collect()
}

fn settings_cmd(rest: &[String]) {
    let started = Instant::now();
    let sittings = num(rest, "--sittings", 200);
    let out = arg(rest, "--out", ".");
    let rules = passed(rest, &out);
    let mut result = String::new();
    let mut cells: Vec<Vec<String>> = vec![Vec::new(); rules.len()];
    for (name, seed) in SETTINGS {
        let log = collect(seed, sittings, 0, setting_named(name));
        for (i, (class, atoms, _)) in rules.iter().enumerate() {
            let (p, _) = evaluate(&log, class_index(class), &atoms_of(atoms));
            let cell = format!("{name}:{}:{:.4}:{:.1}:{:.4}", p.fired, p.p, p.n_eff, p.lower);
            println!("{class} [{atoms}] {cell}  ({:.0}s)", started.elapsed().as_secs_f64());
            cells[i].push(cell);
        }
    }
    for ((class, atoms, _), c) in rules.iter().zip(&cells) {
        result.push_str(&format!("{class}\t{atoms}\t{}\n", c.join("\t")));
    }
    std::fs::write(format!("{out}/settings.tsv"), result).expect("settings");
    println!("total {:.0}s", started.elapsed().as_secs_f64());
}

fn emit_cmd(rest: &[String]) {
    let out = arg(rest, "--out", ".");
    let rules = passed(rest, &out);
    let settings = std::fs::read_to_string(arg(rest, "--settings", &format!("{out}/settings.tsv")))
        .unwrap_or_default();
    println!("//! The rules that passed M5r (`docs/MEASUREMENTS.md`), generated by");
    println!("//! `m5r emit` from the confirmation and settings runs. Do not edit.\n");
    println!("use crate::m5r::PassedRule;\n");
    println!("pub const PASSED: &[PassedRule] = &[");
    for (class, atoms, f) in &rules {
        let ok = settings
            .lines()
            .find(|l| l.starts_with(&format!("{class}\t{atoms}\t")))
            .is_some_and(|l| {
                l.split('\t').skip(2).all(|c| {
                    let v: Vec<&str> = c.split(':').collect();
                    v[1] != "0" && v[4].parse::<f64>().is_ok_and(|x| x >= GATE)
                })
            });
        let reach: f64 = f[8].parse().expect("reach");
        let list: Vec<String> = atoms_of(atoms).iter().map(|&a| format!("{:?}", ATOMS[a].id)).collect();
        println!("    PassedRule {{");
        println!("        id: \"{class}:{}\",", atoms.replace(',', "+"));
        println!("        class: \"{class}\",");
        println!("        atoms: &[{}],", list.join(", "));
        println!("        fired: {},", f[4]);
        println!("        precision: {},", f[5]);
        println!("        n_eff: {},", f[6]);
        println!("        lower: {},", f[7]);
        println!("        reach: {reach},");
        println!("        too_rare: {},", reach < 1.0 / 40.0);
        println!("        settings_ok: {ok},");
        let atom_ids: Vec<&str> = atoms_of(atoms).iter().map(|&a| ATOMS[a].id).collect();
        let (cw, aw) = word_counts(&atom_ids, class);
        println!("        fits_budget: {},", cw <= 8 && aw <= 10);
        println!("    }},");
    }
    println!("];");
}
