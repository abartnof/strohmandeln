//! The Beta distribution: the posterior of a met rate.
//!
//! From Cassino `crates/cassino-core/src/learner.rs`, unchanged but for
//! `ln_gamma` being public.

/// The natural logarithm of the gamma function (Lanczos, g = 7).
pub fn ln_gamma(x: f64) -> f64 {
    const C: [f64; 9] = [
        0.999_999_999_999_809_9,
        676.520_368_121_885_1,
        -1_259.139_216_722_402_8,
        771.323_428_777_653_1,
        -176.615_029_162_140_6,
        12.507_343_278_686_905,
        -0.138_571_095_265_720_12,
        9.984_369_578_019_572e-6,
        1.505_632_735_149_311_6e-7,
    ];
    if x < 0.5 {
        return (std::f64::consts::PI / (std::f64::consts::PI * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let t = x + 7.5;
    let sum = C[0]
        + C.iter()
            .enumerate()
            .skip(1)
            .map(|(i, c)| c / (x + i as f64))
            .sum::<f64>();
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + sum.ln()
}

/// The regularized incomplete beta function I_x(a, b): the probability that
/// a Beta(a, b) variable is at most `x`. The continued fraction of
/// Numerical Recipes (Lentz's method), on whichever side of (a + 1)/(a + b +
/// 2) converges.
pub fn beta_cdf(x: f64, a: f64, b: f64) -> f64 {
    if x <= 0.0 {
        return 0.0;
    }
    if x >= 1.0 {
        return 1.0;
    }
    let front =
        (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_fraction(x, a, b) / a
    } else {
        1.0 - front * beta_fraction(1.0 - x, b, a) / b
    }
}

fn beta_fraction(x: f64, a: f64, b: f64) -> f64 {
    const TINY: f64 = 1e-300;
    let (qab, qap, qam) = (a + b, a + 1.0, a - 1.0);
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < TINY {
        d = TINY;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..300 {
        let m = f64::from(m);
        let m2 = 2.0 * m;
        let aa = m * (b - m) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa = -(a + m) * (qab + m) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let delta = d * c;
        h *= delta;
        if (delta - 1.0).abs() < 1e-14 {
            break;
        }
    }
    h
}

/// The `p` quantile of Beta(a, b), by bisection on [`beta_cdf`] (the cdf is
/// monotone; 60 halvings give 1e-18).
pub fn beta_quantile(p: f64, a: f64, b: f64) -> f64 {
    let (mut lo, mut hi) = (0.0, 1.0);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if beta_cdf(mid, a, b) < p {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-9, "{a} != {b}");
    }

    #[test]
    fn the_beta_distribution_matches_known_values() {
        // Beta(1, 1) is uniform; Beta(2, 1) has cdf x^2, Beta(1, 3) has
        // 1 - (1 - x)^3.
        close(beta_cdf(0.3, 1.0, 1.0), 0.3);
        close(beta_cdf(0.3, 2.0, 1.0), 0.09);
        close(beta_cdf(0.3, 1.0, 3.0), 1.0 - 0.7f64.powi(3));
        close(beta_quantile(0.1, 9.0, 1.0), 0.1f64.powf(1.0 / 9.0));
        close(beta_quantile(0.9, 1.0, 4.0), 1.0 - 0.1f64.powf(0.25));
        // Symmetric: the median is a half, and the quantiles mirror.
        close(beta_quantile(0.5, 7.5, 7.5), 0.5);
        close(
            beta_quantile(0.1, 3.0, 8.0),
            1.0 - beta_quantile(0.9, 8.0, 3.0),
        );
        // Beta(3, 2) has cdf 4x^3 - 3x^4 (integrating 12x^2(1 - x)).
        let x: f64 = 0.6;
        close(beta_cdf(x, 3.0, 2.0), 4.0 * x.powi(3) - 3.0 * x.powi(4));
        // A large sample is near its mean, with the normal's spread.
        let (a, b): (f64, f64) = (101.0, 21.0);
        let (mean, sd) = (
            a / (a + b),
            (a * b / ((a + b).powi(2) * (a + b + 1.0))).sqrt(),
        );
        assert!((beta_quantile(0.5, a, b) - mean).abs() < 0.005);
        let q = beta_quantile(0.1, a, b);
        assert!((q - (mean - 1.2816 * sd)).abs() < 0.01, "{q}");
        // Round trip.
        for (a, b) in [(0.5, 0.5), (2.0, 30.0), (40.0, 1.0)] {
            for p in [0.05, 0.5, 0.95] {
                close(beta_cdf(beta_quantile(p, a, b), a, b), p);
            }
        }
    }

    #[test]
    fn the_log_gamma_of_integers_is_the_log_factorial() {
        // Gamma(n) = (n - 1)!; Gamma(1/2) = sqrt(pi); Gamma(1/4) * Gamma(3/4)
        // = pi * sqrt(2) (the reflection branch, below one half).
        close(ln_gamma(1.0), 0.0);
        close(ln_gamma(5.0), 24.0f64.ln());
        close(ln_gamma(10.0), 362_880.0f64.ln());
        close(ln_gamma(0.5), std::f64::consts::PI.sqrt().ln());
        close(
            ln_gamma(0.25) + ln_gamma(0.75),
            (std::f64::consts::PI * 2.0f64.sqrt()).ln(),
        );
    }

    #[test]
    fn the_edges_of_the_cdf_are_zero_and_one() {
        assert_eq!(beta_cdf(0.0, 2.0, 3.0), 0.0);
        assert_eq!(beta_cdf(-1.0, 2.0, 3.0), 0.0);
        assert_eq!(beta_cdf(1.0, 2.0, 3.0), 1.0);
        assert_eq!(beta_cdf(2.0, 2.0, 3.0), 1.0);
    }
}
