//! Rung 2, "Rules of thumb": the maxims of the period and modern books as a
//! one-trick scoring of the legal cards, and a bid from the strength of the
//! hand. It looks at its own cards, the covers, the cards pulled into the
//! opponent's hand and the card led. It does not count: what was played in
//! earlier tricks is rung 3's business.
//!
//! The maxims (`research/07` section 1; Werner, "Das moderne Tarokspiel", in
//! `research/03`):
//!
//! - **Finesse (Impassiren)**: do not win a poor trick with an honour. Winning
//!   it hands the lead over, and the player who opens the contested suit
//!   ends worse off [07 1.3]. Each trick won carries a small `BURDEN`.
//! - **Do not lead an evenly split suit** [07 1.3].
//! - **Play the face-up packet cards off** [07 1.2], and exposed Queens and
//!   Cavalls at once, unless the opponent shows two low cards of the suit.
//! - **Keep one packet unplayed when a Pagat ultimo threatens** [07 1.2].
//! - **Hold trump I for the last trick when strong in tarocks** [07 1.4/1.5];
//!   with few tarocks cash it early.
//! - **Follow cheaply**: lose with the least; win with the cheapest winner.
//! - **Take tricks rich in card points.**

use crate::cardset::{leader_wins, CardSet, Kind};
use crate::know::Knowledge;
use strohmandeln_core::cards::{Card, Honour, Suit};
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// Weights of the maxims, in thirds of a point.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Weights {
    /// A won trick passes the lead to its winner, who must open a suit.
    pub burden: f64,
    /// Playing a cover shows what is beneath it sooner.
    pub cover: f64,
    pub exposed_figure: f64,
    pub split_suit: f64,
    pub keep_packet: f64,
    pub hold_pagat: f64,
    pub cash_king: f64,
    pub draw_trumps: f64,
    pub solid_run: f64,
    pub keep_tarocks: f64,
    pub high_trump_lead: f64,
    pub coverless_suit: f64,
    /// The constants a style may move (`docs/STYLES.md` 2, `style.rs`);
    /// Normal's values are the consts below.
    pub strong_tarocks: usize,
    pub weak_tarocks: usize,
    pub threat_tarocks: usize,
    /// The tarock count from which a lead draws trumps (`draw_trumps`);
    /// `None` is Normal's: `strong_tarocks - 1` in rung 2, `- 2` in rung 3.
    pub draw_from: Option<usize>,
    /// Play a lying Dame or Cavall unless the opponent shows two low cards of
    /// the suit (U 74, MS); Bermann has no such exception (Be 148).
    pub low_shown_exception: bool,
    /// `guard-mond`: with the Mond and without the Sküs, the cost of leaving
    /// fewer than `guard_keep` small tarocks (0: off).
    pub guard_mond: f64,
    pub guard_keep: usize,
    /// `cash-mond`: the reward of playing an unescorted Mond to win a rich
    /// trick (0: off).
    pub cash_mond: f64,
    /// `spare-lead`: the reward of opening, with spare low cards, a suit
    /// split between a fork each (0: off).
    pub spare_lead: f64,
    /// Rung 4: the margin (thirds) within which the maxims decide.
    pub band: f64,
    /// Ties are settled by the card order, not by a draw (level 1: the seed
    /// breaks no tie, so the play is readable).
    pub stable_ties: bool,
}

impl Default for Weights {
    fn default() -> Weights {
        Weights {
            burden: 3.0,
            cover: 0.8,
            exposed_figure: 2.0,
            split_suit: 4.0,
            keep_packet: 3.0,
            hold_pagat: 100.0,
            cash_king: 3.0,
            draw_trumps: 3.0,
            solid_run: 2.0,
            keep_tarocks: 4.0,
            high_trump_lead: 5.0,
            coverless_suit: 2.0,
            strong_tarocks: STRONG_TAROCKS,
            weak_tarocks: WEAK_TAROCKS,
            threat_tarocks: THREAT_TAROCKS,
            draw_from: None,
            low_shown_exception: true,
            guard_mond: 0.0,
            guard_keep: 1,
            cash_mond: 0.0,
            spare_lead: 0.0,
            band: 0.34,
            stable_ties: false,
        }
    }
}

