"""Hard rules for the phrase bank (tools/phrases.py).

    python3 -m unittest tools.test_phrases      # or: python3 -m pytest tools

Each rule below is enforced on every line, English and German original.
"""

import re
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import phrases  # noqa: E402

GROUPS = phrases.phrase_groups()
LINES = [(gid, w) for gid, _, ways in GROUPS for w in ways]


def texts(w):
    """The English line and the German line."""
    return [w[0], w[2]]


# Words that mean something else in the other language: "bald" is "soon" in
# German; "Kind", "alt", "jung" are checked in the German original only.
GERMAN_OK = {"bald", "hell", "man"}
ENGLISH_OK = {"kind", "alt", "alte", "alten", "jung", "kinder", "herr", "herren", "frau", "weib"}


def kind_texts(w):
    """The same, with the game's own word for the dummy piles ("straw men",
    "Strohmänner") taken out: it is not about people."""
    return [re.sub(r"straw ?men|Strohm\w+", "", t, flags=re.I) for t in texts(w)]


def words(*ws):
    return re.compile(r"\b(?:" + "|".join(ws) + r")\b", re.I)


# The kindness rule (CLAUDE.md): luck, speed, the score and the run of play are
# fair game; looks, intelligence, age, gender, class, religion, nationality and
# ethnicity never are.
BANNED = {
    "looks": words("hässlich", "dick", "kahl", "Gesicht", "Visage", "Fratze", "schiach", "ugly", "fat", "bald", "handsome", "pretty", "face", "faces", "mug", "looks",
                   "Pappen", "kahl", "kahlen", "Glatze"),
    "intelligence": words("stupid", "dumb", "idiot", "fool", "fools", "foolish", "silly", "clever",
                          "smart", "brain", "brains", "dense", "dumm", "strohdumm", "blöd", "Trottel",
                          "Depp", "Narr", "Narren", "gescheit", "Idiot", "Esel", "Tepp", "Dummkopf", "Blödsinn", "Hornochse", "bungler", "Patzer", "Mordspatzer",
                          "Stümper", "Tschappel", "poodle"),
    "age": words("old", "older", "young", "younger", "elderly", "aged", "ancient", "senile", "grandmother",
                 "grandma", "granny", "grandfather", "boy", "boys", "kid", "kids", "child", "children",
                 "alt", "alte", "alten", "jung", "Kind", "Kinder", "Großmutter", "Greis", "Jungfer", "Opa", "Oma", "Alter", "älter", "jünger", "Bub", "Buben", "Bua", "Mäderl", "Madl"),
    "gender": words("woman", "women", "wife", "husband", "lady", "ladies", "gentleman", "gentlemen",
                    "madam", "mister", "girl", "girls", "male", "female", "man", "men", "he", "she",
                    "Frau", "Frauen", "Weib", "Weiber", "Gemahlin", "Gatte", "Damen", "Herr", "Herren",
                    "Fräulein", "Madame", "Monsieur", "Mr", "Mrs", "Miss", "sir", "Mann", "Männer", "Weibsbild", "Dame"),
    "class": words("peasant", "servant", "coachman", "coachmen", "waiter", "farmer", "commoner", "common",
                   "vulgar", "rich", "poor", "pauper", "beggar", "noble", "nobleman", "aristocrat",
                   "gutter", "lowly", "rabble", "Bauer", "Bauern", "Kutscher", "Gemeiner", "Pöbel",
                   "Knecht", "Magd", "Dienstmädchen", "Fiaker", "Bettler", "Arm", "arm", "reich", "gemein", "Proletarier", "Brodsitzer", "Brotsitzer", "Knotzer", "Kutschergspül"),
    "religion": words("god", "gods", "lord", "devil", "satan", "saint", "church", "priest", "pastor",
                      "pope", "christ", "jesus", "heaven", "heavens", "hell", "damn", "damned", "bless",
                      "blessed", "holy", "pray", "prayer", "sin", "sinner", "amen", "rabbi", "Sinai",
                      "Moses", "Gott", "Herrgott", "Teufel", "Kirche", "Pfarrer", "heilige", "Heilige",
                      "Erlöser", "Himmel", "Hölle", "Sünde", "Segen", "Nepomuk", "Jesus", "Maria", "Pfaffe", "verdammt", "Heiland", "Beten"),
    "nationality / ethnicity": words("german", "french", "italian", "austrian", "hungarian", "czech",
                                     "polish", "slovene", "russian", "turkish", "greek", "prussian",
                                     "bohemian", "portuguese", "swabian", "foreigner", "foreigners",
                                     "jew", "jews", "jewish", "gypsy", "gypsies", "semite", "race",
                                     "racial", "ethnic", "Jude", "Juden", "Jud", "Zigeuner", "Böhm",
                                     "Ausländer", "Rasse", "deutsch", "französisch", "ungarisch", "tschechisch", "polnisch", "slowenisch", "russisch", "türkisch", "griechisch", "preußisch", "Preuße", "Böhme", "Tschusch", "Ungar", "Serbe", "Itaker", "Walsche"),
}

