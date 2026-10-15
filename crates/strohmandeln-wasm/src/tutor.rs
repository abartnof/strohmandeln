//! The tutor's calls (`docs/PROTOCOL.md` "The tutor", `docs/TUTOR.md` 10).
//!
//! The deal record of the deal just scored comes from the sitting; the rest
//! are **stateless** (text in, JSON out), so that a Worker with an instance
//! of its own can run them: the evidence of a record, the learner over
//! stored summaries, the brief of a record or of a sitting. The nudge asks
//! the sitting about the position now. The engine holds the judgement and
//! the keys; the page holds the words.

use super::detail;
use super::{error, Table, PROTOCOL};
use kit_learner::{Bullet, Param, Summary};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::sync::OnceLock;
use strohmandeln_ai::brief::{
    brief_of_deal, brief_of_sitting, detail_of, Brief, Detail, SittingDeal,
};
use strohmandeln_ai::lessons::lesson;
use strohmandeln_ai::rung4::Budget;
use strohmandeln_ai::tutor::{self, Skill, EVIDENCE_VERSION, LADDER, PARAMS};
use strohmandeln_core::dealrec::DealRecord;
use strohmandeln_core::types::Seat;

/// The line between a deal record and its assisted numbers.
const RECORD_END: &str = "==";
/// The line between the deals of a sitting.
const DEAL_SEPARATOR: &str = "++";
/// The line between summaries, and between the deals and the history.
pub const SUMMARY_SEPARATOR: &str = "--";

/// What the table stores of a scored deal: the human's seat and the record.
#[derive(Serialize, Deserialize)]
struct Stored {
    v: u32,
    human: Seat,
    deal: DealRecord,
}

/// The text of a record, which the table stores.
pub fn record_text(human: Seat, deal: &DealRecord) -> String {
    json!({"v": 1, "human": human, "deal": deal}).to_string()
}

fn bad(why: &str) -> Value {
    error("bad-input", json!({"why": why}))
}

/// The ladder and the learner's constants, checked once when the tutor is
/// first used, in the release module too (`learn` asserts them in debug
/// builds only). An error value if they cannot work: nothing then judges.
/// That they work says nothing of whether the focus is right: it is
/// unvalidated until M6 (`docs/TUTOR.md` 5).
fn constants() -> Result<(), Value> {
    static CHECKED: OnceLock<Result<(), String>> = OnceLock::new();
    CHECKED
        .get_or_init(|| LADDER.check().and_then(|()| PARAMS.check()))
        .clone()
        .map_err(|why| error("bad-constants", json!({"why": why})))
}

/// A deal and what is known of it: its record, the human's seat, the
/// actions a hint or a nudge was shown for and the number the table gives it.
struct DealIn {
    human: Seat,
    record: DealRecord,
    assisted: Vec<usize>,
}

/// The record, `==`, the assisted numbers.
fn parse_deal(section: &str) -> Result<DealIn, Value> {
    let mut lines = section.lines().map(str::trim).skip_while(|l| l.is_empty());
    let mut record = String::new();
    let mut numbers = String::new();
    let mut after = false;
    for line in lines.by_ref() {
        if after {
            numbers.push_str(line);
            numbers.push(' ');
        } else if line == RECORD_END {
            after = true;
        } else {
            record.push_str(line);
        }
    }
    if !after {
        return Err(bad("no line =="));
    }
    let stored: Stored = serde_json::from_str(&record).map_err(|_| bad("record"))?;
    if stored.v != 1 {
        return Err(bad("record version"));
    }
    let assisted = numbers
        .split_whitespace()
        .map(|n| n.parse::<usize>().map_err(|_| bad("assisted numbers")))
        .collect::<Result<_, _>>()?;
    Ok(DealIn {
        human: stored.human,
        record: stored.deal,
        assisted,
    })
}

