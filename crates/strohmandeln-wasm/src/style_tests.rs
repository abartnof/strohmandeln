//! The opponent's style on the wire (`docs/PROTOCOL.md`, `docs/STYLES.md`):
//! `new` takes it, state and export carry it, import checks it, and the
//! drawn style of a Random sitting is not shown before the end.

use super::*;
use strohmandeln_ai::style::Style;

fn host(config: &str) -> Host {
    let mut h = Host::default();
    assert_eq!(h.new_sitting(config, default_opponent), 1, "{config}");
    h
}

fn out(h: &Host) -> Value {
    serde_json::from_slice(h.out()).unwrap()
}

/// Plays to the end (the human takes the first legal action), calling `f`
/// with every reply's text.
fn play_out(h: &mut Host, mut f: impl FnMut(&str)) {
    for _ in 0..20_000 {
        f(&String::from_utf8_lossy(h.out()));
        let s = out(h);
        let s = if s["state"].is_null() {
            Value::Null
        } else {
            s["state"].clone()
        };
        match s["prompt"].as_str().unwrap_or("") {
            "over" => return,
            "opponent" => {
                h.step();
            }
            _ => {
                let a = s["legal"][0].clone();
                let a = if a["action"] == "extend" {
                    json!({"action":"extend","accept":false})
                } else {
                    a
                };
                assert_eq!(h.send(&a.to_string()), 1, "{a}");
            }
        }
    }
    panic!("did not finish");
}

fn export(h: &mut Host) -> Value {
    h.show(|t| t.export());
    out(h)
}