# Lines and words the notes flag as not for game use ([REVIEW ...] and the
# kindness flags in research/05, 05b, 05c, 05d, 04, 09).
FLAGGED = [
    "Graßl", "Hirsch", "Herr Hirsch", "du Hirsch", "Brodsitz", "Brotsitz", "Knotzer", "Kutscher",
    "Kutschergspül", "auf'n Juden", "der Jud", "Veitel", "Rosenblatt", "Ihr Blatt", "krumm, taub",
    "taub", "stumm", "lahm", "Zeig ihn her", "Hos'n", "Unglück in der Liebe", "Schwein", "dümmste",
    "Tschappel", "strohdumm", "Pappen", "Mordspatzer", "Giftmischer", "alte Henne", "old hen",
    "Medizinmann", "Nepomuk", "Erlöser", "Krucifix", "kleine Kinder sterben", "Messner", "Zigeuner",
    "kahlen Köpfe", "Fünfer", "Großmutter", "Bruder Skis", "Monsieur Skis", "Schmied von Penzing",
    "Wie haaßt Herz", "Herrgott", "Gstüsmarterer", "Berg Sinai",
]

# Terms the opponent never uses: the bird names and the bonuses named after
# them (not in the canonical rules, research/14 rule 22), Rostopschin, and
# modern terms (Nullrunde 2007). The period runs to 1938.
LATE = words("Uhu", "Kakadu", "Quapil", "Marabu", "Pelikan", "Kanari", "Lämmergeier", "Wildsau", "Dodo",
             "Vogerl", "Vögel", "birds", "Rostopschin", "Topschin",
             "Nullrunde", "Bummerl", "Sküsfang", "Königstrull")

# A line may not claim to know a hidden card: the other player's hand cards or a
# face-down packet card (docs/RULES.md 13.2, 13.3), and a speaker may not give
# away its own.
HIDDEN = [
    re.compile(p, re.I) for p in (
        r"\byou(?:'ve| have| hold| got|'re holding)\b", r"\byour (?:hand|cards|straw|packet|packets|pile|hidden)\b",
        r"\bmust have\b", r"\bI know\b", r"\bI can see\b", r"\bI saw\b", r"\bI bet\b", r"\bI hold\b",
        r"\bI(?:'ve| have| had| still have| still hold) (?:the|a|an|no|your|got)\b", r"\bstill with me\b",
        r"\bhidden\b", r"\bface-?down\b", r"\bunseen\b",
        r"\bSie (?:haben|halten|müssen)\b", r"\bIhre?[nrms]? (?:Blatt|Karten|Hand|Strohm\w+|Päckchen)\b", r"\bich (?:sehe|wette|kenne)\b",
        r"\b(?:du|ihr) (?:hast|habt)\b", r"\bverdeckt\b", r"\bich (?:habe|hab'|halte)\b(?!.*\bgemacht\b)", r"\bich weiß\b",
    )
]

# Frequent moments: five or more wordings each.
FREQUENT = [
    "greet", "touren-3", "touren-6", "deal-mine", "deal-yours", "bid-take-mine", "bid-pass-mine",
    "bid-take-yours", "bid-pass-yours", "pull-king-mine", "pull-king-yours", "pull-tarock-mine",
    "pull-tarock-yours", "run-mine", "run-yours", "nothing-mine", "nothing-yours", "slow",
    "trick-good-mine", "trick-good-yours", "trick-bad-mine", "trick-bad-yours", "mond-caught-mine",
    "mond-caught-yours", "ultimo-won-mine", "ultimo-won-yours", "ultimo-lost-mine", "ultimo-lost-yours",
    "trull-mine", "trull-yours", "kings-mine", "kings-yours", "count-close", "result-won-mine",
    "result-lost-mine", "result-won-yours", "result-lost-yours", "draw", "tour-end", "tour-last",
    "extend-ask", "farewell", "blunder-own", "luck-yours", "luck-mine", "luck-bad-mine", "kibitz",
]

