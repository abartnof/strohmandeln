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
/// points left are "at most" (HINTS 5.2); thirds are `Card::thirds`.
pub const ATOMS: &[Atom] = &[
    atom!("tu0", "tu", "no tarock is out", "kein Tarock draußen ist", |k, _| k.unseen_tarocks() == 0),
    atom!("tu2", "tu", "2 or fewer tarocks are out", "höchstens 2 Tarock draußen sind", |k, _| k.unseen_tarocks() <= 2),
    atom!("tu4", "tu", "4 or fewer tarocks are out", "höchstens 4 Tarock draußen sind", |k, _| k.unseen_tarocks() <= 4),
    atom!("tu6", "tu", "6 or fewer tarocks are out", "höchstens 6 Tarock draußen sind", |k, _| k.unseen_tarocks() <= 6),
    atom!("tu8", "tu", "8 or fewer tarocks are out", "höchstens 8 Tarock draußen sind", |k, _| k.unseen_tarocks() <= 8),
    atom!("mt6", "mt", "you hold 6 or more tarocks", "Sie 6 oder mehr Tarock halten", |k, _| k.my_tarocks() >= 6),
    atom!("mt8", "mt", "you hold 8 or more tarocks", "Sie 8 oder mehr Tarock halten", |k, _| k.my_tarocks() >= 8),
    atom!("mt10", "mt", "you hold 10 or more tarocks", "Sie 10 oder mehr Tarock halten", |k, _| k.my_tarocks() >= 10),
    atom!("mt12", "mt", "you hold 12 or more tarocks", "Sie 12 oder mehr Tarock halten", |k, _| k.my_tarocks() >= 12),
    atom!("ml5", "ml", "you hold 5 or fewer tarocks", "Sie höchstens 5 Tarock halten", |k, _| k.my_tarocks() <= 5),
    atom!("ml7", "ml", "you hold 7 or fewer tarocks", "Sie höchstens 7 Tarock halten", |k, _| k.my_tarocks() <= 7),
    atom!("void", "void", "he lacks the suit", "ihm die Farbe fehlt", |k, f| f.is_some_and(|s| shown_void(k, s))),
    atom!("notarock", "notarock", "he has shown no tarock", "er kein Tarock gezeigt hat", |k, _| shown_no_tarock(k)),
    atom!("skus", "skus", "the Sküs is out and not yours", "der Sküs draußen und nicht Ihrer ist", |k, _| skus_out(k)),
    atom!("mond", "mond", "you hold the Mond", "Sie den Mond halten", |k, _| k.playable().contains(Card::MOND)),
    atom!("left13", "left", "the suit has 13 thirds or fewer left", "von der Farbe höchstens 13 Drittel übrig sind", |k, f| f.is_some_and(|s| left_thirds(k, s) <= 13)),
    atom!("left26", "left", "the suit has 26 thirds or fewer left", "von der Farbe höchstens 26 Drittel übrig sind", |k, f| f.is_some_and(|s| left_thirds(k, s) <= 26)),
    atom!("ph1", "ph", "it is trick 1 to 9", "es der 1. bis 9. Stich ist", |k, _| k.trick_number <= 9),
    atom!("ph2", "ph", "it is trick 10 to 18", "es der 10. bis 18. Stich ist", |k, _| (10..=18).contains(&k.trick_number)),
    atom!("ph3", "ph", "it is trick 19 to 27", "es der 19. bis 27. Stich ist", |k, _| k.trick_number >= 19),
    atom!("cov1", "cov", "you have a cover", "Sie ein Paketblatt haben", |k, _| my_covers(k) >= 1),
    atom!("cov2", "cov", "you have two covers", "Sie zwei Paketblätter haben", |k, _| my_covers(k) >= 2),
    atom!("hc1", "hc", "he shows a cover of the suit", "er ein Paketblatt der Farbe zeigt", |k, f| f.is_some_and(|s| his_covers(k, s) >= 1)),
    atom!("hc2", "hc", "he shows two covers of the suit", "er zwei Paketblätter der Farbe zeigt", |k, f| f.is_some_and(|s| his_covers(k, s) >= 2)),
    atom!("led1", "led", "a card is led", "angespielt wurde", |k, _| led_at_least(k, 1)),
    atom!("led4", "led", "he led 4 thirds or more", "er 4 Drittel oder mehr anspielte", |k, _| led_at_least(k, 4)),
    atom!("led7", "led", "he led 7 thirds or more", "er 7 Drittel oder mehr anspielte", |k, _| led_at_least(k, 7)),
    atom!("led10", "led", "he led 10 thirds or more", "er 10 Drittel oder mehr anspielte", |k, _| led_at_least(k, 10)),
    atom!("led13", "led", "he led 13 thirds", "er 13 Drittel anspielte", |k, _| led_at_least(k, 13)),
    atom!("master", "master", "you hold the top card of the suit", "Sie die höchste Karte der Farbe halten", |k, f| f.is_some_and(|s| holds_master(k, s))),
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
    /// The action, English and German (infinitive).
    pub en: &'static str,
    pub de: &'static str,
    pub focus: fn(&Knowledge) -> Option<Suit>,
    /// `None` where the idea's trigger does not fire, else whether the card
    /// played is in the class.
    pub check: fn(&Knowledge, &Action) -> Option<bool>,
}

