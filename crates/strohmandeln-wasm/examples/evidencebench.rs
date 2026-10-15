//! Times the tutor's evidence of a deal natively on the records
//! `tests/evidencebench.mjs` timed through the wasm build (M7): one line per
//! deal, "index ms", then the median, the 95th percentile and the maximum.
//!
//!   cargo run --release -p strohmandeln-wasm --example evidencebench -- [records file]

use std::time::Instant;

fn main() {
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "evidence-records.txt".to_string());
    let text = std::fs::read_to_string(&path).expect("the records file");
    let mut ms = Vec::new();
    for (i, rec) in text.lines().filter(|l| !l.trim().is_empty()).enumerate() {
        let input = format!("{rec}\n==\n");
        let t = Instant::now();
        let v = strohmandeln_wasm::tutor::evidence_of(&input);
        let dt = t.elapsed().as_secs_f64() * 1e3;
        assert!(v["summary"].is_string(), "{v}");
        println!("{i} {dt:.3}");
        ms.push(dt);
    }
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| ms[((ms.len() - 1) as f64 * p).round() as usize];
    println!(
        "deals {} median {:.1} ms p95 {:.1} ms max {:.1} ms",
        ms.len(),
        q(0.5),
        q(0.95),
        ms[ms.len() - 1]
    );
}