/// Tarocks at which a hand is strong, and weak.
pub const STRONG_TAROCKS: usize = 8;
pub const WEAK_TAROCKS: usize = 4;
/// Tarocks the opponent has shown at which his Pagat ultimo threatens.
pub const THREAT_TAROCKS: usize = 5;

/// A card, its score, and the maxim that weighed most in it (a message key
/// for the tutor, `docs/MESSAGES.md`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Choice {
    pub card: Card,
    pub score: f64,
    pub reason: &'static str,
}

pub(crate) struct Tally {
    pub(crate) total: f64,
    pub(crate) best: (f64, &'static str),
}

impl Tally {
    pub(crate) fn new(base_reason: &'static str) -> Tally {
        Tally {
            total: 0.0,
            best: (0.0, base_reason),
        }
    }
    pub(crate) fn add(&mut self, v: f64, reason: &'static str) {
        self.total += v;
        if v.abs() > self.best.0.abs() {
            self.best = (v, reason);
        }
    }
}

pub(crate) fn cover_index(k: &Knowledge, c: Card) -> Option<usize> {
    k.covers.iter().position(|&x| x == Some(c))
}

/// Cards of `kind` the opponent is known to hold or show.
pub(crate) fn opp_visible(k: &Knowledge, kind: Kind) -> Vec<Card> {
    k.opp_covers
        .iter()
        .flatten()
        .copied()
        .chain(k.opp_known.iter())
        .filter(|&c| Kind::of(c) == kind)
        .collect()
}

pub(crate) fn my_value_in(k: &Knowledge, suit: Suit) -> u32 {
    k.playable()
        .iter()
        .filter(|c| c.suit() == Some(suit))
        .map(|c| c.value())
        .sum()
}

/// The packet to keep unplayed when a Pagat ultimo threatens: the one with
/// the most face-down cards.
pub(crate) fn reserved_packet(k: &Knowledge) -> Option<usize> {
    reserved_packet_with(k, THREAT_TAROCKS)
}

pub(crate) fn reserved_packet_with(k: &Knowledge, threat_tarocks: usize) -> Option<usize> {
    let threat = k.opp_known.iter().filter(|c| c.is_tarock()).count() >= threat_tarocks;
    if !threat {
        return None;
    }
    let (i, &n) = k
        .my_hidden
        .iter()
        .enumerate()
        .max_by_key(|&(i, n)| (*n, 3 - i))?;
    (n >= 2).then_some(i)
}

/// What spending a card costs the hand's future.
pub(crate) fn spend(c: Card) -> f64 {
    if c.is_tarock() {
        f64::from(c.strength()) / 21.0 * 3.0
    } else {
        match c.strength() {
            7 => 2.0,
            6 => 1.2,
            5 => 0.6,
            _ => 0.0,
        }
    }
}

/// Scores every legal card, best first.
pub fn score_cards(k: &Knowledge, wt: &Weights) -> Vec<Choice> {
    let mut out: Vec<Choice> = k
        .legal
        .iter()
        .map(|&c| {
            let t = match k.lead {
                Some(led) => follow(k, led, c, wt),
                None => lead(k, c, wt),
            };
            Choice {
                card: c,
                score: t.total,
                reason: t.best.1,
            }
        })
        .collect();
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap()
            .then(a.card.cmp(&b.card))
    });
    out
}

fn follow(k: &Knowledge, led: Card, c: Card, wt: &Weights) -> Tally {
    let mut t = Tally::new("follow-cheaply");
    let wins = !leader_wins(led, c);
    if wins {
        let gain = f64::from(led.thirds());
        t.add(
            gain,
            if led.thirds() >= 7 {
                "take-rich-trick"
            } else {
                "take-trick"
            },
        );
        t.add(-wt.burden, "finesse");
        t.add(-spend(c), "follow-cheaply");
    } else {
        t.add(-f64::from(c.thirds()), "follow-cheaply");
        t.add(-0.01 * f64::from(c.strength()), "follow-cheaply");
    }
    common(k, c, &mut t, k.legal.len() > 1, wt);
    t
}

