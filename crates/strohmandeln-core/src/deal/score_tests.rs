//! Tests of counting, bonuses, announcements and game values (RULES 8 to 12, 14.2).

use super::tests::c;
use super::*;

type PacketSpec<'a> = (Option<&'a str>, &'a [&'a str]);
const EMPTY: PacketSpec = (None, &[]);

fn arranged(a_hand: &[&str], b_hand: &[&str]) -> Deal {
    Deal::arranged(
        Seat::B,
        [a_hand, b_hand],
        [[EMPTY; 3], [EMPTY; 3]],
        Settings::default(),
    )
}

/// Gives seat A takes worth exactly `a_thirds` and B the rest of the pack.
fn give_thirds(deal: &mut Deal, a_thirds: u32) {
    let mut cards: Vec<Card> = Card::all().collect();
    cards.sort_by_key(|c| std::cmp::Reverse(c.thirds()));
    let mut left = a_thirds;
    deal.takes = [Vec::new(), Vec::new()];
    for card in cards {
        if card.thirds() <= left {
            left -= card.thirds();
            deal.takes[0].push(card);
        } else {
            deal.takes[1].push(card);
        }
    }
    assert_eq!(left, 0);
}

fn fake_log(
    deal: &mut Deal,
    tricks_a: usize,
    last: [Card; 2],
    last_leader: Seat,
    last_winner: Seat,
) {
    deal.log = (0..26)
        .map(|i| TrickRecord {
            leader: Seat::A,
            cards: [c("4h"), c("4d")],
            sources: [Source::Hand; 2],
            winner: if i < tricks_a { Seat::A } else { Seat::B },
        })
        .collect();
    deal.log.push(TrickRecord {
        leader: last_leader,
        cards: last,
        sources: [Source::Hand; 2],
        winner: last_winner,
    });
}

fn scored(settings: Settings, mult: u32, declarer: Option<Seat>, a_thirds: u32) -> DealScore {
    let mut deal = arranged(&[], &[]);
    deal.settings = settings;
    deal.multiplier = mult;
    deal.declarer = declarer;
    give_thirds(&mut deal, a_thirds);
    // an even split of tricks, no Pagat in the last trick
    fake_log(&mut deal, 13, [c("4h"), c("4d")], Seat::A, Seat::A);
    deal.score()
}

fn game_credit(score: &DealScore) -> Vec<(Seat, u32)> {
    score
        .credits
        .iter()
        .filter(|cr| cr.cause == Cause::Game)
        .map(|cr| (cr.seat, cr.points))
        .collect()
}

fn s(values: Values) -> Settings {
    Settings {
        values,
        ..Settings::default()
    }
}

#[test]
fn rules_8_1_the_helper_deals_the_whole_pack_in_exact_thirds() {
    let mut deal = arranged(&[], &[]);
    give_thirds(&mut deal, 107);
    let a: u32 = deal.takes[0].iter().map(|c| c.thirds()).sum();
    let b: u32 = deal.takes[1].iter().map(|c| c.thirds()).sum();
    assert_eq!((a, b), (107, 103));
}

#[test]
fn rules_8_2_and_8_3_the_declarer_needs_107_thirds_and_35_1_3_loses() {
    let st = Settings::default();
    let win = scored(st, 1, Some(Seat::A), 107);
    assert_eq!(win.result, GameResult::DeclarerWin);
    let lose = scored(st, 1, Some(Seat::A), 106);
    assert_eq!(lose.result, GameResult::DeclarerLoss);
    // no draw in a declared game
    let even = scored(st, 1, Some(Seat::A), 105);
    assert_eq!(even.result, GameResult::DeclarerLoss);
    assert_eq!(even.thirds, [105, 105]);
}

#[test]
fn rules_8_4_simple_game_with_107_wins_and_no_107_is_a_draw() {
    let st = Settings::default();
    assert_eq!(scored(st, 1, None, 107).result, GameResult::SimpleWin);
    assert_eq!(scored(st, 1, None, 103).result, GameResult::SimpleWin);
    assert_eq!(scored(st, 1, None, 106).result, GameResult::Draw);
    assert_eq!(scored(st, 1, None, 105).result, GameResult::Draw);
    assert_eq!(scored(st, 1, None, 104).result, GameResult::Draw);
}

