//! Normal plays as it did before styles existed: a hash of every decision of
//! the opponent over seeded sittings, recorded from the code of 15 Oct 2026
//! (`docs/STYLES.md` 1.1: "Normal at every level reproduces today's
//! decisions"). Levels 2, 3 and 4 only: level 1 changes meaning.

use crate::opponent::{Config, Level, Opponent};
use crate::testutil::walk;
use strohmandeln_core::view::view;

/// FNV-1a over the decisions of `o` at every view of `seeds` random sittings.
fn decisions_hash(o: &Opponent, seeds: u64) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for seed in 0..seeds {
        walk(seed, |s, seat| {
            for b in format!("{:?}", o.decide(&view(s, seat))).bytes() {
                h ^= u64::from(b);
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
        });
    }
    h
}

fn small() -> Config {
    let mut c = Config::default();
    c.budget.nodes = 2_000;
    c.budget.worlds = 3;
    c.budget.bid_worlds = 4;
    c
}

#[test]
fn normal_plays_as_it_did_before_styles_at_levels_2_3_and_4() {
    let at = |halves: u8, seeds| {
        let o = Opponent::new(Level::from_halves(halves), 11).with_config(small());
        decisions_hash(&o, seeds)
    };
    assert_eq!(at(4, 8), 5553358986496164697, "level 2");
    assert_eq!(at(6, 3), 9087963498525432781, "level 3");
    assert_eq!(at(8, 2), 3904573716754122553, "level 4");
}
