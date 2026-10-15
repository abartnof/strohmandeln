//! What the table keeps of a scored deal for the sitting's brief, as JSON
//! (`docs/PROTOCOL.md` "The tutor"): the engine's [`Detail`] and back. The
//! text comes from storage, so nothing in it is trusted: a field that is not
//! as expected is an error, never a panic.

use kit_learner::Summary;
use serde_json::{json, Map, Value};
use strohmandeln_ai::brief::Detail;
use strohmandeln_ai::tutor::{Consequence, Direction, Facts, Miss, Skill, LADDER, SKILLS};
use strohmandeln_core::cards::{Card, Honour};
use strohmandeln_core::types::Settings;

const DIRECTIONS: [(Direction, &str); 8] = [
    (Direction::Greedy, "greedy"),
    (Direction::Timid, "timid"),
    (Direction::Figure, "figure"),
    (Direction::Pip, "pip"),
    (Direction::Cash, "cash"),
    (Direction::Keep, "keep"),
    (Direction::Passed, "passed"),
    (Direction::Took, "took"),
];

fn honour_name(h: Honour) -> &'static str {
    match h {
        Honour::Trull => "trull",
        Honour::Kings => "kings",
    }
}

fn card(c: Option<Card>) -> Value {
    c.map_or(Value::Null, |c| json!(c.code()))
}

fn miss_json(m: &Miss) -> Value {
    let direction = m
        .direction
        .and_then(|d| DIRECTIONS.iter().find(|(x, _)| *x == d))
        .map(|(_, n)| *n);
    let consequence = m
        .consequence
        .map(|Consequence::FellUnder { by, trick }| json!({"by": by.code(), "trick": trick}));
    json!({
        "action": m.action,
        "trick": m.trick,
        "played": card(m.played),
        "better": card(m.better),
        "cost": m.cost,
        "direction": direction,
        "led": card(m.led),
        "trick_thirds": m.trick_thirds,
        "tarocks": m.tarocks,
        "sure_thirds": m.sure_thirds,
        "consequence": consequence,
    })
}

/// A detail as JSON: the summary text, the counts, the facts, the costliest
/// miss at each skill that has one, the settings and the variety.
pub fn to_json(d: &Detail) -> Value {
    let misses: Map<String, Value> = d
        .misses
        .iter()
        .flatten()
        .map(|m| (m.skill.slug().to_string(), miss_json(m)))
        .collect();
    json!({
        "summary": d.summary.to_text(&LADDER),
        "decisions": d.decisions,
        "facts": {
            "ultimo_won": d.facts.ultimo_won,
            "mond_caught": d.facts.mond_caught,
            "announced": d.facts.announced.iter().map(|h| honour_name(*h)).collect::<Vec<_>>(),
        },
        "misses": misses,
        "settings": d.settings,
        "variety": d.variety,
    })
}

fn uint(v: &Value, name: &str) -> Result<u32, String> {
    v.get(name)
        .and_then(Value::as_u64)
        .and_then(|n| u32::try_from(n).ok())
        .ok_or_else(|| name.to_string())
}

fn opt_card(v: &Value, name: &str) -> Result<Option<Card>, String> {
    match v.get(name) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Card::parse(s).map(Some).ok_or_else(|| name.to_string()),
        _ => Err(name.to_string()),
    }
}

fn miss_of(skill: Skill, v: &Value) -> Result<Miss, String> {
    let direction = match v.get("direction") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => Some(
            DIRECTIONS
                .iter()
                .find(|(_, n)| n == s)
                .map(|(d, _)| *d)
                .ok_or("direction")?,
        ),
        _ => return Err("direction".into()),
    };
    let consequence = match v.get("consequence") {
        None | Some(Value::Null) => None,
        Some(c) => Some(Consequence::FellUnder {
            by: opt_card(c, "by")?.ok_or("by")?,
            trick: uint(c, "trick")?,
        }),
    };
    let cost = v.get("cost").and_then(Value::as_f64).ok_or("cost")?;
    if !cost.is_finite() || cost < 0.0 {
        return Err("cost".into());
    }
    Ok(Miss {
        skill,
        action: v.get("action").and_then(Value::as_u64).ok_or("action")? as usize,
        trick: uint(v, "trick")?,
        played: opt_card(v, "played")?,
        better: opt_card(v, "better")?,
        cost,
        direction,
        led: opt_card(v, "led")?,
        trick_thirds: uint(v, "trick_thirds")?,
        tarocks: uint(v, "tarocks")?,
        sure_thirds: uint(v, "sure_thirds")?,
        consequence,
    })
}

