//! One deal: dealing, bidding, the reveal, play and scoring (RULES 2 to 12, 14).
//!
//! `Deal` is the whole state of a deal, including the hidden cards; what a
//! player may see is made from it by `view`. It is driven by `apply`, which
//! checks the action, changes the state and appends the events.

use crate::cards::{Card, Honour};
use crate::types::*;
use std::collections::VecDeque;

/// A card in a hand, and whether it is publicly known (RULES 13.4): a pulled
/// card is public, a dealt card private.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HandCard {
    pub card: Card,
    pub public: bool,
}

/// A straw man: the cards still face down, top first, and the cover (RULES 2.4).
#[derive(Clone, Debug, Default)]
struct Packet {
    rest: VecDeque<Card>,
    cover: Option<Card>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Bidding(Seat),
    Playing,
    Done,
}

/// How a finished deal ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DealEnd {
    ThrownIn,
    Played(DealScore),
}

/// The outcome of a played deal (RULES 8 to 12, 14.2): the takes in thirds,
/// the result, and every payment, already doubled if a doubling applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DealScore {
    pub result: GameResult,
    pub thirds: [u32; 2],
    pub credits: Vec<Credit>,
    pub valat: Option<Seat>,
    pub ultimo: Option<(Seat, UltimoResult)>,
    pub multiplier: u32,
}

/// What became of one seat's claim to one honour (RULES 11.3, 11.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HonourState {
    Open,
    Announced,
    Forfeited,
}

/// Valat is worth 12 Points (RULES 10.1); the threshold is 107 thirds (8.2).
pub const VALAT_POINTS: u32 = 12;
pub const THRESHOLD_THIRDS: u32 = 107;

fn honour_index(h: Honour) -> usize {
    match h {
        Honour::Trull => 0,
        Honour::Kings => 1,
    }
}

const HONOURS: [Honour; 2] = [Honour::Trull, Honour::Kings];

#[derive(Clone, Debug)]
pub struct Deal {
    dealer: Seat,
    settings: Settings,
    multiplier: u32,
    hands: [Vec<HandCard>; 2],
    packets: [[Packet; 3]; 2],
    phase: Phase,
    declarer: Option<Seat>,
    end: Option<DealEnd>,
    /// The leader of the current trick.
    leader: Seat,
    /// Number of the current trick, 1 to 27.
    trick: u32,
    /// The card led to the current trick, with its player and source.
    led: Option<(Seat, Card, Source)>,
    /// The cards taken by each seat (RULES 7.2).
    takes: [Vec<Card>; 2],
    /// Completed tricks, in order.
    log: Vec<TrickRecord>,
    /// Announcement state, by seat then honour (Trull, Kings).
    honours: [[HonourState; 2]; 2],
}

/// A completed trick: leader's play first.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrickRecord {
    pub leader: Seat,
    pub cards: [Card; 2],
    pub sources: [Source; 2],
    pub winner: Seat,
}

/// The suit led (RULES 5.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Led {
    Trump,
    Suit(crate::cards::Suit),
}

impl Led {
    fn of(card: Card) -> Led {
        match card.suit() {
            None => Led::Trump,
            Some(s) => Led::Suit(s),
        }
    }
    fn matches(self, card: Card) -> bool {
        Led::of(card) == self
    }
}

impl Deal {
    /// Deals from a shuffled deck (RULES 2.2 to 2.4). `deck` has 54 cards;
    /// index 0 is the first dealt.
    pub fn new(deck: &[Card], dealer: Seat, multiplier: u32, settings: Settings) -> Deal {
        assert_eq!(deck.len(), Card::COUNT, "a deal needs the whole pack");
        let forehand = dealer.other();
        let mut hands: [Vec<HandCard>; 2] = [Vec::new(), Vec::new()];
        for round in 0..6 {
            let seat = if round % 2 == 0 { forehand } else { dealer };
            for &card in &deck[5 * round..5 * round + 5] {
                hands[seat.index()].push(HandCard {
                    card,
                    public: false,
                });
            }
        }
        let mut packets: [[Packet; 3]; 2] = Default::default();
        for i in 0..6 {
            let seat = if i % 2 == 0 { forehand } else { dealer };
            let slot = i / 2;
            packets[seat.index()][slot].rest =
                deck[30 + 4 * i..34 + 4 * i].iter().copied().collect();
        }
        Deal {
            dealer,
            settings,
            multiplier,
            hands,
            packets,
            phase: Phase::Bidding(forehand),
            declarer: None,
            end: None,
            leader: forehand,
            trick: 1,
            led: None,
            takes: [Vec::new(), Vec::new()],
            log: Vec::new(),
            honours: [[HonourState::Open; 2]; 2],
        }
    }

