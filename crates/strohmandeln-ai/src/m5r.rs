//! M5r, the rule refinement of `docs/HINTS.md` 5.2: for each idea of
//! `docs/TUTOR.md` 9 that failed M5, search a small fixed family of
//! conditions on what the player can see for one under which rung 4 does
//! what the idea says at 0.80.
//!
//! An **idea** is a trigger and a prescription (the candidates of
//! [`crate::rules`]); a **class** is one prescription of an idea, a test of
//! the card played. An **atom** is a condition of visible state only (the
//! mover's [`Knowledge`], no hidden card). A candidate rule is a class with
//! one atom or a conjunction of two atoms of different groups. The
//! measurement (`docs/MEASUREMENTS.md`, M5r) logs, for each decision of
//! rung 4 and each class whose trigger fires, the atoms that hold and
//! whether the card played is in the class ([`Log`]); [`select`] fits the
//! candidates on the selection seeds; [`step`] is the sequential
//! confirmation; [`sentence`] says a passed rule in the atoms' words.

use crate::cardset::{leader_wins, Kind};
use crate::know::Knowledge;
use crate::m5r_rules::PASSED;
use crate::measure::{self, Precision};
use crate::opponent::Config;
use crate::rules;
use crate::tutor::shown_void;
use strohmandeln_core::cards::{Card, Suit};
use strohmandeln_core::types::{Action, Settings};
use strohmandeln_core::view::bare_view;

// ---- atoms ------------------------------------------------------------------

/// A condition of visible state. `test` takes the mover's knowledge and the
/// class's **focus suit** (the suit led when following, else the suit the
/// idea is about; `None` where there is none, which makes a suit atom false).
#[derive(Copy, Clone)]
pub struct Atom {
    pub id: &'static str,
    /// Atoms of one group are alternative thresholds of one quantity and are
    /// never paired.
    pub group: &'static str,
    /// The clause after "when", in English and German (subordinate clause).
    pub en: &'static str,
    pub de: &'static str,
    pub test: fn(&Knowledge, Option<Suit>) -> bool,
}

fn skus_out(k: &Knowledge) -> bool {
    !k.played.contains(Card::SKUS) && !k.playable().contains(Card::SKUS)
}

/// Thirds of `suit` not yet played (mine, covers and unseen alike).
fn left_thirds(k: &Knowledge, suit: Suit) -> u32 {
    Card::all()
        .filter(|c| c.suit() == Some(suit) && !k.played.contains(*c))
        .map(Card::thirds)
        .sum()
}

fn holds_master(k: &Knowledge, suit: Suit) -> bool {
    Card::all()
        .filter(|c| c.suit() == Some(suit) && !k.played.contains(*c))
        .max_by_key(|c| c.strength())
        .is_some_and(|top| k.playable().contains(top))
}

fn his_covers(k: &Knowledge, suit: Suit) -> usize {
    k.opp_covers
        .iter()
        .flatten()
        .filter(|c| c.suit() == Some(suit))
        .count()
}

/// He has shown no tarock: he could not follow a tarock or did not trump a
/// suit he lacked, and none of his known cards is one.
fn shown_no_tarock(k: &Knowledge) -> bool {
    k.opp_void[Kind::Trump.index()]
        && !k
            .opp_covers
            .iter()
            .flatten()
            .copied()
            .chain(k.opp_known.iter())
            .any(Card::is_tarock)
}

fn led_at_least(k: &Knowledge, thirds: u32) -> bool {
    k.lead.is_some_and(|c| c.thirds() >= thirds)
}

fn my_covers(k: &Knowledge) -> usize {
    k.covers.iter().flatten().count()
}

macro_rules! atom {
    ($id:expr, $group:expr, $en:expr, $de:expr, $t:expr) => {
        Atom {
            id: $id,
            group: $group,
            en: $en,
            de: $de,
            test: $t,
        }
    };
}

