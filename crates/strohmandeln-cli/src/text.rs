//! Every string of the terminal client, in English and German, in one place.
//! The engine emits message keys with parameters (`docs/MESSAGES.md`); this
//! table turns them into words. `{0}`, `{1}` ... stand for the parameters.

use strohmandeln_core::cards::{Card, Honour, Suit};
use strohmandeln_core::types::{Refusal, Seat};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    De,
}

impl Lang {
    pub fn parse(s: &str) -> Option<Lang> {
        match s {
            "en" => Some(Lang::En),
            "de" => Some(Lang::De),
            _ => None,
        }
    }
}

/// (key, English, German). Refusal keys are those of `docs/MESSAGES.md`.
pub const STRINGS: &[(&str, &str, &str)] = &[
    // Interface
    ("you", "You", "Du"),
    ("opponent", "The opponent", "Der Gegner"),
    ("your-hand", "Your hand", "Deine Hand"),
    ("their-hand", "Opponent's hand", "Hand des Gegners"),
    ("cards", "{0} cards", "{0} Karten"),
    ("known", "known: {0}", "bekannt: {0}"),
    ("your-packets", "Your straw men", "Deine Strohmänner"),
    ("their-packets", "Opponent's straw men", "Strohmänner des Gegners"),
    ("packet", "packet {0}", "Packerl {0}"),
    ("hidden", "{0} face down", "{0} verdeckt"),
    ("empty", "empty", "leer"),
    ("trick", "Trick {0}", "Stich {0}"),
    ("led", "{0} led {1}", "{0} spielt {1} aus"),
    ("no-lead", "no card led", "noch keine Karte ausgespielt"),
    ("count", "Count (thirds): you {0}, opponent {1} of 70", "Zählung (Drittel): du {0}, Gegner {1} von 70"),
    ("slate", "Slate: you {0}, opponent {1}, balance {2}", "Schiefertafel: du {0}, Gegner {1}, Saldo {2}"),
    ("deal-line", "Deal {0} of {1}, Tour {2}{3}", "Spiel {0} von {1}, Tour {2}{3}"),
    ("doubled", ", doubled", ", doppelt"),
    ("declarer-you", "You have taken the game.", "Du hast das Spiel angesagt."),
    ("declarer-them", "The opponent has taken the game.", "Der Gegner hat das Spiel angesagt."),
    ("choose", "Your move (number, card or q to quit): ", "Dein Zug (Nummer, Karte oder q zum Beenden): "),
    ("bad-choice", "Not one of the choices.", "Das steht nicht zur Wahl."),
    ("quit", "Sitting abandoned.", "Sitzung abgebrochen."),
    ("act-take", "Take the game", "Das Spiel nehmen"),
    ("act-pass", "Pass", "Weitergeben"),
    ("act-announce", "Announce {0}", "{0} ansagen"),
    ("act-play", "Play {0} (from {1})", "{0} spielen (aus {1})"),
    ("act-extend-yes", "Play another {0} Touren", "Noch {0} Touren spielen"),
    ("act-extend-no", "Settle now", "Jetzt abrechnen"),
    ("from-hand", "hand", "der Hand"),
    ("from-packet", "packet {0}", "Packerl {0}"),
    ("trull", "the Troule", "die Troule"),
    ("kings", "the four kings", "die vier Könige"),
    ("hearts", "hearts", "Herz"),
    ("diamonds", "diamonds", "Karo"),
    ("clubs", "clubs", "Kreuz"),
    ("spades", "spades", "Pik"),
    ("trump", "tarock", "Tarock"),
    ("skus", "Sküs", "Sküs"),
    ("usage", "Usage: strohmandeln [--lang en|de] [--seed N] [--touren 3|6] [--seat 0|1] [--level N]", "Aufruf: strohmandeln [--lang en|de] [--seed N] [--touren 3|6] [--seat 0|1] [--level N]"),
    // Events
    ("ev-sitting-start", "A sitting of {0} Touren begins. Dealer: {1}.", "Eine Sitzung von {0} Touren beginnt. Geber: {1}."),
    ("ev-void-deal", "A hand without a tarock: the deal is void and dealt again.", "Eine Hand ohne Tarock: neu gegeben."),
    ("ev-bid/you", "You take the game.", "Du nimmst das Spiel."),
    ("ev-bid/them", "The opponent takes the game.", "Der Gegner nimmt das Spiel."),
    ("ev-pass/you", "You pass.", "Du gibst weiter."),
    ("ev-pass/them", "The opponent passes.", "Der Gegner gibt weiter."),
    ("ev-both-pass-play", "Both passed: the deal is played with no declarer.", "Beide weiter: gespielt wird ohne Spielführer."),
    ("ev-both-pass-throw-in", "Both passed: the deal is thrown in.", "Beide weiter: das Spiel wird eingeworfen."),
    ("ev-thrown-in", "The deal is thrown in.", "Das Spiel ist eingeworfen."),
    ("ev-packet-turned/you", "You turn {0} up on packet {1}.", "Du deckst {0} auf Packerl {1} auf."),
    ("ev-packet-turned/them", "The opponent turns {0} up on packet {1}.", "Der Gegner deckt {0} auf Packerl {1} auf."),
    ("ev-card-pulled/you", "You take {0} from packet {1} into your hand.", "Du nimmst {0} aus Packerl {1} in die Hand."),
    ("ev-card-pulled/them", "The opponent takes {0} from packet {1} into the hand.", "Der Gegner nimmt {0} aus Packerl {1} in die Hand."),
    ("ev-announce/you", "You announce {0}.", "Du sagst {0} an."),
    ("ev-announce/them", "The opponent announces {0}.", "Der Gegner sagt {0} an."),
    ("ev-forfeited/you", "You lose {0} for this deal.", "Du verlierst {0} für dieses Spiel."),
    ("ev-forfeited/them", "The opponent loses {0} for this deal.", "Der Gegner verliert {0} für dieses Spiel."),
    ("ev-play/you", "You play {0}.", "Du spielst {0}."),
    ("ev-play/them", "The opponent plays {0}.", "Der Gegner spielt {0}."),
    ("ev-trick/you", "You win the trick ({0}).", "Du gewinnst den Stich ({0})."),
    ("ev-trick/them", "The opponent wins the trick ({0}).", "Der Gegner gewinnt den Stich ({0})."),
    ("ev-ultimo-won/you", "You win with the Pagat in the last trick: Pagat ultimo!", "Du gewinnst mit dem Pagat im letzten Stich: Pagat ultimo!"),
    ("ev-ultimo-won/them", "The opponent wins with the Pagat in the last trick: Pagat ultimo!", "Der Gegner gewinnt mit dem Pagat im letzten Stich: Pagat ultimo!"),
    ("ev-ultimo-lost/you", "You lose the Pagat in the last trick.", "Du verlierst den Pagat im letzten Stich."),
    ("ev-ultimo-lost/them", "The opponent loses the Pagat in the last trick.", "Der Gegner verliert den Pagat im letzten Stich."),
    ("ev-valat/you", "You take every trick: Valat!", "Du machst alle Stiche: Valat!"),
    ("ev-valat/them", "The opponent takes every trick: Valat!", "Der Gegner macht alle Stiche: Valat!"),
    ("ev-count", "Count: you {0}, opponent {1} (of 70).", "Zählung: du {0}, Gegner {1} (von 70)."),
    ("ev-scored", "Deal scored: {0}.", "Spiel gewertet: {0}."),
    ("ev-credit", "{0}: {1} {2}", "{0}: {1} {2}"),
    ("ev-doubling-set", "The next deal will be doubled.", "Das nächste Spiel wird verdoppelt."),
    ("ev-doubling-applied", "This deal counts double.", "Dieses Spiel zählt doppelt."),
    ("ev-doubling-lapsed", "The doubling lapses.", "Die Verdoppelung verfällt."),
    ("ev-tour-end", "Tour {0} over. Slate: you {1}, opponent {2}, balance {3}.", "Tour {0} zu Ende. Tafel: du {1}, Gegner {2}, Saldo {3}."),
    ("ev-extension-offered", "The sitting may be extended.", "Die Sitzung kann verlängert werden."),
    ("ev-extension-accepted", "Extended by {0} Touren.", "Um {0} Touren verlängert."),
    ("ev-extension-declined", "No extension.", "Keine Verlängerung."),
    ("ev-settle-you-pay", "Settlement: you pay {0} kreuzer.", "Abrechnung: du zahlst {0} Kreuzer."),
    ("ev-settle-they-pay", "Settlement: the opponent pays you {0} kreuzer.", "Abrechnung: der Gegner zahlt dir {0} Kreuzer."),
    ("ev-settle-even", "Settlement: square, nobody pays.", "Abrechnung: ausgeglichen, keiner zahlt."),
    ("result-declarer-win", "declarer wins", "Spielführer gewinnt"),
    ("result-declarer-loss", "declarer loses", "Spielführer verliert"),
    ("result-simple-win", "simple game won", "einfaches Spiel gewonnen"),
    ("result-draw", "drawn", "unentschieden"),
    ("cause-game", "game", "Spiel"),
    ("cause-trull", "Troule", "Troule"),
    ("cause-kings", "kings", "Könige"),
    ("cause-ultimo-won", "Pagat ultimo", "Pagat ultimo"),
    ("cause-ultimo-lost", "Pagat lost", "Pagat verloren"),
    ("cause-valat", "Valat", "Valat"),
    // Refusals (docs/MESSAGES.md)
    ("not-your-turn", "It is not your turn.", "Du bist nicht am Zug."),
    ("wrong-phase", "That cannot be done now.", "Das geht jetzt nicht."),
    ("finished", "This deal is over.", "Dieses Spiel ist zu Ende."),
    ("not-playable", "{0} is not among your hand cards or uncovered packet cards.", "{0} liegt nicht in deiner Hand oder offen auf einem Packerl."),
    ("must-follow-suit", "You must follow suit ({1} was led); {0} is not allowed; play one of: {2}.", "Du musst Farbe bedienen ({1} wurde ausgespielt); {0} ist nicht erlaubt; spiele: {2}."),
    ("must-trump", "You hold nothing of the suit led, so you must play a tarock; {0} is not allowed; play one of: {1}.", "Du hast die Farbe nicht, also musst du Tarock spielen; {0} ist nicht erlaubt; spiele: {1}."),
    ("not-held", "You cannot announce {0}: you do not hold {1} in your hand now.", "Du kannst {0} nicht ansagen: {1} hast du nicht in der Hand."),
    ("already-announced", "You have already announced {0}.", "Du hast {0} schon angesagt."),
    ("honour-forfeited", "You played a card of {0} without announcing it, so it is lost for this deal.", "Du hast eine Karte von {0} unangesagt gespielt; sie ist für dieses Spiel verloren."),
    ("duplicate-announcement", "You listed {0} twice.", "Du hast {0} doppelt genannt."),
];

