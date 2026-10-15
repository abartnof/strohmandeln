//! The card tracker: what has gone, what is still out, from one seat's view
//! (`docs/TABLE3D.md` 6.6). A learner's slate of the 54 cards, the counts a
//! player who keeps count would keep, and which of my top cards are masters.
//!
//! [`tracker`] takes only a [`View`] and rebuilds everything through
//! [`Knowledge::from_view`], so it cannot name a hidden card: an unseen card is
//! `Unseen` and nothing more (a test plays twins of a deal that differ in
//! the hidden cards and finds the same output).

use crate::cardset::{CardSet, Kind};
use crate::know::Knowledge;
use strohmandeln_core::cards::Card;
use strohmandeln_core::types::Seat;
use strohmandeln_core::view::View;

/// Where a card is, from my seat.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    /// In my hand.
    Hand,
    /// My packet's face-up cover.
    Cover,
    /// The opponent's packet cover, face up.
    OppCover,
    /// In the opponent's hand, and shown: pulled and not yet played, or part
    /// of an announced honour.
    OppHand,
    /// Played, by `by`, to trick `trick` (1 to 27; the open trick counts).
    Played { by: Seat, trick: u32 },
    /// Not placed: the opponent's private hand or a face-down packet card.
    Unseen,
}

/// What the tracker says of one kind of card: a suit, or the tarocks.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub kind: Kind,
    /// My hand and covers of this kind, high to low.
    pub mine: Vec<Card>,
    /// Cards of this kind that are not mine and not played, high to low:
    /// unseen, in his hand or on his covers.
    pub out: Vec<Card>,
    /// How many of the `out` cards are strictly unseen.
    pub unseen: usize,
    /// He has failed to follow this kind and no card of it has been shown in
    /// his hand or on his covers since. A packet can refill a hand, so the
    /// flag lifts when one does.
    pub void: bool,
    /// My highest card of this kind.
    pub best: Option<Card>,
    /// The highest card of this kind that is out.
    pub top_out: Option<Card>,
    /// I hold a card of this kind and nothing out is higher: a master.
    pub master: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tracker {
    pub me: Seat,
    /// The trick under way, 1 to 27.
    pub trick: u32,
    /// All 54 cards in canonical order (tarocks I to XXI and Sküs, then the
    /// suits), each with where it is.
    pub cards: Vec<(Card, Place)>,
    /// The four suits in `Suit::ALL` order, then the tarocks.
    pub groups: Vec<Group>,
    /// Cards out (not mine, not played), and their count in thirds of a Point
    /// (RULES 8.1: 70 Points are 210 thirds).
    pub out_count: usize,
    pub out_thirds: u32,
    /// Of those, the strictly unseen.
    pub unseen_count: usize,
    pub unseen_thirds: u32,
}

