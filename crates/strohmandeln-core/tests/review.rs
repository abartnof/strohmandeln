//! Tests added by the engine review (docs/review/engine-core.md): independent
//! re-computations of the rules from the event stream, a fuzz over arbitrary
//! (mostly illegal) actions, and leak checks on the events a player is sent.
//!
//! Nothing here reuses the engine's own scoring or legality code: the
//! reference implementations below are written from RULES.md.

use std::collections::{BTreeMap, HashSet};
use strohmandeln_core::agents::{play_out, Player, RandomPlayer};
use strohmandeln_core::cards::{Card, Honour};
use strohmandeln_core::rng::Rng;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::*;
use strohmandeln_core::view::{filter_event, view};

fn settings_for(seed: u64) -> Settings {
    Settings {
        both_pass: if seed.is_multiple_of(2) {
            BothPass::Play
        } else {
            BothPass::ThrowIn
        },
        values: [Values::V234, Values::V123, Values::V124][(seed / 2 % 3) as usize],
        ultimo: 1 + (seed / 6 % 2) as u32,
    }
}

fn sitting_for(seed: u64) -> Sitting {
    Sitting::new(
        seed,
        settings_for(seed),
        if seed.is_multiple_of(5) { 6 } else { 3 },
    )
    .unwrap()
}

/// A player that picks uniformly among legal actions but, with a play, also
/// announces a random subset of the honours it may announce.
struct AnnouncingPlayer {
    rng: Rng,
    sitting_hint: std::rc::Rc<std::cell::RefCell<Vec<Honour>>>,
}

impl Player for AnnouncingPlayer {
    fn choose(&mut self, v: &strohmandeln_core::view::View) -> Option<Action> {
        if v.legal_actions.is_empty() {
            return None;
        }
        let a = v.legal_actions[self.rng.below(v.legal_actions.len() as u64) as usize].clone();
        Some(match a {
            Action::Play { card, .. } => {
                let held = self.sitting_hint.borrow().clone();
                let announce = held
                    .into_iter()
                    .filter(|_| self.rng.below(2) == 0)
                    .collect();
                Action::Play { card, announce }
            }
            other => other,
        })
    }
}

// ---------------------------------------------------------------------------
// A reference scorer, from the events alone (RULES 7 to 12, 14.2)
// ---------------------------------------------------------------------------

#[derive(Default)]
struct DealLog {
    multiplier: u32,
    declarer: Option<Seat>,
    announced: Vec<(Seat, Honour)>,
    played: Vec<(Seat, Card, u32)>,
    won: Vec<(Seat, [Card; 2], u32)>,
}

fn thirds(c: Card) -> u32 {
    let v = match c.code().as_str() {
        "SK" | "T21" | "T1" => 5,
        code if code.starts_with('K') => 5,
        code if code.starts_with('Q') => 4,
        code if code.starts_with('C') => 3,
        code if code.starts_with('J') => 2,
        _ => 1,
    };
    3 * v - 2
}

fn key(c: &Credit) -> String {
    format!("{:?}/{:?}/{}", c.seat, c.cause, c.points)
}