    /// A deal in the playing phase with the given hands and packets, for
    /// tests: `(cover, face-down cards top first)` per packet.
    #[cfg(test)]
    pub(crate) fn arranged(
        dealer: Seat,
        hands: [&[&str]; 2],
        packets: [[(Option<&str>, &[&str]); 3]; 2],
        settings: Settings,
    ) -> Deal {
        let code = |c: &str| Card::parse(c).unwrap_or_else(|| panic!("bad code {c}"));
        let mut deal = Deal::new(&Card::all().collect::<Vec<_>>(), dealer, 1, settings);
        for s in 0..2 {
            deal.hands[s] = hands[s]
                .iter()
                .map(|c| HandCard {
                    card: code(c),
                    public: false,
                })
                .collect();
            for (p, (cover, rest)) in packets[s].iter().enumerate() {
                deal.packets[s][p] = Packet {
                    cover: cover.map(code),
                    rest: rest.iter().map(|c| code(c)).collect(),
                };
            }
        }
        deal.phase = Phase::Playing;
        deal
    }

    pub fn dealer(&self) -> Seat {
        self.dealer
    }

    pub fn forehand(&self) -> Seat {
        self.dealer.other()
    }

    pub fn multiplier(&self) -> u32 {
        self.multiplier
    }

    pub fn declarer(&self) -> Option<Seat> {
        self.declarer
    }

    pub fn is_playing(&self) -> bool {
        self.phase == Phase::Playing
    }

    pub fn end(&self) -> Option<&DealEnd> {
        self.end.as_ref()
    }

    /// The hand cards of a seat with their public flags.
    pub fn hand(&self, seat: Seat) -> &[HandCard] {
        &self.hands[seat.index()]
    }

    /// The hand cards of a seat, in hand order.
    pub fn hand_cards(&self, seat: Seat) -> Vec<Card> {
        self.hands[seat.index()].iter().map(|h| h.card).collect()
    }

    /// All cards still in a packet, the cover first, then the face-down
    /// cards top first. Full knowledge: not for views.
    pub fn packet_cards(&self, seat: Seat, packet: u8) -> Vec<Card> {
        let p = &self.packets[seat.index()][usize::from(packet) - 1];
        p.cover.iter().chain(p.rest.iter()).copied().collect()
    }

    pub fn cover(&self, seat: Seat, packet: u8) -> Option<Card> {
        self.packets[seat.index()][usize::from(packet) - 1].cover
    }

    pub fn hidden_count(&self, seat: Seat, packet: u8) -> usize {
        self.packets[seat.index()][usize::from(packet) - 1]
            .rest
            .len()
    }

    /// Seats whose 15 hand cards hold no tarock (RULES 2.6). Packets are not
    /// looked at.
    pub fn void_seats(&self) -> Vec<Seat> {
        Seat::BOTH
            .into_iter()
            .filter(|s| !self.hands[s.index()].iter().any(|h| h.card.is_tarock()))
            .collect()
    }

    /// Whose action is awaited.
    pub fn to_move(&self) -> Option<Seat> {
        match self.phase {
            Phase::Bidding(s) => Some(s),
            Phase::Playing => Some(match self.led {
                None => self.leader,
                Some((s, _, _)) => s.other(),
            }),
            Phase::Done => None,
        }
    }

    /// The number of the current trick (1 to 27).
    pub fn trick_number(&self) -> u32 {
        self.trick
    }

    pub fn leader(&self) -> Seat {
        self.leader
    }

    pub fn tricks(&self) -> &[TrickRecord] {
        &self.log
    }

    /// The card led to the open trick, if any.
    pub fn led_card(&self) -> Option<(Seat, Card, Source)> {
        self.led
    }

    /// The state of a seat's claim to an honour.
    pub fn honour_state(&self, seat: Seat, honour: Honour) -> HonourState {
        self.honours[seat.index()][honour_index(honour)]
    }

    /// Whether every card of the honour is among the seat's hand cards now
    /// (RULES 11.2).
    fn holds(&self, seat: Seat, honour: Honour) -> bool {
        let hand = &self.hands[seat.index()];
        honour
            .cards()
            .iter()
            .all(|c| hand.iter().any(|h| h.card == *c))
    }

    /// The honours a seat may announce now (RULES 11.4).
    pub fn legal_announcements(&self, seat: Seat) -> Vec<Honour> {
        if self.phase != Phase::Playing || self.to_move() != Some(seat) {
            return Vec::new();
        }
        HONOURS
            .into_iter()
            .filter(|&h| self.honour_state(seat, h) == HonourState::Open && self.holds(seat, h))
            .collect()
    }

