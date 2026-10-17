#!/usr/bin/env python3
"""The opponent's phrase bank: everything said at the table, by game moment.

    python3 tools/phrases.py           # rewrite docs/PHRASES.md and web3d/words.json
    python3 tools/phrases.py --check   # fail if either is out of date

The single source of truth. A group is a moment (docs/RULES.md section 16,
the events); each wording in it is a tuple

    (english, source_key, german)
    (english, source_key, german, locator)

`source_key` resolves in SOURCES to a research-note id and the period wording.
"T" marks the table's own lines. `german` is what the German game says: where a
source gives a wording it is that original, trimmed or inflected to be spoken, in its period Austrian
form; the table's own ("T") lines are written in a plausible Austrian register. The voice is set
around 1891 and its sources run to 1938, the end of the game's documented life in
print; later sources are out (tools/test_phrases.py enforces it,
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
           "\"das Impassiren\"; \"ein Conto ... bei fortlaufender Addition\"; the Mond is not to be laid early \"so lange man nicht weiß, wo der Skis steht\". \"Mondfang\" is Werner's noun in his "
           "other games (Kauf, Königrufen), not in the Strohmann section. The spoken \"Ich nehme auf!\", "
           "\"Weiter!\", \"Ich passe!\" are inflections of Werner's \"aufnehmen\", \"weiter\", \"Paßt\"; the "
           "exact sentences are as Mayr and Sedlaczek (2007) quote them [05-S5]", True),
    "Ul": (1899, "[03-S14][13-S1]",
           "Ulmann, Illustrirtes Wiener Tarokbuch, 2nd ed. (1899): \"Es ist üblich, ehe die "
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
            "Ultimo in einem Nu\"; \"Mein Herr hat halt überall 's Glück in Pacht\" (Bonz 1890 only). The wording is edition-dependent: the Projekt Gutenberg text prints "
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
           "Karten geb'n?\"; \"Und wanns G'spiel aus is, werd'n Stich zählt und dabei erst kritisirt\"; \"Jeder Karten geben will\"", False),
    "Ba": (1756, "[05-S12][05d POLY-S1]",
           "Bauer (Vienna and Nuremberg 1756), Ode auf das Taroc-Spiel: \"Wenn mein Pagat den letzten "
           "sticht\"; \"Und kommt der lustge Scüs darzu, So sitz ich in vergnügter Ruh\"; \"Ein wenig "
           "Spielen bleibt erlaubt, Doch nur zum Zeitvertreib\"; \"Der Pagat ist den andern Mitspielern "
           "ein Dorn in den Augen\"; \"Ein andrer leg sich als ein Block ins weiche Bett: ich spiel Tarock\"", False),
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
           "Grazer Tagblatt, 15 Oct 1894, p. 2, on Johann Strauss's card play: \"bei zwei bis drei Kreuzer der Point\" (the newspaper's words, not Strauss's)", False),
    "Ga": (1869, "[09-S14]",
           "Die Gartenlaube (1869): \"das Fallen der Karten, das Rasseln der Sechser, die Seufzer der "
           "Verlierenden\"", False),
    "Jö": (1883, "[05c LIT-S27]",
           "Jörgel Briefe, 24 Mar 1883, p. 9 (OCR snippet): \"Ohne Tarok is kein Spiel\"", False),
    "Hn": (1727, "[05b LEX-S9]",
           "Henrici (1727), in Grimm's Wörterbuch, s.v. Stich: \"denn manchmahl bleibt ein blat in händen, "
           "das macht den allerletzten stich\"", False),
    "Lö": (1903, "[05c LIT-S9]",
           "Löwy, \"Der Giftnik'l\", Illustrirtes Wiener Extrablatt, 23 Aug 1903, p. 7: \"Jetzt waß i net, "
           "san zwölf oder fufzehn Tarock draußen!\"", False),
    "Ra": (1917, "[05c LIT-S8]",
           "Ranzenhofer, \"Königrufen\", Pilsner Tagblatt, 30 Sep 1917: \"'Guter Mond, du gehst so stille' "
           "summt der Bankdirektor und sticht mit dem Sküs den gegnerischen Mond ab\"; \"Ein guter Misch ist "
           "das halbe Spiel\"; \"Alles hat seine Reizes!\"; \"Nur mit gezogenen Plätzen spiele ich\"; "
           "\"Wieder lauter Skatindeln ... Den ganzen Abend hab' ich noch kein Tarock gesehen!\"; \"Na, so "
           "einen Dusel\"; \"Aha, jetzt kommt der Spatz unters Messer!\"; \"Nur noch eine Runde\"; "
           "\"Alles spielt gegen mich und trischakt mich mit Tarock?\"", False),
    "Mw": (1924, "[05c LIT-S10]",
           "(Neuigkeits) Welt Blatt, 23 Jan 1924, p. 7: \"Eine Karte oder ein Scheit Holz\"; \"Tun wir doch "
           "gleich lieber Schach spielen\"", False),
    "Kg": (1908, "[05c LIT-S16][03 §4 REVIEW RD]",
           "Kärntner Zeitung, 16 May 1908, p. 4: \"Just wie beim Tarock, beim Strohmandl: 35 und 2 Blatt; das "
           "ist eigentlich der stolzeste Sieg\" (page image reads 35)", False),
    "Lv": (1891, "[05c LIT-S58]",
           "Linzer Volksblatt, 20 Dec 1891, p. 1 (OCR snippet): \"der unwiderstehliche, alles niederrennende "
           "Sküs, der gute, sanfte Mond, und der kokette, fröhliche Pagat! Ein würdiges Trifolium!\"", False),
    "Fh": (1900, "[05c LIT-S60]",
           "Der Floh, 1900, Nr. 34, p. 3 (verse, OCR snippet): the Sküs \"der den Mond kann stechen\"", False),
    "Kz": (1911, "[05c LIT-S57]",
           "Kärntner Zeitung, 3 Dec 1911, p. 9 (OCR snippet): \"Heute regieren Sküs, Mond und Spatz oft an "
           "drei Tischen zugleich\"", False),
    "Mu": (1913, "[05c LIT-S36]",
           "Die Muskete, 14 Aug 1913, p. 6 (verse, OCR snippet): \"Verloren ist die Seelenruh', Der Pagat "
           "ultimo dazu.\"", False),
    "Wb": (1904, "[05c LIT-S44]",
           "(Neuigkeits) Welt Blatt, 1 Jun 1904, p. 7 (OCR snippet): \"Das 'Kiebitz'-Reglement hat auch nur "
           "einen Paragraphen ... Schweigende Kiebitze werden geduldet\"", False),
    "Mk": (1923, "[05c LIT-S42]",
           "Die Muskete, 1 Aug 1923, p. 6 (OCR snippet): \"als Kiebitz darf ich höchstens ein Nachtmahl "
           "zahlen, aber nichts dreinreden\"", False),
    "Ne5": (1845, "[05-S9][05-S22][05c LIT-S3]",
            "Nestroy, Unverhofft (1845), Bonz 1890 edition (OCR), Ledig: \"Beim Tarock kann ich doch 'was "
            "g'winnen, wenn ich einige Ultimo mach'\"", False),
    "Id": (1824, "[05b LEX-S33][05b LEX-S16]",
           "Idioticon Austriacum (Vienna 1824), also in Wander, Trumpf 5: \"Den hab i an Trumpf geben\" (a "
           "sharp retort)", False),
    "Sk": (1905, "[05-S2]",
           "Schranka, Wiener Dialekt-Lexikon (1905): \"Auf jemanden einen Pick haben\" (to bear a grudge, "
           "after the suit Pick)", False),
    "Kk": (1887, "[05c LIT-S23]",
           "Kikeriki, 9 Jan 1887, p. 3 (a riddle, OCR snippet): \"Der Sküs sticht immer Alles\"", False),
    "Gs": (1934, "[05c LIT-S54]",
           "Das kleine Volksblatt, 24 May 1934, p. 10 (OCR snippet): \"der Mond nur, wann er in G'sellschaft "
           "vom Gstieß und Pagat is\" (Gstieß: the Viennese street name of the Sküs)", False),
    "Bt": (1927, "[05c LIT-S62]",
           "Bettauers Wochenschrift, 3 Jan 1927, p. 7 (OCR snippet): \"Den ganzen Nachmittag haben wir "
           "Tarock gespielt! ... Strohmandl!\"", False),
}

MONDFANG_SOURCES = {k for k, v in SOURCES.items() if v[3]}


def phrase_groups():
    """Every group, in the order the table meets them, as (id, moment, [wordings])."""
    out = []

    def add(gid, moment, *ways):
        out.append((gid, moment, list(ways)))

    add('greet', 'sitting_start: the greeting',
        ('Strohmandeln, then. At your service.', 'Fi', 'Zu dienen.'),
        ('Two at Strohmandeln: how splendid.', 'Fi', 'Das trifft sich ja herrlich!'),
        ('A game of Tarock, to recover.', 'Sc', 'Und jetzt spielen wir eine Partie Tarock, um uns zu erholen.'),
        ('A little play is allowed, for pastime.', 'Ba', 'Ein wenig Spielen bleibt erlaubt, doch nur zum Zeitvertreib.'),
        ('No third for a Tapper? Strohmandeln it is.', 'Fi', 'Wie schade, daß wir keinen Dritten zu einem Tapper haben!'),
        ('Good evening. Straw men for two.', 'T', 'Guten Abend. Strohmandeln zu zweit.'))
    add('touren-3', 'sitting_start, three Touren: the Touren announced',
        ('Three Touren, then.', 'Ul', 'Drei Touren sagen wir an.'),
        ('Let us say three Touren.', 'Ul', 'Sagen wir drei Touren an.'),
        ('Three Touren: six deals, a short evening.', 'T', 'Drei Touren: sechs Spiele, ein kurzer Abend.'),
        ('Three Touren, if that suits you.', 'Hs', 'Drei Touren, wenn es Ihnen recht ist.'),
        ('Three Touren, then the reckoning.', 'T', 'Drei Touren, dann die Abrechnung.'))
    add('touren-6', 'sitting_start, six Touren: the Touren announced',
        ('Six Touren: a full evening.', 'Ul', 'Sechs Touren: ein voller Abend.'),
        ('Let us say six Touren.', 'Ul', 'Sagen wir sechs Touren an.'),
        ('Six Touren, twelve deals. We have time.', 'T', 'Sechs Touren, zwölf Spiele. Wir haben Zeit.'),
        ('Six Touren, if that suits you.', 'Hs', 'Sechs Touren, wenn es Ihnen recht ist.'),
        ('Six Touren, and the slate to keep.', 'Wn', 'Sechs Touren, und das Conto wird geführt.'))
    add('cut', 'sitting_start: drawing for the first deal',
        ('Let us cut for the deal.', 'T', 'Ziehen wir, wer zuerst gibt.'),
        ('Who leads? Who deals?', 'Mo', "Wer spielt aus? Wer hat Karten geb'n?"),
        ('Draw a card; the Touren begin with the dealer.', 'Be', 'Die Touren beginnen bei dem, der zuerst die Karten ausgab.'),
        ('Seats by lot only, I say.', 'Ra', 'Nur mit gezogenen Plätzen spiele ich.'))
    add('deal-mine', 'deal_start: the opponent deals',
        ('My deal. Fifteen, and three fours.', 'T', 'Ich gebe. Fünfzehn und dreimal vier.'),
        ('Dealt right, laid neat, and the whole wrong.', 'Wa', "Liegen thut's tüchtig, gegeben ist's richtig und's Ganze ist unrichtig.", 'Liegen 122'),
        ('Dealt one way, played another.', 'Wa', 'Die Karten werden anders gegeben, anders gespielet.', 'Karte 7'),
        ('I deal. Then the straw men.', 'T', 'Ich gebe. Dann die Strohmänner.'),
        ('A good shuffle is half the game.', 'Ra', 'Ein guter Misch ist das halbe Spiel.'))
    add('deal-yours', 'deal_start: the player deals',
        ('Your deal.', 'T', 'Sie geben.'),
        ('Deal on; the straw men wait.', 'T', 'Geben Sie nur, die Strohmänner warten.'),
        ('Your deal. Good luck to us both.', 'T', 'Sie geben. Viel Glück uns beiden.'),
        ('A good shuffle is half the game.', 'Ra', 'Ein guter Misch ist das halbe Spiel.'),
        ('Everyone wants to deal; you first.', 'Mo', 'Jeder will Karten geben; Sie zuerst.'))
    add('void-deal', 'void_deal: a hand with no tarock, the deal annulled',
        ('No trumps, no game.', 'Jö', 'Ohne Tarok is kein Spiel.'),
        ('Void. Gather them up and deal again.', 'T', 'Vergeben! Zusammenwerfen und neu geben.'),
        ('A void deal. It does not count.', 'T', 'Ein vergebenes Spiel zählt nicht.'),
        ('Not a tarock to be had: dealt again.', 'T', 'Kein Tarok zu haben: neu gegeben.'),
        ('Void: a misdeal.', 'Ne1', "'s is vergeb'n."))
    add('doubling-applied', 'doubling_applied: the deal counts double after a draw',
        ('Doubled: every payment twice.', 'Wn', "Doppelt zählt's, sammt allen Ansagen."),
        ('The last draw doubles this deal.', 'Wn', "Doppelt zählt's."),
        ('Double, mind: honours and all.', 'Wn', 'Doppelt, sammt allen Ansagen.'),
        ('This one counts twice.', 'T', 'Das zählt doppelt.'),
        ('Remember the draw: twice the Points.', 'T', 'Denken Sie an das Unentschieden: doppelte Points.'))
    add('bid-take-mine', 'bid: the opponent takes up',
        ('I take it up!', 'Wn', 'Ich nehme auf!'),
        ('I take it up. Let the straw speak.', 'T', 'Ich nehme auf. Der Strohmann soll reden.'),
        ('My game.', 'Wn', 'Mein Spiel.'),
        ("My own fortune's smith: I take it up.", 'Ul', 'Ich bin meines Glückes Schmied: ich nehme auf.'),
        ("I'll take it up, and trust the straw.", 'T', 'Ich nehme auf und vertraue dem Stroh.'),
        ('Taken up: a game, then.', 'T', 'Aufgenommen: ein Spiel also.'))
    add('bid-pass-mine', 'pass: the opponent passes',
        ('On, then: I pass.', 'Wn', 'Weiter!'),
        ('I pass.', 'Wn', 'Ich passe!'),
        ('Pass, if you please.', 'Wn', 'Weiter, bitte.'),
        ('I pass, and keep my peace.', 'T', "Ich passe und halt' Ruh'."),
        ('Not this time. On we go.', 'T', 'Diesmal nicht. Weiter.'))
    add('bid-take-yours', 'bid: the player takes up',
        ('Taken up! Good.', 'Wn', 'Aufgenommen! Gut.'),
        ('Taken up. Let us see the straw.', 'T', 'Aufgenommen. Schauen wir den Strohmann an.'),
        ('Then it is a game in earnest.', 'T', "Dann ist's ein Spiel im Ernst."),
        ('Good. Let it stand.', 'Wn', 'Gut. Es bleibt.'),
        ('You take it up. The luck will tell.', 'T', "Sie nehmen auf. Das Glück wird's zeigen."))
    add('bid-pass-yours', 'pass: the player passes',
        ('Pass, you say. Noted.', 'Wn', 'Weiter, sagen Sie. Gut.'),
        ('A quiet start.', 'Mo', 'Mäuserlstill.'),
        ('You pass; then I speak.', 'T', 'Sie passen; dann spreche ich.'),
        ('Passed. We shall see.', 'T', 'Gepasst. Wir werden sehen.'),
        ('Pass: quite so.', 'Wn', 'Weiter: sehr wohl.'))
    add('both-pass', 'both_pass, play: both pass and a simple game is played',
        ('Both pass: a simple game, then.', 'T', 'Beide passen: eine einfache Partie.'),
        ('Nobody takes it up. A plain game.', 'T', 'Keiner nimmt auf. Eine einfache Partie.'),
        ('Neither takes it. Plain play, then.', 'T', 'Beide weiter. Dann einfach gespielt.'),
        ('No one risks a game. On we go.', 'T', "Keiner riskiert ein Spiel. Weiter geht's."))
    add('thrown-in', 'thrown_in: both_pass = throw_in, nothing played',
        ('Thrown in. The other deals next.', 'T', 'Hingeworfen. Der andere gibt als Nächster.'),
        ('Nothing played; on to the next.', 'T', 'Nichts gespielt; weiter zum nächsten.'),
        ('Both pass, so we start afresh.', 'T', 'Beide passen, wir fangen neu an.'),
        ('Thrown in: the straw stays shut.', 'T', 'Hingeworfen: der Strohmann bleibt zu.'))
    add('reveal-begin', 'reveal_start: a player turns the straw men',
        ('Now the straw men, one by one.', 'T', 'Jetzt die Strohmänner, einen nach dem andern.'),
        ('Turn them over, left to right.', 'T', 'Umdrehen, von links nach rechts.'),
        ('Let us see what the straw says.', 'T', 'Schauen wir, was das Stroh sagt.'),
        ('Everything has its charms.', 'Ra', 'Alles hat seine Reizes!'),
        ('Look at the straw men, then.', 'T', 'Schauen wir die Strohmänner an.'))
    add('pull-king-mine', "card_pulled: a king pulled into the opponent's hand",
        ('A king from the straw!', 'T', 'Ein König aus dem Stroh!'),
        ('Ah, a king. Welcome.', 'T', 'Ah, ein König. Willkommen.'),
        ('A king: worth little, then worth much.', 'Wa', 'Wie der König im Kartenspiel, bald gilt er nichts, bald gilt er viel.', 'König 175'),
        ('The straw is generous today.', 'T', 'Das Stroh ist heut großzügig.'),
        ('Another king for my hand.', 'T', 'Noch ein König in die Hand.'),
        ('Kings and courts from the straw: an easy game.', 'Ha', 'Schmiert der Strohmann viel Könige und Figuren, so ist das Spiel leicht zu gewinnen.'),
        ('Honours are only the cadres of the army.', 'Ul', 'Die Honneurs sind nur die Cadres der Kartenarmee.'),
        ('Everything has its charms.', 'Ra', 'Alles hat seine Reizes!'))
    add('pull-king-yours', "card_pulled: a king pulled into the player's hand",
        ('A king for you. Well turned.', 'T', 'Ein König für Sie. Schön umgedreht.'),
        ('The king comes to you.', 'T', 'Der König kommt zu Ihnen.'),
        ('A king: worth little, then worth much.', 'Wa', 'Wie der König im Kartenspiel, bald gilt er nichts, bald gilt er viel.', 'König 175'),
        ('The straw is kind to you.', 'T', 'Das Stroh ist gut zu Ihnen.'),
        ('There goes a king, into the hand.', 'T', 'Da geht ein König in die Hand.'),
        ('Kings and courts from the straw: an easy game.', 'Ha', 'Schmiert der Strohmann viel Könige und Figuren, so ist das Spiel leicht zu gewinnen.'),
        ('Luck on lease, everywhere.', 'Ne1', "Der hat halt überall 's Glück in Pacht."),
        ('Each is the smith of his own luck.', 'Ul', 'Jeder ist seines eigenen Glückes Schmied.'))
    add('pull-tarock-mine', "card_pulled: a tarock pulled into the opponent's hand",
        ('A tarock! Gladly taken.', 'T', 'Ein Tarok! Gern genommen.'),
        ('Tarock from the straw.', 'T', 'Tarok aus dem Stroh.'),
        ('Better taken than missed.', 'Wa', 'Besser mitgenommen als drumgekommen.', 'Mitnehmen 1'),
        ('One more trump. That helps.', 'T', 'Noch ein Tarok. Das hilft.'),
        ('To play, one must know what is trump.', 'Wa', 'Wer spielen will, muss wissen, was Trumpf ist.', 'Spielen 72'),
        ('Without trumps there is no game.', 'Jö', 'Ohne Tarok is kein Spiel.'),
        ('Let others sleep like logs; I play Tarock.', 'Ba', 'Ein andrer leg sich als ein Block ins weiche Bett: ich spiel Tarock.'),
        ('How splendidly that falls out!', 'Fi', 'Das trifft sich ja herrlich!'))
    add('pull-tarock-yours', "card_pulled: a tarock pulled into the player's hand",
        ('A tarock for you.', 'T', 'Ein Tarok für Sie.'),
        ('Trumps from the straw: well turned.', 'T', 'Tarok aus dem Stroh: schön umgedreht.'),
        ('One more trump to count.', 'T', 'Noch ein Tarok zu zählen.'),
        ('The straw gives, the straw takes.', 'T', 'Das Stroh gibt, das Stroh nimmt.'),
        ('To play, one must know what is trump.', 'Wa', 'Wer spielen will, muss wissen, was Trumpf ist.', 'Spielen 72'),
        ('The little ones chase the big.', 'Wa', 'Die Kleinen jagen die Grossen.', 'Kleine 10'),
        ('Let others sleep like logs; I play Tarock.', 'Ba', 'Ein andrer leg sich als ein Block ins weiche Bett: ich spiel Tarock.'),
        ('A few ultimos, and Tarock pays.', 'Ne5', "Beim Tarock kann man doch 'was g'winnen, wenn man einige Ultimo macht."))
    add('pull-sku', 'card_pulled: the Sküs pulled (either side)',
        ('The Sküs! A wish nearly granted.', 'Hu', "Der Sküs! Er hängt scho' der Max'."),
        ('And the merry Sküs joins in.', 'Ba', 'Und kommt der lustge Scüs darzu.'),
        ('The Sküs has come home.', 'T', 'Der Sküs ist heimgekehrt.'),
        ('The highest trump, out of the straw.', 'T', 'Der höchste Tarok, aus dem Stroh.'),
        ('A Sküs: the one card nothing beats.', 'T', 'Ein Sküs: die eine Karte, die keiner sticht.'),
        ('The irresistible Sküs!', 'Lv', 'Der unwiderstehliche, alles niederrennende Sküs!'),
        ('Sküs, Mond and the little sparrow.', 'Kz', 'Sküs, Mond und Spatz.'),
        ('The Sküs beats everything.', 'Kk', 'Der Sküs sticht immer Alles.'))
    add('pull-mond', 'card_pulled: the Mond (XXI) pulled',
        ('The Mond! Watch the Sküs.', 'T', 'Der Mond! Achtung auf den Sküs.'),
        ('Mond out of the straw.', 'T', 'Der Mond aus dem Stroh.'),
        ('The Mond is turned.', 'T', 'Der Mond ist umgedreht.'),
        ('The good, gentle Mond.', 'Lv', 'Der gute, sanfte Mond.'),
        ('Good moon, you come so quietly.', 'Ra', 'Guter Mond, du gehst so stille.'),
        ('Keep the Mond back while the Sküs is unknown.', 'Wn', 'Den Mond nicht früh legen, solange man nicht weiß, wo der Skis steht.'),
        ('Only the Sküs can trump the Mond.', 'Fh', 'Nur der Sküs kann den Mond stechen.'),
        ('The Mond matters only with Sküs and Pagat.', 'Gs', "Der Mond interessiert nur in G'sellschaft vom Gstieß und Pagat."))
    add('pull-pagat', 'card_pulled: the Pagat (I) pulled',
        ('The Pagat! A little trump, a great hope.', 'T', 'Der Pagat! Ein kleiner Tarok, eine große Hoffnung.'),
        ('Pagat out of the straw.', 'T', 'Pagat aus dem Stroh.'),
        ('The Pagat is here.', 'T', 'Der Pagat ist da.'),
        ('The Pagat: coquettish and cheerful.', 'Lv', 'Der kokette, fröhliche Pagat!'),
        ('Sküs, Mond and the little sparrow.', 'Kz', 'Sküs, Mond und Spatz.'),
        ('The Pagat, a thorn in my eye!', 'Ba', 'Der Pagat ist mir ein Dorn im Auge!'),
        ('A few ultimos, and Tarock pays.', 'Ne5', "Beim Tarock kann man doch 'was g'winnen, wenn man einige Ultimo macht."),
        ('Now the little sparrow goes under the knife!', 'Ra', 'Jetzt kommt der Spatz unters Messer!'))
    add('run-mine', 'card_pulled, long chain: a lucky run for the opponent',
        ('Three in a row! The straw is kind.', 'T', 'Drei hintereinander! Das Stroh ist gut.'),
        ('Luck is better than good play.', 'Wa', 'Glück ist besser als gut Spielen.', 'Glück 360'),
        ('What a run from the straw!', 'T', 'Was für eine Serie aus dem Stroh!'),
        ('Luck on lease, everywhere.', 'Ne1', "Der hat halt überall 's Glück in Pacht."),
        ('Lucky folk play weak cards well.', 'Wa', 'Wer Glück hat, spielt mit schlechten Karten gut.', 'Glück 850'),
        ('Three, four, in a flash!', 'Ne1', 'Der macht drei, vier in einem Nu.'))
    add('run-yours', 'card_pulled, long chain: a lucky run for the player',
        ('What a run! Three, four, five.', 'T', 'Was für eine Serie! Drei, vier, fünf.'),
        ('Easy to play when luck deals.', 'Wa', 'Wem das Glück die Karten gibt, der hat gut spielen.', 'Glück 665'),
        ('The cards run your way tonight.', 'T', 'Die Karten laufen Ihnen heute zu.'),
        ('The straw empties itself into the hand.', 'T', 'Das Stroh leert sich in die Hand.'),
        ('Luck at cards is April weather.', 'Wa', 'Glück im Spiel ist Wetter im April.', 'Glück'),
        ('Three, four, in a flash!', 'Ne1', 'Der macht drei, vier in einem Nu.'))
    add('nothing-mine', "packet_turned, no pull: the opponent's packets give nothing",
        ('Nothing but low cards!', 'Gl', 'Lauter Skartindeln!'),
        ('Nothing from the straw. Patience.', 'T', 'Nichts aus dem Stroh. Geduld.'),
        ('Not a trump, not a king.', 'T', 'Kein Tarok, kein König.'),
        ('Not a single tarock all evening!', 'Ra', "Den ganzen Abend hab' ich noch kein Tarock gesehen!"),
        ('Without the best cards, nothing takes a trick.', 'Wa', 'Wenn die besten Blätter verworffen, so kann man nichts mehr stechen.', 'Blatt 53'),
        ('Lead small; one can always grow big.', 'Wa', "Klan ausg'spielt, gross koan ma oallweil wer'n.", 'Klein 14'),
        ('Not won yet; the cards still lie out.', 'Wa', "Das Spiel ist noch nicht gewonnen, die Kart liegt noch auf'm Tisch.", 'Spiel 42'),
        ('A card or a log of wood, either will do.', 'Mw', 'Eine Karte oder ein Scheit Holz!'))
    add('nothing-yours', "packet_turned, no pull: the player's packets give nothing",
        ('Nothing for you. The straw is stingy.', 'T', 'Nichts für Sie. Das Stroh ist geizig.'),
        ('Blind chance, nothing more.', 'Ha', 'Blinder Zufall, sonst nichts.'),
        ('The straw gave you low cards only.', 'T', 'Das Stroh gab Ihnen nur kleine Karten.'),
        ('It counts the same for both of us.', 'Ha', 'Es gilt ja Einem wie dem Andern.'),
        ('Without the best cards, nothing takes a trick.', 'Wa', 'Wenn die besten Blätter verworffen, so kann man nichts mehr stechen.', 'Blatt 53'),
        ('Lead small; one can always grow big.', 'Wa', "Klan ausg'spielt, gross koan ma oallweil wer'n.", 'Klein 14'),
        ('Nothing but low cards!', 'Gl', 'Lauter Skartindeln!'),
        ('The straw was frugal with you today.', 'T', 'Das Stroh war heut sparsam mit Ihnen.'))
    add('slow', 'the player is slow to play',
        ('Your turn, I believe.', 'T', "Sie sind an der Reihe, glaub' ich."),
        ('Mouse-quiet: the table waits.', 'Mo', 'Mäuserlstill.'),
        ('Take your time. The evening is long.', 'T', "Lassen S' Ihnen Zeit. Der Abend ist lang."),
        ('Your lead, whenever you are ready.', 'T', "Spielen S' aus, wann's Ihnen recht ist."),
        ('No hurry. The cards are patient.', 'T', 'Keine Eile. Die Karten warten.'),
        ('Thinking it over?', 'T', "Überlegen S'?"),
        ('Not a trump will walk by itself.', 'T', 'Von selber geht kein Tarok.'),
        ('A card or a log of wood, either will do.', 'Mw', 'Eine Karte oder ein Scheit Holz!'),
        ('This pace? We might have played chess.', 'Mw', 'Tun wir doch gleich lieber Schach spielen!'))
    add('trick-good-mine', 'trick_won: a good trick for the opponent',
        ('Trump against trump, and mine.', 'Wa', 'Trumpf wider Trumpf.', 'Trumpf 4'),
        ('That one is mine, I think.', 'T', "Der gehört mir, glaub' ich."),
        ('Taken. Thank you.', 'T', 'Genommen. Danke.'),
        ('A trick for me, it seems.', 'T', "Ein Stich für mich, scheint's."),
        ('Another trick on my side.', 'T', 'Wieder ein Stich auf meiner Seite.'),
        ('Struck, the buck: why stray into the cabbage?', 'Wa', 'Gestochen der Bock, warum geht er ins Kraut.', 'Bock'),
        ('I gave that one a trump.', 'Id', 'Den hab i an Trumpf geben.'),
        ('A kind heart wins no game.', 'Wa', 'Mit einem guten Herzen gewinnt man kein Spiel.', 'Herz 239'),
        ('The lesser beat the greater these days.', 'Wa', 'Die geringern stechen die höhern.', 'Spielen 22'))
    add('trick-good-yours', 'trick_won: a good trick for the player',
        ('Well taken.', 'T', 'Gut genommen.'),
        ('The little ones chase the big.', 'Wa', 'Die Kleinen jagen die Grossen.', 'Kleine 10'),
        ('Your trick, and fairly won.', 'T', 'Ihr Stich, und ehrlich gewonnen.'),
        ('Good cards like you, it seems.', 'Wa', 'An einen geschickten Spieler gewöhnen sich gute Karten.', 'Spieler 3'),
        ('Trump against trump, and yours.', 'Wa', 'Trumpf wider Trumpf.', 'Trumpf 4'),
        ('Struck, the buck: why stray into the cabbage?', 'Wa', 'Gestochen der Bock, warum geht er ins Kraut.', 'Bock'),
        ('The lesser beat the greater these days.', 'Wa', 'Die geringern stechen die höhern.', 'Spielen 22'),
        ('You know what is trump.', 'Wa', 'Sie wissen, was Trumpf ist.', 'Wissen 541'),
        ('Early winners, late losers, they say.', 'Wa', 'Die ersten Hunde ertränkt man gern.', 'Hund 301'))
    add('trick-bad-mine', 'trick_won: a bad trick for the opponent, points lost',
        ('The leaf has turned.', 'Wa', 'Das Blatt hat sich gewandt.', 'Blatt 29'),
        ('Not won till the cards are out.', 'Wa', 'So lange die Karten auf dem Tisch liegen, ist das Spiel nicht gewonnen.', 'Karte 89'),
        ('There goes a trick. It happens.', 'T', 'Da geht ein Stich. Das kommt vor.'),
        ('That one was yours, and rightly.', 'T', 'Das war Ihrer, und mit Recht.'),
        ('Everyone plays against me, and with trumps!', 'Ra', 'Alles spielt gegen mich und trischakt mich mit Tarock?'),
        ('A grudge against you, in play only.', 'Sk', 'Einen Pick auf Sie, nur im Spiel.'),
        ('Tricks counted, and criticised after.', 'Mo', "Werd'n Stich zählt und dabei erst kritisirt."),
        ('The card has turned.', 'Wa', 'Die Karte hat sich gewendet.', 'Karte 44'))
    add('trick-bad-yours', 'trick_won: a bad trick for the player',
        ('The next one may be yours.', 'T', 'Der nächste gehört vielleicht Ihnen.'),
        ('Nothing is lost while cards lie out.', 'Wa', 'So lange die Karten auf dem Tisch liegen, ist das Spiel nicht gewonnen.', 'Karte 89'),
        ('A good hand comes back.', 'Wa', 'Ein gut Spiel kommt wieder.', 'Spiel'),
        ('Dealt one way, played another.', 'Wa', 'Die Karten werden anders gegeben, anders gespielet.', 'Karte 7'),
        ('I gave that one a trump.', 'Id', 'Den hab i an Trumpf geben.'),
        ('A kind heart wins no game.', 'Wa', 'Mit einem guten Herzen gewinnt man kein Spiel.', 'Herz 239'),
        ('Count the tarocks diligently.', 'Ro', "Zählen S' fleißig die Tarok!"),
        ('A trick lost is not a game lost.', 'T', 'Ein verlorener Stich ist noch kein verlorenes Spiel.'))
    add('sku-played', 'card_played: the Sküs played (either side)',
        ('Sküs: I excuse myself!', 'Me', 'Ich skisiere mich!'),
        ('Pardon me: the Sküs.', 'Ha', 'Pardon, ich skisire mich.'),
        ('And the merry Sküs joins in.', 'Ba', 'Und kommt der lustge Scüs darzu.'),
        ('The Sküs: the highest of the trumps.', 'T', 'Der Sküs: der höchste Tarok.'),
        ('Sküs it is. Nothing beats it.', 'T', 'Sküs! Da hilft nichts mehr.'))
    add('mond-caught-mine', "trick_won: the opponent's Sküs catches the player's Mond",
        ('Mondfang! The Sküs has your Mond.', 'Wn', 'Mondfang!'),
        ('Wasted effort: the Sküs takes the Mond.', 'Ro', 'Unnütze Mühe, der Scüs sticht ja den Mond.'),
        ('Struck by the proud Sküs.', 'Zi', 'Vom stolzen Skis getroffen.'),
        ('A Mondfang: a special catastrophe.', 'NF', 'Der Mondfang ist eine Special-Katastrophe.'),
        ('The Sküs caught the Mond. Sorry.', 'T', 'Der Sküs hat den Mond. Verzeihung.'),
        ('Good moon, you go so quietly.', 'Ra', 'Guter Mond, du gehst so stille.'))
    add('mond-caught-yours', "trick_won: the player's Sküs catches the opponent's Mond",
        ('Caught again: the Sküs hunts my Mond.', 'Fl', 'Wieder fangt mir der Sküs den Mond ab.'),
        ('Mondfang! It had to happen.', 'Wn', 'Mondfang!'),
        ('Wasted effort: the Sküs takes the Mond.', 'Ro', 'Unnütze Mühe, der Scüs sticht ja den Mond.'),
        ('Well caught: the Sküs wins the Mond.', 'T', 'Gut gefangen: der Sküs hat den Mond.'),
        ('A Mondfang: a special catastrophe.', 'NF', 'Der Mondfang ist eine Special-Katastrophe.'),
        ('Only the Sküs can trump the Mond.', 'Fh', 'Nur der Sküs kann den Mond stechen.'))
    add('ultimo-won-mine', 'pagat_ultimo won by the opponent',
        ('My Pagat takes the last trick!', 'Ba', 'Mein Pagat sticht den letzten!'),
        ('Pagat ultimo: made.', 'Ro', 'Ich habe den Pagat Ultimo gemacht!'),
        ('Won with flair.', 'Sc', 'Mit Glanz gewonnen.'),
        ('One card kept back makes the last trick.', 'Hn', 'Manchmal bleibt ein Blatt in Händen, das macht den allerletzten Stich.'),
        ('The Pagat comes last, and wins.', 'T', 'Der Pagat kommt zuletzt, und er gewinnt.'))
    add('ultimo-won-yours', 'pagat_ultimo won by the player',
        ('Pagat ultimo for you. Won with flair.', 'Sc', 'Mit Glanz gewonnen.'),
        ('Your Pagat takes the last trick.', 'Ba', 'Der Pagat sticht den letzten.'),
        ('One card kept back makes the last trick.', 'Hn', 'Manchmal bleibt ein Blatt in Händen, das macht den allerletzten Stich.'),
        ('The Pagat, a thorn in my eye!', 'Ba', 'Der Pagat ist mir ein Dorn im Auge!'),
        ('The Pagat last of all: well done.', 'T', 'Der Pagat zuletzt: gut gemacht.'))
    add('ultimo-lost-mine', 'pagat_ultimo lost by the opponent',
        ('Lost on the very last card.', 'Wa', 'Man kann ein gutes Spiel noch mit der letzten Karte verlieren.', 'Spiel 104'),
        ('My ultimo has floated away.', 'Fg', "Der Pagat-Ultimo is wegg'schwommen."),
        ('The ultimo is lost.', 'Nw', 'Der Ultimo ist verloren.'),
        ('Caught. I should have counted.', 'T', "Gefangen. Ich hätt' zählen sollen."),
        ('The Pagat falls; so it goes.', 'T', "Der Pagat fällt; so geht's."),
        ('Lost my calm, and my Pagat with it.', 'Mu', "Verloren ist die Seelenruh', der Pagat ultimo dazu."))
    add('ultimo-lost-yours', 'pagat_ultimo lost by the player',
        ('Lost on the very last card.', 'Wa', 'Man kann ein gutes Spiel noch mit der letzten Karte verlieren.', 'Spiel 104'),
        ('Count the tarocks next time.', 'Ro', "Zählen S' fleißig die Tarok!"),
        ('The ultimo is lost.', 'Nw', 'Der Ultimo ist verloren.'),
        ('Caught! The Pagat will try again.', 'T', "Gefangen! Der Pagat versucht's wieder."),
        ('A pity: the Pagat was so near.', 'T', 'Schade: der Pagat war so nah.'),
        ('Now the little sparrow goes under the knife!', 'Ra', 'Jetzt kommt der Spatz unters Messer!'))
    add('trull-mine', 'announcement trull by the opponent',
        ('I announce the Troule.', 'Wn', 'Ich sage die Troule an.'),
        ('All three!', 'Ne1', 'Tous les trois!'),
        ('The Troule: Sküs, Mond, Pagat.', 'Wn', 'Die Troule: Sküs, Mond, Pagat.'),
        ('The Troule, announced aloud, one Point.', 'T', 'Die Troule, laut angesagt: ein Point.'),
        ('All three, if you please.', 'Ne1', 'Tous les trois, wenn ich bitten darf.'))
    add('trull-yours', 'announcement trull by the player',
        ('All three! One Point to you.', 'Ne1', 'Tous les trois! Ein Point für Sie.'),
        ('The Troule is announced. Noted.', 'Wn', 'Die Troule ist angesagt. Gut.'),
        ('The Troule: I shall mind the Sküs.', 'T', "Die Troule: ich pass' auf den Sküs auf."),
        ('All three: the evening pays.', 'T', 'Tous les trois: der Abend zahlt sich aus.'),
        ('A Troule, and said in time.', 'Wn', 'Eine Troule, und rechtzeitig angesagt.'))
    add('kings-mine', 'announcement kings by the opponent',
        ('Four kings!', 'Ne1', 'Vier Könige!'),
        ('Four kings, announced.', 'T', 'Vier Könige, angesagt.'),
        ('All four kings: the straw was kind.', 'T', 'Alle vier Könige: das Stroh war gnädig.'),
        ('The four kings, one Point.', 'T', 'Die vier Könige: ein Point.'),
        ('Four kings, and said in time.', 'Wn', 'Vier Könige, rechtzeitig angesagt.'))
    add('kings-yours', 'announcement kings by the player',
        ('Four kings! One Point to you.', 'Ne1', 'Vier Könige! Ein Point für Sie.'),
        ('Four kings: duly noted.', 'Ne1', 'Vier Könige: wohlvermerkt.'),
        ('The four kings are announced.', 'T', 'Die vier Könige sind angesagt.'),
        ('Honours are only the cadres of the army.', 'Ul', 'Die Honneurs sind nur die Cadres der Kartenarmee.'),
        ('Four kings, and said in time.', 'Wn', 'Vier Könige, rechtzeitig angesagt.'))
    add('honour-forfeited', 'honour_forfeited: announced too late',
        ('Not said in time: no honour.', 'T', 'Nicht rechtzeitig angesagt: kein Honneur.'),
        ('Too late to announce; it is forfeit.', 'T', 'Zu spät angesagt; es verfällt.'),
        ('Say it by the first card played.', 'T', 'Spätestens mit der ersten Karte muß es angesagt sein.'),
        ('The honour is lost by silence.', 'T', 'Wer schweigt, verliert den Honneur.'))
    add('valat-mine', 'valat by the opponent (every trick)',
        ('Matsch! Every last trick.', 'T', 'Matsch! Jeder Stich.'),
        ('Valat! The whole table is mine.', 'T', 'Valat! Der ganze Tisch gehört mir.'),
        ('Not a trick for you. Quite matsch.', 'T', 'Kein Stich für Sie. Ganz matsch.'),
        ('Valat: every trick of the twenty-seven.', 'T', 'Valat: alle siebenundzwanzig Stich.'))
    add('valat-yours', 'valat by the player (every trick)',
        ('I am matsch: finished!', 'Ne2', 'Ich bin matsch!'),
        ('Not one trick for me. Matsch!', 'Ne3', 'Ich bin Matsch worden.'),
        ('Every trick yours. Well played.', 'T', 'Jeder Stich gehört Ihnen. Gut gespielt.'),
        ('Everyone plays against me tonight!', 'Ra', 'Alles spielt gegen mich!'))
    add('count-start', 'count: the tricks are counted',
        ('Now we count.', 'T', 'Jetzt wird gezählt.'),
        ('Counted, and then argued about.', 'Mo', 'Wird gezählt, und dann erst kritisirt.'),
        ('To the thirds, if you please.', 'T', 'Auf die Drittel genau, bitte.'),
        ('The count, honestly kept.', 'T', 'Das Zählen, ehrlich geführt.'),
        ('Let us count it out to the thirds.', 'T', "Zählen wir's auf die Drittel aus."))
    add('count-close', 'count: a very close count, 35 and two thirds',
        ('Thirty-five and two thirds: just enough.', 'Pr', 'Fünfunddreißig und zwei Blatt: gerade genug.'),
        ("A hair's breadth: thirty-five and two thirds.", 'Pr', 'Auf Haaresbreite: fünfunddreißig und zwei Blatt.'),
        ('It counts the same for both of us.', 'Ha', 'Es gilt ja Einem wie dem Andern.'),
        ('By a single third!', 'T', 'Um ein einziges Drittel!'),
        ('Thirty-five and two thirds: the proudest win.', 'Kg', 'Fünfunddreißig und zwei Blatt: der stolzeste Sieg.'))
    add('result-won-mine', 'game_scored: the opponent wins the deal',
        ('Won, and fairly, I hope.', 'T', "Gewonnen, und ehrlich, hoff' ich."),
        ('Taken up and won. The straw obliged.', 'T', 'Aufgenommen und gewonnen. Das Stroh war willig.'),
        ('Hearts are trumps: boldness wins.', 'Wa', 'Herz ist Trumpf.', 'Herz 197'),
        ('Lucky folk play weak cards well.', 'Wa', 'Wer Glück hat, spielt mit schlechten Karten gut.', 'Glück 850'),
        ('The deal is mine; the slate will say so.', 'T', 'Das Spiel geht an mich; die Tafel wird es sagen.'))
    add('result-lost-mine', 'game_scored: the opponent loses the deal',
        ('I play just as well, only with worse luck.', 'Cr', 'Ich spiele es gerade so gut, nur habe ich mehr Pech.'),
        ('The game hates me today.', 'Wa', 'Das Spiel hasst mich.', 'Spiel 274, Zusätze'),
        ('Yours, then. Deal on.', 'T', 'Ihres also. Geben wir weiter.'),
        ('Not won yet; the cards still lie out.', 'Wa', "Das Spiel ist noch nicht gewonnen, die Kart liegt noch auf'm Tisch.", 'Spiel 42'),
        ('One more round, then.', 'Ra', 'Nur noch eine Runde!'))
    add('result-won-yours', 'game_scored: the player wins the deal',
        ('Yours. Well played, and well turned.', 'T', 'Ihres. Gut gespielt und schön umgedreht.'),
        ('Easy to play when luck deals.', 'Wa', 'Wem das Glück die Karten gibt, der hat gut spielen.', 'Glück 665'),
        ('Won with flair. My compliments.', 'Sc', 'Mit Glanz gewonnen. Mein Kompliment.'),
        ('A deal for you. The evening is long.', 'T', 'Ein Spiel für Sie. Der Abend ist lang.'),
        ('A deal to you, and fairly won.', 'T', 'Das Spiel geht an Sie, und ehrlich gewonnen.'))
    add('result-lost-yours', 'game_scored: the player loses the deal',
        ('Mine this time. Next one may be yours.', 'T', 'Diesmal meines. Das nächste vielleicht Ihres.'),
        ('A good hand comes back.', 'Wa', 'Ein gut Spiel kommt wieder.', 'Spiel'),
        ('You play as well; the luck was mine.', 'Cr', 'Sie spielen ebenso gut; das Glück war meines.'),
        ('The card has turned.', 'Wa', 'Die Karte hat sich gewendet.', 'Karte 44'),
        ('Mine this time; the evening is long.', 'T', 'Diesmal meines; der Abend ist lang.'))
    add('draw', 'game_scored, draw: 35 and 35, nothing decided',
        ('A draw. The next deal counts double.', 'Wn', 'Unentschieden. Das nächste Spiel zählt doppelt.'),
        ('A dead heat. Doubled next time.', 'Wn', 'Gleichstand. Das nächste Mal doppelt.'),
        ('Thirty-five each: nobody wins.', 'T', 'Fünfunddreißig zu fünfunddreißig: keiner gewinnt.'),
        ('Nobody won. The next counts twice.', 'T', 'Keiner hat gewonnen. Das nächste zählt doppelt.'),
        ('Neither wins. The doubling waits.', 'T', 'Keiner gewinnt. Die Verdopplung wartet.'))
    add('doubling-lapsed', 'doubling_lapsed: a pending doubling ends with the sitting',
        ('The doubling lapses with the evening.', 'T', 'Die Verdopplung verfällt mit dem Abend.'),
        ('No more deals, so no doubling.', 'T', 'Keine Spiele mehr, also keine Verdopplung.'),
        ('A doubling does not outlast the evening.', 'T', 'Eine Verdopplung überdauert den Abend nicht.'))
    add('tour-end', 'tour_end: two deals are done',
        ('A Tour done. The slate, if you please.', 'T', 'Eine Tour um. Die Tafel, bitte.'),
        ('Let us write it up.', 'Wn', "Schreiben wir's an."),
        ('Two deals done: that is a Tour.', 'T', 'Zwei Spiele sind um: das ist eine Tour.'),
        ('Fetch the chalk.', 'T', 'Die Kreide her!'),
        ('Tour over; the account runs on.', 'Wn', 'Tour vorbei; das Conto läuft weiter.'))
    add('tour-last', 'deal_start of the last Tour',
        ('The closing Touren, then.', 'Be', 'Die Schluß-Touren.'),
        ('The final Touren: play them well.', 'Be', "Die Schluß-Touren: spielen S' sie gut."),
        ('One more Tour, and then the reckoning.', 'T', 'Noch eine Tour, dann die Abrechnung.'),
        ('The Touren run out. Deal with care.', 'T', "Die Touren gehen aus. Geben S' acht."),
        ('Last Tour: we are nearly home.', 'T', 'Letzte Tour: wir sind bald daheim.'))
    add('extend-ask', 'extension_offered: one more Tour?',
        ('One more Tour?', 'T', 'Noch eine Tour?'),
        ('Another Tour, if we both agree.', 'T', 'Eine Tour mehr, wenn wir beide zustimmen.'),
        ('Shall we add a Tour?', 'T', 'Hängen wir eine Tour an?'),
        ('Only if we both agree: one more?', 'Be', 'Nur mit beiderseitiger Zustimmung: noch eine?'),
        ('One more Tour, what do you say?', 'T', "Noch eine Tour, was sagen S'?"),
        ('Just one more round?', 'Ra', 'Nur noch eine Runde?'))
    add('extend-yes', 'extension_decided, accepted',
        ('One more Tour! Gladly.', 'T', 'Noch eine Tour! Gern.'),
        ('Agreed: the Touren are extended.', 'Be', 'Einverstanden: die Touren werden erstreckt.'),
        ('One more, then. Deal.', 'T', "Noch eine also. Geben S'."),
        ('Good. The evening is not over.', 'T', 'Gut. Der Abend ist noch nicht aus.'),
        ('One more round, then.', 'Ra', 'Nur noch eine Runde.'),
        ('Agreed. The cards are still warm.', 'T', 'Einverstanden. Die Karten sind noch warm.'))
    add('extend-no', 'extension_decided, declined',
        ('Then we stop here, as friends.', 'T', 'Dann hören wir hier auf, als Freunde.'),
        ('Enough for tonight. The reckoning.', 'T', "Genug für heut'. Die Abrechnung."),
        ('No more Touren. To the slate.', 'T', 'Keine Touren mehr. Zur Tafel.'),
        ('That is all for tonight.', 'T', 'Schluß für heute.'),
        ('Enough: the cards may rest.', 'T', 'Genug: die Karten dürfen ruhen.'))
    add('settle-opp-pays', 'sitting_end: the opponent pays',
        ('Nothing for it: I owe you.', 'T', 'Da hilft nichts: ich schulde Ihnen was.'),
        ('Two kreuzer a Point: cheap at the price.', 'St', 'Zwei Kreuzer der Point: billig.'),
        ('The rattle of the coins, and my sighs.', 'Ga', 'Das Rasseln der Sechser, und ich seufze.'),
        ('Paid is paid. Well played.', 'T', 'Bezahlt ist bezahlt. Gut gespielt.'),
        ('The coins are yours.', 'T', 'Die Münzen gehören Ihnen.'))
    add('settle-player-pays', 'sitting_end: the player pays',
        ("Two kreuzer a Point; I'll take it gladly.", 'St', "Zwei Kreuzer der Point; ich nehm's gern."),
        ('The rattle of the coins: mine, this time.', 'Ga', 'Das Rasseln der Sechser: diesmal meine.'),
        ('I stop while ahead, like Charlemagne.', 'Wa', "Ich spiele wie Karl der Große: ich hör' auf, wenn ich gewonnen hab'.", 'Spielen 109'),
        ('The slate is settled. My thanks.', 'T', 'Die Tafel ist abgerechnet. Meinen Dank.'),
        ('A small price for a good evening.', 'T', 'Ein kleiner Preis für einen guten Abend.'))
    add('settle-even', 'sitting_end: quits, nobody owes',
        ('Quits. Nobody owes anything.', 'T', 'Quitt. Keiner schuldet was.'),
        ('An even evening. The luck was fair.', 'T', 'Ein ausgeglichener Abend. Das Glück war gerecht.'),
        ('Not a kreuzer either way.', 'T', 'Kein Kreuzer hin oder her.'),
        ('Quits, and the evening was the gain.', 'T', 'Quitt, und der Abend war der Gewinn.'))
    add('farewell', 'after sitting_end: the farewell',
        ('Tomorrow, the honour of another game?', 'Bl', 'Darf ich morgen wieder um die Ehre einer Partie bitten?'),
        ('Thank you for the game.', 'T', 'Ich danke für das Spiel.'),
        ('A good evening. Until the next.', 'T', 'Ein guter Abend. Bis zum nächsten Mal.'),
        ('Good night, and good cards.', 'T', 'Gute Nacht, und gute Karten.'),
        ('A little play is allowed, they say.', 'Ba', 'Ein wenig Spielen bleibt erlaubt.'),
        ('That is all for tonight. Do come again.', 'T', "Schluß für heut'. Kommen S' wieder."),
        ('All afternoon we played Tarock!', 'Bt', 'Den ganzen Nachmittag haben wir Tarock gespielt!'))
    add('blunder-own', "the opponent's own blunder",
        ('A slip is forfeit, even for me.', 'Ve', 'Versehen ist verspielt.'),
        ('A slip; I play on.', 'Ve', "Ein Versehen; ich spiel' ruhig weiter."),
        ('I should have counted the tarocks.', 'Ro', "Ich hätt' fleißig die Tarok zählen sollen."),
        ('That one was a slip.', 'T', 'Das war ein Versehen.'),
        ("I wasn't looking. My own fault.", 'T', 'Nicht aufgepasst. Meine Schuld.'),
        ('Twelve out, or fifteen? I lost count.', 'Lö', 'Jetzt waß i net, san zwölf oder fufzehn Tarock draußen!'))
    add('slip-yours', "the player's slip (a card played in error)",
        ('A slip is forfeit. Play on.', 'Ve', "Versehen ist verspielt. Spielen S' ruhig weiter."),
        ('A slip is forfeit, that is all.', 'Ve', 'Versehen ist verspielt.'),
        ('It happens. Go on.', 'T', 'Das kommt vor. Weiter.'),
        ('A slip is forfeit, but we forgive it.', 'Ve', "Versehen ist verspielt, wir verzeih'ns."))
    add('luck-yours', 'luck talk: the player is lucky',
        ('Luck beats good play.', 'Wa', 'Glück ist besser als gut Spielen.', 'Glück 360'),
        ('The luck is on your side tonight.', 'T', "Heut' Abend ist das Glück auf Ihrer Seite."),
        ('Good cards get used to you.', 'Wa', 'An einen geschickten Spieler gewöhnen sich gute Karten.', 'Spieler 3'),
        ('Such a fluke!', 'Ra', 'Na, so einen Dusel!'),
        ('Each is the smith of his own luck.', 'Ul', 'Jeder ist seines eigenen Glückes Schmied.'),
        ('The cards fall your way; I wish you joy.', 'T', 'Die Karten fallen für Sie; viel Freude damit.'))
    add('luck-mine', 'luck talk: the opponent is lucky',
        ('Pure luck, I fear.', 'T', "Reines Glück, fürcht' ich."),
        ('Blind chance, nothing more.', 'Ha', 'Blinder Zufall, sonst nichts.'),
        ('Lucky today; tomorrow, who knows?', 'T', "Heut' hab' ich Glück; morgen, wer weiß?"),
        ('Luck at cards is April weather.', 'Wa', 'Glück im Spiel ist Wetter im April.', 'Glück'),
        ('Such a fluke! I admit it.', 'Ra', "Na, so einen Dusel! Ich gesteh's."))
    add('luck-bad-mine', "luck talk: the opponent's bad luck",
        ('The game hates me today.', 'Wa', 'Das Spiel hasst mich.', 'Spiel 274, Zusätze'),
        ('I play as well, with worse luck.', 'Cr', 'Ich spiele es gerade so gut, nur habe ich mehr Pech.'),
        ('The leaf has turned against me.', 'Wa', 'Das Blatt hat sich gewandt.', 'Blatt 29'),
        ('Everyone plays against me, and with trumps!', 'Ra', 'Alles spielt gegen mich und trischakt mich mit Tarock?'),
        ('Lost my calm, and my Pagat with it.', 'Mu', "Verloren ist die Seelenruh', der Pagat ultimo dazu."))
    add('kibitz', 'the kibitzer (the tutor, § 13) speaks or is asked',
        ('Paragraph thirteen: kibitzer, be quiet!', 'Ki', "§ 13: Kiebitz, halt's Maul!"),
        ('This game loves no audience.', 'Wa', 'Spiel liebt keine Zuschauer.', 'Spiel'),
        ('Advice from the gallery? I beg your pardon.', 'Ne4', 'Da muß ich bitten!'),
        ('Let the cards be seen and you lose.', 'Wa', 'Wer sich in die Karten sehen lässt, verliert das Spiel.', 'Karte 36'),
        ('A kibitzer may pay for supper, not advise.', 'Mk', 'Ein Kiebitz darf höchstens ein Nachtmahl zahlen.'))
    add('kibitz-mute', 'the kibitzer is muted',
        ('Paragraph thirteen it is.', 'Ki', 'Paragraph 13.'),
        ('Silence, then. The table is ours.', 'T', 'Stille also. Der Tisch gehört uns.'),
        ("Kibitzer, shut your trap: quite so.", 'Ki', "Kiebitz, halt's Maul!"),
        ('Silent kibitzers are tolerated.', 'Wb', 'Schweigende Kiebitze werden geduldet.'))
    return out

# German must not reach the English talk. The terms below stay German in English
# by design: the game's own names (docs/RULES.md, strings.json keep them in the
# English text) and its Viennese table terms. Anything else German in an English
# line is a bug; a period interjection is translated (the German line keeps it).
ENGLISH_ALLOW = {"tarock", "tarocks", "touren", "tour", "strohmandeln", "strohmandl", "sküs", "mond", "mondfang",
                 "pagat", "troule", "valat", "matsch", "tapper", "kreuzer", "ultimo", "ultimos", "conto", "contos"}
# What German looks like when it is not a game term: any umlaut or ß, and these
# common words (none of them an English word).
GERMAN_WORDS = set("""weiter gut gute guten guter nacht abend gewonnen besser mitgenommen drumgekommen glück ist als
    und der die das nicht ich sie ein eine mit für auf ja nein schon noch mal partie spiel herr herrn aber wie wir es zu
    den dem im bei auch dann doch na also stich karte bitte danke servus gnä frau prost mein jetzt nun halt's maul
    kiebitz spielen tous les trois wenn sehr wohl bleibt beide diesmal keiner riskiert geht's""".split())