/// The family of `docs/HINTS.md` 5.2, fixed before the first run. The suit's
/// points left are "at most" (HINTS 5.2). Each text is a noun phrase that
/// follows "With" (German: "Mit", dative), so atoms join with "and"; points
/// are thirds / 3. "Still out" is the Cards-out tab's count
/// ([`Knowledge::out_tarocks`]).
pub const ATOMS: &[Atom] = &[
    atom!(
        "tu0",
        "tu",
        "no tarock still out",
        "keinem Tarock mehr draußen",
        |k, _| k.out_tarocks() == 0
    ),
    atom!(
        "tu2",
        "tu",
        "2 or fewer tarocks still out",
        "höchstens 2 Tarock noch draußen",
        |k, _| k.out_tarocks() <= 2
    ),
    atom!(
        "tu4",
        "tu",
        "4 or fewer tarocks still out",
        "höchstens 4 Tarock noch draußen",
        |k, _| k.out_tarocks() <= 4
    ),
    atom!(
        "tu6",
        "tu",
        "6 or fewer tarocks still out",
        "höchstens 6 Tarock noch draußen",
        |k, _| k.out_tarocks() <= 6
    ),
    atom!(
        "tu8",
        "tu",
        "8 or fewer tarocks still out",
        "höchstens 8 Tarock noch draußen",
        |k, _| k.out_tarocks() <= 8
    ),
    atom!(
        "mt6",
        "mt",
        "6 or more tarocks of yours",
        "mindestens 6 eigenen Tarock",
        |k, _| k.my_tarocks() >= 6
    ),
    atom!(
        "mt8",
        "mt",
        "8 or more tarocks of yours",
        "mindestens 8 eigenen Tarock",
        |k, _| k.my_tarocks() >= 8
    ),
    atom!(
        "mt10",
        "mt",
        "10 or more tarocks of yours",
        "mindestens 10 eigenen Tarock",
        |k, _| k.my_tarocks() >= 10
    ),
    atom!(
        "mt12",
        "mt",
        "12 or more tarocks of yours",
        "mindestens 12 eigenen Tarock",
        |k, _| k.my_tarocks() >= 12
    ),
    atom!(
        "ml5",
        "ml",
        "5 or fewer tarocks of yours",
        "höchstens 5 eigenen Tarock",
        |k, _| k.my_tarocks() <= 5
    ),
    atom!(
        "ml7",
        "ml",
        "7 or fewer tarocks of yours",
        "höchstens 7 eigenen Tarock",
        |k, _| k.my_tarocks() <= 7
    ),
    atom!(
        "void",
        "void",
        "your opponent out of the suit",
        "Ihrem Gegner ohne die Farbe",
        |k, f| f.is_some_and(|s| shown_void(k, s))
    ),
    atom!(
        "notarock",
        "notarock",
        "your opponent out of tarocks",
        "Ihrem Gegner ohne Tarock",
        |k, _| shown_no_tarock(k)
    ),
    atom!(
        "skus",
        "skus",
        "the Sküs still out",
        "dem Sküs noch draußen",
        |k, _| skus_out(k)
    ),
    atom!(
        "mond",
        "mond",
        "the Mond in your hand",
        "dem Mond in Ihrer Hand",
        |k, _| k.playable().contains(Card::MOND)
    ),
    atom!(
        "left13",
        "left",
        "4⅓ points or fewer of the suit left",
        "höchstens 4⅓ Punkten der Farbe übrig",
        |k, f| f.is_some_and(|s| left_thirds(k, s) <= 13)
    ),
    atom!(
        "left26",
        "left",
        "8⅔ points or fewer of the suit left",
        "höchstens 8⅔ Punkten der Farbe übrig",
        |k, f| f.is_some_and(|s| left_thirds(k, s) <= 26)
    ),
    atom!(
        "ph1",
        "ph",
        "trick 1 to 9 under way",
        "dem 1. bis 9. Stich",
        |k, _| k.trick_number <= 9
    ),
    atom!(
        "ph2",
        "ph",
        "trick 10 to 18 under way",
        "dem 10. bis 18. Stich",
        |k, _| (10..=18).contains(&k.trick_number)
    ),
    atom!(
        "ph3",
        "ph",
        "trick 19 to 27 under way",
        "dem 19. bis 27. Stich",
        |k, _| k.trick_number >= 19
    ),
    atom!(
        "cov1",
        "cov",
        "a cover of yours",
        "einem eigenen Paketblatt",
        |k, _| my_covers(k) >= 1
    ),
    atom!(
        "cov2",
        "cov",
        "two covers of yours",
        "zwei eigenen Paketblättern",
        |k, _| my_covers(k) >= 2
    ),
    atom!(
        "hc1",
        "hc",
        "your opponent showing a cover of the suit",
        "Ihrem Gegner mit einem Paketblatt der Farbe",
        |k, f| f.is_some_and(|s| his_covers(k, s) >= 1)
    ),
    atom!(
        "hc2",
        "hc",
        "your opponent showing two covers of the suit",
        "Ihrem Gegner mit zwei Paketblättern der Farbe",
        |k, f| f.is_some_and(|s| his_covers(k, s) >= 2)
    ),
    atom!("led1", "led", "a card led", "einem Anspiel", |k, _| {
        led_at_least(k, 1)
    }),
    atom!(
        "led4",
        "led",
        "a card worth 1⅓ points or more led",
        "einem Anspiel von mindestens 1⅓ Punkten",
        |k, _| led_at_least(k, 4)
    ),
    atom!(
        "led7",
        "led",
        "a card worth 2⅓ points or more led",
        "einem Anspiel von mindestens 2⅓ Punkten",
        |k, _| led_at_least(k, 7)
    ),
    atom!(
        "led10",
        "led",
        "a card worth 3⅓ points or more led",
        "einem Anspiel von mindestens 3⅓ Punkten",
        |k, _| led_at_least(k, 10)
    ),
    atom!(
        "led13",
        "led",
        "a card worth 4⅓ points led",
        "einem Anspiel von 4⅓ Punkten",
        |k, _| led_at_least(k, 13)
    ),
    atom!(
        "master",
        "master",
        "the suit's top card in your hand",
        "der höchsten Karte der Farbe in Ihrer Hand",
        |k, f| f.is_some_and(|s| holds_master(k, s))
    ),
];