    fn announce_refusal(&self, seat: Seat, honour: Honour) -> Option<Refusal> {
        match self.honour_state(seat, honour) {
            HonourState::Announced => {
                return Some(Refusal::of(RefusalKind::AlreadyAnnounced).with_honour(honour))
            }
            HonourState::Forfeited => {
                return Some(Refusal::of(RefusalKind::HonourForfeited).with_honour(honour))
            }
            HonourState::Open => {}
        }
        if !self.holds(seat, honour) {
            let hand = &self.hands[seat.index()];
            let missing: Vec<Card> = honour
                .cards()
                .into_iter()
                .filter(|c| !hand.iter().any(|h| h.card == *c))
                .collect();
            return Some(
                Refusal::of(RefusalKind::NotHeld)
                    .with_honour(honour)
                    .with_missing(missing),
            );
        }
        None
    }

    fn announce(&mut self, seat: Seat, honour: Honour, events: &mut Vec<Event>) {
        self.honours[seat.index()][honour_index(honour)] = HonourState::Announced;
        events.push(Event::Announcement {
            seat,
            honour,
            cards: honour.cards(),
        });
    }

    pub fn take(&self, seat: Seat) -> &[Card] {
        &self.takes[seat.index()]
    }

    /// Every card a seat may play to the current trick, with its source
    /// (RULES 5.2 to 5.7).
    pub fn legal_plays(&self, seat: Seat) -> Vec<(Card, Source)> {
        let set = self.playable(seat);
        let Some((_, led_card, _)) = self.led else {
            return set;
        };
        if self.to_move() != Some(seat) {
            return Vec::new();
        }
        let led = Led::of(led_card);
        let following: Vec<_> = set
            .iter()
            .filter(|(c, _)| led.matches(*c))
            .copied()
            .collect();
        if !following.is_empty() {
            return following;
        }
        if led != Led::Trump {
            let trumps: Vec<_> = set.iter().filter(|(c, _)| c.is_tarock()).copied().collect();
            if !trumps.is_empty() {
                return trumps;
            }
        }
        set
    }

    /// Hand plus covers (RULES 5.2).
    fn playable(&self, seat: Seat) -> Vec<(Card, Source)> {
        let mut v: Vec<(Card, Source)> = self.hands[seat.index()]
            .iter()
            .map(|h| (h.card, Source::Hand))
            .collect();
        for (i, p) in self.packets[seat.index()].iter().enumerate() {
            if let Some(c) = p.cover {
                v.push((c, Source::Packet(i as u8 + 1)));
            }
        }
        v
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        match self.phase {
            Phase::Bidding(_) => vec![Action::Take, Action::Pass],
            Phase::Playing => {
                let seat = self.to_move().expect("someone is to play");
                let mut v: Vec<Action> = self
                    .legal_announcements(seat)
                    .into_iter()
                    .map(|honour| Action::Announce { honour })
                    .collect();
                v.extend(
                    self.legal_plays(seat)
                        .into_iter()
                        .map(|(card, _)| Action::play(card)),
                );
                v
            }
            Phase::Done => Vec::new(),
        }
    }

    /// Applies an action by `seat`, appending its events, or refuses it with
    /// a reason and changes nothing.
    pub fn apply(
        &mut self,
        seat: Seat,
        action: Action,
        events: &mut Vec<Event>,
    ) -> Result<(), Refusal> {
        if self.phase == Phase::Done {
            return Err(Refusal::of(RefusalKind::Finished));
        }
        match action {
            Action::Take | Action::Pass => self.bid(seat, action, events),
            Action::Extend { .. } => Err(Refusal::of(RefusalKind::WrongPhase)),
            Action::Announce { honour } => {
                self.check_turn(seat)?;
                if let Some(r) = self.announce_refusal(seat, honour) {
                    return Err(r);
                }
                self.announce(seat, honour, events);
                Ok(())
            }
            Action::Play { card, announce } => self.play(seat, card, &announce, events),
        }
    }

    fn check_turn(&self, seat: Seat) -> Result<(), Refusal> {
        if matches!(self.phase, Phase::Bidding(_)) {
            return Err(Refusal::of(RefusalKind::WrongPhase));
        }
        match self.to_move() {
            Some(s) if s == seat => Ok(()),
            Some(s) => Err(Refusal::of(RefusalKind::NotYourTurn).with_speaker(s)),
            None => Err(Refusal::of(RefusalKind::Finished)),
        }
    }

