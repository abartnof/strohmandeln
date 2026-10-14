# 07 - Strategy, statistics, digital implementations and AI (Strohmandeln and transferable Tarock)

Strand 07. Written 2026-10-14. All local copies are under `research/sources/07*/` (see Sources). Items marked
`[UGC]` are forum/blog/user content, `[promo]` commercial pages, `[OCR]` OCR text I normalised (Fraktur long-s and
the OCR's spaced punctuation silently repaired; the PDFs are poor scans), `[derived]` = a secondary document that
re-quotes a primary one. "Original computation" = my own Python, labelled as such, never presented as published.

## Headline findings

1. **[review: read as 'one substantial MODERN body'; headline 2 below shows Werner's independent period chapter, and the claim that everything in 'the Finnish blogs and the 2026 GitHub spec traces back to it' is too strong - the blogs only report play, the spec also draws on Tirol 2025, Wareh and Furr.] There is exactly one substantial body of Strohmandeln strategy, and it is Mayr & Sedlaczek's "Spieltaktische
   Hinweise"** on their web page for *Das große Tarockbuch* (Wayback copy of 29 Jan 2002, German original read by me;
   English translation by Tarik Wareh also read). Everything the modern literature says (including the Finnish blogs
   and the 2026 GitHub spec) traces back to it. [07-S1] [07-S2]
2. **A period (19th-century) strategy text exists**: K. Werner, *Das moderne Tarokspiel*, 2nd ed. (Wien/Pest/Leipzig,
   Hartleben; the preface to the 2nd edition says it added "Strohmann-Tarok und Neunzehner-Rufen"), chapter "Das
   Strohmann-Tarok" pp. 129-135, with a take-up rule, an "Impassiren is decisive" maxim and advice on delaying
   honour announcements. The scanned PDFs carry no year [UNVERIFIED year]. [07-S3] [07-S3b]
3. **The take-up rule is a number.** M&S: take up when the "sure trick" cards in hand are worth 20-25 points
   (fractional values, Trull card/King 4 1/3, Queen 3 1/3), and hold at least 6-7 tarocks. Werner's older rule:
   "three Honneurs, 6 to 7 Taroks and one or two guarded Queens/Cavalls". [07-S1] [07-S3]
4. **Original computation: hands meeting the M&S threshold are rare** - only about 4-7 % of 15-card hands reach a
   "safe-trick" value of 20, and only ~2.6-4 % if 6+ tarocks are also required (section 5). So if players really
   keep to the book rule, neither player qualifies in roughly nine deals in ten and the "simple game" is the normal
   outcome - which squares with the old advice to fold instead of playing a simple game. [07-S9 = my computation]
5. **The computer opponent is a textbook PIMC case.** In Strohmandeln the unknown set shrinks fast and the opponent's
   face-up stack cards and the cards he took publicly are known; the only unusual thing is that *your own* straw
   cards are unknown to you too, so you must sample both. Gams & Luštrek's Tarok program (alpha-beta + Monte Carlo
   sampling) and Long et al. (why PIMC works) are the directly relevant literature. [07-S6] [07-S7]
6. **Digital**: I found no web/app implementation of Strohmandeln that is actually playable. The closest is
   `smatkovi/harbour-tarock` (GitHub, created 11 Sep 2026): a full written Strohmandeln specification incl. a
   "hints for the computer opponent" chapter and UI components already built (a `StrawmanRow` component), but
   Strohmandeln play is "planned after 1.0". Officer's Skat has real apps (KDE LSkat, an iOS app) with the same
   face-down/face-up stack mechanism. [07-S10] [07-S11] [07-S12]

---

## 1. Strategy maxims (Strohmandeln proper)

All quotes of M&S are from the German 2002 web page [07-S1] unless noted; the English is my translation (Wareh's
translation [07-S2] agrees in substance; it adds the 40-card variant and is a later revision).

### 1.1 Whether to take up ("Aufnehmen") or pass

- Stakes: taking up wins 3, loses 4 (opponent gets 4); the simple game is worth 2 (classic) - so the taker "has the
  chance to win 1 more game point ... but risks losing 2 more". Werner has the same 3 / 4 / 2 table and the same
  remark ("Der Aufnehmer hat also die Chance, 1 Point mehr ... zu gewinnen, er riskirt dagegen 2 Points"). [07-S1]
  [07-S3 p.130-131]
- **Hand-strength rule (M&S):** "Wir haben festgestellt, dass man ein aufgenommenes Spiel riskieren kann, wenn man
  sichere Stecher im Blatt hat, deren Wert 20 bis 25 Punkte ausmacht. Kommt man nur auf knappe 20 Punkte, so ist die
  Aufnahme eher riskant, kommt man in den Bereich von 25 Punkte oder sogar darüber, so sind die Aussichten auf einen
  Erfolg sehr groß." - "A game can be risked with sure trick-winners worth 20-25 points; barely 20 is rather risky; 25
  or more has very good prospects." [07-S1]
- **What counts as a sure trick-winner:** "Natürlich Sküs und Mond, aber auch der Pagat, der im Strohmann-Tarock meist
  heimzubringen ist ... Weitere sichere Stecher ... sind Könige und natürlich Damen der selben Farbe. Auch mehrfach
  besetzte Damen im Handblatt ohne den dazu passenden König garantieren meist einen Stich." Values are the fractional
  ones: Trull card and King 4 1/3, Queen 3 1/3 (Cavall 2 1/3). [07-S1]
- **Worked examples (M&S):** Sküs, Mond, H-King, H-Queen + a guarded C-Queen = 19 2/3, "Risikofreudige Spieler werden
  in diesem Fall aufnehmen und auf überdurchschnittlich gute Karten in den Strohmännern hoffen. Stellen sich diese
  nicht ein, ist das Spiel verloren." Sküs, Pagat, S-King, S-Queen, C-King + a well-held H-Queen = 24, "wirst du wohl
  aufnehmen". [07-S1]
- **Second condition:** "Wichtig ist allerdings, dass du auch mindestens 6 bis 7 Tarock besitzt. Denn wer ein Spiel
  mit ganz wenigen Tarock im Handblatt beginnt, muss befürchten, dass sein Gegner in Tarock besonders stark ist und mit
  Tarock einige fette Farbstiche macht." [07-S1]
- **Why the hand is hard to value:** "zu Beginn des Spiels 12 deiner insgesamt 27 Karten in den Strohmännern liegen, das
  sind rund 45 Prozent deiner Karten" - you cannot see nearly every second card of your own. (12/27 = 44.4 %, checks.)
  [07-S1]
