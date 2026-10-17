//! M5r, the rule refinement of `docs/HINTS.md` 5.2 (`docs/MEASUREMENTS.md`,
//! "M5r"). The computations are in `strohmandeln_ai::m5r`.
//!
//! ```text
//! m5r select   [--seed 71000000] [--sittings 400] [--out DIR]    fit: selection.log.tsv, table, selection.tsv
//! m5r confirm  [--seed 76000000] [--batch 100] [--k N] [--cands FILE] [--out DIR]
//!                                       sequential confirmation at the Bonferroni boundaries for k
//!                                       classes (default: the candidates): confirmed.tsv, confirm.log
//! m5r settings [--sittings 200] [--rules FILE] [--out DIR]       each other setting: settings.tsv
//! m5r emit     [--confirmed FILE] [--settings FILE]              prints m5r_rules.rs
//! m5r bounds   [--k N]                                           prints the boundaries for k classes
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

/// The settings of the settings run, with their seed blocks.
const SETTINGS: [(&str, u64); 4] = [
    ("v123", 77_000_000),
    ("v124", 78_000_000),
    ("throwin", 79_000_000),
    ("ult2", 80_000_000),
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
    assert!(!argv.is_empty(), "m5r select|confirm|settings|emit|bounds");
    let which = argv.remove(0);
    match which.as_str() {
        "select" => select_cmd(&argv),
        "confirm" => confirm_cmd(&argv),
        "settings" => settings_cmd(&argv),
        "emit" => emit_cmd(&argv),
        "bounds" => println!("{:?}", family_boundaries(num(&argv, "--k", 1) as usize)),
        other => panic!("unknown command {other}"),
    }
}

fn select_cmd(rest: &[String]) {
    let started = Instant::now();
    let first = num(rest, "--seed", 71_000_000);
    let sittings = num(rest, "--sittings", 400);
    let out = arg(rest, "--out", ".");
    let log = collect(first, sittings, 0, Settings::default());
    println!(
        "M5r selection: {} deals ({sittings} sittings from {first}), {:.0}s wall",
        log.ndeals,
        started.elapsed().as_secs_f64()
    );
    std::fs::write(format!("{out}/selection.log.tsv"), log.to_tsv()).expect("the log");
    let sel = select(&log);
    let mut chosen = String::new();
    println!(
        "{:20} {:>6} {:>6} {:>6} {:>5} {:>4}  best candidate (atoms, fired, p, n_eff, lower, reach, late10)",
        "class", "fired", "base", "baseLB", "cand", "elig"
    );
    for s in &sel {
        let cl = &CLASSES[s.class];
        let best = s.best.as_ref().map_or("none".to_string(), |b| {
            format!(
                "{}  {} {:.3} {:.0} {:.3} {:.3} {:.3}",
                names(&b.atoms),
                b.prec.fired,
                b.prec.p,
                b.prec.n_eff,
                b.prec.lower,
                b.reach,
                b.late10
            )
        });
        println!(
            "{:20} {:>6} {:>6.3} {:>6.3} {:>5} {:>4}  {best}",
            cl.id, s.base.fired, s.base.p, s.base.lower, s.ncand, s.eligible
        );
        if let Some(b) = &s.best {
            chosen.push_str(&format!(
                "{}\t{}\t{}\t{:.4}\t{:.1}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\n",
                cl.id,
                names(&b.atoms),
                b.prec.fired,
                b.prec.p,
                b.prec.n_eff,
                b.prec.lower,
                b.reach,
                b.late10,
                b.late19
            ));
        }
    }
    std::fs::write(format!("{out}/selection.tsv"), &chosen).expect("the selection");
    println!(
        "{} classes taken forward; total {:.0}s wall",
        chosen.lines().count(),
        started.elapsed().as_secs_f64()
    );
}