    fn play(
        &mut self,
        seat: Seat,
        card: Card,
        announce: &[Honour],
        events: &mut Vec<Event>,
    ) -> Result<(), Refusal> {
        self.check_turn(seat)?;
        for (i, &h) in announce.iter().enumerate() {
            if announce[..i].contains(&h) {
                return Err(Refusal::of(RefusalKind::DuplicateAnnouncement).with_honour(h));
            }
            if let Some(r) = self.announce_refusal(seat, h) {
                return Err(r);
            }
        }
        let Some(&(_, source)) = self.playable(seat).iter().find(|(c, _)| *c == card) else {
            return Err(Refusal::of(RefusalKind::NotPlayable).with_card(card));
        };
        let legal = self.legal_plays(seat);
        if !legal.iter().any(|(c, _)| *c == card) {
            return Err(self.follow_refusal(&legal, card));
        }
        // The announcements come first, then the card (RULES 11.3).
        for &h in announce {
            self.announce(seat, h, events);
        }
        let forfeits: Vec<Honour> = card
            .honours()
            .into_iter()
            .filter(|&h| self.honour_state(seat, h) == HonourState::Open && self.holds(seat, h))
            .collect();
        match source {
            Source::Hand => {
                let hand = &mut self.hands[seat.index()];
                let at = hand.iter().position(|h| h.card == card).expect("held");
                hand.remove(at);
            }
            Source::Packet(n) => {
                self.packets[seat.index()][usize::from(n) - 1].cover = None;
            }
        }
        events.push(Event::CardPlayed {
            seat,
            card,
            source,
            trick: self.trick,
        });
        for honour in forfeits {
            self.honours[seat.index()][honour_index(honour)] = HonourState::Forfeited;
            events.push(Event::HonourForfeited { seat, honour });
        }
        match self.led {
            None => self.led = Some((seat, card, source)),
            Some((leader, led_card, led_source)) => {
                self.led = None;
                self.finish_trick(leader, [led_card, card], [led_source, source], events);
            }
        }
        Ok(())
    }

    fn follow_refusal(&self, legal: &[(Card, Source)], tried: Card) -> Refusal {
        let (_, led_card, _) = self.led.expect("a refusal needs a led card");
        let allowed: Vec<Card> = legal.iter().map(|(c, _)| *c).collect();
        let led = Led::of(led_card);
        if allowed.iter().all(|c| led.matches(*c)) {
            let name = match led {
                Led::Trump => "trump".to_string(),
                Led::Suit(s) => s.letter().to_string(),
            };
            Refusal::of(RefusalKind::MustFollowSuit)
                .with_card(tried)
                .with_led(&name)
                .with_allowed(allowed)
        } else {
            Refusal::of(RefusalKind::MustTrump)
                .with_card(tried)
                .with_allowed(allowed)
        }
    }

    /// Trick over (RULES 7): winner takes both cards, then replacement (6.1).
    fn finish_trick(
        &mut self,
        leader: Seat,
        cards: [Card; 2],
        sources: [Source; 2],
        events: &mut Vec<Event>,
    ) {
        let winner = if trick_won_by_leader(cards[0], cards[1]) {
            leader
        } else {
            leader.other()
        };
        self.takes[winner.index()].extend(cards);
        events.push(Event::TrickWon {
            winner,
            cards,
            trick: self.trick,
        });
        self.log.push(TrickRecord {
            leader,
            cards,
            sources,
            winner,
        });
        // Replacement, leader's packet first (RULES 6.3).
        for (seat, source) in [(leader, sources[0]), (leader.other(), sources[1])] {
            if let Source::Packet(n) = source {
                self.turn_packet(seat, n, events);
            }
        }
        self.trick += 1;
        self.leader = winner;
        if self.trick > 27 {
            self.finish_deal(events);
        }
    }

    /// After trick 27: the ultimo, the Valat, the count and the score.
    fn finish_deal(&mut self, events: &mut Vec<Event>) {
        let score = self.score();
        if let Some((seat, result)) = score.ultimo {
            events.push(Event::PagatUltimo { seat, result });
        }
        if let Some(seat) = score.valat {
            events.push(Event::Valat { seat });
        }
        events.push(Event::Count {
            thirds: score.thirds,
            declarer: self.declarer,
            threshold: THRESHOLD_THIRDS,
        });
        self.phase = Phase::Done;
        self.end = Some(DealEnd::Played(score));
    }

