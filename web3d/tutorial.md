<!--
Die Seiten des Kiebitzes (docs/TABLE3D.md 6.2), deutsch. Schlüssel nach Stelle,
wie in der Tabelle 6.2.2: packets, bidding, the-pull, playing, following,
from-a-packet, honours, thirds, pagat-ultimo, scoring, draw, touren-slate,
extension. Parallel zu tutorial.en.md. Jede Seite enthält, was ihre erste
Entscheidung braucht, nicht mehr (docs/review/tutorial-pacing.md).
Klauseln (docs/RULES.md): packets 2.3 2.4 4.4 4.8 5.2 13.1; bidding 3.1-3.4 8.2
8.3 8.4 9.1 (Kartenwerte 1.7); the-pull 4.3 4.5 4.6 4.7; playing 5.1-5.3 5.5;
following 5.2 5.4; from-a-packet 6.1 6.2; honours 11.1-11.5; thirds 8.1 8.2;
pagat-ultimo 1.4 12.1-12.4; scoring 9.2 9.3 11 12 15.5 15.7; draw 8.4 9.2 11.7
14.1-14.4; touren-slate 15.2 15.3 15.5 (Rekonstruktion: 15 Einleitung, TABLE3D
4.6); extension 15.6. Ausdrücke der Zeit: 03-S12 (Werner), 03-S14 (Ulmann), 05c
headline 1 (Kiebitz). Stimme: TABLE3D 6.5. Text vor der ersten Überschrift
überliest der Parser. Mit einer if-Zeile auf eine Einstellung gekennzeichnete
Blöcke gelten nur für diese (tutor.js, tutorialPages): bidding und draw haben je
einen für beide both_pass (RULES 3.4, 14.6).
-->

# Die Pakete

## Was auf dem Tisch liegt

- Sie halten 15 Karten und haben verdeckt vor sich **drei Pakete** zu vier Karten. Ihr Gegner hat dasselbe.
- Die verdeckten Karten kennt niemand, auch ihr Besitzer nicht.
- Gleich wird die oberste Karte jedes Pakets aufgedeckt. Eine Karte, die offen liegen bleibt, heißt **Deckkarte**.
- Eine Deckkarte dürfen Sie ausspielen wie eine Karte aus der Hand.
- Die kleine Zahl am Paket zählt die Karten, die darunter noch verdeckt liegen.
- Wenn ich mir erlauben darf: Sehen Sie auch die Deckkarten Ihres Gegners an. Sie sind kein Geheimnis.

# Das Aufnehmen

## Aufnehmen oder weiter

- Zuerst spricht, wer nicht gegeben hat.
- **Ich nehme auf** heißt: Sie spielen auf Gewinn. Sie sind der Aufnehmer.
- **Weiter** heißt: Sie lassen es sein.
- Die 70 Points stecken in den Karten: Könige, Sküs, Mond und Pagat 5; Damen 4; Cavalle 3; Buben 2; jede andere Karte 1.
- Der Aufnehmer braucht 36 der 70.
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
- Nichts zu entscheiden: das Aufdecken geschieht für Sie.

# Eine Karte spielen

## Sie sind am Zug

- Wer ausspielt, legt eine Karte, der andere antwortet. Der Gewinner nimmt den Stich und spielt als Nächster aus.
- Ein Tarock sticht jede Farbkarte; das höhere Tarock gewinnt. Sonst gewinnt die höhere Karte der ausgespielten Farbe.
- Tippen Sie eine helle Karte an, dann nochmals oder drücken Sie **Ausspielen**.
- Helle Karten sind erlaubt, abgedunkelte nicht. Tippen Sie auf eine abgedunkelte, und ich sage Ihnen, warum.

# Bedienen

## Welche Karte darf ich spielen?

- Haben Sie die Farbe? Dann müssen Sie bedienen. Sonst müssen Sie stechen, mit einem Tarock. Haben Sie auch keines, dürfen Sie irgendeine Karte spielen.
- Deckkarten gelten hier wie Handkarten.
- Sie müssen nicht höher spielen.

# Aus einem Paket spielen

## Eine Deckkarte spielen

- Ist der Stich eingezogen, wird die Karte darunter aufgedeckt. Ein Tarock oder König kommt in Ihre Hand, eine niedrige Karte wird die neue Deckkarte.
- Die Karte darunter kennen weder Sie noch Ihr Gegner, bis sie aufgedeckt wird.

# Troule und vier Könige

## Honneurs zu einem Point

- Die **Troule** sind der Sküs, der Mond (XXI) und der Pagat (I). Alle drei zugleich in der Hand: ein Point.
- Die **vier Könige** zugleich in der Hand: ein Point.
- Aus den Paketen gezogene Karten zählen. Deckkarten nicht.
- Sagen Sie es an, ehe die erste dieser Karten gespielt wird.
- Drücken Sie **Troule ansagen** oder **Vier Könige ansagen**, oder antworten Sie auf die Frage beim Wählen der Karte.
- Angesagt wird es bezahlt, auch wenn die Karten später verloren gehen. Verschwiegen wird es nicht bezahlt.

# Das Zählen in Dritteln

## Die Zählung

- Jede Karte zählt ihren Wert weniger zwei Drittel. Bei drei Karten zählen Sie die Summe weniger 2.
- Der Tisch zeigt **35⅔** als genug für 36; **35⅓** reicht nicht.
- Der Stand oben läuft mit, während Sie Stiche machen.

# Pagat ultimo

## Der letzte Stich

- Der Pagat (I) ist das niedrigste Tarock.
- Spielen Sie ihn im letzten, dem 27. Stich aus und gewinnen: ein Point von Ihrem Gegner. Verlieren Sie: Sie zahlen Ihrem Gegner einen Point.
- Er wird nie angesagt; er wird einfach bezahlt, wenn es so kommt, zusätzlich zur Partie und zu den Honneurs.
- Wenn ich mir erlauben darf: Ein Point, der an der letzten Karte hängt. Schonen Sie Ihre Nerven.

# Was eine Partie zahlt

## Die Abrechnung

- Die Partie wird zwischen Ihnen beiden bezahlt: der Aufnehmer gewinnt 3 oder zahlt 4; eine einfache Partie zahlt ihrem Sieger 2.
- Jeder Honneur und der Pagat zahlen zusätzlich einen Point.
- Jede Zahlung kommt mit ihrem Grund auf die **Tafel**. Tippen Sie auf **Tafel**, um sie zu sehen.
- Am Ende des Abends wird der Unterschied zu 2 Kreuzer der Point beglichen.

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

# Touren

## Die Rechnung des Abends

- Eine **Tour** sind zwei Partien, eine von jedem von Ihnen gegeben. Die Tafel zählt jede Tour zusammen.
- So wurde ein Tarockabend geführt. Keine Quelle sagt, dass man das Strohmandl so führte: wir haben es geborgt.

# Noch eine Tour

## Weiterspielen

- Sind die Touren gespielt, können Sie weitere anhängen. Beide müssen zustimmen.
- Drücken Sie **Noch eine Tour** oder **Genug**.
- Es kommen 3 oder 6 Touren dazu, wie Sie anfangs gewählt haben. Die Rechnung läuft weiter.
