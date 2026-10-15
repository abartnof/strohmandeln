//! The tutor's hint: the best move at a level, and why, as a message key with
//! parameters (never prose: the table translates the key, as it does a
//! refusal's, `docs/MESSAGES.md`).
//!
//! The move is the one the opponent at that level would make for the same
//! seed and view (a half step may slip, so a hint at level 3.5 is as erratic
//! as the opponent; a tutor asks for a whole level). The reason explains it
//! (`explain.rs`) in terms the table can check: the trick's points, the card
//! it beats or gives up, the tarocks unseen, how often a lead wins over the
//! sampled deals. It may name a rule it follows (`rule`), and, at the top
//! rung, how the search ranks the card against the best other (`odds`):
//! evidence from a fresh sample of deals, given only when it agrees with the
//! choice.
//!
//! The keys and parameters are tabled in `docs/MESSAGES.md`, "Hint reasons";
//! [`KEYS`] and [`RULES`] list them and a test holds the table to them.

use crate::explain::{explain_play, Maxim, Param, Reason, Sampled};
use crate::know::Knowledge;
use crate::opponent::{Config, Level, Opponent};
use crate::rung2;
use crate::rung3;
use crate::rung4;
use crate::seed::view_seed;
use strohmandeln_core::cards::Card;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::Action;
use strohmandeln_core::view::View;