    /// Counts the takes and works out every payment of the deal from the
    /// tricks played and the announcements made (RULES 8 to 12, 14.2).
    pub(crate) fn score(&self) -> DealScore {
        let thirds: [u32; 2] =
            std::array::from_fn(|i| self.takes[i].iter().map(|c| c.thirds()).sum());
        let tricks_won = |seat: Seat| self.log.iter().filter(|t| t.winner == seat).count();
        let valat = Seat::BOTH
            .into_iter()
            .find(|&s| tricks_won(s) == self.log.len() && self.log.len() == 27);
        let (simple, wins, loses) = self.settings.values.triple();
        let mut credits = Vec::new();
        let result = match (self.declarer, valat) {
            (Some(d), Some(v)) => {
                if d == v {
                    GameResult::DeclarerWin
                } else {
                    GameResult::DeclarerLoss
                }
            }
            (None, Some(_)) => GameResult::SimpleWin,
            (Some(d), None) => {
                if thirds[d.index()] >= THRESHOLD_THIRDS {
                    credits.push(Credit {
                        seat: d,
                        cause: Cause::Game,
                        points: wins,
                    });
                    GameResult::DeclarerWin
                } else {
                    credits.push(Credit {
                        seat: d.other(),
                        cause: Cause::Game,
                        points: loses,
                    });
                    GameResult::DeclarerLoss
                }
            }
            (None, None) => match Seat::BOTH
                .into_iter()
                .find(|s| thirds[s.index()] >= THRESHOLD_THIRDS)
            {
                Some(w) => {
                    credits.push(Credit {
                        seat: w,
                        cause: Cause::Game,
                        points: simple,
                    });
                    GameResult::SimpleWin
                }
                None => GameResult::Draw,
            },
        };
        if let Some(v) = valat {
            credits.push(Credit {
                seat: v,
                cause: Cause::Valat,
                points: VALAT_POINTS,
            });
        }
        for honour in HONOURS {
            for seat in Seat::BOTH {
                if self.honour_state(seat, honour) == HonourState::Announced {
                    credits.push(Credit {
                        seat,
                        cause: match honour {
                            Honour::Trull => Cause::Trull,
                            Honour::Kings => Cause::Kings,
                        },
                        points: 1,
                    });
                }
            }
        }
        let ultimo = self.log.last().and_then(|t| {
            let player = if t.cards[0] == Card::PAGAT {
                t.leader
            } else if t.cards[1] == Card::PAGAT {
                t.leader.other()
            } else {
                return None;
            };
            Some(if t.winner == player {
                (player, UltimoResult::Won)
            } else {
                (player, UltimoResult::Lost)
            })
        });
        match ultimo {
            Some((p, UltimoResult::Won)) => credits.push(Credit {
                seat: p,
                cause: Cause::UltimoWon,
                points: self.settings.ultimo,
            }),
            Some((p, UltimoResult::Lost)) => credits.push(Credit {
                seat: p.other(),
                cause: Cause::UltimoLost,
                points: self.settings.ultimo,
            }),
            None => {}
        }
        for credit in &mut credits {
            credit.points *= self.multiplier;
        }
        DealScore {
            result,
            thirds,
            credits,
            valat,
            ultimo,
            multiplier: self.multiplier,
        }
    }