#[test]
fn rules_8_1_the_two_takes_always_sum_to_210() {
    for a in [0, 1, 105, 107, 210] {
        let sc = scored(Settings::default(), 1, None, a);
        assert_eq!(sc.thirds[0] + sc.thirds[1], 210);
        assert_eq!(sc.thirds[0], a);
    }
}

#[test]
fn rules_9_1_and_9_2_values_for_each_setting() {
    // (setting, simple, wins, loses)
    for (v, simple, wins, loses) in [
        (Values::V234, 2, 3, 4),
        (Values::V123, 1, 2, 3),
        (Values::V124, 1, 2, 4),
    ] {
        // declarer A wins: defender pays A the "wins" value
        assert_eq!(
            game_credit(&scored(s(v), 1, Some(Seat::A), 120)),
            vec![(Seat::A, wins)]
        );
        // declarer A loses: A pays B the "loses" value
        assert_eq!(
            game_credit(&scored(s(v), 1, Some(Seat::A), 90)),
            vec![(Seat::B, loses)]
        );
        // declarer B loses
        assert_eq!(
            game_credit(&scored(s(v), 1, Some(Seat::B), 120)),
            vec![(Seat::A, loses)]
        );
        // simple game won by B
        assert_eq!(
            game_credit(&scored(s(v), 1, None, 90)),
            vec![(Seat::B, simple)]
        );
    }
}

#[test]
fn rules_9_2_a_draw_pays_no_game_value() {
    let sc = scored(Settings::default(), 1, None, 105);
    assert!(game_credit(&sc).is_empty());
    assert!(sc.credits.is_empty());
}

#[test]
fn rules_14_2_a_doubling_doubles_every_payment() {
    let mut deal = arranged(&[], &[]);
    deal.multiplier = 2;
    deal.declarer = Some(Seat::A);
    deal.honours[0][0] = HonourState::Announced;
    give_thirds(&mut deal, 120);
    fake_log(&mut deal, 13, [c("T1"), c("4d")], Seat::A, Seat::A);
    let sc = deal.score();
    let by: Vec<(Seat, Cause, u32)> = sc
        .credits
        .iter()
        .map(|c| (c.seat, c.cause, c.points))
        .collect();
    assert_eq!(
        by,
        vec![
            (Seat::A, Cause::Game, 6),
            (Seat::A, Cause::Trull, 2),
            (Seat::A, Cause::UltimoWon, 2),
        ]
    );
}

#[test]
fn rules_10_1_and_10_2_valat_is_12_and_replaces_the_game_value_only() {
    let mut deal = arranged(&[], &[]);
    deal.declarer = Some(Seat::A);
    give_thirds(&mut deal, 210);
    fake_log(&mut deal, 27, [c("4h"), c("4d")], Seat::A, Seat::A);
    deal.honours[0][1] = HonourState::Announced;
    let sc = deal.score();
    assert_eq!(sc.valat, Some(Seat::A));
    assert_eq!(sc.result, GameResult::DeclarerWin);
    let by: Vec<(Seat, Cause, u32)> = sc
        .credits
        .iter()
        .map(|c| (c.seat, c.cause, c.points))
        .collect();
    assert_eq!(
        by,
        vec![(Seat::A, Cause::Valat, 12), (Seat::A, Cause::Kings, 1)]
    );
}

#[test]
fn rules_10_1_a_defender_valat_and_a_simple_game_valat() {
    let mut deal = arranged(&[], &[]);
    deal.declarer = Some(Seat::A);
    give_thirds(&mut deal, 0);
    fake_log(&mut deal, 0, [c("4h"), c("4d")], Seat::A, Seat::B);
    let sc = deal.score();
    assert_eq!(sc.valat, Some(Seat::B));
    assert_eq!(sc.result, GameResult::DeclarerLoss);
    assert_eq!(game_credit(&sc), vec![]);
    assert_eq!(sc.credits[0].points, 12);
    assert_eq!(sc.credits[0].seat, Seat::B);
    deal.declarer = None;
    let sc = deal.score();
    assert_eq!(sc.result, GameResult::SimpleWin);
}

#[test]
fn rules_10_1_26_tricks_are_not_a_valat() {
    let mut deal = arranged(&[], &[]);
    give_thirds(&mut deal, 200);
    fake_log(&mut deal, 26, [c("4h"), c("4d")], Seat::A, Seat::B);
    assert_eq!(deal.score().valat, None);
}

