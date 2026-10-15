//! Tests of the evidence over whole deals: students with a skill knocked out,
//! the assisted decisions, purity and the hidden cards (`docs/TUTOR.md` 12).

use super::*;
use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung2::play_card;
use crate::testutil::{blind_check_values, c};
use kit_learner::Tally;
use std::sync::OnceLock;
use strohmandeln_core::agents::{Player, RandomPlayer};
use strohmandeln_core::deal::TrickRecord;
use strohmandeln_core::rng::Rng;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Lesion {
    /// Rung 4 whole.
    None,
    /// Never plays a cover while a hand card is legal.
    NoCover,
    /// Never announces.
    Silent,
    /// Takes up every hand.
    AlwaysTake,
}

/// Rung 4 with one skill knocked out; the seat is the human's.
struct Student {
    opp: Opponent,
    lesion: Lesion,
}

impl Player for Student {
    fn choose(&mut self, view: &View) -> Option<Action> {
        if view.legal_actions.is_empty() {
            return None;
        }
        let a = self.opp.decide(view);
        Some(match (self.lesion, a) {
            (Lesion::NoCover, Action::Play { card, .. }) => {
                let k = Knowledge::from_view(view)?;
                if !k.is_cover(card) || k.legal.iter().all(|&c| k.is_cover(c)) {
                    return Some(play_card(&k, card));
                }
                let mut rng = Rng::stream(5, crate::seed::view_seed(view));
                let (scores, _) = rung4::move_scores(
                    &k,
                    &mut rng,
                    self.opp.config.budget,
                    &self.opp.config.weights,
                );
                let pick = scores
                    .iter()
                    .find(|(c, _)| !k.is_cover(*c))
                    .map(|&(c, _)| c)
                    .expect("a hand card is legal");
                play_card(&k, pick)
            }
            (Lesion::Silent, Action::Play { card, .. }) => Action::play(card),
            (Lesion::AlwaysTake, Action::Pass) => Action::Take,
            (_, a) => a,
        })
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
    Student {
        opp: Opponent::new(Level::from_halves(8), seed).with_config(config()),
        lesion,
    }
}

const SEEDS: std::ops::Range<u64> = 0..3;

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
    silent_t: [Tally; SKILLS],
    taker_t: [Tally; SKILLS],
    random_t: [Tally; SKILLS],
}

/// The students are played once and shared by the tests.
fn students() -> &'static Students {
    static S: OnceLock<Students> = OnceLock::new();
    S.get_or_init(|| {
        let whole = play(Lesion::None);
        let no_cover = play(Lesion::NoCover);
        let random: Vec<DealRecord> = SEEDS
            .flat_map(|seed| deals_of(seed, &mut RandomPlayer::new(seed, HUMAN)))
            .collect();
        Students {
            whole_t: totals(&whole),
            no_cover_t: totals(&no_cover),
            silent_t: totals(&play(Lesion::Silent)),
            taker_t: totals(&play(Lesion::AlwaysTake)),
            random_t: totals(&random),
            whole,
            no_cover,
        }
    })
}