/// `None` when the view has no deal in progress.
pub fn tracker(view: &View) -> Option<Tracker> {
    let k = Knowledge::from_view(view)?;
    let d = view.deal.as_ref()?;
    let mine = k.playable();
    let opp_covers = CardSet::of(k.opp_covers.iter().flatten().copied());
    // Where each played card went, and in which trick.
    let mut played = [None; Card::COUNT];
    for (n, t) in d.tricks.iter().enumerate() {
        let trick = n as u32 + 1;
        played[t.cards[0].index()] = Some(Place::Played {
            by: t.leader,
            trick,
        });
        played[t.cards[1].index()] = Some(Place::Played {
            by: t.leader.other(),
            trick,
        });
    }
    let open = d.tricks.len() as u32 + 1;
    if let Some(l) = &d.lead {
        played[l.card.index()] = Some(Place::Played {
            by: l.seat,
            trick: open,
        });
    }
    let cards: Vec<(Card, Place)> = Card::all()
        .map(|c| {
            let place = if let Some(p) = played[c.index()] {
                p
            } else if k.hand.contains(c) {
                Place::Hand
            } else if k.is_cover(c) {
                Place::Cover
            } else if opp_covers.contains(c) {
                Place::OppCover
            } else if k.opp_known.contains(c) {
                Place::OppHand
            } else {
                Place::Unseen
            };
            (c, place)
        })
        .collect();
    let is_out = |p: Place| matches!(p, Place::OppCover | Place::OppHand | Place::Unseen);
    let high_to_low = |mut v: Vec<Card>| {
        v.sort_by_key(|c| std::cmp::Reverse(c.strength()));
        v
    };
    // He has shown a kind as void; a card of it shown in his hand or on his
    // covers since lifts the flag.
    let shown = k.opp_known.union(opp_covers);
    let groups: Vec<Group> = Kind::ALL
        .into_iter()
        .map(|kind| {
            let of_kind = kind.cards();
            let mine = high_to_low(mine.intersect(of_kind).to_vec());
            let out_places: Vec<(Card, Place)> = cards
                .iter()
                .copied()
                .filter(|(c, p)| of_kind.contains(*c) && is_out(*p))
                .collect();
            let unseen = out_places
                .iter()
                .filter(|(_, p)| *p == Place::Unseen)
                .count();
            let out = high_to_low(out_places.into_iter().map(|(c, _)| c).collect());
            let best = mine.first().copied();
            let top_out = out.first().copied();
            let master = best.is_some_and(|b| top_out.is_none_or(|t| t.strength() < b.strength()));
            Group {
                kind,
                mine,
                out,
                unseen,
                void: k.opp_void[kind.index()] && shown.intersect(of_kind).is_empty(),
                best,
                top_out,
                master,
            }
        })
        .collect();
    let thirds = |f: &dyn Fn(Place) -> bool| -> (usize, u32) {
        cards
            .iter()
            .filter(|(_, p)| f(*p))
            .fold((0, 0), |(n, t), (c, _)| (n + 1, t + c.thirds()))
    };
    let (out_count, out_thirds) = thirds(&is_out);
    let (unseen_count, unseen_thirds) = thirds(&|p| p == Place::Unseen);
    Some(Tracker {
        me: k.me,
        trick: open,
        cards,
        groups,
        out_count,
        out_thirds,
        unseen_count,
        unseen_thirds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::{blind_check_values, walk};
    use strohmandeln_core::session::Sitting;
    use strohmandeln_core::view::view;

    /// Every card I cannot play and that is not yet played, from full
    /// knowledge of the deal: the opponent's hand, and every packet card that
    /// is not my cover.
    fn truth_out(s: &Sitting, seat: Seat) -> CardSet {
        let deal = s.deal().unwrap();
        let mut out = CardSet::of(deal.hand_cards(seat.other()));
        for owner in Seat::BOTH {
            for p in 1..=3u8 {
                let skip = usize::from(owner == seat && deal.cover(seat, p).is_some());
                for c in deal.packet_cards(owner, p).into_iter().skip(skip) {
                    out.insert(c);
                }
            }
        }
        out
    }

    #[test]
    fn no_deal_no_tracker() {
        walk(1, |s, seat| {
            let v = view(s, seat);
            assert_eq!(tracker(&v).is_some(), v.deal.is_some());
        });
    }

    #[test]
    fn the_54_cards_each_have_one_place_matching_the_truth() {
        let mut checked = 0;
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                let t = tracker(&v).expect("a deal");
                assert_eq!(t.cards.len(), 54);
                for (i, (card, _)) in t.cards.iter().enumerate() {
                    assert_eq!(card.index(), i, "canonical order");
                }
                let of = |f: &dyn Fn(Place) -> bool| -> CardSet {
                    CardSet::of(t.cards.iter().filter(|(_, p)| f(*p)).map(|(c, _)| *c))
                };
                assert_eq!(of(&|p| p == Place::Hand), k.hand);
                assert_eq!(of(&|p| p == Place::Unseen), k.unseen);
                assert_eq!(of(&|p| p == Place::OppHand), k.opp_known);
                assert_eq!(
                    of(&|p| matches!(p, Place::Played { .. })),
                    k.played,
                    "seed {seed}"
                );
                assert_eq!(
                    of(&|p| p == Place::Cover),
                    CardSet::of(k.covers.iter().flatten().copied())
                );
                assert_eq!(
                    of(&|p| p == Place::OppCover),
                    CardSet::of(k.opp_covers.iter().flatten().copied())
                );
                checked += 1;
            });
        }
        assert!(checked > 500);
    }

    #[test]
    fn a_played_card_names_who_played_it_and_in_which_trick() {
        let mut plays = 0;
        for seed in 0..6 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(d) = v.deal.as_ref() else { return };
                let t = tracker(&v).unwrap();
                let place = |c: Card| t.cards[c.index()].1;
                for (n, tr) in d.tricks.iter().enumerate() {
                    let trick = n as u32 + 1;
                    let by = [tr.leader, tr.leader.other()];
                    for i in 0..2 {
                        assert_eq!(place(tr.cards[i]), Place::Played { by: by[i], trick });
                        plays += 1;
                    }
                }
                if let Some(l) = &d.lead {
                    let trick = d.tricks.len() as u32 + 1;
                    assert_eq!(place(l.card), Place::Played { by: l.seat, trick });
                }
                assert_eq!(t.trick, d.tricks.len() as u32 + 1);
            });
        }
        assert!(plays > 500);
    }

    #[test]
    fn the_cards_out_are_everything_i_do_not_hold_and_nobody_has_played() {
        for seed in 0..8 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                let t = tracker(&v).unwrap();
                let mut out = CardSet::EMPTY;
                for g in &t.groups {
                    out = out.union(CardSet::of(g.out.iter().copied()));
                    assert_eq!(
                        g.unseen,
                        g.out.iter().filter(|c| k.unseen.contains(**c)).count()
                    );
                }
                assert_eq!(out.len(), t.out_count);
                assert_eq!(t.unseen_count, k.unseen.len());
                // Full knowledge: his hand and every packet card but my covers,
                // less what has been played.
                let want = truth_out(s, seat).minus(k.played);
                assert_eq!(out, want, "seed {seed}");
            });
        }
    }

    #[test]
    fn points_are_the_game_values_in_thirds_and_the_pack_is_70_points() {
        assert_eq!(Card::all().map(|c| c.thirds()).sum::<u32>(), 210);
        let mut looked = 0;
        walk(3, |s, seat| {
            let Some(t) = tracker(&view(s, seat)) else {
                return;
            };
            let thirds: u32 = t
                .groups
                .iter()
                .flat_map(|g| g.out.iter())
                .map(|c| c.thirds())
                .sum();
            assert_eq!(t.out_thirds, thirds);
            let unseen: u32 = t
                .cards
                .iter()
                .filter(|(_, p)| *p == Place::Unseen)
                .map(|(c, _)| c.thirds())
                .sum();
            assert_eq!(t.unseen_thirds, unseen);
            assert!(t.out_thirds <= 210 && t.unseen_thirds <= t.out_thirds);
            looked += 1;
        });
        assert!(looked > 100);
    }

    #[test]
    fn a_master_is_my_best_card_with_nothing_higher_out() {
        let (mut masters, mut not_masters) = (0, 0);
        for seed in 0..10 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                let t = tracker(&v).unwrap();
                let out = truth_out(s, seat).minus(k.played);
                for g in &t.groups {
                    let mine = k.playable().intersect(g.kind.cards());
                    assert_eq!(CardSet::of(g.mine.iter().copied()), mine);
                    let best = mine.iter().max_by_key(|c| c.strength());
                    assert_eq!(g.best, best);
                    let higher = best.is_some_and(|b| {
                        out.intersect(g.kind.cards())
                            .iter()
                            .any(|c| c.strength() > b.strength())
                    });
                    assert_eq!(g.master, best.is_some() && !higher, "seed {seed}");
                    if g.master {
                        masters += 1;
                    } else if best.is_some() {
                        not_masters += 1;
                    }
                }
            });
        }
        assert!(masters > 50 && not_masters > 50, "{masters} {not_masters}");
    }

    #[test]
    fn a_void_is_shown_only_when_the_opponent_really_has_none() {
        let mut voids = 0;
        for seed in 0..40 {
            walk(seed, |s, seat| {
                let Some(t) = tracker(&view(s, seat)) else {
                    return;
                };
                let deal = s.deal().unwrap();
                let opp = seat.other();
                let mut his = CardSet::of(deal.hand_cards(opp));
                for p in 1..=3u8 {
                    if let Some(c) = deal.cover(opp, p) {
                        his.insert(c);
                    }
                }
                for g in &t.groups {
                    if g.void {
                        voids += 1;
                        assert!(
                            his.intersect(g.kind.cards()).is_empty(),
                            "seed {seed}: void {:?} but he holds {:?}",
                            g.kind,
                            his.intersect(g.kind.cards()).to_vec()
                        );
                    }
                }
            });
        }
        assert!(voids > 100, "voids are shown at all ({voids})");
    }

    #[test]
    fn a_void_lifts_when_a_card_of_the_kind_shows_in_his_hand_or_covers() {
        let mut lifted = 0;
        for seed in 0..60 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let Some(k) = Knowledge::from_view(&v) else {
                    return;
                };
                let t = tracker(&v).unwrap();
                for g in &t.groups {
                    if k.opp_void[g.kind.index()] && !g.void {
                        let shown = k
                            .opp_known
                            .union(CardSet::of(k.opp_covers.iter().flatten().copied()));
                        assert!(!shown.intersect(g.kind.cards()).is_empty());
                        lifted += 1;
                    }
                }
            });
        }
        assert!(lifted > 0, "a packet refills a hand now and then");
    }

    #[test]
    fn groups_run_the_four_suits_then_the_tarocks_high_to_low() {
        walk(2, |s, seat| {
            let Some(t) = tracker(&view(s, seat)) else {
                return;
            };
            let kinds: Vec<Kind> = t.groups.iter().map(|g| g.kind).collect();
            assert_eq!(kinds, Kind::ALL.to_vec());
            for g in &t.groups {
                for list in [&g.mine, &g.out] {
                    assert!(list.windows(2).all(|w| w[0].strength() > w[1].strength()));
                }
                assert_eq!(g.top_out, g.out.first().copied());
                assert!(g.unseen <= g.out.len());
            }
        });
    }

    #[test]
    fn the_output_depends_on_the_view_alone() {
        let twins = blind_check_values(&tracker, 12);
        assert!(twins > 20, "only {twins} twins were checked");
    }
}