    fn bid(&mut self, seat: Seat, action: Action, events: &mut Vec<Event>) -> Result<(), Refusal> {
        let Phase::Bidding(speaker) = self.phase else {
            return Err(Refusal::of(RefusalKind::WrongPhase));
        };
        if seat != speaker {
            return Err(Refusal::of(RefusalKind::NotYourTurn).with_speaker(speaker));
        }
        match action {
            Action::Take => {
                events.push(Event::Bid { seat });
                self.declarer = Some(seat);
                self.start_play(events);
            }
            _ => {
                events.push(Event::Pass { seat });
                if seat == self.forehand() {
                    self.phase = Phase::Bidding(self.dealer);
                } else {
                    events.push(Event::BothPass {
                        mode: self.settings.both_pass,
                    });
                    match self.settings.both_pass {
                        BothPass::Play => self.start_play(events),
                        BothPass::ThrowIn => {
                            events.push(Event::ThrownIn);
                            self.phase = Phase::Done;
                            self.end = Some(DealEnd::ThrownIn);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// The reveal (RULES 4), then play begins.
    fn start_play(&mut self, events: &mut Vec<Event>) {
        for seat in [self.forehand(), self.dealer] {
            events.push(Event::RevealStart { seat });
            for packet in 1..=3 {
                self.turn_packet(seat, packet, events);
            }
        }
        self.phase = Phase::Playing;
    }

    /// Turns cards of a packet until a cover is set or the packet is empty
    /// (RULES 4.4 to 4.7, 6.1).
    fn turn_packet(&mut self, seat: Seat, packet: u8, events: &mut Vec<Event>) {
        loop {
            let p = &mut self.packets[seat.index()][usize::from(packet) - 1];
            let Some(card) = p.rest.pop_front() else {
                return;
            };
            events.push(Event::PacketTurned { seat, packet, card });
            let last = p.rest.is_empty();
            if last || card.is_tarock() || card.is_king() {
                self.hands[seat.index()].push(HandCard { card, public: true });
                events.push(Event::CardPulled {
                    seat,
                    packet,
                    card,
                    last,
                });
                if last {
                    events.push(Event::PacketEmpty { seat, packet });
                    return;
                }
            } else {
                p.cover = Some(card);
                events.push(Event::CoverSet { seat, packet, card });
                return;
            }
        }
    }
}

/// Whether the card led beats the card followed (RULES 7.1): the highest
/// trump wins, else the highest card of the suit led; other suits never win.
fn trick_won_by_leader(led: Card, followed: Card) -> bool {
    match (led.is_tarock(), followed.is_tarock()) {
        (true, true) => led.strength() > followed.strength(),
        (true, false) => true,
        (false, true) => false,
        (false, false) => led.suit() != followed.suit() || led.strength() > followed.strength(),
    }
}

#[cfg(test)]
mod play_tests;

#[cfg(test)]
mod score_tests;

#[cfg(test)]
mod tests {
    use super::*;
    pub(crate) fn c(code: &str) -> Card {
        Card::parse(code).unwrap_or_else(|| panic!("bad code {code}"))
    }

    /// A deck with the given codes at the given positions and every other
    /// card after them in canonical order.
    pub(crate) fn deck_from(slots: &[(usize, &str)]) -> Vec<Card> {
        let mut deck: Vec<Option<Card>> = vec![None; 54];
        for &(i, code) in slots {
            assert!(deck[i].is_none(), "slot {i} twice");
            deck[i] = Some(c(code));
        }
        let used: Vec<Card> = deck.iter().flatten().copied().collect();
        let mut rest = Card::all().filter(|x| !used.contains(x));
        deck.into_iter()
            .map(|s| s.unwrap_or_else(|| rest.next().unwrap()))
            .collect()
    }

    /// The canonical order cut so that every hand has tarocks: forehand and
    /// dealer hands get tarocks in their first slot.
    fn plain_deck() -> Vec<Card> {
        // forehand T1 at 0, dealer T2 at 5; the rest in canonical order
        deck_from(&[(0, "T10"), (5, "T11")])
    }

    fn new_deal(deck: &[Card]) -> Deal {
        Deal::new(deck, Seat::B, 1, Settings::default())
    }

    #[test]
    fn rules_2_3_hands_are_dealt_five_at_a_time_forehand_first() {
        let deck: Vec<Card> = Card::all().collect();
        let deal = Deal::new(&deck, Seat::B, 1, Settings::default());
        let fh = deal.forehand();
        assert_eq!(fh, Seat::A);
        let mut want: Vec<Card> = Vec::new();
        for r in [0..5, 10..15, 20..25] {
            want.extend(deck[r].iter());
        }
        assert_eq!(deal.hand_cards(fh), want);
        let mut want: Vec<Card> = Vec::new();
        for r in [5..10, 15..20, 25..30] {
            want.extend(deck[r].iter());
        }
        assert_eq!(deal.hand_cards(Seat::B), want);
        assert_eq!(deal.hand_cards(Seat::A).len(), 15);
    }

    #[test]
    fn rules_2_4_packets_are_dealt_forehand_first_four_cards_top_first() {
        let deck: Vec<Card> = Card::all().collect();
        let deal = Deal::new(&deck, Seat::B, 1, Settings::default());
        // packet i = deck[30+4i ..]; 0,2,4 to forehand (A), 1,3,5 to dealer (B)
        assert_eq!(deal.packet_cards(Seat::A, 1), deck[30..34].to_vec());
        assert_eq!(deal.packet_cards(Seat::B, 1), deck[34..38].to_vec());
        assert_eq!(deal.packet_cards(Seat::A, 2), deck[38..42].to_vec());
        assert_eq!(deal.packet_cards(Seat::B, 2), deck[42..46].to_vec());
        assert_eq!(deal.packet_cards(Seat::A, 3), deck[46..50].to_vec());
        assert_eq!(deal.packet_cards(Seat::B, 3), deck[50..54].to_vec());
    }

    #[test]
    fn rules_2_1_forehand_is_the_non_dealer() {
        let deck = plain_deck();
        let d = Deal::new(&deck, Seat::A, 1, Settings::default());
        assert_eq!(d.forehand(), Seat::B);
        assert_eq!(d.dealer(), Seat::A);
    }

    #[test]
    fn rules_2_6_a_hand_without_tarock_is_void_and_the_skus_counts_as_one() {
        // canonical order puts T1..SK first, so the first 30 cards hold tarocks
        let deck: Vec<Card> = Card::all().collect();
        assert!(new_deal(&deck).void_seats().is_empty());
        // forehand's 15 cards all suit cards: put 15 suit cards in forehand slots
        let fh_slots = [0, 1, 2, 3, 4, 10, 11, 12, 13, 14, 20, 21, 22, 23, 24];
        let suit_codes = [
            "Kh", "Qh", "Ch", "Jh", "1h", "2h", "3h", "4h", "Kd", "Qd", "Cd", "Jd", "1d", "2d",
            "3d",
        ];
        let slots: Vec<(usize, &str)> = fh_slots.iter().copied().zip(suit_codes).collect();
        let deck = deck_from(&slots);
        assert_eq!(new_deal(&deck).void_seats(), vec![Seat::A]);
        // one Sküs among them rescues the hand
        let mut slots = slots.clone();
        slots[14] = (24, "SK");
        let deck = deck_from(&slots);
        assert!(new_deal(&deck).void_seats().is_empty());
        // a tarock hidden in a packet does not count (RULES 2.6)
        let mut slots: Vec<(usize, &str)> = fh_slots.iter().copied().zip(suit_codes).collect();
        slots.push((30, "T5"));
        let deck = deck_from(&slots);
        assert_eq!(new_deal(&deck).void_seats(), vec![Seat::A]);
    }

    #[test]
    fn rules_3_1_forehand_speaks_first_and_the_dealer_may_not() {
        let mut deal = new_deal(&plain_deck());
        let mut ev = Vec::new();
        assert_eq!(deal.to_move(), Some(Seat::A));
        let err = deal.apply(Seat::B, Action::Take, &mut ev).unwrap_err();
        assert_eq!(err.kind, RefusalKind::NotYourTurn);
        assert_eq!(err.speaker, Some(Seat::A));
        assert!(ev.is_empty());
        assert_eq!(deal.legal_actions(), vec![Action::Take, Action::Pass]);
    }

    #[test]
    fn rules_3_2_and_3_3_a_take_makes_a_declarer() {
        let mut deal = new_deal(&plain_deck());
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::Pass, &mut ev).unwrap();
        assert_eq!(deal.to_move(), Some(Seat::B));
        deal.apply(Seat::B, Action::Take, &mut ev).unwrap();
        assert_eq!(deal.declarer(), Some(Seat::B));
        assert_eq!(ev[0], Event::Pass { seat: Seat::A });
        assert_eq!(ev[1], Event::Bid { seat: Seat::B });
        // the reveal follows at once (4.1), then the play: forehand leads
        assert_eq!(deal.to_move(), Some(Seat::A));
    }

    #[test]
    fn rules_3_3_forehand_takes_and_the_dealer_is_not_asked() {
        let mut deal = new_deal(&plain_deck());
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::Take, &mut ev).unwrap();
        assert_eq!(deal.declarer(), Some(Seat::A));
        assert_eq!(ev[0], Event::Bid { seat: Seat::A });
        assert!(matches!(ev[1], Event::RevealStart { seat: Seat::A }));
    }

    #[test]
    fn rules_3_4_both_pass_play_is_a_simple_game() {
        let mut deal = new_deal(&plain_deck());
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::Pass, &mut ev).unwrap();
        deal.apply(Seat::B, Action::Pass, &mut ev).unwrap();
        assert_eq!(deal.declarer(), None);
        assert!(ev.contains(&Event::BothPass {
            mode: BothPass::Play
        }));
        assert!(deal.is_playing());
    }

    #[test]
    fn rules_3_4_both_pass_throw_in_ends_the_deal_unscored_and_unrevealed() {
        let settings = Settings {
            both_pass: BothPass::ThrowIn,
            ..Settings::default()
        };
        let mut deal = Deal::new(&plain_deck(), Seat::B, 1, settings);
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::Pass, &mut ev).unwrap();
        deal.apply(Seat::B, Action::Pass, &mut ev).unwrap();
        assert!(ev.contains(&Event::BothPass {
            mode: BothPass::ThrowIn
        }));
        assert_eq!(ev.last(), Some(&Event::ThrownIn));
        assert!(!ev.iter().any(|e| matches!(e, Event::PacketTurned { .. })));
        assert_eq!(deal.end(), Some(&DealEnd::ThrownIn));
        assert_eq!(deal.to_move(), None);
        let err = deal.apply(Seat::A, Action::Take, &mut ev).unwrap_err();
        assert_eq!(err.kind, RefusalKind::Finished);
    }

    #[test]
    fn rules_3_3_each_player_speaks_at_most_once_and_play_actions_are_premature() {
        let mut deal = new_deal(&plain_deck());
        let mut ev = Vec::new();
        let err = deal
            .apply(Seat::A, Action::play(c("T10")), &mut ev)
            .unwrap_err();
        assert_eq!(err.kind, RefusalKind::WrongPhase);
    }

    /// A deck where forehand's packet 1 is `p`, dealer's is the canonical rest.
    fn deck_with_forehand_packet(p: [&str; 4]) -> Vec<Card> {
        let slots: Vec<(usize, &str)> = vec![
            (0, "T10"),
            (5, "T11"),
            (30, p[0]),
            (31, p[1]),
            (32, p[2]),
            (33, p[3]),
        ];
        deck_from(&slots)
    }

    fn reveal_events(deck: &[Card]) -> (Deal, Vec<Event>) {
        let mut deal = Deal::new(deck, Seat::B, 1, Settings::default());
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::Take, &mut ev).unwrap();
        (deal, ev)
    }

    #[test]
    fn rules_4_5_and_4_6_a_tarock_or_king_is_pulled_and_the_chain_goes_on() {
        let deck = deck_with_forehand_packet(["T5", "Kh", "7c", "8c"]);
        let (deal, ev) = reveal_events(&deck);
        let own: Vec<&Event> = ev
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::PacketTurned {
                        seat: Seat::A,
                        packet: 1,
                        ..
                    } | Event::CardPulled {
                        seat: Seat::A,
                        packet: 1,
                        ..
                    } | Event::CoverSet {
                        seat: Seat::A,
                        packet: 1,
                        ..
                    } | Event::PacketEmpty {
                        seat: Seat::A,
                        packet: 1
                    }
                )
            })
            .collect();
        let want = [
            Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("T5"),
            },
            Event::CardPulled {
                seat: Seat::A,
                packet: 1,
                card: c("T5"),
                last: false,
            },
            Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("Kh"),
            },
            Event::CardPulled {
                seat: Seat::A,
                packet: 1,
                card: c("Kh"),
                last: false,
            },
            Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("7c"),
            },
            Event::CoverSet {
                seat: Seat::A,
                packet: 1,
                card: c("7c"),
            },
        ];
        assert_eq!(own, want.iter().collect::<Vec<_>>());
        assert_eq!(deal.cover(Seat::A, 1), Some(c("7c")));
        assert_eq!(deal.hidden_count(Seat::A, 1), 1);
        // pulled cards are in the hand, flagged public (13.4)
        let hand = deal.hand(Seat::A);
        assert!(hand.len() >= 17);
        assert!(hand.iter().any(|h| h.card == c("T5") && h.public));
        assert!(hand.iter().any(|h| h.card == c("Kh") && h.public));
        assert!(hand.iter().filter(|h| !h.public).count() >= 15);
    }

    #[test]
    fn rules_4_5_a_low_suit_card_stays_as_cover_and_ends_the_chain() {
        let deck = deck_with_forehand_packet(["Qc", "Kh", "T3", "T4"]);
        let (deal, _) = reveal_events(&deck);
        assert_eq!(deal.cover(Seat::A, 1), Some(c("Qc")));
        assert_eq!(deal.hidden_count(Seat::A, 1), 3);
    }

    #[test]
    fn rules_4_7_the_last_card_is_taken_whatever_it_is_and_the_packet_is_empty() {
        let deck = deck_with_forehand_packet(["T5", "Kh", "T7", "Jh"]);
        let (deal, ev) = reveal_events(&deck);
        let tail: Vec<&Event> = ev
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    Event::CardPulled {
                        seat: Seat::A,
                        packet: 1,
                        last: true,
                        ..
                    } | Event::PacketEmpty {
                        seat: Seat::A,
                        packet: 1
                    }
                )
            })
            .collect();
        assert_eq!(
            tail,
            vec![
                &Event::CardPulled {
                    seat: Seat::A,
                    packet: 1,
                    card: c("Jh"),
                    last: true
                },
                &Event::PacketEmpty {
                    seat: Seat::A,
                    packet: 1
                },
            ]
        );
        assert_eq!(deal.cover(Seat::A, 1), None);
        assert_eq!(deal.hidden_count(Seat::A, 1), 0);
    }

    #[test]
    fn rules_4_5_a_third_card_cover_leaves_the_fourth_hidden_to_be_turned_later() {
        let deck = deck_with_forehand_packet(["T5", "Kh", "Jd", "T9"]);
        let (deal, _) = reveal_events(&deck);
        assert_eq!(deal.cover(Seat::A, 1), Some(c("Jd")));
        assert_eq!(deal.hidden_count(Seat::A, 1), 1);
    }

    #[test]
    fn rules_4_2_and_4_3_forehand_reveals_first_then_dealer_packets_in_order() {
        let (_, ev) = reveal_events(&plain_deck());
        let starts: Vec<usize> = ev
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e, Event::RevealStart { .. }))
            .map(|(i, _)| i)
            .collect();
        assert_eq!(starts.len(), 2);
        assert_eq!(ev[starts[0]], Event::RevealStart { seat: Seat::A });
        assert_eq!(ev[starts[1]], Event::RevealStart { seat: Seat::B });
        // within one seat packets 1, 2, 3 in turn: no packet number goes back
        let mut last = (Seat::A, 0u8);
        for e in &ev {
            if let Event::PacketTurned { seat, packet, .. } = e {
                assert!(
                    (seat.index(), *packet) >= (last.0.index(), last.1),
                    "reveal order broke at {e:?}"
                );
                last = (*seat, *packet);
            }
        }
    }

    #[test]
    fn rules_4_4_every_non_empty_packet_has_a_low_cover_after_the_reveal() {
        for seed in 0..50u64 {
            let mut deck: Vec<Card> = Card::all().collect();
            crate::rng::Rng::seeded(seed).shuffle(&mut deck);
            let mut deal = Deal::new(&deck, Seat::B, 1, Settings::default());
            let mut ev = Vec::new();
            deal.apply(Seat::A, Action::Take, &mut ev).unwrap();
            for s in Seat::BOTH {
                for p in 1..=3 {
                    match deal.cover(s, p) {
                        Some(cv) => {
                            assert!(!cv.is_tarock() && !cv.is_king(), "rules 4.8: {cv:?}");
                            assert!(deal.hidden_count(s, p) <= 3);
                        }
                        None => assert_eq!(deal.hidden_count(s, p), 0),
                    }
                }
            }
        }
    }
}