const A_POOR: &[&str] = &["tu4", "tu8", "mt8", "ml7", "ph1", "ph3", "left13", "left26", "hc1", "master", "skus", "led4"];
const A_CHEAP: &[&str] = &["tu4", "tu8", "mt8", "ml7", "ph2", "ph3", "left13", "left26", "master", "led4", "led7", "cov1"];
const A_OFF: &[&str] = &["cov1", "cov2", "hc1", "hc2", "tu4", "tu8", "ph1", "ph3", "left13", "left26", "master", "mt8"];
const A_RUN: &[&str] = &["tu2", "tu4", "tu6", "tu8", "mt6", "mt8", "mt10", "mt12", "notarock", "skus", "mond", "ph3"];
const A_PAGAT: &[&str] = &["tu0", "tu2", "tu4", "mt8", "mt10", "mt12", "ml5", "ml7", "skus", "mond", "ph3", "led7"];
const A_VOID: &[&str] = &["tu4", "tu8", "mt8", "ml7", "notarock", "skus", "ph1", "ph2", "ph3", "left13", "left26", "cov1"];
const A_MOND: &[&str] = &["tu0", "tu2", "tu4", "tu8", "mt8", "mt10", "skus", "ph2", "ph3", "led4", "led7", "notarock"];
const A_TRUMPS: &[&str] = &["tu2", "tu4", "tu6", "tu8", "mt6", "mt8", "mt10", "ml7", "notarock", "skus", "mond", "ph3"];

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
    ($id:expr, $idea:expr, $atoms:expr, $en:expr, $de:expr, $focus:expr, $check:expr) => {
        Class {
            id: $id,
            idea: $idea,
            atoms: $atoms,
            en: $en,
            de: $de,
            focus: $focus,
            check: $check,
        }
    };
}