#[test]
fn rules_11_1_and_11_7_announced_honours_are_paid_even_when_lost_or_drawn() {
    let mut deal = arranged(&[], &[]);
    deal.declarer = None;
    deal.honours[1][0] = HonourState::Announced; // B announced the trull
    deal.honours[0][1] = HonourState::Announced; // A announced the kings
    give_thirds(&mut deal, 105);
    fake_log(&mut deal, 13, [c("4h"), c("4d")], Seat::A, Seat::A);
    let sc = deal.score();
    assert_eq!(sc.result, GameResult::Draw);
    let by: Vec<(Seat, Cause, u32)> = sc
        .credits
        .iter()
        .map(|c| (c.seat, c.cause, c.points))
        .collect();
    assert_eq!(
        by,
        vec![(Seat::B, Cause::Trull, 1), (Seat::A, Cause::Kings, 1)]
    );
}

#[test]
fn rules_11_5_unannounced_and_forfeited_honours_pay_nothing() {
    let mut deal = arranged(&[], &[]);
    deal.honours[0][0] = HonourState::Forfeited;
    give_thirds(&mut deal, 105);
    fake_log(&mut deal, 13, [c("4h"), c("4d")], Seat::A, Seat::A);
    assert!(deal.score().credits.is_empty());
}

#[test]
fn rules_12_2_pagat_ultimo_won() {
    // A leads T1 in trick 27, B has no tarock: A wins with the Pagat
    let mut deal = arranged(&["T1"], &["4d"]);
    deal.trick = 27;
    deal.declarer = None;
    give_thirds(&mut deal, 100);
    deal.log = fake_26(13);
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("T1")), &mut ev).unwrap();
    deal.apply(Seat::B, Action::play(c("4d")), &mut ev).unwrap();
    assert!(ev.contains(&Event::PagatUltimo {
        seat: Seat::A,
        result: UltimoResult::Won
    }));
    let Some(DealEnd::Played(sc)) = deal.end() else {
        panic!("deal should be over")
    };
    assert!(sc.credits.contains(&Credit {
        seat: Seat::A,
        cause: Cause::UltimoWon,
        points: 1
    }));
}

fn fake_26(tricks_a: usize) -> Vec<TrickRecord> {
    (0..26)
        .map(|i| TrickRecord {
            leader: Seat::A,
            cards: [c("4h"), c("4d")],
            sources: [Source::Hand; 2],
            winner: if i < tricks_a { Seat::A } else { Seat::B },
        })
        .collect()
}

#[test]
fn rules_12_2_pagat_ultimo_lost_pays_the_opponent_and_the_setting_sets_the_value() {
    for ultimo in [1, 2] {
        let mut deal = arranged(&["T1"], &["T2"]);
        deal.settings.ultimo = ultimo;
        deal.trick = 27;
        give_thirds(&mut deal, 100);
        deal.log = fake_26(13);
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::play(c("T1")), &mut ev).unwrap();
        deal.apply(Seat::B, Action::play(c("T2")), &mut ev).unwrap();
        assert!(ev.contains(&Event::PagatUltimo {
            seat: Seat::A,
            result: UltimoResult::Lost
        }));
        let Some(DealEnd::Played(sc)) = deal.end() else {
            panic!("deal should be over")
        };
        assert!(sc.credits.contains(&Credit {
            seat: Seat::B,
            cause: Cause::UltimoLost,
            points: ultimo
        }));
    }
}

#[test]
fn rules_12_2_the_pagat_in_an_earlier_trick_is_no_ultimo() {
    let mut deal = arranged(&["T1", "4h"], &["T2", "4d"]);
    deal.trick = 26;
    give_thirds(&mut deal, 100);
    deal.log = fake_26(13)[..25].to_vec();
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("T1")), &mut ev).unwrap();
    deal.apply(Seat::B, Action::play(c("T2")), &mut ev).unwrap();
    deal.apply(Seat::B, Action::play(c("4d")), &mut ev).unwrap();
    deal.apply(Seat::A, Action::play(c("4h")), &mut ev).unwrap();
    assert!(!ev.iter().any(|e| matches!(e, Event::PagatUltimo { .. })));
}

