//! The tutor's hint: the best move at a level, and a short reason as a
//! message key (never prose: the table translates the key, as it does a
//! refusal's, `docs/MESSAGES.md`).
//!
//! The move is the one the opponent at that level would make for the same
//! seed and view (a half step may slip, so a hint at level 3.5 is as erratic
//! as the opponent; a tutor asks for a whole level). The reason is the
//! maxim that weighed most in the scoring of that card (`rung2.rs`,
//! `rung3.rs`), or, when the search chose a card the maxims do not rank
//! first, `search-best`.
//!
//! | Key | Meaning |
//! |---|---|
//! | `bid-strong-hand` | sure winners enough, and the tarocks to back them |
//! | `bid-weak-hand` | too few sure winners to risk the 4 |
//! | `bid-few-tarocks` | honours, but too few tarocks |
//! | `bid-sampled-take` / `bid-sampled-pass` | the sampled deals favour taking up / the simple game |
//! | `take-rich-trick` | win a trick rich in card points |
//! | `take-trick` | win it, cheaply |
//! | `finesse` | do not win a poor trick with an honour; the lead is a burden (Impassiren) |
//! | `follow-cheaply` | lose with the least, or win with the cheapest winner |
//! | `play-packet-off` | play a face-up packet card to see what lies beneath |
//! | `play-exposed-figure` | an exposed queen or cavall goes at once |
//! | `lead-where-he-shows-none` | lead a suit in which he shows no packet card |
//! | `dont-lead-split-suit` | the suit is evenly split; make him open it |
//! | `keep-packet` | keep a packet unplayed: his Pagat ultimo threatens |
//! | `hold-pagat` | hold trump I for the last trick |
//! | `cash-king` | the king is safe to cash |
//! | `draw-trumps` / `lead-solid-run` | strong in tarocks: draw his trumps from a solid run |
//! | `keep-tarocks` | too few tarocks to lead them |
//! | `hold-high-trump` | do not lead the high trumps into a lost cause |
//! | `lead-safe` / `lead-sampled` / `lead-low` | in the sampled deals this lead wins nearly always / often / rarely |
//! | `search-best` | the search found it best over the sampled deals |
//! | `decline-extension` | the sitting is not extended |
//! | `announce-honour` | announce the Troule or the four kings (level 1 only; the others announce with the card) |

use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung2;
use crate::rung3;
use crate::seed::view_seed;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hint {
    pub action: Action,
    /// A message key from [`KEYS`].
    pub key: &'static str,
}

/// Every key a hint can carry.
pub const KEYS: &[&str] = &[
    "bid-strong-hand",
    "bid-weak-hand",
    "bid-few-tarocks",
    "bid-sampled-take",
    "bid-sampled-pass",
    "take-rich-trick",
    "take-trick",
    "finesse",
    "follow-cheaply",
    "play-packet-off",
    "play-exposed-figure",
    "lead-where-he-shows-none",
    "dont-lead-split-suit",
    "keep-packet",
    "hold-pagat",
    "cash-king",
    "draw-trumps",
    "lead-solid-run",
    "keep-tarocks",
    "hold-high-trump",
    "lead-safe",
    "lead-sampled",
    "lead-low",
    "search-best",
    "decline-extension",
    "announce-honour",
];

/// The best move for the seat to move in `view`, at `level`, with a reason.
pub fn hint(view: &View, seed: u64, level: Level, config: Config) -> Hint {
    let opponent = Opponent::new(level, seed).with_config(config);
    let action = opponent.decide(view);
    if matches!(action, Action::Extend { .. }) {
        return Hint {
            action,
            key: "decline-extension",
        };
    }
    let k = Knowledge::from_view(view).expect("a deal is in progress");
    if !k.playing {
        let (rule_take, rule_key) = rung2::bid(&k, config.bid);
        let take = matches!(action, Action::Take);
        let key = if take == rule_take {
            rule_key
        } else if take {
            "bid-sampled-take"
        } else {
            "bid-sampled-pass"
        };
        return Hint { action, key };
    }
    let Action::Play { card, .. } = &action else {
        // Only the random rung announces on its own.
        return Hint {
            action,
            key: "announce-honour",
        };
    };
    let mut rng = Rng::stream(seed, view_seed(view) ^ 0x4811);
    let (rung, _) = level.rungs();
    let scored = if rung >= 3 {
        rung3::score_cards(&k, &mut rng, rung3::WORLDS, &config.weights)
    } else {
        rung2::score_cards(&k, &config.weights)
    };
    let top = scored[0];
    let chosen = scored.iter().find(|c| c.card == *card).unwrap_or(&top);
    let key = if rung >= 4 && chosen.card != top.card {
        "search-best"
    } else {
        chosen.reason
    };
    Hint { action, key }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    #[test]
    fn a_hint_is_the_opponents_move_with_a_known_key() {
        let mut keys = std::collections::HashSet::new();
        for halves in [2u8, 4, 6, 8] {
            let level = Level::from_halves(halves);
            let mut config = Config::default();
            config.budget.nodes = 3_000;
            config.budget.worlds = 3;
            config.budget.bid_worlds = 6;
            for seed in 0..3 {
                walk(seed, |s, seat| {
                    let v = view(s, seat);
                    let h = hint(&v, 9, level, config);
                    assert_eq!(
                        h.action,
                        Opponent::new(level, 9).with_config(config).decide(&v)
                    );
                    assert!(KEYS.contains(&h.key), "{}", h.key);
                    assert_eq!(h, hint(&v, 9, level, config), "asked twice");
                    keys.insert(h.key);
                });
            }
        }
        assert!(keys.len() >= 8, "{keys:?}");
    }

    #[test]
    fn a_hint_is_a_function_of_the_view_alone() {
        // Twin deals that differ only in the cards the seat cannot see give
        // the same hint, move and reason, at every whole level.
        let mut config = Config::default();
        config.budget.nodes = 2_000;
        config.budget.worlds = 3;
        config.budget.bid_worlds = 4;
        let mut twins = 0;
        for halves in [2u8, 4, 6, 8] {
            let level = Level::from_halves(halves);
            twins += crate::testutil::blind_check_values(
                &|v| {
                    let h = hint(v, 9, level, config);
                    (h.action, h.key)
                },
                6,
            );
        }
        assert!(twins > 150, "{twins}");
    }

    #[test]
    fn docs_messages_md_gives_every_hint_key_in_english_and_german() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let section = doc.split("\n## Hint reasons").nth(1).expect("section");
        let rows: Vec<Vec<&str>> = section
            .lines()
            .filter(|l| l.starts_with("| `"))
            .map(|l| l.split('|').map(str::trim).collect())
            .collect();
        let documented: Vec<&str> = rows.iter().map(|r| r[1].trim_matches('`')).collect();
        assert_eq!(documented, KEYS, "same keys, same order");
        for r in &rows {
            assert!(!r[2].is_empty() && !r[3].is_empty(), "{}", r[1]);
        }
    }

    #[test]
    fn the_keys_are_distinct_and_kebab_case() {
        let set: std::collections::HashSet<_> = KEYS.iter().collect();
        assert_eq!(set.len(), KEYS.len());
        for k in KEYS {
            assert!(k.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{k}");
        }
    }
}
