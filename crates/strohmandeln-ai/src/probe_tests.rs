use super::*;
use crate::testutil::{c, know, walk};

const NO: [&str; 3] = ["", "", ""];

fn small() -> Config {
    let mut config = Config::default();
    config.budget.nodes = 2_000;
    config.budget.worlds = 3;
    config.budget.bid_worlds = 4;
    config
}

#[test]
fn the_pair_decision_is_what_the_opponent_decides_at_every_skill() {
    // The shared search at rung 4 must be `Opponent::decide`, for the style
    // and for Normal alike, or the probe would measure something else.
    let normal = small();
    for halves in [2u8, 4, 6, 7, 8] {
        let level = Level::from_halves(halves);
        for style in [Style::Bermann, Style::Cautious, Style::Ms] {
            let cfg = styled(&normal, style);
            let (s, n) = (
                Opponent::new(level, 9).with_config(cfg),
                Opponent::new(level, 9).with_config(normal),
            );
            for seed in 0..2 {
                walk(seed, |st, seat| {
                    let v = view(st, seat);
                    let both = decide_both(level, &cfg, &normal, 9, &v);
                    assert_eq!(both.action, s.decide(&v), "{style:?} at {halves}");
                    assert_eq!(both.normal, n.decide(&v), "Normal at {halves}");
                });
            }
        }
    }
}

#[test]
fn normal_does_not_diverge_from_normal() {
    for halves in [4u8, 6, 8] {
        let (t, rows) = run_style(Level::from_halves(halves), Style::Normal, 0..1, &small());
        assert!(t.decisions > 50, "{t:?}");
        assert_eq!(t.diverged, 0, "level {halves}");
        assert_eq!(rows.len() as u64, 2 * t.deals);
        assert!(rows.iter().all(|r| r.features.meta_diverged == 0));
    }
}

#[test]
fn a_style_diverges_from_normal_in_bids_and_plays() {
    let (t, rows) = run_style(Level::from_halves(4), Style::Bermann, 0..4, &small());
    assert!(t.diverged > 0, "{t:?}");
    assert!(t.divergence() > 0.0 && t.divergence() < 1.0);
    assert!(t.bids > 0 && t.bid_rate() >= 0.0 && t.bid_rate() <= 1.0);
    let by_rows: u32 = rows.iter().map(|r| r.features.meta_diverged).sum();
    assert_eq!(
        u64::from(by_rows),
        t.diverged,
        "the rows add up to the tally"
    );
    let decisions: u32 = rows.iter().map(|r| r.features.meta_decisions).sum();
    assert_eq!(u64::from(decisions), t.decisions);
}

#[test]
fn rung_four_counts_the_near_ties_inside_each_band() {
    let (t, _) = run_style(Level::from_halves(8), Style::Bermann, 0..1, &small());
    assert!(t.searched > 0, "{t:?}");
    assert!(
        t.in_normal_band <= t.in_style_band && t.in_style_band <= t.searched,
        "the wider band holds the narrower: {t:?}"
    );
    // Below rung 4 there is no search to count.
    let (t3, _) = run_style(Level::from_halves(4), Style::Bermann, 0..1, &small());
    assert_eq!(t3.searched, 0);
}

#[test]
fn the_tells_are_counted_from_the_decisions() {
    let play = |card: &str| Action::Play {
        card: c(card),
        announce: Vec::new(),
    };
    // Leading a tarock with six tarocks in hand, early.
    let k = know(
        &["T2", "T3", "T4", "T5", "T6", "T7", "9c"],
        NO,
        NO,
        &[],
        None,
    );
    let mut f = Features::default();
    f.observe(&k, &play("T2"));
    assert_eq!((f.tarock_lead_ge5, f.tarock_lead_early), (1, 1));
    // Leading a suit card is neither.
    let mut f = Features::default();
    f.observe(&k, &play("9c"));
    assert_eq!((f.tarock_lead_ge5, f.tarock_lead_early), (0, 0));
    // The Pagat early with seven tarocks.
    let k = know(
        &["T1", "T2", "T3", "T4", "T5", "T6", "T7", "9c"],
        NO,
        NO,
        &[],
        None,
    );
    let mut f = Features::default();
    f.observe(&k, &play("T1"));
    assert_eq!((f.pagat_early_78, f.pagat_early_10), (1, 0));
    // The Mond early.
    let k = know(&["T21", "T2", "9c"], NO, NO, &[], None);
    let mut f = Features::default();
    f.observe(&k, &play("T21"));
    assert_eq!(f.mond_early, 1);
    // A lying Dame led with no low card of his showing, then with two.
    let k = know(&["7c"], ["Qh", "", ""], NO, &[], None);
    let mut f = Features::default();
    f.observe(&k, &play("Qh"));
    assert_eq!((f.figure_blank, f.figure_two_low), (1, 0));
    let k = know(&["7c"], ["Qh", "", ""], ["2h", "3h", ""], &[], None);
    let mut f = Features::default();
    f.observe(&k, &play("Qh"));
    assert_eq!((f.figure_blank, f.figure_two_low), (0, 1));
    // The Mond maxims: cashed on a rich trick, and a small tarock kept.
    let k = know(&["T21", "T5", "T2", "7c"], NO, NO, &[], Some("Kh"));
    let mut f = Features::default();
    f.observe(&k, &play("T21"));
    assert_eq!(f.cash_mond, 1);
    let k = know(&["T21", "T5", "T2", "7c"], NO, NO, &[], None);
    let mut f = Features::default();
    f.observe(&k, &play("7c"));
    assert_eq!(f.guard_mond_kept, 1);
    f.observe(&k, &play("T2"));
    assert_eq!(
        f.guard_mond_kept, 1,
        "playing the small tarock keeps nothing"
    );
    // The fork position led from with the higher spare card.
    let k = know(&["Kh", "Ch", "2h", "4h", "7c"], NO, NO, &[], None);
    let mut f = Features::default();
    f.observe(&k, &play("2h"));
    assert_eq!(f.spare_lead, 1);
    f.observe(&k, &play("4h"));
    assert_eq!(f.spare_lead, 1);
    // A bid with few tarocks.
    let k = know(&["Kh", "Kd", "T2", "T3"], NO, NO, &[], None);
    let mut f = Features::default();
    f.observe(&k, &Action::Take);
    f.observe(&k, &Action::Pass);
    assert_eq!((f.bid_decisions, f.bid_taken, f.bid_taken_le5), (2, 1, 1));
}

