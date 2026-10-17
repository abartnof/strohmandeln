//! The thinking routine of a hint (`docs/HINTS.md` 2, 3): the natural
//! alternative A to the hinted card X, the question that tells them apart,
//! and the one sentence that answers it. Pure functions of [`Knowledge`].

use crate::cardset::{leader_wins, CardSet, Kind};
use crate::explain::Param;
use crate::know::Knowledge;
use crate::rules::CANDIDATES;
use crate::rung2::{reserved_packet, STRONG_TAROCKS};
use crate::tutor::{suit_share, SPLIT_MIN_POINTS, SPLIT_SHARE};
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::types::Action;

/// A sentence of the hint: a namespaced key (`ans.*`, `so.*`, `reason.*`,
/// `fact.*`, `vs.*`) and its parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub key: String,
    pub params: Vec<(&'static str, Param)>,
}

/// A fact of the "Why?" and where the Cards-out tab shows it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fact {
    pub line: Line,
    pub anchor: String,
}

/// What the samples say, when they were drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Evidence {
    /// X better than A in `.0` of `.2` deals, A better in `.1` (a follow).
    pub differ: Option<(u32, u32, u32)>,
    /// The search's own ranking of X: better in `.0` of `.1` deals by `.2`.
    pub none: Option<(u32, u32, u32)>,
    /// The sample saw no real difference between X and the best other card.
    pub even: bool,
}

/// The default hint after the headline.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Composition {
    pub question: Option<&'static str>,
    /// The card `ask.keep` is about.
    pub keep: Option<Card>,
    pub so: Option<Line>,
    /// `None` when the question has no sentence it can honestly give, or when
    /// there is neither a question nor a sample line: the caller then gives
    /// the plain local reason.
    pub ans: Option<Line>,
    pub facts: Vec<Fact>,
}

pub const QUESTIONS: [&str; 8] = [
    "ask.beat",
    "ask.worth",
    "ask.cheap",
    "ask.keep",
    "ask.beneath",
    "ask.out",
    "ask.suit",
    "ask.trumps",
];

/// A rule that passed M5 and may be told (`docs/HINTS.md` 5.1): the rule's
/// candidate id, the question it answers, its sentence and the lower bound
/// of its confirmed precision (`docs/MEASUREMENTS.md`, M5).
pub struct Passed {
    pub rule: &'static str,
    pub question: &'static str,
    pub so: &'static str,
    pub precision: f64,
}

/// M5's one passed rule that carries a sentence, `worth.rich` (115 of 121
/// firings, lower bound 0.926). A test holds the figure to the results table.
pub const PASSED: &[Passed] = &[Passed {
    rule: "worth.rich.1",
    question: "ask.worth",
    so: "so.worth-rich",
    precision: 0.926,
}];

/// Cost order of two cards that can both win or both lose: the lesser first.
fn dearness(c: Card) -> (bool, u8) {
    (c.is_tarock(), c.strength())
}

/// What giving a card up costs: its thirds, then its dearness.
fn price(c: Card) -> (u32, (bool, u8)) {
    (c.thirds(), dearness(c))
}

fn wins(k: &Knowledge, c: Card) -> bool {
    k.lead.is_some_and(|led| !leader_wins(led, c))
}

/// The natural alternative A to the hinted card X (`docs/HINTS.md` 3.2); `None`
/// when no other card is legal.
pub fn alternative(k: &Knowledge, x: Card) -> Option<Card> {
    let others = || k.legal.iter().copied().filter(move |&c| c != x);
    if k.lead.is_some() {
        // X wins: the cheapest other winner, else the cheapest loser. X loses:
        // the cheapest winner, else the cheapest other loser. The same rule.
        others()
            .filter(|&c| wins(k, c))
            .min_by_key(|&c| dearness(c))
            .or_else(|| others().filter(|&c| !wins(k, c)).min_by_key(|&c| price(c)))
    } else {
        let highest = k.legal.iter().copied().max_by_key(|&c| dearness(c))?;
        if highest != x {
            Some(highest)
        } else {
            others().min_by_key(|&c| price(c))
        }
    }
}

/// A card the hand keeps for its own sake: the Sküs, the Mond, the Pagat, or
/// the cover of the packet kept against his Pagat ultimo.
pub fn key_card(k: &Knowledge, c: Card) -> bool {
    matches!(c, Card::SKUS | Card::MOND | Card::PAGAT)
        || reserved_packet(k).is_some_and(|i| k.covers[i] == Some(c))
}