- **Werner (period):** "Maßgebend zur Entscheidung der Frage, ob man eine Partie aufnehmen soll, sind die Honneurs,
  d. i. die Troule und die 4 Könige. Hat man 3 Honneurs, 6 bis 7 Taroks und eine oder zwei besetzte Damen, respective
  Cavalls, so kann man eine Aufnahme schon riskiren. Hat man aber den Pagat schwach mit Taroks besetzt, so kann man
  nicht darauf bauen, dass man denselben nach Hause bringen wird, kann seinen Zählwerth also nicht in den Calcul über
  den wahrscheinlichen Gewinn der Partie einbeziehen." - i.e. a *weakly guarded Pagat must not be counted* as a sure
  trick, whereas M&S count it. [07-S3 p.131] [OCR]
- **Fold on no tarock:** a hand with no tarock may be thrown in and the same dealer re-deals (M&S rule; Werner's
  text has no such rule in the part I could read). [07-S1] [07-S4 p.2]
- **Fold the simple game altogether:** "Alte Tarockbücher empfehlen, ein einfaches Spiel gar nicht zu spielen, sondern
  die Karten zusammenzuwerfen." If so, deal only the hands first and the straw men only once someone takes up.
  [07-S4 p.2] [07-S1]
- **Dealer vs forehand:** the dealer ("Hinterhand") decides second, knowing forehand passed; nothing in the sources
  says more. (The 2026 GitHub spec's claim that the dealer "may be more aggressive" is its own suggestion, not a quoted
  maxim - see 3.3.) [07-S1] [07-S10]

### 1.2 Managing the straw men (the heart of the game)

- **Play your straw men off quickly**: "Es ist wichtig, dass du deine Strohmänner rasch abspielst, damit du volle
  Einsicht in alle deine Karten gewinnst." Conversely "es [wird] fast immer günstig sein, den Gegner am Abspielen
  seiner Strohmänner zu hindern, um ihn in Unklarheit über seine Stärken und Schwächen zu halten." Werner agrees but
  with a caveat: "wenn es auch meist im Interesse des Spielers liegt, so rasch wie möglich seinen ganzen Talon
  aufzudecken, so giebt es doch zahlreiche Fälle, die den Spieler zwingen, einen Theil seines Talons so lange wie
  möglich unberührt liegen zu lassen" - above all when the opponent threatens Pagat ultimo (to keep tarocks buried
  that the opponent would otherwise "herausreißen") or to avoid leading into a fork (*Gabel*) so as to make a
  favourable *Impaß* oneself. [07-S1] [07-S3 p.134]
- **The recipe (M&S):** if the opponent shows hearts and diamonds atop his stacks, "dann wirst du Pik oder Treff
  spielen, im Idealfall von deinen Strohmännern. Dies führt dazu, dass du deine Strohmänner abbaust, während dein
  Gegner Handkarten verwenden muss und auf seinen Strohmänner-Karten sitzen bleibt." Lead a suit the opponent has *no*
  face-up card in, from your own stack. [07-S1]
- **The trap to avoid:** "Du solltest gegen Ende eines Spiels jedenfalls nie in die Situation kommen, dass dein Gegner
  schon alle Strohmänner-Karten aufgenommen hat, während du alle deine Handkarten verausgabt hast und deshalb
  gezwungen bist, von deinen Strohmännern wegzuspielen. Tritt dieser unangenehme Fall ein, wird dein Gegner nach
  Belieben das Spiel diktieren und der Reihe nach deine Karten abfordern." Never run out of hand cards while stacks
  are full. [07-S1]
- **Exposed Queens go at once**: "Eine auf den Strohmännern aufliegende Dame solltest du sofort wegspielen, auch wenn du
  nicht den König im Handblatt hast ... zu Beginn des Spiels [ist] die Wahrscheinlichkeit recht groß, dass der König in
  einem der Strohmänner schläft und du daher die Dame heimbringst. (In 4 von 10 Fällen wird es dir gelingen.)" Exception:
  "wenn dein Gegner in genau dieser Farbe mindestens 2 niedrige Karten aufliegen hat." Same for exposed Knights
  (Reiter). [07-S1]
- **Read the sleeping King**: "Spielst du eine aufliegende Dame aus und dein Gegner gibt eine niedrige Farbkarte zu, dann
  war also ein schlafender König dafür verantwortlich. Jetzt heißt es, die restlichen Karten dieser Farbe rasch
  wegspielen. Sollte dein Gegner den König später aus seinen Strohmännern aufnehmen, wirst du diesen König mit Tarock
  stechen." (A low card on your Queen means he did not hold the King - it is still buried.) [07-S1]
