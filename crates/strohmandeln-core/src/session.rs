//! The sitting (RULES 15): deals in turn, Touren, the Conto, extension and
//! settlement, with a record that replays it exactly.
//!
//! A sitting is fixed by its seed, its settings, its length in Touren and the
//! moves made; `Sitting::replay` rebuilds it from a [`Record`] and reproduces
//! every event.

use crate::cards::Card;
use crate::deal::{Deal, DealEnd};
use crate::dealrec::DealRecord;
use crate::rng::{purpose, Rng};
use crate::types::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// The version of the record format. A record of another version is refused.
pub const RECORD_VERSION: u32 = 1;

/// Kreuzer per Point (RULES 15.8).
pub const KREUZER_PER_POINT: u32 = 2;

/// One move of a sitting: who did what.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Move {
    pub seat: Seat,
    pub action: Action,
}

mod seed_string {
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(seed: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.collect_str(seed)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Seed {
            Number(u64),
            Text(String),
        }
        match Seed::deserialize(d)? {
            Seed::Number(n) => Ok(n),
            Seed::Text(t) => t.parse().map_err(de::Error::custom),
        }
    }
}

/// Everything needed to replay a sitting exactly.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Record {
    pub version: u32,
    /// A string on the wire: a JavaScript number cannot hold every u64.
    #[serde(with = "seed_string")]
    pub seed: u64,
    pub settings: Settings,
    pub touren: u32,
    pub moves: Vec<Move>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SittingPhase {
    Deal,
    /// Extension offered; the number is how many have answered (0 or 1).
    Extension(u8),
    Over,
}

#[derive(Clone, Debug)]
pub struct Sitting {
    seed: u64,
    settings: Settings,
    touren: u32,
    touren_total: u32,
    rng: Rng,
    decks: VecDeque<Vec<Card>>,
    dealer: Seat,
    counted: u32,
    deal: Option<Deal>,
    contos: [u32; 2],
    pending_doubling: bool,
    phase: SittingPhase,
    events: Vec<Event>,
    moves: Vec<Move>,
    /// The deck of the deal in progress, its actions so far, and the record
    /// of the last deal that ended (`docs/TUTOR.md` 3).
    deck: Vec<Card>,
    deal_actions: Vec<Move>,
    last_deal: Option<DealRecord>,
}

impl Sitting {
    /// Begins a sitting of 3 or 6 Touren (RULES 15.2). The first dealer is
    /// drawn by lot (2.1).
    pub fn new(seed: u64, settings: Settings, touren: u32) -> Result<Sitting, SetupError> {
        Sitting::with_decks(seed, settings, touren, Vec::new())
    }

    /// As `new`, with the first deals taken from `decks` instead of the
    /// shuffle (tests).
    pub(crate) fn with_decks(
        seed: u64,
        settings: Settings,
        touren: u32,
        decks: Vec<Vec<Card>>,
    ) -> Result<Sitting, SetupError> {
        if touren != 3 && touren != 6 {
            return Err(SetupError::BadTouren { touren });
        }
        if !(1..=2).contains(&settings.ultimo) {
            return Err(SetupError::BadUltimo {
                ultimo: settings.ultimo,
            });
        }
        let first = Seat::from_index(Rng::stream(seed, purpose::LOT).below(2) as usize);
        let mut s = Sitting {
            seed,
            settings,
            touren,
            touren_total: touren,
            rng: Rng::stream(seed, purpose::DEALS),
            decks: decks.into(),
            dealer: first,
            counted: 0,
            deal: None,
            contos: [0, 0],
            pending_doubling: false,
            phase: SittingPhase::Deal,
            events: Vec::new(),
            moves: Vec::new(),
            deck: Vec::new(),
            deal_actions: Vec::new(),
            last_deal: None,
        };
        let mut ev = vec![Event::SittingStart {
            first_dealer: first,
            touren,
            settings,
        }];
        s.start_deal(&mut ev);
        s.events = ev;
        Ok(s)
    }