fn confirm_cmd(rest: &[String]) {
    let started = Instant::now();
    let first = num(rest, "--seed", 76_000_000);
    let batch = num(rest, "--batch", 100);
    let out = arg(rest, "--out", ".");
    let cands = std::fs::read_to_string(arg(rest, "--cands", &format!("{out}/selection.tsv")))
        .expect("the selection");
    let mut live: Vec<(usize, Vec<usize>)> = cands
        .lines()
        .map(|l| {
            let f: Vec<&str> = l.split('\t').collect();
            (class_index(f[0]), atoms_of(f[1]))
        })
        .collect();
    let k = num(rest, "--k", live.len() as u64) as usize;
    let bounds = family_boundaries(k);
    let mut log = Log::new();
    let mut verdicts = String::new();
    let mut lookfile = std::fs::File::create(format!("{out}/confirm.log")).expect("a log file");
    let head = format!("k = {k} classes, boundaries {bounds:?}, futility z <= {FUTILITY_Z}");
    println!("{head}");
    writeln!(lookfile, "{head}").expect("log");
    for look in 1..=LOOKS {
        let part = collect(
            first + (look as u64 - 1) * batch,
            batch,
            (look as u64 - 1) * batch,
            Settings::default(),
        );
        log.append(&part);
        let mut still = Vec::new();
        for (class, atoms) in live {
            let e = evaluate(&log, class, &atoms);
            let p = e.prec;
            let st = step(look, &p, &bounds);
            let msg = format!(
                "look {look} ({} sittings, {:.0}s wall) {} [{}] fired {} p {:.3} n_eff {:.0} lower {:.3} z {:.2} (boundary {:.2}) reach {:.3} late10 {:.3}: {:?}",
                look as u64 * batch,
                started.elapsed().as_secs_f64(),
                CLASSES[class].id,
                names(&atoms),
                p.fired,
                p.p,
                p.n_eff,
                p.lower,
                z_of(&p),
                bounds[look - 1],
                e.reach,
                e.late10,
                st
            );
            println!("{msg}");
            writeln!(lookfile, "{msg}").expect("log");
            match st {
                Step::Continue => still.push((class, atoms)),
                Step::Pass | Step::Fail => verdicts.push_str(&format!(
                    "{}\t{}\t{}\t{look}\t{}\t{:.4}\t{:.1}\t{:.4}\t{:.4}\t{:.4}\t{:.4}\n",
                    CLASSES[class].id,
                    names(&atoms),
                    if st == Step::Pass { "pass" } else { "fail" },
                    p.fired,
                    p.p,
                    p.n_eff,
                    p.lower,
                    e.reach,
                    e.late10,
                    e.late19
                )),
            }
        }
        live = still;
        if live.is_empty() {
            break;
        }
    }
    std::fs::write(format!("{out}/confirmed.tsv"), verdicts).expect("verdicts");
    println!("total {:.0}s wall", started.elapsed().as_secs_f64());
}

fn passed(rest: &[String], out: &str) -> Vec<(String, String)> {
    let text = std::fs::read_to_string(arg(rest, "--rules", &format!("{out}/confirmed.tsv")))
        .expect("confirmed.tsv");
    text.lines()
        .map(|l| l.split('\t').map(str::to_string).collect::<Vec<_>>())
        .filter(|f| f[2] == "pass")
        .map(|f| (f[0].clone(), f[1].clone()))
        .collect()
}

fn settings_cmd(rest: &[String]) {
    let started = Instant::now();
    let sittings = num(rest, "--sittings", 200);
    let out = arg(rest, "--out", ".");
    let rules = passed(rest, &out);
    let mut cells: Vec<Vec<String>> = vec![Vec::new(); rules.len()];
    for (name, seed) in SETTINGS {
        let log = collect(seed, sittings, 0, setting_named(name));
        for (i, (class, atoms)) in rules.iter().enumerate() {
            let p = evaluate(&log, class_index(class), &atoms_of(atoms)).prec;
            let cell = format!("{name}:{}:{:.4}:{:.1}:{:.4}", p.fired, p.p, p.n_eff, p.lower);
            println!("{class} [{atoms}] {cell}  ({:.0}s wall)", started.elapsed().as_secs_f64());
            cells[i].push(cell);
        }
    }
    let mut result = String::new();
    for ((class, atoms), c) in rules.iter().zip(&cells) {
        result.push_str(&format!("{class}\t{atoms}\t{}\n", c.join("\t")));
    }
    std::fs::write(format!("{out}/settings.tsv"), result).expect("settings");
    println!("total {:.0}s wall", started.elapsed().as_secs_f64());
}

fn emit_cmd(rest: &[String]) {
    let confirmed = std::fs::read_to_string(arg(rest, "--confirmed", "confirmed.tsv"))
        .expect("confirmed.tsv");
    let settings = std::fs::read_to_string(arg(rest, "--settings", "settings.tsv")).unwrap_or_default();
    print!("{}", emit_source(&confirmed, &settings));
}
