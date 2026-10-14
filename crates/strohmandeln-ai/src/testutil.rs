//! Helpers for the tests: walking whole sittings by random legal play,
//! hand-built positions, and the hidden-information check.

use crate::cardset::{CardSet, Kind};
use crate::know::Knowledge;
use strohmandeln_core::cards::Card;
use strohmandeln_core::deal::Deal;
use strohmandeln_core::rng::{purpose, Rng};
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::*;
use strohmandeln_core::view::{deal_view, View};

/// Plays a sitting of 3 Touren by uniformly random legal actions (declining
/// an extension), calling `f(sitting, seat)` before every move.
pub fn walk(seed: u64, mut f: impl FnMut(&Sitting, Seat)) {
    let mut s = Sitting::new(seed, Settings::default(), 3).unwrap();
    let mut rng = Rng::stream(seed, purpose::agent(0));
    while let Some(seat) = s.to_move() {
        f(&s, seat);
        let legal = s.legal_actions();
        let a = match legal[0] {
            Action::Extend { .. } => Action::Extend { accept: false },
            _ => legal[rng.below(legal.len() as u64) as usize].clone(),
        };
        s.apply(seat, a).unwrap();
    }
}

pub fn c(code: &str) -> Card {
    Card::parse(code).unwrap_or_else(|| panic!("bad card code {code}"))
}

/// A bare deal (dealer B, forehand A) with the moves made.
pub struct Scene {
    pub deal: Deal,
    pub history: Vec<(Seat, Action)>,
}

impl Scene {
    pub fn new(deck: &[Card]) -> Scene {
        Scene {
            deal: Deal::new(deck, Seat::B, 1, Settings::default()),
            history: Vec::new(),
        }
    }

    pub fn apply(&mut self, seat: Seat, action: Action) -> Result<(), Refusal> {
        let mut ev = Vec::new();
        self.deal.apply(seat, action.clone(), &mut ev)?;
        self.history.push((seat, action));
        Ok(())
    }

    pub fn view(&self, seat: Seat) -> View {
        let to_move = self.deal.to_move();
        View {
            seat,
            to_move,
            over: false,
            contos: [0, 0],
            balance: 0,
            pending_doubling: false,
            touren_total: 3,
            deals_counted: 0,
            legal_actions: if to_move == Some(seat) {
                self.deal.legal_actions()
            } else {
                Vec::new()
            },
            deal: Some(deal_view(&self.deal, seat)),
            events: Vec::new(),
        }
    }

    /// The cards `seat` cannot see: the opponent's dealt cards still in his
    /// hand and every face-down packet card (RULES 13.2, 13.3).
    pub fn hidden_from(&self, seat: Seat) -> Vec<Card> {
        let known = Knowledge::from_view(&self.view(seat))
            .expect("a deal")
            .opp_known;
        let mut hidden: Vec<Card> = self
            .deal
            .hand(seat.other())
            .iter()
            .map(|h| h.card)
            .filter(|&x| !known.contains(x))
            .collect();
        for owner in Seat::BOTH {
            for p in 1..=3 {
                let skip = usize::from(self.deal.cover(owner, p).is_some());
                hidden.extend(self.deal.packet_cards(owner, p).into_iter().skip(skip));
            }
        }
        hidden
    }
}

fn replayed(deck: &[Card], history: &[(Seat, Action)]) -> Option<Scene> {
    let mut s = Scene::new(deck);
    for (seat, action) in history {
        s.apply(*seat, action.clone()).ok()?;
    }
    Some(s)
}

