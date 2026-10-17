//! Style: a fixed parameter set for the opponent, the school of a writer or a
//! temperament (`docs/STYLES.md` 2). The skill dial (`opponent.rs`) says what
//! the opponent knows; the style says what it tends to do. Both are fixed for
//! the sitting.
//!
//! Every parameter is tagged: **T** the text states it, **I** one inferential
//! step from the text, **E** emphasis or design (no text gives it). A writer
//! uses T and I only; a temperament may use E and claims no history. Sources:
//! W = Werner [03-S12], U = Ulmann [03-S14], Be = Bermann [03-S13], MS = Mayr
//! and Sedlaczek [07-S1]; page numbers as in `research/16`.
//!
//! Normal is every default (`Weights::default()`, `BID_RULE`), so a Normal
//! opponent plays bit for bit as before styles existed
//! (`golden_tests.rs`).

use crate::opponent::Config;
use crate::rung2::{BidExtra, BidRule, Queens, Weights, BID_RULE};

/// The opponent's style. `Random` is a choice made at the sitting's start
/// and never plays itself: [`Style::resolve`] gives the style drawn.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub enum Style {
    #[default]
    Normal,
    Random,
    Bermann,
    Ulmann,
    Ms,
    Werner,
    Bold,
    Cautious,
    Hunter,
}

/// The rung-4 maxim band of every style but Normal, in thirds: Normal's is
/// 0.34 (`rung4::best_card`). E (`docs/STYLES.md` 3); set against the style's
/// cost in the measurement of 4.3.
const BAND: f64 = 2.0;

/// Rung 4's bid margin for each style, in Points: the advantage at which
/// Normal's rung-4 sample bids the share of hands the style's rule bids
/// (`probe::Calibration`, `stage0`). Rung 4 bids by the sampled advantage, not
/// by `BidRule`. 0.0 is Normal's, and stands for "not yet calibrated" until
/// stage 0 has run.
const MARGINS: [(Style, f64); 7] = [
    (Style::Bermann, 0.0),
    (Style::Ulmann, 0.0),
    (Style::Ms, 0.0),
    (Style::Werner, 0.0),
    (Style::Bold, 0.0),
    (Style::Cautious, 0.0),
    (Style::Hunter, 0.0),
];

impl Style {
    /// Every style, in the dropdown's order.
    pub const ALL: [Style; 9] = [
        Style::Normal,
        Style::Random,
        Style::Bermann,
        Style::Ulmann,
        Style::Ms,
        Style::Werner,
        Style::Bold,
        Style::Cautious,
        Style::Hunter,
    ];

    /// What Random draws from: the candidates, until the gates of
    /// `docs/STYLES.md` 3 and 4.1 drop some. Never `Normal`.
    pub const POOL: [Style; 7] = [
        Style::Bermann,
        Style::Ulmann,
        Style::Ms,
        Style::Werner,
        Style::Bold,
        Style::Cautious,
        Style::Hunter,
    ];

