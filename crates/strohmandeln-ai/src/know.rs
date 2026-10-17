//! What a seat knows, read from its view: the one place the rungs get their
//! facts, so none of them can reach a hidden card (RULES 13).
//!
//! `Knowledge::from_view` takes only a [`View`], which by construction
//! (`strohmandeln_core::view`) holds no hidden card. From it the counting
//! rungs rebuild everything a perfect memory would hold: every card played,
//! the cards still unseen, and the kinds the opponent has shown himself void
//! in.

use crate::cardset::{CardSet, Kind};
use strohmandeln_core::cards::{Card, Honour};
use strohmandeln_core::types::*;
use strohmandeln_core::view::View;

#[derive(Clone, Debug)]
pub struct Knowledge {
    pub me: Seat,
    pub opp: Seat,
    /// Whether the bidding is over and play has begun.
    pub playing: bool,
    pub declarer: Option<Seat>,
    /// The sitting's game values, both-pass setting and the deal's doubling
    /// multiplier: public, and what a bid is worth.
    pub values: Values,
    pub both_pass: BothPass,
    pub multiplier: u32,
    /// The non-dealer, who bids first and leads the first trick.
    pub forehand: Seat,
    /// 1 to 27.
    pub trick_number: u32,
    /// The card the opponent led to the open trick, when I follow.
    pub lead: Option<Card>,
    /// My hand cards.
    pub hand: CardSet,
    /// My covers by packet (0 to 2).
    pub covers: [Option<Card>; 3],
    pub opp_covers: [Option<Card>; 3],
    /// Face-down cards left in each of my packets, and in the opponent's.
    pub my_hidden: [u8; 3],
    pub opp_hidden: [u8; 3],
    pub opp_hand_size: usize,
    /// The opponent's hand cards I know: pulled and unplayed, or announced.
    pub opp_known: CardSet,
    /// Every card played, in completed tricks and the open one.
    pub played: CardSet,
    /// Kinds the opponent could not follow, so none of his dealt (private)
    /// hand cards is of that kind. Indexed by `Kind::index`.
    pub opp_void: [bool; 5],
    /// Cards I cannot place: not in my hand, not a cover, not played, not
    /// known in the opponent's hand. They are the opponent's private hand
    /// cards and the face-down packet cards of both players.
    pub unseen: CardSet,
    /// Takes in thirds, by seat index.
    pub take: [u32; 2],
    /// The cards I may play now (empty when it is not my move to play).
    pub legal: Vec<Card>,
    /// The honours I may announce now.
    pub announceable: Vec<Honour>,
    pub announced_by_opp: Vec<Honour>,
}

impl Knowledge {
    /// `None` when the view has no deal in progress.
    pub fn from_view(view: &View) -> Option<Knowledge> {
        let d = view.deal.as_ref()?;
        let me = view.seat;
        let opp = me.other();
        let hand = CardSet::of(d.own_hand.iter().map(|h| h.card));
        let covers = d.covers[me.index()];
        let opp_covers = d.covers[opp.index()];
        let opp_known = CardSet::of(d.opponent_known.iter().copied());
        let mut played = CardSet::EMPTY;
        let mut opp_void = [false; 5];
        for t in &d.tricks {
            played.insert(t.cards[0]);
            played.insert(t.cards[1]);
            if t.leader == me {
                let (led, reply) = (t.cards[0], t.cards[1]);
                if Kind::of(reply) != Kind::of(led) {
                    // He could not follow: nothing of the kind in his
                    // playable set, so none among his dealt cards. And if
                    // he did not trump a suit, no tarock either (RULES 5.4).
                    opp_void[Kind::of(led).index()] = true;
                    if !reply.is_tarock() {
                        opp_void[Kind::Trump.index()] = true;
                    }
                }
            }
        }
        let lead = d.lead.as_ref().map(|l| l.card);
        if let Some(l) = lead {
            played.insert(l);
        }
        let mut seen = hand.union(played).union(opp_known);
        for c in covers.iter().chain(opp_covers.iter()).flatten() {
            seen.insert(*c);
        }
        let unseen = CardSet::full().minus(seen);
        let mut legal = Vec::new();
        let mut announceable = Vec::new();
        for a in &view.legal_actions {
            match a {
                Action::Play { card, .. } => legal.push(*card),
                Action::Announce { honour } => announceable.push(*honour),
                _ => {}
            }
        }
        Some(Knowledge {
            me,
            opp,
            playing: d.playing,
            declarer: d.declarer,
            values: d.values,
            both_pass: d.both_pass,
            multiplier: d.multiplier,
            forehand: d.forehand,
            trick_number: d.trick_number,
            lead,
            hand,
            covers,
            opp_covers,
            my_hidden: d.hidden[me.index()],
            opp_hidden: d.hidden[opp.index()],
            opp_hand_size: d.opponent_hand_size,
            opp_known,
            played,
            opp_void,
            unseen,
            take: d.take_thirds,
            legal,
            announceable,
            announced_by_opp: d
                .announcements
                .iter()
                .filter(|a| a.seat == opp)
                .map(|a| a.honour)
                .collect(),
        })
    }