/// Plays random deals; before each move of the seat to move, builds twins of
/// the deal that differ in the cards that seat cannot see but replay the same
/// moves and give it the very same view, and checks that `decide` answers
/// the same in all of them (the bids included). Returns how many twins were
/// checked.
pub fn blind_check(decide: &dyn Fn(&View) -> Action, deals: u64) -> usize {
    let mut twins_checked = 0;
    for seed in 0..deals {
        let mut rng = Rng::stream(seed, 77);
        let mut deck: Vec<Card> = Card::all().collect();
        rng.shuffle(&mut deck);
        let mut scene = Scene::new(&deck);
        while let Some(seat) = scene.deal.to_move() {
            let v = scene.view(seat);
            let a = decide(&v);
            let hidden = scene.hidden_from(seat);
            let at: Vec<usize> = hidden
                .iter()
                .map(|h| deck.iter().position(|x| x == h).unwrap())
                .collect();
            for _ in 0..4 {
                let mut shuffled = hidden.clone();
                rng.shuffle(&mut shuffled);
                let mut twin_deck = deck.clone();
                for (pos, card) in at.iter().zip(&shuffled) {
                    twin_deck[*pos] = *card;
                }
                if twin_deck == deck {
                    continue;
                }
                let Some(twin) = replayed(&twin_deck, &scene.history) else {
                    continue;
                };
                let tv = twin.view(seat);
                if tv != v {
                    continue;
                }
                assert_eq!(
                    decide(&tv),
                    a,
                    "seed {seed}: the hidden cards changed a decision"
                );
                twins_checked += 1;
            }
            let legal = v.legal_actions;
            let next = legal[rng.below(legal.len() as u64) as usize].clone();
            scene.apply(seat, next).unwrap();
        }
    }
    twins_checked
}

/// A hand-built position for the rungs' unit tests. `covers` are my packet
/// covers (empty string: none), `opp_covers` his. Hidden counts are 3 in
/// every packet with a cover and 0 without. The legal cards follow RULES 5.4.
pub fn know(
    hand: &[&str],
    covers: [&str; 3],
    opp_covers: [&str; 3],
    opp_known: &[&str],
    lead: Option<&str>,
) -> Knowledge {
    let cover = |s: &str| (!s.is_empty()).then(|| c(s));
    let covers = covers.map(cover);
    let opp_covers = opp_covers.map(cover);
    let hand_set = CardSet::of(hand.iter().map(|s| c(s)));
    let opp_known_set = CardSet::of(opp_known.iter().map(|s| c(s)));
    let lead = lead.map(c);
    let mut playable = hand_set;
    for x in covers.iter().flatten() {
        playable.insert(*x);
    }
    let legal: Vec<Card> = match lead {
        None => playable.to_vec(),
        Some(l) => {
            let same: Vec<Card> = playable
                .iter()
                .filter(|&x| Kind::of(x) == Kind::of(l))
                .collect();
            if !same.is_empty() {
                same
            } else {
                let trumps: Vec<Card> = playable.iter().filter(|x| x.is_tarock()).collect();
                if !trumps.is_empty() && !l.is_tarock() {
                    trumps
                } else {
                    playable.to_vec()
                }
            }
        }
    };
    let hidden = |cv: &[Option<Card>; 3]| cv.map(|x| if x.is_some() { 3 } else { 0 });
    let mut seen = hand_set.union(opp_known_set);
    for x in covers.iter().chain(opp_covers.iter()).flatten() {
        seen.insert(*x);
    }
    if let Some(l) = lead {
        seen.insert(l);
    }
    let mut played = CardSet::EMPTY;
    if let Some(l) = lead {
        played.insert(l);
    }
    Knowledge {
        me: Seat::B,
        opp: Seat::A,
        playing: true,
        declarer: Some(Seat::A),
        trick_number: 1,
        lead,
        hand: hand_set,
        covers,
        opp_covers,
        my_hidden: hidden(&covers),
        opp_hidden: hidden(&opp_covers),
        opp_hand_size: 15 + opp_known.len(),
        opp_known: opp_known_set,
        played,
        opp_void: [false; 5],
        unseen: CardSet::full().minus(seen),
        take: [0, 0],
        legal,
        announceable: Vec::new(),
        announced_by_opp: Vec::new(),
    }
}