# Every moment of docs/RULES.md section 16 the opponent can talk about.
REQUIRED = set(FREQUENT) | {
    "cut", "void-deal", "doubling-applied", "both-pass", "thrown-in", "reveal-begin", "pull-sku",
    "pull-mond", "pull-pagat",
    "sku-played", "honour-forfeited", "valat-mine", "valat-yours", "count-start", "doubling-lapsed",
    "extend-yes", "extend-no", "settle-opp-pays", "settle-player-pays", "settle-even", "slip-yours",
    "kibitz-mute",
}


class SourceKeys(unittest.TestCase):
    def test_every_phrase_has_a_valid_source_key(self):
        for gid, w in LINES:
            self.assertGreaterEqual(len(w), 2, (gid, w))
            self.assertIn(w[1], phrases.SOURCES, (gid, w))

    def test_every_source_resolves_to_a_citation(self):
        for key, (year, ids, text, _) in phrases.SOURCES.items():
            self.assertTrue(text, key)
            if key != "T":
                self.assertRegex(ids, r"\[(?:\d\d[a-d]?|[A-Z]+)[- ]", key)
                self.assertTrue(re.search(r"\[\d", ids), key)

    def test_only_the_table_has_no_note_id(self):
        for gid, w in LINES:
            if w[1] != "T":
                self.assertTrue(phrases.SOURCES[w[1]][1], (gid, w))

    def test_every_source_is_used(self):
        used = {w[1] for _, w in LINES}
        self.assertEqual(set(phrases.SOURCES) - used, set())

    def test_every_wording_has_english_and_german(self):
        for gid, w in LINES:
            self.assertEqual(len(w) >= 3 and bool(w[0].strip()) and bool(w[2].strip()), True, (gid, w))
        import json
        data = json.loads(phrases.words_json())
        self.assertEqual(len(data["texts"]), len(LINES))
        for key, v in data["texts"].items():
            self.assertTrue(v["en"].strip() and v["de"].strip(), key)


class Period(unittest.TestCase):
    def test_no_source_after_1938(self):
        for key, (year, *_rest) in phrases.SOURCES.items():
            if key != "T":
                self.assertLessEqual(year, 1938, key)

    def test_no_twentieth_century_terms(self):
        for gid, w in LINES:
            for t in texts(w):
                self.assertIsNone(LATE.search(t), (gid, t))

    def test_bird_names_in_particular(self):
        for gid, w in LINES:
            for t in texts(w):
                for bird in ("Uhu", "Kakadu", "Quapil"):
                    self.assertNotIn(bird.lower(), t.lower(), (gid, t))

    def test_mondfang_only_where_a_note_attests_it(self):
        found = 0
        for gid, w in LINES:
            if any("mondfang" in t.lower() for t in texts(w)):
                found += 1
                self.assertIn(w[1], phrases.MONDFANG_SOURCES, (gid, w))
        self.assertGreater(found, 0)
        self.assertEqual(phrases.MONDFANG_SOURCES, {"Wn", "NF"})


class Kindness(unittest.TestCase):
    def test_banned_words_and_topics(self):
        for gid, w in LINES:
            for i, t in enumerate(kind_texts(w)):
                for topic, rx in BANNED.items():
                    m = rx.search(t)
                    if m and i == 1 and m.group(0).lower() in GERMAN_OK:
                        continue
                    if m and i == 0 and m.group(0).lower() in ENGLISH_OK:
                        continue
                    self.assertIsNone(m, f"{gid}: {topic}: {m and m.group(0)!r} in {t!r}")

    def test_lines_the_notes_flag(self):
        for gid, w in LINES:
            for t in texts(w):
                for flagged in FLAGGED:
                    self.assertNotIn(flagged.lower(), t.lower(), (gid, t))


class HiddenCards(unittest.TestCase):
    def test_no_line_claims_a_hidden_card(self):
        for gid, w in LINES:
            for t in texts(w):
                for rx in HIDDEN:
                    m = rx.search(t)
                    self.assertIsNone(m, f"{gid}: {m and m.group(0)!r} in {t!r}")


