//! Sequential measurement with a stopping rule fixed before the first batch
//! (CLAUDE.md, "Simulations stop early when the signal is clear").
//!
//! A run fixes the comparison, the batch size, the most looks and the
//! boundary at each look before it draws a single observation. After each
//! batch it compares the running z-score with the O'Brien-Fleming boundary
//! for that look, which keeps the chance of a false alarm at 5% over all the
//! looks together. It stops
//!
//! - **clear**, when |z| crosses the boundary;
//! - **negligible**, from the second look on, when the whole 95% interval of
//!   the mean lies inside plus or minus `negligible` (the effect, if any, is
//!   too small to matter; stopping for futility only costs power, never
//!   inflates the false alarms);
//! - **inconclusive**, at the last look, with the interval at that look.
//!
//! Observations are mirrored pairs (each deal played twice, seats swapped),
//! so the luck of the deal cancels.

/// The O'Brien-Fleming constant for `looks` equally spaced looks at a
/// two-sided 5%: the boundary at look k is this times sqrt(looks / k).
/// From the standard group-sequential tables (cassino's table; the
/// simulation test below checks the false-alarm rate).
pub fn obf_constant(looks: usize) -> f64 {
    const C: [f64; 10] = [
        1.960, 1.977, 2.004, 2.024, 2.040, 2.053, 2.063, 2.072, 2.080, 2.087,
    ];
    assert!((1..=10).contains(&looks), "1 to 10 looks");
    C[looks - 1]
}

