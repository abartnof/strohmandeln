# 16 - The skill ladder: what a learner of Strohmandeln climbs

Strand 16. Written 15 October 2026. Input to `docs/TUTOR.md`. Every claim about the game is from a source read for this note; page numbers are of the printed books (the OCR files in `research/sources/03/` carry them). Translations are mine.

Sources used (all read in full for the passages cited):

- **W** Werner, *Das moderne Tarokspiel*, 2nd ed., pp. 129-135 [03-S12]; `sources/03/wb/3752218.txt`.
- **U** Ulmann, *Illustrirtes Wiener Tarokbuch*, 2nd ed. [1899], pp. 69-78 [03-S14]; `sources/03/abo/ulmann1899/ocr.txt`.
- **Be** Bermann, *Der praktische Tarokspieler*, 1894, pp. 142-151 [03-S13]; `sources/03/abo/bermann1894/ocr.txt`.
- **MS** Mayr & Sedlaczek, "Spieltaktische Hinweise", tarockspiele.com (Wayback 29 Jan 2002) [07-S1]; `sources/07d/ms_web_2002.html.txt`. No page numbers (web page); cited by its sub-headings.
- **MF** Macfadyen, "Königrufen - remarks on strategy", pagat.com [07-S5]; **GL** Gams & Lustrek 2003 [07-S6].
- **D** Dummett, *The Game of Tarot*, pp. 457-459 [02-S1]: rules only, no strategy on Strohmandeln (read: `sources/02b/dummett4.txt`).
- pagat.com has no Strohmandeln page [07-S6c]; the English Tarock strategy there (MF) concerns Königrufen and is cited only for trump counting.
- The eleven tactical rules "H1-H11" [12, finding on harbour-tarock; 07-S10] are the spec author's numbering of MS [07-S10]. They are not cited here; MS's own text is.

## Headline findings

1. **The sources give a bag of maxims, not a ladder.** W says of the strategy "Ueber die Behandlung des Spieles ist nur wenig zu sagen" (p. 133) and, of the decisive skill, "darüber lassen sich keine allgemeinen Regeln aufstellen, das kann man nur durch die Uebung erlernen" (p. 135). U: "Eine Anweisung, wie in allen Fällen vorzugehen sei, läßt sich daher nicht geben" (p. 72). The order below is therefore **logical** (what a skill presupposes), not a teachers' sequence from a source, as in Cassino (`~/cassino/docs/DESIGN.md` 12.8).
2. **Where the game is decided, three sources agree**: W "Die größte Rolle ... spielt das Impassiren. Dies allein entscheidet meist über den Ausgang der Partie" (p. 134); Be "Die Entscheidung liegt also meist bei den mittleren Figuren, bei Dame und Cavall der vier Farben" (p. 147), "Durch glückliches Impassiren ... wird oft ein Spiel zu Gunsten desjenigen entschieden, der in den Tarok viel schwächer war" (p. 146); MS "Die Kunst des Strohmandelns besteht darin, ... die richtigen Prioritäten zu setzen". The suit play with the Dame and the Cavall is the capstone. It is also the least rule-able.
3. **Five rules are stated by two or more independent sources, period and modern, nearly in the same words**: play the lying Dame or Cavall at once; play the packets off to reach the cards beneath; lead tarocks from a solid run, of any height; take up with 20-25 points of honours and figures; keep the Pagat out of a thin tarock holding. These are the rules that can be taught as rules (table below).
4. **The sources contradict one another in five places**; the tutor must not state either side as a rule without the measurement of `docs/TUTOR.md` 11 (section "Contradictions" below).
5. **A decision in this game has a cost the advisor can value from the player's view**: the engine's rung 4 values every legal card over sampled worlds in thirds of the count margin, with the ultimo at 12 thirds a Point (`strohmandeln-ai/src/rung4.rs` `move_table`, `sim.rs` `ULTIMO_THIRDS`). The ladder is therefore built from skills that are a **question about a decision** (which class of cards is right here), as in Cassino.

## 1. What is not a skill, and why