- **Two exposed cards of one suit: play the higher** ("Liegen Dame und Kavall oder Kavall und Bube einer Farbe auf zwei
  deiner Strohmänner auf, so spielst du in der Regel die höhere Karte. Das gleiche gilt für Skartindeln; es besteht
  immer die Möglichkeit, dass die nächsthöhere Figur oder das nächsthöhere Skartindel noch in den Strohmännern
  schläft."). [07-S1]
- **Pips on the stacks, honour in hand:** "Liegen bei dir zwei oder drei Skartindeln einer Farbe auf, von welcher du eine
  der höheren Figuren ... in den Handkarten besitzt, während dein Gegner kein Blatt dieser Farbe aufliegen hat, so
  spiele die Figur aus den Handkarten." [07-S1]
- **Order of honours**: King in hand + Queen exposed: "zuvor die Dame wegzuspielen"; Queen and King in hand + Cavall
  exposed: play the Cavall quickly - "Hiedurch gelangst du früher in den Besitz der verdeckten Karten." [07-S1]
- **Following suit with junk**: if the opponent leads a suit in which you hold only pips (hand and stacks), "bedienst du
  von den Strohmännern"; if a Jack is exposed and he led a higher honour, follow with the Jack "selbst dann, wenn du
  auch ein oder zwei Skartindeln in den Händen hast". [07-S1]
- **Do not sit on a void**: "Schwache Strohmännerspieler lieben es, keine Karten von ihren Päckchen wegzuspielen, wenn sie
  in einer Farbe blank sind. Sie erhoffen sich, dass der Gegner diese Farbe bringen wird, und dass sie dann mit Tarock
  stechen können. Aber wahrscheinlich wird der Gegner mißtrauisch sein ..." - a strong opponent plays that suit only
  once most stacks are down. [07-S1]
- **The only feint listed**: follow the opponent's King with your Cavall while holding a pip, so he thinks the Cavall
  was your last card and does not lead the Queen after it; "so durchsichtig, dass es nur ein oder zwei Mal pro Partie
  angewendet werden kann. Sobald alle 6 Strohmänner abgebaut sind, wird dein Gegner ohnedies erkennen, ob du geblufft
  hast." [07-S1]
- **Avoid trumping a hidden-suit gamble**: "Beim Impassiren muß man natürlich auch immer auf die in den Talons noch
  verdeckt liegenden Karten Rücksicht nehmen; da man diese aber nicht kennt, ... bleibt der Combination und allerdings
  auch dem Spielglück ein weites Feld gelassen, was zur Erhöhung des Interesses am Strohmann-Tarock wesentlich
  beiträgt." [07-S3 p.135]
- **Information-gathering is part of the skill**: "gute Spieler [zählen] nicht nur die gefallenen Tarock, sondern [achten]
  auch genau darauf, wie oft jede Farbe gespielt worden ist und welche Farben vom Gegner mit Tarock gestochen wurden ...
  kann oft noch ein Spiel im letzten Moment für sich entscheiden, obwohl er in den Tarock viel schwächer war."
  Werner: "Das Spiel erfordert ein genaues Tarokzählen und eine große Aufmerksamkeit auf jede einzelne Farbenkarte. Der
  Spieler muß stets genau wissen, wie viel Karten von jeder Farbe schon 'draußen' ... sind, welche Karte jeweilig major
  ist." [07-S1] [07-S3 p.134]

### 1.3 Suit play ("Nach hinten spielen", Impassiren)

- **Play from below in unsettled suits**: "Ein Kriterium des Strohmann-Tarock ist es, sich bei nicht geschlossenen Farben
  nach hinten zu spielen. Ein Ass oder einen Zehner des Gegners wirst du nicht gleich mit dem König stechen, denn dieser
  könnte einen fetteren Stich bringen ... Du wirst also ... den König schinden und ein Skartindel zugeben." Limit:
  "zu Beginn des Spiels [liegen] im Durchschnitt 3 Karten einer Farbe in den Strohmännern. Du darfst daher anfangs die
  Taktik ... nicht übertreiben, ansonsten kann es passieren, dass dein König mit Tarock gestochen wird." [07-S1]
- **If your King was trumped**: "heißt es abwarten" - hold the Queen/Cavall; the opponent may still draw one of the
  missing suit cards from his stacks. [07-S1]
- **Zugzwang principle**: "Sind die Karten einer nicht geschlossenen Farbe ziemlich gleichmäßig auf die beiden Spieler
  verteilt, so ... kommt [es] darauf an, wer als erster diese Farbe anzieht ... Wer als erster beginnt, die umkämpfte
  Farbe zu spielen, wird am Ende eine schlechtere Punktebilanz in dieser Farbe aufweisen." Usually this happens in 2 or
  3 suits at once; "Die Kunst des Strohmandelns besteht darin, in diesen Fällen die richtigen Prioritäten zu setzen."
  [07-S1]
- **Werner, same idea, nearly the same words, decades earlier (undated 2nd edition)**: "Die größte Rolle beim Strohmann-Tarok spielt das
  Jmpassiren [Impassiren]. Dies allein entscheidet meist über den Ausgang der Partie. Man muß sich hüten, eine Farbe zu
  bringen, deren Zählkarten zwischen beiden Spielern gleichmäßig vertheilt sind, man muß vielmehr den Gegner zu
  zwingen suchen, eine solche Farbe anzuspielen. Wie letzteres zu geschehen hat, darüber lassen sich keine allgemeinen
  Regeln aufstellen, das kann man nur durch die Uebung erlernen. In zahlreichen Fällen wird man sich erst von seinen
  Taroks befreien müssen, um im Farbenspiel vortheilhaft agiren zu können." (Note: Werner says *shed* tarocks to make
  the opponent open the contested suit; M&S say the tarock-strong player draws trumps first.) [07-S3 p.134-135]

### 1.4 Tarock play

- "Jener Spieler, der in Tarock stärker ist, wird zwischendurch tarockieren, ehe er eine nicht geschlossene Farbe
  anzieht. ... Du ziehst daher immer von einer geschlossenen Reihe Tarock an, egal von welcher Höhe." Example: with III,
  VI, IX, XIV, XV, XVI, XVII, XX, Sküs play the solid run XIV ... XVIII (the printed example is loose - XVIII is not in
  the listed hand; the sense is "lead from your longest solid run"); with no solid run (V, VII, XI, XVI, XVIII) "spielst
  du eines der niederen Tarock, um mit den höheren später Stiche zu machen. Stellt sich heraus, dass du in Tarock zu
  schwach warst, wirst du schließlich doch wieder zu den Farben wechseln." [07-S1]
- Rule of thumb from the same text, for the pagat: if you hold the Pagat with "nur wenige[n] Tarock, sagen wir 7 oder
  noch weniger" then "versäume es nicht, ihn bei nächster Gelegenheit einzustechen, um dann das Hauptaugenmerk auf das
  Farbenspiel richten zu können." (Cash the Pagat early with a short trump holding.) [07-S1]
- Contrast, Werner: the Pagat "kann natürlich nie ultimo angesagt werden, da es ja möglich ist, dass der Gegner, nachdem
  er seine Handkarten abgespielt hat, als letzte Karte seines Talons noch einen Tarok aufdeckt, welcher den Pagat
  aufhalten würde." [07-S3 p.133]

### 1.5 Pagat ultimo, Mondfang, the endgame

- **Silent Pagat ultimo needs >11 tarocks**: "Es ist im Strohmann-Tarock sehr schwierig, den Pagat zu ultimieren, obwohl er
  nur still gespielt werden kann. Nur wenn du nach dem ersten Aufdecken der Strohmänner mehr als die Hälfte aller im
  Spiel befindlichen Tarock, also mehr als 11 Tarock, in deinem Handblatt hast, lohnt es sich, auf diese stille Prämie zu
  spekulieren. Aber auch dann wirst du mit Austarockieren nur in den seltensten Fällen zum Ziel kommen. Du brauchst
  zusätzlich zumindest eine lange Farbe, mit der du dem Gegner Tarock ziehst." [07-S1]
- **Hunting the Pagat**: with >11 tarocks but no Pagat "wirst du versuchen, auf den Pagat ... Jagd zu machen" by playing
  trumps before suits. [07-S1]
- **Defence vs an Pagat ultimo threat**: "kann es sich lohnen, das Abspielen der Strohmänner hinauszuzögern. Findest du in
  den letzten Karten der Strohmänner einige Tarock, kann es dir unter Umständen gelingen, den Pagat abzufangen" - but "nur
  ... wenn das Spiel bereits verloren ist. Wird mit Grand Point gespielt, ist davon überhaupt abzuraten" (you would
  usually pay the Grand Point instead). [07-S1]
- **Mondfang (capture of the Mond)**: not separately scored "aber oft spielentscheidend, weil es um einen Fünferzähler geht".
  The defence: "Du behältst dir ein oder zwei kleine Tarock, um den Gegner in den Stich zu bringen. Dieser spielt dann
  eine Tarock nach, die du mit dem Mond stichst." Corollary: "Routinierte Strohmann-Tarockierer heben sich ohnedies
  Tarock II und Tarock III auf" - to draw trumps *and* for Uhu/Kakadu - which "nur mit einem in Tarock extrem starken
  Blatt zu gewinnen sein [werden]". [07-S1]
