//! Drawing the table as text, from a `View` alone, and telling events in
//! words. Nothing here can show more than the viewer may know: it is handed
//! the view, never the deal.

use crate::text::{self, card, cards, t, thirds, ts, who, Lang};
use strohmandeln_core::cards::Card;
use strohmandeln_core::types::{Action, Cause, Event, GameResult, Seat, Source, UltimoResult};
use strohmandeln_core::view::{DealView, View};

/// Where a card the viewer may play comes from, read from the view.
pub fn source_of(deal: &DealView, seat: Seat, c: Card) -> Source {
    if deal.own_hand.iter().any(|h| h.card == c) {
        return Source::Hand;
    }
    (0..3)
        .find(|&p| deal.covers[seat.index()][p] == Some(c))
        .map(|p| Source::Packet(p as u8 + 1))
        .unwrap_or(Source::Hand)
}

fn source_text(lang: Lang, s: Source) -> String {
    match s {
        Source::Hand => t(lang, "from-hand", &[]),
        Source::Packet(n) => t(lang, "from-packet", &[&n.to_string()]),
    }
}

/// The numbered choices of a view: each legal action with its label.
pub fn choices(lang: Lang, view: &View) -> Vec<(Action, String)> {
    let touren = view.touren_total.to_string();
    // Bids and announcements first, then the plays in card order.
    let mut actions = view.legal_actions.clone();
    actions.sort_by_key(|a| match a {
        Action::Play { card, .. } => (1, card.index()),
        _ => (0, 0),
    });
    actions
        .iter()
        .map(|a| {
            let label = match a {
                Action::Take => t(lang, "act-take", &[]),
                Action::Pass => t(lang, "act-pass", &[]),
                Action::Announce { honour } => {
                    t(lang, "act-announce", &[&text::honour(lang, *honour)])
                }
                Action::Play { card: c, .. } => {
                    let src = view
                        .deal
                        .as_ref()
                        .map(|d| source_of(d, view.seat, *c))
                        .unwrap_or(Source::Hand);
                    t(
                        lang,
                        "act-play",
                        &[&card(lang, *c), &source_text(lang, src)],
                    )
                }
                Action::Extend { accept: true } => t(lang, "act-extend-yes", &[&touren]),
                Action::Extend { accept: false } => t(lang, "act-extend-no", &[]),
            };
            (a.clone(), label)
        })
        .collect()
}

fn packets(lang: Lang, deal: &DealView, owner: Seat) -> String {
    (0..3)
        .map(|p| {
            let cover = deal.covers[owner.index()][p]
                .map(|c| card(lang, c))
                .unwrap_or_else(|| "--".into());
            let hidden = deal.hidden[owner.index()][p];
            let tail = if hidden == 0 && cover == "--" {
                t(lang, "empty", &[])
            } else {
                t(lang, "hidden", &[&hidden.to_string()])
            };
            format!(
                "{} [{}] {}",
                t(lang, "packet", &[&(p + 1).to_string()]),
                cover,
                tail
            )
        })
        .collect::<Vec<_>>()
        .join("   ")
}

/// The table for the viewer: opponent's side, the trick, the viewer's side,
/// the count in thirds and the slate.
pub fn table(lang: Lang, view: &View) -> String {
    let me = view.seat;
    let them = me.other();
    let mut out = String::new();
    let line = |out: &mut String, s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    if let Some(deal) = &view.deal {
        let doubled = if deal.multiplier > 1 {
            t(lang, "doubled", &[])
        } else {
            String::new()
        };
        line(
            &mut out,
            format!(
                "== {} ==",
                t(
                    lang,
                    "deal-line",
                    &[
                        &(view.deals_counted + 1).to_string(),
                        &(view.touren_total * 2).to_string(),
                        &(view.deals_counted / 2 + 1).to_string(),
                        &doubled
                    ]
                )
            ),
        );
        if let Some(d) = deal.declarer {
            line(
                &mut out,
                t(
                    lang,
                    if d == me {
                        "declarer-you"
                    } else {
                        "declarer-them"
                    },
                    &[],
                ),
            );
        }
        let known = if deal.opponent_known.is_empty() {
            String::new()
        } else {
            format!(
                " ({})",
                t(lang, "known", &[&cards(lang, &deal.opponent_known)])
            )
        };
        line(
            &mut out,
            format!(
                "{}: {}{}",
                t(lang, "their-hand", &[]),
                t(lang, "cards", &[&deal.opponent_hand_size.to_string()]),
                known
            ),
        );
        line(
            &mut out,
            format!(
                "{}: {}",
                t(lang, "their-packets", &[]),
                packets(lang, deal, them)
            ),
        );
        let trick = match &deal.lead {
            Some(l) => t(lang, "led", &[&who(lang, l.seat, me), &card(lang, l.card)]),
            None => t(lang, "no-lead", &[]),
        };
        line(
            &mut out,
            format!(
                "{}: {}",
                t(lang, "trick", &[&deal.trick_number.to_string()]),
                trick
            ),
        );
        line(
            &mut out,
            format!(
                "{}: {}",
                t(lang, "your-packets", &[]),
                packets(lang, deal, me)
            ),
        );
        let mut hand: Vec<Card> = deal.own_hand.iter().map(|h| h.card).collect();
        hand.sort();
        line(
            &mut out,
            format!("{}: {}", t(lang, "your-hand", &[]), cards(lang, &hand)),
        );
        line(
            &mut out,
            t(
                lang,
                "count",
                &[
                    &thirds(deal.take_thirds[me.index()]),
                    &thirds(deal.take_thirds[them.index()]),
                ],
            ),
        );
    }
    line(
        &mut out,
        t(
            lang,
            "slate",
            &[
                &view.contos[me.index()].to_string(),
                &view.contos[them.index()].to_string(),
                &format!("{:+}", view.balance_for(me)),
            ],
        ),
    );
    out
}

