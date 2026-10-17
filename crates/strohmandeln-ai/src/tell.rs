//! "Their style": the after-deal brief's line on how the opponent has played
//! this sitting (`docs/STYLES.md` 5, `docs/TUTOR.md` 6). Facts only: stage 2
//! found no simple counter that pays (`docs/STYLES.md` 9), so the line states
//! what was seen and never what to do about it. It reads the public record
//! alone (the bids are public), and it never names a style, so under Random
//! it is the same line and reveals nothing.
//!
//! Only the bid is a tell that reads in a sitting: stage 0 at skill 3 puts
//! Normal's take-up rate at 18.9% of its bids (Bermann 6%, MS and Cautious
//! 3%, Bold 46%), while the other counted tells (an exposed queen played at
//! once, a contested suit opened) do not separate the styles from Normal in
//! the per-deal counts (`docs/STYLES.md` 9), so no line is made of them.

use crate::style::Style;
use kit_learner::{Bullet, Param};
use strohmandeln_core::dealrec::DealRecord;
use strohmandeln_core::types::{Action, Seat};

/// The lead of the line.
pub const LEAD: &str = "tutor.lead.style";

/// The keys the line says.
pub const KEYS: &[&str] = &[
    LEAD,
    "tutor.style.took",
    "tutor.style.took-once",
    "tutor.style.took-none",
];

/// Normal's take-up rate per bid at skill 3 (`docs/STYLES.md` 9, stage 0:
/// 18.9%, 20.4% and 18.8% on three seeds).
pub const NORMAL_RATE: f64 = 0.19;

/// The bids the opponent must have made before the line is said: fewer is
/// the luck of the hands.
pub const MIN_BIDS: u32 = 3;

/// How far from Normal's rate, in standard deviations of a count of that
/// many bids, a count must lie to be worth saying.
pub const MIN_Z: f64 = 1.0;

/// What the opponent did at its bids, over the deals of a sitting.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Tells {
    /// The bids it made (a take-up or a pass).
    pub bids: u32,
    /// Those it took up.
    pub takes: u32,
}

impl Tells {
    /// Adds the bids of `opp`, the opponent's seat, in `record`.
    pub fn add(&mut self, record: &DealRecord, opp: Seat) {
        for m in record.actions.iter().filter(|m| m.seat == opp) {
            match m.action {
                Action::Take => {
                    self.bids += 1;
                    self.takes += 1;
                }
                Action::Pass => self.bids += 1,
                _ => {}
            }
        }
    }

    /// How many standard deviations the count of take-ups lies above what
    /// Normal's rate would give over these bids.
    pub fn z(&self) -> f64 {
        let n = f64::from(self.bids);
        (f64::from(self.takes) - n * NORMAL_RATE) / (n * NORMAL_RATE * (1.0 - NORMAL_RATE)).sqrt()
    }
}

