import tempfile
import unittest
from pathlib import Path

import credits_check as cc


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


if __name__ == "__main__":
    unittest.main()