#[test]
fn rules_12_2_trick_27_ends_the_deal_with_count_and_valat_events_in_order() {
    let mut deal = arranged(&["4h"], &["4d"]);
    deal.trick = 27;
    deal.declarer = Some(Seat::A);
    give_thirds(&mut deal, 100);
    deal.log = fake_26(0); // B has won everything so far
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("4h")), &mut ev).unwrap();
    deal.apply(Seat::B, Action::play(c("4d")), &mut ev).unwrap();
    // 4d is off suit (hearts led), so A wins trick 27; no Valat for B
    assert!(!ev.iter().any(|e| matches!(e, Event::Valat { .. })));
    let last = ev.last().unwrap();
    assert!(
        matches!(last, Event::Count { threshold: 107, .. }),
        "{last:?}"
    );
    assert_eq!(deal.to_move(), None);
}

// ----- announcements (RULES 11.3 to 11.6) -----

fn announce(deal: &mut Deal, seat: Seat, h: Honour) -> Result<Vec<Event>, Refusal> {
    let mut ev = Vec::new();
    deal.apply(seat, Action::Announce { honour: h }, &mut ev)
        .map(|_| ev)
}

#[test]
fn rules_11_2_and_11_3_an_honour_can_be_announced_when_held_on_ones_turn() {
    let mut deal = arranged(&["SK", "T21", "T1", "7c"], &["Kh", "Kd", "Kc"]);
    assert_eq!(deal.legal_announcements(Seat::A), vec![Honour::Trull]);
    assert!(deal.legal_actions().contains(&Action::Announce {
        honour: Honour::Trull
    }));
    let ev = announce(&mut deal, Seat::A, Honour::Trull).unwrap();
    assert_eq!(
        ev,
        vec![Event::Announcement {
            seat: Seat::A,
            honour: Honour::Trull,
            cards: vec![c("SK"), c("T21"), c("T1")]
        }]
    );
    // the announcement is public and only once
    assert_eq!(deal.legal_announcements(Seat::A), vec![]);
    let err = announce(&mut deal, Seat::A, Honour::Trull).unwrap_err();
    assert_eq!(err.kind, RefusalKind::AlreadyAnnounced);
}

#[test]
fn rules_11_4_a_false_announcement_is_refused_with_a_reason() {
    let mut deal = arranged(&["SK", "T21", "7c"], &["Kh", "Kd", "Kc", "Ks"]);
    let err = announce(&mut deal, Seat::A, Honour::Trull).unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotHeld);
    assert_eq!(err.missing, vec![c("T1")]);
    let err = announce(&mut deal, Seat::A, Honour::Kings).unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotHeld);
    // the opponent holds the kings but it is not their turn
    let err = announce(&mut deal, Seat::B, Honour::Kings).unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotYourTurn);
}

#[test]
fn rules_11_2_cards_pulled_from_a_packet_count_toward_the_honour() {
    let mut deal = Deal::arranged(
        Seat::B,
        [&["SK", "T21", "7c"], &["8c"]],
        [
            [(Some("Jh"), &["T1", "9c", "8s"]), EMPTY, EMPTY],
            [EMPTY; 3],
        ],
        Settings::default(),
    );
    assert_eq!(deal.legal_announcements(Seat::A), vec![]);
    // A plays the cover, the replacement pulls T1 into the hand
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("Jh")), &mut ev).unwrap();
    deal.apply(Seat::B, Action::play(c("8c")), &mut ev).unwrap();
    assert_eq!(deal.to_move(), Some(Seat::A));
    assert_eq!(deal.legal_announcements(Seat::A), vec![Honour::Trull]);
}

#[test]
fn rules_11_3_the_announcement_may_ride_with_the_play_and_comes_first() {
    let mut deal = arranged(&["SK", "T21", "T1"], &["Kh"]);
    let mut ev = Vec::new();
    deal.apply(
        Seat::A,
        Action::Play {
            card: c("T21"),
            announce: vec![Honour::Trull],
        },
        &mut ev,
    )
    .unwrap();
    assert!(matches!(
        ev[0],
        Event::Announcement {
            honour: Honour::Trull,
            ..
        }
    ));
    assert!(matches!(ev[1], Event::CardPlayed { .. }));
    assert!(!ev
        .iter()
        .any(|e| matches!(e, Event::HonourForfeited { .. })));
}