    /// Rebuilds a sitting from its record, move by move.
    pub fn replay(record: &Record) -> Result<Sitting, SetupError> {
        if record.version != RECORD_VERSION {
            return Err(SetupError::RecordVersion {
                found: record.version,
                expected: RECORD_VERSION,
            });
        }
        let mut s = Sitting::new(record.seed, record.settings, record.touren)?;
        for (i, m) in record.moves.iter().enumerate() {
            s.apply(m.seat, m.action.clone())
                .map_err(|refusal| SetupError::ReplayRefused { index: i, refusal })?;
        }
        Ok(s)
    }

    pub fn record(&self) -> Record {
        Record {
            version: RECORD_VERSION,
            seed: self.seed,
            settings: self.settings,
            touren: self.touren,
            moves: self.moves.clone(),
        }
    }

    pub fn settings(&self) -> Settings {
        self.settings
    }

    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Every event so far, in order, with nothing hidden. Per-player
    /// filtering is `view::view`.
    pub fn events(&self) -> &[Event] {
        &self.events
    }

    pub fn deal(&self) -> Option<&Deal> {
        self.deal.as_ref()
    }

    /// The record of the last deal that ended, thrown in or scored: replays
    /// that deal without the sitting.
    pub fn last_deal(&self) -> Option<&DealRecord> {
        self.last_deal.as_ref()
    }

    pub fn dealer(&self) -> Seat {
        self.dealer
    }

    pub fn deals_counted(&self) -> u32 {
        self.counted
    }

    pub fn contos(&self) -> [u32; 2] {
        self.contos
    }

    /// Conto(A) minus Conto(B) (RULES 15.5).
    pub fn balance(&self) -> i32 {
        self.contos[0] as i32 - self.contos[1] as i32
    }

    pub fn is_over(&self) -> bool {
        self.phase == SittingPhase::Over
    }

    pub fn pending_doubling(&self) -> bool {
        self.pending_doubling
    }

    /// The multiplier of the deal in progress, or of the next (RULES 14).
    pub fn multiplier(&self) -> u32 {
        if self.pending_doubling {
            2
        } else {
            1
        }
    }

    pub fn touren_total(&self) -> u32 {
        self.touren_total
    }

    pub fn to_move(&self) -> Option<Seat> {
        match self.phase {
            SittingPhase::Deal => self.deal.as_ref().and_then(|d| d.to_move()),
            SittingPhase::Extension(n) => Some(Seat::from_index(usize::from(n))),
            SittingPhase::Over => None,
        }
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        match self.phase {
            SittingPhase::Deal => self
                .deal
                .as_ref()
                .map(|d| d.legal_actions())
                .unwrap_or_default(),
            SittingPhase::Extension(_) => vec![
                Action::Extend { accept: true },
                Action::Extend { accept: false },
            ],
            SittingPhase::Over => Vec::new(),
        }
    }

    /// Applies an action, returning the events it caused, or refuses it and
    /// changes nothing.
    pub fn apply(&mut self, seat: Seat, action: Action) -> Result<Vec<Event>, Refusal> {
        let mut ev = Vec::new();
        match self.phase {
            SittingPhase::Over => return Err(Refusal::of(RefusalKind::Finished)),
            SittingPhase::Extension(asked) => {
                self.answer_extension(seat, &action, asked, &mut ev)?
            }
            SittingPhase::Deal => {
                let deal = self.deal.as_mut().expect("a deal is in progress");
                deal.apply(seat, action.clone(), &mut ev)?;
                self.deal_actions.push(Move {
                    seat,
                    action: action.clone(),
                });
                if let Some(end) = deal.end().cloned() {
                    self.last_deal = Some(DealRecord {
                        deck: self.deck.clone(),
                        dealer: deal.dealer(),
                        multiplier: deal.multiplier(),
                        settings: self.settings,
                        actions: std::mem::take(&mut self.deal_actions),
                    });
                    self.settle(&end, &mut ev);
                }
            }
        }
        self.moves.push(Move { seat, action });
        self.events.extend(ev.iter().cloned());
        Ok(ev)
    }