/// Summaries from their text, oldest first, separated by lines of `--`. An
/// entry that is empty or does not parse is `None`: it counts as stale.
pub fn summaries(text: &str) -> Vec<Option<Summary>> {
    if text.trim().is_empty() {
        return Vec::new();
    }
    let mut out = vec![String::new()];
    for line in text.lines() {
        if line.trim() == SUMMARY_SEPARATOR {
            out.push(String::new());
        } else if let Some(last) = out.last_mut() {
            last.push_str(line);
            last.push('\n');
        }
    }
    out.iter()
        .map(|t| Summary::parse(&LADDER, t).ok())
        .collect()
}

/// Splits `text` at its first line `--`: before it, and after it.
fn split_history(text: &str) -> (String, String) {
    let mut before = Vec::new();
    let mut lines = text.lines();
    for line in lines.by_ref() {
        if line.trim() == SUMMARY_SEPARATOR {
            break;
        }
        before.push(line);
    }
    (before.join("\n"), lines.collect::<Vec<_>>().join("\n"))
}

/// The opponent's level from the first line, `level <n>`, 1 to 4, and the
/// rest.
fn parse_level(text: &str) -> Result<(f64, &str), Value> {
    let (first, rest) = text.split_once('\n').unwrap_or((text, ""));
    first
        .trim()
        .strip_prefix("level ")
        .and_then(|n| n.trim().parse::<f64>().ok())
        .filter(|n| (1.0..=4.0).contains(n))
        .map(|n| (n, rest))
        .ok_or_else(|| bad("level"))
}

fn param_json(p: &Param) -> Value {
    match p {
        Param::Int(n) => json!(n),
        Param::Num(n) => json!(n),
        Param::Text(s) => json!(s),
        Param::Skill(i) => json!(LADDER.skills[*i].slug),
    }
}

fn bullet_json(b: &Bullet) -> Value {
    let params: Map<String, Value> = b
        .params
        .iter()
        .map(|(k, v)| (k.clone(), param_json(v)))
        .collect();
    json!({"lead": b.lead, "key": b.key, "params": params})
}

fn brief_json(b: &Brief) -> Value {
    json!({
        "bullets": b.bullets.iter().map(bullet_json).collect::<Vec<_>>(),
        "method": b.method,
        "summary": b.summary.to_text(&LADDER),
        "detail": b.detail.as_ref().map(detail::to_json),
        "focus": b.focus.map(Skill::slug),
    })
}

fn replay_error(e: &impl Serialize) -> Value {
    error("replay", json!({"why": e}))
}

/// `strohmandeln_evidence_of`: a record, a line `==`, the assisted numbers.
/// `{summary, detail}` (the text the learner reads, and what the sitting's
/// brief is made from) or an error. The advisor runs at every unassisted
/// decision with a choice.
pub fn evidence_of(input: &str) -> Value {
    if let Err(e) = constants() {
        return e;
    }
    let d = match parse_deal(input) {
        Ok(d) => d,
        Err(e) => return e,
    };
    match detail_of(&d.record, d.human, &d.assisted, Budget::default()) {
        Ok(x) => json!({
            "protocol": PROTOCOL,
            "summary": x.summary.to_text(&LADDER),
            "detail": detail::to_json(&x),
        }),
        Err(e) => replay_error(&e),
    }
}

/// `strohmandeln_learner`: the learner over summaries, oldest first.
pub fn learner_of(input: &str) -> Value {
    if let Err(e) = constants() {
        return e;
    }
    let all = summaries(input);
    let current: Vec<Summary> = all.iter().flatten().cloned().collect();
    let stale_at: Vec<usize> = all
        .iter()
        .enumerate()
        .filter(|(_, s)| s.as_ref().is_none_or(|s| !s.is_current(&LADDER)))
        .map(|(i, _)| i)
        .collect();
    let learnt = tutor::learner(&current);
    let slugs = |list: &mut dyn Iterator<Item = usize>| -> Vec<&'static str> {
        list.map(|i| LADDER.skills[i].slug).collect()
    };
    json!({
        "protocol": PROTOCOL,
        "deals": learnt.games,
        "focus": learnt.focus.map(|i| LADDER.skills[i].slug),
        "mastered": slugs(&mut (0..LADDER.skills.len()).filter(|&i| learnt.mastered(i))),
        "just_mastered": slugs(&mut learnt.just_mastered.iter().copied()),
        "stale": stale_at,
        "version": EVIDENCE_VERSION,
    })
}

