#!/usr/bin/env python3
"""Fail unless CREDITS.md names every direct dependency: web3d/package.json
(dependencies and devDependencies) and each crate's Cargo.toml (dependencies,
dev-dependencies, build-dependencies; path dependencies are our own).
Run by bin/gate. Usage: credits_check.py [repo-root]"""

import json
import re
import sys
import tomllib
from pathlib import Path

CARGO_TABLES = ("dependencies", "dev-dependencies", "build-dependencies")


def dependencies(root):
    root = Path(root)
    found = set()
    pkg = json.loads((root / "web3d" / "package.json").read_text())
    for table in ("dependencies", "devDependencies"):
        found |= set(pkg.get(table, {}))
    # [workspace.dependencies] entries that are our own crates (path) need no credit
    ws = tomllib.loads((root / "Cargo.toml").read_text()) if (root / "Cargo.toml").exists() else {}
    own = {n for n, s in ws.get("workspace", {}).get("dependencies", {}).items()
           if isinstance(s, dict) and "path" in s}
    for manifest in sorted(root.glob("crates/*/Cargo.toml")):
        data = tomllib.loads(manifest.read_text())
        for table in CARGO_TABLES:
            for name, spec in data.get(table, {}).items():
                if name not in own and not (isinstance(spec, dict) and "path" in spec):
                    found.add(name)
    return found


def missing(root):
    text = (Path(root) / "CREDITS.md").read_text().lower()
    return sorted(
        d for d in dependencies(root)
        if not re.search(r"(?<![\w-])" + re.escape(d.lower()) + r"(?![\w-])", text)
    )


if __name__ == "__main__":
    root = sys.argv[1] if len(sys.argv) > 1 else Path(__file__).resolve().parent.parent
    gone = missing(root)
    if gone:
        print("CREDITS.md does not mention: " + ", ".join(gone), file=sys.stderr)
        sys.exit(1)
    print("CREDITS.md names every direct dependency")