fn lead(k: &Knowledge, c: Card, wt: &Weights) -> Tally {
    let mut t = Tally::new("lead-low");
    let tarocks = k.my_tarocks();
    match c.suit() {
        None => {
            t.add(-spend(c), "keep-tarocks");
            if tarocks >= wt.draw_from.unwrap_or(wt.strong_tarocks - 1) {
                t.add(wt.draw_trumps, "draw-trumps");
                let neighbour = |d: i32| {
                    let s = i32::from(c.strength()) + d;
                    (0..=21).contains(&s)
                        && k.playable()
                            .iter()
                            .any(|x| x.is_tarock() && i32::from(x.strength()) == s)
                };
                if neighbour(-1) || neighbour(1) {
                    t.add(wt.solid_run, "lead-solid-run");
                }
                t.add(-0.05 * f64::from(c.strength()), "lead-solid-run");
            } else if tarocks <= wt.weak_tarocks {
                t.add(-wt.keep_tarocks, "keep-tarocks");
            }
            if c.thirds() > 1 && k.legal.len() > 1 {
                t.add(-wt.high_trump_lead, "hold-high-trump");
            }
        }
        Some(suit) => {
            let kind = Kind::of(c);
            let visible = opp_visible(k, kind);
            let mine = my_value_in(k, suit);
            if mine >= 3 && (0.3..=0.7).contains(&(f64::from(mine) / 18.0)) {
                t.add(-wt.split_suit, "dont-lead-split-suit");
            }
            if visible.iter().any(|v| v.strength() > c.strength()) {
                t.add(-f64::from(c.thirds()) - 2.0, "finesse");
            }
            spare_lead(k, c, &mut t, wt);
            if c.is_king() {
                t.add(wt.cash_king, "cash-king");
            } else {
                t.add(-0.5 * f64::from(c.thirds()), "lead-low");
            }
            if cover_index(k, c).is_some() {
                let low_shown = visible.iter().filter(|v| v.thirds() == 1).count();
                if (c.strength() == 6 || c.strength() == 5)
                    && (low_shown < 2 || !wt.low_shown_exception)
                {
                    t.add(wt.exposed_figure, "play-exposed-figure");
                }
                if visible.is_empty() {
                    t.add(wt.coverless_suit, "lead-where-he-shows-none");
                }
            }
        }
    }
    common(k, c, &mut t, k.legal.len() > 1, wt);
    t
}

/// The maxims that hold for leading and following alike.
pub(crate) fn common(k: &Knowledge, c: Card, t: &mut Tally, choice: bool, wt: &Weights) {
    if let Some(i) = cover_index(k, c) {
        t.add(wt.cover, "play-packet-off");
        if reserved_packet_with(k, wt.threat_tarocks) == Some(i) {
            t.add(-wt.keep_packet, "keep-packet");
        }
    }
    if c == Card::PAGAT && choice && k.my_tarocks() >= wt.strong_tarocks {
        t.add(-wt.hold_pagat, "hold-pagat");
    }
    if choice {
        mond(k, c, t, wt);
    }
}

/// Tarocks 2 to 10: the small ones a Mond reserve keeps (I: no text numbers
/// them; Be 151-152 "eine oder mehrere Tarok", MS "Mondfang" one or two).
fn small_tarock(c: Card) -> bool {
    c.is_tarock() && (1..=9).contains(&c.strength())
}