    /// The id on the wire and in the record.
    pub fn id(self) -> &'static str {
        match self {
            Style::Normal => "normal",
            Style::Random => "random",
            Style::Bermann => "bermann",
            Style::Ulmann => "ulmann",
            Style::Ms => "ms",
            Style::Werner => "werner",
            Style::Bold => "bold",
            Style::Cautious => "cautious",
            Style::Hunter => "hunter",
        }
    }

    pub fn parse(id: &str) -> Option<Style> {
        Style::ALL.into_iter().find(|s| s.id() == id)
    }

    /// The style Random draws for a sitting of `seed`: a hash of the seed and
    /// the label `style`, so it is a function of the seed. A record carries
    /// the result (`drawn`), which is authoritative: the pool may change.
    pub fn drawn(seed: u64) -> Style {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in seed.to_le_bytes().into_iter().chain(*b"style") {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        // FNV's low bits are weak; fold the high ones in.
        h ^= h >> 32;
        Style::POOL[(h % Style::POOL.len() as u64) as usize]
    }

    /// `Random` as the style it drew for `seed`; any other style as itself.
    pub fn resolve(self, seed: u64) -> Style {
        if self == Style::Random {
            Style::drawn(seed)
        } else {
            self
        }
    }

    /// This style's rung-4 bid margin (`MARGINS`); Normal's is 0.
    pub fn rung4_margin(self) -> f64 {
        MARGINS
            .iter()
            .find(|(s, _)| *s == self)
            .map_or(0.0, |&(_, m)| m)
    }

    /// Moves `config` to this style's parameters. `Random` is not a parameter
    /// set and changes nothing: resolve it first.
    pub fn apply(self, c: &mut Config) {
        c.style = self;
        if matches!(self, Style::Normal | Style::Random) {
            return;
        }
        c.bid_margin = Some(self.rung4_margin());
        let w: &mut Weights = &mut c.weights;
        w.band = BAND;
        match self {
            Style::Normal | Style::Random => {}
            Style::Bermann => {
                // Be 145-146 "20 bis 25 Points in Honneurs und höheren Figuren":
                // 60 thirds, a queen counted as a figure (T); no tarock minimum
                // from the tarocks' "Nebenrolle" (I, one step).
                c.bid = BidRule {
                    sure_thirds: 60,
                    min_tarocks: 0,
                    queens: Queens::Always,
                    ..BID_RULE
                };
                // Be 148 "so sind sie stets abzuspielen": no `low_shown < 2`
                // exception ("leere Blätter" read as blank pips, Be 151) (T).
                w.low_shown_exception = false;
                // Be 151-152: keep one or more small tarocks (I: the text keeps
                // them to catch the Mond, STYLES reads it as holding it), play
                // an unescorted Mond "stets bei passender Gelegenheit" (I: a
                // fitting chance read as a rich suit trick it takes).
                w.guard_mond = 4.0;
                w.guard_keep = 1;
                w.cash_mond = 4.0;
                // Be 150-151: the fork position only (I, narrow). The weight
                // outbids `split_suit` (4.0), which would forbid the lead (E).
                w.spare_lead = 6.0;
            }
            Style::Ulmann => {
                // U 77-78: four honours (the Pagat only with six tarocks, U 78)
                // at once (T). The dealer clause on three weak honours is not
                // modelled: Normal's thresholds already refuse such a hand.
                c.bid = BidRule {
                    extra: BidExtra::Ulmann,
                    ..BID_RULE
                };
                // U 74: 10-12 tarocks hunt the Pagat (T). U's other conditions,
                // "no Pagat in hand" and "a closed run", are not modelled.
                w.draw_from = Some(10);
                // U 74 "suche man rechtzeitig heimzubringen" (I, as Bermann). The Pagat
                // cash with 7-8 tarocks is Normal's (the spec: no parameter).
                w.cash_mond = 4.0;
            }
            Style::Ms => {
                // MS "Wie stark ist dein Blatt": 60 sure thirds, 6 tarocks,
                // the Pagat always counted (T).
                c.bid = BidRule {
                    sure_thirds: 60,
                    min_tarocks: 6,
                    pagat_guard: 0,
                    ..BID_RULE
                };
                // MS "mehr als 11 Tarock" (T); "Mondfang": one or two small
                // tarocks (T).
                w.draw_from = Some(12);
                w.guard_mond = 4.0;
                w.guard_keep = 2;
            }
            Style::Werner => {
                // W 131: three honours, six or more tarocks and a guarded
                // queen; a sufficient condition read as the rule (I).
                c.bid = BidRule {
                    min_tarocks: 6,
                    extra: BidExtra::Werner,
                    ..BID_RULE
                };
            }
            Style::Bold => {
                // E throughout: a temperament claims no history (2.2).
                c.bid = BidRule {
                    sure_thirds: 36,
                    min_tarocks: 4,
                    ..BID_RULE
                };
                w.split_suit *= 0.5;
                w.exposed_figure *= 1.5;
            }
            Style::Cautious => {
                c.bid = BidRule {
                    sure_thirds: 60,
                    min_tarocks: 6,
                    ..BID_RULE
                };
                w.split_suit *= 1.5;
                w.keep_packet *= 1.5;
                w.strong_tarocks = 10;
            }
            Style::Hunter => {
                c.bid = BidRule {
                    sure_thirds: 36,
                    min_tarocks: 7,
                    ..BID_RULE
                };
                w.strong_tarocks = 6;
                w.draw_from = Some(8);
            }
        }
    }
}

impl Config {
    /// Normal's tuning moved to `style`'s; `Random` is first resolved for
    /// `seed`.
    pub fn for_style(style: Style, seed: u64) -> Config {
        let mut c = Config::default();
        style.resolve(seed).apply(&mut c);
        c
    }
}

#[cfg(test)]
#[path = "style_tests.rs"]
mod tests;