class Shape(unittest.TestCase):
    def test_lines_are_short(self):
        for gid, w in LINES:
            self.assertLessEqual(len(w[0].split()), 10, (gid, w[0]))

    def test_every_required_moment_is_present(self):
        self.assertEqual(REQUIRED - {g for g, *_ in GROUPS}, set())

    def test_no_group_outside_the_moments(self):
        self.assertEqual({g for g, *_ in GROUPS} - REQUIRED, set())

    def test_frequent_moments_have_five_or_more(self):
        by = {g: ways for g, _, ways in GROUPS}
        for gid in FREQUENT:
            self.assertGreaterEqual(len(by[gid]), 5, gid)

    def test_every_group_has_two_ways(self):
        for gid, _, ways in GROUPS:
            self.assertGreaterEqual(len(ways), 2, gid)

    def test_no_wording_twice_in_a_group(self):
        for gid, _, ways in GROUPS:
            seen = [w[0] for w in ways]
            self.assertEqual(len(seen), len(set(seen)), gid)

    def test_group_ids_are_unique(self):
        ids = [g for g, *_ in GROUPS]
        self.assertEqual(len(ids), len(set(ids)))


class Signatures(unittest.TestCase):
    """Each text carries a `sig`, the saying it is (docs/TABLE3D.md 5.3.1): the talk
    keeps one saying from being heard again from another group."""

    def test_every_text_in_words_json_has_a_sig(self):
        import json
        data = json.loads(phrases.WORDS.read_text())
        for key, t in data["texts"].items():
            self.assertTrue(t.get("sig"), key)

    def test_a_table_text_is_its_own_sig(self):
        import json
        data = json.loads(phrases.WORDS.read_text())
        for gid, _, ways in GROUPS:
            for k, w in enumerate(ways):
                if w[1] == "T":
                    self.assertEqual(data["texts"][f"{gid}.{k}"]["sig"], f"{gid}.{k}")

    def test_the_same_saying_has_the_same_sig_whatever_the_group(self):
        self.assertEqual(phrases.sig("a", 0, ("x", "Wa", "y", "Karte 7")), phrases.sig("b", 3, ("z", "Wa", "q", "Karte 7")))
        self.assertNotEqual(phrases.sig("a", 0, ("x", "Wa", "y")), phrases.sig("a", 1, ("x", "Wa", "z")))
        # a bare lemma is not an entry: two sayings under "Spiel" differ
        self.assertNotEqual(phrases.sig("a", 0, ("x", "Wa", "y", "Spiel")), phrases.sig("a", 1, ("x", "Wa", "z", "Spiel")))

    def test_a_saying_is_in_at_most_two_groups(self):
        """The talk's record (TABLE3D 5.3) cannot make up for a saying that turns up in
        many groups: a sourced saying (not a table line) appears in two at most."""
        groups = {}
        for gid, _, ways in GROUPS:
            for k, w in enumerate(ways):
                if w[1] != "T":
                    groups.setdefault(phrases.sig(gid, k, w), set()).add(gid)
        crowded = {s: sorted(g) for s, g in groups.items() if len(g) > 2}
        self.assertEqual(crowded, {})

    def test_the_most_frequent_groups_have_eight_or_more_wordings(self):
        by = {g: ways for g, _, ways in GROUPS}
        for gid in by:
            if gid.startswith(("pull-", "nothing-", "trick-")):
                self.assertGreaterEqual(len(by[gid]), 8, gid)


class Citations(unittest.TestCase):
    """Wordings checked against the downloaded texts (docs/review/talk-and-phrases.md)."""

    def german(self, src):
        return [w[2] for _, _, ways in GROUPS for w in ways if w[1] == src]

    def test_the_volksblatt_line_keeps_the_sources_verb(self):
        # Das kleine Volksblatt, 24 May 1934: "interessiert der Mond nur, wann er in Gsellschaft ..."
        for t in self.german("Gs"):
            self.assertIn("interessiert", t)


class Generated(unittest.TestCase):
    def test_generated_files_are_current(self):
        for path, text in ((phrases.DOC, phrases.document()), (phrases.WORDS, phrases.words_json())):
            self.assertTrue(path.exists(), f"{path}: run tools/phrases.py")
            self.assertEqual(path.read_text(), text, f"{path} is stale: run tools/phrases.py")

    def test_check_mode_exits_non_zero_when_stale(self):
        import subprocess
        import tempfile
        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "tools").mkdir()
            (root / "tools" / "phrases.py").write_text((phrases.ROOT / "tools" / "phrases.py").read_text())
            run = lambda *a: subprocess.run([sys.executable, str(root / "tools" / "phrases.py"), *a],
                                            capture_output=True, text=True).returncode
            self.assertEqual(run("--check"), 1)      # nothing generated yet
            self.assertEqual(run(), 0)
            self.assertEqual(run("--check"), 0)
            (root / "docs" / "PHRASES.md").write_text("stale\n")
            self.assertEqual(run("--check"), 1)


if __name__ == "__main__":
    unittest.main()