pub fn atom_index(id: &str) -> usize {
    ATOMS
        .iter()
        .position(|a| a.id == id)
        .unwrap_or_else(|| panic!("no atom {id}"))
}

/// The bits of the atoms that hold, by index in [`ATOMS`].
pub fn atom_mask(k: &Knowledge, focus: Option<Suit>) -> u32 {
    ATOMS
        .iter()
        .enumerate()
        .filter(|(_, a)| (a.test)(k, focus))
        .fold(0, |m, (i, _)| m | (1 << i))
}

// ---- ideas and classes ---------------------------------------------------------

/// One prescription of an idea, with the atoms listed for the idea (at most
/// 12, fixed before the first run).
#[derive(Copy, Clone)]
pub struct Class {
    pub id: &'static str,
    pub idea: &'static str,
    pub atoms: &'static [&'static str],
    /// What opens the sentence before the atoms: "With" (German "Mit"), or
    /// the trigger when it fits ("Following with").
    pub lead_en: &'static str,
    pub lead_de: &'static str,
    /// The action, English and German (infinitive), with the trigger where
    /// it is not in the lead.
    pub en: &'static str,
    pub de: &'static str,
    pub focus: fn(&Knowledge) -> Option<Suit>,
    /// `None` where the idea's trigger does not fire, else whether the card
    /// played is in the class.
    pub check: fn(&Knowledge, &Action) -> Option<bool>,
}

const A_POOR: &[&str] = &[
    "tu4", "tu8", "mt8", "ml7", "ph1", "ph3", "left13", "left26", "hc1", "master", "skus", "led4",
];
const A_CHEAP: &[&str] = &[
    "tu4", "tu8", "mt8", "ml7", "ph2", "ph3", "left13", "left26", "master", "led4", "led7", "cov1",
];
const A_OFF: &[&str] = &[
    "cov1", "cov2", "hc1", "hc2", "tu4", "tu8", "ph1", "ph3", "left13", "left26", "master", "mt8",
];
const A_RUN: &[&str] = &[
    "tu2", "tu4", "tu6", "tu8", "mt6", "mt8", "mt10", "mt12", "notarock", "skus", "mond", "ph3",
];
const A_PAGAT: &[&str] = &[
    "tu0", "tu2", "tu4", "mt8", "mt10", "mt12", "ml5", "ml7", "skus", "mond", "ph3", "led7",
];
const A_VOID: &[&str] = &[
    "tu4", "tu8", "mt8", "ml7", "notarock", "skus", "ph1", "ph2", "ph3", "left13", "left26", "cov1",
];
const A_MOND: &[&str] = &[
    "tu0", "tu2", "tu4", "tu8", "mt8", "mt10", "skus", "ph2", "ph3", "led4", "led7", "notarock",
];
const A_TRUMPS: &[&str] = &[
    "tu2", "tu4", "tu6", "tu8", "mt6", "mt8", "mt10", "ml7", "notarock", "skus", "mond", "ph3",
];

fn played(a: &Action) -> Option<Card> {
    match a {
        Action::Play { card, .. } => Some(*card),
        _ => None,
    }
}

fn focus_led(k: &Knowledge) -> Option<Suit> {
    k.lead.and_then(Card::suit)
}

fn focus_none(_: &Knowledge) -> Option<Suit> {
    None
}

/// Leading with a cover and a hand card legal: the suit of the highest cover.
fn focus_lead_cover(k: &Knowledge) -> Option<Suit> {
    k.legal
        .iter()
        .filter(|&&c| k.is_cover(c))
        .max_by_key(|c| c.strength())
        .and_then(|c| c.suit())
}

fn focus_figure(k: &Knowledge) -> Option<Suit> {
    rules::first_figure(&rules::figures(k)).and_then(|c| c.suit())
}