/// The line for an opponent of `style` that has played `tells` so far, or
/// `None`: Normal has no style to speak of, and a count near Normal's rate
/// or over too few bids is not worth a line. At most one line.
pub fn bullet(style: Style, tells: &Tells) -> Option<Bullet> {
    if style == Style::Normal || tells.bids < MIN_BIDS || tells.z().abs() < MIN_Z {
        return None;
    }
    let key = match tells.takes {
        0 => "tutor.style.took-none",
        1 => "tutor.style.took-once",
        _ => "tutor.style.took",
    };
    let mut b = Bullet::new(LEAD, key).with("of", Param::Int(i64::from(tells.bids)));
    if tells.takes >= 2 {
        b = b.with("n", Param::Int(i64::from(tells.takes)));
    }
    Some(b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use strohmandeln_core::cards::Card;
    use strohmandeln_core::session::Move;
    use strohmandeln_core::types::Settings;

    fn record(moves: &[(Seat, Action)]) -> DealRecord {
        DealRecord {
            deck: Vec::new(),
            dealer: Seat::A,
            multiplier: 1,
            settings: Settings::default(),
            actions: moves
                .iter()
                .map(|(seat, action)| Move {
                    seat: *seat,
                    action: action.clone(),
                })
                .collect(),
        }
    }

    fn tells(bids: u32, takes: u32) -> Tells {
        Tells { bids, takes }
    }

    fn n(b: &Bullet, name: &str) -> i64 {
        match b.params.iter().find(|(k, _)| k == name).map(|(_, v)| v) {
            Some(Param::Int(n)) => *n,
            other => panic!("{name}: {other:?}"),
        }
    }

    #[test]
    fn only_the_opponents_bids_are_counted() {
        let mut t = Tells::default();
        let play = Action::Play {
            card: Card::PAGAT,
            announce: vec![],
        };
        t.add(
            &record(&[
                (Seat::B, Action::Take),
                (Seat::A, Action::Pass),
                (Seat::A, play),
            ]),
            Seat::A,
        );
        assert_eq!(t, tells(1, 0));
        t.add(&record(&[(Seat::A, Action::Take)]), Seat::A);
        assert_eq!(t, tells(2, 1));
    }

    #[test]
    fn a_bold_sitting_of_many_take_ups_says_how_many() {
        let b = bullet(Style::Bold, &tells(6, 4)).expect("a line");
        assert_eq!((b.lead.as_str(), b.key.as_str()), (LEAD, "tutor.style.took"));
        assert_eq!((n(&b, "n"), n(&b, "of")), (4, 6));
    }

    #[test]
    fn a_cautious_sitting_of_none_says_so() {
        let b = bullet(Style::Cautious, &tells(5, 0)).expect("a line");
        assert_eq!(b.key, "tutor.style.took-none");
        assert_eq!(n(&b, "of"), 5);
        assert!(b.params.iter().all(|(k, _)| k != "n"));
        // a count of one stands out only over many bids
        let once = bullet(Style::Ms, &tells(14, 1)).expect("a line");
        assert_eq!(once.key, "tutor.style.took-once");
    }

    #[test]
    fn normal_says_nothing_and_neither_does_a_count_near_its_rate() {
        assert_eq!(bullet(Style::Normal, &tells(6, 5)), None);
        assert_eq!(bullet(Style::Bold, &tells(6, 1)), None, "Normal's own rate");
        assert_eq!(bullet(Style::Bold, &tells(2, 2)), None, "too few bids");
        assert_eq!(bullet(Style::Bold, &tells(0, 0)), None);
    }

    #[test]
    fn random_has_a_line_that_names_no_style() {
        let b = bullet(Style::Random, &tells(6, 4)).expect("a line");
        assert_eq!(b, bullet(Style::Bold, &tells(6, 4)).unwrap(), "the same line");
        assert!(b.params.iter().all(|(_, v)| matches!(v, Param::Int(_))));
        for k in KEYS {
            for s in Style::CANDIDATES {
                assert!(!k.contains(s.id()), "{k}");
            }
        }
    }

    #[test]
    fn every_line_is_short_in_both_languages_and_its_slots_are_supplied() {
        let table: serde_json::Value =
            serde_json::from_str(include_str!("../../../web3d/strings.json")).unwrap();
        for (bullet, slots) in [
            (bullet(Style::Bold, &tells(6, 4)).unwrap(), 2),
            (bullet(Style::Bold, &tells(5, 0)).unwrap(), 1),
            (bullet(Style::Bold, &tells(14, 1)).unwrap(), 1),
        ] {
            assert_eq!(bullet.params.len(), slots);
            for lang in ["en", "de"] {
                let text = table[&bullet.key][lang].as_str().expect("a string");
                assert!(text.split_whitespace().count() <= 14, "{text}");
                assert!(table[LEAD][lang].is_string());
                for (name, _) in &bullet.params {
                    assert!(text.contains(&format!("{{{name}}}")), "{text}");
                }
                assert_eq!(text.matches('{').count(), slots, "{text}");
            }
        }
    }
}