fn brief_reply(r: Result<Brief, impl Serialize>) -> Value {
    match r {
        Ok(b) => json!({"protocol": PROTOCOL, "brief": brief_json(&b)}),
        Err(e) => {
            let mut v = replay_error(&e);
            if let Value::Object(m) = &mut v {
                m.insert("brief".into(), Value::Null);
            }
            v
        }
    }
}

fn history_of(text: &str) -> Vec<Summary> {
    summaries(text).into_iter().flatten().collect()
}

/// `strohmandeln_brief_of_record`: `level <n>`, a record, `==`, the assisted
/// numbers, a line `--`, the history. One advisor pass serves the evidence,
/// the misses and the brief. `{brief: {bullets, method, summary, focus}}`;
/// `brief` is `null` with an error for a deal that did not restore or end.
pub fn brief_of_record(input: &str) -> Value {
    if let Err(e) = constants() {
        return e;
    }
    let (level, rest) = match parse_level(input) {
        Ok(x) => x,
        Err(e) => return e,
    };
    let (deal, history) = split_history(rest);
    let d = match parse_deal(&deal) {
        Ok(d) => d,
        Err(e) => return e,
    };
    brief_reply(brief_of_deal(
        &d.record,
        d.human,
        &d.assisted,
        &history_of(&history),
        level,
        Budget::default(),
    ))
}

/// A sitting's deal: `deal N` (1 or more, the number the table shows), then
/// the deal's detail as one line of JSON, as `evidence_of` gave it.
fn parse_kept(section: &str) -> Result<(u32, Detail), Value> {
    let mut lines = section.lines().map(str::trim).filter(|l| !l.is_empty());
    let number = lines
        .next()
        .and_then(|l| l.strip_prefix("deal "))
        .and_then(|n| n.trim().parse::<u32>().ok())
        .filter(|&n| n >= 1)
        .ok_or_else(|| bad("deal number"))?;
    let text = lines.next().ok_or_else(|| bad("detail"))?;
    if lines.next().is_some() {
        return Err(bad("detail"));
    }
    let value: Value = serde_json::from_str(text).map_err(|_| bad("detail"))?;
    let detail = detail::from_json(&value).map_err(|field| bad(&format!("detail {field}")))?;
    Ok((number, detail))
}

/// `strohmandeln_brief_sitting`: `level <n>`, the sitting's deals each as
/// `deal N` and the detail `evidence_of` gave (the deals separated by a line
/// `++`), a line `--`, the history before the sitting. The advisor does not
/// run: the sitting's brief is made from what was kept when each deal was
/// scored.
pub fn brief_sitting(input: &str) -> Value {
    if let Err(e) = constants() {
        return e;
    }
    let (level, rest) = match parse_level(input) {
        Ok(x) => x,
        Err(e) => return e,
    };
    let (deals, history) = split_history(rest);
    let mut kept = Vec::new();
    let mut section = String::new();
    for line in deals.lines().chain(std::iter::once(DEAL_SEPARATOR)) {
        if line.trim() == DEAL_SEPARATOR {
            if !section.trim().is_empty() {
                match parse_kept(&section) {
                    Ok(k) => kept.push(k),
                    Err(e) => return e,
                }
            }
            section.clear();
        } else {
            section.push_str(line);
            section.push('\n');
        }
    }
    let deals: Vec<SittingDeal> = kept
        .iter()
        .map(|(number, detail)| SittingDeal {
            number: *number,
            detail,
        })
        .collect();
    match brief_of_sitting(&deals, &history_of(&history), level) {
        Some(b) => json!({"protocol": PROTOCOL, "brief": brief_json(&b)}),
        None => bad("no deals"),
    }
}

/// A slug that names no skill.
#[derive(Debug, PartialEq, Eq)]
pub struct UnknownSkill;