def german_in(text):
    """The German tokens of an English line, game terms (ENGLISH_ALLOW) excepted."""
    import re
    out = []
    for tok in re.findall(r"[\wÄÖÜäöüß']+", text):
        low = tok.lower().strip("'")
        if low in ENGLISH_ALLOW:
            continue
        if re.search("[äöüßÄÖÜ]", tok) or low in GERMAN_WORDS or tok.lower() in GERMAN_WORDS:
            out.append(tok)
    return out


def sig(gid, k, w):
    """The saying a wording is, whatever group says it (docs/TABLE3D.md 5.3.1): a
    table line is its own key; a sourced one is its source with the locator when
    that pins an entry (it has a number: "Karte 7"), else with its German wording
    (a bare lemma such as "Spiel" holds many sayings)."""
    if w[1] == "T":
        return f"{gid}.{k}"
    return f"{w[1]} {w[3]}" if len(w) > 3 and any(c.isdigit() for c in w[3]) else f"{w[1]} {w[2]}"


def texts():
    """Every wording: key -> {en, de, sig}."""
    return {f"{gid}.{k}": {"en": w[0], "de": w[2], "sig": sig(gid, k, w)}
            for gid, _, ways in phrase_groups() for k, w in enumerate(ways)}


def groups():
    """Every group the page can ask for: id -> the wordings it picks among."""
    return {gid: [f"{gid}.{k}" for k in range(len(ways))] for gid, _, ways in phrase_groups()}


def words_json():
    """web3d/words.json: the build embeds it."""
    return json.dumps({"groups": groups(), "texts": texts()},
                      ensure_ascii=False, separators=(",", ":"), sort_keys=True) + "\n"


def document():
    """docs/PHRASES.md: every group, its lines and their sources."""
    lines = [
        "# What the table says",
        "",
        "The opponent's phrase bank, by game moment (events: `docs/RULES.md` section 16). The voice is",
        "set around 1891; sources run from 1756 to 1938, none later. Each line is given in English and German;",
        "the German is the period wording where a source gives one (trimmed or inflected to speak), else",
        "the table's own. \"T\" marks the table's own lines. Generated from `tools/phrases.py`; edit the bank,",
        "not this file.",
        "",
        "Sources:",
        "",
    ]
    for key, (year, ids, text, _) in SOURCES.items():
        lines.append(f"- **{key}**{' ' + ids if ids else ''}: {text}")
    for gid, moment, ways in phrase_groups():
        lines += ["", f"### `{gid}`", "", f"{moment}", "", "| English | German | Source |", "|---|---|---|"]
        for w in ways:
            text, src, de = w[0], w[1], w[2]
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
