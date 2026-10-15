<!--
Die Seiten des Kiebitzes (docs/TABLE3D.md 6.2), deutsch. Schlüssel nach Stelle,
wie in der Tabelle 6.2.2: packets, bidding, the-pull, following, from-a-packet,
honours, pagat-ultimo, thirds, draw, touren-slate, extension. Parallel zu
tutorial.en.md. Klauseln (docs/RULES.md): packets 2.3 2.4 4.4 4.8 5.2 13.1;
bidding 3.1-3.4 8.3 8.4 9.1; the-pull 4.3 4.5 4.6 4.7; following 5.2 5.4 5.5;
from-a-packet 5.2 6.1 6.2; honours 11.1-11.5; pagat-ultimo 1.4 12.1-12.4;
thirds 1.7 8.1 8.2; draw 8.4 9.2 11.7 14.1-14.4; touren-slate 15.2 15.3 15.5 15.8
15.9 (Rekonstruktion: 15 Einleitung, TABLE3D 4.6); extension 15.6. Ausdrücke der
Zeit: 03-S12 (Werner), 03-S14 (Ulmann), 05c headline 1 (Kiebitz). Stimme:
TABLE3D 6.5. Text vor der ersten Überschrift überliest der Parser. Mit einer
if-Zeile auf eine Einstellung gekennzeichnete Blöcke gelten nur für diese
(tutor.js, tutorialPages): bidding und draw haben je einen für beide both_pass (RULES 3.4, 14.6).
-->

# Die Pakete

## Was auf dem Tisch liegt

- Sie halten 15 Karten. Vor Ihnen liegen **drei Pakete** zu vier Karten, verdeckt.
- Ihr Gegner hat dasselbe. Die verdeckten Karten kennt niemand, auch ihr Besitzer nicht.
- Gleich wird die oberste Karte jedes Pakets aufgedeckt. Eine Karte, die offen auf dem Paket liegen bleibt, heißt **Deckkarte**.
- Eine Deckkarte dürfen Sie ausspielen wie eine Karte aus der Hand.
- Die kleine Zahl am Paket zählt die Karten, die darunter noch verdeckt liegen.
- Wenn ich mir erlauben darf: Sehen Sie auch die Deckkarten Ihres Gegners an. Sie sind kein Geheimnis.

# Das Aufnehmen

## Aufnehmen oder weiter

- Zuerst spricht, wer nicht gegeben hat. Dann der Geber.
- **Ich nehme auf** heißt: Sie spielen auf Gewinn. Sie sind der Aufnehmer.
- **Weiter** heißt: Sie lassen es sein.
- Der Aufnehmer braucht 36 der 70 Points. Wer sie verfehlt, verliert.
- Mit den Standardwerten gewinnt der Aufnehmer 3 Points oder verliert 4.
<!-- if both_pass=play -->
- Sagen beide weiter, wird eine einfache Partie gespielt. Der Sieger erhält 2 Points.
<!-- end -->
<!-- if both_pass=throw_in -->
- Sagen beide weiter, wird eingeworfen: nichts wird gezählt, und kein Paket wird aufgedeckt.
<!-- end -->
- Wenn ich mir erlauben darf: Was gesagt ist, gilt. Überbieten kann niemand.

# Das Aufdecken

## Vor der ersten Karte

- Die Pakete werden der Reihe nach aufgedeckt, von links nach rechts. Tippen Sie, um fortzufahren.
- Ein **Tarock** (auch der Sküs) oder ein **König** wird gezeigt und kommt in die Hand. Dann wird die nächste Karte aufgedeckt.
- Jede andere Karte bleibt als Deckkarte offen liegen. Damit ist das Paket fertig.
- Die letzte Karte eines Pakets kommt immer in die Hand, was sie auch sei.
- Das Aufdecken geschieht für Sie; Sie haben nichts zu entscheiden.

# Bedienen

## Welche Karte darf ich spielen?

- Wer ausspielt, legt eine Karte. Sie antworten mit einer.
- Haben Sie die Farbe? Dann müssen Sie bedienen. Sonst müssen Sie stechen, mit einem Tarock. Haben Sie auch keines, dürfen Sie irgendeine Karte spielen.
- Handkarten und Deckkarten gelten gleich. Sind beide hell, wählen Sie frei.
- Helle Karten sind erlaubt, abgedunkelte nicht. Tippen Sie auf eine abgedunkelte, und ich sage Ihnen, warum.
- Sie müssen nicht höher spielen. Ein Tarock sticht jede Farbkarte; das höhere Tarock gewinnt. Sonst gewinnt die höhere Karte der ausgespielten Farbe.

