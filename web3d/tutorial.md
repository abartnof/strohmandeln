<!--
Die Seiten des Kiebitzes (docs/TABLE3D.md 6.2), deutsch. Schlüssel nach Stelle,
wie in der Tabelle 6.2.2: the-cards, packets, bidding, the-pull, playing, following,
from-a-packet, honours, thirds, pagat-ultimo, scoring, draw, touren-slate,
extension, scoring-full. Parallel zu tutorial.en.md. Jede Seite enthält, was
ihre erste Entscheidung braucht, nicht mehr (docs/review/tutorial-pacing.md);
scoring-full ist die eine lange Seite, nie fällig, aus der Hilfe oder über die
Verweise auf bidding (nach thirds) sowie thirds und scoring (nach ihr) zu
lesen: `[Text](#key)`, TABLE3D 6.2.4.
Quellen: the-cards 15-S1 (Dummett, Kap. 4 und S. 100: zum Spielen gemacht, 1442; keine
Kartomantie vor 1781), 15-S6 (Court de Gébelin 1781), 15-S7 und 15-S8 (Tarock und
Tarot im Deutschen), 15-S9 und 15-S10 (die österreichischen Spiele; die Trümpfe),
15-S11 (Piatnik verkauft Tarockblätter, geprüft am 15. Okt. 2026; die Seite nennt weder Wien noch das Muster), 15-S12 und 15-S14 (der Cavall, 3 Augen
wert), 08-S9 und 08-S10 (ein echtes Blatt hat keine Indizes auf den Farbkarten),
alle in research/15-the-pack-today.md.
Klauseln (docs/RULES.md): the-cards 1.2 1.7; packets 2.3 2.4 4.4 4.8 5.2 13.1; bidding 3.1-3.4 8.1
8.2 8.4 9.1 (Kartenwerte 1.7); the-pull 4.3 4.5 4.6 4.7; playing 5.1-5.3 5.5;
following 5.2 5.4; from-a-packet 6.1 6.2; honours 11.1-11.5; thirds 1.7 8.1 8.2;
pagat-ultimo 1.4 12.1-12.4; scoring 9.1-9.3 11 12.3 15.5 15.8; draw 8.4 9.2 11.7
14.1-14.4; touren-slate 15.2 15.3 15.5 (Rekonstruktion: 15 Einleitung, TABLE3D
4.6); extension 15.6; scoring-full 1.3 1.7 8.1-8.5 9.1-9.3 10.1 10.2 11.1-11.7
12.1-12.4 14.1-14.5 15.2 15.3 15.5 15.6 15.8 15.9 (das durchgerechnete Beispiel
halten Kerntests, web3d/test/scoring_pages.test.js). Ausdrücke der Zeit: 03-S12
(Werner), 03-S14 (Ulmann), 05c headline 1 (Kiebitz). Stimme: TABLE3D 6.5. Text
vor der ersten Überschrift überliest der Parser. Mit einer if-Zeile auf eine
oder mehrere Einstellungen gekennzeichnete Blöcke gelten nur für diese
(tutor.js, tutorialPages): die Seiten, die einen Spielwert, das Ultimo oder das
Weiterpassen nennen, haben je einen Block für jeden Wert von `values`, `ultimo`
und `both_pass` (RULES 3.4, 9.1, 12.3, 14.6).
-->

# Die Karten

## Die Tarocke

- Die Tarocke sind schlicht Trümpfe: I bis XXI, und der Sküs über allen.
- Tarockkarten wurden zum Spielen gemacht, in Italien, spätestens 1442. Vom Wahrsagen mit ihnen ist erst ab 1781 die Rede, in Frankreich.
- Hier wird kein Schicksal gedeutet. In Österreich spielt man noch immer Tarock, und Piatnik verkauft heute noch Tarockblätter.
- Im Deutschen heißt das Spiel Tarock und das Blatt zum Wahrsagen Tarot. Das Englische hat für beides ein Wort, daher die Verwechslung.

## Das C