fn focus_void(k: &Knowledge) -> Option<Suit> {
    k.legal
        .iter()
        .filter_map(|c| c.suit())
        .find(|&s| shown_void(k, s))
}

/// Following with the Mond legal, winning, and another card legal.
fn mond_check(k: &Knowledge, a: &Action) -> Option<bool> {
    let led = k.lead?;
    let card = played(a)?;
    (k.legal.len() >= 2 && k.legal.contains(&Card::MOND) && !leader_wins(led, Card::MOND))
        .then(|| card == Card::MOND)
}

/// Leading with a tarock and a suit card both legal: is the card a tarock.
fn lead_split(k: &Knowledge, a: &Action, tarock: bool) -> Option<bool> {
    let card = played(a)?;
    (k.lead.is_none()
        && k.legal.iter().any(|c| c.is_tarock())
        && k.legal.iter().any(|c| !c.is_tarock()))
    .then(|| card.is_tarock() == tarock)
}

macro_rules! class {
    ($id:expr, $idea:expr, $atoms:expr, $lead:expr, $en:expr, $de:expr, $focus:expr, $check:expr) => {
        Class {
            id: $id,
            idea: $idea,
            atoms: $atoms,
            lead_en: $lead.0,
            lead_de: $lead.1,
            en: $en,
            de: $de,
            focus: $focus,
            check: $check,
        }
    };
}

const WITH: (&str, &str) = ("With", "Mit");
const FOLLOWING: (&str, &str) = ("Following with", "Beim Bedienen mit");

/// The ideas and their classes (`docs/HINTS.md` 5.2). The triggers are M5's
/// (`rules::CANDIDATES`) without their numbers: the atoms say the numbers.
/// Texts avoid the words the tutor's voice does not use (`m5r_tests`).
pub const CLASSES: &[Class] = &[
    class!(
        "worth.poor/no-king",
        "worth.poor",
        A_POOR,
        WITH,
        "if a king of yours would win, play another card",
        "würde ein König von Ihnen stechen, eine andere Karte spielen",
        focus_led,
        |k, a| rules::poor_king(k, a, 4)
    ),
    class!(
        "cheap/outcome",
        "cheap",
        A_CHEAP,
        FOLLOWING,
        "win or lose with the fewest-points card",
        "stechen oder nicht, mit der punkteärmsten Karte",
        focus_led,
        |k, a| rules::cheapest(k, a, false)
    ),
    class!(
        "cheap/winner",
        "cheap",
        A_CHEAP,
        FOLLOWING,
        "take the trick with the fewest-points card that wins",
        "den Stich mit der punkteärmsten stechenden Karte holen",
        focus_led,
        |k, a| rules::cheapest(k, a, true)
    ),
    class!(
        "off/follow-cover",
        "off",
        A_OFF,
        FOLLOWING,
        "play a cover",
        "ein Paketblatt spielen",
        focus_led,
        |k, a| rules::off_any(k, a, false)
    ),
    class!(
        "off/lead-cover",
        "off",
        A_OFF,
        WITH,
        "lead a cover",
        "ein Paketblatt anspielen",
        focus_lead_cover,
        |k, a| rules::off_any(k, a, true)
    ),
    class!(
        "off/figure-cover",
        "off",
        A_OFF,
        WITH,
        "play a Dame or Cavall cover",
        "ein Paketblatt Dame oder Cavall spielen",
        focus_figure,
        rules::figure_any
    ),
    class!(
        "off/figure-higher",
        "off",
        A_OFF,
        WITH,
        "play the higher Dame or Cavall cover",
        "das höhere Paketblatt Dame oder Cavall spielen",
        focus_figure,
        |k, a| rules::figure_played(k, a, false)
    ),
    class!(
        "run/top",
        "run",
        A_RUN,
        WITH,
        "lead the top of your tarocks next in rank, else your lowest",
        "das höchste Ihrer im Rang aufeinanderfolgenden Tarock anspielen, sonst das niedrigste",
        focus_none,
        rules::run_top
    ),
    class!(
        "run/any",
        "run",
        A_RUN,
        WITH,
        "lead a tarock next in rank to another of yours, else your lowest",
        "ein Tarock anspielen, das im Rang neben einem Ihrer Tarock steht, sonst das niedrigste",
        focus_none,
        rules::run_any
    ),
    class!(
        "pagat/keep",
        "pagat",
        A_PAGAT,
        FOLLOWING,
        "keep the Pagat",
        "den Pagat behalten",
        focus_led,
        |k, a| rules::pagat_keep(k, a, 0)
    ),
    class!(
        "pagat/cash",
        "pagat",
        A_PAGAT,
        FOLLOWING,
        "play the Pagat",
        "den Pagat spielen",
        focus_led,
        |k, a| rules::pagat_cash(k, a, 99)
    ),
    class!(
        "void/avoid",
        "void",
        A_VOID,
        WITH,
        "do not lead the suit your opponent lacks",
        "die Farbe, die Ihrem Gegner fehlt, nicht anspielen",
        focus_void,
        rules::void_lead
    ),
    class!(
        "mond/mond",
        "mond",
        A_MOND,
        FOLLOWING,
        "answer your opponent's lead with the Mond",
        "das Anspiel Ihres Gegners mit dem Mond beantworten",
        focus_led,
        mond_check
    ),
    class!(
        "trumps/tarock",
        "trumps",
        A_TRUMPS,
        WITH,
        "when leading, choose a tarock over a suit card",
        "beim Anspielen ein Tarock einer Farbkarte vorziehen",
        focus_none,
        |k, a| lead_split(k, a, true)
    ),
    class!(
        "trumps/suit",
        "trumps",
        A_TRUMPS,
        WITH,
        "when leading, choose a suit card over a tarock",
        "beim Anspielen eine Farbkarte einem Tarock vorziehen",
        focus_none,
        |k, a| lead_split(k, a, false)
    ),
];

