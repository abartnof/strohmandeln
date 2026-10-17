//! The tutor's candidate rules as tests of a view and a move (`docs/TUTOR.md`
//! 11, M5): each has a **trigger** (a position the rule is about) and a
//! **prescription** (what the rule says to do there). A rule's precision is
//! how often the strongest rung does what it prescribes when the trigger
//! fires (`docs/MEASUREMENTS.md`, M5); only a rule that passes the gate is
//! told, otherwise the fact. The tests are pure functions of the mover's
//! [`Knowledge`] and the action taken, so they use no hidden card.

use crate::cardset::leader_wins;
use crate::know::Knowledge;
use crate::rung2::sure_thirds;
use crate::tutor::{classify_announce, is_split, shown_void, suit_share, Skill};
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::types::Action;

/// A candidate rule. `check` is `None` where the trigger does not fire, and
/// otherwise whether the move does what the rule prescribes.
#[derive(Copy, Clone)]
pub struct Rule {
    /// The candidate's id, `<family>.<n>`; the family is the rule of
    /// `docs/TUTOR.md` 9 it is a candidate for.
    pub id: &'static str,
    pub skill: Skill,
    /// What trigger and prescription are, in a line.
    pub text: &'static str,
    pub check: fn(&Knowledge, &Action) -> Option<bool>,
}

impl Rule {
    /// The rule this candidate competes for: the id up to its last dot.
    pub fn family(&self) -> &'static str {
        self.id.rsplit_once('.').map_or(self.id, |(f, _)| f)
    }

    /// Whether the candidate can ship: the `part` candidates are diagnostics
    /// of one half of a rule.
    pub fn eligible(&self) -> bool {
        !self.id.contains(".part.")
    }
}

fn played(a: &Action) -> Option<Card> {
    match a {
        Action::Play { card, .. } => Some(*card),
        _ => None,
    }
}

/// Whether the follower holds a legal card that beats the lead and one that
/// does not.
fn choice_to_win(k: &Knowledge) -> Option<(Card, Vec<Card>, Vec<Card>)> {
    let led = k.lead?;
    let (win, lose): (Vec<Card>, Vec<Card>) = k.legal.iter().partition(|&&c| !leader_wins(led, c));
    (!win.is_empty() && !lose.is_empty()).then_some((led, win, lose))
}

fn rich(k: &Knowledge, a: &Action, least: u32) -> Option<bool> {
    let (led, win, _) = choice_to_win(k)?;
    let card = played(a)?;
    (led.thirds() >= least && !win.is_empty()).then(|| win.contains(&card))
}

pub(crate) fn poor_king(k: &Knowledge, a: &Action, at_most: u32) -> Option<bool> {
    let led = k.lead?;
    let card = played(a)?;
    let king_wins = k.legal.iter().any(|&c| c.is_king() && !leader_wins(led, c));
    let other = k.legal.iter().any(|c| !c.is_king());
    (led.thirds() <= at_most && king_wins && other).then(|| !card.is_king())
}

/// The cheapest card of the class the card played is in (it wins, or it
/// does not), by what it costs, then by strength.
pub(crate) fn cheapest(k: &Knowledge, a: &Action, winners_only: bool) -> Option<bool> {
    let led = k.lead?;
    let card = played(a)?;
    let wins = |c: Card| !leader_wins(led, c);
    if winners_only && !wins(card) {
        return None;
    }
    let class: Vec<Card> = k
        .legal
        .iter()
        .copied()
        .filter(|&c| wins(c) == wins(card))
        .collect();
    let cheapest = class
        .iter()
        .copied()
        .min_by_key(|c| (c.thirds(), c.strength()))?;
    (class.len() >= 2)
        .then(|| (card.thirds(), card.strength()) == (cheapest.thirds(), cheapest.strength()))
}

