//! Tests of legal plays, tricks and replacement (RULES 5 to 7).

use super::tests::c;
use super::*;

type PacketSpec<'a> = (Option<&'a str>, &'a [&'a str]);
const EMPTY: PacketSpec = (None, &[]);

/// A deal in the playing phase with dealer B (forehand A leads), the given
/// hands and packets (cover, face-down cards top first).
fn arranged(
    a_hand: &[&str],
    b_hand: &[&str],
    a_packets: [PacketSpec; 3],
    b_packets: [PacketSpec; 3],
) -> Deal {
    Deal::arranged(
        Seat::B,
        [a_hand, b_hand],
        [a_packets, b_packets],
        Settings::default(),
    )
}

fn play(deal: &mut Deal, seat: Seat, code: &str) -> Result<Vec<Event>, Refusal> {
    let mut ev = Vec::new();
    deal.apply(seat, Action::play(c(code)), &mut ev).map(|_| ev)
}

fn plays(deal: &Deal, seat: Seat) -> Vec<String> {
    let mut v: Vec<String> = deal
        .legal_plays(seat)
        .iter()
        .map(|(card, _)| card.code())
        .collect();
    v.sort();
    v
}

fn sorted(codes: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = codes.iter().map(|s| s.to_string()).collect();
    v.sort();
    v
}

#[test]
fn rules_5_1_forehand_leads_the_first_trick_whoever_declared() {
    let mut deal = arranged(&["T5", "7c"], &["T6", "8c"], [EMPTY; 3], [EMPTY; 3]);
    deal.declarer = Some(Seat::B);
    assert_eq!(deal.to_move(), Some(Seat::A));
    let err = play(&mut deal, Seat::B, "T6").unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotYourTurn);
}

#[test]
fn rules_5_1_and_7_2_the_winner_leads_next() {
    let mut deal = arranged(&["T5", "7c"], &["T6", "8c"], [EMPTY; 3], [EMPTY; 3]);
    play(&mut deal, Seat::A, "T5").unwrap();
    assert_eq!(deal.to_move(), Some(Seat::B));
    let ev = play(&mut deal, Seat::B, "T6").unwrap();
    assert!(ev.contains(&Event::TrickWon {
        winner: Seat::B,
        cards: [c("T5"), c("T6")],
        trick: 1
    }));
    assert_eq!(deal.to_move(), Some(Seat::B));
}

#[test]
fn rules_5_2_covers_are_playable_and_face_down_cards_are_not() {
    let mut deal = arranged(
        &["7c"],
        &["8c"],
        [(Some("Jh"), &["T3", "Kc", "9c"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    assert_eq!(plays(&deal, Seat::A), sorted(&["7c", "Jh"]));
    let err = play(&mut deal, Seat::A, "T3").unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotPlayable);
    assert_eq!(err.card, Some(c("T3")));
    // the opponent's cards are not playable either
    let err = play(&mut deal, Seat::A, "8c").unwrap_err();
    assert_eq!(err.kind, RefusalKind::NotPlayable);
}

#[test]
fn rules_5_7_legal_plays_name_their_source() {
    let deal = arranged(
        &["7c"],
        &["8c"],
        [(Some("Jh"), &["T3"]), EMPTY, (Some("Qd"), &["T4"])],
        [EMPTY; 3],
    );
    let mut got: Vec<(String, Source)> = deal
        .legal_plays(Seat::A)
        .into_iter()
        .map(|(c, s)| (c.code(), s))
        .collect();
    got.sort_by_key(|(c, _)| c.clone());
    assert_eq!(
        got,
        vec![
            ("7c".to_string(), Source::Hand),
            ("Jh".to_string(), Source::Packet(1)),
            ("Qd".to_string(), Source::Packet(3)),
        ]
    );
}

#[test]
fn rules_5_3_the_leader_may_lead_any_playable_card() {
    let deal = arranged(
        &["T5", "7c", "Kh"],
        &["8c"],
        [(Some("Jh"), &["T3"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    assert_eq!(plays(&deal, Seat::A), sorted(&["T5", "7c", "Kh", "Jh"]));
}

#[test]
fn rules_5_4_1_must_follow_suit_from_hand_or_cover_freely() {
    let mut deal = arranged(
        &["7c"],
        &["2h", "Qc", "T5"],
        [EMPTY; 3],
        [(Some("4h"), &["T9"]), EMPTY, EMPTY],
    );
    play(&mut deal, Seat::A, "7c").unwrap();
    // B holds Qc: must play it; tarock and hearts are refused
    let err = play(&mut deal, Seat::B, "T5").unwrap_err();
    assert_eq!(err.kind, RefusalKind::MustFollowSuit);
    assert_eq!(err.led.as_deref(), Some("c"));
    assert_eq!(err.allowed, vec![c("Qc")]);
    let err = play(&mut deal, Seat::B, "4h").unwrap_err();
    assert_eq!(err.kind, RefusalKind::MustFollowSuit);
    assert_eq!(plays(&deal, Seat::B), sorted(&["Qc"]));
}

#[test]
fn rules_5_4_1_a_cover_of_the_suit_may_be_played_while_a_hand_card_is_available_and_back() {
    let mut deal = arranged(
        &["Kh"],
        &["2h", "T5"],
        [EMPTY; 3],
        [(Some("4h"), &["T9", "T8"]), EMPTY, EMPTY],
    );
    play(&mut deal, Seat::A, "Kh").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["2h", "4h"]));
    let ev = play(&mut deal, Seat::B, "4h").unwrap();
    assert!(ev.contains(&Event::CardPlayed {
        seat: Seat::B,
        card: c("4h"),
        source: Source::Packet(1),
        trick: 1
    }));
}

#[test]
fn rules_5_4_2_void_in_the_suit_must_trump_from_hand_or_cover() {
    let mut deal = arranged(
        &["7c"],
        &["Kh", "T5"],
        [EMPTY; 3],
        [(Some("T4"), &[]), EMPTY, EMPTY],
    );
    // T4 as a cover is impossible by 4.8 but legal to the play rules
    play(&mut deal, Seat::A, "7c").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["T5", "T4"]));
    let err = play(&mut deal, Seat::B, "Kh").unwrap_err();
    assert_eq!(err.kind, RefusalKind::MustTrump);
    assert_eq!(err.allowed.len(), 2);
}

#[test]
fn rules_5_4_3_void_in_the_suit_and_in_tarocks_may_play_anything() {
    let mut deal = arranged(
        &["7c"],
        &["Kh", "Qd"],
        [EMPTY; 3],
        [(Some("Jh"), &["T9"]), EMPTY, EMPTY],
    );
    play(&mut deal, Seat::A, "7c").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["Kh", "Qd", "Jh"]));
}

#[test]
fn rules_5_4_trump_led_follower_must_play_a_tarock_else_anything() {
    let mut deal = arranged(&["SK"], &["Kh", "T5"], [EMPTY; 3], [EMPTY; 3]);
    play(&mut deal, Seat::A, "SK").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["T5"]));
    let mut deal = arranged(&["T1"], &["Kh", "Qd"], [EMPTY; 3], [EMPTY; 3]);
    play(&mut deal, Seat::A, "T1").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["Kh", "Qd"]));
}