pub fn class_index(id: &str) -> usize {
    CLASSES
        .iter()
        .position(|c| c.id == id)
        .unwrap_or_else(|| panic!("no class {id}"))
}

// ---- the log -----------------------------------------------------------------

/// One firing of a class: the deal it happened in (the cluster), the atoms
/// that held, and whether the card played was in the class.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub deal: u32,
    pub mask: u32,
    pub ok: bool,
}

/// The decisions of rung 4 as firings of each class, in deal order.
#[derive(Clone, Debug)]
pub struct Log {
    pub ndeals: usize,
    pub by_class: Vec<Vec<Entry>>,
}

impl Default for Log {
    fn default() -> Self {
        Self::new()
    }
}

impl Log {
    pub fn new() -> Log {
        Log {
            ndeals: 0,
            by_class: vec![Vec::new(); CLASSES.len()],
        }
    }

    /// Logs one decision of the player in deal `deal`.
    pub fn record(&mut self, deal: usize, k: &Knowledge, action: &Action) {
        self.ndeals = self.ndeals.max(deal + 1);
        for (i, cl) in CLASSES.iter().enumerate() {
            if let Some(ok) = (cl.check)(k, action) {
                let mask = atom_mask(k, (cl.focus)(k));
                self.by_class[i].push(Entry {
                    deal: deal as u32,
                    mask,
                    ok,
                });
            }
        }
    }

    /// Appends `other`, its deals after this log's.
    pub fn append(&mut self, other: &Log) {
        let shift = self.ndeals as u32;
        for (mine, theirs) in self.by_class.iter_mut().zip(&other.by_class) {
            mine.extend(theirs.iter().map(|e| Entry {
                deal: e.deal + shift,
                ..*e
            }));
        }
        self.ndeals += other.ndeals;
    }

    /// `ndeals N`, then a line `class deal mask(hex) ok` per firing.
    pub fn to_tsv(&self) -> String {
        let mut out = format!("ndeals\t{}\n", self.ndeals);
        for (cl, entries) in CLASSES.iter().zip(&self.by_class) {
            for e in entries {
                out.push_str(&format!(
                    "{}\t{}\t{:x}\t{}\n",
                    cl.id,
                    e.deal,
                    e.mask,
                    u8::from(e.ok)
                ));
            }
        }
        out
    }

    pub fn from_tsv(text: &str) -> Log {
        let mut log = Log::new();
        for line in text.lines() {
            let f: Vec<&str> = line.split('\t').collect();
            if f[0] == "ndeals" {
                log.ndeals = f[1].parse().expect("a count");
            } else {
                log.by_class[class_index(f[0])].push(Entry {
                    deal: f[1].parse().expect("a deal"),
                    mask: u32::from_str_radix(f[2], 16).expect("a mask"),
                    ok: f[3] == "1",
                });
            }
        }
        log
    }
}

