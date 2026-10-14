#!/usr/bin/env python3
"""Original computation: card points under the Tarock count always total 70.

Counting in threes minus 2 per group is the same as subtracting 2/3 per card.
Kept in exact thirds, the two players' totals sum to 70 for every split of the
54 cards (all three remainder classes of the split are exercised). Rounding each
player's total separately is what can make 35 1/3 against 34 2/3 look like a tie;
Ulmann's "36 (35 2/3)" and the Prager Tagblatt column of 12 Feb 1892 both say the
declarer with 35 1/3 has lost (research/03-period-rulebooks.md section 4).
"""
import random
from fractions import Fraction as F

VALUES = [5] * 4 + [5] * 3 + [4] * 4 + [3] * 4 + [2] * 4 + [1] * 35  # K, Trull, Q, C, J, others


def points(cards):
    return sum(F(v) for v in cards) - F(2, 3) * len(cards)


def main(trials=200_000, seed=20261014):
    rng = random.Random(seed)
    vals = VALUES[:]
    assert len(vals) == 54 and sum(vals) == 106
    for _ in range(trials):
        rng.shuffle(vals)
        n = rng.randint(0, 54)
        assert points(vals[:n]) + points(vals[n:]) == 70
    print(f"{trials} random splits: totals always exactly 70")


if __name__ == "__main__":
    main()