| Candidate | Decision | Reason |
|---|---|---|
| Following suit, then trumping, then any card | none | Forced: the engine refuses an illegal card and the table dims it (RULES 5.4). Taught by the tutorial page `following` and the `explain-*` keys. The one free choice, a hand card or a cover of the suit led, "steht die Wahl ... frei" (Be p. 145), belongs to `off`. |
| Counting the cards out | none (as a skill of its own) | It is not a move. W: "ein genaues Tarokzählen und eine große Aufmerksamkeit auf jede einzelne Farbenkarte" (p. 133-134); U: the fallen tarocks, "wie oft jede Farbe gespielt worden", how many a suit the opponent trumped or threw (p. 76); Be: "es ist meist viel besser, den Stand der Farben zu kennen als jenen der Tarok" (p. 150); MS: "nicht nur die gefallenen Tarock zählen, sondern auch ... wie oft jede Farbe gespielt worden ist". It shows only in its consequences: `void` (a lead into a suit he has shown void) and `endgame`. Cassino reached the same conclusion for counting ("shows only through its consequences"). |
| The Cavall feint (answer his king with your cavall, holding a pip) | none | W does not mention it. U: "bereits so abgebraucht, daß es wohl nur ausnahmsweise angewendet werden sollte" (p. 76); Be: "Man muß sich aber wohl hüten, derlei Finten so oft anzuwenden, daß sie zu einer Gewohnheit werden" (p. 149); MS: "so durchsichtig, dass es nur ein oder zwei Mal pro Partie angewendet werden kann". Its value is that a reader is deceived. Rung 4 samples worlds and models no reader (PIMC, [07-S6], [07-S7]), so it cannot value a feint, and "a clear chance" cannot be shown. Never a focus. |
| Mondopfer, Rostopschin, Uhu, Kakadu, Grand Point | none | Not canonical rules (14 rules 22-24). |
| Valat | none | One in a very great many deals (W "eine außerordentliche Seltenheit", p. 133; U "weil ein solcher nie vorkommt", p. 76). |
| Keeping a packet back (`hold`) | parked | The sources give it only as exceptions to playing the packets off: W p. 134 (the ultimo threatened), U p. 75 ("Zugeknöpftbleiben"), Be p. 149, MS ("nie in die Situation kommen, dass dein Gegner schon alle Strohmänner-Karten aufgenommen hat, während du alle deine Handkarten verausgabt hast"). It is a candidate once the measurement shows rung 4 chooses it often enough (`docs/TUTOR.md` 11). |
| Guarding the Mond (keep one or two small tarocks) | parked | U p. 74 ("Auch den Mond, wenn er nicht vom Sküs begleitet ist, suche man rechtzeitig heimzubringen"; keep the smallest tarock to bring the opponent to lead) and MS ("Mondfang": keep one or two small tarocks) agree, but it is a rare position and the canon pays no Mondfang bonus (14 rule 24). Candidate on the same condition. |

## 2. The ladder

Five stages, ten skills. A skill is a **question** about a decision: a partition of the legal cards into classes; the right class is the one the advisor values highest. A **clear chance** exists when the best class beats the next by more than the advisor's noise (`docs/TUTOR.md` 4). Costs are in thirds of the count margin, the advisor's unit; one Point is 12 thirds (`sim::ULTIMO_THIRDS`; its basis is not documented in the repo and is measured, `docs/TUTOR.md` 11, item M2).

| Stage | Skill (slug) | Needs | Source(s) | In one line |
|---|---|---|---|---|
| 1 Tricks | `worth` | | Be 147; MS "Das Nach-hinten-Spielen"; W 134-135 | Take a rich trick; do not spend an honour winning a poor one. |
| 1 Tricks | `cheap` | `worth` | MS "Das Nach-hinten-Spielen", "Die Strohmänner rasch abspielen"; U 75 | Pay with the cheapest card that does the job. |
| 2 Packets | `off` | `worth` | W 134; U 71-75; Be 145, 148; MS "Die Strohmänner rasch abspielen!" | Play the covers off, and a lying Dame or Cavall at once. |
| 3 Tarocks | `run` | `cheap` | U 73-74; MS "Das Nach-hinten-Spielen" | Lead tarocks from a solid run, of any height; with none, a low one. |
| 3 Tarocks | `pagat` | `run` | U 74; MS "Den Pagat fangen/heimbringen", "Der stille Pagat ultimo"; W 133 | Cash the Pagat early with few tarocks; keep it for the last trick only with very many. |
| 4 Suits | `void` | `worth` | MS (intro); Be 150; U 76 | Do not lead a suit he has shown he is out of. |
| 4 Suits | `behind` | `off`, `void` | W 134-135; U 72-73; Be 146-151; MS "Das Nach-hinten-Spielen" | Do not open a suit the two hands share evenly; make him open it. |
| 5 Finish | `endgame` | `cheap`, `void` | U 77; Be 147, 150; GL | When the position is known, count to 35 2/3 and play to the line. |
| The two calls | `bid` | `worth` | W 131; U 77-78; Be 145-146; MS "Wie stark ist dein Blatt?" | Take up with 20-25 points of sure winners and 6-7 tarocks. |
| The two calls | `announce` | | W 133, 135; U 71; Be 146; MS "Die Prämien" | Announce the Troule or the four kings before their first card is played. |

