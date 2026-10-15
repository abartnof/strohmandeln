//! What the constants must satisfy to be learnt with.

mod common;

use common::*;
use kit_learner::{learn, Params};

#[test]
fn cassinos_constants_pass() {
    assert_eq!(PARAMS.check(), Ok(()));
}

#[test]
fn constants_that_cannot_work_are_refused() {
    let with = |f: fn(&mut Params)| {
        let mut p = PARAMS;
        f(&mut p);
        p.check()
    };
    for (what, got) in [
        ("discount nan", with(|p| p.discount = f64::NAN)),
        ("discount zero", with(|p| p.discount = 0.0)),
        ("discount above one", with(|p| p.discount = 1.01)),
        ("lower zero", with(|p| p.lower = 0.0)),
        ("upper one", with(|p| p.upper = 1.0)),
        ("lower above upper", with(|p| p.lower = 0.95)),
        ("lower is upper", with(|p| p.lower = 0.9)),
        ("min_gain negative", with(|p| p.min_gain = -0.1)),
        ("min_gain inf", with(|p| p.min_gain = f64::INFINITY)),
        ("costly_gain nan", with(|p| p.costly_gain = f64::NAN)),
        ("costly_chances negative", with(|p| p.costly_chances = -1.0)),
        ("master_of_top zero", with(|p| p.master_of_top = 0.0)),
        ("master_of_top above one", with(|p| p.master_of_top = 1.1)),
        ("lose_below negative", with(|p| p.lose_below = -0.2)),
    ] {
        assert!(got.is_err(), "{what} was accepted");
    }
    // The edges that are allowed: no discount at all, no hysteresis.
    assert_eq!(with(|p| p.discount = 1.0), Ok(()));
    assert_eq!(with(|p| p.lose_below = 0.0), Ok(()));
    assert_eq!(with(|p| p.min_gain = 0.0), Ok(()));
}

#[test]
fn the_error_names_the_constant() {
    let mut p = PARAMS;
    p.discount = f64::NAN;
    assert!(p.check().unwrap_err().contains("discount"));
    p = PARAMS;
    p.lower = 0.95;
    let e = p.check().unwrap_err();
    assert!(e.contains("lower") && e.contains("upper"), "{e}");
}

#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "discount")]
fn learn_asserts_its_constants_in_debug() {
    let mut p = PARAMS;
    p.discount = f64::NAN;
    learn(&LADDER, &p, &[]);
}