/// The ideas and their classes (`docs/HINTS.md` 5.2). The triggers are M5's
/// (`rules::CANDIDATES`) without their numbers: the atoms say the numbers.
pub const CLASSES: &[Class] = &[
    class!("worth.poor/no-king", "worth.poor", A_POOR, "on a low lead, do not win with a king", "bei niedrigem Anspiel nicht mit einem König stechen", focus_led, |k, a| rules::poor_king(k, a, 4)),
    class!("cheap/outcome", "cheap", A_CHEAP, "play the cheapest card that gives the same result", "die billigste Karte mit gleichem Ergebnis spielen", focus_led, |k, a| rules::cheapest(k, a, false)),
    class!("cheap/winner", "cheap", A_CHEAP, "win with the cheapest card that wins", "mit der billigsten stechenden Karte stechen", focus_led, |k, a| rules::cheapest(k, a, true)),
    class!("off/follow-cover", "off", A_OFF, "follow with a cover", "mit einem Paketblatt bedienen", focus_led, |k, a| rules::off_any(k, a, false)),
    class!("off/lead-cover", "off", A_OFF, "lead a cover", "ein Paketblatt anspielen", focus_lead_cover, |k, a| rules::off_any(k, a, true)),
    class!("off/figure-cover", "off", A_OFF, "play a Dame or Cavall cover", "ein Paketblatt Dame oder Cavall spielen", focus_figure, rules::figure_any),
    class!("off/figure-higher", "off", A_OFF, "play the higher Dame or Cavall cover", "das höhere Paketblatt Dame oder Cavall spielen", focus_figure, |k, a| rules::figure_played(k, a, false)),
    class!("run/top", "run", A_RUN, "lead the top of your neighbouring tarocks, else the lowest", "das höchste Ihrer benachbarten Tarock anspielen, sonst das niedrigste", focus_none, rules::run_top),
    class!("run/any", "run", A_RUN, "lead one of your neighbouring tarocks, else the lowest", "eines Ihrer benachbarten Tarock anspielen, sonst das niedrigste", focus_none, rules::run_any),
    class!("pagat/keep", "pagat", A_PAGAT, "keep the Pagat", "den Pagat behalten", focus_led, |k, a| rules::pagat_keep(k, a, 0)),
    class!("pagat/cash", "pagat", A_PAGAT, "play the Pagat", "den Pagat spielen", focus_led, |k, a| rules::pagat_cash(k, a, 99)),
    class!("void/avoid", "void", A_VOID, "do not lead the suit he lacks", "die Farbe, die ihm fehlt, nicht anspielen", focus_void, rules::void_lead),
    class!("mond/mond", "mond", A_MOND, "answer his lead with the Mond", "sein Anspiel mit dem Mond beantworten", focus_led, mond_check),
    class!("trumps/tarock", "trumps", A_TRUMPS, "lead a tarock", "ein Tarock anspielen", focus_none, |k, a| lead_split(k, a, true)),
    class!("trumps/suit", "trumps", A_TRUMPS, "lead a suit card", "eine Farbkarte anspielen", focus_none, |k, a| lead_split(k, a, false)),
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
                out.push_str(&format!("{}\t{}\t{:x}\t{}\n", cl.id, e.deal, e.mask, u8::from(e.ok)));
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
                        out.push((i, sitting_log(first + i, settings, (index0 + i) % 2 == 0)));
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

/// A candidate's precision (clustered by deal) over the firings where its
/// atoms hold, and its reach: the share of deals in which it fires.
pub fn evaluate(log: &Log, class: usize, atoms: &[usize]) -> (Precision, f64) {
    let need = mask_of(atoms);
    let mut fired = vec![0.0; log.ndeals];
    let mut sat = vec![0.0; log.ndeals];
    for e in &log.by_class[class] {
        if e.mask & need == need {
            fired[e.deal as usize] += 1.0;
            sat[e.deal as usize] += f64::from(u8::from(e.ok));
        }
    }
    let reach = fired.iter().filter(|&&f| f > 0.0).count() as f64 / log.ndeals.max(1) as f64;
    (measure::precision(&sat, &fired), reach)
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
            let always = log.by_class[class]
                .iter()
                .fold(u32::MAX, |m, e| m & e.mask);
            let (base, _) = evaluate(log, class, &[]);
            let mut ncand = 0;
            let mut eligible = 0;
            let mut best: Option<Cand> = None;
            if !log.by_class[class].is_empty() {
                for atoms in candidate_atoms(class) {
                    if mask_of(&atoms) & always != 0 {
                        continue;
                    }
                    ncand += 1;
                    let (prec, reach) = evaluate(log, class, &atoms);
                    if prec.n_eff >= MIN_N && prec.lower >= GATE {
                        eligible += 1;
                        let wider = best.as_ref().is_none_or(|b| {
                            (prec.fired, prec.lower) > (b.prec.fired, b.prec.lower)
                        });
                        if wider {
                            best = Some(Cand { atoms, prec, reach });
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

/// The one-sided boundaries at the four looks: the two-sided O'Brien-Fleming
/// boundaries of `stats::Plan { looks: 4 }`, rounded as fixed in the plan.
pub const BOUNDARIES: [f64; 4] = [4.05, 2.86, 2.34, 2.02];
/// Futility from the second look.
pub const FUTILITY_Z: f64 = -1.645;

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

/// The decision at `look` (1 to 4): pass at or above the boundary with an
/// effective sample of at least 60; fail from the second look at z at most
/// -1.645, and at the last look without a pass.
pub fn step(look: usize, p: &Precision) -> Step {
    let last = look >= BOUNDARIES.len();
    if p.fired == 0 || p.n_eff <= 0.0 {
        return if last { Step::Fail } else { Step::Continue };
    }
    let z = z_of(p);
    if z >= BOUNDARIES[look - 1] && p.n_eff >= MIN_N {
        Step::Pass
    } else if (look >= 2 && z <= FUTILITY_Z) || last {
        Step::Fail
    } else {
        Step::Continue
    }
}

// ---- wording -------------------------------------------------------------------------

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
    /// Fires in fewer than one deal in 40: true, but not counted for reach.
    pub too_rare: bool,
    /// The lower bound is at least 0.80 under every other setting.
    pub settings_ok: bool,
}

/// The "so" sentence of a rule: the atoms' clauses joined with "and", then
/// the class's action. English: "When A and B, act."; German: "Wenn A und B:
/// act."
pub fn sentence(atoms: &[&str], class: &str, de: bool) -> String {
    let cl = &CLASSES[class_index(class)];
    let clauses: Vec<&str> = atoms
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
        format!("Wenn {}: {}.", clauses.join(" und "), cl.de)
    } else {
        format!("When {}, {}.", clauses.join(" and "), cl.en)
    }
}

fn words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// The words of the condition (the atoms' clauses, without "when" and "and")
/// and of the action, in English.
pub fn word_counts(atoms: &[&str], class: &str) -> (usize, usize) {
    let cond = atoms.iter().map(|a| words(ATOMS[atom_index(a)].en)).sum();
    (cond, words(CLASSES[class_index(class)].en))
}

/// As [`word_counts`], in German.
pub fn word_counts_de(atoms: &[&str], class: &str) -> (usize, usize) {
    let cond = atoms.iter().map(|a| words(ATOMS[atom_index(a)].de)).sum();
    (cond, words(CLASSES[class_index(class)].de))
}

// ---- what the hint reads ----------------------------------------------------------------

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

/// The passed rules that hold at this decision for the card the hint
/// plays, widest first (most firings): the hint's "so" is the first.
pub fn matching(k: &Knowledge, action: &Action) -> Vec<&'static PassedRule> {
    let mut v: Vec<&'static PassedRule> = PASSED
        .iter()
        .filter(|r| rule_matches(r, k, action))
        .collect();
    v.sort_by_key(|r| std::cmp::Reverse(r.fired));
    v
}
