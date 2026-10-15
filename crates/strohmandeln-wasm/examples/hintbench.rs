//! Times the tutor's hint natively on the positions `tests/hintbench.mjs`
//! times through the wasm build (same sittings, the human taking each hint,
//! the opponent at level 3): one line per hint, "sitting move kind ms".
//!
//!   cargo run --release -p strohmandeln-wasm --example hintbench -- [sittings]

use serde_json::{json, Value};
use std::time::Instant;
use strohmandeln_wasm::{default_opponent, Host};

fn out(h: &Host) -> Value {
    serde_json::from_slice(h.out()).unwrap()
}

fn main() {
    let sittings: u64 = std::env::args()
        .nth(1)
        .map_or(4, |s| s.parse().expect("sittings"));
    for seed in 0..sittings {
        let mut h = Host::default();
        let config = format!(
            r#"{{"seed":{seed},"touren":3,"human":{},"level":3}}"#,
            seed % 2
        );
        assert_eq!(h.new_sitting(&config, default_opponent), 1);
        let mut r = out(&h);
        let mut move_no = 0;
        while r["state"]["prompt"] != "over" {
            if r["state"]["prompt"] == "opponent" {
                h.step();
            } else if r["state"]["phase"] == "extend" {
                assert_eq!(h.send(r#"{"action":"extend","accept":false}"#), 1);
            } else {
                let t = Instant::now();
                h.show(|t| t.hint());
                let ms = t.elapsed().as_secs_f64() * 1e3;
                let hint = out(&h);
                let play = hint["hint"]["action"] == "play";
                let kind = if !play {
                    "other"
                } else if hint["reason"].as_str().unwrap().starts_with("lead-") {
                    "lead"
                } else {
                    "follow"
                };
                if r["state"]["legal"].as_array().unwrap().len() > 1 {
                    println!("{seed} {move_no} {kind} {ms:.3}");
                }
                let a = if play {
                    json!({"action": "play", "card": hint["hint"]["card"]}).to_string()
                } else {
                    hint["hint"].to_string()
                };
                assert_eq!(h.send(&a), 1);
            }
            r = out(&h);
            move_no += 1;
        }
    }
}