/// As `t`, for a string with a subject: looks up `key/you` or `key/them`.
pub fn ts(lang: Lang, key: &str, seat: Seat, human: Seat, args: &[&str]) -> String {
    let suffix = if seat == human { "you" } else { "them" };
    t(lang, &format!("{key}/{suffix}"), args)
}

/// Looks a key up and fills its parameters. An unknown key comes back as
/// itself, so a missing string shows rather than hides.
pub fn t(lang: Lang, key: &str, args: &[&str]) -> String {
    let Some(&(_, en, de)) = STRINGS.iter().find(|(k, _, _)| *k == key) else {
        return key.to_string();
    };
    let mut s = match lang {
        Lang::En => en,
        Lang::De => de,
    }
    .to_string();
    for (i, a) in args.iter().enumerate() {
        s = s.replace(&format!("{{{i}}}"), a);
    }
    s
}

pub fn honour(lang: Lang, h: Honour) -> String {
    t(
        lang,
        match h {
            Honour::Trull => "trull",
            Honour::Kings => "kings",
        },
        &[],
    )
}

const ROMAN: [&str; 21] = [
    "I", "II", "III", "IIII", "V", "VI", "VII", "VIII", "IX", "X", "XI", "XII", "XIII", "XIV",
    "XV", "XVI", "XVII", "XVIII", "XIX", "XX", "XXI",
];