fn expected_credits(log: &DealLog, settings: Settings) -> (GameResult, [u32; 2], Vec<String>) {
    let mut t = [0u32; 2];
    for (w, cards, _) in &log.won {
        t[w.index()] += cards.iter().map(|&c| thirds(c)).sum::<u32>();
    }
    assert_eq!(log.won.len(), 27);
    let (simple, wins, loses) = match settings.values {
        Values::V234 => (2, 3, 4),
        Values::V123 => (1, 2, 3),
        Values::V124 => (1, 2, 4),
    };
    let mut out: Vec<Credit> = Vec::new();
    let tricks = |s: Seat| log.won.iter().filter(|(w, _, _)| *w == s).count();
    let valat = Seat::BOTH.into_iter().find(|&s| tricks(s) == 27);
    let result = match (log.declarer, valat) {
        (Some(d), Some(v)) => {
            if d == v {
                GameResult::DeclarerWin
            } else {
                GameResult::DeclarerLoss
            }
        }
        (None, Some(_)) => GameResult::SimpleWin,
        (Some(d), None) => {
            if t[d.index()] * 3 >= 321 {
                // 107 thirds; written as 35 2/3 points = 107/3
                out.push(Credit {
                    seat: d,
                    cause: Cause::Game,
                    points: wins,
                });
                GameResult::DeclarerWin
            } else {
                out.push(Credit {
                    seat: d.other(),
                    cause: Cause::Game,
                    points: loses,
                });
                GameResult::DeclarerLoss
            }
        }
        (None, None) => match Seat::BOTH.into_iter().find(|s| t[s.index()] >= 107) {
            Some(w) => {
                out.push(Credit {
                    seat: w,
                    cause: Cause::Game,
                    points: simple,
                });
                GameResult::SimpleWin
            }
            None => GameResult::Draw,
        },
    };
    if let Some(v) = valat {
        out.push(Credit {
            seat: v,
            cause: Cause::Valat,
            points: 12,
        });
    }
    for &(seat, h) in &log.announced {
        out.push(Credit {
            seat,
            cause: match h {
                Honour::Trull => Cause::Trull,
                Honour::Kings => Cause::Kings,
            },
            points: 1,
        });
    }
    // Pagat ultimo: T1 played in trick 27
    if let Some(&(seat, _, _)) = log
        .played
        .iter()
        .find(|(_, c, n)| *c == Card::PAGAT && *n == 27)
    {
        let winner = log.won.last().unwrap().0;
        if winner == seat {
            out.push(Credit {
                seat,
                cause: Cause::UltimoWon,
                points: settings.ultimo,
            });
        } else {
            out.push(Credit {
                seat: seat.other(),
                cause: Cause::UltimoLost,
                points: settings.ultimo,
            });
        }
    }
    for c in &mut out {
        c.points *= log.multiplier;
    }
    let mut keys: Vec<String> = out.iter().map(key).collect();
    keys.sort();
    (result, t, keys)
}

#[test]
fn rules_8_to_12_the_engine_scores_every_played_deal_as_an_independent_reading_does() {
    let (mut checked, mut honours, mut ultimos) = (0, 0, 0);
    for seed in 0..150u64 {
        let mut s = sitting_for(seed);
        let held = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        // Both seats share the hint cell; it is refreshed before each move.
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(AnnouncingPlayer {
                rng: Rng::seeded(seed),
                sitting_hint: held.clone(),
            }),
            Box::new(AnnouncingPlayer {
                rng: Rng::seeded(seed + 1000),
                sitting_hint: held.clone(),
            }),
        ];
        while let Some(seat) = s.to_move() {
            if let Some(d) = s.deal() {
                *held.borrow_mut() = d.legal_announcements(seat);
            }
            let v = view(&s, seat);
            let a = players[seat.index()].choose(&v).unwrap();
            s.apply(seat, a).unwrap();
        }
        let mut log = DealLog::default();
        let mut contos = [0u32; 2];
        for e in s.events() {
            match e {
                Event::DealStart { multiplier, .. } => {
                    log = DealLog {
                        multiplier: *multiplier,
                        ..DealLog::default()
                    }
                }
                Event::Bid { seat } => log.declarer = Some(*seat),
                Event::Announcement { seat, honour, .. } => log.announced.push((*seat, *honour)),
                Event::CardPlayed {
                    seat, card, trick, ..
                } => log.played.push((*seat, *card, *trick)),
                Event::TrickWon {
                    winner,
                    cards,
                    trick,
                } => log.won.push((*winner, *cards, *trick)),
                Event::GameScored {
                    result,
                    credits,
                    multiplier,
                    contos: c,
                } => {
                    let (want_result, want_thirds, want) = expected_credits(&log, s.settings());
                    assert_eq!(*result, want_result, "seed {seed}");
                    assert_eq!(*multiplier, log.multiplier);
                    let mut got: Vec<String> = credits.iter().map(key).collect();
                    got.sort();
                    assert_eq!(got, want, "seed {seed}");
                    for cr in credits {
                        contos[cr.seat.index()] += cr.points;
                    }
                    assert_eq!(*c, contos);
                    // the count event agrees with the takes
                    checked += 1;
                    honours += log.announced.len();
                    ultimos += usize::from(got.iter().any(|k| k.contains("Ultimo")));
                    assert_eq!(want_thirds[0] + want_thirds[1], 210);
                }
                Event::Count { thirds: t, .. } => {
                    let (_, want, _) = expected_credits(&log, s.settings());
                    assert_eq!(*t, want, "seed {seed}");
                }
                _ => {}
            }
        }
    }
    assert!(checked > 400);
    assert!(honours > 20, "announcements must be exercised: {honours}");
    assert!(ultimos > 5, "ultimo must be exercised: {ultimos}");
}