/// The Mond maxims, for a hand holding the Mond without the Sküs.
fn mond(k: &Knowledge, c: Card, t: &mut Tally, wt: &Weights) {
    let mine = k.playable();
    if !mine.contains(Card::MOND) || mine.contains(Card::SKUS) {
        return;
    }
    // guard-mond [Be 151-152 T, MS "Mondfang" T]: keep small tarocks, to force
    // the opponent to trump and to come into the rear hand late in the deal.
    if wt.guard_mond > 0.0 && small_tarock(c) {
        let small = mine.iter().filter(|&x| small_tarock(x)).count();
        if small - 1 < wt.guard_keep {
            t.add(-wt.guard_mond, "guard-mond");
        }
    }
    // cash-mond [Be 152 T, U 74 T]: "stets bei passender Gelegenheit
    // einstechen" (I: a fitting chance is a rich suit trick it takes).
    if wt.cash_mond > 0.0 && c == Card::MOND {
        if let Some(led) = k.lead {
            if !led.is_tarock() && led.thirds() >= 7 {
                t.add(wt.cash_mond, "cash-mond");
            }
        }
    }
}

/// spare-lead [Be 150-151 T, this position only]: a suit wholly split, a fork
/// each (my king and cavall against his queen and jack), and spare low cards:
/// open with the highest of the spare low cards (Be: "Zwei", not "Vier"; in
/// the order of RULES 1.5 the two is the higher), so that he must take with the queen (or give up the
/// jack) and my figures come in.
pub(crate) fn spare_lead(k: &Knowledge, c: Card, t: &mut Tally, wt: &Weights) {
    let Some(suit) = c.suit() else { return };
    if wt.spare_lead <= 0.0 {
        return;
    }
    let mine = k.playable();
    let theirs = k.opp_possible();
    let of = |s: u8| Card::suit_card(suit, s);
    let fork = mine.contains(of(7))
        && mine.contains(of(5))
        && theirs.contains(of(6))
        && theirs.contains(of(4));
    let low: Vec<Card> = mine
        .iter()
        .filter(|x| x.suit() == Some(suit) && x.strength() <= 3)
        .collect();
    if fork
        && low.len() >= 2
        && low.contains(&c)
        && low.iter().all(|x| x.strength() <= c.strength())
    {
        t.add(wt.spare_lead, "spare-lead");
    }
}

/// The bid rule: the strength of the hand in thirds of the points of its sure
/// trick winners, and its tarocks. M&S: take up with sure winners worth 20 to
/// 25 points (60 to 75 thirds) and 6 or 7 tarocks [07 1.1]; that is a strong
/// hand, one in twenty (`research/07` 5.4), so the strict book rule almost
/// never bids and the threshold here is lower, set by measurement
/// (`docs/OPPONENT.md`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BidRule {
    pub sure_thirds: u32,
    pub min_tarocks: usize,
    /// Tarocks from which the Pagat counts as a sure winner (Normal 6; MS
    /// always counts it: 0).
    pub pagat_guard: usize,
    /// Whether a queen counts as a sure winner only with its king or from
    /// three cards of its suit (Normal), or always (Bermann: "Figuren").
    pub queens: Queens,
    /// A writer's condition beside the thresholds (`docs/STYLES.md` 2.1).
    pub extra: BidExtra,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Queens {
    Guarded,
    Always,
}

/// What a writer's bid adds to, or replaces in, the thresholds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BidExtra {
    None,
    /// U 77-78: four or more honours (the Troule cards and the kings) take up
    /// at once, any tarocks; three honours and no queen do not, as dealer.
    Ulmann,
    /// W 131: three honours, six tarocks and a guarded queen take up, and
    /// nothing else does.
    Werner,
}

pub const BID_RULE: BidRule = BidRule {
    sure_thirds: 45,
    min_tarocks: 5,
    pagat_guard: 6,
    queens: Queens::Guarded,
    extra: BidExtra::None,
};

/// Sure trick winners in `hand`, in thirds: the Sküs, Mond and kings
/// (13 each), the Pagat when well guarded by tarocks (Werner counts a weakly
/// held Pagat for nothing [07 1.1]), queens with their king or from three
/// cards (10 each). Also the tarock count.
pub fn sure_thirds(hand: CardSet) -> (u32, usize) {
    sure_thirds_with(hand, BID_RULE)
}