- **Mondopfer (Mond sacrifice)**: when all high trumps except the Sküs have fallen and the game is already won, play the
  Mond hoping the opponent trumps with the Sküs "in der Meinung, das Spiel nochmals herumzureißen" and so pays
  Rostopschin too - only once your Pagat is safely home, "sonst würde dein Gegner die stille Trull gewinnen." [07-S1]
- **Rostopschin free option**: you lose nothing by trying; if you see he will capture the XVII or XVIII "kannst du dennoch
  die beiden Karten spielen und lediglich ... auf die Wörter Ross! und Topschin! verzichten." [07-S1]
- **Honour bonuses (Werner, period)**: because honours are paid extra, with two Trull cards or three Kings do not trump
  too early with them, since you may yet draw the third Trull card / fourth King and then announce; "zögere man mit dem
  Ansagen derselben doch so lange wie möglich, um dem Gegner nicht vorzeitig sein Blatt zu verrathen. Auch kann man die
  Honneurs, wenn es im Interesse der Partie liegt, verschweigen, erhält sie aber dann natürlich nicht bezahlt." [07-S3
  p.135] M&S, by the same logic, don't allow an *announced* Pagat ultimo because success would hinge on whether the
  opponent's last stack cards are tarocks ("Schon ein stiller Pagat ultimo ist schwer genug"). [07-S4 p.2]
