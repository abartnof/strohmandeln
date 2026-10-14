//! Property tests over many seeds: whole sittings played by random legal
//! players (hand-rolled loops over seeds, so the result is reproducible).

use strohmandeln_core::agents::{play_out, FirstLegal, Player, RandomPlayer};
use strohmandeln_core::cards::Card;
use strohmandeln_core::session::Sitting;
use strohmandeln_core::types::*;
use strohmandeln_core::view::view;

fn all_settings() -> Vec<Settings> {
    let mut v = Vec::new();
    for both_pass in [BothPass::Play, BothPass::ThrowIn] {
        for values in [Values::V234, Values::V123, Values::V124] {
            for ultimo in [1, 2] {
                v.push(Settings {
                    both_pass,
                    values,
                    ultimo,
                });
            }
        }
    }
    v
}

fn random_players(seed: u64) -> [Box<dyn Player>; 2] {
    [
        Box::new(RandomPlayer::new(seed, Seat::A)),
        Box::new(RandomPlayer::new(seed, Seat::B)),
    ]
}

fn sitting_for(seed: u64) -> Sitting {
    let settings = all_settings()[(seed % 12) as usize];
    let touren = if seed.is_multiple_of(5) { 6 } else { 3 };
    Sitting::new(seed, settings, touren).unwrap()
}

const SEEDS: u64 = 120;

#[test]
fn every_random_game_finishes_legally() {
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000)
            .unwrap_or_else(|e| panic!("seed {seed}: {e}"));
        assert!(s.is_over(), "seed {seed}");
        assert!(matches!(s.events().last(), Some(Event::SittingEnd { .. })));
    }
}

#[test]
fn rules_8_1_the_two_takes_always_total_70_points_in_thirds() {
    let mut counts = 0;
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        for e in s.events() {
            if let Event::Count { thirds, .. } = e {
                assert_eq!(thirds[0] + thirds[1], 210, "seed {seed}");
                counts += 1;
            }
        }
    }
    assert!(counts > 300);
}

#[test]
fn rules_5_1_every_played_deal_has_27_tricks() {
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        let mut tricks = 0;
        let mut deals_played = 0;
        for e in s.events() {
            match e {
                Event::TrickWon { .. } => tricks += 1,
                Event::Count { .. } => {
                    assert_eq!(tricks, 27, "seed {seed}");
                    tricks = 0;
                    deals_played += 1;
                }
                Event::ThrownIn => assert_eq!(tricks, 0),
                _ => {}
            }
        }
        assert!(deals_played > 0);
    }
}

#[test]
fn rules_15_5_and_15_8_the_conto_nets_to_zero_between_the_players() {
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        let mut credited = [0u32; 2];
        for e in s.events() {
            if let Event::GameScored { credits, .. } = e {
                for c in credits {
                    credited[c.seat.index()] += c.points;
                }
            }
        }
        assert_eq!(credited, s.contos(), "seed {seed}");
        let Some(Event::SittingEnd {
            contos,
            balance,
            payer,
            kreuzer,
        }) = s.events().last().cloned()
        else {
            panic!("no sitting end")
        };
        // the payer's loss is the other's gain, at 2 kreuzer a Point
        let paid = kreuzer as i64;
        let (a, b) = match payer {
            Some(Seat::A) => (-paid, paid),
            Some(Seat::B) => (paid, -paid),
            None => (0, 0),
        };
        assert_eq!(a + b, 0);
        assert_eq!(kreuzer, balance.unsigned_abs() * 2);
        assert_eq!(balance, contos[0] as i32 - contos[1] as i32);
        // the payer is the one with the smaller Conto
        if let Some(p) = payer {
            assert!(contos[p.index()] < contos[p.other().index()]);
        }
    }
}

#[test]
fn replaying_a_record_reproduces_every_event() {
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        let record = s.record();
        // through JSON, as the wasm layer will keep it
        let json = serde_json::to_string(&record).unwrap();
        let back = serde_json::from_str(&json).unwrap();
        assert_eq!(record, back);
        let replayed = Sitting::replay(&back).unwrap();
        assert_eq!(replayed.events(), s.events(), "seed {seed}");
        assert_eq!(replayed.contos(), s.contos());
    }
}

#[test]
fn a_record_of_another_version_is_refused() {
    let s = Sitting::new(1, Settings::default(), 3).unwrap();
    let mut r = s.record();
    r.version += 1;
    assert!(Sitting::replay(&r).is_err());
}