/// How the search ranks the hinted card against the best other legal card,
/// over `worlds` freshly sampled deals.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Odds {
    pub alt: Card,
    /// Worlds in which the hinted card does strictly better than `alt`.
    pub better: u32,
    pub worlds: u32,
    /// The average margin gained over `alt`, in thirds of a point (at least 1).
    pub gain_thirds: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Hint {
    pub action: Action,
    /// A message key from [`KEYS`].
    pub key: &'static str,
    pub params: Vec<(&'static str, Param)>,
    /// A key from [`RULES`]: the rule the move follows.
    pub rule: Option<&'static str>,
    pub odds: Option<Odds>,
}

impl Hint {
    fn plain(action: Action, key: &'static str) -> Hint {
        Hint {
            action,
            key,
            params: Vec::new(),
            rule: None,
            odds: None,
        }
    }
}

/// Every key a hint can carry.
pub const KEYS: &[&str] = &[
    "bid-strong-hand",
    "bid-weak-hand",
    "bid-few-tarocks",
    "bid-sampled-take",
    "bid-sampled-pass",
    "only-card",
    "hold-pagat",
    "keep-packet",
    "win-cheapest",
    "win-over-cheaper",
    "win-sole",
    "decline-poor",
    "decline-rich",
    "lose-cheapest",
    "lose-other",
    "lose-packet",
    "lead-king",
    "lead-draw",
    "lead-exposed",
    "lead-his-none",
    "lead-trump-void",
    "lead-rate",
    "lead-cheap",
    "lead-cost",
    "decline-extension",
    "announce-honour",
];

/// Every rule a hint can name.
pub const RULES: &[&str] = &[
    "bid",
    "cheapest",
    "finesse",
    "pagat-last",
    "keep-packet",
    "packet-off",
    "draw-trumps",
    "exposed",
    "lead-none",
];

/// Worlds in the evidence for the search's ranking.
const ODDS_WORLDS: usize = 10;

/// The best move for the seat to move in `view`, at `level`, with a reason.
pub fn hint(view: &View, seed: u64, level: Level, config: Config) -> Hint {
    let opponent = Opponent::new(level, seed).with_config(config);
    let action = opponent.decide(view);
    if matches!(action, Action::Extend { .. }) {
        return Hint::plain(action, "decline-extension");
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
        let (sure, tarocks) = rung2::sure_thirds(k.hand);
        return Hint {
            action,
            key,
            params: vec![
                ("sure_thirds", Param::Int(i64::from(sure))),
                ("tarocks", Param::Int(tarocks as i64)),
            ],
            rule: Some("bid"),
            odds: None,
        };
    }
    let Action::Play { card, .. } = &action else {
        // Only the random rung announces on its own.
        return Hint::plain(action, "announce-honour");
    };
    let card = *card;
    let mut rng = Rng::stream(seed, view_seed(view) ^ 0x4811);
    let (rung, _) = level.rungs();
    let scored = if rung >= 3 {
        rung3::score_cards(&k, &mut rng, rung3::WORLDS, &config.weights)
    } else {
        rung2::score_cards(&k, &config.weights)
    };
    let top = scored[0];
    let chosen = scored.iter().find(|c| c.card == card).unwrap_or(&top);
    let maxim = Maxim {
        reason: chosen.reason,
        is_top: chosen.card == top.card,
    };
    // The sampled deals a lead is weighed in; fresh, so the reason does not
    // depend on the stream that chose the card.
    let sims = if rung >= 3 {
        let mut srng = Rng::stream(seed, view_seed(view) ^ 0x4813);
        rung3::lead_sims(&k, &mut srng, rung3::WORLDS)
    } else {
        Vec::new()
    };
    let wins = |c: Card| rung3::lead_wins(&sims, c);
    let Reason { key, params, rule } = explain_play(
        &k,
        card,
        &maxim,
        &Sampled {
            worlds: sims.len(),
            wins: &wins,
        },
    );
    let odds = if rung >= 4 && k.legal.len() > 1 {
        let mut orng = Rng::stream(seed, view_seed(view) ^ 0x4812);
        odds_with(&k, card, &mut orng, config, ODDS_WORLDS)
    } else {
        None
    };
    Hint {
        action,
        key,
        params,
        rule,
        odds,
    }
}

/// The search's evidence for `card` against the best other legal card, over
/// `worlds` sampled deals (the hint uses `ODDS_WORLDS`; `bin/hintbench.rs`
/// varies it). The node budget in `config` is shared by the worlds.
pub fn odds_with(
    k: &Knowledge,
    card: Card,
    rng: &mut Rng,
    config: Config,
    worlds: usize,
) -> Option<Odds> {
    let mut budget = config.budget;
    budget.worlds = worlds;
    let table = rung4::move_table(k, rng, budget, &config.weights);
    if table.per_world.is_empty() {
        return None;
    }
    let mine = table.scores.iter().find(|(x, _)| *x == card)?.1;
    let (alt, theirs) = table.scores.iter().find(|(x, _)| *x != card).copied()?;
    let gain = (mine - theirs).round();
    if gain < 1.0 {
        return None;
    }
    let value = |row: &[(Card, i32)], c: Card| row.iter().find(|(x, _)| *x == c).map(|(_, v)| *v);
    let better = table
        .per_world
        .iter()
        .filter(|row| matches!((value(row, card), value(row, alt)), (Some(a), Some(b)) if a > b))
        .count();
    Some(Odds {
        alt,
        better: better as u32,
        worlds: table.per_world.len() as u32,
        gain_thirds: gain as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::walk;
    use strohmandeln_core::view::view;

    fn small() -> Config {
        let mut config = Config::default();
        config.budget.nodes = 3_000;
        config.budget.worlds = 3;
        config.budget.bid_worlds = 6;
        config
    }

    #[test]
    fn a_hint_is_the_opponents_move_with_a_known_key() {
        let mut keys = std::collections::HashSet::new();
        for halves in [2u8, 4, 6, 8] {
            let level = Level::from_halves(halves);
            let config = small();
            for seed in 0..3 {
                walk(seed, |s, seat| {
                    let v = view(s, seat);
                    let h = hint(&v, 9, level, config);
                    assert_eq!(
                        h.action,
                        Opponent::new(level, 9).with_config(config).decide(&v)
                    );
                    assert!(KEYS.contains(&h.key), "{}", h.key);
                    assert!(h.rule.is_none_or(|r| RULES.contains(&r)), "{:?}", h.rule);
                    assert_eq!(h, hint(&v, 9, level, config), "asked twice");
                    keys.insert(h.key);
                });
            }
        }
        assert!(keys.len() >= 10, "{keys:?}");
    }

    #[test]
    fn every_hinted_card_is_a_parameter_the_reason_is_about() {
        // The card hinted is always named by the reason, and the odds (when
        // given) compare it to a different legal card.
        let level = Level::from_halves(8);
        let mut odds = 0;
        for seed in 0..3 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let h = hint(&v, 9, level, small());
                if let Action::Play { card, .. } = &h.action {
                    assert!(h.params.contains(&("card", Param::Card(*card))), "{h:?}");
                    if let Some(o) = &h.odds {
                        assert_ne!(o.alt, *card);
                        assert!(o.better <= o.worlds && o.gain_thirds >= 1, "{h:?}");
                        let k = Knowledge::from_view(&v).unwrap();
                        assert!(k.legal.contains(&o.alt));
                        odds += 1;
                    }
                }
            });
        }
        assert!(odds > 0, "no hint carried odds");
    }

    #[test]
    fn the_hint_s_odds_are_odds_with_the_hint_s_worlds() {
        let level = Level::from_halves(8);
        let config = small();
        let mut checked = 0;
        walk(2, |s, seat| {
            let v = view(s, seat);
            let h = hint(&v, 9, level, config);
            let (Some(odds), Action::Play { card, .. }) = (&h.odds, &h.action) else {
                return;
            };
            let k = Knowledge::from_view(&v).unwrap();
            let mut rng = Rng::stream(9, view_seed(&v) ^ 0x4812);
            let again = odds_with(&k, *card, &mut rng, config, ODDS_WORLDS);
            assert_eq!(again.as_ref(), Some(odds));
            checked += 1;
        });
        assert!(checked > 0);
    }

    #[test]
    fn a_bid_hint_gives_the_hand_s_sure_winners_and_tarocks() {
        let level = Level::from_halves(8);
        let mut bids = 0;
        walk(1, |s, seat| {
            let v = view(s, seat);
            let k = Knowledge::from_view(&v).unwrap();
            if !k.playing && !matches!(v.legal_actions.first(), Some(Action::Extend { .. })) {
                let h = hint(&v, 9, level, small());
                let (sure, tarocks) = rung2::sure_thirds(k.hand);
                assert_eq!(h.params[0], ("sure_thirds", Param::Int(i64::from(sure))));
                assert_eq!(h.params[1], ("tarocks", Param::Int(tarocks as i64)));
                assert_eq!(h.rule, Some("bid"));
                bids += 1;
            }
        });
        assert!(bids > 0);
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
                    (h.action, h.key, h.params, h.rule, h.odds)
                },
                6,
            );
        }
        assert!(twins > 150, "{twins}");
    }

    /// The rows of a `docs/MESSAGES.md` table between two headings.
    fn rows<'a>(doc: &'a str, from: &str, to: &str) -> Vec<Vec<&'a str>> {
        let section = doc.split(from).nth(1).expect("section");
        let section = section.split(to).next().expect("section");
        section
            .lines()
            .filter(|l| l.starts_with("| `"))
            .map(|l| l.split('|').map(str::trim).collect())
            .collect()
    }

    #[test]
    fn docs_messages_md_gives_every_hint_key_in_english_and_german() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let rows = rows(doc, "\n## Hint reasons", "\n### Hint rules");
        let documented: Vec<&str> = rows.iter().map(|r| r[1].trim_matches('`')).collect();
        assert_eq!(documented, KEYS, "same keys, same order");
        for r in &rows {
            assert!(!r[3].is_empty() && !r[4].is_empty(), "{}", r[1]);
        }
    }

    #[test]
    fn docs_messages_md_gives_every_rule_with_its_basis() {
        let doc = include_str!("../../../docs/MESSAGES.md");
        let rows = rows(doc, "\n### Hint rules", "\n## Review keys");
        let documented: Vec<&str> = rows.iter().map(|r| r[1].trim_matches('`')).collect();
        assert_eq!(documented, RULES, "same rules, same order");
        for r in &rows {
            assert!(!r[2].is_empty() && !r[3].is_empty(), "{}", r[1]);
            assert!(
                r[4].contains("research/") || r[4].contains("RULES"),
                "{}",
                r[1]
            );
        }
    }

    /// The tutor's cost over the opponent's own move, at the top rung:
    /// `cargo test -p strohmandeln-ai --release hint_latency -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn hint_latency() {
        let level = Level::from_halves(8);
        let config = Config::default();
        let (mut decide, mut whole, mut n) = (0.0, 0.0, 0);
        walk(3, |s, seat| {
            let v = view(s, seat);
            let k = Knowledge::from_view(&v).unwrap();
            if !k.playing || k.legal.len() < 2 || n >= 8 {
                return;
            }
            let t = std::time::Instant::now();
            Opponent::new(level, 9).with_config(config).decide(&v);
            decide += t.elapsed().as_secs_f64();
            let t = std::time::Instant::now();
            hint(&v, 9, level, config);
            whole += t.elapsed().as_secs_f64();
            n += 1;
        });
        let ms = |x: f64| x / f64::from(n) * 1e3;
        eprintln!(
            "n={n} decide {:.0} ms, whole hint {:.0} ms",
            ms(decide),
            ms(whole)
        );
    }

    #[test]
    fn the_keys_are_distinct_and_kebab_case() {
        for list in [KEYS, RULES] {
            let set: std::collections::HashSet<_> = list.iter().collect();
            assert_eq!(set.len(), list.len());
            for k in list {
                assert!(k.chars().all(|c| c.is_ascii_lowercase() || c == '-'), "{k}");
            }
        }
    }
}