/// [`sure_thirds`] counted as `rule` counts (its Pagat guard and queen rule).
pub fn sure_thirds_with(hand: CardSet, rule: BidRule) -> (u32, usize) {
    let tarocks = hand.iter().filter(|c| c.is_tarock()).count();
    let mut sum = 0;
    for c in hand.iter() {
        if c == Card::SKUS || c == Card::MOND {
            sum += 13;
        } else if c == Card::PAGAT {
            if tarocks >= rule.pagat_guard {
                sum += 13;
            }
        } else if let Some(suit) = c.suit() {
            let len = hand.iter().filter(|x| x.suit() == Some(suit)).count();
            match c.strength() {
                7 => sum += 13,
                6 if rule.queens == Queens::Always
                    || hand.contains(Card::suit_card(suit, 7))
                    || len >= 3 =>
                {
                    sum += 10
                }
                _ => {}
            }
        }
    }
    (sum, tarocks)
}

/// The honours in `hand`: the Troule cards and the kings.
fn honours(hand: CardSet) -> usize {
    hand.iter().filter(|c| c.is_trull() || c.is_king()).count()
}

/// Whether to take up, and why (a message key), not as dealer.
pub fn bid_with(hand: CardSet, rule: BidRule) -> (bool, &'static str) {
    bid_as(hand, rule, false)
}

/// Whether to take up, and why; `dealer` is the Rückhand, who declares second
/// (RULES 3.2).
pub fn bid_as(hand: CardSet, rule: BidRule, dealer: bool) -> (bool, &'static str) {
    let (sure, tarocks) = sure_thirds_with(hand, rule);
    let strong = (true, "bid-strong-hand");
    match rule.extra {
        BidExtra::Werner => {
            let guarded_queen = Suit::ALL.iter().any(|&s| {
                let q = Card::suit_card(s, 6);
                let len = hand.iter().filter(|x| x.suit() == Some(s)).count();
                hand.contains(q) && (hand.contains(Card::suit_card(s, 7)) || len >= 3)
            });
            return if honours(hand) >= 3 && tarocks >= 6 && guarded_queen {
                strong
            } else {
                (false, "bid-weak-hand")
            };
        }
        BidExtra::Ulmann => {
            let n = honours(hand);
            if n >= 4 {
                return strong;
            }
            let queen = Suit::ALL
                .iter()
                .any(|&s| hand.contains(Card::suit_card(s, 6)));
            if dealer && n == 3 && !queen {
                return (false, "bid-weak-hand");
            }
        }
        BidExtra::None => {}
    }
    if tarocks < rule.min_tarocks {
        (false, "bid-few-tarocks")
    } else if sure >= rule.sure_thirds {
        strong
    } else {
        (false, "bid-weak-hand")
    }
}

pub fn bid(k: &Knowledge, rule: BidRule) -> (bool, &'static str) {
    bid_as(k.hand, rule, k.me != k.forehand)
}

/// The play for a card: announcing the honours it belongs to, which is free
/// and which the card would otherwise forfeit (RULES 11.3, 11.5).
pub fn play_card(k: &Knowledge, card: Card) -> Action {
    let announce: Vec<Honour> = k
        .announceable
        .iter()
        .copied()
        .filter(|h| h.cards().contains(&card))
        .collect();
    Action::Play { card, announce }
}

/// The best-scoring card, ties broken by `rng`.
pub fn best_card(k: &Knowledge, rng: &mut Rng, wt: &Weights) -> Choice {
    let scored = score_cards(k, wt);
    let top = scored[0].score;
    let tied: Vec<&Choice> = scored.iter().filter(|c| top - c.score < 1e-9).collect();
    if wt.stable_ties {
        return *tied[0];
    }
    *tied[rng.below(tied.len() as u64) as usize]
}

/// A decision at rung 2 with the standard bid rule.
pub fn decide(view: &View, rng: &mut Rng) -> Action {
    decide_with(view, rng, BID_RULE, &Weights::default())
}

