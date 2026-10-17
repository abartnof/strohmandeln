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

use crate::ask::{alternative, compose, Evidence, Fact, Line};
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
    /// The decisive question of `docs/HINTS.md` 2 and the card it is about
    /// (`ask.keep`); `None` for a forced card, a bid, or when no question
    /// is decisive.
    pub question: Option<&'static str>,
    pub keep: Option<Card>,
    /// The sentence after the question: a passed rule (`so`), else a fact
    /// (`ans`). Both `None` for a bid: the table then gives `key`'s reason.
    pub so: Option<Line>,
    pub ans: Option<Line>,
    pub why: Why,
}

/// What the "Why?" shows (`docs/HINTS.md` 3.1, 4, 5.5).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Why {
    /// The natural alternative A.
    pub alt: Option<Card>,
    /// X against A over the sampled deals (`vs.lead`, `vs.follow`), and for a
    /// lead how often he trumps each (`vs.lead-trumped`).
    pub contrast: Option<Line>,
    pub trumped: Option<Line>,
    pub facts: Vec<Fact>,
}

impl Hint {
    fn plain(action: Action, key: &'static str) -> Hint {
        Hint {
            action,
            key,
            params: Vec::new(),
            rule: None,
            odds: None,
            question: None,
            keep: None,
            so: None,
            ans: None,
            why: Why::default(),
        }
    }
}

/// Every sentence key of the default hint and the "Why?" (`docs/MESSAGES.md`,
/// "Hint sentences"), besides the `reason.*` keys of [`KEYS`].
pub const SENTENCES: &[&str] = &[
    "ans.keep-mond",
    "ans.keep-mond-safe",
    "ans.keep-pagat",
    "ans.keep-skus",
    "ans.beneath",
    "ans.worth-poor",
    "ans.worth-rich",
    "ans.cheap",
    "ans.cheap-lose",
    "ans.ask-differ",
    "ans.ask-none",
    "ans.out-void",
    "ans.out-master",
    "ans.out-master-tarock",
    "ans.suit",
    "ans.trumps",
    "so.worth-rich",
    "fact.tarocks-out",
    "fact.skus-out",
    "fact.his-void",
    "fact.master",
    "fact.split",
    "fact.beneath",
    "vs.lead",
    "vs.lead-trumped",
    "vs.follow",
];