#[test]
#[ignore = "calibration output"]
fn print_pagat_variants() {
    let wide = 0..9u64;
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
    let filters: [(&str, Filter); 6] = [
        ("all", |_| true),
        ("follower", |k| k.lead.is_some()),
        ("cash would win the trick", wins),
        ("last 5 tricks", |k| k.trick_number >= 23),
        ("sourced zones", |k| {
            k.my_tarocks() <= 7 || k.my_tarocks() >= 12
        }),
        ("leader", |k| k.lead.is_none()),
    ];
    for (who, recs) in [("rung 4", &whole), ("random", &random)] {
        let mut t = [(0u32, 0u32); 6];
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

#[test]
#[ignore = "calibration output"]
fn print_students() {
    let s = students();
    for (name, t) in [
        ("whole", &s.whole_t),
        ("no cover", &s.no_cover_t),
        ("silent", &s.silent_t),
        ("taker", &s.taker_t),
        ("random", &s.random_t),
    ] {
        println!("{name} overall {:.2}", overall(t));
        for sk in Skill::ALL {
            let x = t[sk.index()];
            println!(
                "  {:8} chances {:4} met {:4} missed {:7.1}",
                sk.slug(),
                x.chances,
                x.met,
                x.missed
            );
        }
    }
}

#[test]
fn a_student_who_never_plays_a_cover_off_misses_every_off_chance() {
    let s = students();
    let off = s.no_cover_t[Skill::Off.index()];
    assert!(off.chances >= 10, "{off:?}");
    assert_eq!(off.met, 0, "{off:?}");
    let whole = s.whole_t[Skill::Off.index()];
    assert!(rate(&s.whole_t, Skill::Off) >= 0.8, "{whole:?}");
    assert!(
        off.missed > 3.0 * whole.missed,
        "{} against {}",
        off.missed,
        whole.missed
    );
}

#[test]
fn a_student_who_never_announces_misses_the_honours_he_holds() {
    let s = students();
    let silent = s.silent_t[Skill::Announce.index()];
    assert!(silent.chances >= 1 && silent.met == 0, "{silent:?}");
    assert_eq!(silent.missed, THIRDS_PER_POINT * f64::from(silent.chances));
    let whole = s.whole_t[Skill::Announce.index()];
    assert!(
        whole.chances >= 1 && whole.met == whole.chances,
        "{whole:?}"
    );
}

#[test]
fn a_student_who_takes_up_every_hand_misses_the_bids_that_should_pass() {
    let s = students();
    let taker = s.taker_t[Skill::Bid.index()];
    assert!(taker.chances >= 3 && taker.met == 0, "{taker:?}");
    let whole = s.whole_t[Skill::Bid.index()];
    assert!(whole.met * 10 >= whole.chances * 8, "{whole:?}");
}

/// The fixture's values at the margin as set (3 sittings, default budget):
/// rung 4 meets 0.96 of its chances (every skill with 8 or more chances at
/// 0.88 or better), a random player 0.63 (`worth` 24 of 44).
#[test]
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
    let rec = &students().whole[0];
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
    let s = students();
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
    let s = students();
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
    let (rec, at) = students()
        .whole
        .iter()
        .find_map(|r| {
            let at = r.actions.iter().position(|m| {
                m.seat == HUMAN
                    && matches!(&m.action, Action::Play { announce, .. } if !announce.is_empty())
            })?;
            Some((r.clone(), at))
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

#[test]
fn the_evidence_is_a_function_of_the_record() {
    let rec = &students().whole[1];
    let a = evidence(rec, HUMAN, &[2], small()).unwrap();
    assert_eq!(a, evidence(rec, HUMAN, &[2], small()).unwrap());
    assert_eq!(a.summary.version, EVIDENCE_VERSION);
    assert_eq!(a.summary.tallies.len(), SKILLS);
    assert_eq!(a.summary, Summary::from_chances(&LADDER, &a.chances));
}

#[test]
fn a_record_that_does_not_replay_is_an_error_not_evidence() {
    let mut rec = students().whole[0].clone();
    rec.actions.truncate(3);
    rec.actions.push(rec.actions[2].clone());
    assert!(evidence(&rec, HUMAN, &[], small()).is_err());
}

#[test]
fn the_evidence_never_reads_a_hidden_card() {
    // Twin deals that differ only in what the seat cannot see give it the
    // same view; the verdicts on the first and the last legal action must
    // be the same in both.
    let tiny = Budget {
        worlds: 2,
        nodes: 1_500,
        bid_worlds: 4,
    };
    let twins = blind_check_values(
        &|v: &View| {
            let first = v.legal_actions[0].clone();
            let last = v.legal_actions[v.legal_actions.len() - 1].clone();
            (
                decision_verdicts(v, &first, tiny),
                decision_verdicts(v, &last, tiny),
            )
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