/// A decision at rung 2 with the given bid rule.
pub fn decide_with(view: &View, rng: &mut Rng, rule: BidRule, wt: &Weights) -> Action {
    if matches!(view.legal_actions.first(), Some(Action::Extend { .. })) {
        return Action::Extend { accept: false };
    }
    let k = Knowledge::from_view(view).expect("a deal is in progress");
    if !k.playing {
        return if bid(&k, rule).0 {
            Action::Take
        } else {
            Action::Pass
        };
    }
    play_card(&k, best_card(&k, rng, wt).card)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{c, know, walk};
    use strohmandeln_core::view::view;

    const NO: [&str; 3] = ["", "", ""];

    fn pick(k: &Knowledge) -> Card {
        best_card(k, &mut Rng::seeded(1), &Weights::default()).card
    }

    #[test]
    fn follow_cheaply_and_finesse_do_not_beat_a_poor_lead_with_the_king() {
        let k = know(&["Kh", "4h", "9c"], NO, NO, &[], Some("2h"));
        assert_eq!(pick(&k), c("4h"));
    }

    #[test]
    fn take_tricks_rich_in_card_points() {
        let k = know(&["Kh", "4h", "9c"], NO, NO, &[], Some("Qh"));
        assert_eq!(pick(&k), c("Kh"));
    }

    #[test]
    fn win_with_the_cheapest_winner() {
        let k = know(&["Kh", "Qh", "9c"], NO, NO, &[], Some("Jh"));
        assert_eq!(pick(&k), c("Qh"));
    }

    #[test]
    fn rules_of_thumb_play_the_face_up_packet_cards_off() {
        // Void in the suit led, no tarock: every card is a one-third discard;
        // a cover goes before a hand card.
        let k = know(
            &["4d", "3d", "2d", "1d"],
            ["4h", "", ""],
            NO,
            &[],
            Some("9s"),
        );
        assert!(k.is_cover(pick(&k)));
    }

    #[test]
    fn do_not_lead_an_evenly_split_suit() {
        // Qh Ch 1h are 8 of the 18 points of hearts: split. Leading a club
        // pip is better.
        let hand = ["Qh", "Ch", "1h", "7c", "8c", "T5", "T6", "T7", "T8", "T9"];
        let k = know(&hand, NO, NO, &[], None);
        let p = pick(&k);
        assert_eq!(p.suit(), Some(Suit::Clubs), "{p:?}");
    }

    #[test]
    fn hold_trump_one_for_the_last_trick_when_strong_in_tarocks() {
        let hand = ["T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "9c"];
        let k = know(&hand, NO, NO, &[], Some("Kh"));
        assert_eq!(pick(&k), c("T2"));
    }

    #[test]
    fn with_few_tarocks_cash_the_pagat_on_a_rich_trick() {
        let k = know(&["T1", "T9", "T10", "9c"], NO, NO, &[], Some("Kh"));
        assert_eq!(pick(&k), c("T1"));
    }

    #[test]
    fn keep_one_packet_unplayed_when_a_pagat_ultimo_threatens() {
        // He has shown six tarocks; I have three equal covers and no card to
        // follow with: the first packet (the fullest, first on ties) stays.
        let known = ["T3", "T4", "T5", "T6", "T7", "T8"];
        let k = know(&["4d"], ["2h", "3h", "4h"], NO, &known, Some("9s"));
        assert_eq!(reserved_packet(&k), Some(0));
        let p = pick(&k);
        assert_ne!(p, c("2h"));
        // Without the threat the first cover is as good as any.
        let k = know(&["4d"], ["2h", "3h", "4h"], NO, &[], Some("9s"));
        assert_eq!(reserved_packet(&k), None);
    }

    #[test]
    fn bids_by_hand_strength() {
        let strong = CardSet::of(
            [
                "SK", "T21", "T1", "T10", "T9", "T8", "T7", "Kh", "Kd", "Qh", "Qd", "5c", "7s",
            ]
            .iter()
            .filter_map(|s| Card::parse(s))
            .chain([c("7s")]),
        );
        assert!(bid_with(strong, BID_RULE).0);
        let weak = CardSet::of(
            [
                "T2", "T3", "T4", "T5", "T6", "Jh", "Jd", "7c", "8c", "9c", "10c", "7s", "8s",
                "9s", "10s",
            ]
            .iter()
            .map(|s| c(s)),
        );
        assert!(!bid_with(weak, BID_RULE).0);
        // plenty of honours but few tarocks: no
        let few = CardSet::of(
            ["SK", "T21", "Kh", "Kd", "Kc", "Ks", "Qh", "Qd", "Qc", "Qs"]
                .iter()
                .map(|s| c(s)),
        );
        assert_eq!(bid_with(few, BID_RULE), (false, "bid-few-tarocks"));
    }

    #[test]
    fn the_book_rule_would_almost_never_bid_and_this_one_does() {
        // 20 points of sure winners (60 thirds) with 6 tarocks: research/07 5.4.
        let book = BidRule {
            sure_thirds: 60,
            min_tarocks: 6,
            ..BID_RULE
        };
        let (mut book_bids, mut ours, mut n) = (0, 0, 0);
        for seed in 0..400u64 {
            let mut deck: Vec<Card> = Card::all().collect();
            Rng::seeded(seed).shuffle(&mut deck);
            let hand = CardSet::of(deck[..15].iter().copied());
            book_bids += usize::from(bid_with(hand, book).0);
            ours += usize::from(bid_with(hand, BID_RULE).0);
            n += 1;
        }
        assert!(book_bids * 100 < n * 8, "the book bids {book_bids} of {n}");
        assert!(
            ours * 100 > n * 5 && ours * 100 < n * 60,
            "we bid {ours} of {n}"
        );
    }

    #[test]
    fn it_always_chooses_a_legal_action_and_is_a_function_of_the_view() {
        let mut moves = 0;
        for seed in 0..30 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let a = decide(&v, &mut Rng::seeded(seed));
                assert_eq!(a, decide(&v, &mut Rng::seeded(seed)));
                match &a {
                    Action::Play { card, announce } => {
                        assert!(v.legal_actions.contains(&Action::play(*card)));
                        for h in announce {
                            assert!(v.legal_actions.contains(&Action::Announce { honour: *h }));
                        }
                    }
                    Action::Extend { accept: false } => {}
                    other => assert!(v.legal_actions.contains(other), "{other:?}"),
                }
                moves += 1;
            });
        }
        assert!(moves > 5000);
    }

    #[test]
    fn hidden_cards_never_change_a_decision() {
        // Twin deals that differ only in cards this seat cannot see, with the
        // same moves made and the very same view: the answer is the same.
        let twins = crate::testutil::blind_check(&|v| decide(v, &mut Rng::seeded(5)), 40);
        assert!(
            twins > 200,
            "only {twins} twins; the check would be vacuous"
        );
    }

    #[test]
    fn it_announces_an_honour_when_it_plays_a_card_of_it() {
        let mut found = false;
        for seed in 0..300u64 {
            let mut s =
                strohmandeln_core::session::Sitting::new(seed, Default::default(), 3).unwrap();
            let first = s.to_move().unwrap();
            s.apply(first, Action::Take).unwrap();
            if let Some(seat) = s.to_move() {
                let v = view(&s, seat);
                let k = Knowledge::from_view(&v).unwrap();
                if !k.announceable.is_empty() {
                    let honour = k.announceable[0];
                    let card = honour.cards().into_iter().find(|c| k.legal.contains(c));
                    if let Some(card) = card {
                        let Action::Play { announce, .. } = play_card(&k, card) else {
                            panic!()
                        };
                        assert!(announce.contains(&honour));
                        s.apply(seat, play_card(&k, card)).unwrap();
                        found = true;
                    }
                }
            }
            if found {
                break;
            }
        }
        assert!(found);
    }
}