#[test]
fn rules_5_5_there_is_no_obligation_to_win() {
    // must follow suit but may play lower; must trump but may play a lower tarock
    let mut deal = arranged(&["Qh"], &["4h", "Kh"], [EMPTY; 3], [EMPTY; 3]);
    play(&mut deal, Seat::A, "Qh").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["4h", "Kh"]));
    let mut deal = arranged(&["T20"], &["T2", "T21"], [EMPTY; 3], [EMPTY; 3]);
    play(&mut deal, Seat::A, "T20").unwrap();
    assert_eq!(plays(&deal, Seat::B), sorted(&["T2", "T21"]));
}

#[test]
fn rules_7_1_trick_winners() {
    let cases = [
        // (led, followed, winner is leader?)
        ("Kh", "4h", true),
        ("4h", "Kh", false),
        ("1h", "4h", true), // 1 is the highest pip
        ("4d", "3d", false),
        ("Jh", "1h", true), // pips are below the jack
        ("Cs", "Js", true),
        ("10c", "7c", true),
        ("7s", "8s", false),
        ("Kh", "T1", false), // lowest tarock beats any suit card
        ("T1", "Kh", true),  // off-suit never wins
        ("T2", "T1", true),
        ("T20", "T21", false),
        ("T21", "SK", false), // Skus is highest
        ("SK", "T21", true),
    ];
    for (led, follow, leader_wins) in cases {
        // give the follower no obligation problem: arrange hands to hold only the card
        let mut deal = arranged(&[led], &[follow], [EMPTY; 3], [EMPTY; 3]);
        play(&mut deal, Seat::A, led).unwrap();
        let ev = play(&mut deal, Seat::B, follow).unwrap();
        let want = if leader_wins { Seat::A } else { Seat::B };
        assert!(
            ev.iter()
                .any(|e| matches!(e, Event::TrickWon { winner, .. } if *winner == want)),
            "{led} then {follow}"
        );
    }
}