// ---------------------------------------------------------------------------
// A reference for the legal plays (RULES 5.2 to 5.4), from the view alone
// ---------------------------------------------------------------------------

fn reference_legal(v: &strohmandeln_core::view::View) -> Vec<Card> {
    let d = v.deal.as_ref().unwrap();
    let mut set: Vec<Card> = d.own_hand.iter().map(|h| h.card).collect();
    set.extend(d.covers[v.seat.index()].iter().flatten().copied());
    if let Some(lead) = &d.lead {
        let same = |c: Card| c.suit() == lead.card.suit();
        let following: Vec<Card> = set.iter().copied().filter(|&c| same(c)).collect();
        if !following.is_empty() {
            set = following;
        } else if !lead.card.is_tarock() {
            let trumps: Vec<Card> = set.iter().copied().filter(|c| c.is_tarock()).collect();
            if !trumps.is_empty() {
                set = trumps;
            }
        }
    }
    set.sort();
    set
}

#[test]
fn rules_5_2_to_5_4_legal_plays_equal_an_independent_reading_at_every_step() {
    let mut steps = 0;
    for seed in 0..80u64 {
        let mut s = sitting_for(seed);
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(RandomPlayer::new(seed, Seat::A)),
            Box::new(RandomPlayer::new(seed, Seat::B)),
        ];
        while let Some(seat) = s.to_move() {
            let v = view(&s, seat);
            if v.deal.as_ref().is_some_and(|d| d.playing) && !v.legal_actions.is_empty() {
                let mut got: Vec<Card> = v
                    .legal_actions
                    .iter()
                    .filter_map(|a| match a {
                        Action::Play { card, .. } => Some(*card),
                        _ => None,
                    })
                    .collect();
                got.sort();
                assert_eq!(got, reference_legal(&v), "seed {seed}");
                // a deadlock would show as no legal play
                assert!(!got.is_empty());
                steps += 1;
            }
            let a = players[seat.index()].choose(&v).unwrap();
            s.apply(seat, a).unwrap();
        }
    }
    assert!(steps > 10_000);
}

// ---------------------------------------------------------------------------
// Conservation of cards (RULES 2, 4, 5.6)
// ---------------------------------------------------------------------------