fn suit_symbol(s: Suit) -> &'static str {
    match s {
        Suit::Hearts => "♥",
        Suit::Diamonds => "♦",
        Suit::Clubs => "♣",
        Suit::Spades => "♠",
    }
}

/// A card as printed: tarocks in Roman numerals (IIII as on real packs), the
/// Sküs by name, suit cards as rank and symbol; German court labels K D C B.
pub fn card(lang: Lang, c: Card) -> String {
    if c == Card::SKUS {
        return t(lang, "skus", &[]);
    }
    if c.is_tarock() {
        let code = c.code();
        let n: usize = code[1..].parse().unwrap_or(1);
        return ROMAN[n - 1].to_string();
    }
    let code = c.code();
    let rank = &code[..code.len() - 1];
    let rank = match (lang, rank) {
        (Lang::De, "Q") => "D",
        (Lang::De, "J") => "B",
        (_, r) => r,
    };
    format!("{rank}{}", suit_symbol(c.suit().expect("a suit card")))
}

pub fn cards(lang: Lang, cs: &[Card]) -> String {
    cs.iter()
        .map(|&c| card(lang, c))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A led-suit parameter (`h d c s` or `trump`) as a word.
fn led(lang: Lang, led: &str) -> String {
    t(
        lang,
        match led {
            "h" => "hearts",
            "d" => "diamonds",
            "c" => "clubs",
            "s" => "spades",
            _ => "trump",
        },
        &[],
    )
}

/// A refusal in words, from its key and parameters.
pub fn refusal(lang: Lang, r: &Refusal) -> String {
    let key = serde_key(r);
    let card_s = r.card.map(|c| card(lang, c)).unwrap_or_default();
    let allowed = cards(lang, &r.allowed);
    let h = r.honour.map(|h| honour(lang, h)).unwrap_or_default();
    let missing = cards(lang, &r.missing);
    let led_s = led(lang, r.led.as_deref().unwrap_or(""));
    match key.as_str() {
        "must-follow-suit" => t(lang, &key, &[&card_s, &led_s, &allowed]),
        "must-trump" => t(lang, &key, &[&card_s, &allowed]),
        "not-held" => t(lang, &key, &[&h, &missing]),
        "already-announced" | "honour-forfeited" | "duplicate-announcement" => t(lang, &key, &[&h]),
        _ => t(lang, &key, &[&card_s]),
    }
}

/// The wire key of a refusal (kebab-case), read back from its JSON.
fn serde_key(r: &Refusal) -> String {
    format!("{:?}", r.kind)
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['-', c.to_ascii_lowercase()]
            } else {
                vec![c.to_ascii_lowercase()]
            }
        })
        .collect()
}