- Counting advice that carries over: keep track of trumps (Macfadyen: "calculate at the beginning how many trumps you
  have yet to see (22 less those discarded less those in your own hand), and knock one off this number each time you
  see a new trump"). [07-S5]

### 1.6 Transferable Tarock strategy (Königrufen/Tapp) that fits the two-hand game

Handled briefly because Strohmandeln has no partners and no talon exchange; only these carry over:

- *Count trumps* (above). [07-S5]
- *Suit nobody holds behaves like a trump*: "a suit of which everyone is void behaves like the 0 of trumps if you lead
  it" (Macfadyen on Besserrufer). In Strohmandeln the corresponding fact is that a suit in which the opponent has no
  hand card but still has cards in a stack can reappear (void inference decays when he next turns a card). [07-S5]
  [07-S10 for the inference-decay rule - that rule is the spec author's, not a published maxim]
- *Lead small first, keep the master trump for the moment only one opponent can follow* (Macfadyen, Dreier) - in a
  two-hand game "only one opponent" is always true, so this reduces to "do not lead the Sküs/XXI into a trick the
  opponent can win cheaply". [07-S5]
- *The Trischaken tip* "when on lead, to play a small card where you have a king, so that the king will probably be
  trumped on the second round" is a Königrufen negative-game tip; only relevant if the optional Trischaken (Tirol
  profile) is used. [07-S5] [07-S7]
- Mayr & Sedlaczek's own *Die Strategie des Tarockspiels* (3rd ed. 2010) has chapters "Mondfang - wenn der Gstieß auf
  der Lauer liegt" (p.126), "Kaiserstich - wenn der Pagat über Gstieß und Mond triumphiert" (128), "Strategische
  Überlegungen zur Pagatkassa" (130), "Warum die Damen so wichtig sind" (122), "Tarockschlagen" (114-116) and, for
  the two-hand game, only "Zweiertarock oder Strohmandeln - so sehen die Regeln aus" (p.369) and "Mit welchem Handblatt
  Du ein Spiel aufnehmen kannst" (p.372). I read only the table of contents (DNB record 1041884893), not the book;
  the strategy for Strohmandeln in the book is therefore probably the same take-up advice as the web page. [07-S8]

---

## 2. Published numbers

- **Total points**: 70, counted in threes; win = 36 = "35 and 2 Blatt" (35 2/3 rounds to 36); **[review correction: the first draft's parenthesis said '35 + 2 cards counted = loss for the taker', which is the wrong way round. M&S: the taker needs "mindestens 35 Punkte und 2 Blatt"; 35 + 1 Blatt LOSES (04-S37, Krone 1916); Tirol: "36 Punkte (= 35/2 Blatt)". The same wrong parenthesis is printed in the header of `sim/stats07.py`'s output (`sources/07f/original_computation_out07.txt`, section 1), which is not a computation error.]** Pagat: "The total value
  of the pack comes to 70 card points." (Tirol: "Das Spiel hat somit 70 Punkte; um zu gewinnen sind 36 Punkte
  erforderlich (= 35/2 Blatt ...)") [07-S6b] [07-S7]
- **Share of unseen cards**: 12 of 27 = 45 % (M&S, above); "im Durchschnitt 3 Karten einer Farbe in den Strohmännern"
  at the start (M&S; my check: all 24 straw cards x 8/54 = 3.56 per suit across both players, 1.78 per player). [07-S1]
- **4 in 10**: M&S's only probability claim, for an exposed Queen coming home (see 5.5 for my check). [07-S1]
- **"Pagat comes home usually"**; **">11 tarocks"** (=more than half of 22) as the Pagat-ultimo threshold. [07-S1]
- **No published frequency** for how often a game is taken up, average points or score distributions was found in
  any source I read (M&S, Werner, Furr, Tirol, blogs). [gap]
- Tiny anecdotal data: a 2008 session report (Finnish) played "about ten deals", the second player winning easily;
  Mikko's blog: three deals, 7-4 final in game points, one deal featuring an Uhu. [07-S13] [07-S14] [UGC]
- Scoring tables vary (3/4/2; Finnish blog: 3 won / 4 lost / 1 simple): collect in the rules strand; mentioned here
  only because take-up risk/reward depends on them. [07-S2] [07-S13] [07-S14]

---

## 3. Digital implementations

### 3.1 Strohmandeln proper
- **smatkovi/harbour-tarock** (GitHub, repo created 2026-09-11, last push 2026-10-12, GPL-3.0-or-later per its README [review: GitHub's own licence field in the saved `07e/repo.json` is "Other / NOASSERTION", i.e. no recognisable LICENSE file was detected; the GPL statement rests on the README text only, and the spec re-quotes M&S/Tirol/Wareh text whose copyright is not GPL], 0 stars):
  "Tarock for Sailfish OS and Android: Königrufen, Hungarian tarokk, Tapp-/Dreiertarock and Strohmandeln with computer
  players, LAN and a learning mode". README: Strohmandeln and Tapp are "specified in `docs/` and planned after 1.0".
  The design doc's milestone table nonetheless lists "M12 Strohmandeln zu zweit" with the `AT-STROH-MS-ERW` rule
  profile. Source files fetched: README, `docs/strohmandeln.md` (186 kB German spec; [review] the headings run §0-§14, i.e. 15 sections), `docs/design.md`,
  `qml-common/StrawmanRow.qml`, `HintBubble.qml`, `WhyDialog.qml`, `src/core/Ai.cpp`,
  `src/core/profiles/AtStrohMsErw.cpp`. [07-S10] `[UGC]` `[derived]` (the spec re-quotes M&S).
  - **UI lessons visible in the code/spec** (a design, not a shipped Strohmandeln UI):
    - Straw men shown as three packets, the top card face-up and *clickable like a hand card* because it belongs to the
      hand; a number beside each packet "sagt nur, wie viele Karten es noch sind"; a second small, non-clickable row
      shows "die Karten, die dieser Sitz zuletzt offen aus einem Päckchen ins Blatt genommen hat ... weil der Gegner sie
      sehen muss". (`StrawmanRow.qml` header comment.) [07-S10]
    - Auto-reveal: the engine turns the next card after a stack card is played and moves K/trump into the hand; the
      opponent must be shown each card before it is taken (Wareh: "The opponent has the right to see the cards ...
      It is therefore advisable to lay them aside face-up ... and then afterwards to take them into the hand").
      [07-S10 s.4] [07-S2]
    - Network model: a guest gets the face-up cards and the *number* of buried cards, never the cards underneath
      (design doc M12 row). [07-S10]
    - **Legality hints with reasons**: spec §11.4 lists exact refusal messages, e.g. E2 "{Farbe} musst du bedienen - vom
      Strohmann." (suit is only available as a face-up stack card), E6 "Diese Karte liegt verdeckt.", E7 "Das ist ein
      Strohmann deines Gegners." and a green-marking rule when a suit is available both in hand and on a stack (E3).
      Coaching hints C1-C5, e.g. C2 "Tipp: Spiel lieber vom Strohmann", C3 "Tipp: Spiel die aufliegende Dame - in vier
      von zehn Fällen schläft der König noch in einem Strohmann", C4 warning when hand cards would run out while
      stacks are full. [07-S10 s.11.4]
    - Hints share the evaluation of the computer player ("the same ranking that steers the computer players, so the
      advice never contradicts what they would do"), and the hint bubble never opens by itself except at the beginner
      level after 1.5 s. (`HintBubble.qml` header.) [07-S10]
  - The spec's AI chapter (s.10) is useful but **is the spec author's design**, not published tournament knowledge:
    levels (rule heuristics H1-H11 transcribed from M&S; "Normal" adds void inference + card counting; "Hard" PIMC with
    ~40 worlds + alpha-beta, exact from trick ~18 [review: the spec is inconsistent with itself - its level table says "ab Stich 15 exakt", its budget paragraph "Ab Stich 18"]), a take-up threshold of 21 2/3 +/- and an evaluation function with
    weights. It also stresses the one point that differs from every other Tarock AI: **sample your own buried cards
    too**. [07-S10]
- **Searches for others**: GitHub repository search for `strohmandeln` returned only the repo above; searching `tarok`,
  `tarock`, `strohmandeln` produced Königrufen/Zwanzigerrufen/Slovenian projects (e.g. `semanticweights/tarok`, an
  OpenSpiel environment for Slovenian Tarok; `DerDodo/tarock20`, Zwanzigerrufen in Kotlin/React; `Lenart12/tarok`,
  `coder0987/Taroky` web pages) - none for the two-hand game. App-store/web search found none either. [07-S15] `[UGC]`
- **Slovenian Tarok app `mytja/Tarok` ("Palčka")** (Flutter + Go backend, 25 stars, AGPL-3.0): bots run fully offline
  via the "StockŠkis" engine; open source and the best-maintained Tarock UI/engine to crib architecture from, but
  three-/four-player Slovenian rules. [07-S16] `[UGC]`
- **Hungarian "Strohmandli"** (42-card version) is described on scordatura.hu (StüsziVadász pages); a page of
  instructions exists, no program found. The page claims the "original" rules are on pagat.com - but pagat has no such
  page (it only lists the game in the Austria overview). [07-S17] [07-S6c] `[UGC]`

### 3.2 Officer's Skat and other relatives with the same stack mechanism
- **KDE LSkat ("Lieutenant Skat", from Offiziersskat)**: two players, AI or human; "Cards are displayed in two horizontal
  rows - four cards per row. Half of the cards are face down and half are face up. As a player you can see the
  opponent's cards as well as your own. ... The opponent has to reply by choosing a corresponding card from his/her own
  stack. ... if your opponent is artificial intelligence, its cards will be chosen automatically." Objective: "score more
  than 60 points"; 16 rounds. Updated through 2026-10-08 (release 1.50.26082). It is a minimal version of the same idea
  (face-down cards uncovered when the one above is played). [07-S11] 
- **Officer's Skat (iOS, Victor Olaru)** `[promo]`: "most popular Skat variant for two players", simplified rules,
  32 cards, "organic AI with three different playing strengths", English/German, ads removable for $1.99; last release
  1.1.3 on 11/04/2018; its other apps listed include Piquet Pro, Classic German Whist, Schnapsen. [07-S12]
- Not examined: Finnish board-game-group's "Tarotista Texas holdemiin" (book by Mikko Saari) which teaches Strohmann;
  the two Finnish blogs are the user reports. [07-S13] [07-S14] `[UGC]`

### 3.3 AI and solver literature relevant to a strong opponent
- **Gams & Luštrek (2003), "Intelligent System for Playing Tarok"**, CIT 11(3):209-215 (the three-player game, program
  "Silicon Tarokist"). Alpha-beta with a "fuzzy" transposition table that clusters strategically similar positions
  (2.3x fewer nodes at depth 9), history heuristic, minimal-window search, selective depth, and **Monte Carlo
  sampling for the unknown hands**: "each time it needs to choose a card to play, it generates a number of random
  distributions of other players' cards ... game-tree search is performed ... These values are then summed over all
  distributions and the one with the highest sum is played." All enhancements together: "184 times less game-tree nodes"
  (84 times less time). Bidding is done by playing out test games against a margin that adapts after wins/losses.
  They note the weakness of sampling: "it postpones decisions" and flag "making bad moves, because they are wrongly
  considered inevitable". Humans judged the program "good enough for playing against it to be interesting"; it "does make
  occasional mistakes and sometimes seems to be playing 'aimlessly'", but once held back a King to win a Queen "that could
  be called cunning if it were human". Relevant lesson: **search is adequate at the end and "quite inadequate at the
  beginning"**; knowledge rules were needed. They also say two-player tarok "relatively unknown" exists. [07-S6] 
- **Long, Sturtevant, Buro, Furtak (2010), "Understanding the Success of Perfect Information Monte Carlo Sampling in
  Game Tree Search"** (AAAI-10): PIMC is criticised in theory but strong in practice; they identify game-tree
  properties (leaf correlation, bias, disambiguation factor - how fast information is revealed) that predict whether
  PIMC works, and measure them for Skat and Hearts. Their general point: PIMC suits games where hidden information is
  revealed quickly - which Strohmandeln does (stack cards turn up every trick, unseen set shrinks). [07-S7]
- **Skat**: Kupferschmid & Helmert (2007), "A Skat Player Based on Monte-Carlo Simulation" (Springer LNCS 4630, DOI
  10.1007/978-3-540-75538-8_12) and Buro, Long, Furtak, Sturtevant (2009), "Improving state evaluation, inference, and
  search in trick-based card games" (IJCAI) - **metadata only** (OpenAlex), papers not read. Rebstock et al. (2019),
  "Policy Based Inference in Trick-Taking Card Games" (CIG; arXiv 1905.10911): trains opponent models on human Skat data
  to infer hidden cards, "vastly improves the inference" and the strength of the Skat AI Kermit's determinized search -
  I read only the abstract page. Edelkamp (2021) "On the Power of Refined Skat Selection" / "Challenging Human Supremacy
  in Skat" (SoCS 2021): abstract read - "reinforcement learning is less effective" in Skat than in Chess/Go, hence
  classical search + expert rules. [07-S7b] [07-S18] [07-S19]
- **ISMCTS**: Cowling, Powley, Whitehouse (2012), "Information Set Monte Carlo Tree Search" (IEEE TCIAIG) - metadata
  only; the standard answer to PIMC's "strategy fusion" if a determinised search proves too brittle. [07-S18]
- **Other Tarok AI titles** (metadata only, via OpenAlex; not read): Lapajne (2011) "A program for playing Tarok using
  Monte-Carlo tree search" (thesis); Marinčič, Gams, Luštrek (2006) "Bidding decision model for the card game Tarok";
  "Knowledge vs. simulation for bidding in Tarok" (2006); Požrl (2021?) master's thesis on reinforcement learning in
  Tarock (catalogue record only, via search snippet - [UNVERIFIED]). [07-S18]
- **Applying to Strohmandeln**: (a) the take-up decision is the analogue of the Tarok bidding paper (simulate play-outs
  from the opening position, adapt a margin); with 12 buried cards each you can estimate P(win) by dealing the
  unknown 39 cards 1-2 thousand times. (b) Mid-game: PIMC over worlds consistent with: the opponent's taken cards,
  the face-up stack cards, public count of buried cards per stack, void inferences that expire when he turns a new card.
  (c) Exact solving: late in the game (my guess: from about trick 15-18 when each side has <= 10 cards) an exact
  double-dummy solver is feasible; the spec author's estimate is the same. [07-S6] [07-S10] [UNVERIFIED as to speed]

---

## 4. Hunches followed

- *Is there a period strategy source for Strohmandeln?* Yes - Werner's 2nd-edition chapter (Wienbibliothek digital).
  Found by the hint in the Wikipedia article's own citations and a search for "Strohmanndl"; the OCR is poor but good
  enough to quote. Year of the 2nd edition not found (the preface to the 1st edition talks of the game's "bald 500jähriges
  Jubiläum" - i.e. written some time before the 1900s? not strong enough to date). [07-S3] [07-S3b] [OCR]
- *Is the pagat.com Strohmandeln page real?* The Mikko blog, the Hungarian page and the 2026 spec all say "rules on
  pagat"; pagat has no such page, only a one-line mention. Nothing. [07-S6c] [07-S10 s.0.2]
- *Wareh's "very good page with rules and strategy"* (Mikko's blog link) is the translation of M&S - it was the key find;
  `wareh.org` is dead but archive.org holds it. [07-S13] [07-S2]
- *Does the original German exist?* Yes: Wayback 2002 of tarockspiele.com; the 2007 Wiener Zeitung piece is the rules
  half only (30/40/20 "Cent" scoring, no tactics). [07-S1] [07-S4]
- *Are there GitHub/Strohmandeln projects?* One - and it was created a month ago. Likely to be what other agents' notes
  call "a spec"; treat it as `[derived]`. [07-S10]
- *Tapp Tarock page on pagat* yields nothing (points to Krüpl's archived page). [07-S6c]

## 5. Original computation (by the author of this note, Python 3, stdlib only)

Code: `research/sim/stats07.py`
(exact combinatorics + Monte Carlo; run as `stats07_run.py` with 100,000 deals per section and 500,000 for 5.5, seed
20261014) and `stats07b.py` (600,000 deals, seed 7). Outputs are saved in `research/sources/07f/`. The code is not in
the sources folder by the brief's rule 6. Reveal rule modelled as in M&S/Wareh/Werner: take K and tarocks from the top
until a non-K suit card appears; the **last card of a packet is taken regardless** (M&S/WZ 2007 and Wareh: "Die letzte Karte eines Pakets wird sofort aufgenommen"; [review: in Werner, p.133, it is the RULE that the last card of a packet stays face-up until played, and Wareh calls it the variant - the first draft had this the wrong way round] the variant that
it stays face-up). Each player has 15 hand cards and three packets of 4. Deal = random 54-card shuffle.

### 5.1 Total points
- Nominal sum of the 54 cards = **106**; 54/3 = 18 groups; 106 - 2x18 = **70**. Fractional values (Trull/King 4 1/3,
  Queen 3 1/3, Knight 2 1/3, Jack 1 1/3, empties 1/3) also sum to exactly **70**. Breakdown: Kings 20, Queens 16,
  Knights 12, Jacks 8, Trull 15, the 19 other tarocks 19, 16 pip cards 16. A game has 27 tricks. [07-S9]

### 5.2 The 15-card hand (exact hypergeometric)
| tarocks | P(exactly) | P(at least) |
|---|---|---|
| 3 | 0.0402 | 0.9895 |
| 4 | 0.1091 | 0.9493 |
| 5 | 0.1963 | 0.8402 |
| 6 | 0.2418 | 0.6439 |
| 7 | 0.2073 | 0.4021 |
| 8 | 0.1244 | 0.1948 |
| 9 | 0.0521 | 0.0705 |
| 10 | 0.0150 | 0.0184 |
| 11+ | 0.0033 | 0.0033 |
(mean 6.11 tarocks.)
- P(no tarock in the 15 cards, the fold/redeal case) = 6.5 x 10^-5 = **1 in 15,298**.
- P(holding Sküs, Mond and Pagat in the 15-card hand) = 15x14x13/(54x53x52) = **0.0183 (1 in 55)**; Sküs+Mond: 0.0734.
- P(four Kings in the 15 cards) = **0.0043 (1 in 232)**.
- So the M&S rule "6-7 tarocks" is about the median hand; "more than 11" (= at least 12) in the 15-card hand is a **0.04 %** event (1 in ~2,500; exact hypergeometric 0.000400; the 0.33 % in the table row '11+' is P(at least 11), a different threshold) [review correction: first draft said 0.33 %] (before the
  reveal; after taking tarocks from the stacks it is likelier - see 5.3). [07-S9]

### 5.3 Straw men (Monte Carlo, 100,000 deals; player 0)
- A priori the 12 straw cards hold on average **4.89 tarocks, 0.89 Kings, 5.78 "takers"** (K or tarock; 26 of 54).
  Conditional on your own hand: with 6 tarocks + 1 King in hand expect 5.85 takers in the packets; with 11 tarocks +
  2 Kings, 4.00 takers (the more you hold, the fewer remain).
- P(the top card of a packet is a taker) = 26/54 = 0.48. P(a 4-card packet has no taker at all) = C(28,4)/C(54,4)
  = **0.065**; P(at least one of your three packets is all non-taker: nothing can be drawn early) = **0.186**.
- Cards taken into hand at the opening reveal (after all your top-card draws): mean **2.76**; 0 taken: **13.0 %**; 1: 20.3 %;
  2: 20.3 %; 3: 11.6 %; 4: 14.2 %; 5: 9.7 %; 6: 6.3 %; 7+: 4.0 %. Mean hand size after the reveal 17.76.
- Opening exhausts no whole packet 71.2 % of the time, one packet 26.2 %, two 2.5 %, all three 0.07 %.
- Tarocks among all 27 cards of one player: mean 11.0; **P(>= 12 of the 22) = 39.2 %**, P(>= 14) = 8.4 %. P(all three
  Trull cards among the 27 cards of one player) = 0.118 (exact 27x26x25/54x53x52 = 0.1179). So the ">11 tarocks
  after the first reveal" condition for the silent Pagat ultimo is not rare in total holdings (39 % hold more than 11
  of the 22 tarocks in their 27 cards), but most of those are still buried at the start of play. [07-S9]

### 5.4 How often does a hand satisfy the M&S take-up rule? (Operationalisation - mine)
"Safe-trick" value = 4 1/3 for each of Sküs/Mond/Pagat and each King, plus 3 1/3 per Queen under three readings:
(QwithK) Queen counts only with its King; (Qlong) with its King or with >= 3 cards of the suit; (Qany) every Queen.
| reading | value >= 20 | >= 22 | >= 25 | >= 20 and >= 6 tarocks |
|---|---|---|---|---|
| QwithK | 4.44 % | 1.32 % | 0.77 % | 2.56 % |
| Qlong | 5.36 % | 1.94 % | 0.88 % | 3.04 % |
| Qany | 7.06 % | 3.54 % | 1.22 % | 4.16 % |
Reading: the book's 20-25 band is a *strong* hand. If both players follow it strictly, P(neither takes up) is about
(1-0.0536)^2 = **0.90** (approximate; hands are not independent). Either real players take up on much weaker hands
(M&S also count Cavalls and "good" Pagats, and they say "risikofreudige Spieler" take 19 2/3), or the simple game
(2 points or a thrown-in deal) is the usual result. Implementers should not copy 20 as a hard rule for the computer's
bid; it would almost never bid. [07-S9] [07-S1]

### 5.5 "In 4 von 10 Fällen" - the exposed Queen
Condition: a Queen is face-up atop one of my packets after both players' opening reveals; where is the King of that suit?
(600,000 deals, 230,114 such Queens, no information about my own hand.)
- King in **my hand** 36.9 %; King in the **opponent's hand** (can top the Queen immediately) 38.0 %;
  King still **asleep** in a packet: under my own cards 12.8 %, under his 12.3 % = **25.1 %**.
- Hence the Queen is unthreatened at once in **62 %** of cases (King mine or asleep), vs. M&S's "4 out of 10" for
  succeeding; the asleep-King share alone (25 %) is lower than 40 % while "King mine or asleep" is higher; the book's
  figure sits between them and may refer to a narrower situation (e.g. one in which you hold no King). My result
  **[review: the data DO reconcile with M&S. M&S's maxim is explicitly about playing the exposed Queen 'auch wenn du nicht den König im Handblatt hast'. Conditioning on 'I do not hold the King': P(King asleep) = 0.251 / (1 - 0.369) = 0.398, i.e. 4 in 10 - exactly M&S's figure. The sentence below that 'neither confirms nor refutes' is therefore too weak; the simulation confirms the book's number under its stated condition.]** (original wording: neither confirms nor refutes the maxim; it gives the numbers an implementer can use for a rule such as "if Q
  exposed, P(opponent has K in hand) = 0.38 before looking at the rest of the position".) [07-S9] [07-S1]

---

## 6. Gaps / leads not followed

- Dummett, *The Game of Tarot* (1980) pp. 457-459 and Beck (1972), Bamberger (2011), Furr's text beyond the rules
  (Furr's chapter has no strategy; checked) - Dummett may have comments on value of the straw men; not accessible.
- Mayr & Sedlaczek, *Die Strategie des Tarockspiels* (3rd ed. 2010) chapter p.372 "Mit welchem Handblatt Du ein Spiel
  aufnehmen kannst" - not read (only TOC). *Das große Tarockbuch* (2001), the printed source of the web page.
- The year of Werner's *Das moderne Tarokspiel* 2nd ed. and the pages before p.129 (the remainder of the Strohmann
  chapter's examples, which the PDFs omit; the PDF extracts are 7 + 2 pages only). Wienbibliothek ids 3752218, 3752195,
  3752194. The extract starts with the end of a worked three-player example, so there is more of the book.
- Kosakeln and Tapp strategy (no source found).
- Buro et al. 2009 (IJCAI) and Kupferschmid & Helmert 2007 full texts; Požrl (2021) and Lapajne (2011) theses; ISMCTS
  paper full text (download link was OA at eprints.whiterose.ac.uk but I did not fetch it).
- BoardGameGeek thread 312120 ("Strohmandeln - two-player Tarock"; the one the Mikko blog calls "a more thorough
  session report") could not be fetched (JavaScript wall).
- Forum posts [UGC] on Strohmandeln tactics: searches in German returned nothing usable (only "straw-man argument"
  noise).
- Experiment suggested for the build team: rather than trusting any threshold, run self-play of the heuristic AI to
  tune the take-up margin the way Gams & Luštrek adapt theirs.

## Sources

Local copy paths are relative to `research/sources/`.

- **07-S1** Wolfgang Mayr & Robert Sedlaczek, "Strohmann-Tarock (Strohmandeln)", Tarockspiele.com (website for *Das große
  Tarockbuch*, Perlen-Reihe 642, Wien 2001), Wayback copy of 29 Jan 2002.
  https://web.archive.org/web/20020129045230/http://www.tarockspiele.com:80/strohmann.html - local `07d/ms_web_2002.html(.txt)`.
  Section "Spieltaktische Hinweise" (all maxims in 1.1-1.5), "Renoncerregeln". German original.
- **07-S2** Tarik Wareh, "Strohmandeln (Tarock for 2 players)", English translation of the above (later revision, with
  40-card variant), https://web.archive.org/web/2015/http://www.wareh.org/cards/strohmandeln.html - local `07b/wareh.html(.txt)`.
- **07-S3** K. Werner, *Das moderne Tarokspiel. Eine Anleitung zur gründlichen Erlernung desselben nebst zahlreichen
  erläuternden Beispielen*, 2nd ed. (Wien/Pest/Leipzig: A. Hartleben; year not on the scan), chapter "Das Strohmann-Tarok"
  pp. 129-135, Wienbibliothek digital https://digital.wienbibliothek.at/download/pdf/3752218.pdf - local `07c/wb3752218.pdf/.txt`
  `[OCR]`. The publisher is from the search-result summary of the title page, not checked in the extracted PDFs [UNVERIFIED].
- **07-S3b** same book, "Vorwort zur zweiten Auflage" (adds "Strohmann-Tarok und Neunzehner-Rufen") - https://digital.wienbibliothek.at/download/pdf/3752195.pdf
  (local `07c/wb3752195.pdf`); "Vorwort zur ersten Auflage" 3752194 (`07c/wb3752194_vorwort_1aufl.pdf`).
- **07-S4** Mayr & Sedlaczek, "Zweiertarock (Strohmandeln) mit 54 Karten: Wie sehen die idealen Regeln aus", Wiener Zeitung,
  25 Sep 2007 (update 16 Jun 2011), https://www.tagblatt-wienerzeitung.at/startseite/archiv/95356_Wie-sehen-die-idealen-Regeln-aus.html
  (p.1) and `?em_cnt_page=2` (p.2) - local `07d/wz1.html`, `wz2.html` (+ `.txt`).
- **07-S5** Matthew Macfadyen (ed. John McLeod), "Königrufen - remarks on strategy", pagat.com, 1995,
  https://www.pagat.com/tarot/krufsk.html - local `07/pagat_krufsk.html(.txt)`.
- **07-S6** Mitja Luštrek & Matjaž Gams, "Intelligent System for Playing Tarok", *Journal of Computing and Information
  Technology* 11(3), 2003, pp. 209-215, DOI 10.2498/cit.2003.03.09, http://cit.fer.hr/index.php/CIT/article/download/1516/1220
  - local `07c/gams_lustrek_2003.pdf`, `gl.txt`. **07-S6b** Pagat, "Counting card points in Tarot games"
  https://www.pagat.com/tarot/counting.html - local `07/pagat_counting.html`; and Königrufen rules page `07/pagat_koenig.html`.
  **07-S6c** Pagat Austrian card games overview, https://www.pagat.com/national/austria.html - `07/pagat_austria.html`
  ("Strohmandeln (2 players) are less widespread", no link); Pagat Tapp Tarock `07/pagat_tapptarock.html`.
- **07-S7** Jeffrey Long, Nathan R. Sturtevant, Michael Buro, Timothy Furtak, "Understanding the Success of Perfect
  Information Monte Carlo Sampling in Game Tree Search", AAAI-10, https://doi.org/10.1609/aaai.v24i1.7562 - local `07c/long2010.pdf/.txt`.
  **07-S7b** Douglas Rebstock, Christopher Solinas, Michael Buro, Nathan Sturtevant, "Policy Based Inference in Trick-Taking
  Card Games", IEEE CIG 2019, arXiv 1905.10911 - `07c/rebstock.pdf` (2 pp. abstract). Tirol: Markus Mair, "Strohmandeln
  TAROCK", Jänner 2025, www.tarock.tirol - `07/tirol_strohmandln.txt`/`.pdf` (**07-S7c**, quoted in section 2 and 1.6).
- **07-S8** DNB catalogue record 1041884893 and table of contents, Mayr & Sedlaczek, *Die Strategie des Tarockspiels*
  (3rd, expanded ed., Wien: Ed. Atelier 2010, 407 pp.) - local `07/dnb_1041884893.xml`, `dnb_1041884893_toc.txt`; also
  `07c/lehmanns.pdf` (same TOC).
- **07-S9** Original computation: `research/sim/stats07.py`,
  `stats07_run.py`, `stats07b.py`; outputs `07f/original_computation_out07.txt`, `07f/original_computation_out07b.txt`.
- **07-S10** smatkovi, `harbour-tarock` (GitHub, GPL-3.0-or-later), https://github.com/smatkovi/harbour-tarock - local
  `07e/` (README.md, docs_strohmandeln.md, docs_design.md, qml-common_*.qml, src_core_*.cpp, repo.json). `[UGC]`
- **07-S11** KDE, "LSkat" (Lieutenant Skat / Offiziersskat), https://apps.kde.org/lskat/ - `07d/lskat.html(.txt)`.
- **07-S12** "Officer's Skat" (Victor Olaru), Apple App Store, https://apps.apple.com/us/app/officers-skat/id1308253159 -
  `07d/officers_skat_ios.html(.txt)` `[promo]`.
- **07-S13** Mikko Saari, "Thursday session: Thief of Baghdad, Strohmann-Tarock", Gameblog, May 2008,
  https://mikkosgameblog.com/2008/05/thursday-session-thief-of-baghdad-strohmann-tarock/ - `07/mikko.html(.txt)` `[UGC]`.
- **07-S14** "Strohmann-tarot (tai tarock)", Lautapelailua blog, 30 Sep 2008, http://lautapelailua.blogspot.com/ (page
  saved as `07/lautapelailua.html`) `[UGC]`.
- **07-S15** GitHub repository search API for tarok/tarock/strohmandeln, queried 2026-10-14 - `07d/gh_search.json`, `gh_topic_tarok.html`.
- **07-S16** mytja, `Tarok` (Palčka), https://github.com/mytja/tarok - `07c/mytja.html` `[UGC]`.
- **07-S17** scordatura.hu / StüsziVadász, "Strohmandli" (Hungarian 42-card version), https://www.scordatura.hu/stuszivadasz/paros.html - `07d/scordatura_paros.html(.txt)` (mojibake; not analysed in detail) `[UGC]`.
- **07-S18** OpenAlex API metadata (api.openalex.org) for the AI works named in 3.3 (Lapajne 2011; Marinčič et al. 2006;
  Kupferschmid & Helmert 2007; Buro et al. 2009; Cowling et al. 2012; Edelkamp 2021) - queried 2026-10-14; metadata only.
- **07-S19** Stefan Edelkamp, "On the Power of Refined Skat Selection", arXiv 2104.02997 (2021) - `07c/edelkamp_selection.pdf/.txt` (abstract read).
- Also saved and consulted: Furr, "Straw Man Tarock" (tarocchino.com archive and Tarocchi, Philebus 2009, pp. 109-111,
  no strategy) `07d/tarocchino.html(.txt)`, `07d/furr_djvu.txt`; Krüpl's Tarock page `07/krupl_tarock.html` (no
  Strohmandeln content); gist by "sbeyer" `07/gist_sbeyer.md` (a rules draft, no strategy) `[UGC]`.