trait BalanceFor {
    fn balance_for(&self, seat: Seat) -> i32;
}

impl BalanceFor for View {
    /// The balance is seat A's lead over seat B (`contos[0] - contos[1]`).
    fn balance_for(&self, seat: Seat) -> i32 {
        match seat {
            Seat::A => self.balance,
            Seat::B => -self.balance,
        }
    }
}

fn result_key(r: GameResult) -> &'static str {
    match r {
        GameResult::DeclarerWin => "result-declarer-win",
        GameResult::DeclarerLoss => "result-declarer-loss",
        GameResult::SimpleWin => "result-simple-win",
        GameResult::Draw => "result-draw",
    }
}

fn cause_key(c: Cause) -> &'static str {
    match c {
        Cause::Game => "cause-game",
        Cause::Trull => "cause-trull",
        Cause::Kings => "cause-kings",
        Cause::UltimoWon => "cause-ultimo-won",
        Cause::UltimoLost => "cause-ultimo-lost",
        Cause::Valat => "cause-valat",
    }
}

/// An event in words, or nothing for those the table shows anyway (the deal
/// of one's own cards).
pub fn narrate(lang: Lang, e: &Event, me: Seat) -> Option<String> {
    let sub = |key: &str, seat: Seat, args: &[&str]| Some(ts(lang, key, seat, me, args));
    let c = |x: Card| card(lang, x);
    match e {
        Event::SittingStart {
            first_dealer,
            touren,
            ..
        } => Some(t(
            lang,
            "ev-sitting-start",
            &[&touren.to_string(), &who(lang, *first_dealer, me)],
        )),
        Event::VoidDeal { .. } => Some(t(lang, "ev-void-deal", &[])),
        Event::Bid { seat } => sub("ev-bid", *seat, &[]),
        Event::Pass { seat } => sub("ev-pass", *seat, &[]),
        Event::BothPass { mode } => Some(t(
            lang,
            match mode {
                strohmandeln_core::types::BothPass::Play => "ev-both-pass-play",
                strohmandeln_core::types::BothPass::ThrowIn => "ev-both-pass-throw-in",
            },
            &[],
        )),
        Event::ThrownIn => Some(t(lang, "ev-thrown-in", &[])),
        Event::PacketTurned { seat, packet, card } => {
            sub("ev-packet-turned", *seat, &[&c(*card), &packet.to_string()])
        }
        Event::CardPulled {
            seat, packet, card, ..
        } => sub("ev-card-pulled", *seat, &[&c(*card), &packet.to_string()]),
        // The card turned up already told it; the cover is that card.
        Event::CoverSet { .. } => None,
        Event::Announcement { seat, honour, .. } => {
            sub("ev-announce", *seat, &[&text::honour(lang, *honour)])
        }
        Event::HonourForfeited { seat, honour } => {
            sub("ev-forfeited", *seat, &[&text::honour(lang, *honour)])
        }
        Event::CardPlayed { seat, card, .. } => sub("ev-play", *seat, &[&c(*card)]),
        Event::TrickWon { winner, cards, .. } => {
            sub("ev-trick", *winner, &[&text::cards(lang, cards)])
        }
        Event::PagatUltimo { seat, result } => sub(
            match result {
                UltimoResult::Won => "ev-ultimo-won",
                UltimoResult::Lost => "ev-ultimo-lost",
            },
            *seat,
            &[],
        ),
        Event::Valat { seat } => sub("ev-valat", *seat, &[]),
        Event::Count { thirds: th, .. } => Some(t(
            lang,
            "ev-count",
            &[&thirds(th[me.index()]), &thirds(th[me.other().index()])],
        )),
        Event::GameScored {
            result, credits, ..
        } => {
            let mut s = t(lang, "ev-scored", &[&t(lang, result_key(*result), &[])]);
            for cr in credits {
                s.push_str("\n  ");
                s.push_str(&t(
                    lang,
                    "ev-credit",
                    &[
                        &who(lang, cr.seat, me),
                        &t(lang, cause_key(cr.cause), &[]),
                        &cr.points.to_string(),
                    ],
                ));
            }
            Some(s)
        }
        Event::DoublingSet => Some(t(lang, "ev-doubling-set", &[])),
        Event::DoublingApplied { .. } => Some(t(lang, "ev-doubling-applied", &[])),
        Event::DoublingLapsed => Some(t(lang, "ev-doubling-lapsed", &[])),
        Event::TourEnd {
            tour,
            contos,
            balance,
        } => {
            let bal = if me == Seat::A { *balance } else { -*balance };
            Some(t(
                lang,
                "ev-tour-end",
                &[
                    &tour.to_string(),
                    &contos[me.index()].to_string(),
                    &contos[me.other().index()].to_string(),
                    &format!("{bal:+}"),
                ],
            ))
        }
        Event::ExtensionOffered => Some(t(lang, "ev-extension-offered", &[])),
        Event::ExtensionDecided {
            accepted,
            touren_added,
        } => Some(if *accepted {
            t(lang, "ev-extension-accepted", &[&touren_added.to_string()])
        } else {
            t(lang, "ev-extension-declined", &[])
        }),
        Event::SittingEnd { payer, kreuzer, .. } => Some(match payer {
            Some(p) if *p == me => t(lang, "ev-settle-you-pay", &[&kreuzer.to_string()]),
            Some(_) => t(lang, "ev-settle-they-pay", &[&kreuzer.to_string()]),
            None => t(lang, "ev-settle-even", &[]),
        }),
        Event::DealStart { .. } | Event::Dealt { .. } | Event::RevealStart { .. } => None,
        Event::PacketEmpty { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strohmandeln_core::session::Sitting;
    use strohmandeln_core::types::Settings;
    use strohmandeln_core::view::view;

    fn sitting() -> Sitting {
        Sitting::new(4, Settings::default(), 3).unwrap()
    }

    #[test]
    fn the_table_shows_hand_packets_counts_and_slate_in_both_languages() {
        let s = sitting();
        let seat = s.to_move().unwrap();
        let v = view(&s, seat);
        let en = table(Lang::En, &v);
        for want in [
            "Your hand",
            "Your straw men",
            "Opponent's straw men",
            "15 cards",
            "Count (thirds)",
            "Slate",
        ] {
            assert!(en.contains(want), "{want} in\n{en}");
        }
        let de = table(Lang::De, &v);
        for want in [
            "Deine Hand",
            "Deine Strohmänner",
            "15 Karten",
            "Zählung",
            "Schiefertafel",
        ] {
            assert!(de.contains(want), "{want} in\n{de}");
        }
    }

    #[test]
    fn the_table_never_prints_the_opponents_hidden_cards() {
        let s = sitting();
        let me = Seat::A;
        let v = view(&s, me);
        let shown = table(Lang::En, &v);
        let deal = s.deal().unwrap();
        // Face-down packet cards and the other hand are not in the picture.
        let mut secret: Vec<Card> = deal.hand_cards(me.other());
        for p in 1..=3 {
            let all = deal.packet_cards(me.other(), p);
            secret.extend(all.into_iter().skip(1));
            let mine = deal.packet_cards(me, p);
            secret.extend(mine.into_iter().skip(1));
        }
        let visible: Vec<Card> = v
            .deal
            .as_ref()
            .unwrap()
            .covers
            .iter()
            .flatten()
            .flatten()
            .copied()
            .chain(v.deal.as_ref().unwrap().own_hand.iter().map(|h| h.card))
            .collect();
        for c in secret.into_iter().filter(|c| !visible.contains(c)) {
            let label = card(Lang::En, c);
            // A label can be shared by visible words only as a substring of
            // another card; compare whole tokens.
            let tokens: Vec<&str> = shown
                .split(|ch: char| ch.is_whitespace() || "[]():,".contains(ch))
                .collect();
            assert!(
                !tokens.contains(&label.as_str()),
                "{label} leaked:\n{shown}"
            );
        }
    }

    #[test]
    fn choices_name_the_source_of_each_playable_card() {
        let mut s = sitting();
        while s.deal().is_some_and(|d| !d.is_playing()) {
            let seat = s.to_move().unwrap();
            s.apply(seat, Action::Take).ok();
        }
        let seat = s.to_move().unwrap();
        let v = view(&s, seat);
        let cs = choices(Lang::En, &v);
        assert_eq!(cs.len(), v.legal_actions.len());
        assert!(cs.iter().all(|(_, l)| l.contains("from hand")
            || l.contains("from packet")
            || l.starts_with("Announce")));
    }

    #[test]
    fn every_event_of_a_whole_sitting_is_told_in_both_languages() {
        use strohmandeln_core::agents::{play_out, FirstLegal, Player};
        let mut s = sitting();
        let mut players: [Box<dyn Player>; 2] = [Box::new(FirstLegal), Box::new(FirstLegal)];
        play_out(&mut s, &mut players, 100_000).unwrap();
        let mut told = 0;
        for e in s.events() {
            for lang in [Lang::En, Lang::De] {
                if let Some(line) = narrate(lang, e, Seat::A) {
                    assert!(!line.is_empty());
                    assert!(!line.contains("ev-"), "missing string for {e:?}: {line}");
                    told += 1;
                }
            }
        }
        assert!(told > 100);
    }
}
