"""Stage 1 of docs/STYLES.md: the tells gate.

One sample = one sitting (a seed: 3 Touren, 6 deals) and one seat, the sum of
the public per-deal features over the sitting plus two rates.  For each pair of
styles (and each skill) a ridge logistic regression is cross-validated, folds
grouped by sitting (both classes share the sitting seeds, so a fold holds the
same deals for both), out-of-fold scores pooled, AUC by rank.  Pass = AUC >= 0.8.

    python3 -I research/sim/styles_stage1.py RUN.csv [--null OTHER.csv] [--reps 5]
"""
import csv
import sys
from collections import defaultdict
import numpy as np

IDENT = {"style", "skill", "seed", "deal", "seat"}
GATE = 0.8
NOT_OFFERED = {"werner"}
ORDER = ["normal", "bermann", "ulmann", "ms", "werner", "bold", "cautious", "hunter"]


def is_meta(name):
    return name.startswith("meta_")


def load(path):
    """Rows as dicts of strings; legacy 'decisions'/'diverged' become meta_*."""
    with open(path, newline="") as f:
        rows = list(csv.DictReader(f))
    return [{("meta_" + k if k in ("decisions", "diverged") else k): v for k, v in r.items()}
            for r in rows]


def feature_names(rows):
    """Every column except identifiers and meta_*."""
    return [k for k in rows[0] if k not in IDENT and not is_meta(k)]


def add_rates(v, names):
    ix = {n: i for i, n in enumerate(names)}
    extra = []
    if "bid_taken" in ix and "bid_decisions" in ix:
        extra.append(v[ix["bid_taken"]] / max(v[ix["bid_decisions"]], 1))
    if "bid_taken_le5" in ix and "bid_taken" in ix:
        extra.append(v[ix["bid_taken_le5"]] / max(v[ix["bid_taken"]], 1))
    return np.concatenate([v, extra])


def sittings(rows, names=None):
    """{(style, skill): {seed: [vec_seat0, vec_seat1]}}.
    Padding deals (meta_decisions == 0) are dropped."""
    names = names or feature_names(rows)
    acc = defaultdict(lambda: defaultdict(dict))
    for r in rows:
        if "meta_decisions" in r and float(r["meta_decisions"]) == 0:
            continue
        v = np.array([float(r[n]) for n in names])
        d = acc[(r["style"], int(r["skill"]))][r["seed"]]
        seat = int(r["seat"])
        d[seat] = d.get(seat, 0) + v
    return {key: {s: [add_rates(v, names) for _, v in sorted(seats.items())]
                  for s, seats in d.items()} for key, d in acc.items()}


def fit_logistic(X, y, l2=1.0, iters=50):
    """Ridge logistic regression by Newton steps; X standardised by the caller."""
    n, d = X.shape
    A = np.hstack([X, np.ones((n, 1))])
    w = np.zeros(d + 1)
    reg = np.eye(d + 1) * l2
    reg[-1, -1] = 0
    for _ in range(iters):
        p = 1 / (1 + np.exp(-np.clip(A @ w, -30, 30)))
        g = A.T @ (p - y) + reg @ w
        H = A.T @ (A * (p * (1 - p))[:, None]) + reg + 1e-9 * np.eye(d + 1)
        step = np.linalg.solve(H, g)
        w -= step
        if np.abs(step).max() < 1e-8:
            break
    return w


def predict(w, X):
    return np.hstack([X, np.ones((len(X), 1))]) @ w


def auc(score, y):
    """Mann-Whitney AUC, ties counted half."""
    score = np.asarray(score, float)
    y = np.asarray(y)
    pos, neg = score[y == 1], score[y == 0]
    if len(pos) == 0 or len(neg) == 0:
        return float("nan")
    gt = (pos[:, None] > neg[None, :]).sum()
    eq = (pos[:, None] == neg[None, :]).sum()
    return float((gt + 0.5 * eq) / (len(pos) * len(neg)))


