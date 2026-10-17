"""Tests for styles_stage1.  Run: python3 -I research/sim/test_styles_stage1.py"""
import os
import sys
import tempfile
import unittest
import numpy as np

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import styles_stage1 as s1

FEATS = ["tarock_lead_ge5", "bid_decisions", "bid_taken", "bid_taken_le5", "split_open"]


def synth(styles, n=120, meta_shift=None, seed=0):
    """styles: {name: Poisson mean of bid_taken per deal}; other features are
    identical noise; the meta_diverged mean can differ per style."""
    rng = np.random.default_rng(seed)
    rows = []
    for st, lam in styles.items():
        for i in range(n):
            for deal in range(7):
                for seat in range(2):
                    pad = deal == 6
                    r = {"style": st, "skill": 3, "seed": i, "deal": deal, "seat": seat,
                         "tarock_lead_ge5": 0 if pad else rng.poisson(2),
                         "bid_decisions": 0 if pad else 1,
                         "bid_taken": 0 if pad else rng.poisson(lam),
                         "bid_taken_le5": 0,
                         "split_open": 0 if pad else rng.poisson(3),
                         "meta_decisions": 0 if pad else 24,
                         "meta_diverged": 0 if pad else int(rng.poisson((meta_shift or {}).get(st, 1)))}
                    rows.append({k: str(v) for k, v in r.items()})
    return rows


class T(unittest.TestCase):
    def test_features_exclude_meta_and_ids(self):
        self.assertEqual(s1.feature_names(synth({"a": 0.2}, n=1)), FEATS)

    def test_legacy_columns_become_meta(self):
        with tempfile.NamedTemporaryFile("w", suffix=".csv", delete=False) as f:
            f.write("style,skill,seed,deal,seat,bid_taken,decisions,diverged\nnormal,3,1,0,0,1,20,0\n")
        rows = s1.load(f.name)
        os.unlink(f.name)
        self.assertEqual(s1.feature_names(rows), ["bid_taken"])

    def test_meta_never_used(self):
        rows = synth({"a": 0.3, "b": 0.3}, meta_shift={"a": 1, "b": 9})
        sit = s1.sittings(rows)
        self.assertLess(abs(s1.cv_auc(sit[("a", 3)], sit[("b", 3)], reps=3) - 0.5), 0.1)

    def test_one_feature_difference_separates(self):
        sit = s1.sittings(synth({"a": 0.1, "b": 1.5}))
        self.assertGreater(s1.cv_auc(sit[("a", 3)], sit[("b", 3)], reps=3), 0.95)

    def test_identical_near_half(self):
        sit = s1.sittings(synth({"a": 0.4, "b": 0.4}))
        self.assertLess(abs(s1.cv_auc(sit[("a", 3)], sit[("b", 3)], reps=3) - 0.5), 0.1)

    def test_null_control_halves(self):
        sit = s1.sittings(synth({"normal": 0.4}))
        self.assertLess(abs(s1.null_control(sit, None, 3, reps=3) - 0.5), 0.1)

    def test_padding_dropped_and_rates_added(self):
        sit = s1.sittings(synth({"a": 0.4}, n=3))
        v = sit[("a", 3)]["0"][0]
        self.assertEqual(len(v), len(FEATS) + 2)
        self.assertEqual(v[FEATS.index("bid_decisions")], 6)

    def test_auc_known(self):
        self.assertEqual(s1.auc([1, 2, 3, 4], [0, 0, 1, 1]), 1.0)
        self.assertEqual(s1.auc([1, 1, 1, 1], [0, 0, 1, 1]), 0.5)
        self.assertEqual(s1.auc([4, 3, 2, 1], [0, 0, 1, 1]), 0.0)

    def test_logistic_direction(self):
        w = s1.fit_logistic(np.array([[-2.], [-1.], [1.], [2.]]), np.array([0, 0, 1, 1]))
        self.assertGreater(w[0], 0)


if __name__ == "__main__":
    unittest.main()