/// The O'Brien-Fleming constant for `looks` equally spaced looks at the
/// overall two-sided level `alpha` (the boundary at look k is this times
/// sqrt(looks / k)), by numerical integration of the joint density of the
/// partial sums (Armitage, McPherson and Rowe's recursion): on the scale of
/// the sums the O'Brien-Fleming boundary is one constant, c sqrt(looks).
/// `obf_constant` is the table at 0.05; this gives any level, as the family
/// correction of M5r needs.
pub fn obf_constant_at(alpha: f64, looks: usize) -> f64 {
    assert!(alpha > 0.0 && alpha < 1.0 && looks >= 1);
    const N: usize = 600;
    let phi = |x: f64| (-0.5 * x * x).exp() / (2.0 * std::f64::consts::PI).sqrt();
    // The chance of staying inside the boundary at every look.
    let stay = |c: f64| -> f64 {
        let b = c * (looks as f64).sqrt();
        let h = 2.0 * b / N as f64;
        let grid: Vec<f64> = (0..N).map(|i| -b + (i as f64 + 0.5) * h).collect();
        let mut f: Vec<f64> = grid.iter().map(|&s| phi(s)).collect();
        for _ in 1..looks {
            f = grid
                .iter()
                .map(|&t| {
                    grid.iter()
                        .zip(&f)
                        .map(|(&s, &fs)| fs * phi(t - s) * h)
                        .sum()
                })
                .collect();
        }
        f.iter().sum::<f64>() * h
    };
    let (mut lo, mut hi) = (1.0, 9.0);
    for _ in 0..50 {
        let mid = 0.5 * (lo + hi);
        if 1.0 - stay(mid) > alpha {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The boundaries at each of `looks` looks for overall two-sided level
/// `alpha`, rounded to two decimals.
pub fn obf_boundaries(alpha: f64, looks: usize) -> Vec<f64> {
    let c = obf_constant_at(alpha, looks);
    (1..=looks)
        .map(|k| ((c * (looks as f64 / k as f64).sqrt()) * 100.0).round() / 100.0)
        .collect()
}

/// A sequential run's design, fixed before its first batch.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Plan {
    /// Observations per batch.
    pub batch: usize,
    /// The most looks the run may take.
    pub looks: usize,
    /// An effect whose whole interval lies inside plus or minus this is
    /// "plainly negligible" (in the observation's own unit). Zero turns the
    /// futility stop off.
    pub negligible: f64,
}

impl Plan {
    /// The |z| that stops the run at each look.
    pub fn boundaries(&self) -> Vec<f64> {
        let c = obf_constant(self.looks);
        (1..=self.looks)
            .map(|k| c * (self.looks as f64 / k as f64).sqrt())
            .collect()
    }
}

/// The state of a run at one look.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Look {
    pub look: usize,
    pub n: usize,
    pub mean: f64,
    /// The standard error of the mean.
    pub se: f64,
    pub z: f64,
    pub boundary: f64,
}

impl Look {
    pub fn low(&self) -> f64 {
        self.mean - 1.96 * self.se
    }
    pub fn high(&self) -> f64 {
        self.mean + 1.96 * self.se
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Verdict {
    /// The boundary was crossed: the difference is real.
    Clear(Look),
    /// The interval lies inside the negligible band.
    Negligible(Look),
    /// Neither by the last look.
    Inconclusive(Look),
}

impl Verdict {
    /// The look the run ended at: the interval to report.
    pub fn last(&self) -> &Look {
        match self {
            Verdict::Clear(l) | Verdict::Negligible(l) | Verdict::Inconclusive(l) => l,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Report {
    pub plan: Plan,
    pub looks: Vec<Look>,
    pub verdict: Verdict,
}

/// Runs a sequential measurement. `sample(i)` gives observation `i`
/// (0, 1, 2, ...); `on_look` hears each look as it is taken.
pub fn run_sequential(
    plan: Plan,
    mut sample: impl FnMut(usize) -> f64,
    mut on_look: impl FnMut(&Look),
) -> Report {
    let boundaries = plan.boundaries();
    let mut values: Vec<f64> = Vec::with_capacity(plan.batch * plan.looks);
    let mut looks = Vec::new();
    for (k, &boundary) in boundaries.iter().enumerate() {
        for _ in 0..plan.batch {
            values.push(sample(values.len()));
        }
        let n = values.len();
        let mean = values.iter().sum::<f64>() / n as f64;
        let var = if n > 1 {
            values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / (n - 1) as f64
        } else {
            0.0
        };
        let se = (var / n as f64).sqrt();
        let z = if se > 0.0 {
            mean / se
        } else if mean == 0.0 {
            0.0
        } else {
            mean.signum() * f64::INFINITY
        };
        let look = Look {
            look: k + 1,
            n,
            mean,
            se,
            z,
            boundary,
        };
        on_look(&look);
        looks.push(look);
        let verdict = if z.abs() >= boundary {
            Some(Verdict::Clear(look))
        } else if k >= 1
            && plan.negligible > 0.0
            && look.low() > -plan.negligible
            && look.high() < plan.negligible
        {
            Some(Verdict::Negligible(look))
        } else if k + 1 == boundaries.len() {
            Some(Verdict::Inconclusive(look))
        } else {
            None
        };
        if let Some(verdict) = verdict {
            return Report {
                plan,
                looks,
                verdict,
            };
        }
    }
    unreachable!("the last look always ends the run")
}

#[cfg(test)]
mod tests {
    #[test]
    fn the_constant_at_any_level_matches_the_table() {
        for looks in 1..=5 {
            let c = obf_constant_at(0.05, looks);
            assert!((c - obf_constant(looks)).abs() < 0.01, "{looks}: {c}");
        }
        assert_eq!(obf_boundaries(0.05, 4)[0], 4.05);
        let strict = obf_boundaries(0.05 / 7.0, 4);
        let plain = obf_boundaries(0.05, 4);
        for (s, p) in strict.iter().zip(&plain) {
            assert!(s > p);
        }
        assert!(strict[3] > 2.6 && strict[3] < 2.8, "{strict:?}");
    }

    use super::*;
    use strohmandeln_core::rng::Rng;

    /// A standard normal draw (Box-Muller).
    fn normal(rng: &mut Rng) -> f64 {
        let u = (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        let v = (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64;
        (-2.0 * (1.0 - u).ln()).sqrt() * (2.0 * std::f64::consts::PI * v).cos()
    }

    fn plan(batch: usize, looks: usize) -> Plan {
        Plan {
            batch,
            looks,
            negligible: 0.0,
        }
    }

    #[test]
    fn the_boundaries_are_obrien_flemings() {
        let b = plan(100, 4).boundaries();
        for (got, want) in b.iter().zip([4.05, 2.86, 2.34, 2.02]) {
            assert!((got - want).abs() < 0.01, "{b:?}");
        }
        assert_eq!(plan(1, 1).boundaries(), vec![1.96]);
    }

    #[test]
    fn the_boundaries_keep_false_alarms_at_five_percent() {
        let mut rng = Rng::seeded(2026);
        let runs = 20_000;
        let mut alarms = 0;
        for _ in 0..runs {
            let r = run_sequential(plan(50, 4), |_| normal(&mut rng), |_| {});
            alarms += usize::from(matches!(r.verdict, Verdict::Clear(_)));
        }
        let rate = alarms as f64 / runs as f64;
        assert!((0.042..0.058).contains(&rate), "false alarm rate {rate}");
    }

    #[test]
    fn a_clear_effect_stops_early_and_reports_the_look_it_stopped_at() {
        let mut rng = Rng::seeded(7);
        let mut early = 0;
        for _ in 0..200 {
            let r = run_sequential(plan(50, 4), |_| 0.5 + normal(&mut rng), |_| {});
            match r.verdict {
                Verdict::Clear(l) => {
                    assert!(l.mean > 0.0);
                    assert_eq!(r.looks.len(), l.look);
                    early += usize::from(l.look <= 2);
                }
                v => panic!("missed a clear effect: {v:?}"),
            }
        }
        assert!(early > 180, "{early} of 200 stopped by the second look");
    }

    #[test]
    fn a_plainly_negligible_effect_stops_before_the_last_look() {
        let mut rng = Rng::seeded(8);
        let p = Plan {
            batch: 200,
            looks: 5,
            negligible: 0.5,
        };
        let r = run_sequential(p, |_| normal(&mut rng), |_| {});
        match r.verdict {
            Verdict::Negligible(l) => {
                assert!(l.look >= 2 && l.look < 5, "{l:?}");
                assert!(l.low() > -0.5 && l.high() < 0.5);
            }
            v => panic!("{v:?}"),
        }
    }

    #[test]
    fn the_futility_stop_is_not_taken_at_the_first_look() {
        let p = Plan {
            batch: 500,
            looks: 3,
            negligible: 5.0,
        };
        let mut rng = Rng::seeded(1);
        let r = run_sequential(p, |_| normal(&mut rng), |_| {});
        assert_eq!(r.verdict.last().look, 2);
    }

    #[test]
    fn no_effect_and_no_band_reports_the_interval_at_the_last_look() {
        let mut rng = Rng::seeded(8);
        let r = run_sequential(plan(200, 2), |_| normal(&mut rng), |_| {});
        let Verdict::Inconclusive(l) = r.verdict else {
            panic!("{:?}", r.verdict)
        };
        assert_eq!(l.n, 400);
        assert!(l.low() < 0.0 && 0.0 < l.high() && (l.high() - l.low()) < 0.3);
        assert_eq!(r.looks.len(), 2);
    }

    #[test]
    fn identical_observations_are_no_difference_and_a_constant_gap_is_clear() {
        let r = run_sequential(plan(10, 3), |_| 0.0, |_| {});
        assert!(matches!(r.verdict, Verdict::Inconclusive(l) if l.mean == 0.0));
        let r = run_sequential(plan(10, 3), |_| 2.0, |_| {});
        assert!(matches!(r.verdict, Verdict::Clear(l) if l.look == 1));
    }

    #[test]
    fn looks_are_heard_as_they_are_taken() {
        let mut heard = Vec::new();
        let mut rng = Rng::seeded(9);
        run_sequential(plan(20, 3), |_| normal(&mut rng), |l| heard.push(l.n));
        assert_eq!(heard, vec![20, 40, 60]);
    }
}