#[test]
fn rules_6_1_a_played_cover_is_replaced_after_the_trick_with_events_in_order() {
    let mut deal = arranged(
        &["Kh", "T7"],
        &["2h", "Qc"],
        [(Some("Jh"), &["T3", "Kc", "9c"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    // A leads the cover Jh, B follows 2h: A wins and leads
    play(&mut deal, Seat::A, "Jh").unwrap();
    let ev = play(&mut deal, Seat::B, "2h").unwrap();
    let kinds: Vec<&Event> = ev.iter().collect();
    // CardPlayed, TrickWon, then T3, Kc and the last card 9c turned and pulled
    assert!(matches!(kinds[0], Event::CardPlayed { .. }));
    assert!(matches!(kinds[1], Event::TrickWon { .. }));
    assert_eq!(
        kinds[2..],
        [
            &Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("T3")
            },
            &Event::CardPulled {
                seat: Seat::A,
                packet: 1,
                card: c("T3"),
                last: false
            },
            &Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("Kc")
            },
            &Event::CardPulled {
                seat: Seat::A,
                packet: 1,
                card: c("Kc"),
                last: false
            },
            &Event::PacketTurned {
                seat: Seat::A,
                packet: 1,
                card: c("9c")
            },
            &Event::CardPulled {
                seat: Seat::A,
                packet: 1,
                card: c("9c"),
                last: true
            },
            &Event::PacketEmpty {
                seat: Seat::A,
                packet: 1
            },
        ]
    );
    // 6.4: pulled cards are in the hand, public, and the packet is empty
    assert!(deal
        .hand(Seat::A)
        .iter()
        .any(|h| h.card == c("T3") && h.public));
    assert_eq!(deal.cover(Seat::A, 1), None);
}

#[test]
fn rules_6_1_a_low_card_beneath_becomes_the_new_cover() {
    let mut deal = arranged(
        &["Kh"],
        &["2h"],
        [(Some("Jh"), &["Qc", "T3", "9c"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    play(&mut deal, Seat::A, "Jh").unwrap();
    let ev = play(&mut deal, Seat::B, "2h").unwrap();
    assert!(ev.contains(&Event::CoverSet {
        seat: Seat::A,
        packet: 1,
        card: c("Qc")
    }));
    assert_eq!(deal.cover(Seat::A, 1), Some(c("Qc")));
    assert_eq!(deal.hidden_count(Seat::A, 1), 2);
}

#[test]
fn rules_6_1_playing_a_hand_card_replaces_nothing() {
    let mut deal = arranged(
        &["Kh"],
        &["2h"],
        [(Some("Jh"), &["Qc", "T3", "9c"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    play(&mut deal, Seat::A, "Kh").unwrap();
    let ev = play(&mut deal, Seat::B, "2h").unwrap();
    assert!(!ev.iter().any(|e| matches!(e, Event::PacketTurned { .. })));
}

#[test]
fn rules_6_3_when_both_played_covers_the_leaders_packet_is_replaced_first() {
    let mut deal = arranged(
        &["7c"],
        &["8c"],
        [(Some("Jh"), &["Qc", "T3", "9c"]), EMPTY, EMPTY],
        [(Some("Jd"), &["Qs", "T4", "9s"]), EMPTY, EMPTY],
    );
    // A leads Jh; B has hearts? no: B void in hearts, has no tarock -> free; plays cover Jd
    play(&mut deal, Seat::A, "Jh").unwrap();
    let ev = play(&mut deal, Seat::B, "Jd").unwrap();
    let turned: Vec<Seat> = ev
        .iter()
        .filter_map(|e| match e {
            Event::PacketTurned { seat, .. } => Some(*seat),
            _ => None,
        })
        .collect();
    assert_eq!(turned, vec![Seat::A, Seat::B]);
}

#[test]
fn rules_6_2_the_follower_cannot_use_the_card_the_leaders_cover_reveals() {
    // A leads cover Jh; beneath it lies T3. B must follow with a tarock only
    // if holding one; T3 is not B's, and is not in A's playable set until the
    // trick is over.
    let mut deal = arranged(
        &["7c"],
        &["Kc"],
        [(Some("Jh"), &["T3", "9c", "8c"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    play(&mut deal, Seat::A, "Jh").unwrap();
    // during the trick the packet is still unrevealed
    assert_eq!(deal.cover(Seat::A, 1), None);
    assert_eq!(deal.hidden_count(Seat::A, 1), 3);
    assert_eq!(plays(&deal, Seat::B), sorted(&["Kc"]));
}

#[test]
fn rules_6_4_a_card_pulled_by_replacement_may_be_played_next_trick() {
    let mut deal = arranged(
        &["7c"],
        &["Kc", "9c"],
        [(Some("Jh"), &["T3", "T4", "9s"]), EMPTY, EMPTY],
        [EMPTY; 3],
    );
    play(&mut deal, Seat::A, "Jh").unwrap();
    play(&mut deal, Seat::B, "Kc").unwrap(); // A wins and leads again
    assert_eq!(deal.to_move(), Some(Seat::A));
    assert_eq!(plays(&deal, Seat::A), sorted(&["7c", "T3", "T4", "9s"]));
    play(&mut deal, Seat::A, "T3").unwrap();
    assert!(deal.hand(Seat::A).iter().all(|h| h.card != c("T3")));
}