#[test]
fn a_partial_record_replays_to_the_same_state_and_play_goes_on() {
    let mut s = sitting_for(7);
    let mut players = random_players(7);
    for _ in 0..200 {
        let seat = s.to_move().unwrap();
        let v = view(&s, seat);
        let a = players[seat.index()].choose(&v).unwrap();
        s.apply(seat, a).unwrap();
    }
    let r = Sitting::replay(&s.record()).unwrap();
    assert_eq!(r.events(), s.events());
    assert_eq!(r.to_move(), s.to_move());
    assert_eq!(r.legal_actions(), s.legal_actions());
}

#[test]
fn rules_4_8_every_non_empty_packet_has_a_low_cover_at_every_step() {
    for seed in 0..40 {
        let mut s = sitting_for(seed);
        let mut players = random_players(seed);
        while let Some(seat) = s.to_move() {
            if let Some(d) = view(&s, seat).deal {
                if d.playing {
                    for owner in 0..2 {
                        for p in 0..3 {
                            let cover = d.covers[owner][p];
                            let hidden = d.hidden[owner][p];
                            if let Some(c) = cover {
                                assert!(!c.is_tarock() && !c.is_king(), "seed {seed}");
                            }
                            // the trick in progress may have vacated a cover
                            let mid_trick = d.lead.is_some();
                            if hidden > 0 && cover.is_none() {
                                assert!(mid_trick, "seed {seed}: packet without cover");
                            }
                        }
                    }
                }
            }
            let v = view(&s, seat);
            let a = players[seat.index()].choose(&v).unwrap();
            s.apply(seat, a).unwrap();
        }
    }
}

#[test]
fn rules_5_4_every_card_not_legal_is_refused_with_a_key_and_changes_nothing() {
    let mut refused = 0;
    for seed in 0..25 {
        let mut s = sitting_for(seed);
        let mut players = random_players(seed);
        while let Some(seat) = s.to_move() {
            if s.deal().map(|d| d.is_playing()).unwrap_or(false) {
                let legal = s.legal_actions();
                for card in Card::all() {
                    let action = Action::play(card);
                    if legal.contains(&action) {
                        continue;
                    }
                    let mut copy = s.clone();
                    let before = copy.events().len();
                    let err = copy.apply(seat, action).unwrap_err();
                    assert!(!format!("{err}").is_empty());
                    assert_eq!(copy.events().len(), before);
                    refused += 1;
                }
            }
            let v = view(&s, seat);
            let a = players[seat.index()].choose(&v).unwrap();
            s.apply(seat, a).unwrap();
        }
    }
    assert!(refused > 10_000);
}

#[test]
fn rules_14_a_doubling_never_exceeds_two() {
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        for e in s.events() {
            match e {
                Event::DealStart { multiplier, .. } => assert!(*multiplier <= 2),
                Event::GameScored { multiplier, .. } => assert!(*multiplier <= 2),
                _ => {}
            }
        }
    }
}

#[test]
fn first_legal_players_finish_every_setting() {
    for (i, settings) in all_settings().into_iter().enumerate() {
        let mut s = Sitting::new(i as u64, settings, 3).unwrap();
        let mut players: [Box<dyn Player>; 2] = [Box::new(FirstLegal), Box::new(FirstLegal)];
        play_out(&mut s, &mut players, 100_000).unwrap();
        assert!(s.is_over());
    }
}

#[test]
fn the_variety_of_outcomes_is_seen_over_the_seeds() {
    // not a rule: a guard that the random games exercise draws, valats-free
    // wins, honours and the Pagat, so the properties above are not vacuous
    let (mut draws, mut announcements, mut ultimos, mut doubled) = (0, 0, 0, 0);
    for seed in 0..SEEDS {
        let mut s = sitting_for(seed);
        play_out(&mut s, &mut random_players(seed), 200_000).unwrap();
        for e in s.events() {
            match e {
                Event::GameScored {
                    result: GameResult::Draw,
                    ..
                } => draws += 1,
                Event::Announcement { .. } => announcements += 1,
                Event::PagatUltimo { .. } => ultimos += 1,
                Event::DoublingApplied { .. } => doubled += 1,
                _ => {}
            }
        }
    }
    assert!(draws > 0, "no drawn game");
    assert!(announcements > 0, "no announcement");
    assert!(ultimos > 0, "no Pagat ultimo");
    assert!(doubled > 0, "no doubled deal");
}