/// A cover and a hand card are both legal.
pub(crate) fn cover_choice(k: &Knowledge) -> bool {
    k.legal.iter().any(|&c| k.is_cover(c)) && k.legal.iter().any(|&c| !k.is_cover(c))
}

pub(crate) fn off_any(k: &Knowledge, a: &Action, leading: bool) -> Option<bool> {
    let card = played(a)?;
    (k.lead.is_none() == leading && cover_choice(k)).then(|| k.is_cover(card))
}

/// The legal figure covers (Dame or Cavall of a suit) when a hand card is
/// also legal.
pub(crate) fn figures(k: &Knowledge) -> Vec<Card> {
    if !cover_choice(k) {
        return Vec::new();
    }
    k.legal
        .iter()
        .copied()
        .filter(|&c| k.is_cover(c) && c.suit().is_some() && matches!(c.strength(), 5 | 6))
        .collect()
}

/// He shows two low cards (a pip below the Cavall) of `suit` on his packets.
fn shows_two_low(k: &Knowledge, suit: Suit) -> bool {
    k.opp_covers
        .iter()
        .flatten()
        .filter(|c| c.suit() == Some(suit) && c.strength() <= 4)
        .count()
        >= 2
}

/// Of the figures, the one the rule says to play: the higher.
pub(crate) fn first_figure(figs: &[Card]) -> Option<Card> {
    figs.iter().copied().max_by_key(|c| c.strength())
}

pub(crate) fn figure_any(k: &Knowledge, a: &Action) -> Option<bool> {
    let card = played(a)?;
    (!figures(k).is_empty()).then(|| k.is_cover(card))
}

pub(crate) fn figure_played(k: &Knowledge, a: &Action, exception: bool) -> Option<bool> {
    let card = played(a)?;
    let mut figs = figures(k);
    if exception {
        figs.retain(|f| !shows_two_low(k, f.suit().expect("a suit card")));
    }
    let want = first_figure(&figs)?;
    Some(card == want)
}

