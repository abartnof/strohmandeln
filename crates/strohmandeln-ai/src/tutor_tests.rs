//! Tests of the evidence over whole deals: students with a skill knocked out,
//! the assisted decisions, purity and the hidden cards (`docs/TUTOR.md` 12).

use super::*;
use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::students::{Lesion, Student};
use crate::testutil::{blind_check_values, c};
use kit_learner::Tally;
use std::sync::OnceLock;
use strohmandeln_core::agents::{Player, RandomPlayer};
use strohmandeln_core::deal::TrickRecord;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::Settings;
use strohmandeln_core::types::Source;
use strohmandeln_core::view::view as sitting_view;

/// A search small enough for tests; noisy, which the margin must cover.
fn small() -> Budget {
    Budget::default()
}

fn config() -> Config {
    Config {
        budget: small(),
        ..Config::default()
    }
}

const HUMAN: Seat = Seat::A;

/// The records of the deals of a sitting of 3 Touren in which seat A is
/// `human` and seat B is rung 4.
fn deals_of(seed: u64, human: &mut dyn Player) -> Vec<DealRecord> {
    let mut b = Opponent::new(Level::from_halves(8), seed + 1000).with_config(config());
    let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
    let mut out = Vec::new();
    let mut counted = 0;
    while let Some(seat) = s.to_move() {
        let v = sitting_view(&s, seat);
        let a = if matches!(v.legal_actions.first(), Some(Action::Extend { .. })) {
            Action::Extend { accept: false }
        } else if seat == HUMAN {
            human.choose(&v).unwrap()
        } else {
            b.choose(&v).unwrap()
        };
        s.apply(seat, a).unwrap();
        if s.deals_counted() > counted {
            counted = s.deals_counted();
            out.push(s.last_deal().unwrap().clone());
        }
    }
    out
}

fn totals(records: &[DealRecord]) -> [Tally; SKILLS] {
    let mut t = [Tally::default(); SKILLS];
    for r in records {
        let e = evidence(r, HUMAN, &[], small()).unwrap();
        for (acc, x) in t.iter_mut().zip(&e.summary.tallies) {
            acc.chances += x.chances;
            acc.met += x.met;
            acc.missed += x.missed;
        }
    }
    t
}

fn student(lesion: Lesion, seed: u64) -> Student {
    Student::new(lesion, seed, config())
}

const SEEDS: std::ops::Range<u64> = 0..4;

/// The sittings the printed measurements use (`print_students`,
/// `print_lesions`, `print_pagat_variants`).
const WIDE: std::ops::Range<u64> = 0..9;

fn play(lesion: Lesion) -> Vec<DealRecord> {
    SEEDS
        .flat_map(|seed| deals_of(seed, &mut student(lesion, seed + 77)))
        .collect()
}

fn rate(t: &[Tally; SKILLS], skill: Skill) -> f64 {
    let x = t[skill.index()];
    f64::from(x.met) / f64::from(x.chances.max(1))
}

fn overall(t: &[Tally; SKILLS]) -> f64 {
    let (m, n) = t
        .iter()
        .fold((0, 0), |(m, n), x| (m + x.met, n + x.chances));
    f64::from(m) / f64::from(n.max(1))
}

struct Students {
    whole: Vec<DealRecord>,
    whole_t: [Tally; SKILLS],
    no_cover: Vec<DealRecord>,
    no_cover_t: [Tally; SKILLS],
    random_t: [Tally; SKILLS],
}

/// Rung 4 whole, the no-cover student and a random player over `seeds`
/// sittings of seat A.
fn fixture(seeds: std::ops::Range<u64>) -> Students {
    let plays = |lesion: Lesion| -> Vec<DealRecord> {
        seeds
            .clone()
            .flat_map(|seed| deals_of(seed, &mut student(lesion, seed + 77)))
            .collect()
    };
    let whole = plays(Lesion::None);
    let no_cover = plays(Lesion::Off);
    let random: Vec<DealRecord> = seeds
        .clone()
        .flat_map(|seed| deals_of(seed, &mut RandomPlayer::new(seed, HUMAN)))
        .collect();
    Students {
        whole_t: totals(&whole),
        no_cover_t: totals(&no_cover),
        random_t: totals(&random),
        whole,
        no_cover,
    }
}

