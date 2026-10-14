#!/usr/bin/env python3
"""The opponent's phrase bank: everything said at the table, by game moment.

    python3 tools/phrases.py           # rewrite docs/PHRASES.md and web3d/words.json
    python3 tools/phrases.py --check   # fail if either is out of date

The single source of truth. A group is a moment (docs/RULES.md section 16,
the events); each wording in it is a tuple

    (english, source_key)
    (english, source_key, german_original)
    (english, source_key, german_original, locator)

`source_key` resolves in SOURCES to a research-note id and the period wording.
"T" marks the table's own lines. `german_original` is the period wording,
verbatim from the note, and only where a source gives one. The voice is set
around 1891; no source after 1900 is used (tools/test_phrases.py enforces it,
with the kindness rule, the bird names, hidden cards and "Mondfang").

Structure after Cassino's web3d/tools/phrases.py. The page picks among a
group's wordings without repeating the last one (words.json).
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# key: (year of the source, research-note ids, citation with the period wording,
#       attests "Mondfang" as a noun)
SOURCES = {
    "T": (None, "", "the table's own: a period formula carried to a case the sources do not spell out, "
                    "or plain table talk", False),
    "Wn": (1891, "[05-S3][03-S12][13-S2][05 headline 1]",
           "Werner, Das moderne Tarokspiel, 2nd ed. (Hartleben, by 1891): Strohmann-Tarok \"in der "
           "Umgangssprache Strohmanndl\"; the Vorhand \"hat sich zu erklären, ob sie 'aufnehmen', d. h. ein "
           "Spiel riskiren wolle. Paßt die Vorhand, so hat der Kartengeber ... das Recht, die Partie "
           "aufzunehmen\"; Königrufen calls \"Mein Spiel\", \"Ich rufe\", \"weiter\", \"gut\"; \"Ich sage die "
           "Troule an\"; Troule and kings paid if \"angesagt\" at the latest as the first of them is played; "
           "a 35:35 tie is undecided and the next deal \"sammt allen Ansagen doppelt zählt\"; \"Mondfang\"; "
           "\"das Impassiren\"; \"ein Conto ... bei fortlaufender Addition\". The spoken \"Ich nehme auf!\", "
           "\"Weiter!\", \"Ich passe!\" are quoted from Mayr and Sedlaczek (2007) [05-S5]", True),
    "Ul": (1899, "[03-S14][13-S1]",
           "Ulmann, Illustrirtes Wiener Tarockbuch, 2nd ed. (1899): \"Es ist üblich, ehe die "
           "Spielgesellschaft auseinandergeht, drei oder sechs Touren anzusagen\" (p. 19); two players, "
           "\"stets selbst seines eigenen Glückes Schmied\" (p. 69); the honours \"nur die Cadres der "
           "Kartenarmee\" (p. 78); a 35:35 draw \"zählt nicht\"; Vorhand \"hat die nächste Donne\"", False),
    "Be": (1894, "[13-S3]",
           "Bermann (Vienna 1894), pp. 43-44: \"Ist beim Beginne des Spieles nicht eine bestimmte Zeitdauer "
           "desselben festgesetzt, so ist es üblich, um es abzubrechen, die Schluß-Touren anzumelden. "
           "Gewöhnlich überläßt man dies aus Courtoisie dem im stärksten Verlust befindlichen Spieler ... "
           "Nur mit allseitiger Zustimmung können die angesagten Touren erstreckt werden\"", False),
    "Hs": (1829, "[13-S7]",
           "Haas, Neuestes allgemeines Spielbuch (Vienna 1829), pp. 6-7: a sensible player will not stop "
           "abruptly but asks his fellow players \"um die Festsetzung der letzten Tour, Partie oder Robber "
           "mit bescheidener Nachgiebigkeit\"; slates and chalk for the score", False),
    "Ha": (1853, "[05d POLY-S5]",
           "Haas's Strohmandeln as printed in Alvensleben, Encyklopädie der Spiele (1853), pp. 561, 576-577: "
           "\"da hierbei weit weniger die Geschicklichkeit als der blinde Zufall wirkt\"; \"Schmiert "
           "demnach der Strohmann zufälliger Weise dem Spieler viel Könige und Figuren ... so ist das Spiel "
           "leicht zu gewinnen\"; \"Es gilt ja Einem wie dem Andern\"; \"ich skisire mich\"", False),
    "Ne1": (1835, "[05-S6][05c LIT-S1][05 REVIEW NOTE on headline 3]",
            "Nestroy, Zu ebener Erde und erster Stock (1835), Act 2, Bonz 1890 edition: Goldfuchs calls "
            "\"Tous les trois ... vier Könige ... Pagat Ultimo!\"; Johann's Lied: \"Der macht drei, vier "
            "Ultimo in einem Nu\". The wording is edition-dependent: the Projekt Gutenberg text prints "
            "\"Tous les trois! – Vier Könige! – Pagat Ultimo!\"", False),
    "Ne2": (1832, "[05-S10][05c LIT-S4][05 REVIEW NOTE on headline 3]",
            "Nestroy, Nagerl und Handschuh (1832), Bonz 1891 edition (OCR): \"Pagat Ultimo Volat! ... Ich "
            "bin matsch!\" (the modern text has only \"Ich bin matsch!\")", False),
    "Ne3": (1834, "[05-S7]",
            "Nestroy, Die Familien Zwirn, Knieriem und Leim (1834), Zwirn: \"Mein Herz hat ein' Solo Ultimo "
            "ang'sagt, und ich bin Matsch worden.\" (not compared with a modern text)", False),
    "Ne4": (1842, "[05-S8][05c LIT-S2]",
            "Nestroy, Einen Jux will er sich machen (1842), Weinberl's couplet on coffee-house onlookers: "
            "\"Schaun zwei'n in die Karten und raten dem dritten, Ob er Carreau oder Pick spiel'n soll ... "
            "da muß ich bitten\"", False),
    "Mo": (1844, "[05-S11]",
           "Moser, \"Die Tarok- und Hundert-Eins-Brüder\" (Vienna 1844): \"Wann sich drei zum Spieltisch "
           "setzen, Seins im Anfang mäuserlstill\"; \"Da fragt Aner: wer spielt aus, der And're: wer hat "
           "Karten geb'n?\"; \"Und wanns G'spiel aus is, werd'n Stich zählt und dabei erst kritisirt\"", False),
    "Ba": (1756, "[05-S12][05d POLY-S1]",
           "Bauer (Vienna and Nuremberg 1756), Ode auf das Taroc-Spiel: \"Wenn mein Pagat den letzten "
           "sticht\"; \"Und kommt der lustge Scüs darzu, So sitz ich in vergnügter Ruh\"; \"Ein wenig "
           "Spielen bleibt erlaubt, Doch nur zum Zeitvertreib\"; \"Der Pagat ist den andern Mitspielern "
           "ein Dorn in den Augen\"", False),
    "Hu": (1873, "[05-S1][05 headline 8 REVIEW]",
           "Hügel, Der Wiener Dialekt (1873): \"Max'\" as a name of the Sküs card and, separately, \"Er hängt "
           "scho' der Max'\" (his wish is near fulfilment). That the phrase is said of the card is the "
           "note's inference", False),
    "Wa": (1880, "[05b LEX-S10][05b LEX-S13][05b LEX-S17][05b LEX-S19][05b LEX-S20][05b LEX-S21]",
           "Wander, Deutsches Sprichwörter-Lexikon (1867-1880), entry given as the locator; the original "
           "wording is the German column; Austrian and Viennese items are marked in 05b", False),
    "Me": (1889, "[05-S4][05b LEX-S22 REVIEW]",
           "Meyers Konversations-Lexikon, 4th ed. (1889), Tarock: \"Ich skisiere (exküsiere) mich!\"; the "
           "6th ed. adds the etymology \"vom ital. scusa\"; cite the edition", False),
    "Fi": (1888, "[05c LIT-S15]",
           "Figaro (Vienna), 21 Jul 1888, p. 6: \"Wie schade, daß wir keinen 'Dritten' zu einem 'Tapper' "
           "haben! Können Sie nicht 'Strohmandeln'?\" — \"Zu dienen.\" — \"Das trifft sich ja herrlich!\"", False),
    "Fl": (1887, "[05c LIT-S20]",
           "Der Floh, 2 Jan 1887, p. 3 (OCR snippet): \"Seit Neujahr fangt mir fast jeden Tag der 'Sküs' den "
           "'Mond' ab.\"", False),
    "NF": (1893, "[05c LIT-S56]",
           "Neue Freie Presse, 20 Dec 1893, p. 4 (OCR snippet): \"der Mondfang ist in der hazardeusen "
           "ungarischen Tarokpartie eine Special-Katastrophe\"", True),
    "Ro": (1821, "[05d POLY-S4]",
           "Rosenau, Scüs, Mond und Pagat (Vienna 1821): \"Unnütze Mühe, der Scüs sticht ja den Mond\"; "
           "\"Ich habe den Pagat Ultimo gemacht!\"; \"wenn Sie künftig den Pagat Ultimo machen wollen, so "
           "zählens fleißig die Tarok\"", False),
    "Sc": (1899, "[05-S21][05b LEX-S30]",
           "Schubin, Vollmondzauber (1899), ch. 10: \"Und jetzt spielen wir eine Partie Tarock, um uns zu "
           "erholen\"; \"einen 'Pagat ultimo' angesagt und mit Glanz gewonnen\"", False),
    "Bl": (1775, "[05b LEX-S32]",
           "Blum, Spatziergänge II (Berlin 1775): \"Morgen bitte ich mir die Ehre von ihnen auf eine Partie "
           "Tarock aus\"", False),
    "Ve": (1892, "[05c LIT-S39][05c LIT-S38]",
           "Linzer Volksblatt, 27 Apr 1892: \"Versehen ist verspielt, lachten die anderen; spielt ruhig "
           "weiter\"; Grazer Tagblatt, 15 Nov 1896: \"Im Kartenspiel gilt als Grundsatz die Regel: "
           "'Versehen ist verspielt.'\"", False),
    "Ki": (1897, "[05c LIT-S11][05c headline 1 REVIEW RD]",
           "Neues Wiener Journal, 29 Mar 1895, p. 6: \"'Kiebitz, halt's Maul!' so lautet bekanntlich einer "
           "der Hauptparagraphen\" (the number is first found in Kikeriki, 9 Sep 1897: \"§ 13 (Kiebitz "
           "halt's Maul)\")", False),
    "Zi": (1860, "[05-S14]",
           "Zigo-Taroc (Mannheim 1860), a mock law of a Tarock club, not Viennese: \"Nichtspieler gehören "
           "unter den Tisch\"; \"Vom stolzen Skis getroffen schön\"", False),
    "Pr": (1892, "[03 §4]",
           "Prager Tagblatt, 12 Feb 1892, p. 18: \"fünfunddreißig und ein Blatt, also nur fünfunddreißig "
           "...\", the Strohmann-Tarock count in thirds (a \"Blatt\" is a third of a Point)", False),
    "Fg": (1887, "[05c LIT-S29]",
           "Figaro, 30 Apr 1887, p. 7 (OCR snippet): \"... daß ihm der Pagat-Ultimo wegg'schwommen is.\"", False),
    "Gl": (1898, "[05c LIT-S28]",
           "Glühlichter, 4 Aug 1898, p. 4 (OCR snippet): \"zu fad! Lauter Skartindeln!\"", False),
    "Cr": (1900, "[04-S26]",
           "Illustrirtes Wiener Extrablatt, 25 Aug 1900, p. 10, a Strohmanntarock case in court: \"Ich "
           "spiele es gerade so gut, nur habe ich mehr Pech.\"", False),
    "Nw": (1887, "[03 §4]",
           "Neues Wiener Tagblatt, 4 Sep 1887, p. 2: the Pagat ruined by the last straw-man card, \"und "
           "'Ultimo' samt Spiel sind verloren\"", False),
    "St": (1894, "[13-S13]",
           "Johann Strauss on the stakes (1894): \"bei zwei bis drei Kreuzer der Point\"", False),
    "Ga": (1869, "[09-S14]",
           "Die Gartenlaube (1869): \"das Fallen der Karten, das Rasseln der Sechser, die Seufzer der "
           "Verlierenden\"", False),
    "Jö": (1883, "[05c LIT-S27]",
           "Jörgel Briefe, 24 Mar 1883, p. 9 (OCR snippet): \"Ohne Tarok is kein Spiel\"", False),
    "Hn": (1727, "[05b LEX-S9]",
           "Henrici (1727), in Grimm's Wörterbuch, s.v. Stich: \"denn manchmahl bleibt ein blat in händen, "
           "das macht den allerletzten stich\"", False),
}

MONDFANG_SOURCES = {k for k, v in SOURCES.items() if v[3]}


def phrase_groups():
    """Every group, in the order the table meets them, as (id, moment, [wordings])."""
    out = []

    def add(gid, moment, *ways):
        out.append((gid, moment, list(ways)))

    # --- the sitting ---------------------------------------------------------
    add("greet", "sitting_start: the greeting",
        ("Strohmandeln, then. At your service.", "Fi", "Zu dienen."),
        ("Two at Strohmandeln: how splendid.", "Fi", "Das trifft sich ja herrlich!"),
        ("A game of Tarock, to recover.", "Sc", "Und jetzt spielen wir eine Partie Tarock, um uns zu erholen."),
        ("A little play is allowed, for pastime.", "Ba", "Ein wenig Spielen bleibt erlaubt, Doch nur zum Zeitvertreib."),
        ("No third for a Tapper? Strohmandeln it is.", "Fi", "Wie schade, daß wir keinen 'Dritten' zu einem 'Tapper' haben!"),
        ("Good evening. Straw men for two.", "T"))
    add("touren-3", "sitting_start, three Touren: the Touren announced",
        ("Three Touren, then.", "Ul", "drei oder sechs Touren anzusagen"),
        ("Let us say three Touren.", "Ul", "Touren anzusagen"),
        ("Three Touren: six deals, a short evening.", "T"),
        ("Three Touren, if that suits you.", "Hs", "mit bescheidener Nachgiebigkeit"),
        ("Drei Touren, and then the reckoning.", "Be", "die Schluß-Touren anzumelden"))
    add("touren-6", "sitting_start, six Touren: the Touren announced",
        ("Six Touren: a full evening.", "Ul", "drei oder sechs Touren anzusagen"),
        ("Let us say six Touren.", "Ul", "Touren anzusagen"),
        ("Six Touren, twelve deals. We have time.", "T"),
        ("Six Touren, if that suits you.", "Hs", "mit bescheidener Nachgiebigkeit"),
        ("Six Touren, and the slate to keep.", "Wn", "ein Conto ... bei fortlaufender Addition"))
    add("cut", "sitting_start: drawing for the first deal",
        ("Let us cut for the deal.", "T"),
        ("Who deals? Who leads? Cut and see.", "Mo", "wer spielt aus, ... wer hat Karten geb'n?"),
        ("Draw a card; the Touren begin with the dealer.", "Be", "Die Touren beginnen stets bei jenem Spieler, der zuerst die Karten ausgab"))

    # --- the deal ------------------------------------------------------------
    add("deal-mine", "deal_start: the opponent deals",
        ("My deal. Fifteen, and three fours.", "T"),
        ("Dealt right, laid neat, and the whole wrong.", "Wa", "Liegen thut's tüchtig, gegeben ist's richtig und's Ganze ist unrichtig.", "Liegen 122"),
        ("Dealt one way, played another.", "Wa", "Die Karten werden anders gegeben, anders gespielet.", "Karte 7"),
        ("Well shuffled, well dealt.", "Wa", "Hastus wol gemischt, so karts wol.", "Mischen 2"),
        ("I deal. Then the straw men.", "T"))
    add("deal-yours", "deal_start: the player deals",
        ("Your deal.", "T"),
        ("Deal on; the straw men wait.", "T"),
        ("Shuffle well, deal well.", "Wa", "Hastus wol gemischt, so karts wol.", "Mischen 2"),
        ("Dealt one way, played another. Go on.", "Wa", "Die Karten werden anders gegeben, anders gespielet.", "Karte 7"),
        ("Your deal, and a fair one.", "T"))
    add("void-deal", "void_deal: a hand with no tarock, the deal annulled",
        ("No trumps, no game.", "Jö", "Ohne Tarok is kein Spiel."),
        ("Void. Gather them up and deal again.", "T"),
        ("A void deal. It does not count.", "T"),
        ("Not a tarock to be had: dealt again.", "T"),
        ("Vergeben. The Tour waits.", "Ne1", "'s is vergeb'n"))
    add("doubling-applied", "doubling_applied: the deal counts double after a draw",
        ("Doppelt zählt's: every payment twice.", "Wn", "sammt allen Ansagen doppelt zählt"),
        ("The last draw doubles this deal.", "Wn", "doppelt zählt"),
        ("Double, mind: honours and all.", "Wn", "sammt allen Ansagen doppelt zählt"),
        ("This one counts twice.", "T"),
        ("Remember the draw: twice the Points.", "T"))

    # --- the bidding ---------------------------------------------------------
    add("bid-take-mine", "bid: the opponent takes up",
        ("Ich nehme auf!", "Wn", "Ich nehme auf!"),
        ("I take it up. Let the straw speak.", "T"),
        ("Mein Spiel.", "Wn", "Mein Spiel"),
        ("My own fortune's smith: I take it up.", "Ul", "stets selbst seines eigenen Glückes Schmied"),
        ("I'll take it up, and trust the straw.", "T"),
        ("Taken up: a game, then.", "T"))
    add("bid-pass-mine", "pass: the opponent passes",
        ("Weiter!", "Wn", "Weiter!"),
        ("Ich passe.", "Wn", "Ich passe!"),
        ("Weiter, bitte.", "Wn", "weiter"),
        ("I pass, and keep my peace.", "T"),
        ("Not this time. Weiter.", "T"))
    add("bid-take-yours", "bid: the player takes up",
        ("Aufgenommen! Gut.", "Wn", "gut"),
        ("Taken up. Let us see the straw.", "T"),
        ("Then it is a game in earnest.", "T"),
        ("Gut. Let it stand.", "Wn", "gut"),
        ("You take it up. The luck will tell.", "T"))
    add("bid-pass-yours", "pass: the player passes",
        ("Weiter, you say. Noted.", "Wn", "weiter"),
        ("A quiet start.", "Mo", "mäuserlstill"),
        ("You pass; then I speak.", "T"),
        ("Passed. We shall see.", "T"),
        ("Weiter: quite so.", "Wn", "weiter"))
    add("both-pass", "both_pass, play: both pass and a simple game is played",
        ("Both pass: a simple game, then.", "Ul", "ein Point"),
        ("Nobody takes it up. A plain game.", "Wn", "einfache Partie"),
        ("A simple game for one Point.", "Ul", "ein Point"),
        ("Both weiter. Plain play, then.", "T"),
        ("No one risks a game. On we go.", "T"))
    add("thrown-in", "thrown_in: both_pass = throw_in, nothing played",
        ("Thrown in. The deal passes.", "Ul", "Vorhand hat die nächste Donne"),
        ("Nothing played; on to the next.", "T"),
        ("Both pass, so we start afresh.", "T"))

    # --- the opening reveal --------------------------------------------------
    add("reveal-begin", "reveal_start: a player turns the straw men",
        ("Now the straw men, one by one.", "T"),
        ("Turn them over, left to right.", "T"),
        ("Let us see what the straw says.", "T"))
    add("pull-king-mine", "card_pulled: a king pulled into the opponent's hand",
        ("A king from the straw!", "T"),
        ("Ah, ein König. Welcome.", "T"),
        ("A king: worth little, then worth much.", "Wa", "Wie der König im Kartenspiel, bald gilt er nichts, bald gilt er viel.", "König 175"),
        ("The straw is generous today.", "T"),
        ("Another king for my hand.", "T"))
    add("pull-king-yours", "card_pulled: a king pulled into the player's hand",
        ("A king for you. Well turned.", "T"),
        ("Der König kommt zu Ihnen.", "T"),
        ("A king: worth little, then worth much.", "Wa", "Wie der König im Kartenspiel, bald gilt er nichts, bald gilt er viel.", "König 175"),
        ("The straw is kind to you.", "T"),
        ("There goes a king, into the hand.", "T"))
    add("pull-tarock-mine", "card_pulled: a tarock pulled into the opponent's hand",
        ("A tarock! Gladly taken.", "T"),
        ("Tarock from the straw.", "T"),
        ("Besser mitgenommen als drumgekommen.", "Wa", "Besser mitgenommen als drumgekommen.", "Mitnehmen 1"),
        ("One more trump. That helps.", "T"),
        ("Dealt one way, turned another.", "Wa", "Die Karten werden anders gegeben, anders gespielet.", "Karte 7"))
    add("pull-tarock-yours", "card_pulled: a tarock pulled into the player's hand",
        ("A tarock for you.", "T"),
        ("Trumps from the straw: well turned.", "T"),
        ("Good cards get used to you.", "Wa", "An einen geschickten Spieler gewöhnen sich gute Karten.", "Spieler 3"),
        ("One more trump to count.", "T"),
        ("The straw gives, the straw takes.", "T"))
    add("pull-sku", "card_pulled: the Sküs pulled (either side)",
        ("The Sküs! Er hängt scho' der Max'.", "Hu", "Er hängt scho' der Max'."),
        ("And the merry Sküs joins in.", "Ba", "Und kommt der lustge Scüs darzu"),
        ("The Sküs has come home.", "T"),
        ("The highest trump, out of the straw.", "T"),
        ("A Sküs: the one card nothing beats.", "T"))
    add("pull-mond", "card_pulled: the Mond (XXI) pulled",
        ("The Mond! Watch the Sküs.", "T"),
        ("Mond out of the straw.", "T"),
        ("The Mond is turned.", "T"))
    add("pull-pagat", "card_pulled: the Pagat (I) pulled",
        ("The Pagat! A little trump, a great hope.", "T"),
        ("Pagat out of the straw.", "T"),
        ("Der Pagat ist da.", "T"))
    add("run-mine", "card_pulled, long chain: a lucky run for the opponent",
        ("Three in a row! The straw is kind.", "T"),
        ("Glück ist besser als gut Spielen.", "Wa", "Glück ist besser als gut Spielen.", "Glück 360"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"),
        ("What a run from the straw!", "T"),
        ("Easy to play when luck deals.", "Wa", "Wem das Glück die Karten gibt, der hat gut spielen.", "Glück 665"))
    add("run-yours", "card_pulled, long chain: a lucky run for the player",
        ("What a run! Three, four, five.", "T"),
        ("Easy to play when luck deals.", "Wa", "Wem das Glück die Karten gibt, der hat gut spielen.", "Glück 665"),
        ("Three or four ultimos in a flash!", "Ne1", "Der macht drei, vier Ultimo in einem Nu"),
        ("The straw empties itself into the hand.", "T"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"))
    add("nothing-mine", "packet_turned, no pull: the opponent's packets give nothing",
        ("Lauter Skartindeln.", "Gl", "Lauter Skartindeln!"),
        ("Nothing from the straw. Patience.", "T"),
        ("The game hates me today.", "Wa", "Das Spiel hasst mich.", "Spiel 274, Zusätze"),
        ("Not a trump, not a king.", "T"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"))
    add("nothing-yours", "packet_turned, no pull: the player's packets give nothing",
        ("Nothing for you. The straw is stingy.", "T"),
        ("Blind chance, nothing more.", "Ha", "blinder Zufall"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("The straw gave you low cards only.", "T"),
        ("Es gilt ja Einem wie dem Andern.", "Ha", "Es gilt ja Einem wie dem Andern"))

    # --- the play ------------------------------------------------------------
    add("slow", "the player is slow to play",
        ("A card or a log of wood!", "Wa", "'Ne Kart' oder 'ne Klob Holz.", "Karte 77"),
        ("Out with it, then!", "Wa", "Heraus mit der Ziege auf den Teichdamm.", "Ziege 107"),
        ("Who leads? I have forgotten.", "Mo", "wer spielt aus, ... wer hat Karten geb'n?"),
        ("Mouse-quiet: the table waits.", "Mo", "mäuserlstill"),
        ("Take your time. The evening is long.", "T"),
        ("Your lead, whenever you are ready.", "T"),
        ("No hurry. The cards are patient.", "T"),
        ("Thinking it over?", "T"),
        ("Not a trump will walk by itself.", "T"))
    add("trick-good-mine", "trick_won: a good trick for the opponent",
        ("Gestochen! The buck is stuck.", "Wa", "Gestochen der Bock, warum geht er ins Kraut.", "Bock 142"),
        ("Small trump, big catch.", "Wa", "Ins gemein spielt man jetzo Trumpffspiel, die geringern stechen die höhern.", "Spielen 22"),
        ("Trump against trump, and mine.", "Wa", "Trumpf wider Trumpf.", "Trumpf 4"),
        ("That one is mine, I think.", "T"),
        ("Taken. Thank you.", "T"))
    add("trick-good-yours", "trick_won: a good trick for the player",
        ("Well taken.", "T"),
        ("The little ones chase the big.", "Wa", "Die Kleinen jagen die Grossen.", "Kleine 10"),
        ("Your trick, and fairly won.", "T"),
        ("Good cards like you, it seems.", "Wa", "An einen geschickten Spieler gewöhnen sich gute Karten.", "Spieler 3"),
        ("Trump against trump, and yours.", "Wa", "Trumpf wider Trumpf.", "Trumpf 4"))
    add("trick-bad-mine", "trick_won: a bad trick for the opponent, points lost",
        ("Das Blatt hat sich gewandt.", "Wa", "Das Blatt hat sich gewandt.", "Blatt 29"),
        ("Not won till the cards are out.", "Wa", "So lange die Karten auf dem Tisch liegen, ist das Spiel nicht gewonnen.", "Karte 89"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("There goes a trick. It happens.", "T"),
        ("The better of that one is yours.", "T"))
    add("trick-bad-yours", "trick_won: a bad trick for the player",
        ("The next one may be yours.", "T"),
        ("The leaf can turn yet.", "Wa", "Die Karte hat sich gewendet.", "Karte 44"),
        ("Nothing is lost while cards lie out.", "Wa", "So lange die Karten auf dem Tisch liegen, ist das Spiel nicht gewonnen.", "Karte 89"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("Dealt one way, played another.", "Wa", "Die Karten werden anders gegeben, anders gespielet.", "Karte 7"))
    add("sku-played", "card_played: the Sküs played (either side)",
        ("Ich skisiere mich!", "Me", "Ich skisiere (exküsiere) mich!"),
        ("Pardon me: the Sküs.", "Ha", "ich skisire mich"),
        ("And the merry Sküs joins in.", "Ba", "Und kommt der lustge Scüs darzu"),
        ("The Sküs: the highest of the trumps.", "T"),
        ("Sküs it is. Nothing beats it.", "T"))
    add("mond-caught-mine", "trick_won: the opponent's Sküs catches the player's Mond",
        ("Mondfang! The Sküs has your Mond.", "Wn", "Mondfang"),
        ("Unnütze Mühe: the Sküs takes the Mond.", "Ro", "Unnütze Mühe, der Scüs sticht ja den Mond."),
        ("Struck by the proud Sküs.", "Zi", "Vom stolzen Skis getroffen"),
        ("A Mondfang: a special catastrophe.", "NF", "Mondfang ... eine Special-Katastrophe"),
        ("The Sküs caught the Mond. Sorry.", "T"))
    add("mond-caught-yours", "trick_won: the player's Sküs catches the opponent's Mond",
        ("Caught again: the Sküs hunts my Mond.", "Fl", "fangt mir ... der 'Sküs' den 'Mond' ab"),
        ("Mondfang! It had to happen.", "Wn", "Mondfang"),
        ("Unnütze Mühe: the Sküs takes the Mond.", "Ro", "Unnütze Mühe, der Scüs sticht ja den Mond."),
        ("Well caught: the Sküs wins the Mond.", "T"),
        ("A Mondfang: a special catastrophe.", "NF", "Mondfang ... eine Special-Katastrophe"))

    # --- the Pagat -----------------------------------------------------------
    add("ultimo-won-mine", "pagat_ultimo won by the opponent",
        ("My Pagat takes the last trick!", "Ba", "Wenn mein Pagat den letzten sticht"),
        ("Pagat ultimo: made.", "Ro", "Ich habe den Pagat Ultimo gemacht!"),
        ("Mit Glanz gewonnen.", "Sc", "mit Glanz gewonnen"),
        ("Keep a good card for the last.", "Wa", "Behalt ein gute Kart auf die letzte Less.", "Karte 2"),
        ("One card kept back makes the last trick.", "Hn", "manchmahl bleibt ein blat in händen, das macht den allerletzten stich"))
    add("ultimo-won-yours", "pagat_ultimo won by the player",
        ("Pagat ultimo for you. Mit Glanz.", "Sc", "mit Glanz gewonnen"),
        ("The last card, and well kept.", "Wa", "Behalt ein gute Kart auf die letzte Less.", "Karte 2"),
        ("Your Pagat takes the last trick.", "Ba", "Wenn mein Pagat den letzten sticht"),
        ("One card kept back makes the last trick.", "Hn", "manchmahl bleibt ein blat in händen, das macht den allerletzten stich"),
        ("The Pagat, a thorn in my eye!", "Ba", "ein Dorn in den Augen"))
    add("ultimo-lost-mine", "pagat_ultimo lost by the opponent",
        ("Lost on the very last card.", "Wa", "Man kann ein gutes Spiel noch mit der letzten Karte verlieren.", "Spiel 104"),
        ("My ultimo has floated away.", "Fg", "daß ihm der Pagat-Ultimo wegg'schwommen is"),
        ("Ultimo and game both gone.", "Nw", "'Ultimo' samt Spiel sind verloren"),
        ("Caught. I should have counted.", "T"),
        ("The Pagat falls; so it goes.", "T"))
    add("ultimo-lost-yours", "pagat_ultimo lost by the player",
        ("Lost on the very last card.", "Wa", "Man kann ein gutes Spiel noch mit der letzten Karte verlieren.", "Spiel 104"),
        ("Count the tarocks next time.", "Ro", "zählens fleißig die Tarok"),
        ("Ultimo and game both gone.", "Nw", "'Ultimo' samt Spiel sind verloren"),
        ("Caught! The Pagat will try again.", "T"),
        ("A pity: the Pagat was so near.", "T"))

    # --- announcements and honours ------------------------------------------
    add("trull-mine", "announcement trull by the opponent",
        ("Ich sage die Troule an.", "Wn", "Ich sage die Troule an"),
        ("Tous les trois!", "Ne1", "Tous les trois"),
        ("Die Troule: Sküs, Mond, Pagat.", "Wn", "Die Troule"),
        ("The Troule, announced aloud, one Point.", "T"),
        ("Tous les trois, if you please.", "Ne1", "Tous les trois"))
    add("trull-yours", "announcement trull by the player",
        ("Tous les trois! One Point to you.", "Ne1", "Tous les trois"),
        ("The Troule is announced. Noted.", "Wn", "Die Troule"),
        ("Die Troule: I shall mind the Sküs.", "T"),
        ("Tous les trois: the evening pays.", "T"),
        ("A Troule, and said in time.", "Wn", "spätestens in dem Augenblicke ... angesagt"))
    add("kings-mine", "announcement kings by the opponent",
        ("Vier Könige!", "Ne1", "vier Könige"),
        ("Four kings, announced.", "T"),
        ("All four kings: the straw was kind.", "T"),
        ("The four kings, one Point.", "T"),
        ("Four kings, and said in time.", "Wn", "spätestens in dem Augenblicke ... angesagt"))
    add("kings-yours", "announcement kings by the player",
        ("Four kings! One Point to you.", "Ne1", "vier Könige"),
        ("Vier Könige: duly noted.", "Ne1", "vier Könige"),
        ("The four kings are announced.", "T"),
        ("Honours are only the cadres of the army.", "Ul", "nur die Cadres der Kartenarmee"),
        ("Four kings, and said in time.", "Wn", "spätestens in dem Augenblicke ... angesagt"))
    add("honour-forfeited", "honour_forfeited: announced too late",
        ("Not said in time: no honour.", "Wn", "spätestens in dem Augenblicke ... angesagt"),
        ("Too late to announce; it is forfeit.", "T"),
        ("An honour must be said before it is played.", "Wn", "angesagt"))
    add("valat-mine", "valat by the opponent (every trick)",
        ("Matsch! Every last trick.", "Ne2", "matsch"),
        ("Valat! The whole table is mine.", "Ne2", "Valat"),
        ("Pagat, ultimo, valat: all of it.", "Ne2", "Pagat Ultimo Volat"),
        ("Not a trick for you. Quite matsch.", "Ne3", "Matsch"))
    add("valat-yours", "valat by the player (every trick)",
        ("Ich bin matsch!", "Ne2", "Ich bin matsch!"),
        ("Not one trick for me. Matsch!", "Ne3", "ich bin Matsch worden"),
        ("Pagat, ultimo, valat — and I am matsch.", "Ne2", "Pagat Ultimo Volat! ... Ich bin matsch!"),
        ("Every trick yours. Well played.", "T"))

    # --- the count and the result -------------------------------------------
    add("count-start", "count: the tricks are counted",
        ("Now we count.", "T"),
        ("Counted, and then argued about.", "Mo", "werd'n Stich zählt und dabei erst kritisirt"),
        ("To the thirds, if you please.", "Pr", "ein Blatt"),
        ("The count, honestly kept.", "T"))
    add("count-close", "count: a very close count, 35 and two thirds",
        ("Thirty-five and two Blatt: just enough.", "Pr", "fünfunddreißig und ... Blatt"),
        ("A hair's breadth: thirty-five and two thirds.", "Pr", "fünfunddreißig und ... Blatt"),
        ("Not won while a card lies out.", "Wa", "Das Spiel ist noch nicht gewonnen, die Kart liegt noch vffm Tisch.", "Spiel 42"),
        ("Es gilt ja Einem wie dem Andern.", "Ha", "Es gilt ja Einem wie dem Andern"),
        ("By a single third of a Point!", "T"))
    add("result-won-mine", "game_scored: the opponent wins the deal",
        ("Gewonnen, and fairly, I hope.", "T"),
        ("Luck beats good play, today.", "Wa", "Glück ist besser als gut Spielen.", "Glück 360"),
        ("Taken up and won. The straw obliged.", "T"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"),
        ("Mit Glanz gewonnen.", "Sc", "mit Glanz gewonnen"))
    add("result-lost-mine", "game_scored: the opponent loses the deal",
        ("I play just as well, only with worse luck.", "Cr", "Ich spiele es gerade so gut, nur habe ich mehr Pech."),
        ("The game hates me today.", "Wa", "Das Spiel hasst mich.", "Spiel 274, Zusätze"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("Yours, then. Deal on.", "T"),
        ("The leaf will turn.", "Wa", "Das Blatt hat sich gewandt.", "Blatt 29"))
    add("result-won-yours", "game_scored: the player wins the deal",
        ("Yours. Well played, and well turned.", "T"),
        ("Easy to play when luck deals.", "Wa", "Wem das Glück die Karten gibt, der hat gut spielen.", "Glück 665"),
        ("Good cards get used to you.", "Wa", "An einen geschickten Spieler gewöhnen sich gute Karten.", "Spieler 3"),
        ("Mit Glanz gewonnen. My compliments.", "Sc", "mit Glanz gewonnen"),
        ("A deal for you. The evening is long.", "T"))
    add("result-lost-yours", "game_scored: the player loses the deal",
        ("Mine this time. Next one may be yours.", "T"),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"),
        ("You play as well; the luck was mine.", "Cr", "Ich spiele es gerade so gut, nur habe ich mehr Pech."),
        ("The leaf can turn yet.", "Wa", "Die Karte hat sich gewendet.", "Karte 44"))
    add("draw", "game_scored, draw: 35 and 35, nothing decided",
        ("Unentschieden. The next deal counts double.", "Wn", "sammt allen Ansagen doppelt zählt"),
        ("A dead heat. Doubled next time.", "Wn", "sammt allen Ansagen doppelt zählt"),
        ("Thirty-five and thirty-five: it does not count.", "Ul", "zählt nicht"),
        ("No one's game. The next one counts twice.", "T"),
        ("Neither wins. The doubling waits.", "T"))
    add("doubling-lapsed", "doubling_lapsed: a pending doubling ends with the sitting",
        ("The doubling lapses with the evening.", "T"),
        ("No more deals, so no doubling.", "T"))

    # --- the Tour and the sitting's end -------------------------------------
    add("tour-end", "tour_end: two deals are done",
        ("A Tour done. The slate, if you please.", "T"),
        ("Let us write it up.", "Wn", "ein Conto ... fortlaufende Addition"),
        ("One Tour round the table.", "Ul", "Touren"),
        ("Count the chalk.", "Hs", "Kreide"),
        ("Tour over; the account runs on.", "Wn", "bei fortlaufender Addition"))
    add("tour-last", "deal_start of the last Tour",
        ("The last Tour, then.", "Be", "die Schluß-Touren"),
        ("Die Schluß-Touren: play them well.", "Be", "die Schluß-Touren anzumelden"),
        ("One more Tour, and then the reckoning.", "Ul", "Touren"),
        ("The Touren run out. Deal with care.", "T"),
        ("Last Tour: we are nearly home.", "T"))
    add("extend-ask", "extension_offered: one more Tour?",
        ("Noch eine Tour?", "Be", "Nur mit allseitiger Zustimmung können die angesagten Touren erstreckt werden"),
        ("Another Tour, if we both agree.", "Be", "Nur mit allseitiger Zustimmung"),
        ("Shall we add a Tour?", "T"),
        ("Only if we both agree: one more?", "Be", "Nur mit allseitiger Zustimmung"),
        ("Noch eine Tour, what do you say?", "Be", "erstreckt"))
    add("extend-yes", "extension_decided, accepted",
        ("Noch eine Tour! Gladly.", "Be", "erstreckt"),
        ("Agreed: the Touren are extended.", "Be", "allseitiger Zustimmung"),
        ("One more, then. Deal.", "T"),
        ("Good. The evening is not over.", "T"))
    add("extend-no", "extension_decided, declined",
        ("Then we stop here, as friends.", "Be", "aus Courtoisie"),
        ("Enough for tonight. The reckoning.", "T"),
        ("No more Touren. To the slate.", "T"),
        ("Schluß für heute.", "Be", "Schluß-Touren"))
    add("settle-opp-pays", "sitting_end: the opponent pays",
        ("Nothing for it: I owe you.", "T"),
        ("Two kreuzer a Point: cheap at the price.", "St", "bei zwei bis drei Kreuzer der Point"),
        ("The rattle of the coins, and my sighs.", "Ga", "das Rasseln der Sechser, die Seufzer der Verlierenden"),
        ("Paid is paid. Well played.", "T"))
    add("settle-player-pays", "sitting_end: the player pays",
        ("Two kreuzer a Point; I'll take it gladly.", "St", "bei zwei bis drei Kreuzer der Point"),
        ("The rattle of the coins: mine, this time.", "Ga", "das Rasseln der Sechser"),
        ("I stop while ahead, like Charlemagne.", "Wa", "Er spielt wie Karl der Grosse (Er hört auf, wenn er gewonnen hat).", "Spielen 109"),
        ("The slate is settled. My thanks.", "T"))
    add("settle-even", "sitting_end: quits, nobody owes",
        ("Quits. Nobody owes anything.", "T"),
        ("An even evening. The luck was fair.", "T"),
        ("Not a kreuzer either way.", "T"))
    add("farewell", "after sitting_end: the farewell",
        ("Tomorrow, the honour of another game?", "Bl", "Morgen bitte ich mir die Ehre von ihnen auf eine Partie Tarock aus."),
        ("Thank you for the game.", "T"),
        ("A good evening. Until the next.", "T"),
        ("Gut' Nacht, and good cards.", "T"),
        ("A little play is allowed, and this was a little.", "Ba", "Ein wenig Spielen bleibt erlaubt"),
        ("Schluß for tonight. Do come again.", "T"))

    # --- the opponent's own slips, luck, the kibitzer -----------------------
    add("blunder-own", "the opponent's own blunder",
        ("Versehen ist verspielt, even for me.", "Ve", "Versehen ist verspielt."),
        ("A slip; I play on.", "Ve", "spielt ruhig weiter"),
        ("I should have counted the tarocks.", "Ro", "zählens fleißig die Tarok"),
        ("That one was a slip.", "T"),
        ("I wasn't looking. My own fault.", "T"),
        ("Unnütze Mühe, that play.", "Ro", "Unnütze Mühe"))
    add("slip-yours", "the player's slip (a card played in error)",
        ("Versehen ist verspielt. Play on.", "Ve", "Versehen ist verspielt, ... spielt ruhig weiter"),
        ("A slip is a play, that is all.", "Ve", "Versehen ist verspielt."),
        ("It happens. Go on.", "T"))
    add("luck-yours", "luck talk: the player is lucky",
        ("Easy to play when luck deals.", "Wa", "Wem das Glück die Karten gibt, der hat gut spielen.", "Glück 665"),
        ("Luck beats good play.", "Wa", "Glück ist besser als gut Spielen.", "Glück 360"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"),
        ("The luck is on your side tonight.", "T"),
        ("Good cards get used to you.", "Wa", "An einen geschickten Spieler gewöhnen sich gute Karten.", "Spieler 3"))
    add("luck-mine", "luck talk: the opponent is lucky",
        ("Pure luck, I fear.", "T"),
        ("Blind chance, nothing more.", "Ha", "blinder Zufall"),
        ("Luck beats good play, today.", "Wa", "Glück ist besser als gut Spielen.", "Glück 360"),
        ("Lucky today; tomorrow, who knows?", "T"),
        ("Luck at cards is April weather.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"))
    add("luck-bad-mine", "luck talk: the opponent's bad luck",
        ("The game hates me today.", "Wa", "Das Spiel hasst mich.", "Spiel 274, Zusätze"),
        ("I play as well, with worse luck.", "Cr", "Ich spiele es gerade so gut, nur habe ich mehr Pech."),
        ("A good hand comes back.", "Wa", "Ein gut Spiel kommt wieder.", "Spiel"),
        ("The leaf has turned against me.", "Wa", "Das Blatt hat sich gewandt.", "Blatt 29"),
        ("Luck is April weather. I wait.", "Wa", "Glück im Spiel ist Wetter im April.", "Glück"))
    add("kibitz", "the kibitzer (the tutor, § 13) speaks or is asked",
        ("Paragraph thirteen: kibitzer, be quiet!", "Ki", "§ 13 (Kiebitz halt's Maul)"),
        ("Careful: a kibitz is at the table.", "Wa", "Es ist ein Kibitz.", "Kibitz"),
        ("Onlookers belong under the table.", "Zi", "Nichtspieler gehören unter den Tisch"),
        ("This game loves no audience.", "Wa", "Spiel liebt keine Zuschauer.", "Spiel"),
        ("Advice from the gallery? I beg your pardon.", "Ne4", "da muß ich bitten"),
        ("Let the cards be seen and you lose.", "Wa", "Wer sich in die Karten sehen lässt, verliert das Spiel.", "Karte 36"))
    add("kibitz-mute", "the kibitzer is muted",
        ("Paragraph thirteen it is.", "Ki", "§ 13"),
        ("Silence, then. The table is ours.", "T"),
        ("Kiebitz, halt's Maul: quite so.", "Ki", "Kiebitz, halt's Maul!"))
    return out


def files():
    """Every wording: key -> English."""
    return {f"{gid}.{k}": w[0] for gid, _, ways in phrase_groups() for k, w in enumerate(ways)}


def originals():
    """Every German original: key -> its words."""
    return {f"{gid}.{k}": w[2] for gid, _, ways in phrase_groups() for k, w in enumerate(ways) if len(w) > 2}


def groups():
    """Every group the page can ask for: id -> the wordings it picks among."""
    return {gid: [f"{gid}.{k}" for k in range(len(ways))] for gid, _, ways in phrase_groups()}


def words_json():
    """web3d/words.json: the build embeds it."""
    return json.dumps({"groups": groups(), "texts": files(), "de": originals()},
                      ensure_ascii=False, separators=(",", ":"), sort_keys=True) + "\n"


def document():
    """docs/PHRASES.md: every group, its lines and their sources."""
    lines = [
        "# What the table says",
        "",
        "The opponent's phrase bank, by game moment (events: `docs/RULES.md` section 16). The voice is",
        "set around 1891; no source after 1900 is used. German is the period wording where a source",
        "gives one. \"T\" marks the table's own lines. Generated from `tools/phrases.py`; edit the bank,",
        "not this file.",
        "",
        "Sources:",
        "",
    ]
    for key, (year, ids, text, _) in SOURCES.items():
        lines.append(f"- **{key}**{' ' + ids if ids else ''}: {text}")
    for gid, moment, ways in phrase_groups():
        lines += ["", f"### `{gid}`", "", f"{moment}", "", "| Said | German | Source |", "|---|---|---|"]
        for w in ways:
            text, src = w[0], w[1]
            de = w[2] if len(w) > 2 else ""
            loc = f", {w[3]}" if len(w) > 3 else ""
            ids = SOURCES[src][1]
            cite = f"{src}{loc} {ids}".strip()
            lines.append(f"| {text} | {de} | {cite} |")
    lines.append("")
    return "\n".join(lines)


DOC = ROOT / "docs" / "PHRASES.md"
WORDS = ROOT / "web3d" / "words.json"


def main():
    for gid, _, ways in phrase_groups():
        assert len(ways) >= 2, f"{gid} is said only one way"
        assert all(w[1] in SOURCES for w in ways), gid
    if "--check" in sys.argv:
        stale = [p for p, text in ((DOC, document()), (WORDS, words_json()))
                 if not p.exists() or p.read_text() != text]
        if stale:
            print(f"out of date: {', '.join(str(p.relative_to(ROOT)) for p in stale)}; run tools/phrases.py")
            return 1
        return 0
    for p in (DOC, WORDS):
        p.parent.mkdir(exist_ok=True)
    DOC.write_text(document())
    WORDS.write_text(words_json())
    print(f"wrote {DOC.relative_to(ROOT)} and {WORDS.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