#[test]
fn rules_11_4_a_refused_play_with_announcement_changes_nothing() {
    let mut deal = arranged(&["T21", "7c"], &["Kh", "Kd", "Kc", "Ks", "8c"]);
    let mut ev = Vec::new();
    // announcing the trull A does not hold, with a legal card
    let err = deal
        .apply(
            Seat::A,
            Action::Play {
                card: c("7c"),
                announce: vec![Honour::Trull],
            },
            &mut ev,
        )
        .unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotHeld);
    assert!(ev.is_empty());
    assert_eq!(deal.hand(Seat::A).len(), 2);
}

#[test]
fn rules_11_5_playing_a_card_of_a_held_unannounced_honour_forfeits_it() {
    let mut deal = arranged(&["SK", "T21", "T1"], &["Kh"]);
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("T21")), &mut ev)
        .unwrap();
    assert!(ev.contains(&Event::HonourForfeited {
        seat: Seat::A,
        honour: Honour::Trull
    }));
    // too late to announce, whatever the cards left
    assert_eq!(deal.legal_announcements(Seat::A), vec![]);
    deal.apply(Seat::B, Action::play(c("Kh")), &mut ev).unwrap();
}

#[test]
fn rules_11_5_following_or_trumping_with_an_honour_card_also_forfeits() {
    let mut deal = arranged(&["Kc", "7c"], &["Kh", "Kd", "Ks", "SK", "T21", "T1", "9c"]);
    // A leads 7c? B must follow clubs with 9c; use hearts lead instead
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("7c")), &mut ev).unwrap();
    // B holds clubs 9c: forced. Play it: no honour card, no forfeit
    deal.apply(Seat::B, Action::play(c("9c")), &mut ev).unwrap();
    assert!(!ev
        .iter()
        .any(|e| matches!(e, Event::HonourForfeited { .. })));
    // next trick: A leads Kc (A won? 9c < 7c? 9c is above 7c: B won) -> B leads
    assert_eq!(deal.to_move(), Some(Seat::B));
    // B leads Kh (king, kings not held: B lacks Kc): no forfeit; trull held
    // and unannounced: leading T1 forfeits it
    let mut ev = Vec::new();
    deal.apply(Seat::B, Action::play(c("T1")), &mut ev).unwrap();
    assert!(ev.contains(&Event::HonourForfeited {
        seat: Seat::B,
        honour: Honour::Trull
    }));
}

#[test]
fn rules_11_5_an_announced_honour_is_not_forfeited_by_playing_it() {
    let mut deal = arranged(&["SK", "T21", "T1"], &["Kh"]);
    announce(&mut deal, Seat::A, Honour::Trull).unwrap();
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("T1")), &mut ev).unwrap();
    assert!(!ev
        .iter()
        .any(|e| matches!(e, Event::HonourForfeited { .. })));
}

#[test]
fn rules_11_5_a_card_of_an_honour_not_held_forfeits_nothing() {
    let mut deal = arranged(&["SK", "T21", "7c"], &["Kh"]);
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("SK")), &mut ev).unwrap();
    assert!(!ev
        .iter()
        .any(|e| matches!(e, Event::HonourForfeited { .. })));
}

#[test]
fn rules_11_3_the_four_kings_may_be_announced_by_either_player() {
    let mut deal = arranged(&["7c"], &["Kh", "Kd", "Kc", "Ks"]);
    let mut ev = Vec::new();
    deal.apply(Seat::A, Action::play(c("7c")), &mut ev).unwrap();
    // B follows after A: it is B's turn, B holds the kings
    assert_eq!(deal.legal_announcements(Seat::B), vec![Honour::Kings]);
    let ev = announce(&mut deal, Seat::B, Honour::Kings).unwrap();
    assert!(matches!(
        ev[0],
        Event::Announcement {
            seat: Seat::B,
            honour: Honour::Kings,
            ..
        }
    ));
}

#[test]
fn rules_11_1_honours_are_independent_of_each_other() {
    let mut deal = arranged(&["SK", "T21", "T1", "Kh", "Kd", "Kc", "Ks"], &["7c"]);
    assert_eq!(
        deal.legal_announcements(Seat::A),
        vec![Honour::Trull, Honour::Kings]
    );
    announce(&mut deal, Seat::A, Honour::Kings).unwrap();
    assert_eq!(deal.legal_announcements(Seat::A), vec![Honour::Trull]);
}