fn sentence(key: &str, params: &[(&'static str, Param)]) -> Line {
    Line {
        key: key.to_string(),
        params: params.to_vec(),
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

/// Worlds in the evidence for the search's ranking. Each is searched as
/// deeply as one of the decision's own (`odds_config`). Measured in
/// `docs/OPPONENT.md`, "Hint evidence".
pub const ODDS_WORLDS: usize = 50;

/// Worlds a lead's rate is counted over ("takes the trick in N of
/// {LEAD_WORLDS} sampled deals"). The opponent's own lead weighs
/// `rung3::WORLDS`; this is only the evidence the learner is shown.
pub const LEAD_WORLDS: usize = 1000;

/// `config` with the node budget grown to `worlds`, at the share per world the
/// decision has, so more worlds are not shallower worlds.
pub fn odds_config(config: Config, worlds: usize) -> Config {
    let mut c = config;
    c.budget.nodes =
        (config.budget.nodes / config.budget.worlds.max(1) as u64).saturating_mul(worlds as u64);
    c
}

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
            params: vec![
                ("sure_thirds", Param::Int(i64::from(sure))),
                ("tarocks", Param::Int(tarocks as i64)),
            ],
            rule: Some("bid"),
            ..Hint::plain(action, key)
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
        rung3::lead_sims(&k, &mut srng, LEAD_WORLDS)
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
    let alt = alternative(&k, card).filter(|_| k.legal.len() > 1);
    let (odds, follow) = if rung >= 4 && k.legal.len() > 1 {
        let mut orng = Rng::stream(seed, view_seed(view) ^ 0x4812);
        odds_and_contrast(
            &k,
            card,
            alt,
            &mut orng,
            odds_config(config, ODDS_WORLDS),
            ODDS_WORLDS,
        )
    } else {
        (None, None)
    };
    let evidence = Evidence {
        differ: follow.map(|f| (f.better, f.alt_better, f.worlds)),
        none: odds.as_ref().map(|o| (o.better, o.worlds, o.gain_thirds)),
    };
    let comp = compose(&k, card, &evidence);
    // A hint always has a "because": the plain local reason stands where the
    // question has no sentence of its own.
    let ans = comp.ans.or_else(|| {
        Some(Line {
            key: format!("reason.{key}"),
            params: params.clone(),
        })
    });
    let (contrast, trumped) = match (alt, k.lead) {
        (Some(a), Some(_)) => (
            follow.map(|f| {
                sentence(
                    "vs.follow",
                    &[
                        ("card", Param::Card(card)),
                        ("alt", Param::Card(a)),
                        ("w", Param::Int(i64::from(f.worlds))),
                        ("b", Param::Int(i64::from(f.better))),
                        ("a", Param::Int(i64::from(f.alt_better))),
                    ],
                )
            }),
            None,
        ),
        (Some(a), None) if !sims.is_empty() => {
            let n = |v: usize| Param::Int(v as i64);
            let worlds = n(sims.len());
            let contrast = sentence(
                "vs.lead",
                &[
                    ("card", Param::Card(card)),
                    ("alt", Param::Card(a)),
                    ("worlds", worlds),
                    ("wins", n(wins(card))),
                    ("alt_wins", n(wins(a))),
                    ("cost_thirds", Param::Int(i64::from(card.thirds()))),
                    ("alt_cost_thirds", Param::Int(i64::from(a.thirds()))),
                ],
            );
            let trumped = (!card.is_tarock() || !a.is_tarock()).then(|| {
                sentence(
                    "vs.lead-trumped",
                    &[
                        ("card", Param::Card(card)),
                        ("alt", Param::Card(a)),
                        ("worlds", worlds),
                        ("trumped", n(rung3::lead_trumped(&sims, card))),
                        ("alt_trumped", n(rung3::lead_trumped(&sims, a))),
                    ],
                )
            });
            (Some(contrast), trumped)
        }
        _ => (None, None),
    };
    Hint {
        action,
        key,
        params,
        rule,
        odds,
        question: comp.question,
        keep: comp.keep,
        so: comp.so,
        ans,
        why: Why {
            alt,
            contrast,
            trumped,
            facts: comp.facts,
        },
    }
}

/// How the sample sees the hinted card against the alternative A, deal by
/// deal: better in `better`, A better in `alt_better`, of `worlds` (the rest
/// equal).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FollowContrast {
    pub better: u32,
    pub alt_better: u32,
    pub worlds: u32,
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
    odds_and_contrast(k, card, None, rng, config, worlds).0
}

/// [`odds_with`] and, from the same table of solved worlds, `card` against
/// `alt` (`docs/HINTS.md` 4: one `move_table` call).
pub fn odds_and_contrast(
    k: &Knowledge,
    card: Card,
    alt: Option<Card>,
    rng: &mut Rng,
    config: Config,
    worlds: usize,
) -> (Option<Odds>, Option<FollowContrast>) {
    let mut budget = config.budget;
    budget.worlds = worlds;
    let table = rung4::move_table(k, rng, budget, &config.weights);
    if table.per_world.is_empty() {
        return (None, None);
    }
    let value = |row: &[(Card, i32)], c: Card| row.iter().find(|(x, _)| *x == c).map(|(_, v)| *v);
    let contrast = alt.map(|a| {
        let count = |first: Card, second: Card| {
            table
                .per_world
                .iter()
                .filter(|row| matches!((value(row, first), value(row, second)), (Some(x), Some(y)) if x > y))
                .count() as u32
        };
        FollowContrast {
            better: count(card, a),
            alt_better: count(a, card),
            worlds: table.per_world.len() as u32,
        }
    });
    (best_other(&table, card), contrast)
}

fn best_other(table: &rung4::MoveTable, card: Card) -> Option<Odds> {
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
            let again = odds_with(
                &k,
                *card,
                &mut rng,
                odds_config(config, ODDS_WORLDS),
                ODDS_WORLDS,
            );
            assert_eq!(again.as_ref(), Some(odds));
            assert!(odds.worlds as usize <= ODDS_WORLDS);
            checked += 1;
        });
        assert!(checked > 0);
    }

    #[test]
    fn the_odds_search_keeps_the_depth_per_world_of_the_decision() {
        // More worlds must not mean shallower worlds: the node budget grows
        // with them, at the decision's own share per world (30 000 / 10).
        let config = Config::default();
        let share = config.budget.nodes / config.budget.worlds as u64;
        for worlds in [10usize, 50, 200] {
            let c = odds_config(config, worlds);
            assert_eq!(c.budget.nodes, share * worlds as u64, "{worlds}");
            assert_eq!(c.weights, config.weights);
        }
        assert_eq!(odds_config(config, 50).budget.nodes, 150_000);
        let mut huge = config;
        huge.budget.nodes = u64::MAX;
        assert_eq!(odds_config(huge, 50).budget.nodes, u64::MAX, "saturates");
        assert_eq!(ODDS_WORLDS, 50);
    }

    #[test]
    fn a_lead_s_evidence_is_over_the_hint_s_many_worlds_and_the_move_is_the_opponent_s() {
        // The evidence sample is its own stream and size: the lead-rate line
        // counts LEAD_WORLDS, and the move is still what the opponent plays
        // with its own 24 worlds.
        let level = Level::from_halves(8);
        let config = small();
        let mut seen = 0;
        for seed in 0..3 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let h = hint(&v, 9, level, config);
                assert_eq!(
                    h.action,
                    Opponent::new(level, 9).with_config(config).decide(&v)
                );
                if h.key == "lead-rate" || h.key == "lead-cheap" {
                    let worlds = h.params.iter().find(|(n, _)| *n == "worlds").unwrap();
                    assert_eq!(worlds.1, Param::Int(LEAD_WORLDS as i64), "{h:?}");
                    seen += 1;
                }
            });
        }
        assert!(seen > 0, "no lead carried a rate");
        assert_eq!(LEAD_WORLDS, 1000);
        assert_eq!(
            rung3::WORLDS,
            24,
            "the opponent's own play is not the hint's"
        );
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
                    (
                        h.action, h.key, h.params, h.rule, h.odds, h.question, h.keep, h.so, h.ans,
                        h.why,
                    )
                },
                6,
            );
        }
        assert!(twins > 150, "{twins}");
    }

    #[test]
    fn every_play_hint_has_a_because_and_known_sentence_keys() {
        let level = Level::from_halves(8);
        let (mut asked, mut contrasts, mut none) = (0, 0, 0);
        for seed in 0..3 {
            walk(seed, |s, seat| {
                let v = view(s, seat);
                let h = hint(&v, 9, level, small());
                let Action::Play { .. } = &h.action else {
                    assert!(h.ans.is_none() && h.question.is_none(), "{h:?}");
                    return;
                };
                let ans = h.ans.as_ref().expect("a because");
                let mut lines: Vec<&Line> = vec![ans];
                lines.extend(h.so.iter());
                lines.extend(h.why.contrast.iter().chain(h.why.trumped.iter()));
                lines.extend(h.why.facts.iter().map(|f| &f.line));
                for l in lines {
                    let known = match l.key.split_once('.') {
                        Some(("reason", r)) => KEYS.contains(&r),
                        _ => SENTENCES.contains(&l.key.as_str()),
                    };
                    assert!(known, "{}", l.key);
                }
                if let Some(q) = h.question {
                    assert!(crate::ask::QUESTIONS.contains(&q) && q != "ask.beat");
                    assert_eq!(q == "ask.keep", h.keep.is_some(), "{h:?}");
                    asked += 1;
                } else {
                    none += 1;
                }
                // A hint with a rule has a question.
                assert!(h.so.is_none() || h.question.is_some());
                contrasts += usize::from(h.why.contrast.is_some());
                // A is another legal card whenever one exists.
                let k = Knowledge::from_view(&v).unwrap();
                assert_eq!(h.why.alt.is_some(), k.legal.len() > 1, "{h:?}");
                assert_ne!(
                    h.why.alt,
                    h.params
                        .iter()
                        .find(|(n, _)| *n == "card")
                        .and_then(|(_, p)| match p {
                            Param::Card(c) => Some(*c),
                            _ => None,
                        })
                );
            });
        }
        assert!(asked > 10 && contrasts > 10, "{asked} {contrasts} {none}");
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