    fn answer_extension(
        &mut self,
        seat: Seat,
        action: &Action,
        asked: u8,
        ev: &mut Vec<Event>,
    ) -> Result<(), Refusal> {
        let Action::Extend { accept } = action else {
            return Err(Refusal::of(RefusalKind::WrongPhase));
        };
        let expected = Seat::from_index(usize::from(asked));
        if seat != expected {
            return Err(Refusal::of(RefusalKind::NotYourTurn).with_speaker(expected));
        }
        if !accept {
            ev.push(Event::ExtensionDecided {
                accepted: false,
                touren_added: 0,
            });
            self.finish(ev);
        } else if asked == 0 {
            self.phase = SittingPhase::Extension(1);
        } else {
            self.touren_total += self.touren;
            ev.push(Event::ExtensionDecided {
                accepted: true,
                touren_added: self.touren,
            });
            self.dealer = self.dealer.other();
            self.start_deal(ev);
        }
        Ok(())
    }

    fn next_deck(&mut self) -> Vec<Card> {
        if let Some(d) = self.decks.pop_front() {
            return d;
        }
        let mut deck: Vec<Card> = Card::all().collect();
        self.rng.shuffle(&mut deck);
        deck
    }

    /// Deals until a deal has a tarock in both hands (RULES 2.6).
    fn start_deal(&mut self, ev: &mut Vec<Event>) {
        self.phase = SittingPhase::Deal;
        loop {
            let multiplier = self.multiplier();
            let forehand = self.dealer.other();
            ev.push(Event::DealStart {
                deal_number: self.counted + 1,
                tour: self.counted / 2 + 1,
                dealer: self.dealer,
                forehand,
                multiplier,
            });
            if multiplier == 2 {
                ev.push(Event::DoublingApplied { multiplier });
            }
            let deck = self.next_deck();
            let deal = Deal::new(&deck, self.dealer, multiplier, self.settings);
            for seat in [forehand, self.dealer] {
                ev.push(Event::Dealt {
                    seat,
                    cards: deal.hand_cards(seat),
                });
            }
            let void = deal.void_seats();
            if void.is_empty() {
                self.deck = deck;
                self.deal_actions.clear();
                self.deal = Some(deal);
                return;
            }
            ev.push(Event::VoidDeal { seats: void });
        }
    }

    /// A deal is over: credit the Contos, set or clear the doubling, count
    /// the deal, and go on (RULES 14, 15).
    fn settle(&mut self, end: &DealEnd, ev: &mut Vec<Event>) {
        if let DealEnd::Played(score) = end {
            for credit in &score.credits {
                self.contos[credit.seat.index()] += credit.points;
            }
            ev.push(Event::GameScored {
                result: score.result,
                credits: score.credits.clone(),
                multiplier: score.multiplier,
                contos: self.contos,
            });
            self.pending_doubling = score.result == GameResult::Draw;
            if self.pending_doubling {
                ev.push(Event::DoublingSet);
            }
        }
        self.counted += 1;
        if self.counted.is_multiple_of(2) {
            ev.push(Event::TourEnd {
                tour: self.counted / 2,
                contos: self.contos,
                balance: self.balance(),
            });
        }
        if self.counted >= 2 * self.touren_total {
            ev.push(Event::ExtensionOffered);
            self.phase = SittingPhase::Extension(0);
        } else {
            self.dealer = self.dealer.other();
            self.start_deal(ev);
        }
    }