/// "You" or "The opponent" for a seat, seen from `human`.
pub fn who(lang: Lang, seat: Seat, human: Seat) -> String {
    t(lang, if seat == human { "you" } else { "opponent" }, &[])
}

/// Thirds as a count: whole points and a fraction ("12 1/3").
pub fn thirds(n: u32) -> String {
    match n % 3 {
        0 => format!("{}", n / 3),
        r => format!("{} {r}/3", n / 3),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strohmandeln_core::types::RefusalKind;

    #[test]
    fn every_string_has_both_languages_and_matching_parameters() {
        for (key, en, de) in STRINGS {
            assert!(!en.is_empty() && !de.is_empty(), "{key}");
            for i in 0..4 {
                let p = format!("{{{i}}}");
                assert_eq!(en.contains(&p), de.contains(&p), "{key}: parameter {p}");
            }
        }
        let mut keys: Vec<_> = STRINGS.iter().map(|s| s.0).collect();
        keys.sort();
        let n = keys.len();
        keys.dedup();
        assert_eq!(keys.len(), n, "duplicate key");
    }

    #[test]
    fn every_refusal_key_of_the_messages_table_has_a_string() {
        for k in [
            "not-your-turn",
            "wrong-phase",
            "finished",
            "not-playable",
            "must-follow-suit",
            "must-trump",
            "not-held",
            "already-announced",
            "honour-forfeited",
            "duplicate-announcement",
        ] {
            assert!(STRINGS.iter().any(|s| s.0 == k), "{k}");
        }
    }

    #[test]
    fn the_refusal_key_is_the_wire_key() {
        let r = Refusal::of(RefusalKind::MustFollowSuit)
            .with_card(Card::MOND)
            .with_led("h")
            .with_allowed(vec![Card::PAGAT]);
        assert_eq!(serde_key(&r), "must-follow-suit");
        assert!(refusal(Lang::En, &r).contains("hearts"));
        assert!(refusal(Lang::De, &r).contains("Herz"));
    }

    #[test]
    fn cards_print_in_numerals_and_german_courts() {
        assert_eq!(card(Lang::En, Card::PAGAT), "I");
        assert_eq!(card(Lang::En, Card::tarock(4)), "IIII");
        assert_eq!(card(Lang::En, Card::MOND), "XXI");
        assert_eq!(card(Lang::En, Card::SKUS), "Sküs");
        let q = Card::suit_card(Suit::Hearts, 6);
        let j = Card::suit_card(Suit::Spades, 4);
        assert_eq!(card(Lang::En, q), "Q♥");
        assert_eq!(card(Lang::De, q), "D♥");
        assert_eq!(card(Lang::En, j), "J♠");
        assert_eq!(card(Lang::De, j), "B♠");
    }

    #[test]
    fn thirds_read_as_points_and_a_fraction() {
        assert_eq!(thirds(0), "0");
        assert_eq!(thirds(37), "12 1/3");
        assert_eq!(thirds(107), "35 2/3");
    }

    #[test]
    fn an_unknown_key_shows_itself() {
        assert_eq!(t(Lang::En, "no-such-key", &[]), "no-such-key");
    }
}