# Aus einem Paket spielen

## Eine Deckkarte spielen

- Tippen Sie eine Deckkarte an wie eine Handkarte. Tippen Sie sie nochmals an, um sie auszuspielen.
- Ist der Stich eingezogen, wird die Karte darunter aufgedeckt. Ein Tarock oder König kommt in Ihre Hand, eine niedrige Karte wird die neue Deckkarte.
- Die Karte darunter kennen weder Sie noch Ihr Gegner, bis sie aufgedeckt wird.
- Ist die Karte darunter die letzte des Pakets, kommt sie in Ihre Hand, was sie auch sei.
- Wenn ich mir erlauben darf: Eine gespielte Deckkarte deckt eine neue auf. Mehr ist es nicht.

# Troule und vier Könige

## Honneurs zu einem Point

- Die **Troule** sind der Sküs, der Mond (XXI) und der Pagat (I). Alle drei zugleich in der Hand: ein Point.
- Die **vier Könige** zugleich in der Hand: ein Point.
- Aus den Paketen gezogene Karten zählen. Deckkarten nicht.
- Sagen Sie es an, ehe die erste dieser Karten gespielt wird.
- Drücken Sie **Troule ansagen** oder **Vier Könige ansagen**, oder antworten Sie auf die Frage beim Wählen der Karte.
- Angesagt wird es bezahlt, auch wenn die Karten später verloren gehen. Verschwiegen wird es nicht bezahlt.
- Sie dürfen schweigen. Die Wahl liegt bei Ihnen.

# Pagat ultimo

## Der letzte Stich

- Der Pagat (I) ist das niedrigste Tarock.
- Spielen Sie ihn im letzten, dem 27. Stich aus und gewinnen: ein Point von Ihrem Gegner.
- Spielen Sie ihn dort aus und verlieren: Sie zahlen Ihrem Gegner einen Point.
- Er wird nie angesagt; er wird einfach bezahlt, wenn es so kommt.
- Der Pagat wird zusätzlich zur Partie und zu den Honneurs bezahlt.
- Wenn ich mir erlauben darf: Ein Point, der an der letzten Karte hängt. Schonen Sie Ihre Nerven.

# Das Zählen in Dritteln

## Die Zählung

- Könige, Sküs, Mond und Pagat zählen 5. Damen 4, Cavalle 3, Buben 2. Jede andere Karte 1.
- Jede Karte zählt ihren Wert weniger zwei Drittel. Bei drei Karten zählen Sie die Summe weniger 2.
- 36 Points gewinnen. Der Tisch zeigt **35⅔** als genug; **35⅓** reicht nicht.
- Der Stand oben läuft mit, während die Zählung ausgelegt wird.
- Alle 70 Points werden zwischen Ihnen und Ihrem Gegner geteilt.

# Das Unentschieden

## Wenn keiner 36 erreicht

<!-- if both_pass=play -->
- In einer einfachen Partie ist das Spiel unentschieden, wenn keiner von Ihnen 35⅔ erreicht. Niemand gewinnt es.
<!-- end -->
<!-- if both_pass=throw_in -->
- Beim Einwerfen wird nur eine angesagte Partie gespielt, und die hat immer einen Sieger. Unentschieden gibt es nicht.
<!-- end -->
- Honneurs und der Pagat werden trotzdem bezahlt.
<!-- if both_pass=play -->
- Die nächste gespielte Partie zählt doppelt, mit allem darin. Eine eingeworfene Partie verbraucht es nicht.
- Der Tisch zeigt **×2**, wenn es gilt.
<!-- end -->

# Touren und die Tafel

## Die Rechnung des Abends

- Eine **Tour** sind zwei Partien, eine von jedem von Ihnen gegeben. Sie wählen 3 Touren oder 6.
- Die **Tafel** führt die Rechnung. Tippen Sie auf **Tafel**, um jede Zahlung mit ihrem Grund zu sehen.
- Am Ende des Abends wird der Unterschied zu 2 Kreuzer der Point beglichen.
- So wurde ein Tarockabend geführt. Keine Quelle sagt, dass man das Strohmandl so führte: wir haben es geborgt.

# Noch eine Tour

## Weiterspielen

- Sind die Touren gespielt, können Sie weitere anhängen. Beide müssen zustimmen.
- Drücken Sie **Noch eine Tour** oder **Genug**.
- Es kommen 3 oder 6 Touren dazu, wie Sie anfangs gewählt haben. Die Rechnung läuft weiter.
- Hat Ihr Gegner genug, ist der Abend aus.