/// Seat A holds the 22 tarocks, the four kings and `Qh`; B holds the other
/// 27 suit cards. A leads tarocks, then the kings, `Qh`, and `T1` last.
fn valat_deal(declarer: Option<Seat>) -> Deal {
    let a_cards: Vec<String> = ["SK"]
        .into_iter()
        .map(String::from)
        .chain((1..=21).map(|n| format!("T{n}")))
        .chain(["Kh", "Kd", "Kc", "Ks", "Qh"].map(String::from))
        .collect();
    let b_cards: Vec<String> = Card::all()
        .map(|c| c.code())
        .filter(|code| !a_cards.contains(code))
        .collect();
    assert_eq!(b_cards.len(), 27);
    let a: Vec<&str> = a_cards.iter().map(String::as_str).collect();
    let b: Vec<&str> = b_cards.iter().map(String::as_str).collect();
    let mut deal = arranged(&a, &b);
    deal.declarer = declarer;
    // A leads in this order; B answers with its first legal card
    let order: Vec<String> = ["SK"]
        .into_iter()
        .map(String::from)
        .chain((2..=21).rev().map(|n| format!("T{n}")))
        .chain(["Kh", "Kd", "Kc", "Ks", "Qh", "T1"].map(String::from))
        .collect();
    assert_eq!(order.len(), 27);
    for code in order {
        let mut ev = Vec::new();
        deal.apply(Seat::A, Action::play(c(&code)), &mut ev)
            .unwrap();
        let reply = deal.legal_plays(Seat::B)[0].0;
        deal.apply(Seat::B, Action::play(reply), &mut ev).unwrap();
    }
    deal
}

#[test]
fn rules_10_1_a_valat_played_out_pays_12_and_the_ultimo_and_no_game_value() {
    for (declarer, result) in [
        (Some(Seat::A), GameResult::DeclarerWin),
        (Some(Seat::B), GameResult::DeclarerLoss),
        (None, GameResult::SimpleWin),
    ] {
        let deal = valat_deal(declarer);
        let Some(DealEnd::Played(sc)) = deal.end() else {
            panic!("deal over")
        };
        assert_eq!(sc.valat, Some(Seat::A));
        assert_eq!(sc.result, result);
        let mut got: Vec<(Seat, Cause, u32)> = sc
            .credits
            .iter()
            .map(|cr| (cr.seat, cr.cause, cr.points))
            .collect();
        got.sort_by_key(|(_, cause, _)| format!("{cause:?}"));
        // 12 for the Valat, 1 for the Pagat ultimo: the game value is replaced
        assert_eq!(
            got,
            vec![(Seat::A, Cause::UltimoWon, 1), (Seat::A, Cause::Valat, 12)]
        );
        assert_eq!(sc.thirds, [210, 0]);
    }
}

/// The worked deal of the table's "Scoring in full" page (web3d/tutorial*.md,
/// TABLE3D 6.2.4): the declarer's take counts 109 thirds (36 1/3), the Trull
/// was announced, and the declarer's Pagat loses the last trick. The page
/// prints these credits for each `values` and `ultimo` setting.
#[test]
fn the_pages_worked_deal_pays_game_trull_and_a_lost_ultimo() {
    // (values, the declarer wins, ultimo)
    for (v, wins) in [(Values::V234, 3), (Values::V123, 2), (Values::V124, 2)] {
        for ultimo in [1, 2] {
            let mut deal = arranged(&["T1"], &["T2"]);
            deal.settings = Settings {
                values: v,
                ultimo,
                ..Settings::default()
            };
            deal.declarer = Some(Seat::A);
            deal.honours[0][0] = HonourState::Announced;
            deal.trick = 27;
            give_thirds(&mut deal, 109);
            deal.log = fake_26(13);
            let mut ev = Vec::new();
            deal.apply(Seat::A, Action::play(c("T1")), &mut ev).unwrap();
            deal.apply(Seat::B, Action::play(c("T2")), &mut ev).unwrap();
            let Some(DealEnd::Played(sc)) = deal.end() else {
                panic!("deal should be over")
            };
            assert_eq!(sc.result, GameResult::DeclarerWin);
            let by: Vec<(Seat, Cause, u32)> = sc
                .credits
                .iter()
                .map(|c| (c.seat, c.cause, c.points))
                .collect();
            assert_eq!(
                by,
                vec![
                    (Seat::A, Cause::Game, wins),
                    (Seat::A, Cause::Trull, 1),
                    (Seat::B, Cause::UltimoLost, ultimo),
                ]
            );
        }
    }
}