#[test]
fn rules_2_to_6_every_played_deal_plays_each_card_once_27_per_seat() {
    for seed in 0..60u64 {
        let mut s = sitting_for(seed);
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(RandomPlayer::new(seed, Seat::A)),
            Box::new(RandomPlayer::new(seed, Seat::B)),
        ];
        play_out(&mut s, &mut players, 200_000).unwrap();
        let mut per_deal: Vec<(HashSet<Card>, [u32; 2])> = vec![(HashSet::new(), [0, 0])];
        for e in s.events() {
            match e {
                Event::CardPlayed { seat, card, .. } => {
                    let cur = per_deal.last_mut().unwrap();
                    assert!(cur.0.insert(*card), "seed {seed}: {card:?} played twice");
                    cur.1[seat.index()] += 1;
                }
                Event::Count { .. } => {
                    let cur = per_deal.last().unwrap();
                    assert_eq!(cur.0.len(), 54);
                    assert_eq!(cur.1, [27, 27]);
                    per_deal.push((HashSet::new(), [0, 0]));
                }
                _ => {}
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Fuzz: arbitrary actions never panic, refusals change nothing and are keys
// ---------------------------------------------------------------------------

fn random_action(rng: &mut Rng) -> Action {
    let honour = |rng: &mut Rng| {
        if rng.below(2) == 0 {
            Honour::Trull
        } else {
            Honour::Kings
        }
    };
    match rng.below(6) {
        0 => Action::Take,
        1 => Action::Pass,
        2 => Action::Announce {
            honour: honour(rng),
        },
        3 => Action::Extend {
            accept: rng.below(2) == 0,
        },
        _ => {
            let n = rng.below(3) as usize;
            let announce = (0..n).map(|_| honour(rng)).collect();
            Action::Play {
                card: Card::from_index(rng.below(54) as u8).unwrap(),
                announce,
            }
        }
    }
}

fn snapshot(s: &Sitting) -> String {
    // The whole deal state (hidden cards included) through Debug.
    format!(
        "{:?}|{:?}|{}|{}|{:?}",
        s.deal(),
        s.contos(),
        s.events().len(),
        s.record().moves.len(),
        s.to_move()
    )
}

#[test]
fn arbitrary_actions_never_panic_and_a_refusal_changes_nothing() {
    let mut refusals = 0;
    let mut kinds = HashSet::new();
    for seed in 0..30u64 {
        let mut s = sitting_for(seed);
        let mut rng = Rng::seeded(seed ^ 0xfeed);
        for _ in 0..3000 {
            if s.is_over() {
                break;
            }
            let seat = if rng.below(10) == 0 {
                s.to_move().unwrap().other()
            } else {
                s.to_move().unwrap()
            };
            let action = if rng.below(10) < 6 {
                let legal = s.legal_actions();
                legal[rng.below(legal.len() as u64) as usize].clone()
            } else {
                random_action(&mut rng)
            };
            let before = snapshot(&s);
            match s.apply(seat, action) {
                Ok(_) => {}
                Err(r) => {
                    assert_eq!(before, snapshot(&s), "seed {seed}: refusal changed state");
                    // the wire form is a key and parameters, never prose
                    let j = serde_json::to_value(&r).unwrap();
                    let obj = j.as_object().unwrap();
                    assert!(obj.contains_key("key"));
                    for (k, val) in obj {
                        assert!(
                            matches!(
                                k.as_str(),
                                "key"
                                    | "card"
                                    | "led"
                                    | "allowed"
                                    | "missing"
                                    | "honour"
                                    | "speaker"
                            ),
                            "{k}"
                        );
                        let mut strings = Vec::new();
                        collect_strings(val, &mut strings);
                        for st in strings {
                            assert!(
                                !st.contains(' ') && st.len() <= 24,
                                "prose on the wire: {st:?}"
                            );
                        }
                    }
                    kinds.insert(obj["key"].as_str().unwrap().to_string());
                    refusals += 1;
                }
            }
        }
    }
    assert!(refusals > 2000);
    for k in [
        "not-your-turn",
        "wrong-phase",
        "not-playable",
        "must-follow-suit",
        "must-trump",
        "not-held",
        "already-announced",
        "honour-forfeited",
        "duplicate-announcement",
    ] {
        assert!(kinds.contains(k), "the fuzz never produced {k}");
    }
}

fn collect_strings(v: &serde_json::Value, out: &mut Vec<String>) {
    match v {
        serde_json::Value::String(s) => out.push(s.clone()),
        serde_json::Value::Array(a) => a.iter().for_each(|x| collect_strings(x, out)),
        serde_json::Value::Object(o) => o.values().for_each(|x| collect_strings(x, out)),
        _ => {}
    }
}

// ---------------------------------------------------------------------------
// Leaks: the events a player is sent
// ---------------------------------------------------------------------------

fn cards_in(v: &serde_json::Value, out: &mut HashSet<Card>) {
    let mut strings = Vec::new();
    collect_strings(v, &mut strings);
    out.extend(strings.iter().filter_map(|s| Card::parse(s)));
}

/// The cards `seat` must not know now: the opponent's unannounced dealt cards
/// still in hand, and every face-down packet card.
fn hidden_from(s: &Sitting, seat: Seat) -> HashSet<Card> {
    let mut hidden = HashSet::new();
    let Some(deal) = s.deal() else {
        return hidden;
    };
    let opp = seat.other();
    let announced: Vec<Card> = [Honour::Trull, Honour::Kings]
        .into_iter()
        .filter(|&h| deal.honour_state(opp, h) == strohmandeln_core::deal::HonourState::Announced)
        .flat_map(|h| h.cards())
        .collect();
    for h in deal.hand(opp) {
        if !h.public && !announced.contains(&h.card) {
            hidden.insert(h.card);
        }
    }
    for owner in Seat::BOTH {
        for p in 1..=3 {
            let skip = usize::from(deal.cover(owner, p).is_some());
            hidden.extend(deal.packet_cards(owner, p).into_iter().skip(skip));
        }
    }
    hidden
}

#[test]
fn rules_13_the_events_returned_by_apply_leak_nothing_once_filtered() {
    let mut seen_dealt = 0;
    for seed in 0..10u64 {
        let mut s = sitting_for(seed);
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(RandomPlayer::new(seed, Seat::A)),
            Box::new(RandomPlayer::new(seed, Seat::B)),
        ];
        while let Some(mover) = s.to_move() {
            let v = view(&s, mover);
            let a = players[mover.index()].choose(&v).unwrap();
            let events = s.apply(mover, a).unwrap();
            // Events of a deal that just ended mention cards that the next
            // deal has shuffled again: look only at the newest deal's events.
            let from = events
                .iter()
                .rposition(|e| matches!(e, Event::DealStart { .. }))
                .unwrap_or(0);
            let events = &events[from..];
            for viewer in Seat::BOTH {
                let hidden = hidden_from(&s, viewer);
                let mut seen = HashSet::new();
                for e in events.iter().filter_map(|e| filter_event(e, viewer)) {
                    if matches!(e, Event::Dealt { .. }) {
                        seen_dealt += 1;
                    }
                    cards_in(&serde_json::to_value(&e).unwrap(), &mut seen);
                }
                let leaked: Vec<_> = seen.intersection(&hidden).collect();
                assert!(leaked.is_empty(), "seed {seed} {viewer:?}: {leaked:?}");
                // and the view built right after
                let mut seen = HashSet::new();
                cards_in(&serde_json::to_value(view(&s, viewer)).unwrap(), &mut seen);
                assert!(seen.intersection(&hidden).next().is_none());
            }
        }
    }
    assert!(seen_dealt > 50);
}

#[test]
fn rules_13_3_a_view_gives_no_more_than_the_public_record_and_own_hand() {
    // Everything in a view's cards is own hand, own Dealt, a played or turned
    // card, a cover, or a known opponent card: build that set from the events
    // and compare, for every step.
    for seed in 0..3u64 {
        let mut s = sitting_for(seed);
        let mut players: [Box<dyn Player>; 2] = [
            Box::new(RandomPlayer::new(seed, Seat::A)),
            Box::new(RandomPlayer::new(seed, Seat::B)),
        ];
        while let Some(mover) = s.to_move() {
            for viewer in Seat::BOTH {
                let v = view(&s, viewer);
                let Some(d) = &v.deal else { continue };
                let mut allowed: HashSet<Card> = d.own_hand.iter().map(|h| h.card).collect();
                allowed.extend(d.opponent_known.iter().copied());
                allowed.extend(d.covers.iter().flatten().flatten().copied());
                for t in &d.tricks {
                    allowed.extend(t.cards);
                }
                if let Some(l) = &d.lead {
                    allowed.insert(l.card);
                }
                // own dealt cards that were played already are in tricks/lead
                let mut seen = HashSet::new();
                cards_in(&serde_json::to_value(d).unwrap(), &mut seen);
                for e in &v.events {
                    match e {
                        Event::Dealt { seat, cards } => {
                            assert_eq!(*seat, viewer);
                            allowed.extend(cards.iter().copied());
                        }
                        Event::PacketTurned { card, .. } | Event::CardPlayed { card, .. } => {
                            allowed.insert(*card);
                        }
                        _ => {}
                    }
                }
                let mut seen_all = seen;
                cards_in(&serde_json::to_value(&v.events).unwrap(), &mut seen_all);
                let extra: Vec<_> = seen_all.difference(&allowed).collect();
                assert!(extra.is_empty(), "seed {seed}: {extra:?}");
            }
            let v = view(&s, mover);
            let a = players[mover.index()].choose(&v).unwrap();
            s.apply(mover, a).unwrap();
        }
    }
}

// ---------------------------------------------------------------------------
// Determinism (RULES 2.1, record)
// ---------------------------------------------------------------------------

#[test]
fn the_cards_of_a_deal_do_not_depend_on_how_earlier_deals_were_played() {
    // Two sittings of one seed, played by different agents, deal the same
    // cards in the same order (the shuffle has its own stream).
    for seed in 0..20u64 {
        let dealt = |agent_seed: u64| {
            let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
            let mut players: [Box<dyn Player>; 2] = [
                Box::new(RandomPlayer::new(agent_seed, Seat::A)),
                Box::new(RandomPlayer::new(agent_seed, Seat::B)),
            ];
            play_out(&mut s, &mut players, 200_000).unwrap();
            // void deals may differ in count only if the cards differ: compare
            // the sequence of Dealt events
            let mut by_seat: BTreeMap<usize, Vec<Vec<Card>>> = BTreeMap::new();
            for e in s.events() {
                if let Event::Dealt { seat, cards } = e {
                    by_seat.entry(seat.index()).or_default().push(cards.clone());
                }
            }
            by_seat
        };
        let a = dealt(1);
        let b = dealt(2);
        // the first deals match; later ones too, as far as both go
        for seat in 0..2 {
            let n = a[&seat].len().min(b[&seat].len());
            assert!(n >= 6);
            assert_eq!(a[&seat][..n], b[&seat][..n], "seed {seed}");
        }
    }
}