/// The deals of `sittings` sittings of rung 4 (the player) against rung 3,
/// seeds `first..`, the player in seat A when the sitting's index (counted
/// from `index0`) is even, under `settings`: every decision of the player.
pub fn collect(first: u64, sittings: u64, index0: u64, settings: Settings) -> Log {
    const THREADS: u64 = 2;
    let mut parts: Vec<(u64, Log)> = std::thread::scope(|s| {
        let handles: Vec<_> = (0..THREADS)
            .map(|t| {
                s.spawn(move || {
                    let mut out = Vec::new();
                    let mut i = t;
                    while i < sittings {
                        out.push((
                            i,
                            sitting_log(first + i, settings, (index0 + i).is_multiple_of(2)),
                        ));
                        i += THREADS;
                    }
                    out
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().expect("a worker"))
            .collect()
    });
    parts.sort_by_key(|(i, _)| *i);
    let mut log = Log::new();
    for (_, part) in &parts {
        log.append(part);
    }
    log
}

fn sitting_log(seed: u64, settings: Settings, player_a: bool) -> Log {
    let (seat, records) = measure::player_sitting(seed, settings, player_a, Config::default());
    let mut log = Log::new();
    for rec in &records {
        let deal = log.ndeals;
        rec.replay(|_, d, who, action| {
            if who != seat {
                return;
            }
            let view = bare_view(d, who);
            if let Some(k) = Knowledge::from_view(&view) {
                log.record(deal, &k, action);
            }
        })
        .expect("a played deal replays");
        log.ndeals = deal + 1;
    }
    log
}

// ---- the search ----------------------------------------------------------------

/// The candidates of a class: each of its atoms alone, then each pair of
/// atoms of different groups (indices into [`ATOMS`], ascending).
pub fn candidate_atoms(class: usize) -> Vec<Vec<usize>> {
    let mut idx: Vec<usize> = CLASSES[class].atoms.iter().map(|a| atom_index(a)).collect();
    idx.sort_unstable();
    let mut out: Vec<Vec<usize>> = idx.iter().map(|&a| vec![a]).collect();
    for (n, &a) in idx.iter().enumerate() {
        for &b in &idx[n + 1..] {
            if ATOMS[a].group != ATOMS[b].group {
                out.push(vec![a, b]);
            }
        }
    }
    out
}

fn mask_of(atoms: &[usize]) -> u32 {
    atoms.iter().fold(0, |m, &a| m | (1 << a))
}

/// A candidate's precision and where it fires.
#[derive(Clone, Copy, Debug)]
pub struct Eval {
    /// Precision clustered by deal.
    pub prec: Precision,
    /// The share of deals in which the candidate fires.
    pub reach: f64,
    /// The share of its firings in tricks 10 to 27, and in tricks 19 to 27.
    pub late10: f64,
    pub late19: f64,
}

/// A candidate's precision (clustered by deal) over the firings where its
/// atoms hold, its reach and its share of late firings.
pub fn evaluate(log: &Log, class: usize, atoms: &[usize]) -> Eval {
    let need = mask_of(atoms);
    let (ph1, ph3) = (1u32 << atom_index("ph1"), 1u32 << atom_index("ph3"));
    let mut fired = vec![0.0; log.ndeals];
    let mut sat = vec![0.0; log.ndeals];
    let (mut n, mut l10, mut l19) = (0.0, 0.0, 0.0);
    for e in &log.by_class[class] {
        if e.mask & need == need {
            fired[e.deal as usize] += 1.0;
            sat[e.deal as usize] += f64::from(u8::from(e.ok));
            n += 1.0;
            l10 += f64::from(u8::from(e.mask & ph1 == 0));
            l19 += f64::from(u8::from(e.mask & ph3 != 0));
        }
    }
    let share = |x: f64| if n > 0.0 { x / n } else { 0.0 };
    Eval {
        prec: measure::precision(&sat, &fired),
        reach: fired.iter().filter(|&&f| f > 0.0).count() as f64 / log.ndeals.max(1) as f64,
        late10: share(l10),
        late19: share(l19),
    }
}

/// The smallest effective sample a candidate may be fitted or pass on.
pub const MIN_N: f64 = 60.0;
/// The gate: the Wilson lower bound the precision must reach.
pub const GATE: f64 = 0.80;

/// A fitted candidate.
#[derive(Clone, Debug)]
pub struct Cand {
    pub atoms: Vec<usize>,
    pub prec: Precision,
    pub reach: f64,
    pub late10: f64,
    pub late19: f64,
}

/// The fit of one class on the selection data.
#[derive(Clone, Debug)]
pub struct Selected {
    pub class: usize,
    /// The class with no atom: how often the idea holds in all its firings.
    pub base: Precision,
    /// How many candidates were evaluated.
    pub ncand: usize,
    /// The widest eligible candidate (effective sample at least 60, Wilson
    /// lower bound at least 0.80; most firings, ties by lower bound).
    pub best: Option<Cand>,
    /// How many were eligible.
    pub eligible: usize,
}

/// Per class, the widest eligible candidate. An atom true at every firing
/// of the class adds nothing and is not tried.
pub fn select(log: &Log) -> Vec<Selected> {
    (0..CLASSES.len())
        .map(|class| {
            let always = log.by_class[class].iter().fold(u32::MAX, |m, e| m & e.mask);
            let base = evaluate(log, class, &[]).prec;
            let mut ncand = 0;
            let mut eligible = 0;
            let mut best: Option<Cand> = None;
            if !log.by_class[class].is_empty() {
                for atoms in candidate_atoms(class) {
                    if mask_of(&atoms) & always != 0 {
                        continue;
                    }
                    ncand += 1;
                    let Eval {
                        prec,
                        reach,
                        late10,
                        late19,
                    } = evaluate(log, class, &atoms);
                    if prec.n_eff >= MIN_N && prec.lower >= GATE {
                        eligible += 1;
                        let wider = best.as_ref().is_none_or(|b| {
                            (prec.fired, prec.lower) > (b.prec.fired, b.prec.lower)
                        });
                        if wider {
                            best = Some(Cand {
                                atoms,
                                prec,
                                reach,
                                late10,
                                late19,
                            });
                        }
                    }
                }
            }
            Selected {
                class,
                base,
                ncand,
                best,
                eligible,
            }
        })
        .collect()
}

// ---- the sequential confirmation ---------------------------------------------------

/// The boundaries of the first run (superseded): the two-sided O'Brien-
/// Fleming boundaries of `stats::Plan { looks: 4 }` at 0.05, rounded, used
/// one-sided.
pub const BOUNDARIES: [f64; 4] = [4.05, 2.86, 2.34, 2.02];
/// Futility from the second look.
pub const FUTILITY_Z: f64 = -1.645;
/// Looks of the confirmation.
pub const LOOKS: usize = 4;

/// The boundaries when `k` classes are confirmed: the Bonferroni level
/// 0.05 / k (the two-sided level whose boundaries are used one-sided, as
/// above), O'Brien-Fleming for four looks, from `stats.rs`, two decimals.
pub fn family_boundaries(k: usize) -> Vec<f64> {
    crate::stats::obf_boundaries(0.05 / k.max(1) as f64, LOOKS)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Step {
    Pass,
    Fail,
    Continue,
}

/// z = (p - 0.80) / sqrt(0.8 x 0.2 / n_eff).
pub fn z_of(p: &Precision) -> f64 {
    (p.p - GATE) / (GATE * (1.0 - GATE) / p.n_eff).sqrt()
}

/// The decision at `look` (1 to 4) against `bounds`: pass at or above the
/// boundary with an effective sample of at least 60; fail from the second
/// look at z at most -1.645, and at the last look without a pass.
pub fn step(look: usize, p: &Precision, bounds: &[f64]) -> Step {
    let last = look >= bounds.len();
    if p.fired == 0 || p.n_eff <= 0.0 {
        return if last { Step::Fail } else { Step::Continue };
    }
    let z = z_of(p);
    if z >= bounds[look - 1] && p.n_eff >= MIN_N {
        Step::Pass
    } else if (look >= 2 && z <= FUTILITY_Z) || last {
        Step::Fail
    } else {
        Step::Continue
    }
}

// ---- wording -------------------------------------------------------------------------

/// Classes whose rules passed but are not told (the review of 17 Oct 2026:
/// over the word budget as worded, or failing settings).
pub const NOT_SHIPPED: [&str; 2] = ["run/any", "void/avoid"];

/// A rule that passed M5r (`m5r_rules.rs`, generated).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PassedRule {
    pub id: &'static str,
    pub class: &'static str,
    pub atoms: &'static [&'static str],
    pub fired: u32,
    pub precision: f64,
    pub n_eff: f64,
    pub lower: f64,
    /// The share of deals in which the rule fires.
    pub reach: f64,
    /// The share of its firings in tricks 10 to 27, and in tricks 19 to 27.
    pub late10: f64,
    pub late19: f64,
    /// At least 80% of the firings are after trick 9: the "Why?" says the
    /// rule describes play late in the deal.
    pub late: bool,
    /// Fires in fewer than one deal in 40: true, but not counted for reach.
    pub too_rare: bool,
    /// The lower bound is at least 0.80 under every other setting.
    pub settings_ok: bool,
    /// The sentence is within the budget: condition at most 8 words, action
    /// at most 10 ([`word_counts`]).
    pub fits_budget: bool,
    /// Told: fits the budget and is not in [`NOT_SHIPPED`].
    pub ships: bool,
}

/// The "so" sentence of a rule: the class's lead, the atoms' phrases joined
/// with "and", then the action. English: "With A and B, act."; German: "Mit A
/// und B: act."
pub fn sentence(atoms: &[&str], class: &str, de: bool) -> String {
    let cl = &CLASSES[class_index(class)];
    let phrases: Vec<&str> = atoms
        .iter()
        .map(|a| {
            let a = &ATOMS[atom_index(a)];
            if de {
                a.de
            } else {
                a.en
            }
        })
        .collect();
    if de {
        format!("{} {}: {}.", cl.lead_de, phrases.join(" und "), cl.de)
    } else {
        format!("{} {}, {}.", cl.lead_en, phrases.join(" and "), cl.en)
    }
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// The words of the condition (the atoms' phrases) and of the action (with
/// the trigger words of the lead beyond "With"), in English.
pub fn word_counts(atoms: &[&str], class: &str) -> (usize, usize) {
    let cl = &CLASSES[class_index(class)];
    let cond = atoms.iter().map(|a| words(ATOMS[atom_index(a)].en)).sum();
    (cond, words(cl.en) + words(cl.lead_en) - 1)
}

/// As [`word_counts`], in German.
pub fn word_counts_de(atoms: &[&str], class: &str) -> (usize, usize) {
    let cl = &CLASSES[class_index(class)];
    let cond = atoms.iter().map(|a| words(ATOMS[atom_index(a)].de)).sum();
    (cond, words(cl.de) + words(cl.lead_de) - 1)
}

/// Whether a passed rule holds at this decision and the action is in its
/// class: its idea's trigger fires, every atom holds, the card is in the
/// class.
pub fn rule_matches(r: &PassedRule, k: &Knowledge, action: &Action) -> bool {
    let cl = &CLASSES[class_index(r.class)];
    if (cl.check)(k, action) != Some(true) {
        return false;
    }
    let mask = atom_mask(k, (cl.focus)(k));
    let need = mask_of(&r.atoms.iter().map(|a| atom_index(a)).collect::<Vec<_>>());
    mask & need == need
}

/// The shipped rules that hold at this decision for the card the hint
/// plays, widest first (most firings): the hint's "so" is the first.
pub fn matching(k: &Knowledge, action: &Action) -> Vec<&'static PassedRule> {
    let mut v: Vec<&'static PassedRule> = PASSED
        .iter()
        .filter(|r| r.ships && rule_matches(r, k, action))
        .collect();
    v.sort_by_key(|r| std::cmp::Reverse(r.fired));
    v
}

/// The text of `m5r_rules.rs` from the confirmation and settings lists
/// (`docs/measurements/m5r-confirmed.tsv`, `m5r-settings.tsv`): the passed
/// rows of the first, with the settings verdict, reach and word budget.
pub fn emit_source(confirmed: &str, settings: &str) -> String {
    let mut out = String::from(
        "//! The rules that passed M5r (`docs/MEASUREMENTS.md`), generated by\n\
         //! `m5r emit` from `docs/measurements/m5r-confirmed.tsv` and\n\
         //! `m5r-settings.tsv` (a test checks it). Do not edit.\n\n\
         use crate::m5r::PassedRule;\n\n\
         pub const PASSED: &[PassedRule] = &[\n",
    );
    for line in confirmed.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        if f[2] != "pass" {
            continue;
        }
        let (class, atoms) = (f[0], f[1]);
        let ids: Vec<&str> = atoms.split(',').filter(|a| !a.is_empty()).collect();
        let ok = settings
            .lines()
            .find(|l| l.starts_with(&format!("{class}\t{atoms}\t")))
            .is_some_and(|l| {
                l.split('\t').skip(2).all(|c| {
                    let v: Vec<&str> = c.split(':').collect();
                    v[1] != "0" && v[4].parse::<f64>().is_ok_and(|x| x >= GATE)
                })
            });
        let reach: f64 = f[8].parse().expect("reach");
        let late10: f64 = f[9].parse().expect("late10");
        let (cw, aw) = word_counts(&ids, class);
        let fits = cw <= 8 && aw <= 10;
        let list: Vec<String> = ids.iter().map(|a| format!("{a:?}")).collect();
        out.push_str(&format!(
            "    PassedRule {{\n        id: \"{class}:{}\",\n        class: \"{class}\",\n        atoms: &[{}],\n        fired: {},\n        precision: {},\n        n_eff: {},\n        lower: {},\n        reach: {reach},\n        late10: {late10},\n        late19: {},\n        late: {},\n        too_rare: {},\n        settings_ok: {ok},\n        fits_budget: {fits},\n        ships: {},\n    }},\n",
            ids.join("+"),
            list.join(", "),
            f[4],
            f[5],
            f[6],
            f[7],
            f[10],
            late10 >= 0.80,
            reach < 1.0 / 40.0,
            fits && !NOT_SHIPPED.contains(&class),
        ));
    }
    out.push_str("];\n");
    out
}