/// The students over `SEEDS`, played once and shared by the lesion tests
/// (`bin/lesions`).
fn students() -> &'static Students {
    static S: OnceLock<Students> = OnceLock::new();
    S.get_or_init(|| fixture(SEEDS))
}

/// One sitting (6 deals, a few seconds): what the tests that only need
/// records, and the gate's lesion check, use.
fn one_sitting() -> &'static Students {
    static S: OnceLock<Students> = OnceLock::new();
    S.get_or_init(|| fixture(0..1))
}

#[test]
#[ignore = "calibration output"]
fn print_pagat_variants() {
    let wide = WIDE;
    let whole: Vec<DealRecord> = wide
        .clone()
        .flat_map(|seed| deals_of(seed, &mut student(Lesion::None, seed + 77)))
        .collect();
    let random: Vec<DealRecord> = wide
        .flat_map(|seed| deals_of(seed, &mut RandomPlayer::new(seed, HUMAN)))
        .collect();
    // (name, filter on the position) -> (met, chances), for rung 4 and random
    type Filter = fn(&Knowledge) -> bool;
    let wins: Filter = |k| {
        k.lead
            .is_some_and(|l| !crate::cardset::leader_wins(l, Card::PAGAT))
    };
    let filters: [(&str, Filter); 5] = [
        ("all", |_| true),
        ("follower", |k| k.lead.is_some()),
        ("cash would win the trick", wins),
        ("last 5 tricks", |k| k.trick_number >= 23),
        ("sourced zones", |k| {
            k.my_tarocks() <= 7 || k.my_tarocks() >= 12
        }),
    ];
    for (who, recs) in [("rung 4", &whole), ("random", &random)] {
        let mut t = [(0u32, 0u32); 5];
        for rec in recs {
            rec.replay(|_, d, seat, a| {
                if seat != HUMAN {
                    return;
                }
                let v = bare_view(d, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                for ch in decision_verdicts(&v, a, small())
                    .iter()
                    .filter(|x| x.skill == Skill::Pagat)
                {
                    for (i, (_, f)) in filters.iter().enumerate() {
                        if f(&k) {
                            t[i].1 += 1;
                            t[i].0 += u32::from(ch.met);
                        }
                    }
                }
            })
            .unwrap();
        }
        for (i, (name, _)) in filters.iter().enumerate() {
            println!("{who:7} {name:26} met {} of {}", t[i].0, t[i].1);
        }
    }
}

/// The met-rate matrix of `docs/TUTOR.md` 11 (M4): for each knock-out, every
/// skill's met chances over `WIDE` sittings (`cargo test -p strohmandeln-ai
/// --release print_lesions -- --ignored --nocapture`).
#[test]
#[ignore = "calibration output"]
fn print_lesions() {
    let run = |lesion: Lesion| -> [Tally; SKILLS] {
        let recs: Vec<DealRecord> = WIDE
            .flat_map(|seed| deals_of(seed, &mut student(lesion, seed + 77)))
            .collect();
        totals(&recs)
    };
    let row = |name: &str, t: &[Tally; SKILLS]| {
        let cells: Vec<String> = Skill::ALL
            .iter()
            .map(|sk| {
                let x = t[sk.index()];
                format!("{:>3}/{:<3}", x.met, x.chances)
            })
            .collect();
        println!("{name:12} {}", cells.join(" "));
    };
    let heads: Vec<String> = Skill::ALL
        .iter()
        .map(|s| format!("{:7}", s.slug()))
        .collect();
    println!("{:12} {}", "", heads.join(" "));
    row("whole", &run(Lesion::None));
    for l in [
        Lesion::Worth,
        Lesion::Cheap,
        Lesion::Off,
        Lesion::Run,
        Lesion::Pagat,
        Lesion::Void,
        Lesion::Behind,
        Lesion::Endgame,
        Lesion::Bid,
        Lesion::AlwaysPass,
        Lesion::AlwaysTake,
        Lesion::Announce,
    ] {
        row(&format!("{l:?}"), &run(l));
    }
}

/// Rung 4 and a random player over `WIDE` sittings, by skill: what `LADDER`'s
/// `top_rate` and `top_cost` are refitted from.
#[test]
#[ignore = "calibration output"]
fn print_students() {
    let s = fixture(WIDE);
    let deals = s.whole.len() as f64;
    for (name, t) in [("whole", &s.whole_t), ("random", &s.random_t)] {
        println!("{name} overall {:.3} over {deals} deals", overall(t));
        for sk in Skill::ALL {
            let x = t[sk.index()];
            println!(
                "  {:8} chances {:4} met {:4} rate {:.3} missed {:7.1} ({:.2} a deal)",
                sk.slug(),
                x.chances,
                x.met,
                f64::from(x.met) / f64::from(x.chances.max(1)),
                x.missed,
                x.missed / deals
            );
        }
    }
}

/// In the gate, on one sitting: a lesion is seen (0 of 34 over four sittings
/// against 14 of 15 for rung 4).
#[test]
fn a_student_who_never_plays_a_cover_off_misses_every_off_chance() {
    let s = one_sitting();
    let off = s.no_cover_t[Skill::Off.index()];
    assert!(off.chances >= 4, "{off:?}");
    assert_eq!(off.met, 0, "{off:?}");
    let whole = s.whole_t[Skill::Off.index()];
    assert!(off.missed > 3.0 * whole.missed, "{off:?} against {whole:?}");
}

#[test]
fn rung_four_whole_plays_as_the_opponent_does() {
    // Lesion::None is the opponent; the other tests treat it as one.
    struct Checked(Student, Opponent);
    impl Player for Checked {
        fn choose(&mut self, view: &View) -> Option<Action> {
            let mine = self.0.choose(view)?;
            assert_eq!(Some(mine.clone()), self.1.choose(view));
            Some(mine)
        }
    }
    let config = config();
    let mut checked = Checked(
        Student::new(Lesion::None, 77, config),
        Opponent::new(Level::from_halves(8), 77).with_config(config),
    );
    assert!(deals_of(0, &mut checked).len() >= 6);
}

/// A student with `lesion` over the fixture's sittings: how its target
/// skill's met rate compares with rung 4 whole on the same deals. Total
/// lesions only (partial ones show in tens of deals, M6).
fn lesion_shows(lesion: Lesion, at_least_chances: u32, rate_below_by: f64) {
    lesion_shows_missing(lesion, at_least_chances, rate_below_by, 3.0);
}

fn lesion_shows_missing(lesion: Lesion, at_least_chances: u32, rate_below_by: f64, times: f64) {
    let whole = &students().whole_t;
    let skill = lesion.skill().unwrap();
    let t = totals(&play(lesion));
    let (l, w) = (t[skill.index()], whole[skill.index()]);
    assert!(
        l.chances >= at_least_chances,
        "{lesion:?}: {} chances of {}",
        l.chances,
        skill.slug()
    );
    assert!(
        rate(&t, skill) + rate_below_by <= rate(whole, skill),
        "{lesion:?}: lesioned {l:?} against whole {w:?}"
    );
    assert!(
        l.missed > times * w.missed,
        "{lesion:?}: {l:?} against {w:?}"
    );
    // the knock-out hurts its own skill most, among the skills it had five
    // chances at (a class lesion costs others something: `docs/TUTOR.md` 11)
    for other in Skill::ALL {
        let x = t[other.index()];
        if other != skill && x.chances >= 5 {
            assert!(
                rate(&t, skill) <= rate(&t, other),
                "{lesion:?}: {} {x:?} is worse than {} {l:?}",
                other.slug(),
                skill.slug()
            );
        }
    }
}

// The numbers in the comments are from 6 sittings at the default budget
// (`print_lesions`): the target skill's met chances, lesioned and whole.

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_lets_every_trick_go_misses_worth() {
    lesion_shows(Lesion::Worth, 20, 0.5); // 0/56 and 54/57
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_plays_the_dearest_card_misses_cheap() {
    lesion_shows(Lesion::Cheap, 20, 0.5); // 11/65 and 91/92
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_never_plays_a_cover_off_misses_off() {
    lesion_shows(Lesion::Off, 10, 0.5); // 0/48 and 17/18
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_leads_a_random_tarock_misses_run() {
    lesion_shows(Lesion::Run, 10, 0.1); // 27/38 and 44/48
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_tosses_a_coin_for_the_pagat_misses_pagat() {
    lesion_shows(Lesion::Pagat, 4, 0.25); // 10/20 and 18/18
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_leads_into_his_void_misses_void() {
    lesion_shows(Lesion::Void, 3, 0.5); // 0/17 and 25/25
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_opens_the_split_suit_misses_behind() {
    lesion_shows(Lesion::Behind, 10, 0.5); // 0/39 and 31/34
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_stops_counting_in_the_endgame_misses_endgame() {
    lesion_shows_missing(Lesion::Endgame, 10, 0.1, 2.0); // 37/52 and 50/54
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_bids_by_a_coin_misses_bid() {
    lesion_shows(Lesion::Bid, 5, 0.25); // 9/17 and 17/17
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_takes_up_every_hand_misses_bid() {
    lesion_shows(Lesion::AlwaysTake, 5, 0.5); // 1/17 and 17/17
}

#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn a_student_who_never_announces_misses_announce() {
    lesion_shows(Lesion::Announce, 1, 0.5); // 0/4 and 4/4
}

/// The fixture's values at the margin as set (3 sittings, default budget):
/// rung 4 meets 0.96 of its chances (every skill with 8 or more chances at
/// 0.88 or better), a random player 0.63 (`worth` 24 of 44).
#[test]
#[ignore = "lesions, about 100 s: bin/lesions"]
fn whole_rung_four_meets_nearly_every_chance_and_a_random_player_does_not() {
    let s = students();
    assert!(overall(&s.whole_t) >= 0.9, "{:?}", s.whole_t);
    for sk in Skill::ALL {
        if s.whole_t[sk.index()].chances >= 8 {
            assert!(
                rate(&s.whole_t, sk) >= 0.8,
                "{}: {:?}",
                sk.slug(),
                s.whole_t
            );
        }
    }
    assert!(overall(&s.random_t) <= 0.75, "{:?}", s.random_t);
    // the first rung of the ladder shows the difference most
    assert!(rate(&s.random_t, Skill::Worth) <= 0.65);
    let (w, r) = (s.whole_t[0], s.random_t[0]);
    assert!(r.missed > 5.0 * w.missed, "{r:?} against {w:?}");
}

#[test]
fn a_search_that_finished_no_trick_gives_no_play_verdict() {
    // With a budget too small to finish a trick `move_table` falls back to
    // rung 3's scale, which is not thirds: the decision is skipped.
    let rec = &one_sitting().whole[0];
    let starved = Budget {
        worlds: 3,
        nodes: 1,
        bid_worlds: 4,
    };
    let (mut full, mut none) = (0, 0);
    rec.replay(|_, d, seat, a| {
        let k = Knowledge::from_view(&bare_view(d, seat)).unwrap();
        if seat != HUMAN || !k.playing || k.legal.len() < 2 {
            return;
        }
        let plays = |b| {
            decision_verdicts(&bare_view(d, seat), a, b)
                .iter()
                .filter(|v| v.skill != Skill::Announce)
                .count()
        };
        full += plays(small());
        none += plays(starved);
    })
    .unwrap();
    assert!(full > 3, "{full}");
    assert_eq!(none, 0);
}

#[test]
fn the_costliest_miss_is_a_real_decision_of_the_human_in_the_deal() {
    let s = one_sitting();
    let mut seen = 0;
    for rec in &s.no_cover {
        let e = evidence(rec, HUMAN, &[], small()).unwrap();
        let off = e.summary.of(Skill::Off.index());
        let Some(m) = &e.costliest[Skill::Off.index()] else {
            assert_eq!(off.chances, off.met, "a miss is kept when there is one");
            continue;
        };
        seen += 1;
        let a = &rec.actions[m.action];
        assert_eq!(a.seat, HUMAN);
        assert!(matches!(&a.action, Action::Play { card, .. } if Some(*card) == m.played));
        assert!((1..=27).contains(&m.trick));
        assert_ne!(m.played, m.better);
        // it is the costliest: at least the mean of the misses, at most their sum
        let misses = f64::from(off.chances - off.met);
        assert!(m.cost >= off.missed / misses - 1e-9 && m.cost <= off.missed + 1e-9);
        assert!(matches!(
            m.direction,
            Some(Direction::Figure | Direction::Pip)
        ));
        // the better card is the human's own cover at that moment
        let mut covers = Vec::new();
        rec.replay(|i, d, seat, _| {
            if i == m.action {
                covers = (1..=3).filter_map(|p| d.cover(seat, p)).collect();
            }
        })
        .unwrap();
        assert!(covers.contains(&m.better.unwrap()));
    }
    assert!(seen >= 3, "{seen}");
}

#[test]
fn an_assisted_decision_is_no_evidence() {
    let s = one_sitting();
    let rec = &s.no_cover[0];
    let full = evidence(rec, HUMAN, &[], small()).unwrap();
    // every action assisted: nothing counted
    let all: Vec<usize> = (0..rec.actions.len()).collect();
    let none = evidence(rec, HUMAN, &all, small()).unwrap();
    assert!(none.chances.is_empty());
    assert!(none.costliest.iter().all(Option::is_none));
    // one decision assisted: exactly its chances are gone, the rest the same
    let mut per_action: Vec<(usize, usize)> = Vec::new();
    rec.replay(|i, d, seat, a| {
        if seat == HUMAN {
            let n = decision_verdicts(&bare_view(d, seat), a, small()).len();
            if n > 0 {
                per_action.push((i, n));
            }
        }
    })
    .unwrap();
    assert!(per_action.len() >= 3);
    let (assisted, _) = per_action[1];
    let some = evidence(rec, HUMAN, &[assisted], small()).unwrap();
    let mut kept = Vec::new();
    let mut at = 0;
    for &(i, n) in &per_action {
        if i != assisted {
            kept.extend_from_slice(&full.chances[at..at + n]);
        }
        at += n;
    }
    assert_eq!(at, full.chances.len());
    assert_eq!(some.chances, kept);
}

#[test]
fn an_assisted_announcement_covers_the_play_it_belongs_to() {
    // A hint is shown for the position; the human may then announce and play
    // in the same turn. Marking the announcement marks the play too.
    use strohmandeln_core::session::Move;
    // (seed 0 has one: the human announces with a card in it)
    let (rec, at) = one_sitting()
        .whole
        .iter()
        .cloned()
        .find_map(|r| {
            let at = r.actions.iter().position(|m| {
                m.seat == HUMAN
                    && matches!(&m.action, Action::Play { announce, .. } if !announce.is_empty())
            })?;
            Some((r, at))
        })
        .expect("a deal in which the human announces with a card");
    // split the announcing play in two: Announce, then the card alone
    let Action::Play { card, announce } = rec.actions[at].action.clone() else {
        unreachable!()
    };
    let mut split = rec.clone();
    split.actions[at].action = Action::play(card);
    for (n, honour) in announce.iter().enumerate() {
        split.actions.insert(
            at + n,
            Move {
                seat: HUMAN,
                action: Action::Announce { honour: *honour },
            },
        );
    }
    let play_at = at + announce.len();
    let full = evidence(&split, HUMAN, &[], small()).unwrap();
    // the chances of the announcements and of the play
    let count = |range: std::ops::Range<usize>| {
        let mut n = 0;
        split
            .replay(|i, d, seat, a| {
                if seat == HUMAN && range.contains(&i) {
                    n += decision_verdicts(&bare_view(d, seat), a, small()).len();
                }
            })
            .unwrap();
        n
    };
    let turn = count(at..play_at + 1);
    assert!(turn >= 1);
    let only_first = evidence(&split, HUMAN, &[at], small()).unwrap();
    assert_eq!(only_first.chances.len(), full.chances.len() - turn);
    // asking at the play alone leaves the announcement counted
    let play_only = evidence(&split, HUMAN, &[play_at], small()).unwrap();
    assert!(play_only.chances.len() > only_first.chances.len());
}

/// A scene in which forehand (seat A, who bids first) holds `hand`.
fn bidding_with(hand: [&str; 15]) -> crate::testutil::Scene {
    let mine: Vec<Card> = hand.iter().map(|x| c(x)).collect();
    let slots = [0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24];
    let mut rest = Card::all().filter(|x| !mine.contains(x));
    let deck: Vec<Card> = (0..54)
        .map(|i| match slots.iter().position(|&s| s == i) {
            Some(j) => mine[j],
            None => rest.next().unwrap(),
        })
        .collect();
    crate::testutil::Scene::new(&deck)
}

#[test]
fn a_bid_is_judged_end_to_end_both_ways() {
    // A hand worth taking up (`rung4_tests`: the estimate is positive): taking
    // up is the met chance and passing the missed one; a poor hand the other
    // way round. The default budget's 40 worlds.
    let strong = bidding_with([
        "SK", "T21", "T1", "T20", "T19", "T18", "T17", "T16", "Kh", "Kd", "Kc", "Ks", "Qh", "Qd",
        "Qc",
    ]);
    let weak = bidding_with([
        "T2", "T3", "T4", "T5", "T6", "Jh", "Jd", "7c", "8c", "9c", "10c", "7s", "8s", "9s", "10s",
    ]);
    let bid = |scene: &crate::testutil::Scene, action: &Action| {
        decision_verdicts(&scene.view(Seat::A), action, small())
            .into_iter()
            .find(|v| v.skill == Skill::Bid)
    };
    let took = bid(&strong, &Action::Take).expect("a clear chance");
    assert!(took.met);
    let passed = bid(&strong, &Action::Pass).expect("a clear chance");
    assert!(!passed.met);
    assert_eq!(passed.direction, Some(Direction::Passed));
    assert!(passed.cost > 0.0);
    assert!(bid(&weak, &Action::Pass).expect("a clear chance").met);
    let took = bid(&weak, &Action::Take).expect("a clear chance");
    assert!(!took.met);
    assert_eq!(took.direction, Some(Direction::Took));
}

#[test]
fn the_evidence_is_a_function_of_the_record() {
    let rec = &one_sitting().whole[1];
    let a = evidence(rec, HUMAN, &[2], small()).unwrap();
    assert_eq!(a, evidence(rec, HUMAN, &[2], small()).unwrap());
    assert_eq!(a.summary.version, EVIDENCE_VERSION);
    assert_eq!(a.summary.tallies.len(), SKILLS);
    assert_eq!(a.summary, Summary::from_chances(&LADDER, &a.chances));
}

#[test]
fn a_record_that_does_not_replay_is_an_error_not_evidence() {
    let mut rec = one_sitting().whole[0].clone();
    rec.actions.truncate(3);
    rec.actions.push(rec.actions[2].clone());
    assert!(evidence(&rec, HUMAN, &[], small()).is_err());
}

#[test]
fn the_evidence_never_reads_a_hidden_card() {
    // Twin deals that differ only in what the seat cannot see give it the
    // same view; the verdicts on every legal action (the fields the `Miss`
    // copies, better card, direction, tarocks, sure winners, among them) must
    // be the same in both. `evidence` adds to these only the view's own trick
    // and card led, and the public tricks of the finished deal.
    let tiny = Budget {
        worlds: 2,
        nodes: 1_500,
        bid_worlds: 4,
    };
    let twins = blind_check_values(
        &|v: &View| {
            v.legal_actions
                .iter()
                .map(|a| decision_verdicts(v, a, tiny))
                .collect::<Vec<_>>()
        },
        6,
    );
    assert!(twins > 100, "{twins}");
}

#[test]
fn a_figure_that_fell_under_a_higher_card_of_its_suit_is_found() {
    let trick = |leader, cards: [&str; 2], winner| TrickRecord {
        leader,
        cards: [c(cards[0]), c(cards[1])],
        sources: [Source::Hand; 2],
        winner,
    };
    let tricks = [
        trick(Seat::B, ["3h", "4h"], Seat::B),
        trick(Seat::B, ["Kh", "Qh"], Seat::B),
        trick(Seat::A, ["Ch", "Qd"], Seat::A),
    ];
    // A's Dame fell under B's king in trick 2
    assert_eq!(
        fell_under(&tricks, c("Qh"), Seat::A),
        Some(Consequence::FellUnder {
            by: c("Kh"),
            trick: 2
        })
    );
    // A's Cavall won its trick
    assert_eq!(fell_under(&tricks, c("Ch"), Seat::A), None);
    // a card never played
    assert_eq!(fell_under(&tricks, c("Jh"), Seat::A), None);
    // a trump over a suit card is not a higher card of its suit
    let trumped = [trick(Seat::B, ["Qs", "T3"], Seat::A)];
    assert_eq!(fell_under(&trumped, c("Qs"), Seat::B), None);
}

#[test]
fn the_ladder_is_well_formed_and_matches_the_skills() {
    LADDER.check().unwrap();
    PARAMS.check().unwrap();
    assert_eq!(LADDER.skills.len(), SKILLS);
    assert_eq!(LADDER.version, EVIDENCE_VERSION);
    for s in Skill::ALL {
        let spec = LADDER.skills[s.index()];
        assert_eq!(spec.slug, s.slug());
        let needs: Vec<usize> = s.needs().iter().map(|n| n.index()).collect();
        assert_eq!(spec.needs, &needs[..], "{}", s.slug());
    }
}

#[test]
fn the_skills_are_ten_in_ladder_order_and_the_prerequisites_come_first() {
    assert_eq!(Skill::ALL.len(), SKILLS);
    let slugs: std::collections::HashSet<_> = Skill::ALL.iter().map(|s| s.slug()).collect();
    assert_eq!(slugs.len(), SKILLS);
    for (i, s) in Skill::ALL.iter().enumerate() {
        assert_eq!(s.index(), i);
        for n in s.needs() {
            assert!(n.index() < i, "{} needs {}", s.slug(), n.slug());
        }
    }
    assert!(Skill::Worth.needs().is_empty());
    assert_eq!(Skill::Behind.needs(), &[Skill::Off, Skill::Void]);
    assert_eq!(Skill::Endgame.needs(), &[Skill::Cheap, Skill::Void]);
}

/// The evidence over whole deals, native:
/// `cargo test -p strohmandeln-ai --release evidence_latency -- --ignored --nocapture`
#[test]
#[ignore]
fn evidence_latency() {
    let recs = play(Lesion::None);
    let mut ms: Vec<f64> = recs
        .iter()
        .map(|r| {
            let t = std::time::Instant::now();
            evidence(r, HUMAN, &[], Budget::default()).unwrap();
            t.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let q = |p: f64| ms[((ms.len() - 1) as f64 * p).round() as usize];
    eprintln!(
        "deals {} median {:.0} ms p95 {:.0} ms max {:.0} ms",
        ms.len(),
        q(0.5),
        q(0.95),
        ms[ms.len() - 1]
    );
}

/// M1 re-draws a position under other streams: stream 0 is the evidence's own,
/// the others differ and are reproducible.
#[test]
fn the_advisor_streams_are_the_evidence_for_zero_and_independent_otherwise() {
    let rec = &one_sitting().whole[0];
    let mut checked = 0;
    let mut differed = 0;
    rec.replay(|_, d, seat, a| {
        let view = bare_view(d, seat);
        let Some(k) = Knowledge::from_view(&view) else {
            return;
        };
        if !k.playing || k.legal.len() < 2 || k.hidden_total() == 0 {
            return;
        }
        let Action::Play { card, .. } = a else {
            return;
        };
        let t0 = play_table(&k, &view, small(), 0).unwrap();
        let again = play_table(&k, &view, small(), 0).unwrap();
        assert_eq!(t0.scores, again.scores, "a stream replays");
        // stream 0 gives the verdicts the evidence gives
        let via = classify_play(&k, *card, &advice_of(&k, &t0));
        let direct: Vec<Verdict> = decision_verdicts(&view, a, small())
            .into_iter()
            .filter(|v| v.skill != Skill::Announce)
            .collect();
        assert_eq!(via, direct);
        let t1 = play_table(&k, &view, small(), 1).unwrap();
        differed += usize::from(t0.scores != t1.scores);
        checked += 1;
    })
    .unwrap();
    assert!(checked >= 10, "{checked}");
    assert!(
        differed * 2 > checked,
        "{differed} of {checked} differ under another stream"
    );
}

#[test]
fn a_bid_estimate_replays_and_differs_by_stream() {
    let rec = &one_sitting().whole[0];
    let mut seen = 0;
    rec.replay(|_, d, seat, a| {
        let view = bare_view(d, seat);
        let Some(k) = Knowledge::from_view(&view) else {
            return;
        };
        if k.playing || !matches!(a, Action::Take | Action::Pass) {
            return;
        }
        let e0 = bid_estimate(&k, &view, small(), 0).unwrap();
        assert_eq!(e0, bid_estimate(&k, &view, small(), 0).unwrap());
        assert_ne!(e0, bid_estimate(&k, &view, small(), 3).unwrap());
        seen += 1;
    })
    .unwrap();
    assert!(seen >= 1);
}

/// M4 knocks out two skills at once to check the prerequisites: a student
/// with both lesions does what each does, and one lesion alone is the single
/// student it always was.
#[test]
fn a_student_with_two_knock_outs_does_both() {
    let one = Student::new(Lesion::Off, 77, config());
    let same = Student::knocked_out(&[Lesion::Off], 77, config());
    let rec = &one_sitting().whole[0];
    rec.replay(|_, d, seat, _| {
        let view = bare_view(d, seat);
        if view.legal_actions.is_empty() {
            return;
        }
        assert_eq!(one.decide(&view), same.decide(&view));
    })
    .unwrap();
    let both = Student::knocked_out(&[Lesion::Worth, Lesion::Off], 77, config());
    assert_eq!(both.lesions(), &[Lesion::Worth, Lesion::Off]);
    let recs = deals_of(0, &mut { both });
    let t = totals(&recs);
    let (w, o) = (rate(&t, Skill::Worth), rate(&t, Skill::Off));
    let whole = &one_sitting().whole_t;
    assert!(t[Skill::Worth.index()].chances >= 4, "{t:?}");
    assert!(w + 0.3 <= rate(whole, Skill::Worth), "worth {w}");
    assert!(
        o + 0.3 <= rate(whole, Skill::Off) || t[Skill::Off.index()].chances < 2,
        "off {o}"
    );
}
