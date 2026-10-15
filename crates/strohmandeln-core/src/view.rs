//! Per-player views: exactly what one seat may know (RULES 13).
//!
//! A view is built from the full state but holds only public information and
//! the viewer's own private cards. It never contains the other player's
//! private hand cards or any face-down packet card, and neither the seed nor
//! the deck. `events` are the events of the current deal, filtered for the
//! viewer (the other seat's `dealt` and `honour_forfeited` are dropped).

use crate::cards::{Card, Honour};
use crate::deal::{HandCard, HonourState, TrickRecord};
use crate::session::Sitting;
use crate::types::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct AnnouncementView {
    pub seat: Seat,
    pub honour: Honour,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct LeadView {
    pub seat: Seat,
    pub card: Card,
    pub source: Source,
}

/// What a seat knows of the deal in progress.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DealView {
    pub dealer: Seat,
    pub forehand: Seat,
    pub multiplier: u32,
    /// The sitting's public settings that price a game (RULES 0, 3.4, 9.1).
    pub values: Values,
    pub both_pass: BothPass,
    pub declarer: Option<Seat>,
    pub playing: bool,
    pub trick_number: u32,
    /// The viewer's hand cards, with whether each is public (RULES 13.4).
    pub own_hand: Vec<HandCard>,
    pub opponent_hand_size: usize,
    /// The opponent's hand cards known to the viewer: pulled cards still
    /// unplayed, and the cards of an announced honour (RULES 13.4, 13.5).
    pub opponent_known: Vec<Card>,
    /// The covers by owner seat then packet 1 to 3.
    pub covers: [[Option<Card>; 3]; 2],
    /// Face-down cards left in each packet, by owner seat then packet.
    pub hidden: [[u8; 3]; 2],
    /// Completed tricks of this deal (RULES 13.6).
    pub tricks: Vec<TrickRecord>,
    /// The card led to the open trick.
    pub lead: Option<LeadView>,
    /// The running count of each take in thirds, by seat (RULES 13.1).
    pub take_thirds: [u32; 2],
    pub announcements: Vec<AnnouncementView>,
    pub forfeited: Vec<AnnouncementView>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct View {
    pub seat: Seat,
    pub to_move: Option<Seat>,
    pub over: bool,
    pub contos: [u32; 2],
    pub balance: i32,
    pub pending_doubling: bool,
    pub touren_total: u32,
    pub deals_counted: u32,
    /// The legal actions, when it is the viewer's turn.
    pub legal_actions: Vec<Action>,
    pub deal: Option<DealView>,
    pub events: Vec<Event>,
}

/// An event as `seat` may see it: `None` if it is not for them.
pub fn filter_event(event: &Event, seat: Seat) -> Option<Event> {
    match event {
        Event::Dealt { seat: owner, .. } if *owner != seat => None,
        // A forfeit is private to its holder (RULES 11.5, 13.2).
        Event::HonourForfeited { seat: owner, .. } if *owner != seat => None,
        other => Some(other.clone()),
    }
}

/// The events `apply` returned, as `seat` may see them. `Sitting::apply`
/// returns every event, the other seat's `dealt` included; whatever carries
/// events to a player must pass them through this.
pub fn filter_events(events: &[Event], seat: Seat) -> Vec<Event> {
    events
        .iter()
        .filter_map(|e| filter_event(e, seat))
        .collect()
}

/// The view of `sitting` for `seat`.
pub fn view(sitting: &Sitting, seat: Seat) -> View {
    let all = sitting.events();
    let from = all
        .iter()
        .rposition(|e| matches!(e, Event::DealStart { .. }))
        .unwrap_or(0);
    let events = filter_events(&all[from..], seat);
    let to_move = sitting.to_move();
    View {
        seat,
        to_move,
        over: sitting.is_over(),
        contos: sitting.contos(),
        balance: sitting.balance(),
        pending_doubling: sitting.pending_doubling(),
        touren_total: sitting.touren_total(),
        deals_counted: sitting.deals_counted(),
        legal_actions: if to_move == Some(seat) {
            sitting.legal_actions()
        } else {
            Vec::new()
        },
        deal: sitting.deal().map(|d| deal_view(d, seat)),
        events,
    }
}

/// The view of a bare deal for `seat`, without a sitting around it: the
/// sitting's own fields (Contos, Touren) are zero. What the tutor replays a
/// recorded deal with (`dealrec`).
pub fn bare_view(deal: &crate::deal::Deal, seat: Seat) -> View {
    let to_move = deal.to_move();
    View {
        seat,
        to_move,
        over: false,
        contos: [0, 0],
        balance: 0,
        pending_doubling: false,
        touren_total: 0,
        deals_counted: 0,
        legal_actions: if to_move == Some(seat) {
            deal.legal_actions()
        } else {
            Vec::new()
        },
        deal: Some(deal_view(deal, seat)),
        events: Vec::new(),
    }
}

/// What `seat` knows of `deal`, without a sitting around it (the opponent's
/// measurements and the hidden-information tests play bare deals).
pub fn deal_view(deal: &crate::deal::Deal, seat: Seat) -> DealView {
    let opp = seat.other();
    let announced_cards: Vec<Card> = [Honour::Trull, Honour::Kings]
        .into_iter()
        .filter(|&h| deal.honour_state(opp, h) == HonourState::Announced)
        .flat_map(|h| h.cards())
        .collect();
    let opponent_known = deal
        .hand(opp)
        .iter()
        .filter(|h| h.public || announced_cards.contains(&h.card))
        .map(|h| h.card)
        .collect();
    let covers = std::array::from_fn(|s| {
        std::array::from_fn(|p| deal.cover(Seat::from_index(s), p as u8 + 1))
    });
    let hidden = std::array::from_fn(|s| {
        std::array::from_fn(|p| deal.hidden_count(Seat::from_index(s), p as u8 + 1) as u8)
    });
    let honour_views = |state: HonourState| -> Vec<AnnouncementView> {
        let mut v = Vec::new();
        for honour in [Honour::Trull, Honour::Kings] {
            for s in Seat::BOTH {
                // A forfeit is private to its holder (RULES 13.2).
                let visible = state != HonourState::Forfeited || s == seat;
                if visible && deal.honour_state(s, honour) == state {
                    v.push(AnnouncementView { seat: s, honour });
                }
            }
        }
        v
    };
    DealView {
        dealer: deal.dealer(),
        forehand: deal.forehand(),
        multiplier: deal.multiplier(),
        values: deal.settings().values,
        both_pass: deal.settings().both_pass,
        declarer: deal.declarer(),
        playing: deal.is_playing(),
        trick_number: deal.trick_number(),
        own_hand: deal.hand(seat).to_vec(),
        opponent_hand_size: deal.hand(opp).len(),
        opponent_known,
        covers,
        hidden,
        tricks: deal.tricks().to_vec(),
        lead: deal
            .led_card()
            .map(|(seat, card, source)| LeadView { seat, card, source }),
        take_thirds: std::array::from_fn(|i| {
            deal.take(Seat::from_index(i))
                .iter()
                .map(|c| c.thirds())
                .sum()
        }),
        announcements: honour_views(HonourState::Announced),
        forfeited: honour_views(HonourState::Forfeited),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;
    use crate::deal::HonourState;
    use crate::rng::{purpose, Rng};
    use std::collections::HashSet;

    /// Every card code appearing as a string anywhere in a JSON value.
    fn cards_in(v: &serde_json::Value, out: &mut HashSet<Card>) {
        match v {
            serde_json::Value::String(s) => {
                if let Some(c) = Card::parse(s) {
                    out.insert(c);
                }
            }
            serde_json::Value::Array(a) => a.iter().for_each(|x| cards_in(x, out)),
            serde_json::Value::Object(o) => o.values().for_each(|x| cards_in(x, out)),
            _ => {}
        }
    }

    /// The cards `seat` must not be able to learn from its view: the other
    /// player's private hand cards (not pulled, not in an announced honour)
    /// and every face-down packet card.
    fn hidden_from(sitting: &Sitting, seat: Seat) -> HashSet<Card> {
        let mut hidden = HashSet::new();
        let Some(deal) = sitting.deal() else {
            return hidden;
        };
        let opp = seat.other();
        let announced: Vec<Card> = [Honour::Trull, Honour::Kings]
            .into_iter()
            .filter(|&h| deal.honour_state(opp, h) == HonourState::Announced)
            .flat_map(|h| h.cards())
            .collect();
        for h in deal.hand(opp) {
            if !h.public && !announced.contains(&h.card) {
                hidden.insert(h.card);
            }
        }
        for s in Seat::BOTH {
            for p in 1..=3 {
                let all = deal.packet_cards(s, p);
                let skip = usize::from(deal.cover(s, p).is_some());
                hidden.extend(all.into_iter().skip(skip));
            }
        }
        hidden
    }

    fn random_step(s: &mut Sitting, rng: &mut Rng) {
        let seat = s.to_move().unwrap();
        let legal = s.legal_actions();
        let a = legal[rng.below(legal.len() as u64) as usize].clone();
        s.apply(seat, a).unwrap();
    }

    #[test]
    fn rules_13_7_a_view_never_leaks_a_hidden_card() {
        let mut checked = 0;
        for seed in 0..6u64 {
            let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
            let mut rng = Rng::stream(seed, purpose::agent(0));
            while !s.is_over() {
                for seat in Seat::BOTH {
                    let v = view(&s, seat);
                    let json = serde_json::to_value(&v).unwrap();
                    let mut seen = HashSet::new();
                    cards_in(&json, &mut seen);
                    let hidden = hidden_from(&s, seat);
                    let leaked: Vec<_> = seen.intersection(&hidden).collect();
                    assert!(
                        leaked.is_empty(),
                        "seed {seed} seat {seat:?} leaked {leaked:?}"
                    );
                    checked += 1;
                }
                random_step(&mut s, &mut rng);
            }
        }
        assert!(checked > 2_000);
    }

    #[test]
    fn rules_13_7_the_check_would_catch_a_leak() {
        // Sanity of the test itself: the full state does contain hidden cards,
        // and a view that listed the opponent's hand would be caught.
        let s = Sitting::new(3, Settings::default(), 3).unwrap();
        let hidden = hidden_from(&s, Seat::A);
        assert!(hidden.len() >= 15 + 12 - 12);
        let opp = s.deal().unwrap().hand_cards(Seat::B);
        let fake = serde_json::json!({ "opp": opp });
        let mut seen = HashSet::new();
        cards_in(&fake, &mut seen);
        assert!(seen.intersection(&hidden).count() > 0);
    }

    #[test]
    fn a_bare_deal_gives_the_same_view_as_its_sitting() {
        let s = Sitting::new(11, Settings::default(), 3).unwrap();
        let deal = s.deal().unwrap();
        for seat in Seat::BOTH {
            assert_eq!(Some(deal_view(deal, seat)), view(&s, seat).deal);
        }
    }

    #[test]
    fn rules_13_7_the_seed_and_deck_are_not_in_a_view() {
        let s = Sitting::new(123456789, Settings::default(), 3).unwrap();
        let json = serde_json::to_string(&view(&s, Seat::A)).unwrap();
        assert!(!json.contains("seed"));
        assert!(!json.contains("123456789"));
        assert!(!json.contains("deck"));
    }

    #[test]
    fn rules_13_2_you_see_your_own_hand_and_only_its_size_of_the_opponents() {
        let s = Sitting::new(4, Settings::default(), 3).unwrap();
        let deal = s.deal().unwrap();
        let v = view(&s, Seat::A);
        let d = v.deal.as_ref().unwrap();
        let mine: Vec<Card> = d.own_hand.iter().map(|h| h.card).collect();
        assert_eq!(mine, deal.hand_cards(Seat::A));
        assert_eq!(d.opponent_hand_size, deal.hand(Seat::B).len());
        // before the reveal nothing of the opponent's is known
        assert!(d.opponent_known.is_empty() || deal.is_playing());
    }

    #[test]
    fn rules_13_1_covers_and_hidden_counts_of_all_six_packets_are_public() {
        let mut s = Sitting::new(8, Settings::default(), 3).unwrap();
        let first = s.to_move().unwrap();
        s.apply(first, Action::Take).unwrap();
        let deal = s.deal().unwrap().clone();
        for seat in Seat::BOTH {
            let v = view(&s, seat);
            let d = v.deal.unwrap();
            for owner in Seat::BOTH {
                for p in 1..=3u8 {
                    assert_eq!(
                        d.covers[owner.index()][usize::from(p) - 1],
                        deal.cover(owner, p)
                    );
                    assert_eq!(
                        usize::from(d.hidden[owner.index()][usize::from(p) - 1]),
                        deal.hidden_count(owner, p)
                    );
                }
            }
        }
    }

    #[test]
    fn rules_13_4_pulled_cards_are_known_to_the_opponent_dealt_cards_are_not() {
        let mut s = Sitting::new(8, Settings::default(), 3).unwrap();
        let first = s.to_move().unwrap();
        s.apply(first, Action::Take).unwrap();
        let deal = s.deal().unwrap().clone();
        let v = view(&s, first.other());
        let d = v.deal.unwrap();
        let public: Vec<Card> = deal
            .hand(first)
            .iter()
            .filter(|h| h.public)
            .map(|h| h.card)
            .collect();
        let mut known = d.opponent_known.clone();
        known.sort();
        let mut want = public;
        want.sort();
        assert_eq!(known, want);
    }

    #[test]
    fn rules_13_5_an_announcement_makes_the_honour_cards_known() {
        // find a deal where someone can announce, then announce
        for seed in 0..300u64 {
            let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
            let first = s.to_move().unwrap();
            s.apply(first, Action::Take).unwrap();
            let Some(seat) = s.to_move() else { continue };
            let Some(h) = s.deal().unwrap().legal_announcements(seat).first().copied() else {
                continue;
            };
            s.apply(seat, Action::Announce { honour: h }).unwrap();
            let v = view(&s, seat.other());
            let d = v.deal.unwrap();
            for card in h.cards() {
                assert!(d.opponent_known.contains(&card), "{card:?}");
            }
            assert!(d
                .announcements
                .contains(&AnnouncementView { seat, honour: h }));
            return;
        }
        panic!("no announceable hand found in 300 seeds");
    }

    #[test]
    fn rules_16_the_other_players_dealt_event_is_filtered_out() {
        let s = Sitting::new(4, Settings::default(), 3).unwrap();
        let v = view(&s, Seat::A);
        let dealt: Vec<&Event> = v
            .events
            .iter()
            .filter(|e| matches!(e, Event::Dealt { .. }))
            .collect();
        assert_eq!(dealt.len(), 1);
        assert!(matches!(dealt[0], Event::Dealt { seat: Seat::A, .. }));
        assert_eq!(
            filter_event(
                &Event::Dealt {
                    seat: Seat::B,
                    cards: vec![]
                },
                Seat::A
            ),
            None
        );
        assert!(filter_event(&Event::ThrownIn, Seat::A).is_some());
    }

    #[test]
    fn filter_events_keeps_everything_but_the_other_seats_dealt() {
        let mut s = Sitting::new(4, Settings::default(), 3).unwrap();
        let first = s.to_move().unwrap();
        let mut all = s.events().to_vec();
        all.extend(s.apply(first, Action::Take).unwrap());
        for seat in Seat::BOTH {
            let kept = filter_events(&all, seat);
            let others = all
                .iter()
                .filter(|e| matches!(e, Event::Dealt { seat: o, .. } if *o != seat))
                .count();
            assert!(others >= 1);
            assert_eq!(kept.len(), all.len() - others);
            assert!(kept
                .iter()
                .all(|e| !matches!(e, Event::Dealt { seat: o, .. } if *o != seat)));
        }
    }

    #[test]
    fn legal_actions_are_in_the_view_only_on_your_turn() {
        let s = Sitting::new(4, Settings::default(), 3).unwrap();
        let mover = s.to_move().unwrap();
        assert_eq!(view(&s, mover).legal_actions, s.legal_actions());
        assert!(view(&s, mover.other()).legal_actions.is_empty());
        assert_eq!(view(&s, mover).to_move, Some(mover));
    }

    #[test]
    fn the_running_count_of_each_take_is_public() {
        let mut s = Sitting::new(4, Settings::default(), 3).unwrap();
        let mut rng = Rng::seeded(1);
        // play until a trick has been won
        while s.deal().map(|d| d.tricks().is_empty()).unwrap_or(false) {
            random_step(&mut s, &mut rng);
        }
        let deal = s.deal().unwrap();
        let v = view(&s, Seat::B);
        let d = v.deal.unwrap();
        let want: [u32; 2] = std::array::from_fn(|i| {
            deal.take(Seat::from_index(i))
                .iter()
                .map(|c| c.thirds())
                .sum()
        });
        assert_eq!(d.take_thirds, want);
        assert_eq!(d.tricks.len(), deal.tricks().len());
    }

    /// A deck in which forehand holds the whole Trull (the first three cards
    /// go to forehand, RULES 2.3).
    fn trull_deck() -> Vec<Card> {
        let trull: Vec<Card> = ["SK", "T21", "T1"]
            .iter()
            .map(|c| Card::parse(c).unwrap())
            .collect();
        let rest = Card::all().filter(|c| !trull.contains(c));
        trull.iter().copied().chain(rest).collect()
    }

    #[test]
    fn rules_13_a_forfeit_is_private_to_its_holder() {
        use crate::types::{Action, Event, Settings};
        let mut s = Sitting::with_decks(1, Settings::default(), 3, vec![trull_deck()]).unwrap();
        let holder = s.deal().unwrap().forehand();
        let other = holder.other();
        while s.to_move() != Some(holder) || s.legal_actions().first() == Some(&Action::Take) {
            let seat = s.to_move().unwrap();
            let a = s.legal_actions()[0].clone();
            s.apply(seat, a).unwrap();
        }
        // The holder leads a card of the Trull without announcing it.
        let t1 = Card::parse("T1").unwrap();
        let events = s.apply(holder, Action::play(t1)).unwrap();
        assert!(events
            .iter()
            .any(|e| matches!(e, Event::HonourForfeited { seat, .. } if *seat == holder)));

        // The holder is told; the opponent is not, by event or by view.
        let is_forfeit = |e: &Event| matches!(e, Event::HonourForfeited { .. });
        assert!(filter_events(&events, holder).iter().any(is_forfeit));
        assert!(!filter_events(&events, other).iter().any(is_forfeit));
        let mine = view(&s, holder);
        assert_eq!(mine.deal.as_ref().unwrap().forfeited.len(), 1);
        assert!(mine.events.iter().any(is_forfeit));
        let theirs = view(&s, other);
        assert!(theirs.deal.as_ref().unwrap().forfeited.is_empty());
        assert!(!theirs.events.iter().any(is_forfeit));
        let wire = serde_json::to_string(&theirs).unwrap();
        assert!(!wire.contains("honour_forfeited"), "{wire}");
        // Nor can it be read from the filtered stream of the whole sitting.
        assert!(!filter_events(s.events(), other).iter().any(is_forfeit));
    }
}

#[cfg(test)]
mod settings_tests {
    use super::*;
    use crate::session::Sitting;

    #[test]
    fn the_deal_view_carries_the_public_settings() {
        let settings = Settings {
            values: Values::V123,
            both_pass: BothPass::ThrowIn,
            ..Settings::default()
        };
        let s = Sitting::new(2, settings, 3).unwrap();
        let v = view(&s, Seat::A);
        let d = v.deal.unwrap();
        assert_eq!((d.values, d.both_pass), (Values::V123, BothPass::ThrowIn));
        let json = serde_json::to_value(&d).unwrap();
        assert_eq!(json["values"], "1/2/3");
        assert_eq!(json["both_pass"], "throw_in");
    }
}