#[test]
fn a_sitting_without_a_style_is_normal() {
    let h = host(r#"{"seed":7,"touren":3}"#);
    let s = &out(&h)["state"];
    assert_eq!(s["style"], "normal");
    assert_eq!(s["drawn"], Value::Null);
}

#[test]
fn new_takes_a_style_and_state_and_export_carry_it() {
    for id in [
        "normal", "bermann", "ulmann", "ms", "werner", "bold", "cautious", "hunter",
    ] {
        let mut h = host(&format!(
            r#"{{"seed":7,"touren":3,"level":2,"style":"{id}"}}"#
        ));
        assert_eq!(out(&h)["state"]["style"], id);
        assert_eq!(out(&h)["state"]["drawn"], Value::Null);
        let e = export(&mut h);
        assert_eq!(e["style"], id);
        assert_eq!(e["drawn"], Value::Null);
    }
}

#[test]
fn an_unknown_style_is_a_bad_config() {
    for bad in [r#""Normal""#, r#""""#, r#""bermann2""#, "7", "[]"] {
        let mut h = Host::default();
        let config = format!(r#"{{"seed":7,"touren":3,"style":{bad}}}"#);
        assert_eq!(h.new_sitting(&config, default_opponent), 0, "{bad}");
        let r = out(&h);
        assert_eq!(r["error"], "bad-config", "{bad}");
        assert_eq!(r["field"], "style", "{bad}");
    }
}

#[test]
fn random_is_drawn_at_the_start_and_authoritative_in_the_export() {
    for seed in [3u64, 4, 5, 6, 7, 8] {
        let mut h = host(&format!(r#"{{"seed":{seed},"touren":3,"style":"random"}}"#));
        assert_eq!(out(&h)["state"]["style"], "random");
        assert_eq!(
            out(&h)["state"]["drawn"],
            Value::Null,
            "hidden until the end"
        );
        let e = export(&mut h);
        assert_eq!(e["style"], "random");
        assert_eq!(e["drawn"], Style::drawn(seed).id(), "seed {seed}");
    }
}

#[test]
fn the_drawn_style_is_shown_only_when_the_sitting_is_over() {
    let seed = 9u64;
    let drawn = Style::drawn(seed).id();
    let mut h = host(&format!(
        r#"{{"seed":{seed},"touren":3,"level":1,"style":"random"}}"#
    ));
    let mut replies = Vec::new();
    play_out(&mut h, |r| replies.push(r.to_string()));
    let last = replies.len() - 1;
    for (i, r) in replies.iter().enumerate() {
        if i < last {
            assert!(
                !r.contains(&format!("\"{drawn}\"")),
                "reply {i} names {drawn}: {r}"
            );
        }
    }
    h.render();
    let s = out(&h)["state"].clone();
    assert_eq!(s["prompt"], "over");
    assert_eq!(s["drawn"], drawn);
    assert_eq!(s["style"], "random");
}

#[test]
fn export_then_import_keeps_the_style_and_the_drawn_style() {
    let mut h = host(r#"{"seed":11,"touren":3,"level":2,"style":"random"}"#);
    for _ in 0..6 {
        match out(&h)["state"]["prompt"].as_str().unwrap() {
            "opponent" => {
                h.step();
            }
            _ => {
                let a = out(&h)["state"]["legal"][0].clone();
                h.send(&a.to_string());
            }
        }
    }
    let saved = export(&mut h);
    let mut g = Host::default();
    assert_eq!(g.import(&saved.to_string(), default_opponent), 1);
    assert_eq!(out(&g)["state"]["style"], "random");
    assert_eq!(export(&mut g)["drawn"], saved["drawn"]);
    // The record's drawn is authoritative: another member of the pool is kept.
    let mut other = saved.clone();
    let wrong = Style::POOL
        .iter()
        .find(|s| s.id() != saved["drawn"].as_str().unwrap())
        .unwrap();
    other["drawn"] = json!(wrong.id());
    assert_eq!(g.import(&other.to_string(), default_opponent), 1);
    assert_eq!(export(&mut g)["drawn"], wrong.id());
    // Absent, it is the hash of the seed.
    let mut none = saved.clone();
    none.as_object_mut().unwrap().remove("drawn");
    assert_eq!(g.import(&none.to_string(), default_opponent), 1);
    assert_eq!(export(&mut g)["drawn"], Style::drawn(11).id());
}

#[test]
fn an_old_record_has_no_style_and_is_normal() {
    let mut h = host(r#"{"seed":7,"touren":3}"#);
    let mut saved = export(&mut h);
    let m = saved.as_object_mut().unwrap();
    m.remove("style");
    m.remove("drawn");
    let mut g = Host::default();
    assert_eq!(g.import(&saved.to_string(), default_opponent), 1);
    assert_eq!(out(&g)["state"]["style"], "normal");
}

#[test]
fn an_import_with_a_bad_style_or_drawn_is_a_bad_record() {
    let mut h = host(r#"{"seed":7,"touren":3,"style":"random"}"#);
    let good = export(&mut h);
    let mut normal = host(r#"{"seed":7,"touren":3}"#);
    let normal = export(&mut normal);
    let cases: Vec<(Value, &str)> = vec![
        (with(&good, "style", json!("nope")), "style"),
        (with(&good, "style", json!(3)), "style"),
        (with(&good, "drawn", json!("normal")), "drawn"),
        (with(&good, "drawn", json!("random")), "drawn"),
        (with(&good, "drawn", json!("zzz")), "drawn"),
        (with(&good, "drawn", json!(5)), "drawn"),
        (with(&normal, "drawn", json!("bermann")), "drawn"),
    ];
    for (saved, field) in cases {
        let mut g = host(CONFIG);
        assert_eq!(g.import(&saved.to_string(), default_opponent), 0, "{saved}");
        let r = out(&g);
        assert_eq!(r["error"], "bad-record", "{saved}");
        assert_eq!(r["field"], field, "{saved}");
        // The old sitting stays.
        g.render();
        assert_eq!(out(&g)["ok"], true);
    }
}

const CONFIG: &str = r#"{"seed":7,"touren":3,"human":0}"#;

fn with(v: &Value, key: &str, to: Value) -> Value {
    let mut v = v.clone();
    v[key] = to;
    v
}

#[test]
fn a_styled_opponent_plays_differently_from_normal() {
    let run = |style: &str, seed: u64| {
        let mut h = host(&format!(
            r#"{{"seed":{seed},"touren":3,"level":2,"style":"{style}"}}"#
        ));
        let mut all = String::new();
        play_out(&mut h, |r| {
            all.push_str(&r.replace(&format!("\"style\":\"{style}\""), ""))
        });
        all
    };
    let differs = (1..=4u64).any(|seed| run("bermann", seed) != run("normal", seed));
    assert!(
        differs,
        "Bermann never played unlike Normal over four seeds"
    );
}

#[test]
fn normal_at_level_two_is_the_default_opponent_still() {
    // The style is applied at the factory: Normal leaves every reply as it was.
    let a = {
        let mut h = host(r#"{"seed":5,"touren":3,"level":2}"#);
        let mut all = String::new();
        play_out(&mut h, |r| all.push_str(r));
        all
    };
    let b = {
        let mut h = host(r#"{"seed":5,"touren":3,"level":2,"style":"normal"}"#);
        let mut all = String::new();
        play_out(&mut h, |r| all.push_str(r));
        all
    };
    assert_eq!(a, b);
}