- Das C ist der Cavall, ein Reiter zu Pferd, 3 Augen wert. Tarockblätter haben ihn, das französische Blatt zu 52 Karten nicht.
- Auf einem echten Blatt tragen die Hofkarten Bilder statt Buchstaben. Unsere großen Buchstaben und Eckzahlen sind für den raschen Blick.

# Die Pakete

## Was auf dem Tisch liegt

- Sie halten 15 Karten und haben verdeckt vor sich **drei Pakete** zu vier Karten. Ihr Gegner hat dasselbe.
- Die verdeckten Karten kennt niemand, auch ihr Besitzer nicht.
- Gleich wird die oberste Karte jedes Pakets aufgedeckt. Eine Karte, die offen liegen bleibt, heißt **Decke**.
- Eine Decke dürfen Sie ausspielen wie eine Karte aus der Hand.
- Die kleine Zahl am Paket zählt die Karten, die darunter noch verdeckt liegen.
- Wenn ich mir erlauben darf: Sehen Sie auch die Decken Ihres Gegners an. Sie sind kein Geheimnis.

# Das Aufnehmen

## Aufnehmen oder weiter

- Zuerst spricht, wer nicht gegeben hat.
- **Ich nehme auf** heißt: Sie spielen auf Gewinn. Sie sind der Aufnehmer.
- **Weiter** heißt: Sie lassen es sein.
- Die Karten zählen: Könige (K), Sküs, Mond und Pagat 5; Damen (D) 4; Cavalle (C) 3; Buben (B) 2; jede andere Karte 1.
- In Dreiergruppen gezählt, machen die 54 Karten 70 Augen. Der Aufnehmer braucht 36 davon.
- [Kartenwerte und Zählung](#thirds)
<!-- if values=2/3/4 -->
- Der Aufnehmer, der gewinnt, erhält 3 Points; wer verliert, zahlt 4.
<!-- end -->
<!-- if values=1/2/3 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 3.
<!-- end -->
<!-- if values=1/2/4 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 4.
<!-- end -->
<!-- if both_pass=play -->
- Sagen beide weiter, wird ein einfaches Spiel gespielt.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 2.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1.
<!-- end -->
<!-- if both_pass=throw_in -->
- Sagen beide weiter, wird eingeworfen: nichts wird gezählt, und kein Paket wird aufgedeckt.
<!-- end -->
- Wenn ich mir erlauben darf: Was gesagt ist, gilt. Überbieten kann niemand.

# Das Aufdecken

## Vor der ersten Karte

- Die Pakete werden der Reihe nach aufgedeckt, von links nach rechts. Tippen Sie, um fortzufahren.
- Ein **Tarock** (auch der Sküs) oder ein **König** wird gezeigt und kommt in die Hand. Dann wird die nächste Karte aufgedeckt.
- Jede andere Karte bleibt als Decke offen liegen. Damit ist das Paket fertig.
- Die letzte Karte eines Pakets kommt immer in die Hand, was sie auch sei.
- Nichts zu entscheiden: das Aufdecken geschieht für Sie.

# Eine Karte spielen

## Sie sind am Zug

- Wer ausspielt, legt eine Karte, der andere antwortet. Der Gewinner nimmt den Stich und spielt als Nächster aus.
- Ein Tarock sticht jede Farbkarte; das höhere Tarock gewinnt. Sonst gewinnt die höhere Karte der ausgespielten Farbe.
- Tippen Sie eine helle Karte an, dann nochmals, um sie auszuspielen.
- Helle Karten sind erlaubt, abgedunkelte nicht. Tippen Sie auf eine abgedunkelte, und ich sage Ihnen, warum.

# Bedienen

## Welche Karte darf ich spielen?

- Haben Sie die Farbe? Dann müssen Sie bedienen. Sonst müssen Sie stechen, mit einem Tarock. Haben Sie auch keines, dürfen Sie irgendeine Karte spielen.
- Decken gelten hier wie Handkarten.
- Sie müssen nicht höher spielen.

# Aus einem Paket spielen

## Eine Decke spielen

- Ist der Stich eingezogen, wird die Karte darunter aufgedeckt. Ein Tarock oder König kommt in Ihre Hand, eine niedrige Karte wird die neue Decke.

# Troule und vier Könige

## Honneurs zu einem Point

- Die **Troule** sind der Sküs, der Mond (XXI) und der Pagat (I). Alle drei zugleich in der Hand: ein Point.
- Die **vier Könige** zugleich in der Hand: ein Point.
- Aus den Paketen gezogene Karten zählen. Decken nicht.
- Sagen Sie es an, ehe die erste dieser Karten gespielt wird.
- Drücken Sie **Troule ansagen** oder **Vier Könige ansagen**, oder antworten Sie auf die Frage beim Wählen der Karte.
- Angesagt wird es bezahlt, auch wenn die Karten später verloren gehen. Verschwiegen wird es nicht bezahlt.

# Kartenwerte und Zählung

## Was jede Karte wert ist

- **5** je Karte: die vier Könige (K), der Sküs, der Mond (XXI) und der Pagat (I).
- **4** je Karte: die vier Damen (D).
- **3** je Karte: die vier Cavalle (C).
- **2** je Karte: die vier Buben (B).
- **1** je Karte: jede andere Karte, auch die Tarocks II bis XX.

## So wird gezählt

- Jede Karte zählt ihren Wert weniger zwei Drittel. Bei drei Karten zählen Sie die Summe weniger 2: dieselbe Regel, für drei zugleich.
- Beispiel: ein König, ein Bube und eine Sieben sind 5 + 2 + 1 = 8, weniger 2 = 6.
- Bleiben eine oder zwei Karten übrig, verliert jede zwei Drittel.
- Ihre gewonnenen Karten und die Ihres Gegners machen zusammen 70 Augen. Der Aufnehmer braucht 36: **35⅔** genügt, **35⅓** nicht.
- Der Stand oben läuft mit, während Sie Stiche machen.
- [Mehr zur Abrechnung](#scoring-full)

# Pagat ultimo

## Der letzte Stich

- Der Pagat (I) ist das niedrigste Tarock.
- Spielen Sie ihn im letzten, dem 27. Stich aus und gewinnen: Ihr Gegner zahlt Ihnen das Ultimo, mit den Standardwerten einen Point.
- Verlieren Sie dabei: Sie zahlen das Ultimo.
- Er wird nie angesagt; er wird einfach bezahlt, wenn es so kommt, zusätzlich zur Partie und zu den Honneurs.
- Wenn ich mir erlauben darf: Ein Point, der an der letzten Karte hängt. Schonen Sie Ihre Nerven.

# Was eine Partie zahlt

## Die Abrechnung

- Ein **Point** ist, was bezahlt wird. Die 36 Augen der Zählung entscheiden nur, wer das Spiel gewinnt.
<!-- if values=2/3/4 -->
- Der Aufnehmer, der gewinnt, erhält 3 Points; wer verliert, zahlt 4.
- Mit angesagter Troule dazu zahlt ein Gewinn 3 + 1 = 4.
<!-- end -->
<!-- if values=1/2/3 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 3.
- Mit angesagter Troule dazu zahlt ein Gewinn 2 + 1 = 3.
<!-- end -->
<!-- if values=1/2/4 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 4.
- Mit angesagter Troule dazu zahlt ein Gewinn 2 + 1 = 3.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 2.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1.
<!-- end -->
- Jeder Honneur, den Sie ansagen, zahlt zusätzlich 1 Point.
<!-- if ultimo=1 -->
- Ein Pagat ultimo ist, gewonnen wie verloren, 1 Point wert.
<!-- end -->
<!-- if ultimo=2 -->
- Ein Pagat ultimo ist, gewonnen wie verloren, 2 Points wert.
<!-- end -->
- Jede Zahlung kommt mit ihrem Grund auf die **Tafel**. Tippen Sie auf **Tafel**, um sie zu sehen.
- Am Ende des Abends wird der Unterschied zu 2 Kreuzer der Point beglichen.
- [Mehr zur Abrechnung](#scoring-full)

# Das Unentschieden

## Wenn keiner 36 erreicht

<!-- if both_pass=play -->
- In einem einfachen Spiel ist die Partie unentschieden, wenn keiner von Ihnen 35⅔ erreicht. Niemand gewinnt das Spiel.
- Honneurs und der Pagat werden trotzdem bezahlt.
- Die nächste gespielte Partie zählt doppelt, mit allem darin. Eine Partie, die mangels Tarock neu gegeben wird, verbraucht es nicht.
- Der Tisch zeigt **×2**, wenn es gilt.
<!-- end -->
<!-- if both_pass=throw_in -->
- Beim Einwerfen wird nur eine angesagte Partie gespielt, und die hat immer einen Sieger. Unentschieden gibt es nicht.
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

# Die Abrechnung im Ganzen

## Was jede Karte wert ist

- **5** je Karte: die vier Könige (K), der Sküs, der Mond (XXI) und der Pagat (I). Sieben Karten.
- **4** je Karte: die vier Damen (D).
- **3** je Karte: die vier Cavalle (C).
- **2** je Karte: die vier Buben (B).
- **1** je Karte: jede andere Karte, also die Tarocks II bis XX und die Zahlkarten der Farben.

- Die Karten zeigen die Buchstaben K, D, C und B, die Tarocks in römischen Ziffern. Der Sküs ist das höchste Tarock, der Mond ist XXI, der Pagat ist I.
- Alle 54 Karten mit diesen Werten zusammen ergeben 106. Die Zählung in Dreiergruppen macht daraus 70.

## Zählen in Dreiergruppen

- Die gewonnenen Karten eines Spielers werden in Dreiergruppen gezählt. Zählen Sie die drei Werte zusammen und ziehen Sie 2 ab.
- Ein König, ein Bube und eine Sieben sind 5 + 2 + 1 = 8, weniger 2 = 6. Drei niedrige Karten sind 1 + 1 + 1 = 3, weniger 2 = 1.
- Bleiben eine oder zwei Karten übrig, verliert jede zwei Drittel. Ein letzter König und Bube sind 5 + 2 = 7, weniger 1⅓ = 5⅔.
- 2 von einer Dreiergruppe abzuziehen heißt, von jeder Karte zwei Drittel abzuziehen. So zählt jede Karte ihren Wert weniger zwei Drittel: ein König 4⅓, eine Dame 3⅓, ein Cavall 2⅓, ein Bube 1⅓, jede andere Karte ⅓. Darum ist hier von Dritteln die Rede.
- Das ist die überlieferte Zählung in Dreiergruppen, genau geführt: nichts wird gerundet.
- Das Spiel hat 54 Karten, also 18 Dreiergruppen. Die Werte ergeben 106, und die 18 Gruppen verlieren je 2: 106 − 36 = 70.

## 36 von 70

- Die gewonnenen Karten beider Spieler ergeben immer zusammen 70 Augen, und 36 ist mehr als die Hälfte von 70.
- Die Zählung behält Drittel, also kann ein Spieler 35⅔ zählen. Auf die nächste ganze Zahl gerundet ist das 36; 35⅓ wären 35. Die Regel: 35⅔ gewinnt, 35⅓ nicht.
- Hat ein Spieler 35⅔, hat der andere 34⅓.
- In einem angesagten Spiel braucht der Aufnehmer 35⅔ oder mehr. Mit weniger verliert er, auch mit 35⅓. Unentschieden gibt es nicht.
<!-- if both_pass=play -->
- In einem einfachen Spiel gewinnt, wer 35⅔ oder mehr hat. Hat keiner so viel, ist es unentschieden: 35 : 35 oder 35⅓ : 34⅔.
<!-- end -->

## Was der Stand oben zeigt

- Solange Sie spielen, nennt die Zeile oben die Tour, die Partie und den Stich, etwa Stich 14 / 27.
- Der Stand läuft mit, während Sie Stiche machen. Nach jedem Stich zeigt jede Linie die bisher gewonnenen Karten, so gezählt, wie die Zählung sie zählen wird.
- Die Zahl neben der Stand-Linie ist der laufende Stand in genauen Dritteln. Die Linie füllt je einen Abschnitt für jedes ganze Auge, bis 36.
- Nach dem 27. Stich beginnt die Zählung von vorn. Die gewonnenen Karten jedes Spielers werden aufgedeckt, drei auf einmal, und über jeder Gruppe steigt eine Zahl auf: ihr Wert, etwa +6 für König, Bube und Sieben.
- Sie endet bei denselben Zahlen wie der laufende Stand. Wer 35⅔ erreicht, lässt das Ziel leuchten; wer auf 35⅓ endet, bekommt „35⅓ ist nicht 36“.
- Die Honneurs und das Ultimo kommen nach der Zählung, in Points. Sie haben eine eigene Zeile, „Honneurs und Ultimo“, und werden nicht zu den 36 gerechnet.
- Der Pfeil neben dem Stand öffnet die Liste je Partie: die Zählung, dazu die Honneurs und das Ultimo. Eine Anzeige nennt dann das Ergebnis und die bezahlten Points.

## Was das Spiel wert ist

- Die Zählung entscheidet, wer gewinnt. Das Spiel wird dann zwischen Ihnen beiden in Points bezahlt. Ein Point ist eine Zahlungseinheit, kein Auge der Zählung.
<!-- if values=2/3/4 -->
- Der Aufnehmer, der gewinnt, erhält 3 Points; wer verliert, zahlt 4. Wie hoch die Zählung ist, spielt keine Rolle: 36 Augen und 45 Augen zahlen gleich.
<!-- end -->
<!-- if values=1/2/3 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 3. Wie hoch die Zählung ist, spielt keine Rolle: 36 Augen und 45 Augen zahlen gleich.
<!-- end -->
<!-- if values=1/2/4 -->
- Der Aufnehmer, der gewinnt, erhält 2 Points; wer verliert, zahlt 4. Wie hoch die Zählung ist, spielt keine Rolle: 36 Augen und 45 Augen zahlen gleich.
<!-- end -->
<!-- if values=2/3/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 2. Ein Unentschieden zahlt für das Spiel nichts.
<!-- end -->
<!-- if values=1/2/3 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1. Ein Unentschieden zahlt für das Spiel nichts.
<!-- end -->
<!-- if values=1/2/4 both_pass=play -->
- Ein einfaches Spiel zahlt seinem Sieger 1. Ein Unentschieden zahlt für das Spiel nichts.
<!-- end -->
<!-- if both_pass=throw_in -->
- Sagen beide weiter, wird eingeworfen: nichts wird gezählt und nichts bezahlt.
<!-- end -->
- Gewinnt ein Spieler alle 27 Stiche, ist das ein Valat. Der andere Spieler zahlt 12 Points anstelle des Wertes des Spiels, und Honneurs und Ultimo werden zusätzlich bezahlt.

## Honneurs

- Es gibt zwei, jeder ist 1 Point wert: die Troule (der Sküs, der Mond und der Pagat) und die vier Könige.
- Sie müssen alle seine Karten zugleich in der Hand halten. Aus den Paketen gezogene Karten zählen; Decken nicht.
- Sagen Sie ihn an, ehe Sie die erste seiner Karten spielen. Drücken Sie **Troule ansagen** oder **Vier Könige ansagen**, oder antworten Sie beim Wählen der Karte.
- Angesagt wird er bezahlt, auch wenn Sie die Karten später verlieren, und auch in einer unentschiedenen Partie. Verschwiegen wird er nicht bezahlt.

## Der Pagat ultimo

- Ultimo ist der letzte, der 27. Stich. Spielen Sie darin den Pagat (I) aus und gewinnen: Ihr Gegner zahlt Ihnen das Ultimo. Verlieren Sie: Sie zahlen.
- Er wird nie angesagt. Er wird bezahlt, wenn es so kommt, in jeder Partie, zusätzlich zum Spiel und zu den Honneurs.
<!-- if ultimo=1 -->
- Ein Pagat ultimo ist, gewonnen wie verloren, 1 Point wert.
<!-- end -->
<!-- if ultimo=2 -->
- Ein Pagat ultimo ist, gewonnen wie verloren, 2 Points wert.
<!-- end -->

## Unentschieden und doppelt

<!-- if both_pass=play -->
- Ein einfaches Spiel, in dem kein Spieler 35⅔ erreicht, ist unentschieden. Niemand gewinnt das Spiel. Honneurs und Ultimo werden trotzdem bezahlt.
- Die nächste gespielte Partie zählt doppelt: Spiel, Honneurs und Ultimo je zweimal. Der Tisch zeigt **×2**.
- Eine Partie, die mangels Tarock neu gegeben wird, verbraucht es nicht. Mehr als doppelt wird es nie, und wenn der Abend endet, verfällt es.
<!-- end -->
<!-- if both_pass=throw_in -->
- Beim Einwerfen wird nur eine angesagte Partie gespielt, und die hat immer einen Sieger. Unentschieden gibt es nicht, also auch kein Doppelt.
<!-- end -->

## Eine Partie von Anfang bis Ende

- Sie nehmen auf und gewinnen 14 der 27 Stiche. Sie haben 28 Karten gewonnen, Ihr Gegner 26.
- Ihre 28 Karten sind zusammen 55 wert. Zwei Drittel je Karte sind bei 28 Karten 28 × ⅔ = 18⅔, also zählen Sie 55 − 18⅔ = 36⅓.
- Die 26 Karten Ihres Gegners sind 106 − 55 = 51 wert. Weniger 17⅓ sind das 33⅔. Beide zusammen machen 70.
- 36⅓ sind 36 oder mehr, also gewinnen Sie das Spiel.
- Sie hielten den Sküs, den Mond und den Pagat in der Hand und sagten die Troule an, also wird sie bezahlt.
- Im letzten Stich spielen Sie den Pagat aus, und Ihr Gegner legt ein höheres Tarock. Sie verlieren das Ultimo.
<!-- if values=2/3/4 -->
- Die Tafel schreibt bei Ihnen an: Spiel 3, Troule 1.
<!-- end -->
<!-- if values=1/2/3 -->
- Die Tafel schreibt bei Ihnen an: Spiel 2, Troule 1.
<!-- end -->
<!-- if values=1/2/4 -->
- Die Tafel schreibt bei Ihnen an: Spiel 2, Troule 1.
<!-- end -->
<!-- if ultimo=1 -->
- Bei Ihrem Gegner schreibt sie an: Ultimo − 1.
<!-- end -->
<!-- if ultimo=2 -->
- Bei Ihrem Gegner schreibt sie an: Ultimo − 2.
<!-- end -->
<!-- if both_pass=play -->
- Wäre die Partie doppelt gezählt worden, wäre jede dieser Zahlen doppelt so groß.
<!-- end -->

## Touren und die Tafel

- Eine Tour sind zwei Partien, eine von jedem von Ihnen gegeben. Sie haben anfangs 3 oder 6 Touren gewählt: 6 oder 12 Partien.
- Die Tafel führt für jeden von Ihnen ein Conto. Jeder Point, der Ihnen gezahlt wird, kommt mit seinem Grund in Ihr Conto. Dem Zahlenden wird nichts abgezogen.
- Der Saldo ist der Unterschied der beiden Contos.
- So wurde ein Tarockabend geführt. Keine Quelle sagt, dass man das Strohmandl so führte: wir haben es geborgt.

## Abrechnen

- Am Ende des Abends wird der Unterschied der beiden Contos zu 2 Kreuzer der Point beglichen. Zahlen muss, wer das kleinere Conto hat.
- Beispiel: nach sechs Partien steht Ihr Conto auf 12, das Ihres Gegners auf 9. Der Unterschied ist 3 Points, also 6 Kreuzer, und Ihr Gegner zahlt Ihnen.
- Gleiche Contos: niemand zahlt.
- Sie können weitere Touren anhängen, wenn beide zustimmen. Die Contos laufen weiter.