The prerequisites are logical only: a skill needs what its question presupposes. `bid` needs `worth` because the "sure winners" the sources name are the cards that take a trick worth taking. `behind` needs `off` (what lies in the packets is what makes a suit "contested") and `void`. `endgame` needs `cheap` and `void` because in the known position every decision is a choice of the cheapest card and a suit he cannot answer. These are assumptions to be checked by the lesioned students (`docs/TUTOR.md` 11); Cassino found that pairs of knockouts showed the skills to stand in for one another, so worth cannot order them.

### Each skill, in detail

For each: the sources (what they say), the clear chance as a program observes it from the player's view, how met and missed are told, and what the rule can honestly say. "Right class" is the class the advisor values highest.

**`worth` - what a trick is worth.**
- Sources. MS: "Ein Ass oder einen Zehner des Gegners wirst du nicht gleich mit dem König stechen, denn dieser könnte einen fetteren Stich bringen, sei es eine Dame oder einen Reiter. Du wirst also, wenn du nicht mit einer niederen Figur stechen kannst, den König schinden und ein Skartindel zugeben." Be p. 147: "um entscheiden zu können, ob schon ein geringerer Stich vorzuziehen ist oder man auf einen höheren Zählwerth warten solle". Be p. 148: a small loss is not to be feared. Limit (MS): on average 3 cards of a suit lie in the packets at the start, so "nach hinten spielen" must not be overdone, "ansonsten kann es passieren, dass dein König mit Tarock gestochen wird".
- Chance. The player follows; the legal cards include one that beats the card led and one that does not. Classes: {wins the trick, does not}.
- Met/missed. Met if the card played is in the right class. A miss is directional: *greedy* (won, should have let go) or *timid* (let go, should have won).
- Rule. Both directions are sourced; the text differs (rich trick: win it; poor trick: keep the honour).

**`cheap` - which card pays.**
- Sources. MS (above) and "Spielt dein Gegner eine Farbe an, von der du nur Skartindeln ... hast, so bedienst du von den Strohmännern"; U p. 75 (same, "bedient man mit letzteren"); Be p. 148 ("Ueberhaupt wird man einen kleinen Verlust nicht scheuen dürfen"). Includes the free discard when void with no tarock (RULES 5.4 rule 3): U p. 76 and Be p. 149-150 on what the discard gives the opponent.
- Chance. The follower; the right outcome of `worth` is fixed (win or not); at least two cards give it and the advisor's best differs from its worst by more than the margin (Cassino's "Trailing" shape: a spread within one kind of move).
- Met/missed. Met if the card played is within the margin of the best card of its outcome.
- Rule. "Cheapest" is the sources' wording, not a measured fact: the rule ships only if the strongest rung's best card is the cheapest in at least 80% of these chances (`docs/TUTOR.md` 11).