/// Nothing out is higher in its own kind: a master (`tracker` `Group.master`;
/// the cards out are the unseen, his known cards and his covers).
pub fn is_master(k: &Knowledge, c: Card) -> bool {
    !k.opp_possible()
        .iter()
        .any(|o| Kind::of(o) == Kind::of(c) && o.strength() > c.strength())
}

/// A suit lead he cannot follow, so he trumps it (`Knowledge::opp_void`).
fn void_lead(k: &Knowledge, c: Card) -> bool {
    k.lead.is_none() && !c.is_tarock() && k.opp_void[Kind::of(c).index()]
}

#[derive(PartialEq, Eq)]
enum Owner {
    Mine,
    His,
    Shared,
}

/// Whose a suit is (`tutor::SPLIT_SHARE`): mine above the band, his below it,
/// shared in it or when too little is left to contest.
fn owner(k: &Knowledge, suit: Suit) -> Owner {
    let (mine, total) = suit_share(k, suit);
    if total < SPLIT_MIN_POINTS {
        return Owner::Shared;
    }
    let share = f64::from(mine) / f64::from(total);
    if share > SPLIT_SHARE.1 {
        Owner::Mine
    } else if share < SPLIT_SHARE.0 {
        Owner::His
    } else {
        Owner::Shared
    }
}

/// The four ways `ask.keep` can be asked with a sentence that points the way of
/// the play (`docs/HINTS.md` 3.3): the Mond played on his tarock while the Sküs
/// is out, the Mond held then, the Pagat held by a strong hand, a packet kept
/// against his Pagat ultimo. On a lead only a key card X, or a choice between
/// two tarocks, raises it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum KeepCase {
    MondPlay,
    MondHold,
    PagatHold,
    Packet,
}

fn keep_case(k: &Knowledge, x: Card, a: Card) -> Option<KeepCase> {
    if k.lead.is_none() && !(key_card(k, x) || (x.is_tarock() && a.is_tarock())) {
        return None;
    }
    if x == Card::MOND && k.lead.is_some_and(Card::is_tarock) && wins(k, x) && skus_out(k) {
        return Some(KeepCase::MondPlay);
    }
    if a == Card::MOND && x != Card::MOND && skus_out(k) {
        return Some(KeepCase::MondHold);
    }
    if a == Card::PAGAT && x != Card::PAGAT && k.my_tarocks() >= STRONG_TAROCKS {
        return Some(KeepCase::PagatHold);
    }
    if !key_card(k, x) && reserved_packet(k).is_some_and(|i| k.covers[i] == Some(a)) {
        return Some(KeepCase::Packet);
    }
    None
}

fn decides(k: &Knowledge, x: Card, a: Card, q: &str) -> bool {
    let leading = k.lead.is_none();
    match q {
        "ask.keep" => keep_case(k, x, a).is_some(),
        "ask.beneath" => k.is_cover(x) != k.is_cover(a),
        "ask.out" => {
            leading && (is_master(k, x) != is_master(k, a) || void_lead(k, x) != void_lead(k, a))
        }
        "ask.suit" => match (x.suit(), a.suit()) {
            (Some(sx), Some(sa)) => leading && owner(k, sx) != owner(k, sa),
            _ => false,
        },
        "ask.trumps" => leading && x.is_tarock() != a.is_tarock(),
        "ask.worth" => !leading && wins(k, x) != wins(k, a),
        "ask.cheap" => !leading && wins(k, x) == wins(k, a),
        _ => false,
    }
}

/// The decisive questions for X against A, in the teaching order of
/// `docs/HINTS.md` 3.3. `ask.beat` is never one.
pub fn decisive(k: &Knowledge, x: Card, a: Card) -> Vec<&'static str> {
    let order: &[&str] = if k.lead.is_some() {
        &["ask.keep", "ask.worth", "ask.cheap", "ask.beneath"]
    } else {
        &[
            "ask.keep",
            "ask.out",
            "ask.trumps",
            "ask.beneath",
            "ask.suit",
        ]
    };
    order
        .iter()
        .copied()
        .filter(|q| decides(k, x, a, q))
        .collect()
}

/// The `so` sentence of a passed rule for X on question `q`, with the rule's
/// precision; `None` unless the rule fires and X does what it prescribes.
fn so_for(k: &Knowledge, x: Card, q: &str, passed: &[Passed]) -> Option<(Line, f64)> {
    passed.iter().filter(|p| p.question == q).find_map(|p| {
        let rule = CANDIDATES.iter().find(|r| r.id == p.rule)?;
        ((rule.check)(k, &Action::play(x)) == Some(true)).then(|| {
            (
                Line {
                    key: p.so.to_string(),
                    params: Vec::new(),
                },
                p.precision,
            )
        })
    })
}

