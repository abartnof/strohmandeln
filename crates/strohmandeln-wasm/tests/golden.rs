//! Golden games: fixed seeds played by a fixed policy must produce exactly the
//! recorded stream of replies, byte for byte, so every build of the engine and
//! of the module plays alike. `tests/smoke.mjs` replays the same file through
//! the built .wasm. Also: the hidden-card audit over whole random sittings.
//!
//! Policy: the human takes the first legal action, except that an extension is
//! declined; the opponent is `default_opponent`. The hash is FNV-1a over the
//! raw bytes of every reply, in order (the opening reply, then each send or
//! step). To re-record after an intended change:
//!   UPDATE_GOLDEN=1 cargo test -p strohmandeln-wasm --test golden

use serde_json::Value;
use strohmandeln_wasm::{default_opponent, fnv, Host};

const GOLDEN: &str = include_str!("golden.txt");

fn out(h: &Host) -> Value {
    serde_json::from_slice(h.out()).unwrap()
}

/// Plays a sitting by the policy; returns (hash, number of events, final contos, balance).
fn play(seed: u64, touren: u32, human: u8) -> (u64, usize, Value, i64) {
    let mut h = Host::default();
    let config = format!(r#"{{"seed":{seed},"touren":{touren},"human":{human},"level":1}}"#);
    assert_eq!(h.new_sitting(&config, default_opponent), 1);
    let mut all: Vec<u8> = h.out().to_vec();
    let mut events = out(&h)["events"].as_array().unwrap().len();
    for _ in 0..20_000 {
        let r = out(&h);
        let s = &r["state"];
        match s["prompt"].as_str().unwrap() {
            "over" => {
                return (
                    fnv(&all),
                    events,
                    s["contos"].clone(),
                    s["balance"].as_i64().unwrap(),
                )
            }
            "opponent" => assert_eq!(h.step(), 1),
            _ => {
                let legal = s["legal"].as_array().unwrap();
                let a = if s["phase"] == "extend" {
                    r#"{"action":"extend","accept":false}"#.to_string()
                } else {
                    // The wire form of a play: the card only.
                    let a = &legal[0];
                    if a["action"] == "play" {
                        format!(r#"{{"action":"play","card":{}}}"#, a["card"])
                    } else {
                        a.to_string()
                    }
                };
                assert_eq!(h.send(&a), 1, "{a}: {}", String::from_utf8_lossy(h.out()));
            }
        }
        all.extend_from_slice(h.out());
        events += out(&h)["events"].as_array().unwrap().len();
    }
    panic!("seed {seed} did not finish");
}

#[test]
fn golden_games_replay_byte_for_byte() {
    let lines: Vec<&str> = GOLDEN
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
        .collect();
    let mut recorded = String::from(
        "# Golden sittings: seed touren human, then the hash of every reply, the events\n\
         # sent, the final Contos and the balance (see tests/golden.rs).\n",
    );
    let mut bad = Vec::new();
    for (seed, touren, human) in [
        (1u64, 3u32, 0u8),
        (2, 3, 1),
        (3, 6, 0),
        (17, 3, 0),
        (99, 6, 1),
        (2026, 3, 1),
    ] {
        let (hash, events, contos, balance) = play(seed, touren, human);
        let line = format!("{seed} {touren} {human} {hash} {events} {contos} {balance}");
        recorded.push_str(&line);
        recorded.push('\n');
        if !lines.contains(&line.as_str()) {
            bad.push(line);
        }
    }
    if std::env::var_os("UPDATE_GOLDEN").is_some() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden.txt");
        std::fs::write(path, recorded).unwrap();
        return;
    }
    assert!(
        bad.is_empty(),
        "these sittings no longer play as recorded:\n{}",
        bad.join("\n")
    );
}

/// The cards the human must not learn never appear in what the module sends.
/// Events are checked from the last deal_start on, since an earlier deal's
/// cards are shuffled back in.
#[test]
fn no_hidden_card_ever_reaches_the_human() {
    for (seed, human) in [(5u64, 0u8), (6, 1), (7, 0), (8, 1)] {
        let mut h = Host::default();
        let config = format!(r#"{{"seed":{seed},"touren":3,"human":{human}}}"#);
        assert_eq!(h.new_sitting(&config, default_opponent), 1);
        let mut n = 0usize;
        let mut checked = 0;
        loop {
            let r = out(&h);
            let hidden = audit(&mut h);
            let events = r["events"].as_array().unwrap();
            let from = events
                .iter()
                .rposition(|e| e["type"] == "deal_start")
                .unwrap_or(0);
            let seen = format!("{}{}", r["state"], Value::Array(events[from..].to_vec()));
            for code in &hidden {
                assert!(
                    !seen.contains(&format!("\"{code}\"")),
                    "seed {seed}: hidden card {code} leaked in {seen}"
                );
                checked += 1;
            }
            match r["state"]["prompt"].as_str().unwrap() {
                "over" => break,
                "opponent" => {
                    h.step();
                }
                _ => {
                    let legal = r["state"]["legal"].as_array().unwrap();
                    let a = &legal[n % legal.len()];
                    n += 7;
                    let a = if a["action"] == "play" {
                        format!(r#"{{"action":"play","card":{}}}"#, a["card"])
                    } else {
                        a.to_string()
                    };
                    assert_eq!(h.send(&a), 1);
                }
            }
        }
        assert!(checked > 100, "the audit looked at something");
    }
}

fn audit(h: &mut Host) -> Vec<String> {
    h.show(|t| t.audit());
    let v = out(h);
    v["hidden"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c.as_str().unwrap().to_string())
        .collect()
}
