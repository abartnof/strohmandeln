//! Why a card: the concrete consideration behind a hint, as a message key and
//! parameters the table can check against what it shows (`docs/MESSAGES.md`,
//! "Hint reasons"). It explains a choice already made; it never makes one.

use crate::cardset::leader_wins;
use crate::know::Knowledge;
use crate::rung2::{reserved_packet, spend, Weights, STRONG_TAROCKS};
use strohmandeln_core::cards::Card;

/// A parameter of a reason: a card (sent as its code) or a whole number.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Param {
    Card(Card),
    Int(i64),
}

/// A reason: its key, parameters, and the rule it follows, if one is worth
/// naming (a key of the rule table in `docs/MESSAGES.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reason {
    pub key: &'static str,
    pub params: Vec<(&'static str, Param)>,
    pub rule: Option<&'static str>,
}

/// What the sampled deals say of a lead: in how many of `worlds` the
/// opponent, answering as the rollout policy does, leaves the trick to the
/// card (`wins`). `worlds` is 0 when nothing was sampled.
pub struct Sampled<'a> {
    pub worlds: usize,
    pub wins: &'a dyn Fn(Card) -> usize,
}

/// The maxim that weighed most for the card, and whether the maxims rank the
/// card first (otherwise the search or the draw chose it against them).
pub struct Maxim {
    pub reason: &'static str,
    pub is_top: bool,
}

