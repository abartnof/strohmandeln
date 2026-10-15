//! The end-of-deal review: what a deal showed, as message keys with
//! parameters (never prose: the table translates them, `docs/MESSAGES.md`
//! "Review keys"). It reads the human's filtered events of one deal (the
//! public log, `docs/RULES.md` 13.6, with the human's own `dealt`), so it
//! can say nothing the human's view does not hold. After the last trick every
//! card is in the log, the opponent's included, and all of it is public.
//!
//! A review is two bullets, each with a lead: `count` (the count against the
//! 107-third line) and `decided` (the moment that moved the deal most). It
//! notes what happened; it does not score the play. What to work on is the
//! brief's (`brief.rs`, `docs/TUTOR.md` 7).

use strohmandeln_core::cards::Card;
use strohmandeln_core::types::{Cause, Event, GameResult, Seat, UltimoResult};

/// A parameter of a bullet's key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Param {
    Num(i64),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bullet {
    /// `count`, `decided`, `sitting` or (an explanation) `explain`.
    pub lead: &'static str,
    /// A message key from [`KEYS`].
    pub key: &'static str,
    pub params: Vec<(&'static str, Param)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Review {
    /// The deal's number, or `0` for a sitting.
    pub deal: u32,
    pub bullets: Vec<Bullet>,
}

/// Every key a review can carry.
pub const KEYS: &[&str] = &[
    "count",
    "sitting-tally",
    "ultimo-won-you",
    "ultimo-lost-you",
    "ultimo-won-opp",
    "ultimo-lost-opp",
    "mond-caught",
    "mond-lost",
    "trick-lost-rich",
    "trick-won-rich",
    "valat-you",
    "valat-opp",
    "level",
];

/// A trick is rich from 9 value points between its two cards (a king and a
/// queen), which is 23 thirds (RULES 8.1: `3 x value - 2` a card).
pub const RICH_THIRDS: u32 = 23;

struct Trick {
    n: u32,
    plays: Vec<(Seat, Card)>,
    winner: Option<Seat>,
}

impl Trick {
    fn thirds(&self) -> u32 {
        self.plays.iter().map(|(_, c)| c.thirds()).sum()
    }
}

/// What one scored deal's events show.
struct Deal {
    number: u32,
    tricks: Vec<Trick>,
    ultimo: Option<(Seat, UltimoResult)>,
    count: Option<[u32; 2]>,
    /// Who took all 27 tricks (RULES 10).
    valat: Option<Seat>,
    result: GameResult,
    /// The human's net Points of the deal, all causes.
    net: i32,
    /// Who won the game or the Valat.
    winner: Option<Seat>,
}

fn read(events: &[Event], human: Seat) -> Option<Deal> {
    let mut d = Deal {
        number: 0,
        tricks: Vec::new(),
        ultimo: None,
        count: None,
        valat: None,
        result: GameResult::Draw,
        net: 0,
        winner: None,
    };
    let mut scored = false;
    for e in events {
        match e {
            Event::DealStart { deal_number, .. } => d.number = *deal_number,
            Event::CardPlayed {
                seat, card, trick, ..
            } => {
                if d.tricks.last().map(|t| t.n) != Some(*trick) {
                    d.tricks.push(Trick {
                        n: *trick,
                        plays: Vec::new(),
                        winner: None,
                    });
                }
                if let Some(t) = d.tricks.last_mut() {
                    t.plays.push((*seat, *card));
                }
            }
            Event::TrickWon { winner, trick, .. } => {
                if let Some(t) = d.tricks.iter_mut().find(|t| t.n == *trick) {
                    t.winner = Some(*winner);
                }
            }
            Event::Valat { seat } => d.valat = Some(*seat),
            Event::PagatUltimo { seat, result } => d.ultimo = Some((*seat, *result)),
            Event::Count { thirds, .. } => d.count = Some(*thirds),
            Event::GameScored {
                result, credits, ..
            } => {
                scored = true;
                d.result = *result;
                for cr in credits {
                    let p = cr.points as i32;
                    d.net += if cr.seat == human { p } else { -p };
                    if matches!(cr.cause, Cause::Game | Cause::Valat) {
                        d.winner = Some(cr.seat);
                    }
                }
            }
            _ => {}
        }
    }
    scored.then_some(d)
}

fn num(name: &'static str, n: impl Into<i64>) -> (&'static str, Param) {
    (name, Param::Num(n.into()))
}
fn text(name: &'static str, s: impl Into<String>) -> (&'static str, Param) {
    (name, Param::Text(s.into()))
}

/// The moment that moved the deal most: a Valat, the Pagat ultimo, a caught
/// Mond, the richest trick lost, the richest trick won; else a level deal.
fn decided(d: &Deal, human: Seat) -> Bullet {
    let bullet = |key, params| Bullet {
        lead: "decided",
        key,
        params,
    };
    if let Some(seat) = d.valat {
        let key = if seat == human {
            "valat-you"
        } else {
            "valat-opp"
        };
        return bullet(key, Vec::new());
    }
    if let Some((seat, result)) = d.ultimo {
        let key = match (result, seat == human) {
            (UltimoResult::Won, true) => "ultimo-won-you",
            (UltimoResult::Lost, true) => "ultimo-lost-you",
            (UltimoResult::Won, false) => "ultimo-won-opp",
            (UltimoResult::Lost, false) => "ultimo-lost-opp",
        };
        return bullet(key, Vec::new());
    }
    for t in &d.tricks {
        let sk = t.plays.iter().find(|(_, c)| *c == Card::SKUS);
        let has_mond = t.plays.iter().any(|(_, c)| *c == Card::MOND);
        if let (Some((sk_seat, _)), true) = (sk, has_mond) {
            if Some(*sk_seat) == t.winner {
                let key = if *sk_seat == human {
                    "mond-caught"
                } else {
                    "mond-lost"
                };
                return bullet(key, vec![num("trick", t.n)]);
            }
        }
    }
    let richest = |won: bool| {
        d.tricks
            .iter()
            .filter(|t| t.winner.is_some_and(|w| (w == human) == won) && t.thirds() >= RICH_THIRDS)
            .fold(None::<&Trick>, |best, t| match best {
                Some(b) if b.thirds() >= t.thirds() => Some(b),
                _ => Some(t),
            })
    };
    for (won, key) in [(false, "trick-lost-rich"), (true, "trick-won-rich")] {
        if let Some(t) = richest(won) {
            let card_of = |mine: bool| {
                t.plays
                    .iter()
                    .find(|(s, _)| (*s == human) == mine)
                    .map(|(_, c)| c.code())
                    .unwrap_or_default()
            };
            return bullet(
                key,
                vec![
                    num("trick", t.n),
                    text("mine", card_of(true)),
                    text("theirs", card_of(false)),
                    num("thirds", t.thirds()),
                ],
            );
        }
    }
    bullet("level", Vec::new())
}

fn deal_review(d: &Deal, human: Seat) -> Review {
    let mut bullets = Vec::new();
    if let Some(thirds) = d.count {
        bullets.push(Bullet {
            lead: "count",
            key: "count",
            params: vec![
                num("mine", thirds[human.index()]),
                num("theirs", thirds[human.other().index()]),
            ],
        });
    }
    bullets.push(decided(d, human));
    Review {
        deal: d.number,
        bullets,
    }
}

// ---- the explanation of a trick ------------------------------------------

/// Keys of an explanation: what a trick showed, from the rules of following
/// (RULES 5.4: the suit if the playable set holds it, else a tarock, else any
/// card; the playable set is the hand and the covers, never a face-down
/// packet card, RULES 5.2).
pub const EXPLAIN_KEYS: &[&str] = &[
    "explain-trump-you",
    "explain-trump-opp",
    "explain-void-you",
    "explain-void-opp",
    "explain-notarock-you",
    "explain-notarock-opp",
    "explain-won-you",
    "explain-won-opp",
];

/// What the last trick in `events` (the human's filtered stream) showed, as
/// one bullet with the lead `explain`, or `None` before a trick is won. A
/// follower who trumps a suit led had none of that suit to play; one who
/// plays another suit had neither it nor a tarock; one who plays a suit card
/// to a tarock led had no tarock (RULES 5.4). Otherwise it names who won with
/// what.
pub fn explain_last(events: &[Event], human: Seat) -> Option<Bullet> {
    let end = events
        .iter()
        .rposition(|e| matches!(e, Event::TrickWon { .. }))?;
    let Event::TrickWon { winner, trick, .. } = &events[end] else {
        return None;
    };
    // The two cards just before the win (trick numbers repeat from deal to deal).
    let mut plays: Vec<(Seat, Card)> = events[..end]
        .iter()
        .rev()
        .filter_map(|e| match e {
            Event::CardPlayed {
                seat,
                card,
                trick: t,
                ..
            } if t == trick => Some((*seat, *card)),
            _ => None,
        })
        .take(2)
        .collect();
    plays.reverse();
    let [(leader, lead), (follower, follow)] = plays[..] else {
        return None;
    };
    let bullet = |key: &'static str, params| Bullet {
        lead: "explain",
        key,
        params,
    };
    let you = follower == human;
    if lead.is_tarock() {
        if !follow.is_tarock() {
            let key = if you {
                "explain-notarock-you"
            } else {
                "explain-notarock-opp"
            };
            return Some(bullet(key, Vec::new()));
        }
    } else if follow.suit() != lead.suit() {
        let suit = text(
            "suit",
            lead.suit()
                .map(|s| s.letter().to_string())
                .unwrap_or_default(),
        );
        let key = match (follow.is_tarock(), you) {
            (true, true) => "explain-trump-you",
            (true, false) => "explain-trump-opp",
            (false, true) => "explain-void-you",
            (false, false) => "explain-void-opp",
        };
        return Some(bullet(key, vec![suit]));
    }
    let (win, lose) = if *winner == leader {
        (lead, follow)
    } else {
        (follow, lead)
    };
    let key = if *winner == human {
        "explain-won-you"
    } else {
        "explain-won-opp"
    };
    Some(bullet(
        key,
        vec![text("winner", win.code()), text("loser", lose.code())],
    ))
}