    /// The sitting ends: a pending doubling lapses and the account is settled
    /// (RULES 14.5, 15.7, 15.8).
    fn finish(&mut self, ev: &mut Vec<Event>) {
        if self.pending_doubling {
            self.pending_doubling = false;
            ev.push(Event::DoublingLapsed);
        }
        let balance = self.balance();
        let payer = match balance {
            0 => None,
            b if b > 0 => Some(Seat::B),
            _ => Some(Seat::A),
        };
        ev.push(Event::SittingEnd {
            contos: self.contos,
            balance,
            payer,
            kreuzer: balance.unsigned_abs() * KREUZER_PER_POINT,
        });
        self.deal = None;
        self.phase = SittingPhase::Over;
    }

    #[cfg(test)]
    pub(crate) fn settle_for_test(&mut self, end: &DealEnd) {
        let mut ev = Vec::new();
        self.settle(end, &mut ev);
        self.events.extend(ev);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;
    use crate::deal::{DealEnd, DealScore};

    /// A deck with tarocks for both players (canonical order).
    fn ok_deck() -> Vec<Card> {
        Card::all().collect()
    }

    /// A deck where forehand's 15 hand cards (slots 0-4, 10-14, 20-24) are suit
    /// cards only.
    fn void_deck() -> Vec<Card> {
        let suit: Vec<Card> = Card::all().filter(|c| !c.is_tarock()).collect();
        let tar: Vec<Card> = Card::all().filter(|c| c.is_tarock()).collect();
        let mut deck: Vec<Option<Card>> = vec![None; 54];
        let fh = [0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24];
        for (k, &i) in fh.iter().enumerate() {
            deck[i] = Some(suit[k]);
        }
        let mut rest = suit[15..].iter().chain(tar.iter());
        deck.into_iter()
            .map(|s| s.unwrap_or_else(|| *rest.next().unwrap()))
            .collect()
    }

    fn throw_in() -> Settings {
        Settings {
            both_pass: BothPass::ThrowIn,
            ..Settings::default()
        }
    }

    fn all_pass(s: &mut Sitting) {
        let first = s.to_move().unwrap();
        s.apply(first, Action::Pass).unwrap();
        s.apply(first.other(), Action::Pass).unwrap();
    }

    fn count<F: Fn(&Event) -> bool>(s: &Sitting, f: F) -> usize {
        s.events().iter().filter(|e| f(e)).count()
    }

    fn score(result: GameResult, credits: Vec<Credit>) -> DealEnd {
        DealEnd::Played(DealScore {
            result,
            thirds: [0, 0],
            credits,
            valat: None,
            ultimo: None,
            multiplier: 1,
        })
    }

    fn credit(seat: Seat, points: u32) -> Credit {
        Credit {
            seat,
            cause: Cause::Game,
            points,
        }
    }

    #[test]
    fn rules_2_1_the_lot_decides_the_first_dealer_from_the_seed() {
        let mut seen = [false; 2];
        for seed in 0..40 {
            let s = Sitting::new(seed, Settings::default(), 3).unwrap();
            let again = Sitting::new(seed, Settings::default(), 3).unwrap();
            assert_eq!(s.dealer(), again.dealer());
            seen[s.dealer().index()] = true;
            assert!(matches!(
                s.events()[0],
                Event::SittingStart { touren: 3, .. }
            ));
        }
        assert_eq!(seen, [true, true]);
    }

    #[test]
    fn rules_0_the_ultimo_setting_is_1_or_2_and_nothing_else() {
        for bad in [0, 3, u32::MAX] {
            let settings = Settings {
                ultimo: bad,
                ..Settings::default()
            };
            let err = Sitting::new(1, settings, 3).unwrap_err();
            assert_eq!(err, SetupError::BadUltimo { ultimo: bad });
        }
        assert!(Sitting::new(
            1,
            Settings {
                ultimo: 2,
                ..Settings::default()
            },
            3
        )
        .is_ok());
    }

    #[test]
    fn setup_errors_are_keys_with_parameters_not_sentences() {
        let err = Sitting::new(1, Settings::default(), 4).unwrap_err();
        assert_eq!(
            serde_json::to_string(&err).unwrap(),
            r#"{"key":"bad-touren","touren":4}"#
        );
        let mut r = Sitting::new(1, Settings::default(), 3).unwrap().record();
        r.version = 9;
        assert_eq!(
            serde_json::to_string(&Sitting::replay(&r).unwrap_err()).unwrap(),
            r#"{"key":"record-version","found":9,"expected":1}"#
        );
        let mut r = Sitting::new(1, Settings::default(), 3).unwrap().record();
        let wrong = Sitting::new(1, Settings::default(), 3)
            .unwrap()
            .to_move()
            .unwrap()
            .other();
        r.moves.push(Move {
            seat: wrong,
            action: Action::Take,
        });
        let json = serde_json::to_value(Sitting::replay(&r).unwrap_err()).unwrap();
        assert_eq!(json["key"], "replay-refused");
        assert_eq!(json["index"], 0);
        assert_eq!(json["refusal"]["key"], "not-your-turn");
    }

    #[test]
    fn the_record_keeps_a_seed_beyond_2_pow_53_exactly_through_json() {
        // A JavaScript number cannot hold it; the record writes it as a string.
        let seed = (1u64 << 63) + 12345;
        let s = Sitting::new(seed, Settings::default(), 3).unwrap();
        let json = serde_json::to_string(&s.record()).unwrap();
        assert!(json.contains(&format!("\"seed\":\"{seed}\"")), "{json}");
        let back: Record = serde_json::from_str(&json).unwrap();
        assert_eq!(back.seed, seed);
        // a record with a plain number still reads
        let number = json.replace(&format!("\"{seed}\""), &seed.to_string());
        assert_eq!(serde_json::from_str::<Record>(&number).unwrap().seed, seed);
    }

    #[test]
    fn rules_15_2_the_touren_are_3_or_6() {
        assert!(Sitting::new(1, Settings::default(), 3).is_ok());
        assert!(Sitting::new(1, Settings::default(), 6).is_ok());
        assert!(Sitting::new(1, Settings::default(), 4).is_err());
    }

    #[test]
    fn rules_16_a_deal_opens_with_deal_start_and_each_players_own_cards() {
        let s = Sitting::new(7, Settings::default(), 3).unwrap();
        let ev = s.events();
        let Event::DealStart {
            deal_number,
            tour,
            dealer,
            forehand,
            multiplier,
        } = ev[1].clone()
        else {
            panic!("{:?}", ev[1]);
        };
        assert_eq!((deal_number, tour, multiplier), (1, 1, 1));
        assert_eq!(forehand, dealer.other());
        assert!(
            matches!(&ev[2], Event::Dealt { seat, cards } if *seat == forehand && cards.len() == 15)
        );
        assert!(
            matches!(&ev[3], Event::Dealt { seat, cards } if *seat == dealer && cards.len() == 15)
        );
    }

    #[test]
    fn rules_2_1_the_deal_alternates_every_deal() {
        let mut s = Sitting::with_decks(1, throw_in(), 3, vec![ok_deck(); 6]).unwrap();
        let mut dealers = vec![s.dealer()];
        for _ in 0..5 {
            all_pass(&mut s);
            dealers.push(s.dealer());
        }
        for w in dealers.windows(2) {
            assert_eq!(w[0].other(), w[1]);
        }
    }

    #[test]
    fn rules_2_6_a_void_deal_is_redealt_by_the_same_dealer_and_does_not_count() {
        let mut s = Sitting::with_decks(3, throw_in(), 3, vec![void_deck(), ok_deck()]).unwrap();
        let ev = s.events().to_vec();
        let voids: Vec<_> = ev
            .iter()
            .filter(|e| matches!(e, Event::VoidDeal { .. }))
            .collect();
        assert_eq!(voids.len(), 1);
        let starts: Vec<(u32, Seat)> = ev
            .iter()
            .filter_map(|e| match e {
                Event::DealStart {
                    deal_number,
                    dealer,
                    ..
                } => Some((*deal_number, *dealer)),
                _ => None,
            })
            .collect();
        assert_eq!(starts.len(), 2);
        assert_eq!(starts[0], starts[1], "same dealer, same deal number");
        // the deal that follows is still the first of the Tour
        all_pass(&mut s);
        assert_eq!(s.deals_counted(), 1);
    }

    #[test]
    fn rules_3_4_a_throw_in_counts_toward_the_tour_and_passes_the_deal() {
        let mut s = Sitting::with_decks(5, throw_in(), 3, vec![ok_deck(); 6]).unwrap();
        let d0 = s.dealer();
        all_pass(&mut s);
        assert_eq!(s.deals_counted(), 1);
        assert_eq!(s.dealer(), d0.other());
        all_pass(&mut s);
        assert_eq!(
            count(&s, |e| matches!(e, Event::TourEnd { tour: 1, .. })),
            1
        );
    }

    #[test]
    fn rules_15_3_three_touren_are_six_deals_then_the_extension_is_offered() {
        let mut s = Sitting::with_decks(5, throw_in(), 3, vec![ok_deck(); 6]).unwrap();
        for _ in 0..6 {
            assert!(!s.is_over());
            all_pass(&mut s);
        }
        assert_eq!(count(&s, |e| matches!(e, Event::TourEnd { .. })), 3);
        assert_eq!(count(&s, |e| matches!(e, Event::ExtensionOffered)), 1);
        assert_eq!(s.to_move(), Some(Seat::A));
        assert_eq!(
            s.legal_actions(),
            vec![
                Action::Extend { accept: true },
                Action::Extend { accept: false }
            ]
        );
    }

    #[test]
    fn rules_15_3_six_touren_are_twelve_deals() {
        let mut s = Sitting::with_decks(5, throw_in(), 6, vec![ok_deck(); 12]).unwrap();
        for _ in 0..11 {
            all_pass(&mut s);
        }
        assert_eq!(count(&s, |e| matches!(e, Event::ExtensionOffered)), 0);
        all_pass(&mut s);
        assert_eq!(count(&s, |e| matches!(e, Event::ExtensionOffered)), 1);
    }

    #[test]
    fn rules_15_6_extension_needs_both_and_one_refusal_ends_the_sitting() {
        for (a, b, extended) in [
            (true, true, true),
            (true, false, false),
            (false, true, false),
            (false, false, false),
        ] {
            let mut decks = vec![ok_deck(); 6];
            decks.extend(vec![ok_deck(); 6]);
            let mut s = Sitting::with_decks(5, throw_in(), 3, decks).unwrap();
            for _ in 0..6 {
                all_pass(&mut s);
            }
            s.apply(Seat::A, Action::Extend { accept: a }).unwrap();
            if a {
                s.apply(Seat::B, Action::Extend { accept: b }).unwrap();
            }
            assert_eq!(s.is_over(), !extended, "{a} {b}");
            if extended {
                assert!(s.events().contains(&Event::ExtensionDecided {
                    accepted: true,
                    touren_added: 3
                }));
                // the deal goes on, alternating
                assert!(s.deal().is_some());
                assert_eq!(count(&s, |e| matches!(e, Event::DealStart { .. })), 7);
            } else {
                assert!(s.events().contains(&Event::ExtensionDecided {
                    accepted: false,
                    touren_added: 0
                }));
                assert!(matches!(s.events().last(), Some(Event::SittingEnd { .. })));
            }
        }
    }

    #[test]
    fn rules_15_6_an_extension_adds_the_announced_number_of_touren() {
        let mut decks = vec![ok_deck(); 6];
        decks.extend(vec![ok_deck(); 6]);
        let mut s = Sitting::with_decks(5, throw_in(), 3, decks).unwrap();
        for _ in 0..6 {
            all_pass(&mut s);
        }
        s.apply(Seat::A, Action::Extend { accept: true }).unwrap();
        s.apply(Seat::B, Action::Extend { accept: true }).unwrap();
        for _ in 0..6 {
            all_pass(&mut s);
        }
        assert_eq!(count(&s, |e| matches!(e, Event::ExtensionOffered)), 2);
    }

    #[test]
    fn rules_15_5_the_conto_collects_credits_and_the_balance_is_the_difference() {
        let mut s = Sitting::with_decks(5, Settings::default(), 3, vec![ok_deck(); 6]).unwrap();
        s.settle_for_test(&score(
            GameResult::DeclarerWin,
            vec![credit(Seat::A, 3), credit(Seat::B, 1)],
        ));
        assert_eq!(s.contos(), [3, 1]);
        s.settle_for_test(&score(GameResult::SimpleWin, vec![credit(Seat::B, 2)]));
        assert_eq!(s.contos(), [3, 3]);
        assert_eq!(s.balance(), 0);
        let scored: Vec<&Event> = s
            .events()
            .iter()
            .filter(|e| matches!(e, Event::GameScored { .. }))
            .collect();
        assert_eq!(scored.len(), 2);
        assert!(matches!(
            scored[0],
            Event::GameScored {
                result: GameResult::DeclarerWin,
                contos: [3, 1],
                multiplier: 1,
                ..
            }
        ));
    }

    #[test]
    fn rules_15_7_and_15_8_settlement_is_two_kreuzer_a_point_from_the_smaller_conto() {
        let mut s = Sitting::with_decks(5, Settings::default(), 3, vec![ok_deck(); 6]).unwrap();
        for _ in 0..5 {
            s.settle_for_test(&score(GameResult::DeclarerWin, vec![credit(Seat::A, 3)]));
        }
        s.settle_for_test(&score(GameResult::DeclarerWin, vec![credit(Seat::B, 4)]));
        // A 15, B 4: balance 11, B pays A 22 kreuzer
        s.apply(Seat::A, Action::Extend { accept: false }).unwrap();
        let Some(Event::SittingEnd {
            contos,
            balance,
            payer,
            kreuzer,
        }) = s.events().last().cloned()
        else {
            panic!("sitting should have ended");
        };
        assert_eq!(
            (contos, balance, payer, kreuzer),
            ([15, 4], 11, Some(Seat::B), 22)
        );
    }

    #[test]
    fn rules_15_8_a_zero_balance_owes_nothing() {
        let mut s = Sitting::with_decks(5, Settings::default(), 3, vec![ok_deck(); 6]).unwrap();
        for _ in 0..6 {
            s.settle_for_test(&score(GameResult::Draw, vec![]));
        }
        s.apply(Seat::A, Action::Extend { accept: false }).unwrap();
        let Some(Event::SittingEnd { payer, kreuzer, .. }) = s.events().last().cloned() else {
            panic!()
        };
        assert_eq!((payer, kreuzer), (None, 0));
    }

    #[test]
    fn rules_14_1_a_draw_doubles_the_next_deal_and_it_does_not_stack() {
        let mut s = Sitting::with_decks(5, Settings::default(), 6, vec![ok_deck(); 12]).unwrap();
        s.settle_for_test(&score(GameResult::Draw, vec![]));
        assert!(s.events().contains(&Event::DoublingSet));
        assert_eq!(s.multiplier(), 2);
        let Some(Event::DoublingApplied { multiplier: 2 }) = s
            .events()
            .iter()
            .rev()
            .find(|e| matches!(e, Event::DoublingApplied { .. }))
            .cloned()
        else {
            panic!("doubling not applied to the next deal");
        };
        // a draw in the doubled deal sets the flag again: the next is x2, not x4
        s.settle_for_test(&score(GameResult::Draw, vec![]));
        assert_eq!(s.multiplier(), 2);
        // a played deal that is not a draw clears it
        s.settle_for_test(&score(GameResult::SimpleWin, vec![credit(Seat::A, 2)]));
        assert_eq!(s.multiplier(), 1);
    }

    #[test]
    fn rules_14_4_void_deals_and_throw_ins_do_not_clear_the_doubling() {
        let mut s = Sitting::with_decks(
            5,
            throw_in(),
            6,
            vec![ok_deck(), void_deck(), ok_deck(), ok_deck()],
        )
        .unwrap();
        s.settle_for_test(&score(GameResult::Draw, vec![]));
        // next deal is void: redealt, still doubled
        assert_eq!(s.multiplier(), 2);
        assert_eq!(count(&s, |e| matches!(e, Event::VoidDeal { .. })), 1);
        all_pass(&mut s); // thrown in
        assert_eq!(s.multiplier(), 2);
    }

    #[test]
    fn rules_14_5_a_pending_doubling_lapses_at_the_end_of_the_sitting() {
        let mut s = Sitting::with_decks(5, Settings::default(), 3, vec![ok_deck(); 6]).unwrap();
        for _ in 0..5 {
            s.settle_for_test(&score(GameResult::SimpleWin, vec![credit(Seat::A, 2)]));
        }
        s.settle_for_test(&score(GameResult::Draw, vec![]));
        s.apply(Seat::A, Action::Extend { accept: false }).unwrap();
        let ev = s.events();
        let n = ev.len();
        assert_eq!(ev[n - 2], Event::DoublingLapsed);
        assert!(matches!(ev[n - 1], Event::SittingEnd { .. }));
    }

    #[test]
    fn rules_14_5_a_pending_doubling_carries_through_an_extension() {
        let mut decks = vec![ok_deck(); 6];
        decks.extend(vec![ok_deck(); 3]);
        let mut s = Sitting::with_decks(5, Settings::default(), 3, decks).unwrap();
        for _ in 0..5 {
            s.settle_for_test(&score(GameResult::SimpleWin, vec![credit(Seat::A, 2)]));
        }
        s.settle_for_test(&score(GameResult::Draw, vec![]));
        s.apply(Seat::A, Action::Extend { accept: true }).unwrap();
        s.apply(Seat::B, Action::Extend { accept: true }).unwrap();
        assert_eq!(s.multiplier(), 2);
        assert!(!s.events().contains(&Event::DoublingLapsed));
    }

    #[test]
    fn rules_15_10_after_the_end_nothing_more_is_accepted() {
        let mut s = Sitting::with_decks(5, throw_in(), 3, vec![ok_deck(); 6]).unwrap();
        for _ in 0..6 {
            all_pass(&mut s);
        }
        s.apply(Seat::A, Action::Extend { accept: false }).unwrap();
        assert!(s.is_over());
        let err = s.apply(Seat::A, Action::Take).unwrap_err();
        assert_eq!(err.kind, RefusalKind::Finished);
        assert_eq!(s.to_move(), None);
        assert!(s.legal_actions().is_empty());
    }

    #[test]
    fn a_refused_action_changes_nothing() {
        let mut s = Sitting::new(9, Settings::default(), 3).unwrap();
        let before = s.events().len();
        let moves = s.record().moves.len();
        let wrong = s.to_move().unwrap().other();
        assert!(s.apply(wrong, Action::Take).is_err());
        assert_eq!(s.events().len(), before);
        assert_eq!(s.record().moves.len(), moves);
    }

    #[test]
    fn the_extension_cannot_be_answered_during_a_deal() {
        let mut s = Sitting::new(9, Settings::default(), 3).unwrap();
        let to = s.to_move().unwrap();
        let err = s.apply(to, Action::Extend { accept: true }).unwrap_err();
        assert_eq!(err.kind, RefusalKind::WrongPhase);
    }
}
