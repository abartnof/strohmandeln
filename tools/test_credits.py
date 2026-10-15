import tempfile
import re
import unittest
from pathlib import Path

import credits_check as cc
import phrases

ROOT = Path(__file__).resolve().parent.parent


def repo(credits, package=None, crates=None):
    d = Path(tempfile.mkdtemp())
    (d / "web3d").mkdir()
    (d / "web3d" / "package.json").write_text(
        package or '{"dependencies":{"three":"1"},"devDependencies":{"esbuild":"2"}}'
    )
    default = (
        '[package]\nname="a"\n[dependencies]\nserde = { version = "1" }\n'
        'local = { path = "../b" }\n[dev-dependencies]\nserde_json = "1"\n'
    )
    for name, toml in (crates or {"a": default}).items():
        (d / "crates" / name).mkdir(parents=True)
        (d / "crates" / name / "Cargo.toml").write_text(toml)
    (d / "CREDITS.md").write_text(credits)
    return d


FULL = "three esbuild serde serde_json"


class CreditsCheck(unittest.TestCase):
    def test_all_named_passes(self):
        self.assertEqual(cc.missing(repo(FULL)), [])

    def test_missing_npm_dependency_is_reported(self):
        self.assertEqual(cc.missing(repo("esbuild serde serde_json")), ["three"])

    def test_missing_dev_dependency_of_a_crate_is_reported(self):
        self.assertEqual(cc.missing(repo("three esbuild serde")), ["serde_json"])

    def test_path_dependencies_need_no_credit(self):
        self.assertNotIn("local", cc.dependencies(repo(FULL)))

    def test_a_name_inside_a_longer_word_does_not_count(self):
        self.assertEqual(cc.missing(repo("three esbuild serde_json_extra serde")), ["serde_json"])

    def test_no_credits_file_is_an_error(self):
        d = repo(FULL)
        (d / "CREDITS.md").unlink()
        with self.assertRaises(FileNotFoundError):
            cc.missing(d)

    def test_workspace_path_dependency_needs_no_credit(self):
        d = repo(FULL, crates={"a": '[dependencies]\nown.workspace = true\nserde = "1"\n'
                                    '[dev-dependencies]\nserde_json = "1"\n'})
        (d / "Cargo.toml").write_text('[workspace.dependencies]\nown = { path = "crates/own" }\n')
        self.assertEqual(sorted(cc.dependencies(d)), ["esbuild", "serde", "serde_json", "three"])

    def test_workspace_registry_dependency_is_credited(self):
        d = repo("three esbuild", crates={"a": "[dependencies]\nrand.workspace = true\n"})
        (d / "Cargo.toml").write_text('[workspace.dependencies]\nrand = "0.8"\n')
        self.assertEqual(cc.missing(d), ["rand"])


class RealRepo(unittest.TestCase):
    def test_every_crate_is_scanned(self):
        # strohmandeln-wasm alone brings serde_json in as a direct dependency
        self.assertTrue({"three", "esbuild", "serde", "serde_json"} <= cc.dependencies(ROOT))
        self.assertEqual(cc.missing(ROOT), [])

    def test_credits_names_the_source_of_every_phrase(self):
        """Each source of docs/PHRASES.md is named in CREDITS.md's period sources."""
        text = " ".join((ROOT / "CREDITS.md").read_text().lower().split())
        alias = {"(Neuigkeits) Welt Blatt": "Welt Blatt"}
        absent = []
        for key, (_, _, citation, _) in phrases.SOURCES.items():
            if key == "T":
                continue
            lead = re.split(r"[,(:]| \(", citation)[0].strip() or citation
            lead = alias.get(citation.split(",")[0].split(":")[0].strip(), lead)
            lead = lead.split(" as printed in ")[-1]
            if lead.lower() not in text:
                absent.append((key, lead))
        self.assertEqual(absent, [])


if __name__ == "__main__":
    unittest.main()