def cv_auc(A, B, k=5, reps=5, seed=1):
    """A, B: {seed: [vecs]} of two styles.  Folds by sitting seed, shared by both.
    Mean AUC over `reps` repetitions of k-fold CV (pooled out-of-fold scores)."""
    seeds = sorted(set(A) & set(B))
    out = []
    for rep in range(reps):
        perm = np.random.default_rng(seed * 1000 + rep).permutation(len(seeds))
        fold = {seeds[i]: j % k for j, i in enumerate(perm)}
        X, y, g = [], [], []
        for s in seeds:
            for lab, D in ((1, A), (0, B)):
                for v in D[s]:
                    X.append(v)
                    y.append(lab)
                    g.append(fold[s])
        X, y, g = np.array(X), np.array(y), np.array(g)
        sc = np.zeros(len(y))
        for f in range(k):
            tr, te = g != f, g == f
            mu, sd = X[tr].mean(0), X[tr].std(0)
            sd[sd == 0] = 1
            w = fit_logistic((X[tr] - mu) / sd, y[tr])
            sc[te] = predict(w, (X[te] - mu) / sd)
        out.append(auc(sc, y))
    return float(np.mean(out))


def matrix(sit, skill, styles, **kw):
    return {(a, b): cv_auc(sit[(a, skill)], sit[(b, skill)], **kw)
            for i, a in enumerate(styles) for b in styles[i + 1:]}


def null_control(sit_a, sit_b, skill, **kw):
    """Normal against Normal on disjoint sittings: two runs, or (sit_b None)
    alternate sittings of one run."""
    A = list(sit_a[("normal", skill)].values())
    if sit_b is None:
        P, Q = A[0::2], A[1::2]
    else:
        P, Q = A, list(sit_b[("normal", skill)].values())
    n = min(len(P), len(Q))
    return cv_auc(dict(enumerate(P[:n])), dict(enumerate(Q[:n])), **kw)


def report(sit, skills, styles, **kw):
    lines, results = [], {}
    for skill in skills:
        M = matrix(sit, skill, styles, **kw)
        results[skill] = M
        lines.append(f"skill {skill} AUC (* = below {GATE})")
        lines.append(" " * 10 + " ".join(f"{b:>8}" for b in styles[1:]))
        for i, a in enumerate(styles[:-1]):
            cells = []
            for j, b in enumerate(styles[1:], start=1):
                if i >= j:
                    cells.append(" " * 8)
                else:
                    v = M[(a, b)]
                    cells.append(f"{v:7.3f}{'*' if v < GATE else ' '}")
            lines.append(f"{a:>9} " + " ".join(cells))
        lines.append("")
    return "\n".join(lines), results


def main(argv):
    path = argv[1]
    nullpath = argv[argv.index("--null") + 1] if "--null" in argv else None
    reps = int(argv[argv.index("--reps") + 1]) if "--reps" in argv else 5
    rows = load(path)
    names = feature_names(rows)
    print("features used:", ", ".join(names),
          "+ rates bid_taken/bid_decisions, bid_taken_le5/bid_taken")
    sit = sittings(rows, names)
    skills = sorted({k[1] for k in sit})
    styles = [s for s in ORDER if (s, skills[0]) in sit]
    print("samples per style and skill:", sum(len(v) for v in sit[("normal", skills[0])].values()))
    text, res = report(sit, skills, styles, reps=reps)
    print(text)
    print("not offered:", ", ".join(sorted(NOT_OFFERED)))
    for skill in skills:
        print(f"null control skill {skill}, alternate Normal sittings of one run:",
              f"{null_control(sit, None, skill, reps=reps):.3f}")
    if nullpath:
        other = load(nullpath)
        common = [n for n in names if n in other[0]]
        sa, sb = sittings(rows, common), sittings(other, common)
        print(f"null control skill 3, Normal vs Normal of {nullpath.split('/')[-1]}"
              f" ({len(common)} common features):", f"{null_control(sa, sb, 3, reps=reps):.3f}")
    return res


if __name__ == "__main__":
    main(sys.argv)