**`off` - the covers off.**
- Sources. W p. 134: "meist im Interesse des Spielers liegt, so rasch wie möglich seinen ganzen Talon aufzudecken". U p. 71: each "möglichst rasch wegzuspielen trachtet"; p. 75: "Es gilt allgemein als ein Vortheil, möglichst rasch die aufliegenden Karten abspielen ... man scheut auch ein kleines Opfer nicht, um baldigst in den Besitz der verdeckten Karten zu gelangen"; the higher card first; the card on a packet before the hand card of the same suit. Be p. 145: "im Interesse jedes Spielers ... die eigenen Päckchen möglichst rasch abzuspielen"; and to hinder the opponent doing so. MS: "Es ist wichtig, dass du deine Strohmänner rasch abspielst, damit du volle Einsicht in alle deine Karten gewinnst."
- The lying figure, three sources: U p. 74 "Liegt eine Figur, Dame oder Cavall, auf einem Päckchen über drei verdeckte Karten, so ist dieselbe nur in seltenen Fällen einzubringen ... man zögere daher nicht, sie wegzuspielen"; Be p. 148 "Liegen Figuren in einer Farbe offen auf, in welcher die Päckchen des Gegners leere Blätter zeigen, so sind sie stets abzuspielen"; MS "Eine auf den Strohmännern aufliegende Dame solltest du sofort wegspielen, auch wenn du nicht den König im Handblatt hast ... (In 4 von 10 Fällen wird es dir gelingen.) Du wirst nur dann darauf verzichten, ... wenn dein Gegner in genau dieser Farbe mindestens 2 niedrige Karten aufliegen hat." The exception agrees: U "es sei denn, daß der Gegner einen oder zwei Scartins ... aufliegen hätte". Of two lying figures play the higher (U p. 74; MS). A figure split between hand and packet: lead the packet's (Be p. 148).
- Chance. A cover and a hand card are both legal (the same suit, or a lead) and the best cover beats the best hand card by more than the margin. Classes: {a card from a packet, a hand card}. Only the cover-is-right direction counts (as Cassino's action skills): where the hand card is right nothing is shown either way.
- Met/missed. Met if a cover was played. The miss carries the cover's class: figure (Dame, Cavall) or pip, which picks the rule text.
- Consequence clause. After the deal every card is public; if the missed figure was later taken by a higher card of its suit, the example may say so ("fell under his king in trick 14").

**`run` - leading tarocks.**
- Sources. U p. 73-74 and MS (almost the same text; both give the same example of a closed run versus a loose holding): "Man fordere stets in der Weise, daß man selbst möglichst viele Stiche in den Taroks erzielt, ziehe daher immer von einer geschlossenen Reihe Taroks, gleichviel welcher Höhe, an; findet sich in den Taroks kein Schluß ... so spiele man einen der niederen Taroks, um mit den höheren später Stiche zu machen." Whether to lead tarocks at all is disputed (Contradictions, 3), so only *which* tarock is a skill.
- Chance. The leader; the best lead overall is a tarock and at least two tarock leads differ by more than the margin. Met if the tarock led is within the margin of the best tarock lead.

**`pagat` - the Pagat.**
- Sources. U p. 74: with ten to twelve tarocks and no Pagat "versuche man ... auf diesen ... Jagd zu machen"; holding the Pagat "neben nur wenigen, etwa nur sieben bis acht Taroks, so unterlasse man nicht, ihn bei erster Gelegenheit einzustechen". MS: with "mehr als 11 Tarock" hunt it; holding it with "7 oder noch weniger" cash it; a silent ultimo is worth speculating on only with more than 11 tarocks, "Aber auch dann wirst du mit Austarockieren nur in den seltensten Fällen zum Ziel kommen. Du brauchst zusätzlich zumindest eine lange Farbe." W p. 131 and U p. 77-78: a weakly guarded Pagat is not counted as a sure winner. The ultimo cannot be announced (W p. 133, U p. 76, Be p. 146, MS).
- Chance. The player holds the Pagat in hand and another legal card; classes: {play the Pagat now, keep it}. Met if the card played is in the right class. The rule is told only when the missed chance lies in a sourced zone (own tarocks at most 7, or at least 12); in between the sources are silent or differ (U 10-12 versus MS 12 and more) and the plain fact is told instead.

**`void` - reading the suits.**
- Sources. MS intro (above); Be p. 150: "Hat z. B. einer der Spieler neben einer mit seinem Gegner getheilten Karte ein einzelnes Blatt einer Farbe, die jener nicht mehr hat, so ist wohl zu überlegen, ob es abzuspielen ist"; U p. 75-76 (the opponent who is out of a suit and sits on his packet hopes for your king; "es klüglich unterlassen, mit dem König herauszukommen"; "Schwache Strohmannspieler lieben es auch, vom Päckchen nicht wegzuspielen, wenn sie in einer Farbe renonce sind"). MS says the same of the weak player.
- Chance. The leader; a suit in which the opponent has shown void (could not follow) and shows no cover (the engine's `Knowledge::opp_void`, which rung 3 uses); classes: {leads that suit, leads another}; the right class is the other by more than the margin. Restraint skill. Met if the lead was not into the void.

**`behind` - the contested suit (Impassiren).**
- Sources. W p. 134-135 (above, and: "Man muß sich hüten, eine Farbe zu bringen, deren Zählkarten zwischen beiden Spielern gleichmäßig vertheilt sind, man muß vielmehr den Gegner zu zwingen suchen, eine solche Farbe anzuspielen"; "In zahlreichen Fällen wird man sich erst von seinen Taroks befreien müssen"). U p. 72-73 ("Es entsteht der Streit um die Farbe, der 'Proceß'. Jeder sucht den Gegner zum Anspielen der kritischen Farbe zu zwingen"; whether to give the Prozess up "muß der Beurtheilung des Spielers überlassen bleiben"). MS: "Wer als erster beginnt, die umkämpfte Farbe zu spielen, wird am Ende eine schlechtere Punktebilanz in dieser Farbe aufweisen"; usually two or three suits at once; "die richtigen Prioritäten". Be p. 147-148: do not lead a suit in which you hold the "Gabel" (king and cavall) alone or with a lying card; do lead where your figures are closed; with a Dame and a Jack in hand and the Cavall on a packet, lead the Cavall (if the king takes it, the other two are free).
- Chance. The leader; the suit is *split* (the counting cards of the suit not yet played are shared about evenly between the two known holdings); classes: {opens a split suit, any other lead}; the right class is the other by more than the margin. Restraint. The share that counts as "evenly" is not given by any source (MS "ziemlich gleichmäßig"); the engine's rung 2 uses 30-70% of the suit's 18 points (`rung2.rs`, a design choice); the tutor's share is measured (`docs/TUTOR.md` 11).
- Rule. W: no general rules. The tutor tells a fact and the example, a rule only if one passes the gate.

**`endgame` - the known position.**
- Sources. U p. 77 (a worked end game at trick 24, "Der Aufnehmer muß die Partie gewinnen, wenn er Pique-Neun anzieht"); Be p. 147 and 150 ("Sehr häufig wird die Entscheidung des Spieles in den letzten Stichen liegen ... Mitzählen des Spieles"; the example with 30 counted: lead a low card, not the king); W p. 133-134. GL: search is "adequate at the end and quite inadequate at the beginning", so the advisor is reliable here.
- Chance. The position is exact: no face-down packet card remains for either player, so the opponent's hand follows by elimination from the log. All legal cards; the best differs from the worst by more than the exact-solve margin. Met if the card played is within that margin of the best.

**`bid` - taking up.**
- Sources. W p. 131: "Hat man 3 Honneurs, 6 bis 7 Taroks und eine oder zwei besetzte Damen, respective Cavalls, so kann man eine Aufnahme schon riskiren. Hat man aber den Pagat schwach mit Taroks besetzt, so kann man nicht darauf bauen". U p. 77-78: "drei gut besetzte Honneurs (nebst einigen Damen)"; four or more honours: unconditionally; "Drei schwach besetzte Honneurs (ohne Damen) sind, namentlich in der Rückhand, zur Aufnahme nicht hinreichend". Be p. 145-146: "Wenn man in Honneurs und höheren Figuren etwa 20 bis 25 Points in der Handkarte zählt, so wird man unbedenklich aufnehmen können". MS: sure winners worth 20-25 points (king, trull card 4 1/3, queen 3 1/3), "knappe 20 Punkte" risky, 25 or more very good; "mindestens 6 bis 7 Tarock"; 12 of your 27 cards lie in the packets (45%).
- Chance. The player is to bid; classes: {take up, pass}; the right class is the one the advisor's bid expectation favours by more than a margin in Points (`rung4::bid_advantage`, in Points, scaled by the deal's multiplier). Met if the bid is in the right class. The rule's numbers are sourced; whether the strongest rung follows them is measured: the book rule bids in about 4-7% of hands (`research/07` 5.4), and the engine's rung 2 deliberately bids lower (`rung2::BID_RULE`, 45 thirds and 5 tarocks, `docs/OPPONENT.md`).

**`announce` - the honours.**
- Sources. W p. 133: held together in hand, announced "spätestens in dem Augenblicke, in welchem er ... eines dieser Honneurs verwendet", paid even if lost; p. 135: with two Trull cards or three kings do not spend them early, "zögere man mit dem Ansagen derselben doch so lange wie möglich, um dem Gegner nicht vorzeitig sein Blatt zu verrathen. Auch kann man die Honneurs ... verschweigen, erhält sie aber dann natürlich nicht bezahlt." U p. 71; Be p. 146; MS "Die Prämien" (same deadline; with three kings hold back, the fourth may yet come from a packet).
- Chance. A card of an honour the player holds, and has not announced, is about to be played (the deadline). Classes: {announce with it, play it silent}. The value of announcing is exact (one Point, times the deal's multiplier); the advisor is not needed. Waiting for the third Trull card or the fourth king is not a chance (the honour is not held yet). Met if announced; missed if the engine's `honour_forfeited` event for the human appears.

## 3. Contradictions among the sources

1. **Who opens the contested suit.** W and MS: whoever opens it ends worse (W p. 134-135; MS). Be p. 150-151, a suit divided between the two with a king and cavall against a queen and jack: "Meist fällt der Gewinn dann jenem Spieler zu, welcher zuerst am Ausspielen ist", because the leader can lead a spare low card and force the other to take with a figure; "die Vertheilung und Verwendung der leeren Karten von Wichtigkeit". The principle depends on the spare low cards. No blanket rule is stated; `behind` ships a fact and an example unless a measured rule passes.
2. **Counting tarocks or suits.** W p. 133-134 and U p. 76 stress counting the tarocks; Be p. 150 ("Wie beim Strohmann-Tarok die Tarok überhaupt in ihrer Bedeutung gegen die Farben zurücktreten, ist es meist viel besser, den Stand der Farben zu kennen als jenen der Tarok") and MS (counting the suits) stress the suits. The review's fallback tip `try-count` ("count the tarocks out as they fall", `docs/MESSAGES.md`) follows only the first pair.
3. **Lead tarocks or shed them.** MS and U: the player stronger in tarocks leads them before opening a split suit (MS "wird zwischendurch tarockieren"). W p. 135: "In zahlreichen Fällen wird man sich erst von seinen Taroks befreien müssen, um im Farbenspiel vortheilhaft agiren zu können". Be p. 146: the tarocks recede ("selten einen anderen ... als den Werth der leeren Stiche"). Only *which* tarock to lead is agreed (U, MS).
4. **The Pagat thresholds.** U: cash it with 7-8 tarocks; hunt it with 10-12. MS: cash it with 7 or fewer; hunt it, and speculate on a silent ultimo, with more than 11 (twelve or more). The engine's hint rule `pagat-last` holds the Pagat from 8 tarocks (`rung2::STRONG_TAROCKS`; `docs/MESSAGES.md` notes "the source's threshold is more than 11 tarocks ... the hint holds from 8"). The summary "Pagat hunt only with 11+ trumps" in `research/12` is MS's "mehr als 11", that is 12 or more.
5. **How sure the Pagat is.** MS count the Pagat as a sure winner ("im Strohmann-Tarock meist heimzubringen"); W p. 131 and U p. 77-78 do not count a weakly guarded one. The `bid` skill's own rule cannot say either without the measurement.

## 4. What the sources do not give

- No frequency of a skill's chances or of its misses in real play; no data on how long learners take (`research/07` section 2: "No published frequency"). The ladder's stages are logical.
- No measure of how well a feint or a bluff works against a human.
- Bermann's and Werner's strategy sections are short (Be pp. 146-151, W pp. 133-135); U's are the fullest period text (pp. 69-78) with two worked end games (II. Teil, problems 21-26, not read for this note).
- *Die Strategie des Tarockspiels* (Mayr & Sedlaczek 2010; chapters on Mondfang, Pagatkassa, Strohmandeln pp. 369-372) was seen only as a table of contents [07-S8].

## 5. Sources read for this note (local copies under `research/sources/`)

W `03/wb/3752218.txt`; U `03/abo/ulmann1899/ocr.txt` (pp. 69-78); Be `03/abo/bermann1894/ocr.txt` (pp. 143-151); MS `07d/ms_web_2002.html.txt`; D `02b/dummett4.txt` (pp. 457-459); MF `07/pagat_krufsk.html.txt`; GL `07c/gl.txt`. Engine facts: `crates/strohmandeln-ai/src/{rung2,rung4,sim,hint,review,explain}.rs`, `docs/{RULES,OPPONENT,MESSAGES,PROTOCOL,TABLE3D}.md`. The Cassino model: `~/cassino/docs/DESIGN.md` 12.4, 12.8, `crates/cassino-core/src/{learner,tutor,brief,lessons}.rs`, `measurements/README.md`.
