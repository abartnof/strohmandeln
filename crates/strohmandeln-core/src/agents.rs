//! Players: the trait the table, the opponent and the tests share, and two
//! simple implementations for tests.

use crate::rng::{purpose, Rng};
use crate::session::Sitting;
use crate::types::*;
use crate::view::{view, View};

/// Something that chooses an action from what its seat may know.
pub trait Player {
    /// Chooses one of `view.legal_actions`. Called only on the player's turn.
    fn choose(&mut self, view: &View) -> Action;
}

/// Picks uniformly among the legal actions, from the agent stream of its seed.
pub struct RandomPlayer {
    rng: Rng,
}

impl RandomPlayer {
    pub fn new(seed: u64, seat: Seat) -> RandomPlayer {
        RandomPlayer {
            rng: Rng::stream(seed, purpose::agent(seat.index())),
        }
    }
}

impl Player for RandomPlayer {
    fn choose(&mut self, view: &View) -> Action {
        let n = view.legal_actions.len() as u64;
        view.legal_actions[self.rng.below(n) as usize].clone()
    }
}

/// Always the first legal action, except that it declines an extension (so
/// that a sitting between two of them ends).
pub struct FirstLegal;

impl Player for FirstLegal {
    fn choose(&mut self, view: &View) -> Action {
        match view.legal_actions[0] {
            Action::Extend { .. } => Action::Extend { accept: false },
            ref a => a.clone(),
        }
    }
}

/// Plays a sitting to its end. Returns the number of moves made, or an error
/// if a player's choice is refused or `max_moves` is exceeded.
pub fn play_out(
    sitting: &mut Sitting,
    players: &mut [Box<dyn Player>; 2],
    max_moves: usize,
) -> Result<usize, String> {
    let mut n = 0;
    while let Some(seat) = sitting.to_move() {
        if n >= max_moves {
            return Err(format!("more than {max_moves} moves"));
        }
        let v = view(sitting, seat);
        if v.legal_actions.is_empty() {
            return Err(format!("seat {seat:?} to move with no legal action"));
        }
        let action = players[seat.index()].choose(&v);
        sitting
            .apply(seat, action.clone())
            .map_err(|e| format!("{seat:?} chose {action:?}: {e}"))?;
        n += 1;
    }
    Ok(n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::Card;

    #[test]
    fn first_legal_is_deterministic_and_takes_the_first_bid() {
        let s = Sitting::new(1, Settings::default(), 3).unwrap();
        let seat = s.to_move().unwrap();
        let v = view(&s, seat);
        assert_eq!(FirstLegal.choose(&v), v.legal_actions[0]);
        assert_eq!(FirstLegal.choose(&v), Action::Take);
    }

    #[test]
    fn first_legal_declines_an_extension() {
        let v = View {
            legal_actions: vec![
                Action::Extend { accept: true },
                Action::Extend { accept: false },
            ],
            ..view(&Sitting::new(1, Settings::default(), 3).unwrap(), Seat::A)
        };
        assert_eq!(FirstLegal.choose(&v), Action::Extend { accept: false });
    }

    #[test]
    fn the_random_player_only_chooses_legal_actions_and_varies() {
        let s = Sitting::new(2, Settings::default(), 3).unwrap();
        let seat = s.to_move().unwrap();
        let v = view(&s, seat);
        let mut p = RandomPlayer::new(5, seat);
        let mut seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let a = p.choose(&v);
            assert!(v.legal_actions.contains(&a));
            seen.insert(serde_json::to_string(&a).unwrap());
        }
        assert_eq!(seen.len(), 2, "both bids come up");
    }

    #[test]
    fn a_random_player_is_fixed_by_its_seed() {
        let s = Sitting::new(2, Settings::default(), 3).unwrap();
        let v = view(&s, s.to_move().unwrap());
        let run = |seed| {
            let mut p = RandomPlayer::new(seed, Seat::A);
            (0..20).map(|_| p.choose(&v)).collect::<Vec<_>>()
        };
        assert_eq!(run(9), run(9));
    }

    #[test]
    fn two_first_legal_players_finish_a_sitting() {
        let mut s = Sitting::new(3, Settings::default(), 3).unwrap();
        let mut players: [Box<dyn Player>; 2] = [Box::new(FirstLegal), Box::new(FirstLegal)];
        let n = play_out(&mut s, &mut players, 100_000).unwrap();
        assert!(n > 100);
        assert!(s.is_over());
        // sanity: all 54 cards were dealt in every deal
        assert_eq!(Card::all().count(), 54);
    }

    #[test]
    fn play_out_reports_a_runaway() {
        let mut s = Sitting::new(3, Settings::default(), 3).unwrap();
        let mut players: [Box<dyn Player>; 2] = [Box::new(FirstLegal), Box::new(FirstLegal)];
        assert!(play_out(&mut s, &mut players, 5).is_err());
    }
}