/// A detail from its JSON; `Err` names the field that was not as expected.
pub fn from_json(v: &Value) -> Result<Detail, String> {
    let summary = v
        .get("summary")
        .and_then(Value::as_str)
        .ok_or("summary")
        .and_then(|t| Summary::parse(&LADDER, t).map_err(|_| "summary"))?;
    if !summary.is_current(&LADDER) {
        return Err("summary".into());
    }
    let f = v.get("facts").ok_or("facts")?;
    let mut announced = Vec::new();
    for h in f
        .get("announced")
        .and_then(Value::as_array)
        .ok_or("announced")?
    {
        announced.push(match h.as_str() {
            Some("trull") => Honour::Trull,
            Some("kings") => Honour::Kings,
            _ => return Err("announced".into()),
        });
    }
    let mond_caught = match f.get("mond_caught") {
        None | Some(Value::Null) => None,
        Some(_) => Some(uint(f, "mond_caught")?),
    };
    let facts = Facts {
        ultimo_won: f
            .get("ultimo_won")
            .and_then(Value::as_bool)
            .ok_or("ultimo_won")?,
        mond_caught,
        announced,
    };
    let mut misses: [Option<Miss>; SKILLS] = Default::default();
    for (slug, m) in v.get("misses").and_then(Value::as_object).ok_or("misses")? {
        let skill = Skill::ALL
            .into_iter()
            .find(|s| s.slug() == slug)
            .ok_or("miss skill")?;
        misses[skill.index()] = Some(miss_of(skill, m)?);
    }
    let settings: Settings = serde_json::from_value(v.get("settings").cloned().ok_or("settings")?)
        .map_err(|_| "settings")?;
    Ok(Detail {
        summary,
        decisions: uint(v, "decisions")?,
        facts,
        misses,
        settings,
        variety: uint(v, "variety")? as usize,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Detail {
        let mut misses: [Option<Miss>; SKILLS] = Default::default();
        misses[Skill::Off.index()] = Some(Miss {
            skill: Skill::Off,
            action: 7,
            trick: 14,
            played: Card::parse("4h"),
            better: Card::parse("Qh"),
            cost: 18.5,
            direction: Some(Direction::Figure),
            led: None,
            trick_thirds: 0,
            tarocks: 5,
            sure_thirds: 0,
            consequence: Card::parse("Kh").map(|by| Consequence::FellUnder { by, trick: 22 }),
        });
        misses[Skill::Bid.index()] = Some(Miss {
            skill: Skill::Bid,
            action: 0,
            trick: 0,
            played: None,
            better: None,
            cost: 24.0,
            direction: Some(Direction::Passed),
            led: None,
            trick_thirds: 0,
            tarocks: 7,
            sure_thirds: 61,
            consequence: None,
        });
        Detail {
            summary: Summary::new(&LADDER),
            decisions: 14,
            facts: Facts {
                ultimo_won: true,
                mond_caught: Some(9),
                announced: vec![Honour::Trull, Honour::Kings],
            },
            misses,
            settings: Settings::default(),
            variety: 17,
        }
    }

    #[test]
    fn a_detail_goes_to_json_and_back() {
        let d = sample();
        assert_eq!(from_json(&to_json(&d)).unwrap(), d);
        let text = to_json(&d).to_string();
        assert!(text.len() < 1500, "{} bytes", text.len());
        assert_eq!(from_json(&serde_json::from_str(&text).unwrap()).unwrap(), d);
    }

    #[test]
    fn a_damaged_detail_is_an_error_not_a_panic() {
        let good = to_json(&sample());
        for (path, bad) in [
            ("summary", json!("junk")),
            ("summary", json!(3)),
            ("decisions", json!(-1)),
            ("decisions", json!("x")),
            ("facts", json!(null)),
            (
                "facts",
                json!({"ultimo_won": 1, "announced": [], "mond_caught": null}),
            ),
            (
                "facts",
                json!({"ultimo_won": true, "announced": ["pagat"], "mond_caught": null}),
            ),
            ("misses", json!([])),
            ("misses", json!({"nonsense": {}})),
            ("misses", json!({"worth": {"cost": -1}})),
            (
                "misses",
                json!({"worth": {"cost": 1, "direction": "up", "action": 0, "trick": 0, "trick_thirds": 0, "tarocks": 0, "sure_thirds": 0}}),
            ),
            ("settings", json!({})),
            ("variety", json!(1e30)),
        ] {
            let mut v = good.clone();
            v[path] = bad.clone();
            assert!(from_json(&v).is_err(), "{path} {bad}");
        }
        assert!(from_json(&json!(null)).is_err());
        assert!(from_json(&json!({})).is_err());
        // a card that is not a card
        let mut v = good;
        v["misses"]["off"]["better"] = json!("Zz");
        assert!(from_json(&v).is_err());
    }
}