#[test]
fn a_calibrated_margin_bids_the_share_asked_of_the_sample() {
    let cal = Calibration {
        hands: Vec::new(),
        advantages: (0..100).map(|i| f64::from(i) - 50.0).collect(),
    };
    for share in [0.1, 0.25, 0.5, 0.9] {
        let m = cal.margin(share);
        assert!((cal.share_at(m) - share).abs() < 1e-9, "{share}: {m}");
    }
    assert_eq!(cal.share_at(cal.margin(0.0)), 0.0);
    assert_eq!(cal.share_at(cal.margin(1.0)), 1.0);
    // A higher share is a lower margin.
    assert!(cal.margin(0.2) > cal.margin(0.6));
}

#[test]
fn a_calibration_over_real_hands_gives_each_rule_its_share() {
    let cal = Calibration::sample(0..60, 4);
    assert_eq!((cal.hands.len(), cal.advantages.len()), (60, 60));
    let normal = cal.share(crate::rung2::BID_RULE);
    let bermann = cal.share(styled(&small(), Style::Bermann).bid);
    assert!((0.0..=1.0).contains(&normal) && (0.0..=1.0).contains(&bermann));
    assert_ne!(normal, bermann, "the rules differ on 60 hands");
    // A rule that bids less gets the higher margin (ties among the sampled
    // advantages keep the share from being exact on a sample this small).
    let (lo, hi) = (
        cal.margin(bermann.max(normal)),
        cal.margin(bermann.min(normal)),
    );
    assert!(lo <= hi, "{lo} {hi}");
}

#[test]
fn csv_lines_match_their_header() {
    let (_, rows) = run_style(Level::from_halves(4), Style::Hunter, 0..1, &small());
    let cols = csv_header().split(',').count();
    assert_eq!(cols, 5 + Features::COLUMNS.len());
    // The public features come first; what only the probe knows is `meta_`.
    let names: Vec<&str> = Features::COLUMNS.to_vec();
    let first_meta = names.iter().position(|n| n.starts_with("meta_")).unwrap();
    assert!(names[first_meta..].iter().all(|n| n.starts_with("meta_")));
    assert_eq!(&names[first_meta..], ["meta_decisions", "meta_diverged"]);
    assert!(!names.contains(&"decisions") && !names.contains(&"diverged"));
    for r in &rows {
        let line = csv_line(Style::Hunter, Level::from_halves(4), r);
        assert_eq!(line.split(',').count(), cols, "{line}");
        assert!(line.starts_with("hunter,2,0,"), "{line}");
    }
    assert!(!rows.is_empty());
}

fn args(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn the_options_default_to_the_stage_0_plan() {
    let o = Options::parse(&[]).unwrap();
    assert_eq!(
        (o.deals, o.seed, o.calibrate_only),
        (1000, 5_000_000, false)
    );
    assert_eq!(o.skills, [3.0, 4.0]);
    assert_eq!(o.styles.len(), 8);
    assert!(!o.styles.contains(&Style::Random));
}

#[test]
fn calibrate_only_is_a_flag_and_an_empty_skills_list_is_an_error() {
    let o = Options::parse(&args(&["--calibrate-only", "--calib-hands", "50"])).unwrap();
    assert!(o.calibrate_only);
    assert_eq!(o.calib_hands, Some(50));
    for bad in [
        vec!["--skills", ""],
        vec!["--skills", "3,,4"],
        vec!["--skills", "x"],
        vec!["--styles", "random"],
        vec!["--styles", ""],
        vec!["--deals", "x"],
        vec!["--nope"],
        vec!["--deals"],
    ] {
        assert!(Options::parse(&args(&bad)).is_err(), "{bad:?}");
    }
    // Empty skills are fine when only calibrating: no play is asked for.
    assert!(Options::parse(&args(&["--calibrate-only", "--skills", ""])).is_ok());
}