/// The skill named by `slug`, or `None` for the empty slug; `Err` for one
/// that is not a skill.
pub fn skill_of(slug: &str) -> Result<Option<Skill>, UnknownSkill> {
    let slug = slug.trim();
    if slug.is_empty() {
        return Ok(None);
    }
    Skill::ALL
        .into_iter()
        .find(|s| s.slug() == slug)
        .map(Some)
        .ok_or(UnknownSkill)
}

impl Table {
    /// The record of the deal that last ended, as the table stores it
    /// (`record: null` before one has). The table asks right after the
    /// `game_scored` event, before another deal can end.
    pub fn deal_record(&self) -> Value {
        let record = self.sitting.last_deal().map(|d| record_text(self.human, d));
        json!({"protocol": PROTOCOL, "record": record})
    }

    /// Whether the position now is a clear chance at the focus: `{nudge:
    /// {skill, key}}` or `nudge: null`. One advisor run; a function of the
    /// position and the focus. `action` is the number the decision has in
    /// the deal (`state.deal_actions`), which the table adds to its assisted
    /// list when it shows the nudge.
    pub fn nudge(&self, focus: Option<Skill>) -> Value {
        let nudge = focus
            .filter(|_| !self.legal().is_empty())
            .filter(|&skill| tutor::nudge(skill, &self.my_view(), Budget::default()))
            .map(|skill| json!({"skill": skill.slug(), "key": lesson(skill).nudge}));
        json!({
            "protocol": PROTOCOL,
            "deal_action": self.sitting.deal_action_count(),
            "nudge": nudge,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::super::{default_opponent, Host};
    use super::*;
    use strohmandeln_ai::tutor::evidence;

    const CONFIG: &str = r#"{"seed":7,"touren":3,"human":0}"#;

    fn host(config: &str) -> Host {
        let mut h = Host::default();
        assert_eq!(h.new_sitting(config, default_opponent), 1);
        h
    }

    fn out(h: &Host) -> Value {
        serde_json::from_slice(h.out()).unwrap()
    }

    /// Plays the first legal action for the human until a deal is scored
    /// (`deals_counted` reaches `n`); returns the record text at that moment.
    fn play_to_deal(h: &mut Host, n: u64) -> String {
        h.render();
        for _ in 0..5000 {
            let s = out(h)["state"].clone();
            if s["deals_counted"].as_u64().unwrap_or(0) >= n || s["prompt"] == "over" {
                break;
            }
            if s["prompt"] == "opponent" {
                assert_eq!(h.step(), 1);
                continue;
            }
            let a = s["legal"][0].clone();
            let a = match a["action"].as_str() {
                Some("extend") => json!({"action":"extend","accept":false}),
                Some("play") => json!({"action":"play","card":a["card"]}),
                _ => a,
            };
            assert_eq!(h.send(&a.to_string()), 1);
        }
        h.show(|t| t.deal_record());
        out(h)["record"].as_str().expect("a record").to_string()
    }

    /// A record, the `==` line and the assisted numbers.
    fn input(record: &str, assisted: &str) -> String {
        format!("{record}\n{RECORD_END}\n{assisted}")
    }

    #[test]
    fn there_is_no_record_before_a_deal_is_scored() {
        let mut h = host(CONFIG);
        h.show(|t| t.deal_record());
        assert_eq!(out(&h)["record"], Value::Null);
        assert_eq!(out(&h)["protocol"], PROTOCOL);
    }

    #[test]
    fn the_record_is_text_that_names_the_seat_and_replays_the_deal() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        assert!(text.len() < 4000, "{} bytes", text.len());
        let stored: Stored = serde_json::from_str(&text).unwrap();
        assert_eq!(stored.human, Seat::A);
        assert_eq!(stored.deal.deck.len(), 54);
        assert!(stored.deal.replay_to_end(|_, _, _, _| {}).is_ok());
    }

    #[test]
    fn the_evidence_of_a_record_is_the_engines_summary_and_assisted_numbers_remove_chances() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        let v = evidence_of(&input(&text, ""));
        let summary = v["summary"].as_str().expect("a summary");
        let parsed = Summary::parse(&LADDER, summary).unwrap();
        let stored: Stored = serde_json::from_str(&text).unwrap();
        let direct = evidence(&stored.deal, stored.human, &[], Budget::default()).unwrap();
        assert_eq!(parsed, direct.summary);
        assert!(summary.starts_with("strohmandeln evidence v"));
        // every action assisted: no chance is counted
        let every: Vec<String> = (0..stored.deal.actions.len())
            .map(|i| i.to_string())
            .collect();
        let v = evidence_of(&input(&text, &every.join(" ")));
        let none = Summary::parse(&LADDER, v["summary"].as_str().unwrap()).unwrap();
        assert!(none.tallies.iter().all(|t| t.chances == 0), "{none:?}");
        // a stale number out of the deal's range changes nothing
        let v = evidence_of(&input(&text, "9999"));
        assert_eq!(v["summary"].as_str().unwrap(), summary);
    }

    #[test]
    fn evidence_of_nonsense_is_an_error_not_a_panic() {
        for bad_input in [
            "",
            "nonsense",
            "nonsense\n==\n",
            "{}\n==\n",
            "{\"v\":1,\"human\":0,\"deal\":{}}\n==\n",
            "{\"v\":2,\"human\":0}\n==\n",
        ] {
            let v = evidence_of(bad_input);
            assert!(v["error"].is_string(), "{bad_input:?}: {v}");
        }
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        assert!(evidence_of(&input(&text, "x y"))["error"].is_string());
        // a deal that did not end
        let mut stored: Stored = serde_json::from_str(&text).unwrap();
        stored.deal.actions.truncate(5);
        let half = record_text(stored.human, &stored.deal);
        let v = evidence_of(&input(&half, ""));
        assert_eq!(v["error"], "replay", "{v}");
    }

    #[test]
    fn the_learner_over_nothing_names_no_focus_and_lists_the_stale() {
        let v = learner_of("");
        assert_eq!(v["deals"], 0);
        assert_eq!(v["focus"], Value::Null);
        assert_eq!(v["version"], EVIDENCE_VERSION);
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        let good = evidence_of(&input(&text, ""))["summary"]
            .as_str()
            .unwrap()
            .to_string();
        let old = good.replacen(
            &format!("evidence v{EVIDENCE_VERSION}"),
            &format!("evidence v{}", EVIDENCE_VERSION + 7),
            1,
        );
        let history = format!("{good}\n--\n{old}\n--\n\n--\ngarbage\n--\n{good}\n");
        let v = learner_of(&history);
        assert_eq!(v["deals"], 2, "{v}");
        assert_eq!(v["stale"], json!([1, 2, 3]));
        assert!(v["mastered"].is_array() && v["just_mastered"].is_array());
    }

    #[test]
    fn a_learner_over_bad_deals_names_the_skill_to_work_on() {
        let lines = "worth 6 2 60\ncheap 0 0 0\noff 0 0 0\nrun 0 0 0\npagat 0 0 0\nvoid 0 0 0\nbehind 0 0 0\nendgame 0 0 0\nbid 0 0 0\nannounce 0 0 0\n";
        let worth = format!("strohmandeln evidence v{EVIDENCE_VERSION}\n{lines}");
        let history = [worth.as_str(); 4].join("--\n");
        let v = learner_of(&history);
        assert_eq!(v["focus"], "worth", "{v}");
    }

    #[test]
    fn the_brief_of_a_record_is_up_to_three_bullets_of_keys_and_hands_back_the_summary() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        let v = brief_of_record(&format!("level 3\n{}", input(&text, "")));
        let brief = &v["brief"];
        let bullets = brief["bullets"].as_array().expect("bullets");
        assert!((1..=3).contains(&bullets.len()), "{v}");
        for b in bullets {
            assert!(b["key"].as_str().unwrap().starts_with("tutor."), "{b}");
            assert!(b["params"].is_object());
        }
        assert_eq!(brief["method"], "tutor.method");
        let ev = evidence_of(&input(&text, ""));
        assert_eq!(brief["summary"], ev["summary"], "one pass serves both");
        // the same input, the same brief
        assert_eq!(
            brief_of_record(&format!("level 3\n{}", input(&text, ""))),
            v
        );
        // a history is read; an unusable entry in it is left out
        let history = format!(
            "--\n{0}\n--\njunk\n--\n{0}",
            ev["summary"].as_str().unwrap()
        );
        let with = brief_of_record(&format!("level 3\n{}\n{history}", input(&text, "")));
        assert!(with["brief"]["bullets"].is_array(), "{with}");
    }