fn reason(
    key: &'static str,
    params: &[(&'static str, Param)],
    rule: Option<&'static str>,
) -> Reason {
    Reason {
        key,
        params: params.to_vec(),
        rule,
    }
}

fn int(n: u32) -> Param {
    Param::Int(i64::from(n))
}

/// Cost order of two cards that can both win or both lose: the lesser first.
fn dearness(c: Card) -> (bool, u8) {
    (c.is_tarock(), c.strength())
}

pub fn explain_play(k: &Knowledge, card: Card, maxim: &Maxim, sampled: &Sampled) -> Reason {
    let cd = Param::Card(card);
    if k.legal.len() == 1 {
        return reason("only-card", &[("card", cd)], None);
    }
    // The Pagat is the lowest tarock a strong hand could spend: keep it.
    let lowest_tarock = k
        .legal
        .iter()
        .filter(|c| c.is_tarock())
        .min_by_key(|c| c.strength());
    // On a lead the Pagat is a rival only for another tarock: a king led with
    // a strong hand is not "instead of the Pagat".
    if card != Card::PAGAT
        && (k.lead.is_some() || card.is_tarock())
        && lowest_tarock == Some(&Card::PAGAT)
        && k.my_tarocks() >= STRONG_TAROCKS
    {
        return reason(
            "hold-pagat",
            &[("card", cd), ("tarocks", int(k.my_tarocks() as u32))],
            Some("pagat-last"),
        );
    }
    if let Some(i) = reserved_packet(k) {
        let kept = k.covers[i];
        if kept.is_some_and(|kept| kept != card && k.legal.contains(&kept)) {
            return reason("keep-packet", &[("card", cd)], Some("keep-packet"));
        }
    }
    match k.lead {
        Some(led) => follow(k, led, card),
        None => lead(k, card, maxim, sampled),
    }
}

fn follow(k: &Knowledge, led: Card, card: Card) -> Reason {
    let cd = Param::Card(card);
    let ld = Param::Card(led);
    let beats = |c: Card| !leader_wins(led, c);
    let mut winners: Vec<Card> = k.legal.iter().copied().filter(|&c| beats(c)).collect();
    winners.sort_by_key(|&c| dearness(c));
    if beats(card) {
        let trick = int(led.thirds() + card.thirds());
        let cheaper = winners.iter().find(|&&w| dearness(w) < dearness(card));
        let dearer = winners
            .iter()
            .rev()
            .find(|&&w| dearness(w) > dearness(card));
        return if let Some(&alt) = cheaper {
            reason(
                "win-over-cheaper",
                &[
                    ("card", cd),
                    ("led", ld),
                    ("alt", Param::Card(alt)),
                    ("trick_thirds", trick),
                ],
                None,
            )
        } else if let Some(&alt) = dearer {
            reason(
                "win-cheapest",
                &[
                    ("card", cd),
                    ("led", ld),
                    ("alt", Param::Card(alt)),
                    ("trick_thirds", trick),
                ],
                None,
            )
        } else {
            reason(
                "win-sole",
                &[("card", cd), ("led", ld), ("trick_thirds", trick)],
                None,
            )
        };
    }
    let given = int(card.thirds());
    if let Some(&alt) = winners.first() {
        // "Poor" is the maxim's own measure: the lead's points do not repay
        // the burden of leading next and the cost of spending the winner.
        if f64::from(led.thirds()) > Weights::default().burden + spend(alt) {
            return reason(
                "decline-rich",
                &[
                    ("card", cd),
                    ("led", ld),
                    ("alt", Param::Card(alt)),
                    ("trick_thirds", int(led.thirds() + alt.thirds())),
                    ("given_thirds", given),
                ],
                None,
            );
        }
        return reason(
            "decline-poor",
            &[
                ("card", cd),
                ("led", ld),
                ("alt", Param::Card(alt)),
                ("trick_thirds", int(led.thirds() + alt.thirds())),
                ("given_thirds", given),
            ],
            Some("finesse"),
        );
    }
    if k.is_cover(card) {
        return reason(
            "lose-packet",
            &[("card", cd), ("led", ld), ("given_thirds", given)],
            Some("packet-off"),
        );
    }
    let cheapest = k
        .legal
        .iter()
        .min_by_key(|&&c| (c.thirds(), dearness(c)))
        .expect("a legal card");
    if card.thirds() == cheapest.thirds() {
        reason(
            "lose-cheapest",
            &[("card", cd), ("led", ld), ("given_thirds", given)],
            Some("cheapest"),
        )
    } else {
        let alt = *cheapest;
        reason(
            "lose-other",
            &[
                ("card", cd),
                ("led", ld),
                ("alt", Param::Card(alt)),
                ("given_thirds", given),
            ],
            None,
        )
    }
}

fn lead(k: &Knowledge, card: Card, maxim: &Maxim, sampled: &Sampled) -> Reason {
    let cd = Param::Card(card);
    let out = int(k.unseen_tarocks() as u32);
    if card.is_tarock() && k.opp_void[crate::cardset::Kind::Trump.index()] {
        return reason("lead-trump-void", &[("card", cd), ("out", out)], None);
    }
    if maxim.is_top {
        match maxim.reason {
            "cash-king" => return reason("lead-king", &[("card", cd), ("out", out)], None),
            "draw-trumps" | "lead-solid-run" => {
                return reason(
                    "lead-draw",
                    &[
                        ("card", cd),
                        ("mine", int(k.my_tarocks() as u32)),
                        ("out", out),
                    ],
                    Some("draw-trumps"),
                )
            }
            "play-exposed-figure" => {
                return reason("lead-exposed", &[("card", cd)], Some("exposed"))
            }
            "lead-where-he-shows-none" => {
                return reason("lead-his-none", &[("card", cd)], Some("lead-none"))
            }
            _ => {}
        }
    }
    if sampled.worlds > 0 {
        let wins = (sampled.wins)(card);
        let base = [
            ("card", cd),
            ("wins", int(wins as u32)),
            ("worlds", int(sampled.worlds as u32)),
        ];
        return if wins * 10 <= sampled.worlds * 2 {
            let mut p = base.to_vec();
            p.push(("cost_thirds", int(card.thirds())));
            reason("lead-cheap", &p, None)
        } else {
            reason("lead-rate", &base, None)
        };
    }
    reason(
        "lead-cost",
        &[("card", cd), ("cost_thirds", int(card.thirds()))],
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{c, know};

    const NO: [&str; 3] = ["", "", ""];

    fn plain() -> Maxim {
        Maxim {
            reason: "lead-low",
            is_top: false,
        }
    }
    fn why(k: &Knowledge, card: &str) -> Reason {
        let none = |_: Card| 0;
        explain_play(
            k,
            c(card),
            &plain(),
            &Sampled {
                worlds: 0,
                wins: &none,
            },
        )
    }
    fn p(r: &Reason, name: &str) -> Param {
        r.params
            .iter()
            .find(|(n, _)| *n == name)
            .unwrap_or_else(|| panic!("{name} in {r:?}"))
            .1
    }

    #[test]
    fn the_only_legal_card_says_so() {
        let k = know(&["4h", "9c"], NO, NO, &[], Some("2h"));
        let r = why(&k, "4h");
        assert_eq!(r.key, "only-card");
        assert_eq!(p(&r, "card"), Param::Card(c("4h")));
    }

    #[test]
    fn the_cheapest_winner_names_what_it_beats_the_trick_and_the_dearer_card_it_saves() {
        // Jack of hearts led (4 thirds); Q and K both win. The queen is the
        // cheapest winner; the trick is Jack 4 + Queen 10 = 14 thirds.
        let k = know(&["Kh", "Qh", "9c"], NO, NO, &[], Some("Jh"));
        let r = why(&k, "Qh");
        assert_eq!(r.key, "win-cheapest");
        assert_eq!(p(&r, "led"), Param::Card(c("Jh")));
        assert_eq!(p(&r, "alt"), Param::Card(c("Kh")));
        assert_eq!(p(&r, "trick_thirds"), Param::Int(14));
        // The sources say not to spend a king on a poor trick, not that the
        // cheapest winner is a rule: no rule is named.
        assert_eq!(r.rule, None);
    }

    #[test]
    fn a_tarock_that_saves_a_higher_one_names_it() {
        // XIX and XXI both beat the XV; XIX saves XXI.
        let k = know(&["T19", "T21", "9c"], NO, NO, &[], Some("T15"));
        let r = why(&k, "T19");
        assert_eq!(r.key, "win-cheapest");
        assert_eq!(p(&r, "alt"), Param::Card(c("T21")));
    }

    #[test]
    fn a_dearer_winner_chosen_names_the_cheaper_one_it_passed_over() {
        let k = know(&["Kh", "Qh", "9c"], NO, NO, &[], Some("Jh"));
        let r = why(&k, "Kh");
        assert_eq!(r.key, "win-over-cheaper");
        assert_eq!(p(&r, "alt"), Param::Card(c("Qh")));
    }

    #[test]
    fn the_sole_winner_says_it_is_the_only_one() {
        let k = know(&["Kh", "4h", "9c"], NO, NO, &[], Some("Qh"));
        let r = why(&k, "Kh");
        assert_eq!(r.key, "win-sole");
        assert_eq!(p(&r, "trick_thirds"), Param::Int(23));
    }

    #[test]
    fn a_rich_trick_given_up_is_not_called_poor() {
        // The queen led is worth 10 thirds: not a poor trick, so the
        // "poor trick" reason must not be given for declining the king.
        let k = know(&["Kh", "4h", "9c"], NO, NO, &[], Some("Qh"));
        let r = why(&k, "4h");
        assert_eq!(r.key, "decline-rich");
        assert_eq!(p(&r, "alt"), Param::Card(c("Kh")));
        assert_eq!(p(&r, "trick_thirds"), Param::Int(23));
    }

    #[test]
    fn leading_a_king_is_not_instead_of_the_pagat() {
        let hand = ["T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "Kh"];
        let k = know(&hand, NO, NO, &[], None);
        assert_ne!(why(&k, "Kh").key, "hold-pagat");
        assert_eq!(why(&k, "T2").key, "hold-pagat");
    }

    #[test]
    fn a_poor_trick_is_not_bought_with_an_honour() {
        let k = know(&["Kh", "4h", "9c"], NO, NO, &[], Some("2h"));
        let r = why(&k, "4h");
        assert_eq!(r.key, "decline-poor");
        assert_eq!(p(&r, "alt"), Param::Card(c("Kh")));
        assert_eq!(p(&r, "trick_thirds"), Param::Int(14));
        assert_eq!(p(&r, "given_thirds"), Param::Int(1));
        assert_eq!(r.rule, Some("finesse"));
    }

    #[test]
    fn when_nothing_beats_the_lead_the_cheapest_card_goes() {
        let k = know(&["4h", "Qh", "9c"], NO, NO, &[], Some("Kh"));
        let r = why(&k, "4h");
        assert_eq!(r.key, "lose-cheapest");
        assert_eq!(p(&r, "led"), Param::Card(c("Kh")));
        assert_eq!(r.rule, Some("cheapest"));
        let r = why(&k, "Qh");
        assert_eq!(r.key, "lose-other");
        assert_eq!(p(&r, "alt"), Param::Card(c("4h")));
    }

    #[test]
    fn a_face_up_packet_card_given_up_says_it_turns_up_the_card_beneath() {
        let k = know(&["4d", "3d"], ["2d", "", ""], NO, &[], Some("Ks"));
        let r = why(&k, "2d");
        assert_eq!(r.key, "lose-packet");
        assert_eq!(r.rule, Some("packet-off"));
    }

    #[test]
    fn the_pagat_is_held_when_strong_in_tarocks() {
        let hand = ["T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "9c"];
        let k = know(&hand, NO, NO, &[], Some("Kh"));
        let r = why(&k, "T2");
        assert_eq!(r.key, "hold-pagat");
        assert_eq!(p(&r, "tarocks"), Param::Int(8));
        assert_eq!(r.rule, Some("pagat-last"));
    }

    #[test]
    fn a_packet_is_kept_when_his_pagat_ultimo_threatens() {
        let known = ["T3", "T4", "T5", "T6", "T7", "T8"];
        let k = know(&["4d"], ["2h", "3h", "4h"], NO, &known, Some("9s"));
        let r = why(&k, "3h");
        assert_eq!(r.key, "keep-packet");
        assert_eq!(r.rule, Some("keep-packet"));
    }

    #[test]
    fn a_lead_king_says_only_a_tarock_beats_it_and_how_many_are_unseen() {
        let k = know(&["Kh", "7c", "9c"], NO, NO, &[], None);
        let maxim = Maxim {
            reason: "cash-king",
            is_top: true,
        };
        let none = |_: Card| 0;
        let r = explain_play(
            &k,
            c("Kh"),
            &maxim,
            &Sampled {
                worlds: 0,
                wins: &none,
            },
        );
        assert_eq!(r.key, "lead-king");
        assert_eq!(p(&r, "out"), Param::Int(22));
    }

    #[test]
    fn a_tarock_lead_from_strength_counts_mine_against_the_unseen() {
        let hand = [
            "T8", "T9", "T10", "T12", "T13", "T14", "T15", "T16", "T17", "9c",
        ];
        let k = know(&hand, NO, NO, &[], None);
        let maxim = Maxim {
            reason: "draw-trumps",
            is_top: true,
        };
        let none = |_: Card| 0;
        let r = explain_play(
            &k,
            c("T9"),
            &maxim,
            &Sampled {
                worlds: 0,
                wins: &none,
            },
        );
        assert_eq!(r.key, "lead-draw");
        assert_eq!(p(&r, "mine"), Param::Int(9));
        assert_eq!(p(&r, "out"), Param::Int(22 - 9));
        assert_eq!(r.rule, Some("draw-trumps"));
    }

    #[test]
    fn a_sampled_lead_reports_how_often_it_wins_the_trick() {
        let k = know(&["Qh", "7c", "9c"], NO, NO, &[], None);
        let wins = |card: Card| if card == c("Qh") { 7 } else { 1 };
        let sampled = Sampled {
            worlds: 10,
            wins: &wins,
        };
        let r = explain_play(&k, c("Qh"), &plain(), &sampled);
        assert_eq!(r.key, "lead-rate");
        assert_eq!(p(&r, "wins"), Param::Int(7));
        assert_eq!(p(&r, "worlds"), Param::Int(10));
        let r = explain_play(&k, c("7c"), &plain(), &sampled);
        assert_eq!(r.key, "lead-cheap");
        assert_eq!(p(&r, "wins"), Param::Int(1));
        assert_eq!(p(&r, "cost_thirds"), Param::Int(1));
    }

    #[test]
    fn leading_a_tarock_he_cannot_answer_says_he_showed_none() {
        let mut k = know(&["T9", "7c", "9c"], NO, NO, &[], None);
        k.opp_void[crate::cardset::Kind::Trump.index()] = true;
        let r = why(&k, "T9");
        assert_eq!(r.key, "lead-trump-void");
    }

    #[test]
    fn without_a_sample_a_lead_names_what_it_costs() {
        let k = know(&["Qh", "7c", "9c"], NO, NO, &[], None);
        let r = why(&k, "7c");
        assert_eq!(r.key, "lead-cost");
        assert_eq!(p(&r, "cost_thirds"), Param::Int(1));
    }
}