/// The review of one deal's events, `DealStart` to `GameScored`.
pub fn review_deal(events: &[Event], human: Seat) -> Option<Review> {
    read(events, human).map(|d| deal_review(&d, human))
}

/// The review of the last scored deal in `events` (the human's filtered
/// stream), or `None` when no deal was scored (a void deal, a throw-in).
pub fn review_last(events: &[Event], human: Seat) -> Option<Review> {
    let end = events
        .iter()
        .rposition(|e| matches!(e, Event::GameScored { .. }))?;
    let start = events[..end]
        .iter()
        .rposition(|e| matches!(e, Event::DealStart { .. }))
        .unwrap_or(0);
    review_deal(&events[start..=end], human)
}

/// The review of every scored deal in `events`: the tally, and the deal that
/// went worst (else best) with what it showed.
pub fn review_sitting(events: &[Event], human: Seat) -> Option<Review> {
    let starts: Vec<usize> = events
        .iter()
        .enumerate()
        .filter(|(_, e)| matches!(e, Event::DealStart { .. }))
        .map(|(i, _)| i)
        .collect();
    let deals: Vec<Deal> = starts
        .iter()
        .enumerate()
        .filter_map(|(k, &from)| {
            let to = starts.get(k + 1).copied().unwrap_or(events.len());
            read(&events[from..to], human)
        })
        .collect();
    if deals.is_empty() {
        return None;
    }
    let drawn = deals
        .iter()
        .filter(|d| d.result == GameResult::Draw)
        .count();
    let won = deals
        .iter()
        .filter(|d| d.result != GameResult::Draw && d.winner == Some(human))
        .count();
    let lost = deals.len() - drawn - won;
    let mut pick = &deals[0];
    for d in &deals {
        // The worst deal, the first of equals; with none lost, the best.
        if d.net < pick.net {
            pick = d;
        }
    }
    if pick.net >= 0 {
        for d in &deals {
            if d.net > pick.net {
                pick = d;
            }
        }
    }
    let mut moment = decided(pick, human);
    moment.params.push(num("deal", pick.number));
    let net: i32 = deals.iter().map(|d| d.net).sum();
    Some(Review {
        deal: 0,
        bullets: vec![
            Bullet {
                lead: "sitting",
                key: "sitting-tally",
                params: vec![
                    num("won", won as i64),
                    num("lost", lost as i64),
                    num("drawn", drawn as i64),
                    num("net", net),
                ],
            },
            moment,
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use strohmandeln_core::types::{Credit, Source};

    const ME: Seat = Seat::A;
    const HE: Seat = Seat::B;

    fn c(code: &str) -> Card {
        Card::parse(code).unwrap_or_else(|| panic!("bad card {code}"))
    }

    /// A deal's events built trick by trick. The deal starts with `dealt` for
    /// ME.
    struct Build {
        ev: Vec<Event>,
        n: u32,
    }

    impl Build {
        fn new(deal_number: u32, mine: &[&str]) -> Build {
            let ev = vec![
                Event::DealStart {
                    deal_number,
                    tour: 1,
                    dealer: HE,
                    forehand: ME,
                    multiplier: 1,
                },
                Event::Dealt {
                    seat: ME,
                    cards: mine.iter().map(|x| c(x)).collect(),
                },
            ];
            Build { ev, n: 0 }
        }
        fn trick(&mut self, leader: Seat, lead: &str, follow: &str, winner: Seat) -> &mut Build {
            self.n += 1;
            self.ev.push(Event::CardPlayed {
                seat: leader,
                card: c(lead),
                source: Source::Hand,
                trick: self.n,
            });
            self.ev.push(Event::CardPlayed {
                seat: leader.other(),
                card: c(follow),
                source: Source::Hand,
                trick: self.n,
            });
            self.ev.push(Event::TrickWon {
                winner,
                cards: [c(lead), c(follow)],
                trick: self.n,
            });
            self
        }
        fn push(&mut self, e: Event) -> &mut Build {
            self.ev.push(e);
            self
        }
        fn end(&mut self, mine: u32, theirs: u32, game_to: Seat) -> Vec<Event> {
            let mut thirds = [0; 2];
            thirds[ME.index()] = mine;
            thirds[HE.index()] = theirs;
            self.ev.push(Event::Count {
                thirds,
                declarer: Some(game_to),
                threshold: 107,
            });
            self.ev.push(Event::GameScored {
                result: GameResult::DeclarerWin,
                credits: vec![Credit {
                    seat: game_to,
                    cause: Cause::Game,
                    points: 3,
                }],
                multiplier: 1,
                contos: [0, 0],
            });
            self.ev.clone()
        }
    }

    fn lead_keys(r: &Review) -> Vec<(&'static str, &'static str)> {
        r.bullets.iter().map(|b| (b.lead, b.key)).collect()
    }
    fn num(b: &Bullet, name: &str) -> i64 {
        match b.params.iter().find(|(k, _)| *k == name).map(|(_, v)| v) {
            Some(Param::Num(n)) => *n,
            other => panic!("{name}: {other:?}"),
        }
    }
    fn text(b: &Bullet, name: &str) -> String {
        match b.params.iter().find(|(k, _)| *k == name).map(|(_, v)| v) {
            Some(Param::Text(s)) => s.clone(),
            other => panic!("{name}: {other:?}"),
        }
    }

    #[test]
    fn a_deal_with_no_score_has_no_review() {
        let mut b = Build::new(1, &["T5"]);
        b.push(Event::ThrownIn);
        assert_eq!(review_last(&b.ev, ME), None);
        assert_eq!(review_deal(&b.ev, ME), None);
        assert_eq!(review_sitting(&b.ev, ME), None);
    }

    #[test]
    fn the_count_bullet_reports_both_takes_in_thirds() {
        let ev = Build::new(1, &["T5"]).end(120, 90, ME);
        let r = review_last(&ev, ME).unwrap();
        assert_eq!(r.deal, 1);
        let b = &r.bullets[0];
        assert_eq!((b.lead, b.key), ("count", "count"));
        assert_eq!((num(b, "mine"), num(b, "theirs")), (120, 90));
    }

    #[test]
    fn a_pagat_ultimo_is_the_deciding_moment() {
        let mut b = Build::new(2, &["T1"]);
        b.trick(ME, "T1", "T2", HE).push(Event::PagatUltimo {
            seat: HE,
            result: UltimoResult::Won,
        });
        let ev = b.end(100, 110, HE);
        let r = review_deal(&ev, ME).unwrap();
        assert_eq!(r.bullets[1].lead, "decided");
        assert_eq!(r.bullets[1].key, "ultimo-won-opp");
        let mut b = Build::new(2, &["T1"]);
        b.push(Event::PagatUltimo {
            seat: ME,
            result: UltimoResult::Lost,
        });
        let r = review_deal(&b.end(100, 110, HE), ME).unwrap();
        assert_eq!(r.bullets[1].key, "ultimo-lost-you");
    }

    #[test]
    fn a_mond_caught_by_the_skus_is_named_for_whoever_lost_it() {
        let mut b = Build::new(1, &["T21", "T3"]);
        b.trick(ME, "T21", "SK", HE);
        let r = review_deal(&b.end(100, 110, HE), ME).unwrap();
        assert_eq!(r.bullets[1].key, "mond-lost");
        assert_eq!(num(&r.bullets[1], "trick"), 1);
        let mut b = Build::new(1, &["SK"]);
        b.trick(HE, "T21", "SK", ME);
        let r = review_deal(&b.end(110, 100, ME), ME).unwrap();
        assert_eq!(r.bullets[1].key, "mond-caught");
    }

    #[test]
    fn the_richest_trick_the_human_lost_is_named_with_both_cards() {
        let mut b = Build::new(1, &["Kh", "Qs"]);
        b.trick(ME, "Qs", "Ks", HE) // 4 + 5 = 9: rich
            .trick(ME, "Kh", "T4", HE); // 5 + 1 = 6: not
        let r = review_deal(&b.end(100, 110, HE), ME).unwrap();
        let d = &r.bullets[1];
        assert_eq!(d.key, "trick-lost-rich");
        assert_eq!(num(d, "trick"), 1);
        assert_eq!(
            (text(d, "mine").as_str(), text(d, "theirs").as_str()),
            ("Qs", "Ks")
        );
        assert_eq!(num(d, "thirds"), 10 + 13);
    }

    #[test]
    fn without_a_lost_rich_trick_the_richest_won_is_named() {
        let mut b = Build::new(1, &["Kh"]);
        b.trick(ME, "Kh", "Qh", ME);
        let r = review_deal(&b.end(110, 100, ME), ME).unwrap();
        assert_eq!(r.bullets[1].key, "trick-won-rich");
    }

    #[test]
    fn a_level_deal_says_so() {
        let mut b = Build::new(1, &["T4"]);
        b.trick(ME, "T4", "T3", ME);
        let r = review_deal(&b.end(110, 100, ME), ME).unwrap();
        assert_eq!(r.bullets[1].key, "level");
    }

    #[test]
    fn a_valat_is_the_deciding_moment_for_whoever_took_every_trick() {
        let mut b = Build::new(1, &["T4"]);
        b.trick(ME, "T4", "T3", ME).push(Event::Valat { seat: ME });
        let ev = b.end(210, 0, ME);
        let r = review_deal(&ev, ME).unwrap();
        assert_eq!(lead_keys(&r)[1], ("decided", "valat-you"));
        let r = review_deal(&ev, HE).unwrap();
        assert_eq!(r.bullets[1].key, "valat-opp");
    }

    #[test]
    fn a_review_has_no_advice_in_it() {
        let mut b = Build::new(1, &["T1", "T9", "SK", "T21"]);
        b.trick(ME, "T1", "T2", HE);
        let r = review_deal(&b.end(100, 110, HE), ME).unwrap();
        assert_eq!(r.bullets.len(), 2);
        assert!(r.bullets.iter().all(|b| b.lead != "try"));
        assert!(KEYS.iter().all(|k| !k.starts_with("try-")));
    }

    #[test]
    fn the_review_reads_the_same_from_the_other_chair() {
        let mut b = Build::new(1, &["T4"]);
        b.trick(ME, "T4", "T3", ME);
        let ev = b.end(110, 100, ME);
        // From HE's chair the other seat's `dealt` is not in the stream: the
        // review still works, from the public log.
        let from_he: Vec<Event> = ev
            .iter()
            .filter(|e| !matches!(e, Event::Dealt { .. }))
            .cloned()
            .collect();
        let r = review_deal(&from_he, HE).unwrap();
        let b0 = &r.bullets[0];
        assert_eq!((num(b0, "mine"), num(b0, "theirs")), (100, 110));
    }

    /// Deals as (number, who won the game, the human's cards, human's thirds).
    fn sitting(deals: &[(u32, Seat, &[&str], u32)]) -> Vec<Event> {
        let mut all = Vec::new();
        for &(n, winner, mine, my_thirds) in deals {
            let mut b = Build::new(n, mine);
            all.extend(b.end(my_thirds, 210 - my_thirds, winner));
        }
        all
    }

    #[test]
    fn a_sitting_review_tallies_the_deals_and_names_the_worst() {
        let ev = sitting(&[
            (1, ME, &["T4"], 120),
            (2, HE, &["T4"], 90),
            (3, ME, &["T4"], 130),
        ]);
        let r = review_sitting(&ev, ME).unwrap();
        assert_eq!(r.deal, 0);
        assert_eq!(lead_keys(&r)[0], ("sitting", "sitting-tally"));
        let t = &r.bullets[0];
        assert_eq!((num(t, "won"), num(t, "lost"), num(t, "drawn")), (2, 1, 0));
        assert_eq!(r.bullets.len(), 2, "the tally and the deal that went worst");
        assert_eq!(r.bullets[1].lead, "decided");
        assert_eq!(
            num(&r.bullets[1], "deal"),
            2,
            "the deal lost is the one named"
        );
    }

    #[test]
    fn a_sitting_of_wins_names_the_best_deal() {
        let ev = sitting(&[(1, ME, &["T4"], 120), (2, ME, &["T4"], 130)]);
        let r = review_sitting(&ev, ME).unwrap();
        assert_eq!(num(&r.bullets[1], "deal"), 1, "the first of equals");
    }

    #[test]
    fn every_key_a_review_can_carry_is_listed() {
        let mut b = Build::new(1, &["SK", "T21", "T1"]);
        b.trick(ME, "T1", "T2", HE);
        let r = review_deal(&b.end(100, 110, HE), ME).unwrap();
        for bl in &r.bullets {
            assert!(KEYS.contains(&bl.key), "{}", bl.key);
        }
    }

    #[test]
    fn played_sittings_review_from_either_chair_with_listed_keys() {
        use strohmandeln_core::view::filter_events;
        for seed in 1..=4u64 {
            let mut all = Vec::new();
            crate::testutil::walk(seed, |s, _| all = s.events().to_vec());
            for human in Seat::BOTH {
                let mine = filter_events(&all, human);
                let sitting = review_sitting(&mine, human).expect("deals were scored");
                assert_eq!(sitting.bullets.len(), 2);
                let last = review_last(&mine, human).expect("a last deal");
                assert_eq!(last.bullets.len(), 2);
                for b in sitting.bullets.iter().chain(&last.bullets) {
                    assert!(KEYS.contains(&b.key), "{}", b.key);
                }
            }
        }
    }

    #[test]
    fn a_trump_on_a_suit_led_shows_the_follower_had_none_to_play() {
        let mut b = Build::new(1, &["T4"]);
        b.trick(HE, "Kh", "T4", ME);
        let e = explain_last(&b.ev, ME).unwrap();
        assert_eq!((e.lead, e.key), ("explain", "explain-trump-you"));
        assert_eq!(text(&e, "suit"), "h");
        let mut b = Build::new(1, &["Kh"]);
        b.trick(ME, "Kh", "T4", HE);
        assert_eq!(explain_last(&b.ev, ME).unwrap().key, "explain-trump-opp");
    }

    #[test]
    fn another_suit_on_a_suit_led_shows_neither_the_suit_nor_a_tarock() {
        let mut b = Build::new(1, &["Kh"]);
        b.trick(ME, "Kh", "9s", ME);
        let e = explain_last(&b.ev, ME).unwrap();
        assert_eq!(
            (e.key, text(&e, "suit").as_str()),
            ("explain-void-opp", "h")
        );
        let mut b = Build::new(1, &["9s"]);
        b.trick(HE, "Kh", "9s", HE);
        assert_eq!(explain_last(&b.ev, ME).unwrap().key, "explain-void-you");
    }

    #[test]
    fn a_suit_card_on_a_tarock_led_shows_no_tarock() {
        let mut b = Build::new(1, &["T9"]);
        b.trick(ME, "T9", "9s", ME);
        assert_eq!(explain_last(&b.ev, ME).unwrap().key, "explain-notarock-opp");
        let mut b = Build::new(1, &["9s"]);
        b.trick(HE, "T9", "9s", HE);
        assert_eq!(explain_last(&b.ev, ME).unwrap().key, "explain-notarock-you");
    }

    #[test]
    fn a_trick_that_followed_the_rules_names_the_winner_and_the_loser() {
        let mut b = Build::new(1, &["Kh"]);
        b.trick(ME, "Kh", "Qh", ME);
        let e = explain_last(&b.ev, ME).unwrap();
        assert_eq!(e.key, "explain-won-you");
        assert_eq!(
            (text(&e, "winner").as_str(), text(&e, "loser").as_str()),
            ("Kh", "Qh")
        );
        let mut b = Build::new(1, &["Qh"]);
        b.trick(ME, "T3", "T7", HE);
        let e = explain_last(&b.ev, ME).unwrap();
        assert_eq!(
            (e.key, text(&e, "winner").as_str()),
            ("explain-won-opp", "T7")
        );
    }

    #[test]
    fn trick_numbers_repeat_across_deals_and_the_last_trick_is_the_one_explained() {
        let mut first = Build::new(1, &["Kh"]);
        first.trick(ME, "Kh", "Qh", ME);
        let mut second = Build::new(2, &["Qh"]);
        second.trick(HE, "Kh", "T4", ME);
        let mut all = first.ev.clone();
        all.extend(second.ev.clone());
        assert_eq!(explain_last(&all, ME).unwrap().key, "explain-trump-you");
    }

    #[test]
    fn there_is_nothing_to_explain_before_a_trick_is_won() {
        let b = Build::new(1, &["Kh"]);
        assert_eq!(explain_last(&b.ev, ME), None);
    }

    #[test]
    fn explanations_cite_only_listed_keys_and_are_documented() {
        for k in EXPLAIN_KEYS {
            assert!(k.starts_with("explain-"), "{k}");
        }
        let doc = include_str!("../../../docs/MESSAGES.md");
        let section = doc.split("\n## Explanation keys").nth(1).expect("section");
        let documented: Vec<&str> = section
            .lines()
            .filter(|l| l.starts_with("| `"))
            .map(|l| l.split('|').nth(1).unwrap().trim().trim_matches('`'))
            .collect();
        assert_eq!(documented, EXPLAIN_KEYS);
    }

    #[test]
    fn the_keys_are_distinct_and_kebab_case() {
        let set: std::collections::HashSet<_> = KEYS.iter().collect();
        assert_eq!(set.len(), KEYS.len());
        for k in KEYS {
            assert!(k.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{k}");
        }
    }

    #[test]
    fn docs_messages_md_gives_every_review_key_in_english_and_german() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let section = doc.split("\n## Review keys").nth(1).expect("section");
        let section = section
            .split("\n## Explanation keys")
            .next()
            .expect("section");
        let rows: Vec<Vec<&str>> = section
            .lines()
            .filter(|l| l.starts_with("| `"))
            .map(|l| l.split('|').map(str::trim).collect())
            .collect();
        let documented: Vec<&str> = rows.iter().map(|r| r[1].trim_matches('`')).collect();
        assert_eq!(documented, KEYS, "same keys, same order");
        for r in &rows {
            assert!(
                !r[2].is_empty() && !r[3].is_empty() && !r[4].is_empty(),
                "{}",
                r[1]
            );
        }
    }
}