    #[test]
    fn a_brief_needs_the_level_and_a_deal_that_ended() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        assert!(brief_of_record(&input(&text, ""))["error"].is_string());
        assert!(brief_of_record(&format!("level x\n{}", input(&text, "")))["error"].is_string());
        let mut stored: Stored = serde_json::from_str(&text).unwrap();
        stored.deal.actions.truncate(3);
        let half = record_text(stored.human, &stored.deal);
        let v = brief_of_record(&format!("level 3\n{}", input(&half, "")));
        assert_eq!(v["brief"], Value::Null, "{v}");
        assert!(v["error"].is_string());
    }

    /// The sitting's input for the details `evidence_of` gave.
    fn sitting_input(level: &str, details: &[(u32, &Value)]) -> String {
        let deals: Vec<String> = details
            .iter()
            .map(|(n, d)| format!("deal {n}\n{d}"))
            .collect();
        format!(
            "level {level}\n{}\n--\n",
            deals.join(&format!("\n{DEAL_SEPARATOR}\n"))
        )
    }

    #[test]
    fn the_brief_of_a_sitting_is_made_of_the_kept_details_and_does_not_run_the_advisor() {
        let mut h = host(CONFIG);
        let one = play_to_deal(&mut h, 1);
        let two = play_to_deal(&mut h, 2);
        assert_ne!(one, two);
        let (ea, eb) = (
            evidence_of(&input(&one, "")),
            evidence_of(&input(&two, "3 4")),
        );
        assert!(ea["detail"].is_object(), "{ea}");
        let started = std::time::Instant::now();
        let v = brief_sitting(&sitting_input(
            "2",
            &[(1, &ea["detail"]), (2, &eb["detail"])],
        ));
        assert!(
            started.elapsed().as_millis() < 100,
            "{:?}",
            started.elapsed()
        );
        let brief = &v["brief"];
        assert!(brief["bullets"].as_array().unwrap().len() <= 3, "{v}");
        let parse = |e: &Value| Summary::parse(&LADDER, e["summary"].as_str().unwrap()).unwrap();
        let (a, b) = (parse(&ea), parse(&eb));
        let sum = Summary::parse(&LADDER, brief["summary"].as_str().unwrap()).unwrap();
        for i in 0..LADDER.skills.len() {
            assert_eq!(sum.of(i).chances, a.of(i).chances + b.of(i).chances);
            assert_eq!(sum.of(i).met, a.of(i).met + b.of(i).met);
        }
        // the details are the same as the brief of a record hands back
        let r = brief_of_record(&format!("level 2\n{}", input(&one, "")));
        assert_eq!(r["brief"]["detail"], ea["detail"]);
        assert!(
            brief_sitting("level 2\n--\n")["error"].is_string(),
            "no deals"
        );
    }

    #[test]
    fn a_sitting_deal_needs_its_number_and_a_sound_detail() {
        let mut h = host(CONFIG);
        let one = play_to_deal(&mut h, 1);
        let e = evidence_of(&input(&one, ""));
        let d = &e["detail"];
        for bad_deal in [
            format!("level 2\n{d}\n--\n"),
            format!("level 2\ndeal 0\n{d}\n--\n"),
            format!("level 2\ndeal x\n{d}\n--\n"),
            format!("level 2\ndeal 1\n--\n"),
            format!("level 2\ndeal 1\n{{}}\n--\n"),
            format!("level 2\ndeal 1\nnot json\n--\n"),
            format!("level 2\ndeal 1\n{d}\n{d}\n--\n"),
        ] {
            let v = brief_sitting(&bad_deal);
            assert_eq!(v["error"], "bad-input", "{bad_deal:?}: {v}");
        }
        assert!(brief_sitting(&sitting_input("2", &[(1, d)]))["brief"].is_object());
    }

    #[test]
    fn the_level_is_one_to_four() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        for level in ["0", "0.5", "4.5", "9", "-1", "NaN", "1e999", "x", ""] {
            let v = brief_of_record(&format!("level {level}\n{}", input(&text, "")));
            assert_eq!(v["error"], "bad-input", "{level}: {v}");
        }
        for level in ["1", "2.5", "4"] {
            assert!(
                brief_of_record(&format!("level {level}\n{}", input(&text, "")))["brief"]
                    .is_object()
            );
        }
    }

    #[test]
    fn the_ladder_and_the_constants_are_checked_once_and_pass() {
        assert!(constants().is_ok());
        assert!(constants().is_ok());
    }

    /// Untrusted input (a record from storage, an imported file): an error
    /// value, never a panic, which in the module would trap every later call.
    #[test]
    fn nothing_the_calls_are_given_panics() {
        let mut h = host(CONFIG);
        let text = play_to_deal(&mut h, 1);
        let rec: Value = serde_json::from_str(&text).unwrap();
        let with = |f: &dyn Fn(&mut Value)| {
            let mut v = rec.clone();
            f(&mut v);
            v.to_string()
        };
        let long_numbers = vec!["99999999999999999999"; 50_000].join(" ");
        let many_numbers = vec!["1"; 200_000].join(" ");
        let doubled = with(&|v| {
            let a = v["deal"]["actions"].clone();
            v["deal"]["actions"]
                .as_array_mut()
                .unwrap()
                .extend(a.as_array().unwrap().clone());
        });
        let records: Vec<String> = vec![
            String::new(),
            "\n\n\n".into(),
            "nonsense".into(),
            text.clone(),
            with(&|v| v["deal"]["deck"] = json!([])),
            with(&|v| v["deal"]["deck"] = json!(["T1", "T1"])),
            with(&|v| v["deal"]["deck"].as_array_mut().unwrap().truncate(40)),
            with(&|v| {
                let first = v["deal"]["deck"][0].clone();
                v["deal"]["deck"][1] = first;
            }),
            with(&|v| v["deal"]["actions"] = json!([])),
            with(&|v| v["human"] = json!(1)),
            with(&|v| v["human"] = json!(5)),
            with(&|v| v["deal"]["settings"] = json!({})),
            with(&|v| v["deal"]["dealer"] = json!(7)),
            with(&|v| v["deal"]["multiplier"] = json!(0)),
            with(&|v| v["deal"]["multiplier"] = json!(4_294_967_295u64)),
            with(&|v| v["deal"]["actions"][0]["seat"] = json!(5)),
            with(&|v| v["v"] = json!(2)),
            doubled,
            with(&|v| {
                let m = json!({"seat": 0, "action": {"action": "take"}});
                v["deal"]["actions"].as_array_mut().unwrap().push(m);
            }),
            with(&|v| {
                let m = json!({"seat": 1, "action": {"action": "pass"}});
                v["deal"]["actions"].as_array_mut().unwrap().push(m);
            }),
            with(&|v| {
                for m in v["deal"]["actions"].as_array_mut().unwrap() {
                    m["action"] =
                        json!({"action": "play", "card": "T21", "announce": ["trull", "trull"]});
                }
            }),
        ];
        let tails = [
            String::new(),
            "1 2 3".to_string(),
            "-1 x".to_string(),
            long_numbers,
            many_numbers,
        ];
        let calls: [(&str, fn(&str) -> Value); 3] = [
            ("evidence", evidence_of),
            ("brief", |t| brief_of_record(&format!("level 3\n{t}"))),
            ("learner", learner_of),
        ];
        for r in &records {
            for tail in &tails {
                for (name, call) in &calls {
                    for text in [
                        format!("{r}\n==\n{tail}"),
                        format!("{r}\n==\n{tail}\n==\n{r}"),
                        format!("{r}\n{tail}"),
                    ] {
                        let v = call(&text);
                        assert!(v.is_object(), "{name}: {}", &text[..text.len().min(80)]);
                    }
                }
            }
        }
        // the structure of the text itself
        for odd in [
            "",
            "\n",
            "==",
            "--",
            "++",
            "level",
            "level 3",
            "level 3\n==",
            "level 3\n++\n++",
            "level 3\n--\n--",
            "deal 1\n{}",
            "level 3\ndeal 1\n\n--\ngarbage\n--\n--",
            "level 3\n==\n==\n==",
        ] {
            for call in [brief_of_record, brief_sitting, evidence_of, learner_of] {
                assert!(call(odd).is_object(), "{odd:?}");
            }
        }
        // summaries: garbage and out-of-range tallies are stale, not a trap
        let huge = format!(
            "strohmandeln evidence v{EVIDENCE_VERSION}\nworth 4294967295 4294967295 1e999\n"
        );
        for h in ["garbage", &huge, "--\n--\n--", "\n\n"] {
            assert!(learner_of(h).is_object());
        }
        let mut host = Host::default();
        for slug in ["", " ", "worth ", "WORTH", &"w".repeat(100_000)] {
            let _ = host.set_focus(slug);
        }
    }

    #[test]
    fn the_focus_is_set_by_slug_and_the_nudge_shows_it_where_a_miss_would_have_counted() {
        assert_eq!(skill_of("worth"), Ok(Some(Skill::Worth)));
        assert_eq!(skill_of(" "), Ok(None));
        assert_eq!(skill_of("nonsense"), Err(UnknownSkill));
        let mut h = host(CONFIG);
        let mut shown = 0;
        let mut positions = 0;
        h.render();
        for _ in 0..3000 {
            let s = out(&h)["state"].clone();
            if s["prompt"] == "over" || positions > 25 {
                break;
            }
            if s["prompt"] == "opponent" {
                assert_eq!(h.step(), 1);
                continue;
            }
            if s["phase"] == "play" {
                positions += 1;
                let before = s.clone();
                h.show(|t| t.nudge(Some(Skill::Worth)));
                let r = out(&h);
                assert_eq!(
                    r["deal_action"], s["deal_actions"],
                    "the number of the decision"
                );
                if r["nudge"].is_object() {
                    shown += 1;
                    assert_eq!(r["nudge"]["skill"], "worth");
                    assert_eq!(r["nudge"]["key"], "tutor.nudge.worth");
                    // a clear chance at worth has a card that beats the lead
                    // and one that does not
                    assert!(
                        before["deal"]["lead"].is_object(),
                        "worth is the follower's"
                    );
                }
                // asking changes nothing
                h.render();
                assert_eq!(out(&h)["state"], before);
                // and no focus, no nudge
                h.show(|t| t.nudge(None));
                assert_eq!(out(&h)["nudge"], Value::Null);
            }
            let a = s["legal"][0].clone();
            let a = match a["action"].as_str() {
                Some("extend") => json!({"action":"extend","accept":false}),
                Some("play") => json!({"action":"play","card":a["card"]}),
                _ => a,
            };
            assert_eq!(h.send(&a.to_string()), 1);
        }
        assert!(shown > 0, "no nudge in {positions} positions");
    }

    #[test]
    fn the_nudge_is_null_when_it_is_not_the_humans_turn() {
        let mut waited = 0;
        for human in [0, 1] {
            let mut h = host(&format!(r#"{{"seed":7,"touren":3,"human":{human}}}"#));
            h.render();
            if out(&h)["state"]["prompt"] != "opponent" {
                continue;
            }
            waited += 1;
            h.show(|t| t.nudge(Some(Skill::Worth)));
            assert_eq!(out(&h)["nudge"], Value::Null);
        }
        assert_eq!(waited, 1, "the opponent opens for exactly one of the seats");
    }
}