/// The decisive question to ask (3.3): among those whose `so` rule applies
/// here, the highest measured precision; else the first in teaching order.
pub fn choose(k: &Knowledge, x: Card, a: Card, passed: &[Passed]) -> Option<&'static str> {
    let qs = decisive(k, x, a);
    let best = qs
        .iter()
        .filter_map(|&q| so_for(k, x, q, passed).map(|(_, p)| (q, p)))
        .max_by(|l, r| l.1.total_cmp(&r.1))
        .map(|(q, _)| q);
    best.or_else(|| qs.first().copied())
}

fn int(n: usize) -> Param {
    Param::Int(n as i64)
}

fn num(n: u32) -> Param {
    Param::Int(i64::from(n))
}

fn line(key: &str, params: &[(&'static str, Param)]) -> Line {
    Line {
        key: key.to_string(),
        params: params.to_vec(),
    }
}

fn fact(key: &str, params: &[(&'static str, Param)], anchor: &str) -> Fact {
    Fact {
        line: line(key, params),
        anchor: anchor.to_string(),
    }
}

/// Tarocks neither mine nor played (the Cards-out tab's line 1).
fn tarocks_out(k: &Knowledge) -> usize {
    k.opp_possible().iter().filter(|c| c.is_tarock()).count()
}

fn skus_out(k: &Knowledge) -> bool {
    k.opp_possible().contains(Card::SKUS)
}

fn tarocks_fact(k: &Knowledge) -> Fact {
    fact(
        "fact.tarocks-out",
        &[("out", int(tarocks_out(k))), ("mine", int(k.my_tarocks()))],
        "line1",
    )
}

/// The parameters of a suit's line: my thirds of it among the unplayed
/// cards, and the thirds out.
fn suit_params(k: &Knowledge, suit: Suit) -> Vec<(&'static str, Param)> {
    let sum = |cards: CardSet| -> u32 {
        cards
            .iter()
            .filter(|c| c.suit() == Some(suit))
            .map(Card::thirds)
            .sum()
    };
    vec![
        ("suit", Param::Suit(suit)),
        ("mine_thirds", num(sum(k.playable()))),
        ("out_thirds", num(sum(k.opp_possible()))),
    ]
}

/// The packet index whose cover is `c`.
fn packet_of(k: &Knowledge, c: Card) -> Option<usize> {
    k.covers.iter().position(|&x| x == Some(c))
}

fn none_line(ev: &Evidence) -> Option<Line> {
    if let Some((better, worlds, gain)) = ev.none {
        return Some(line(
            "ans.ask-none",
            &[
                ("better", num(better)),
                ("worlds", num(worlds)),
                ("gain_thirds", num(gain)),
            ],
        ));
    }
    ev.even.then(|| line("ans.ask-none-even", &[]))
}

type Answer = (Option<Line>, Option<Card>, Vec<Fact>);

fn keep_answer(k: &Knowledge, case: KeepCase) -> (Line, Vec<Fact>) {
    let mut facts = Vec::new();
    let ans = match case {
        KeepCase::MondPlay | KeepCase::MondHold => {
            facts.push(fact("fact.skus-out", &[], "row:trump"));
            facts.push(tarocks_fact(k));
            line(
                if case == KeepCase::MondPlay {
                    "ans.keep-mond-play"
                } else {
                    "ans.keep-mond-hold"
                },
                &[],
            )
        }
        KeepCase::PagatHold => {
            facts.push(tarocks_fact(k));
            line("ans.keep-pagat-hold", &[("tarocks", int(k.my_tarocks()))])
        }
        KeepCase::Packet => line("ans.keep-packet", &[]),
    };
    (ans, facts)
}

fn out_answer(k: &Knowledge, x: Card, a: Card) -> (Option<Line>, Vec<Fact>) {
    let mut facts = Vec::new();
    let master = [x, a].into_iter().find(|&c| is_master(k, c));
    let voided = [x, a]
        .into_iter()
        .find(|&c| void_lead(k, c))
        .and_then(Card::suit);
    if let Some(m) = master {
        facts.push(fact("fact.master", &[("card", Param::Card(m))], "line2"));
    }
    if let Some(v) = voided {
        facts.push(fact("fact.his-void", &[("suit", Param::Suit(v))], "line3"));
    }
    let ans = match (master, voided) {
        (Some(m), _) => Some(line(
            if m.is_tarock() {
                "ans.out-master-tarock"
            } else {
                "ans.out-master"
            },
            &[("card", Param::Card(m))],
        )),
        (None, Some(v)) => Some(line(
            "ans.out-void",
            &[("suit", Param::Suit(v)), ("out", int(tarocks_out(k)))],
        )),
        (None, None) => None,
    };
    (ans, facts)
}

fn worth_answer(k: &Knowledge, x: Card, a: Card) -> Line {
    let led = k.lead.expect("a lead");
    let winner = if wins(k, x) { x } else { a };
    if led.thirds() >= 7 {
        line(
            "ans.worth-rich",
            &[("led", Param::Card(led)), ("led_thirds", num(led.thirds()))],
        )
    } else {
        line(
            "ans.worth-poor",
            &[
                ("led", Param::Card(led)),
                ("trick_thirds", num(led.thirds() + winner.thirds())),
            ],
        )
    }
}

fn cheap_answer(k: &Knowledge, x: Card, a: Card, ev: &Evidence) -> Option<Line> {
    let led = k.lead.expect("a lead");
    let both_win = wins(k, x);
    let cheaper = if both_win {
        dearness(x) < dearness(a)
    } else {
        price(x) < price(a)
    };
    if cheaper {
        let key = if both_win {
            "ans.cheap"
        } else {
            "ans.cheap-lose"
        };
        return Some(line(
            key,
            &[("card", Param::Card(x)), ("alt", Param::Card(a))],
        ));
    }
    // The dearer of two winners in a suit trick: the cheapness maxim is for
    // suit tricks (5.4, `ask-differ`); otherwise the sample's own line.
    match ev.differ {
        Some((b, al, w)) if both_win && !led.is_tarock() => Some(line(
            "ans.ask-differ",
            &[
                ("card", Param::Card(x)),
                ("a", num(al)),
                ("b", num(b)),
                ("w", num(w)),
            ],
        )),
        _ => none_line(ev),
    }
}

fn answer(k: &Knowledge, x: Card, a: Card, q: &str, ev: &Evidence) -> Answer {
    match q {
        "ask.keep" => {
            let case = keep_case(k, x, a).expect("decisive");
            let kept = if case == KeepCase::MondPlay { x } else { a };
            let (ans, facts) = keep_answer(k, case);
            (Some(ans), Some(kept), facts)
        }
        "ask.beneath" => {
            let cover = if k.is_cover(x) { x } else { a };
            let n = packet_of(k, cover).map_or(0, |i| usize::from(k.my_hidden[i]));
            let ans = line(
                "ans.beneath",
                &[("card", Param::Card(cover)), ("n", int(n))],
            );
            (
                Some(ans),
                None,
                vec![fact("fact.beneath", &[("n", int(n))], "row:packets")],
            )
        }
        "ask.out" => {
            let (ans, facts) = out_answer(k, x, a);
            (ans, None, facts)
        }
        "ask.suit" => {
            let s = x.suit().expect("a suit card");
            let params = suit_params(k, s);
            let anchor = format!("row:{}", s.letter());
            (
                Some(line("ans.suit", &params)),
                None,
                vec![fact("fact.split", &params, &anchor)],
            )
        }
        "ask.trumps" => {
            let (mine, total) = (k.my_tarocks(), k.my_tarocks() + tarocks_out(k));
            let key = if mine * 2 >= total {
                "ans.trumps-yes"
            } else {
                "ans.trumps-no"
            };
            let ans = line(key, &[("mine", int(mine)), ("total", int(total))]);
            (Some(ans), None, vec![tarocks_fact(k)])
        }
        "ask.worth" => (Some(worth_answer(k, x, a)), None, Vec::new()),
        "ask.cheap" => (cheap_answer(k, x, a, ev), None, Vec::new()),
        _ => (None, None, Vec::new()),
    }
}

/// The default hint after the headline for the hinted card `x`.
pub fn compose(k: &Knowledge, x: Card, ev: &Evidence) -> Composition {
    compose_with(k, x, ev, PASSED)
}

/// [`compose`] with the passed rules given (tests choose their own).
pub fn compose_with(k: &Knowledge, x: Card, ev: &Evidence, passed: &[Passed]) -> Composition {
    let plain = |ans| Composition {
        question: None,
        keep: None,
        so: None,
        ans,
        facts: Vec::new(),
    };
    let Some(a) = alternative(k, x).filter(|_| k.legal.len() > 1) else {
        return plain(None);
    };
    let Some(q) = choose(k, x, a, passed) else {
        return plain(none_line(ev));
    };
    let (ans, keep, facts) = answer(k, x, a, q, ev);
    Composition {
        question: Some(q),
        keep,
        so: so_for(k, x, q, passed).map(|(l, _)| l),
        ans,
        facts,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{c, know};

    const NO: [&str; 3] = ["", "", ""];

    fn alt(k: &Knowledge, x: &str) -> Option<Card> {
        alternative(k, c(x))
    }

    fn qs(k: &Knowledge, x: &str, a: &str) -> Vec<&'static str> {
        decisive(k, c(x), c(a))
    }

    /// The position of the worked case (b): Uhu (II) led, the hand holds the
    /// Pagat, III, the Mond and ten more tarocks.
    fn uhu_led() -> Knowledge {
        let hand = [
            "T1", "T3", "T4", "T5", "T6", "T7", "T8", "T9", "T10", "T11", "T12", "T13", "T21",
        ];
        know(&hand, NO, NO, &[], Some("T2"))
    }

    /// The position of the worked case (a): a lead with the jack of diamonds
    /// a cover, XIX the highest card held.
    fn jack_cover() -> Knowledge {
        let hand = ["T3", "T7", "T9", "T12", "T15", "T19"];
        know(&hand, ["Jd", "", ""], NO, &[], None)
    }

    #[test]
    fn the_alternative_is_fixed_by_rule() {
        // Following, X wins: the cheapest other winner (suits below tarocks).
        let k = know(&["Kh", "Qh", "Jh"], NO, NO, &[], Some("4h"));
        assert_eq!(alt(&k, "Kh"), Some(c("Jh")));
        assert_eq!(alt(&k, "Jh"), Some(c("Qh")));
        let k = know(&["T19", "T21", "9c"], NO, NO, &[], Some("T15"));
        assert_eq!(alt(&k, "T21"), Some(c("T19")));
        // X the only winner: the cheapest loser (least thirds, then dearness).
        let k = know(&["Kh", "4h", "3h"], NO, NO, &[], Some("Qh"));
        assert_eq!(alt(&k, "Kh"), Some(c("4h")));
        // X loses: the cheapest winner.
        let k = know(&["Kh", "Qh", "4h"], NO, NO, &[], Some("Jh"));
        assert_eq!(alt(&k, "4h"), Some(c("Qh")));
        // Nothing wins: the cheapest other loser.
        let k = know(&["4h", "Qh", "3h"], NO, NO, &[], Some("Kh"));
        assert_eq!(alt(&k, "Qh"), Some(c("4h")));
        // Leading: the highest card held, the Sküs included.
        let k = know(&["T9", "T14", "Kh"], NO, NO, &[], None);
        assert_eq!(alt(&k, "Kh"), Some(c("T14")));
        let k = know(&["SK", "T14", "Kh"], NO, NO, &[], None);
        assert_eq!(alt(&k, "Kh"), Some(c("SK")));
        // ... and the cheapest when X is that card.
        let k = know(&["T14", "Kh", "3h"], NO, NO, &[], None);
        assert_eq!(alt(&k, "T14"), Some(c("3h")));
        // No other legal card.
        let k = know(&["4h", "9c"], NO, NO, &[], Some("2h"));
        assert_eq!(alt(&k, "4h"), None);
    }

    #[test]
    fn the_pagat_that_loses_is_not_the_alternative() {
        // Uhu led: the Pagat loses to it, so it is a loser, not A.
        let k = uhu_led();
        assert_eq!(alt(&k, "T21"), Some(c("T3")));
    }

    #[test]
    fn the_decisive_question_follows_the_table() {
        // keep: the Mond (key) against III, with the Sküs out and a tarock led.
        let k = uhu_led();
        assert_eq!(qs(&k, "T21", "T3")[0], "ask.keep");
        // beneath: a cover against a hand card.
        let k = jack_cover();
        assert!(qs(&k, "Jd", "T19").contains(&"ask.beneath"));
        // out: a master (no higher heart out) against a card that is not.
        let k = know(&["Kh", "T9"], NO, NO, &[], None);
        assert!(qs(&k, "Kh", "T9").contains(&"ask.out"));
        // out: a lead he cannot follow, against one he can.
        let mut k = know(&["7c", "8s", "T9"], NO, NO, &[], None);
        k.opp_void[Kind::Suit(Suit::Clubs).index()] = true;
        assert!(qs(&k, "7c", "8s").contains(&"ask.out"));
        // suit: a suit that is mine against one that is his.
        let k = know(&["Kh", "Qh", "Jh", "Ch", "4h", "7s"], NO, NO, &[], None);
        assert!(qs(&k, "Kh", "7s").contains(&"ask.suit"));
        assert!(!qs(&k, "Kh", "Qh").contains(&"ask.suit"));
        // trumps: one tarock, one not.
        let k = know(&["9c", "T9"], NO, NO, &[], None);
        assert_eq!(qs(&k, "9c", "T9"), vec!["ask.trumps"]);
        // worth: one wins the trick, the other does not.
        let k = know(&["Kh", "4h"], NO, NO, &[], Some("Qh"));
        assert_eq!(qs(&k, "Kh", "4h"), vec!["ask.worth"]);
        // cheap: both win.
        let k = know(&["Kh", "Qh"], NO, NO, &[], Some("Jh"));
        assert_eq!(qs(&k, "Qh", "Kh"), vec!["ask.cheap"]);
        // ask.beat is never decisive, in any of these.
        for k in [uhu_led(), jack_cover()] {
            for &x in &k.legal {
                for &a in &k.legal {
                    assert!(!decisive(&k, x, a).contains(&"ask.beat"));
                }
            }
        }
    }

    #[test]
    fn a_passed_rule_outranks_the_teaching_order() {
        // Following: keep (Pagat) and worth are both decisive; with no rule
        // the teaching order puts keep first; with worth's rule firing, worth.
        let hand = ["T1", "T3", "T4", "T5", "T6", "T7", "T8", "T9"];
        let k = know(&hand, NO, NO, &[], Some("Kh"));
        // X = T8 wins the king (13 thirds), A = the Pagat... a winner too.
        let x = c("T8");
        let a = alternative(&k, x).unwrap();
        assert_eq!(a, c("T1"));
        assert_eq!(choose(&k, x, a, &[]), Some("ask.keep"));
        // The same with a rule for ask.cheap that fires and has no bearing:
        // only a rule that fires counts.
        let fake = [Passed {
            rule: "worth.rich.1",
            question: "ask.worth",
            so: "so.worth-rich",
            precision: 0.9,
        }];
        assert_eq!(choose(&k, x, a, &fake), Some("ask.keep"));
        // Mond led-over: X = Kh sole winner of a queen, A = 4h: worth, with
        // the rule firing.
        let k = know(&["Kh", "4h", "T1"], NO, NO, &[], Some("Qh"));
        assert_eq!(choose(&k, c("Kh"), c("4h"), &fake), Some("ask.worth"));
    }

    #[test]
    fn worked_case_b_asks_whether_the_mond_is_safe_to_keep_and_never_the_pagat() {
        let k = uhu_led();
        let comp = compose(&k, c("T21"), &Evidence::default());
        assert_eq!(comp.question, Some("ask.keep"));
        assert_eq!(comp.keep, Some(Card::MOND));
        // The Mond rule has not passed: the fact, no rule.
        assert_eq!(comp.so, None);
        assert_eq!(comp.ans.as_ref().unwrap().key, "ans.keep-mond-play");
        let text = format!("{comp:?}");
        assert!(!text.contains("PAGAT") && !text.contains("pagat"), "{text}");
        // With the Sküs not out there is nothing to say of the Mond.
        let mut k = uhu_led();
        k.unseen.remove(Card::SKUS);
        let comp = compose(&k, c("T21"), &Evidence::default());
        assert_ne!(comp.question, Some("ask.keep"));
    }

    #[test]
    fn worked_case_a_asks_whether_to_lead_a_tarock() {
        let k = jack_cover();
        let a = alternative(&k, c("Jd")).unwrap();
        assert_eq!(a, c("T19"));
        let comp = compose(&k, c("Jd"), &Evidence::default());
        assert_eq!(comp.question, Some("ask.trumps"));
        assert_eq!(comp.so, None);
        let ans = comp.ans.unwrap();
        assert_eq!(ans.key, "ans.trumps-no");
        assert!(ans.params.contains(&("mine", Param::Int(6))));
        // 22 tarocks, 6 mine: 16 out, so 6 of the 22 still out.
        assert!(ans.params.contains(&("total", Param::Int(22))));
    }

    #[test]
    fn a_rich_trick_won_names_the_rule() {
        let k = know(&["Kh", "4h"], NO, NO, &[], Some("Qh"));
        let comp = compose(&k, c("Kh"), &Evidence::default());
        assert_eq!(comp.question, Some("ask.worth"));
        assert_eq!(comp.so.unwrap().key, "so.worth-rich");
        assert_eq!(comp.ans.unwrap().key, "ans.worth-rich");
        // Declining it: the rule fires but X contradicts it, so no sentence.
        let comp = compose(&k, c("4h"), &Evidence::default());
        assert_eq!(comp.so, None);
    }

    #[test]
    fn the_dearer_of_two_winners_gives_the_sample_line_only_in_a_suit_trick() {
        let k = know(&["Kh", "Qh"], NO, NO, &[], Some("Jh"));
        let ev = Evidence {
            differ: Some((9, 3, 20)),
            none: Some((12, 20, 4)),
            even: false,
        };
        let cheaper = compose(&k, c("Qh"), &ev);
        assert_eq!(cheaper.ans.unwrap().key, "ans.cheap");
        let dearer = compose(&k, c("Kh"), &ev);
        assert_eq!(dearer.ans.unwrap().key, "ans.ask-differ");
        // A tarock trick: no cheapness maxim, so the search's own line.
        let k = know(&["T18", "T19"], NO, NO, &[], Some("T15"));
        assert_eq!(compose(&k, c("T19"), &ev).ans.unwrap().key, "ans.ask-none");
        // Without a sample nothing is claimed.
        assert_eq!(compose(&k, c("T19"), &Evidence::default()).ans, None);
    }

    #[test]
    fn no_decisive_question_gives_the_sample_line() {
        // Two clubs, none a master, one suit, no cover, no tarock.
        let k = know(&["9c", "8c"], NO, NO, &[], None);
        let ev = Evidence {
            differ: None,
            none: Some((30, 50, 4)),
            even: false,
        };
        let comp = compose(&k, c("8c"), &ev);
        assert_eq!(comp.question, None);
        assert_eq!(comp.ans.unwrap().key, "ans.ask-none");
        // A sample that sees nothing between them says so, still honestly.
        let even = Evidence {
            even: true,
            ..Evidence::default()
        };
        assert_eq!(
            compose(&k, c("8c"), &even).ans.unwrap().key,
            "ans.ask-none-even"
        );
        assert_eq!(compose(&k, c("8c"), &Evidence::default()).ans, None);
    }

    #[test]
    fn a_forced_card_has_no_question() {
        let k = know(&["4h", "9c"], NO, NO, &[], Some("2h"));
        let comp = compose(&k, c("4h"), &Evidence::default());
        assert_eq!((comp.question, comp.ans), (None, None));
    }

    #[test]
    fn the_passed_rules_are_in_the_measure_s_results_table() {
        let doc = include_str!("../../../docs/MEASUREMENTS.md");
        for p in PASSED {
            let row = doc
                .lines()
                .find(|l| l.starts_with(&format!("| {} ", p.rule)))
                .unwrap_or_else(|| panic!("{} in MEASUREMENTS.md", p.rule));
            assert!(row.contains(&format!("{:.3}", p.precision)), "{row}");
            assert!(CANDIDATES.iter().any(|r| r.id == p.rule), "{}", p.rule);
            assert!(QUESTIONS.contains(&p.question));
        }
    }

    #[test]
    fn facts_are_counts_of_known_cards() {
        let k = uhu_led();
        let comp = compose(&k, c("T21"), &Evidence::default());
        let keys: Vec<&str> = comp.facts.iter().map(|f| f.line.key.as_str()).collect();
        assert_eq!(keys, ["fact.skus-out", "fact.tarocks-out"]);
        // 22 tarocks, 13 mine, the II led: 8 out.
        assert_eq!(
            comp.facts[1].line.params,
            [("out", Param::Int(8)), ("mine", Param::Int(13))]
        );
        assert_eq!(comp.facts[1].anchor, "line1");
    }

    /// A lead with the Mond held and the Sküs out: hand T5, T9, the Mond and Kh.
    fn mond_held() -> Knowledge {
        know(&["T5", "T9", "T21", "Kh"], NO, NO, &[], None)
    }

    #[test]
    fn on_a_lead_keep_is_asked_only_of_a_key_card_or_between_two_tarocks() {
        let k = mond_held();
        // A is the Mond; X a master king is not a choice about keeping it.
        assert_eq!(alt(&k, "Kh"), Some(c("T21")));
        assert!(!qs(&k, "Kh", "T21").contains(&"ask.keep"));
        // X a small tarock: the choice is to spend a tarock or keep the Mond.
        assert_eq!(qs(&k, "T5", "T21")[0], "ask.keep");
    }

    #[test]
    fn a_keep_sentence_points_the_way_of_the_play() {
        // The Mond played on his tarock with the Sküs out: kept == X, "play".
        let k = uhu_led();
        let comp = compose(&k, c("T21"), &Evidence::default());
        assert_eq!(comp.keep, Some(Card::MOND));
        assert_eq!(comp.ans.as_ref().unwrap().key, "ans.keep-mond-play");
        // The Mond held while a small tarock goes: kept == A, "hold".
        let k = mond_held();
        let comp = compose(&k, c("T5"), &Evidence::default());
        assert_eq!(comp.keep, Some(Card::MOND));
        assert_eq!(comp.ans.as_ref().unwrap().key, "ans.keep-mond-hold");
        // The Mond held with the Sküs not out: nothing to say of it.
        let mut k = mond_held();
        k.unseen.remove(Card::SKUS);
        assert_ne!(
            compose(&k, c("T5"), &Evidence::default()).question,
            Some("ask.keep")
        );
        // The Mond led with the Sküs out is not "play it on his tarock".
        let k = mond_held();
        assert_ne!(
            compose(&k, c("T21"), &Evidence::default()).question,
            Some("ask.keep")
        );
        // Over many positions the key of the sentence matches who is kept.
        for k in [uhu_led(), mond_held(), jack_cover()] {
            for &x in &k.legal {
                let comp = compose(&k, x, &Evidence::default());
                if comp.question != Some("ask.keep") {
                    continue;
                }
                let key = comp.ans.unwrap().key;
                if comp.keep == Some(x) {
                    assert!(key.ends_with("-play"), "{x:?} {key}");
                } else {
                    assert!(
                        key.ends_with("-hold") || key == "ans.keep-packet",
                        "{x:?} {key}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_yes_no_question_is_answered_with_its_verdict_first() {
        // Lead a tarock? 6 of 22: not yet; 12 of 22: yes.
        let k = jack_cover();
        assert_eq!(
            compose(&k, c("Jd"), &Evidence::default()).ans.unwrap().key,
            "ans.trumps-no"
        );
        let hand = [
            "T3", "T7", "T9", "T12", "T15", "T19", "T4", "T5", "T6", "T8", "T10", "T11",
        ];
        let k = know(&hand, ["Jd", "", ""], NO, &[], None);
        let ans = compose(&k, c("Jd"), &Evidence::default()).ans.unwrap();
        assert_eq!(ans.key, "ans.trumps-yes");
        // The worth of a trick: yes when rich, no when poor.
        let k = know(&["Kh", "4h"], NO, NO, &[], Some("Qh"));
        assert_eq!(
            compose(&k, c("Kh"), &Evidence::default()).ans.unwrap().key,
            "ans.worth-rich"
        );
        let k = know(&["Kh", "4h"], NO, NO, &[], Some("2h"));
        assert_eq!(
            compose(&k, c("Kh"), &Evidence::default()).ans.unwrap().key,
            "ans.worth-poor"
        );
    }

    #[test]
    fn the_tarock_counts_of_the_sentence_and_the_fact_agree() {
        // One tarock of his is known face up: out counts it, as the tab does.
        let hand = ["T3", "T7", "T9", "T12", "T15", "T19"];
        let k = know(&hand, ["Jd", "", ""], NO, &["T20"], None);
        let comp = compose(&k, c("Jd"), &Evidence::default());
        let out = tarocks_out(&k);
        assert_eq!(out, 16);
        let ans = comp.ans.unwrap();
        assert!(ans.params.contains(&("total", Param::Int(6 + out as i64))));
        let fact = comp
            .facts
            .iter()
            .find(|f| f.line.key == "fact.tarocks-out")
            .unwrap();
        assert!(fact.line.params.contains(&("out", Param::Int(out as i64))));
        // And the sentence about a suit he lacks.
        let mut k = know(&["7c", "8s", "T9"], NO, NO, &["T20"], None);
        k.opp_void[Kind::Suit(Suit::Clubs).index()] = true;
        let (ans, _) = out_answer(&k, c("7c"), c("8s"));
        assert!(ans
            .unwrap()
            .params
            .contains(&("out", Param::Int(tarocks_out(&k) as i64))));
    }

    #[test]
    fn a_suit_is_thirty_eight_thirds() {
        let k = know(&["7s"], NO, NO, &[], None);
        let p = suit_params(&k, Suit::Hearts);
        assert_eq!(p[1].1, Param::Int(0));
        assert_eq!(p[2].1, Param::Int(38), "12 and two thirds points");
    }
}
