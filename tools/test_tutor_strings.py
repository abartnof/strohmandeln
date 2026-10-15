"""The tutor's words pass the phrase bank's kindness test (CLAUDE.md: luck,
speed, the score and the run of play are fair; looks, intelligence, age,
gender, class, religion, nationality never are).

    python3 -m unittest tools.test_tutor_strings

The tutor's strings are the `tutor.*` keys of web3d/strings.json that
docs/MESSAGES.md "Tutor keys" documents, in English and German. The same
banned words as the phrase bank's (tools/test_phrases.py BANNED), with the
exemptions below, each a word of the game and not about a person.
"""

import json
import re
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import test_phrases  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
STRINGS = json.loads((ROOT / "web3d" / "strings.json").read_text(encoding="utf8"))
DOC = (ROOT / "docs" / "MESSAGES.md").read_text(encoding="utf8")

# Not about people: "Dame" is a card (the queen of the German pack), "Damen"
# its plural, "Herz" a suit. Listed with the German word they would match.
GAME_WORDS_DE = {"dame", "damen", "man"}
# "queen" is not banned; nothing in English is exempt.
GAME_WORDS_EN: set = set()

KEYS = re.findall(r"^\| `([a-z0-9.-]+)` \|", DOC.split("\n## Tutor keys")[1], re.M)


def texts(key):
    return STRINGS[key]["en"], STRINGS[key]["de"]


class TutorKindness(unittest.TestCase):
    def test_there_are_tutor_keys(self):
        self.assertGreaterEqual(len(KEYS), 105)

    def test_banned_words_and_topics(self):
        for key in KEYS:
            for i, text in enumerate(texts(key)):
                # the game's own placeholders and card names are not words of ours
                text = re.sub(r"\{\w+\}", "", text)
                for topic, rx in test_phrases.BANNED.items():
                    m = rx.search(text)
                    if not m:
                        continue
                    word = m.group(0).lower()
                    if i == 1 and (word in test_phrases.GERMAN_OK or word in GAME_WORDS_DE):
                        continue
                    if i == 0 and (word in test_phrases.ENGLISH_OK or word in GAME_WORDS_EN):
                        continue
                    self.fail(f"{key}: {topic}: {m.group(0)!r} in {text!r}")

    def test_lines_the_notes_flag(self):
        for key in KEYS:
            for text in texts(key):
                for flagged in test_phrases.FLAGGED:
                    self.assertNotIn(flagged.lower(), text.lower(), (key, text))

    def test_no_second_colon_after_a_lead(self):
        # These are shown after a lead that ends in a colon ("Went well:", "This deal:",
        # "To try:", "If I may:"): they carry none of their own, and the names of the
        # skills, which stand in "Next: {skill}.", none at all.
        after_a_lead = ("tutor.well.", "tutor.miss.", "tutor.rule.", "tutor.try.", "tutor.nudge.", "tutor.name.")
        for key in KEYS:
            if key.startswith(after_a_lead):
                for text in texts(key):
                    self.assertNotIn(":", text, (key, text))

    def test_the_english_that_is_lowered_after_a_colon_can_bear_it(self):
        # The table lowers the first letter of an English body shown after a lead that
        # ends in a colon or after "If I may:" (web3d/src/review.js `after`). That is
        # safe only for a body that does not begin with "I", a Roman numeral, a card
        # or a proper name of the game; a skill's name is lower case already.
        lowered = ("tutor.well.", "tutor.miss.", "tutor.rule.", "tutor.try.", "tutor.nudge.", "tutor.name.")
        keeps_capital = re.compile(r"^(I\b|[IVX]+\b|Pagat|Mond|Sk\u00fcs|Trull|Troule|Tarock|Kibitzer|Point|King|Queen|Jack|Cavall)")
        for key in KEYS:
            if key.startswith(lowered):
                en = STRINGS[key]["en"]
                self.assertIsNone(keeps_capital.match(en), (key, en))
        # and the skills' names begin in lower case, as "Next: {skill}." and the praise need
        for key in KEYS:
            if key.startswith("tutor.name."):
                self.assertTrue(STRINGS[key]["en"][0].islower(), key)

    def test_no_line_judges_the_person(self):
        bad = re.compile(r"\b(stupid|foolish|blunder|mistake|careless|lazy|bad player|dumm|blöd|Fehler|Anfänger|schlecht)", re.I)
        for key in KEYS:
            for text in texts(key):
                self.assertIsNone(bad.search(text), (key, text))


if __name__ == "__main__":
    unittest.main()