    /// My playable set: hand cards and covers.
    pub fn playable(&self) -> CardSet {
        let mut s = self.hand;
        for c in self.covers.iter().flatten() {
            s.insert(*c);
        }
        s
    }

    /// Every card the opponent might still play: the unseen ones, those he
    /// is known to hold and his covers.
    pub fn opp_possible(&self) -> CardSet {
        let mut s = self.unseen.union(self.opp_known);
        for c in self.opp_covers.iter().flatten() {
            s.insert(*c);
        }
        s
    }

    /// The opponent's dealt cards still in his hand, which I cannot see.
    pub fn opp_private(&self) -> usize {
        self.opp_hand_size - self.opp_known.len()
    }

    pub fn is_cover(&self, c: Card) -> bool {
        self.covers.contains(&Some(c))
    }

    /// Face-down cards in all six packets.
    pub fn hidden_total(&self) -> usize {
        self.my_hidden
            .iter()
            .chain(self.opp_hidden.iter())
            .map(|&n| usize::from(n))
            .sum()
    }

    /// Tarocks I can play (hand and covers).
    pub fn my_tarocks(&self) -> usize {
        self.playable().iter().filter(|c| c.is_tarock()).count()
    }

    /// The tarocks still out, as the Cards-out tab counts them
    /// (`tracker`, `Group::out`): not mine and not played, so the unseen ones
    /// and those face up on his covers or known in his hand.
    pub fn out_tarocks(&self) -> usize {
        let mine = self.playable();
        Card::all()
            .filter(|c| c.is_tarock() && !self.played.contains(*c) && !mine.contains(*c))
            .count()
    }

    /// The tarocks I cannot place, the opponent's or in packets.
    pub fn unseen_tarocks(&self) -> usize {
        self.unseen.iter().filter(|c| c.is_tarock()).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    #[test]
    fn the_unseen_cards_are_exactly_the_places_i_cannot_see() {
        let mut checked = 0;
        for seed in 0..12 {
            walk(seed, |s, seat| {
                let Some(k) = Knowledge::from_view(&view(s, seat)) else {
                    return;
                };
                assert_eq!(
                    k.unseen.len(),
                    k.opp_private() + k.hidden_total(),
                    "seed {seed} trick {}",
                    k.trick_number
                );
                checked += 1;
            });
        }
        assert!(checked > 1000);
    }

    #[test]
    fn the_unseen_set_holds_the_true_hidden_cards_and_nothing_i_can_see() {
        for seed in 0..8 {
            walk(seed, |s, seat| {
                let Some(k) = Knowledge::from_view(&view(s, seat)) else {
                    return;
                };
                let deal = s.deal().unwrap();
                let opp = seat.other();
                let mut truth = CardSet::EMPTY;
                for h in deal.hand(opp) {
                    if !k.opp_known.contains(h.card) {
                        truth.insert(h.card);
                    }
                }
                for owner in Seat::BOTH {
                    for p in 1..=3u8 {
                        let all = deal.packet_cards(owner, p);
                        let skip = usize::from(deal.cover(owner, p).is_some());
                        for c in all.into_iter().skip(skip) {
                            truth.insert(c);
                        }
                    }
                }
                assert_eq!(truth, k.unseen, "seed {seed}");
            });
        }
    }

    #[test]
    fn a_void_is_inferred_only_when_the_opponent_really_has_none() {
        let mut voids = 0;
        for seed in 0..40 {
            walk(seed, |s, seat| {
                let Some(k) = Knowledge::from_view(&view(s, seat)) else {
                    return;
                };
                let deal = s.deal().unwrap();
                for h in deal.hand(seat.other()) {
                    if !h.public {
                        let kind = Kind::of(h.card);
                        assert!(!k.opp_void[kind.index()], "seed {seed}: {:?}", h.card);
                    }
                }
                voids += k.opp_void.iter().filter(|&&v| v).count();
            });
        }
        assert!(voids > 100, "voids are inferred at all ({voids})");
    }

    #[test]
    fn the_legal_cards_and_announceable_honours_come_from_the_view() {
        walk(5, |s, seat| {
            let v = view(s, seat);
            let Some(k) = Knowledge::from_view(&v) else {
                return;
            };
            if v.legal_actions.is_empty() {
                assert!(k.legal.is_empty());
            } else if k.playing {
                assert!(!k.legal.is_empty());
                assert!(k.legal.iter().all(|&c| k.playable().contains(c)));
            }
        });
    }
}