/// The tarocks I hold (hand and covers), and the runs among them: maximal
/// blocks of cards that are neighbours in the order of the tarocks not yet
/// played (mine and unplaced), so a gap of cards still out breaks a run.
fn tarock_runs(k: &Knowledge) -> (Vec<Card>, Vec<Vec<Card>>) {
    let mine: Vec<Card> = k.playable().iter().filter(|c| c.is_tarock()).collect();
    let mut live: Vec<Card> = Card::all()
        .filter(|c| c.is_tarock() && !k.played.contains(*c))
        .collect();
    live.sort_by_key(|c| c.strength());
    let mut runs: Vec<Vec<Card>> = Vec::new();
    let mut current: Vec<Card> = Vec::new();
    for c in live {
        if mine.contains(&c) {
            current.push(c);
        } else if !current.is_empty() {
            runs.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        runs.push(current);
    }
    runs.retain(|r| r.len() >= 2);
    (mine, runs)
}

/// The leader leads a tarock, holding at least two.
fn tarock_lead(k: &Knowledge, a: &Action) -> Option<(Card, Vec<Card>, Vec<Vec<Card>>)> {
    let card = played(a)?;
    let (mine, runs) = tarock_runs(k);
    (k.lead.is_none() && card.is_tarock() && mine.len() >= 2).then_some((card, mine, runs))
}

fn lowest(cards: &[Card]) -> Option<Card> {
    cards.iter().copied().min_by_key(|c| c.strength())
}

pub(crate) fn run_top(k: &Knowledge, a: &Action) -> Option<bool> {
    let (card, mine, runs) = tarock_lead(k, a)?;
    Some(match runs.iter().max_by_key(|r| r.len()) {
        Some(longest) => longest.iter().max_by_key(|c| c.strength()) == Some(&card),
        None => lowest(&mine) == Some(card),
    })
}

pub(crate) fn run_any(k: &Knowledge, a: &Action) -> Option<bool> {
    let (card, mine, runs) = tarock_lead(k, a)?;
    Some(if runs.is_empty() {
        lowest(&mine) == Some(card)
    } else {
        runs.iter().any(|r| r.contains(&card))
    })
}

fn run_in_a_run(k: &Knowledge, a: &Action) -> Option<bool> {
    let (card, _, runs) = tarock_lead(k, a)?;
    (!runs.is_empty()).then(|| runs.iter().any(|r| r.contains(&card)))
}

fn run_no_run(k: &Knowledge, a: &Action) -> Option<bool> {
    let (card, mine, runs) = tarock_lead(k, a)?;
    runs.is_empty().then(|| lowest(&mine) == Some(card))
}

/// Following with the Pagat and another card legal.
fn pagat_choice(k: &Knowledge) -> bool {
    k.lead.is_some() && k.legal.contains(&Card::PAGAT) && k.legal.len() >= 2
}

pub(crate) fn pagat_cash(k: &Knowledge, a: &Action, at_most: usize) -> Option<bool> {
    let card = played(a)?;
    (pagat_choice(k) && k.my_tarocks() <= at_most).then(|| card == Card::PAGAT)
}

pub(crate) fn pagat_keep(k: &Knowledge, a: &Action, at_least: usize) -> Option<bool> {
    let card = played(a)?;
    (pagat_choice(k) && k.my_tarocks() >= at_least).then(|| card != Card::PAGAT)
}

pub(crate) fn void_lead(k: &Knowledge, a: &Action) -> Option<bool> {
    let card = played(a)?;
    let into = |c: Card| c.suit().is_some_and(|s| shown_void(k, s));
    (k.lead.is_none() && k.legal.iter().any(|&c| into(c)) && k.legal.iter().any(|&c| !into(c)))
        .then(|| !into(card))
}

fn behind_not_split(k: &Knowledge, a: &Action) -> Option<bool> {
    let card = played(a)?;
    let split = |c: Card| c.suit().is_some_and(|s| is_split(k, s));
    (k.lead.is_none() && k.legal.iter().any(|&c| split(c)) && k.legal.iter().any(|&c| !split(c)))
        .then(|| !split(card))
}

fn behind_tarock(k: &Knowledge, a: &Action) -> Option<bool> {
    let card = played(a)?;
    let split = |c: Card| c.suit().is_some_and(|s| is_split(k, s));
    (k.lead.is_none() && k.legal.iter().any(|&c| split(c)) && k.legal.iter().any(|c| c.is_tarock()))
        .then(|| card.is_tarock())
}

/// A lead in a suit I hold most of (share above the split band) or a tarock.
fn behind_closed(k: &Knowledge, a: &Action) -> Option<bool> {
    let card = played(a)?;
    let split = |c: Card| c.suit().is_some_and(|s| is_split(k, s));
    let mostly_mine = |c: Card| {
        c.suit().is_some_and(|s| {
            let (mine, total) = suit_share(k, s);
            total > 0 && f64::from(mine) / f64::from(total) > crate::tutor::SPLIT_SHARE.1
        })
    };
    let good = |c: Card| c.is_tarock() || mostly_mine(c);
    (k.lead.is_none() && k.legal.iter().any(|&c| split(c)) && k.legal.iter().any(|&c| good(c)))
        .then(|| good(card))
}

fn bid_take(k: &Knowledge, a: &Action, sure: (u32, u32), tarocks: (usize, usize)) -> Option<bool> {
    if k.playing || !matches!(a, Action::Take | Action::Pass) {
        return None;
    }
    let (s, t) = sure_thirds(k.hand);
    ((sure.0..=sure.1).contains(&s) && (tarocks.0..=tarocks.1).contains(&t))
        .then_some(matches!(a, Action::Take))
}

fn announce_made(k: &Knowledge, a: &Action) -> Option<bool> {
    classify_announce(k, a).map(|v| v.met)
}

macro_rules! rule {
    ($id:expr, $skill:expr, $text:expr, $f:expr) => {
        Rule {
            id: $id,
            skill: $skill,
            text: $text,
            check: $f,
        }
    };
}

/// The candidates, written before the first selection run (`docs/MEASUREMENTS.md`
/// M5). Thirds: value 1, 2, 3, 4, 5 are 1, 4, 7, 10, 13.
pub const CANDIDATES: &[Rule] = &[
    rule!("worth.rich.1", Skill::Worth, "follower, a card led worth 7 thirds or more, can win and can not: win", |k, a| rich(k, a, 7)),
    rule!("worth.rich.2", Skill::Worth, "... led worth 10 thirds or more: win", |k, a| rich(k, a, 10)),
    rule!("worth.rich.3", Skill::Worth, "... led worth 13 thirds (king or honour): win", |k, a| rich(k, a, 13)),
    rule!("worth.poor.1", Skill::Worth, "follower, led worth 1 third, a king wins and another card is legal: no king", |k, a| poor_king(k, a, 1)),
    rule!("worth.poor.2", Skill::Worth, "... led worth 4 thirds or less: no king", |k, a| poor_king(k, a, 4)),
    rule!("cheap.1", Skill::Cheap, "follower, two or more cards of the outcome played: the cheapest of them (thirds, then strength)", |k, a| cheapest(k, a, false)),
    rule!("cheap.2", Skill::Cheap, "... only when the card played wins: the cheapest winner", |k, a| cheapest(k, a, true)),
    rule!("off.pip.1", Skill::Off, "follower, a cover and a hand card legal: a cover", |k, a| off_any(k, a, false)),
    rule!("off.pip.2", Skill::Off, "leader, a cover and a hand card legal: a cover", |k, a| off_any(k, a, true)),
    rule!("off.figure.1", Skill::Off, "a Dame or Cavall cover and a hand card legal: a cover", figure_any),
    rule!("off.figure.2", Skill::Off, "... the higher figure cover itself", |k, a| figure_played(k, a, false)),
    rule!("off.figure.3", Skill::Off, "... the higher figure cover itself, unless he shows two low cards of its suit", |k, a| figure_played(k, a, true)),
    rule!("run.1", Skill::Run, "leader leading a tarock from two or more: the top of the longest solid run, else the lowest", run_top),
    rule!("run.2", Skill::Run, "... a card of any solid run, else the lowest", run_any),
    rule!("run.part.3", Skill::Run, "... holding a solid run: a card of a run", run_in_a_run),
    rule!("run.part.4", Skill::Run, "... holding no run: the lowest tarock", run_no_run),
    rule!("pagat.cash.1", Skill::Pagat, "follower with the Pagat and another card, 7 tarocks or fewer: the Pagat", |k, a| pagat_cash(k, a, 7)),
    rule!("pagat.cash.2", Skill::Pagat, "... 8 tarocks or fewer: the Pagat", |k, a| pagat_cash(k, a, 8)),
    rule!("pagat.keep.1", Skill::Pagat, "follower with the Pagat and another card, 12 tarocks or more: keep it", |k, a| pagat_keep(k, a, 12)),
    rule!("pagat.keep.2", Skill::Pagat, "... 11 tarocks or more: keep it", |k, a| pagat_keep(k, a, 11)),
    rule!("void.1", Skill::Void, "leader with a lead into a suit he has shown void and another: not into it", void_lead),
    rule!("behind.1", Skill::Behind, "leader with a lead in a split suit and another: not in a split suit", behind_not_split),
    rule!("behind.2", Skill::Behind, "... and a tarock: lead a tarock", behind_tarock),
    rule!("behind.3", Skill::Behind, "... and a tarock or a suit mostly mine: lead one of those", behind_closed),
    rule!("bid.1", Skill::Bid, "20 to 25 points (60 to 75 thirds) of sure winners and 6 or 7 tarocks: take up", |k, a| bid_take(k, a, (60, 75), (6, 7))),
    rule!("bid.2", Skill::Bid, "20 points or more of sure winners and 6 tarocks or more: take up", |k, a| bid_take(k, a, (60, 999), (6, 99))),
    rule!("announce.1", Skill::Announce, "about to play a card of a held honour: announce it", announce_made),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{c, know};

    const NO: [&str; 3] = ["", "", ""];
    const HID: [&str; 3] = ["T2", "", ""];

    fn rule(id: &str) -> Rule {
        *CANDIDATES.iter().find(|r| r.id == id).expect(id)
    }

    fn check(id: &str, k: &Knowledge, card: &str) -> Option<bool> {
        (rule(id).check)(k, &Action::play(c(card)))
    }

    #[test]
    fn the_candidates_have_distinct_ids_and_the_skill_of_their_family() {
        let mut ids: Vec<&str> = CANDIDATES.iter().map(|r| r.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), CANDIDATES.len());
        for r in CANDIDATES {
            let family = r.family();
            let slug = family.split('.').next().unwrap();
            assert_eq!(slug, r.skill.slug(), "{}", r.id);
        }
    }

    #[test]
    fn worth_rich_fires_on_a_rich_lead_with_a_choice() {
        // Qh led (10 thirds); Kh wins, 4h does not
        let k = know(&["Kh", "4h", "9c"], NO, HID, &[], Some("Qh"));
        assert_eq!(check("worth.rich.1", &k, "Kh"), Some(true));
        assert_eq!(check("worth.rich.1", &k, "4h"), Some(false));
        assert_eq!(check("worth.rich.2", &k, "Kh"), Some(true));
        assert_eq!(
            check("worth.rich.3", &k, "Kh"),
            None,
            "a Dame is not 13 thirds"
        );
        // a pip led: no trigger
        let k = know(&["Kh", "4h"], NO, HID, &[], Some("3h"));
        assert_eq!(check("worth.rich.1", &k, "Kh"), None);
        // no choice: every card wins
        let k = know(&["Kh"], NO, HID, &[], Some("Jh"));
        assert_eq!(check("worth.rich.1", &k, "Kh"), None);
    }

    #[test]
    fn worth_poor_asks_for_no_king_on_a_poor_trick() {
        let k = know(&["Kh", "Qh", "9c"], NO, HID, &[], Some("3h"));
        assert_eq!(check("worth.poor.1", &k, "Kh"), Some(false));
        assert_eq!(check("worth.poor.1", &k, "Qh"), Some(true));
        // a rich lead is not a poor trick
        let k = know(&["Kh", "Qh"], NO, HID, &[], Some("Ch"));
        assert_eq!(check("worth.poor.2", &k, "Kh"), None);
    }

    #[test]
    fn cheap_wants_the_cheapest_of_the_cards_that_give_the_outcome() {
        let k = know(&["Kh", "Qh", "2h"], NO, HID, &[], Some("3h"));
        assert_eq!(check("cheap.1", &k, "2h"), Some(true));
        assert_eq!(check("cheap.1", &k, "Kh"), Some(false));
        assert_eq!(check("cheap.2", &k, "Qh"), Some(false));
        // a lone card of its outcome: no trigger
        let k = know(&["Kh", "4h"], NO, HID, &[], Some("3h"));
        assert_eq!(check("cheap.1", &k, "Kh"), None);
    }

    #[test]
    fn off_asks_for_a_cover_when_a_cover_and_a_hand_card_are_legal() {
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        assert_eq!(check("off.pip.2", &k, "Qh"), Some(true));
        assert_eq!(check("off.pip.2", &k, "Kh"), Some(false));
        assert_eq!(check("off.pip.1", &k, "Qh"), None, "he is leading");
        // no hand card to choose
        let k = know(&["Kd"], ["Qh", "", ""], HID, &[], Some("3h"));
        assert_eq!(check("off.pip.1", &k, "Qh"), None);
    }

    #[test]
    fn off_figure_and_its_exception() {
        let k = know(&["Kh", "4h"], ["Qh", "", ""], HID, &[], None);
        assert_eq!(check("off.figure.1", &k, "Qh"), Some(true));
        assert_eq!(check("off.figure.2", &k, "Qh"), Some(true));
        assert_eq!(check("off.figure.2", &k, "Kh"), Some(false));
        // he shows two low hearts: the exception removes the trigger
        let shy = know(&["Kh", "4h"], ["Qh", "", ""], ["2h", "3h", ""], &[], None);
        assert_eq!(check("off.figure.2", &shy, "Kh"), Some(false));
        assert_eq!(check("off.figure.3", &shy, "Kh"), None);
        // one low heart is not two
        let one = know(&["Kh", "4h"], ["Qh", "", ""], ["2h", "", ""], &[], None);
        assert_eq!(check("off.figure.3", &one, "Kh"), Some(false));
        // a pip cover is not a figure
        let pip = know(&["Kh", "4h"], ["2h", "", ""], HID, &[], None);
        assert_eq!(check("off.figure.1", &pip, "2h"), None);
        // of two figures, the higher
        let two = know(&["4h"], ["Qh", "Ch", ""], HID, &[], None);
        assert_eq!(check("off.figure.2", &two, "Qh"), Some(true));
        assert_eq!(check("off.figure.2", &two, "Ch"), Some(false));
    }

    #[test]
    fn run_leads_the_top_of_a_solid_run_else_the_lowest() {
        // T9 T8 T7 are neighbours among the tarocks not played: a run
        let k = know(&["T9", "T8", "T7", "T3", "Kh"], NO, HID, &[], None);
        assert_eq!(check("run.1", &k, "T9"), Some(true));
        assert_eq!(check("run.1", &k, "T7"), Some(false));
        assert_eq!(check("run.2", &k, "T7"), Some(true));
        assert_eq!(check("run.2", &k, "T3"), Some(false));
        assert_eq!(check("run.part.3", &k, "T8"), Some(true));
        assert_eq!(check("run.part.4", &k, "T3"), None, "there is a run");
        // no two neighbours: the lowest
        let k = know(&["T9", "T5", "T3", "Kh"], NO, HID, &[], None);
        assert_eq!(check("run.1", &k, "T3"), Some(true));
        assert_eq!(check("run.part.4", &k, "T3"), Some(true));
        assert_eq!(check("run.part.4", &k, "T9"), Some(false));
        assert_eq!(check("run.part.3", &k, "T9"), None);
        // a card played between two of mine closes the gap
        let mut k = know(&["T9", "T7", "Kh"], NO, HID, &[], None);
        k.played.insert(c("T8"));
        assert_eq!(check("run.part.3", &k, "T9"), Some(true));
        // a suit lead or a single tarock: no trigger
        assert_eq!(check("run.1", &k, "Kh"), None);
        let single = know(&["T9", "Kh"], NO, HID, &[], None);
        assert_eq!(check("run.1", &single, "T9"), None);
        // following: no trigger
        let f = know(&["T9", "T7"], NO, HID, &[], Some("T5"));
        assert_eq!(check("run.1", &f, "T9"), None);
    }

    #[test]
    fn pagat_cash_and_keep_by_the_tarocks_held() {
        let few = know(&["T1", "T9", "Kh"], NO, HID, &[], Some("3d"));
        assert_eq!(check("pagat.cash.1", &few, "T1"), Some(true));
        assert_eq!(check("pagat.cash.1", &few, "T9"), Some(false));
        assert_eq!(check("pagat.keep.1", &few, "T9"), None);
        let many_cards = [
            "T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "T9", "T10", "T11", "T12",
        ];
        let many = know(&many_cards, NO, HID, &[], Some("T15"));
        assert_eq!(check("pagat.keep.1", &many, "T2"), Some(true));
        assert_eq!(check("pagat.keep.1", &many, "T1"), Some(false));
        assert_eq!(check("pagat.cash.1", &many, "T1"), None);
        // eleven tarocks: the second candidate fires, the first does not
        let eleven = know(&many_cards[..11], NO, HID, &[], Some("T15"));
        assert_eq!(check("pagat.keep.1", &eleven, "T2"), None);
        assert_eq!(check("pagat.keep.2", &eleven, "T2"), Some(true));
        // leading: no trigger
        let lead = know(&["T1", "T9"], NO, HID, &[], None);
        assert_eq!(check("pagat.cash.1", &lead, "T1"), None);
    }

    #[test]
    fn void_asks_for_a_lead_that_is_not_into_a_shown_void() {
        let mut k = know(&["Kh", "Qd", "T3"], NO, HID, &[], None);
        k.opp_void[0] = true;
        assert_eq!(check("void.1", &k, "Kh"), Some(false));
        assert_eq!(check("void.1", &k, "Qd"), Some(true));
        let clean = know(&["Kh", "Qd"], NO, HID, &[], None);
        assert_eq!(check("void.1", &clean, "Kh"), None);
    }

    #[test]
    fn behind_candidates() {
        // Kh Qh: 9 of the 18 points of hearts are mine, a split suit
        let k = know(&["Kh", "Qh", "9c", "T3"], NO, HID, &[], None);
        assert_eq!(check("behind.1", &k, "Kh"), Some(false));
        assert_eq!(check("behind.1", &k, "9c"), Some(true));
        assert_eq!(check("behind.2", &k, "T3"), Some(true));
        assert_eq!(check("behind.2", &k, "9c"), Some(false));
        assert_eq!(check("behind.3", &k, "T3"), Some(true));
        // a follower: no trigger
        let f = know(&["Kh", "Qh", "9c"], NO, HID, &[], Some("Jh"));
        assert_eq!(check("behind.1", &f, "Kh"), None);
    }

    #[test]
    fn bid_rules_look_at_sure_winners_and_tarocks() {
        let mut k = know(
            &["SK", "T21", "T5", "T6", "T7", "T8", "Kh", "Kd", "Kc"],
            NO,
            HID,
            &[],
            None,
        );
        k.playing = false;
        // five sure winners: 65 thirds, six tarocks
        assert_eq!((rule("bid.1").check)(&k, &Action::Take), Some(true));
        assert_eq!((rule("bid.1").check)(&k, &Action::Pass), Some(false));
        assert_eq!((rule("bid.2").check)(&k, &Action::Take), Some(true));
        // a seventh king: 78 thirds, above the 20 to 25 points of the first
        let mut more = k.clone();
        more.hand.insert(c("Ks"));
        assert_eq!((rule("bid.1").check)(&more, &Action::Take), None);
        assert_eq!((rule("bid.2").check)(&more, &Action::Take), Some(true));
        // too few tarocks
        let mut few = know(&["SK", "T21", "Kh", "Kd", "Kc", "Ks"], NO, HID, &[], None);
        few.playing = false;
        assert_eq!((rule("bid.2").check)(&few, &Action::Take), None);
        // not a bid
        assert_eq!((rule("bid.1").check)(&k, &Action::play(c("Kh"))), None);
    }

    #[test]
    fn announce_is_the_evidence_classifier() {
        use strohmandeln_core::cards::Honour;
        let mut k = know(&["SK", "T21", "T1"], NO, HID, &[], None);
        k.announceable = vec![Honour::Trull];
        assert_eq!(check("announce.1", &k, "T21"), Some(false));
        let said = Action::Play {
            card: c("T21"),
            announce: vec![Honour::Trull],
        };
        assert_eq!((rule("announce.1").check)(&k, &said), Some(true));
    }
}
