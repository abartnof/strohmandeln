# The tutor: specification

> Status: **proposed**, 15 Oct 2026. Specifies the post-game tutor that replaces
> the heuristic `try` bullet of the end-of-deal review. Nothing here is built.
> Layout: desktop. The phone layout is deferred (maintainer, 15 Oct 2026).

The tutor finds the one stage of the game the player is ready for next, says one
rule and one or two things to try, and shows this deal's example. It is gentle,
short and never a marking. It is Cassino's tutor (`~/cassino/docs/DESIGN.md`
12.4 and 12.8, `crates/cassino-core/src/{learner,tutor,brief,lessons}.rs`) built
for this game: the same model, a new ladder (`research/16-skill-ladder.md`), a new
evidence, new words. The theory is a plain zone of proximal development: the
next skill whose prerequisites are cleared and where the player is surely short;
no ability model (Cassino 12.8 gives the reason: a difficulty model fitted to
the computer's rungs "would recover how the rungs were written").

Terms: a **skill** is a question about a decision (a partition of the legal
cards into classes; the advisor says which class is right). A **clear chance**
is a decision where the right class beats the next by more than the advisor's
noise. **Met** and **missed** say whether the player's card was in the right
class. The **advisor** is rung 4 of the opponent, run on the player's own view
(`docs/OPPONENT.md`, `rung4::move_table`). **Cost** is thirds of the count
margin (1 Point = 12 thirds, `sim::ULTIMO_THIRDS`).

## 1. What is kept from Cassino, and where this departs

| Cassino | Here | Why |
|---|---|---|
| Evidence, learn, nudge as three pure functions | Kept | The learner is a pure function of stored evidence, so a change to a detector rescores the history (Cassino 12.8). |
| Unit: a game | **A scored deal** | A deal has about 14 decisions with a choice; a sitting (6 or 12 deals) varies in length and is too coarse to fold. |
| Clear chance: best move using the skill against best not | Kept, generalised to a partition of the cards in two or more classes (`worth`: wins / does not) | Two of the questions have no "using the skill" side. |
| Skills judged only by the advisor | Two are not: `announce` (exact, one Point) and `bid` (in Points) | Their value is exact or already in Points. |
| Prose from the engine | **Message keys with parameters**, English and German in the table | Project rule (bilingual: engine keys, not prose). |
| Focus: largest gain among skills with cleared prerequisites | Kept; exact ties go to the earlier stage | The prerequisites carry the order; the stage only breaks ties. |
| Long prose review replaced by the brief | The brief from the start | Cassino wrote the prose first and replaced it: "what people read" is a few bullets (12.4). |
| `hint` and `nudged` are commands that mark the decision | The table keeps the list of assisted decisions and passes it in (section 3) | `strohmandeln_hint` is a pure query (`docs/PROTOCOL.md`). |
| Heavy calls on the main thread, then a Worker | **A Worker from the start** | The evidence is seconds a deal, not milliseconds (section 11, M7). |
| Rules measured: precision lower bound 0.80, confirmed on fresh seeds | Kept | Section 11, M5. |
| Constants set from the strongest rung's games and lesioned students | Kept; none invented here | Section 11. |

## 2. The skills

Ten skills in five stages and two "calls". Sources, one-line statements and the
reasons for what is left out are in `research/16-skill-ladder.md`; this table is
the engine's. A slug is the wire name.

| Stage | Skill (slug) | Needs | The question (classes of the legal cards) | Right class when | Met | Cost of a miss |
|---|---|---|---|---|---|---|
| 1 | `worth` | | The follower: {beats the card led, does not} | The class the advisor values higher by more than the margin | The card played is in it | The gap |
| 1 | `cheap` | `worth` | The follower, outcome fixed: spread among the cards that give it | At least two cards give it and best less worst exceeds the margin | The card played is within the margin of the best | Best less the card's value |
| 2 | `off` | `worth` | {a card from a packet, a hand card} | A packet card is best by more than the margin | A packet card played | The gap |
| 3 | `run` | `cheap` | The leader, best lead a tarock: spread among tarock leads | At least two tarock leads differ by more than the margin | Within the margin of the best tarock lead | As `cheap` |
| 3 | `pagat` | `run` | Holding `T1` and another legal card: {play it now, keep it} | The advisor's class, by more than the margin | The card played is in it | The gap |
| 4 | `void` | `worth` | The leader: {leads a suit he has shown void, leads another} | Another lead is best by more than the margin | Not into the void | The gap |
| 4 | `behind` | `off`, `void` | The leader: {opens a split suit, any other lead} | Another lead is best by more than the margin | Did not open a split suit | The gap |
| 5 | `endgame` | `cheap`, `void` | Exact position (no face-down packet card left for either player): all legal cards | Best less worst exceeds the exact-solve margin | Within the margin of the best | Best less the card's value |
| call | `bid` | `worth` | To bid: {take up, pass} | The advisor's bid expectation (Points, `rung4::bid_advantage`) favours a class by more than a margin in Points | The call is in it | The expectation gap, times 12 |
| call | `announce` | | About to play a card of a held, unannounced honour: {announce with it, play it silent} | Announce (one Point, exact) | Announced; missed = the human's `honour_forfeited` event | 12 thirds times the deal's multiplier |

Definitions the table leans on:

- **Shown void**: `Knowledge::opp_void` for the suit, and no cover of his and no
  card of his known from a pull shows the suit (the counter rung's own
  inference, so the tutor judges with what rung 3 knows).
- **Split suit**: the counting cards of the suit not yet played are shared
  about evenly between the two known holdings. The share is measured (section 11,
  M8); the engine's rung 2 uses 30-70% of the suit's 18 points, a design choice.
- **Exact**: both players' face-down packet cards are all turned, so the
  opponent's hand follows from the log by elimination.
- A card played from a packet is a **cover** played (`source` "packet n").
- One decision can be a chance at several skills, each judged separately. A
  skill's cost is never added to another's.
- **Directional misses** (kept for the words, not the learner): `worth` is
  *greedy* (won a poor trick) or *timid* (let a rich one go); `off` is *figure*
  (a Dame or Cavall lay on) or *pip*; `pagat` is *cash* or *keep*; `bid` is
  *passed* or *took*.
- **Settings**: a rule is told only under the settings it was confirmed under
  (`values`, `both_pass`, `ultimo`; Cassino 12.8, `lessons::taught`). `bid` and
  `pagat` depend on them; the rest do not.

## 3. Evidence: one deal, a record, the assisted decisions

**Replay.** The engine replays a deal from a **deal record** and gives, at each
decision of the human, the view (`view::view`), the action taken and whether it
was assisted. Everything after is a function of the record.

A deal record is: the 54 cards in deal order, the dealer, the doubling in force,
the three settings, and the deal's actions in the short form of the protocol
(about 1 KB). It needs one engine addition: replaying a deal from its deck
(`Deal::new(deck, dealer, multiplier, settings)` exists; `Sitting::with_decks`
is `pub(crate)`). A sitting `Record` (seed, settings, every move) is too large to
keep per deal and replays only from its start.

**Assisted.** A decision is *assisted* when a hint was shown for it
(`strohmandeln_hint`) or the tutor's nudge was (section 7). Assisted decisions
are not evidence: teaching would otherwise spoil the measuring. The table keeps
the list for the deal as the numbers of the human's actions (action *n* of the
deal) and stores it with the record; the engine takes it as an input. Asking for
a hint stays free and changes nothing in the game; `docs/TABLE3D.md` 6.3.2 says
asking "is never counted" and is amended to "is not counted as evidence".

**Summary** (the text the learner reads), one per scored deal:

```
strohmandeln evidence v1
worth 7 5 9
cheap 3 3 0
off 2 1 4
...
announce 0 0 0
```

A line is `<slug> <chances> <met> <missed thirds>`. The version is
`EVIDENCE_VERSION`: bumped when a detector, the margin or the advisor changes,
so that a stored summary made before is recomputed from its record (Cassino
12.8). The deal's **misses** are not stored: the example of the brief is worked
out from the record when asked.

## 4. Clear chances

For a decision with at least two legal cards the engine takes the advisor's
values of the legal cards in thirds (`move_table`, the same sampled worlds for
every card, deterministic from the view as the hint is, `docs/OPPONENT.md`). For
each skill whose question applies it works out the best value of each class.

- **Margin.** A chance needs the best class to beat the next by more than
  `MARGIN`, the advisor's noise. In an exact position the advisor is exact and
  the margin is the least loss the review counts. Near-ties are no evidence
  either way. `MARGIN` is measured (M1), not set here.
- **Action skills** (`off`, and `worth` toward winning) count only where the
  skill's class is right. Where the other class is right, nothing is shown, as in
  Cassino. `worth` and `pagat` and `bid` have two right classes and count both.
- **Spread skills** (`cheap`, `run`, `endgame`): the best card against the worst
  of the same kind; met if the card played is within `MARGIN` of the best.
- **Deterministic skills.** `announce` needs no advisor; `bid` takes the
  expectation of `bid_advantage` over its own sampled worlds (40, `Budget`) and
  scales its margin by the deal's multiplier (the opponent's own fixed margin
  does not, `docs/review/opponent.md`).
- **A skill's rule applies** to a missed chance only when the chance lies in the
  rule's trigger; otherwise the plain fact is told (section 8). Example: the
  `pagat` rule ("seven or fewer", "twelve or more") applies at the tarock counts
  the sources state; between them (8-11) they differ (`research/16`, 3.4), and
  the fact is told.

## 5. The learner

A fold over the stored deals' summaries, oldest first, as Cassino's `learn`:
what is known after deal *n* depends on what was known after *n - 1*.

- **Discount.** Each deal's counts weigh `DISCOUNT` times the next deal's.
- **Beta posterior.** After discounted chances *c* and met *m*, the met rate has
  the posterior Beta(1 + m, 1 + c - m).
- **Mastered** when the `LOWER` quantile of the posterior reaches `master(skill)`
  (the rate is surely that high); **lost** only when the `UPPER` quantile falls
  below `lose(skill)` (surely lower now). Between the two the state stays: no
  flicker. With fewer than `MIN_CHANCES(skill)` discounted chances nothing changes.
- **Focus** (the zone of proximal development): the skill that is
  1. not mastered;
  2. has its prerequisites **cleared** (mastered, or gaining less than `MIN_GAIN`
     because its chances rarely come up, or not shown to be short);
  3. is **surely short** (the `UPPER` quantile of its met rate below
     `master(skill)`), or **costly** (a gain of `COSTLY_GAIN` over at least
     `COSTLY_CHANCES`);
  4. of those, has the largest **gain**: the discounted cost per deal less
     `top_cost(skill)`, what the strongest rung itself loses to the skill; exact
     ties go to the earlier stage.
  `None` when no skill qualifies. Then, if every skill seen is mastered, the
  tutor suggests the next opponent up (the level dial, `docs/OPPONENT.md`).
- **Just mastered** lists the skills that became mastered with the latest deal.

The constants, from Cassino's names, and what sets each (nothing is invented
here; Cassino's values are starting points only, to be re-tested):

| Constant | Meaning | Set by |
|---|---|---|
| `MARGIN`, exact margin | Advisor noise in thirds | M1 |
| thirds per Point | Exchange rate between the margin and the payment | M2 |
| `DISCOUNT` (per deal) | Recency | M3: a window of about a sitting, checked against M6 |
| `MIN_CHANCES(skill)` | Chances before a rate counts | M3: chances a deal per skill |
| `top_rate(skill)`, `top_cost(skill)` | What the strongest rung meets and loses | M3 |
| `master(skill)` = `MASTER_OF_TOP` x `top_rate`; `lose(skill)` = `master` - `LOSE_BELOW` | Mastery with hysteresis | M3, M6; Cassino: 0.85 and 0.2 |
| `LOWER`, `UPPER` | Quantiles for mastery and for "surely short" | M6; Cassino: 0.2 and 0.9 |
| `MIN_GAIN`, `COSTLY_GAIN`, `COSTLY_CHANCES` | When a skill is worth a focus | M3, M4, M6 |

The rare skills (`announce`, `pagat`, `bid`) will have fewer than one chance a
deal and cannot be certified in a sitting from their chances alone (Cassino saw
the same for its rarest). `announce` and `bid` carry no advisor noise or little,
so their `MIN_CHANCES` may be set lower; M3 decides, from the chances a deal.

## 6. The brief

After each scored deal, and at the sitting's end, at most **three bullets and
about 45 words**. How it is worked out is a link, never in the bullets. Each
bullet is `{lead, key, params}` (section 9).

1. **What went well** (at most 12 words): a skill just mastered (only after at
   least two earlier deals; after one a mastery is luck of the draw), else a
   strength of this deal from the unassisted decisions: all chances at a skill
   met, the honour announced, the Pagat ultimo won, the Mond caught by the Sküs
   (the last two carry luck and are only ever praise). Never the skill that is
   the focus.
2. **Next: the skill, and its rule** (at most 18 words): the focus's name and the
   rule of at most 15 words, chosen by the direction of this deal's costliest
   miss when there is one. The rule is the first suggestion. Where no rule has
   passed its gate, one plain fact in its place.
3. **This deal** (at most 18 words): the costliest miss at the focus this deal,
   with the card played and the better one, in the kibitzer's voice ("This deal
   your Q of hearts fell under his king in trick 14; play it at once next time");
   one try from the skill's two (a second suggestion) when the deal holds no miss.
   Where the consequence is in the log (a figure lost to a higher card of its
   suit) the clause says so; after the last trick every card is public.

No focus: bullet 2 says that nothing stood out (and to try the next opponent up,
below the top of the dial), or that it is too early to say; bullet 3 is dropped.
A skill is never praised and worked on in the same brief. A deal with fewer than
`FEW` decisions with a choice says so in one bullet and nothing more. The tone
follows the kindness rule: luck, speed, the score and the run of play are fair;
nothing about the person. The strings pass the phrase bank's kindness test
(`tools/phrases.py`).

**The sitting's brief** (at the sitting's end, the "Review the sitting" chip) is
the same function over the sitting's deals as one game: the strongest strength,
the focus and its rule, the costliest miss of the sitting at the focus. It is
shown beside the sitting's tally.

**Teaching is two touches** (Cassino 12.8): the rule in the brief, and in the next
deal one **nudge** at the first clear chance at the focus (the nudge words,
section 9), shown in the Kibitzer's tab; that decision is then assisted and does
not count. A muted tutor shows no brief and no nudge.

**Mute and setting.** "Tips from the tutor" (default on) off sets no focus and
shows no nudge; the brief is still offered, with its first bullet only (the
review as before).

## 7. Relation to `review.rs`

`strohmandeln-ai/src/review.rs` today builds three bullets from one deal's
events: `count`, `decided`, `try`. Decision: **extend, keep the facts, replace
the advice.**

| Part | Decision |
|---|---|
| `count` (the count against 107) | Kept as the dialog's header line, not a bullet. It is a result, not advice, and the HUD and slate show it too. |
| `decided` (the moment the deal turned: ultimo, Mond, richest trick lost or won) | Kept as a source of *praise* (ultimo won, Mond caught) and of the example when the focus has no miss. A Valat is added to it (open item, `docs/review/tutor.md` finding 2). The "rich trick lost" is not shown unprompted, as a loss with no lesson. |
| `try` (`try-pagat`, `try-announce`, `try-count`) | **Replaced** by the focus. `try-pagat` and `try-announce` become the `pagat` and `announce` rules and facts. `try-count` goes: it follows the tarock-counting pair of sources only (`research/16`, 3.2). |
| `sitting-tally` | Kept as the sitting dialog's header. |
| `explain` (the line under the prompt after a trick) | Unchanged. |
| `review.rs` code | `read` and `decided` stay; `try_this` and the sitting's "most frequent try" are removed. The brief, learner and skills are new modules in `strohmandeln-ai` (`tutor.rs`, `learner.rs`, `brief.rs`, `lessons.rs`), as Cassino's. |
| `progress.js` | The tally-per-sitting store is replaced by the tutor's store (section 8); its pattern (versioned key, try/catch, a newer store never overwritten, bounded import) is kept. The old key `strohmandeln.progress` is left in place and unread. |

## 8. Storage

`strohmandeln.tutor`, version 1, every access in a try/catch; with no storage
everything works as one deal (the brief then rests on this deal alone).

```json
{"v": 1, "deals": [{"record": "<deal record text>", "assisted": [3, 4],
                    "summary": "<evidence text, or null>", "at": 1760000000}]}
```

- The last **60 deals** are kept (about ten sittings; the figure follows M3 and
  the learner's memory). Each stores its record and, once the ending has
  settled, its summary.
- Bounded and checked: a record of at most 20 KB; a summary only if it reads as
  the engine's evidence text; an import file of at most 1 MB.
- The summary is computed when the ending has settled and recomputed from the
  record when the engine calls it stale. A change to the evidence never drops a
  deal. A record the engine can no longer restore is dropped after one failed
  recompute. A summary of a *newer* version is left as it is. A store written by
  a newer page is read as empty and never written over.
- Settings: "Your progress": Export (JSON), Import (a bad file refused in a
  sentence, changing nothing), Clear. Tests must not write the real key (open
  item, `PLAN.md`): the store is injected.

## 9. Words

The engine emits keys with parameters (`docs/MESSAGES.md`, "Tutor keys", to be
added with the code). Cards are codes, thirds integers, honours `trull` or
`kings`; the table names them. Rules are at most 15 words, tries at most 12,
nudges at most 10, examples at most 18 (a test holds this, as for Cassino's
nudges). The voice is the Kibitzer's (`docs/TABLE3D.md` 6.5): dry, brief, kind,
"if I may". `tutor.lead.rule` is the prefix "If I may:" / "Wenn ich mir erlauben
darf:", not counted in the rule's words.

**Leads and fixed lines**

| Key | English | Deutsch |
|---|---|---|
| `tutor.lead.well` | Went well: | Gut gelaufen: |
| `tutor.lead.next` | Next: {skill}. | Als Nächstes: {skill}. |
| `tutor.lead.deal` | This deal: | In dieser Partie: |
| `tutor.lead.rule` | If I may: | Wenn ich mir erlauben darf: |
| `tutor.well.mastered` | {skill}: mastered. | {skill}: gemeistert. |
| `tutor.well.met` | You met {met} of {chances} chances at {skill}. | Sie nutzten {met} von {chances} Gelegenheiten: {skill}. |
| `tutor.none.early` | A deal or two more, and there will be more to say. | Noch eine Partie oder zwei, dann gibt es mehr zu sagen. |
| `tutor.none.clean` | Nothing stood out to work on. | Nichts fiel auf, woran zu arbeiten wäre. |
| `tutor.none.up` | Nothing stood out. Try the next opponent up. | Nichts fiel auf. Versuchen Sie den nächsthöheren Gegner. |
| `tutor.short` | This deal gave you no real choices to review. | Diese Partie ließ Ihnen kaum Wahl, die sich besprechen ließe. |
| `tutor.how` | How is this worked out? | Wie kommt das zustande? |
| `tutor.method` | Your cards were compared afterwards with what the strongest computer player would have played in your place, knowing only what you knew. Only clear differences count, and cards played after a hint or a tip do not. Each skill is judged over your recent deals, the newest counting most; the one shown is the next on the ladder you are surely short of. Each rule was measured against that player's own play before it was added. | Ihre Karten wurden nachträglich mit dem verglichen, was der stärkste Computerspieler an Ihrer Stelle gespielt hätte, wenn er nur wüsste, was Sie wussten. Nur deutliche Unterschiede zählen, Karten nach einem Hinweis oder Tipp nicht. Jede Fertigkeit wird über Ihre letzten Partien beurteilt, die neuesten zählen am meisten; gezeigt wird die nächste auf der Leiter, bei der Sie sicher noch fehlen. Jede Regel wurde am Spiel dieses Spielers gemessen, ehe sie aufgenommen wurde. |

**Per skill: name, rule (a rule ships only if it passes the gate, M5; the fact
is told in its place), tries, nudge.** Each rule's basis is `research/16`, the
skill's entry.

| Skill | Key | English | Deutsch |
|---|---|---|---|
| `worth` | name | what a trick is worth | der Wert des Stichs |
| | rule.rich | A rich trick is worth winning: take it. | Ein reicher Stich lohnt sich: holen Sie ihn. |
| | rule.poor | Do not win a poor trick with a king; give a small card. | Einen armen Stich nicht mit einem König holen; ein Skartindel zugeben. |
| | try.1 | Before you follow, ask what the cards on the table are worth. | Fragen Sie vor dem Zugeben, was die Karten auf dem Tisch wert sind. |
| | try.2 | The king can wait for a queen or a cavall. | Der König kann auf eine Dame oder einen Cavall warten. |
| | nudge | A trick worth a thought is on. | Ein Stich zum Nachdenken liegt auf dem Tisch. |
| `cheap` | name | the cheapest card | die billigste Karte |
| | rule | Win with the lowest card that wins; otherwise give the lowest. | Mit der kleinsten Karte stechen, die gewinnt; sonst die kleinste zugeben. |
| | try.1 | Compare your two cheapest winners before you pick. | Vergleichen Sie Ihre zwei billigsten Stecher, ehe Sie wählen. |
| | try.2 | When nothing wins, give the card that costs least. | Gewinnt nichts, die Karte zugeben, die am wenigsten kostet. |
| | nudge | Which card pays matters here. | Welche Karte zahlt, ist hier wichtig. |
| `off` | name | playing the packets off | die Päckchen abspielen |
| | rule.pip | Play the face-up packet cards off; beneath may lie tarocks or kings. | Die offenen Paketblätter abspielen: darunter liegen vielleicht Tarock oder Könige. |
| | rule.figure | A Dame or Cavall lying on a packet goes at once. | Eine offene Dame oder ein Cavall geht gleich. |
| | try.1 | Look at the packet cards before the hand cards. | Sehen Sie zuerst die Paketblätter an, dann die Handkarten. |
| | try.2 | Exception: he shows two low cards of the same suit. | Ausnahme: Er zeigt zwei niedrige Karten derselben Farbe. |
| | nudge | A packet card could go here. | Ein Paketblatt könnte hier gehen. |
| `run` | name | leading tarocks | Tarock anspielen |
| | rule | Lead tarocks from a solid run, whatever its height; with none, a low one. | Tarock aus geschlossener Reihe anspielen, gleich welcher Höhe; ohne Reihe ein kleines. |
| | try.1 | Look for your longest unbroken run before you lead a tarock. | Suchen Sie die längste geschlossene Reihe, ehe Sie Tarock anspielen. |
| | try.2 | Keep your smallest tarock back to bring him in. | Das kleinste Tarock zurückbehalten, um ihn ans Spiel zu bringen. |
| | nudge | Which tarock you lead matters. | Welches Tarock Sie anspielen, ist wichtig. |
| `pagat` | name | the Pagat | der Pagat |
| | rule.cash | With seven tarocks or fewer, cash the Pagat at the first chance. | Mit höchstens sieben Tarock den Pagat bei erster Gelegenheit einstechen. |
| | rule.keep | With twelve tarocks or more, the Pagat is worth keeping for the last trick. | Mit zwölf und mehr Tarock lohnt es, den Pagat für den letzten Stich zu behalten. |
| | fact | The Pagat is the lowest tarock; in the last trick it wins or loses a Point. | Der Pagat ist das niedrigste Tarock; im letzten Stich gewinnt oder verliert er einen Point. |
| | try.1 | Count your tarocks before you decide about the Pagat. | Zählen Sie Ihre Tarock, ehe Sie über den Pagat entscheiden. |
| | try.2 | A thinly guarded Pagat is not a sure trick. | Ein schwach besetzter Pagat ist kein sicherer Stich. |
| | nudge | Mind the Pagat here. | Achtung auf den Pagat. |
| `void` | name | his empty suits | seine leeren Farben |
| | rule | Lead no suit he has shown he is out of; he would trump it. | Keine Farbe anspielen, in der er schon renonce ist; er sticht sie. |
| | try.1 | Note which suit he could not follow. | Merken Sie sich, welche Farbe er nicht bedienen konnte. |
| | try.2 | Lead a tarock or another suit first. | Spielen Sie erst Tarock oder eine andere Farbe. |
| | nudge | Mind the suit he is out of. | Achtung auf die Farbe, in der er fehlt. |
| `behind` | name | the contested suit | die umkämpfte Farbe |
| | fact | Whoever opens a suit the hands share evenly usually ends worse in it. | Wer eine gleichmäßig geteilte Farbe eröffnet, steht darin meist am Ende schlechter. |
| | try.1 | Lead tarocks or a closed suit and make him open it. | Spielen Sie Tarock oder eine geschlossene Farbe und lassen Sie ihn eröffnen. |
| | try.2 | Spare low cards in the suit can change this. | Überzählige kleine Karten in der Farbe können das ändern. |
| | nudge | Mind which suit you open. | Achten Sie darauf, welche Farbe Sie eröffnen. |
| `endgame` | name | the last tricks | die letzten Stiche |
| | rule | Late in the deal, count what you still need to reach 35⅔. | Gegen Ende zählen: Was fehlt noch auf 35⅔? |
| | try.1 | Add up your take and his; see who needs what. | Zählen Sie Ihre und seine Stiche zusammen; wer braucht was? |
| | try.2 | Count first; sometimes a low card wins what the king would not. | Erst zählen; manchmal gewinnt eine kleine Karte, was der König nicht gewinnt. |
| | nudge | The position is known now: count. | Die Lage ist jetzt bekannt: zählen. |
| `bid` | name | taking up | das Aufnehmen |
| | rule | Take up with 20 to 25 points of sure winners and 6 or 7 tarocks. | Aufnehmen mit 20 bis 25 Augen sicherer Stecher und 6 bis 7 Tarock. |
| | fact | Sure winners: the Sküs, the Mond, kings, guarded queens. | Sichere Stecher: Sküs, Mond, Könige, besetzte Damen. |
| | try.1 | Add up your sure winners: Sküs, Mond, kings, guarded queens. | Zählen Sie Ihre sicheren Stecher: Sküs, Mond, Könige, besetzte Damen. |
| | try.2 | Count your tarocks too; few make the game risky. | Zählen Sie auch Ihre Tarock; wenige machen das Spiel riskant. |
| | nudge | Look at your sure winners before you speak. | Prüfen Sie Ihre sicheren Stecher, ehe Sie sprechen. |
| `announce` | name | announcing | das Ansagen |
| | rule | Announce the Troule or the four kings before you play their first card. | Troule oder vier Könige ansagen, ehe deren erste Karte gespielt wird. |
| | try.1 | Press Announce before the first card of the Troule or kings. | Drücken Sie "Ansagen", ehe die erste Karte der Troule oder der Könige fällt. |
| | try.2 | With two of three you may wait; the third may come from a packet. | Mit zwei von dreien dürfen Sie warten; die dritte kann aus einem Paket kommen. |
| | nudge | You hold an honour: announce it. | Sie halten einen Honneur: sagen Sie ihn an. |

**Examples** (the third bullet; the card names are the table's, the better card
and the played card are the miss's). The trick numbers are of the deal.

| Key | Parameters | English | Deutsch |
|---|---|---|---|
| `tutor.miss.worth.poor` | `trick`, `played`, `better`, `thirds` | Trick {trick}: your {played} won only {thirds}; {better} would have kept it for a richer one. | Im {trick}. Stich holte Ihr {played} nur {thirds}; mit {better} hätten Sie es für einen reicheren aufgehoben. |
| `tutor.miss.worth.rich` | `trick`, `led`, `better`, `thirds` | Trick {trick}: {led} was rich ({thirds}); {better} would have taken it. | Im {trick}. Stich war {led} reich ({thirds}); {better} hätte ihn geholt. |
| `tutor.miss.cheap` | `trick`, `played`, `better` | Trick {trick}: {better} would have done as well as {played}, and costs less. | Im {trick}. Stich hätte {better} so viel geleistet wie {played} und kostet weniger. |
| `tutor.miss.off.pip` | `trick`, `better` | Trick {trick}: {better} from a packet would have shown you what lies beneath. | Im {trick}. Stich hätte {better} vom Päckchen gezeigt, was darunter liegt. |
| `tutor.miss.off.figure` | `card`, `king`, `late` | This deal your {card} fell under his {king} in trick {late}; play it at once next time. | In dieser Partie fiel Ihre {card} im {late}. Stich unter seinen {king}; spielen Sie sie das nächste Mal gleich. |
| `tutor.miss.off.figure.plain` | `trick`, `better` | Trick {trick}: your {better}, lying on a packet, could have gone at once. | Im {trick}. Stich hätte Ihre {better} vom Päckchen gleich gehen können. |
| `tutor.miss.run` | `trick`, `played`, `better` | Trick {trick}: you led {played}; {better} was the better tarock to lead. | Im {trick}. Stich spielten Sie {played} an; {better} war das bessere Tarock. |
| `tutor.miss.pagat.cash` | `trick`, `n` | Trick {trick}: with {n} tarocks the Pagat should have gone at once. | Im {trick}. Stich hätte der Pagat mit {n} Tarock gleich gehen sollen. |
| `tutor.miss.pagat.keep` | `trick`, `n` | Trick {trick}: with {n} tarocks the Pagat was worth keeping. | Im {trick}. Stich lohnte es mit {n} Tarock, den Pagat zu behalten. |
| `tutor.miss.void` | `trick`, `played`, `suit` | Trick {trick}: you led {played}, though he had shown he was out of {suit}. | Im {trick}. Stich spielten Sie {played} an, obwohl er {suit} schon nicht bedient hatte. |
| `tutor.miss.behind` | `trick`, `suit`, `better` | Trick {trick}: you opened {suit}, which the hands share evenly; {better} was better. | Im {trick}. Stich eröffneten Sie {suit}, das sich gleichmäßig verteilt; {better} war besser. |
| `tutor.miss.endgame` | `trick`, `played`, `better` | Trick {trick}: with the position known, {better} was the card, not {played}. | Im {trick}. Stich war bei bekannter Lage {better} die Karte, nicht {played}. |
| `tutor.miss.bid.pass` | `sure`, `n` | You passed with {sure} in sure winners and {n} tarocks; in most sampled deals taking up paid. | Sie passten mit {sure} sicheren Stechern und {n} Tarock; in den meisten durchgespielten Austeilungen lohnte sich das Aufnehmen. |
| `tutor.miss.bid.take` | `sure`, `n` | You took up with {sure} in sure winners and {n} tarocks; in most sampled deals passing paid. | Sie nahmen auf mit {sure} sicheren Stechern und {n} Tarock; in den meisten durchgespielten Austeilungen lohnte sich das Passen. |
| `tutor.miss.announce` | `card`, `honour` | You played {card} without announcing {honour}: a Point lost. | Sie spielten {card}, ohne {honour} anzusagen: ein Point ging verloren. |

Suit names in `suit` follow the language (`suitName`, `web3d/src/review.js`).
All of these are the Kibitzer's; none judges the person, only the card.

## 10. Wasm calls

In the style of `docs/PROTOCOL.md`. Each takes text through the alloc buffer and
renders JSON to the out buffer. Where a call takes summaries it takes them
oldest first, a line `--` between. The engine holds the judgement and the keys;
the client holds the text and the words. Calls that work out evidence are
stateless (`_of`) so a Worker with an instance of its own can run them.

| Export | Takes | Returns |
|---|---|---|
| `strohmandeln_deal_record()` | | The record text of the deal just scored (`null` before one is). The table stores it. |
| `strohmandeln_evidence_of(len)` | A deal record, a line `==`, the assisted action numbers | `{summary}` or `{error}`. The advisor runs at every unassisted decision with a choice. |
| `strohmandeln_learner(len)` | Summaries | `{deals, focus, mastered, just_mastered, stale}`: current summaries counted, the focus (a slug or `null`), slugs mastered, indexes to recompute with `evidence_of`, and `version`. A summary of a newer version is listed stale but is not to be recomputed back. |
| `strohmandeln_brief_of_record(len)` | A record, `==`, the assisted numbers, `--`, the history | `{bullets: [{lead, key, params}], method: "tutor.method", summary}`, or `null` (a deal not finished). One advisor pass serves evidence, misses and brief. |
| `strohmandeln_brief_sitting(len)` | The sitting's deals as records and numbers, `--`, the history | The same shape, over the sitting. |
| `strohmandeln_set_focus(len) -> 0/1` | A slug (empty: none) | Sets the focus for the nudge. |
| `strohmandeln_nudge()` | | `{skill, key}` or `null`: whether this decision is a clear chance at the focus (one advisor run, asked once per position, keyed like the hint). |

The hint (`strohmandeln_hint`) is unchanged. The page asks for `evidence` and the
brief once the deal's ending has settled, off the table's thread (`docs/TABLE3D.md`
6.4.3, now required), and sends nothing to the engine for a hint shown: it adds
the action number to its assisted list.

## 11. Measurements, each planned before its first batch

All are in the style of Cassino's `measurements/README.md`. Comparisons use
mirrored pairs (each deal played twice, seats swapped), sequential batches with
O'Brien-Fleming boundaries, the comparison, batch size, maximum looks and
boundary fixed before the first batch, stopping as soon as the effect crosses or
is plainly too small, reporting the interval at the last look (`CLAUDE.md`;
`strohmandeln-ai/src/stats.rs`, `tournament.rs`). A computed number a document
relies on has a test or a check that reproduces it.

| # | Measurement | Gives |
|---|---|---|
| M1 | **Advisor noise.** Positions from rung 4's own sittings; each skill's gap re-evaluated under independent seeds; the standard deviation across seeds by skill. Descriptive, a fixed count. | `MARGIN`; the exact margin |
| M2 | **Thirds per Point.** From rung 4's own deals, the final margin in thirds against the net Points of the deal. The engine's 12 thirds a Point has no basis in the repo. | The exchange rate for costs and for `bid` |
| M3 | **What the strongest rung meets.** Rung 4 against rung 3, both seats, 30 deals a rule set: chances a deal, met rate and points lost, by skill. | `top_rate`, `top_cost`, `MIN_CHANCES`, the window |
| M4 | **What a skill is worth.** Rung 4 with one skill knocked out against itself whole. Action skills are knocked out by withholding the class (never a cover; the Pagat played at random); restraint skills by choosing as if blind to them (`void`, `behind`); `bid` by always passing and by random; `announce` by never; `endgame` by playing rung 2 from the first exact position. Pairs of knockouts check the prerequisites. | `MIN_GAIN`, `COSTLY_GAIN`; the prerequisites |
| M5 | **Rule precision.** For each rule, the trigger and the prescription as tests of the view and the card; how often rung 4's choice matches when the trigger fires, clustered by deal, the Wilson lower bound from the effective sample; shipped only at 0.80 and confirmed on fresh seeds; each shipped rule re-confirmed under each other setting. Rules that must be measured before shipping: the `off` figure rule with its exception; `run`; `pagat` at seven or fewer and at twelve or more; `bid` at 20-25 points. | The rules (a rule that fails is told as its fact) |
| M6 | **Diagnosis.** Lesioned students (the M4 knockouts) must be pointed at their skill within a few deals; whole rung 4 at none. In `cargo test`, deterministic. Constants are tuned on one set of windows and checked on fresh seeds. | `DISCOUNT`, `LOWER`, `UPPER`, `MASTER_OF_TOP`, `LOSE_BELOW` |
| M7 | **Latency.** `hint_latency` (ignored test, `hint.rs`) and the evidence over a deal, native and in the wasm module. | Whether the Worker is enough; the cap on advisor worlds |
| M8 | **The split suit.** The share of a suit's points at which rung 4 starts to prefer another lead. | The `behind` trigger |

**Compute estimate**, for the maintainer before any run (`CLAUDE.md`): rung 4's
median is 7 ms a decision in wasm (`docs/OPPONENT.md`), about 3.5 s a 3-Touren
sitting, so a mirrored pair is about 7 s and 50 pairs about 6 minutes a
comparison; ten knockouts about an hour. The hint is about 125 ms native
(`PLAN.md`); a deal has about 14 decisions with a choice, so M1 (600 positions x 8
seeds) is about 10 minutes and the evidence of a deal about 2 s native. Total
about 2 hours on the 2-core VM, to be confirmed by timing one pair before the
first batch.

## 12. Tests

Test first (`CLAUDE.md`). Engine (`cargo test`, deterministic):

- **Lesioned students.** For each of `worth`, `cheap`, `off`, `run`, `void`,
  `behind`, `endgame`, `bid`, `announce`: a rung-4 student with that skill knocked
  out is given that skill as focus within `K` deals (K from M6). A student missing
  `worth` is not given `endgame` first (prerequisites). Whole rung 4 gets no focus
  and masters at least three skills (acceptance as Cassino's, in-sample windows
  and fresh seeds). A random player gets a focus at the lowest stage.
- **Hysteresis.** A mastered skill survives one bad deal and is lost after a run
  of them; a short run of misses among few chances names no focus.
- **Round trip.** Summary text parses to the same summary; a stale summary is
  listed and not counted; garbage is refused.
- **Beta.** Known values of the incomplete beta and its quantile.
- **Purity.** The same record and assisted list give the same summary and brief;
  the evidence never reads a hidden card (twin deals, `testutil::blind_check`).
- **Assisted.** An assisted decision is no evidence.
- **Brief.** At most three bullets, at most 45 words, every key present in English
  and German, rules at most 15 words, no focus and mastery never in one brief, a
  short deal says so, every parameter named in a template is supplied, the
  kindness test passes. The keys of the code equal the keys of
  `docs/MESSAGES.md` (as `hint.rs` holds its table).
- **Rules table.** The rules shipped equal the table of M5 and are told only under
  the settings they were confirmed under.
- **Examples.** The costliest miss at the focus is the one named; the figure
  example names a later trick only when the log shows the figure lost to a higher
  card of its suit.

Node: the store round trip with injected storage; a newer store read as empty and
not overwritten; import caps; no storage works; the brief rendered in both
languages; the Worker and the main-thread fallback agree. Browser: the dialog
opens after a scored deal, once, with the header line and at most three bullets;
muted shows nothing; a hint sets the decision's number in the assisted list.

## 13. Shared code: `kit/learner`

The project's kit rule: what two or more games in the series share is a generic
module in `kit/`, not a per-game copy. Cassino has the only learner today; this
is the second game, so the game-independent parts go into the kit. Cassino
(read-only here) keeps its own copy until the maintainer decides to adopt the
crate back; a test makes that safe.

**Game-independent, from `cassino-core/src/learner.rs` and `brief.rs`:**

- the discounted Beta fold: `Tally`, `Summary`, `ln_gamma`, `beta_cdf`,
  `beta_quantile`;
- mastery with hysteresis (`LOWER`, `UPPER`, `master`, `lose`);
- focus selection over a prerequisite graph (`cleared`, `short`, `costly`, gain
  = cost less `top_cost`);
- the summary's text form and versioning, `stale`;
- the brief's shape: a bullet with a lead, at most three bullets, the order well /
  next / none, the "mastery only after the second game" and "enough skills seen"
  guards.

**Game-specific, stays in each game:** the skill list with its slugs, `needs`,
`top_rate` and `top_cost`; the evidence (replay, the advisor, the classifiers of
what a move shows); the margin; the rules and their measurement; the words; the
examples.

**The crate**: `kit/learner/` (a Rust library with no dependencies, a workspace
member of the game), vendored with the STAMP convention (`kit/README.md`): `STAMP`
lists each file with its Cassino source path and how it differs.

Crate `kit-learner` (lib `kit_learner`), as built (15 Oct 2026):

```rust
// The ladder and the constants
pub struct SkillSpec { pub slug: &'static str, pub needs: &'static [usize],
                       pub top_rate: f64, pub top_cost: f64, pub min_chances: f64 }
pub struct Ladder { pub name: &'static str, pub version: u32, pub skills: &'static [SkillSpec] }
impl Ladder { pub fn check(&self) -> Result<(), String>;      // slugs, needs, rates
              pub fn index_of(&self, slug: &str) -> Option<usize>; }
pub struct Params { pub discount: f64, pub min_gain: f64, pub costly_gain: f64,
                    pub costly_chances: f64, pub lower: f64, pub upper: f64,
                    pub master_of_top: f64, pub lose_below: f64 }
impl Params { pub fn check(&self) -> Result<(), String>;      // finite, discount in (0, 1], lower < upper ...
              pub fn master(&self, s: &SkillSpec) -> f64;      // master_of_top * top_rate
              pub fn lose(&self, s: &SkillSpec) -> f64; }      // master - lose_below, at least 0
// Evidence
pub struct Tally { pub chances: u32, pub met: u32, pub missed: f64 }
pub struct Chance { pub skill: usize, pub met: bool, pub gap: f64 }
pub struct Summary { pub version: u32, pub tallies: Vec<Tally> }   // one per skill, ladder order
impl Summary { pub fn new(l: &Ladder) -> Summary;
               pub fn from_chances(l: &Ladder, c: &[Chance]) -> Summary;
               pub fn of(&self, skill: usize) -> Tally;
               pub fn is_current(&self, l: &Ladder) -> bool;        // version and size fit
               pub fn to_text(&self, l: &Ladder) -> String;         // "<name> evidence v<n>", a line a skill
               pub fn parse(l: &Ladder, text: &str) -> Result<Summary, String>; }
pub fn stale(l: &Ladder, games: &[Summary]) -> Vec<usize>;
// The learner
pub struct SkillState { pub mastered: bool, pub chances: f64, pub met: f64, pub cost: f64, pub gain: f64 }
pub struct Learner { pub games: usize, pub skills: Vec<SkillState>,
                     pub just_mastered: Vec<usize>, pub focus: Option<usize> }
impl Learner { pub fn state(&self, skill: usize) -> SkillState;
               pub fn mastered(&self, skill: usize) -> bool; }
pub fn learn(l: &Ladder, p: &Params, games: &[Summary]) -> Learner;
// The brief's shape
pub const MAX_BULLETS: usize = 3;
pub enum Param { Int(i64), Num(f64), Text(String), Skill(usize) }
pub struct Bullet { pub lead: String, pub key: String, pub params: Vec<(String, Param)> }
pub struct Strength { pub bullet: Bullet, pub praises: Vec<usize> }
pub struct Keys { pub short: &'static str, pub well_lead: &'static str, pub well_mastered: &'static str,
                  pub none_early: &'static str, pub none_clean: &'static str, pub none_up: &'static str }
pub struct BriefCfg { pub few: u32, pub min_history: usize, pub enough_skills: usize, pub keys: Keys }
pub struct Deal<'a> { pub decisions: u32, pub before: &'a Learner, pub after: &'a Learner,
                      pub strengths: &'a [Strength], pub next_opponent: bool }
pub fn brief(l: &Ladder, cfg: &BriefCfg, deal: &Deal, focus: impl FnOnce(usize) -> Vec<Bullet>) -> Vec<Bullet>;
pub mod beta { pub fn ln_gamma(x: f64) -> f64; pub fn beta_cdf(x: f64, a: f64, b: f64) -> f64;
               pub fn beta_quantile(p: f64, a: f64, b: f64) -> f64; }
```

Skills are indexes, so the crate needs no generic parameter on a skill enum; a
game keeps its enum and maps it. Where the build departs from the sketch of
15 Oct:

- `min_chances` is per skill (a `SkillSpec` field) and not in `Params`, as section 5
  has it; `costly_chances` stays in `Params`.
- A `Bullet`'s `lead` and `key` are both message keys (`lead` empty for none).
  Cassino's brief carries prose; Strohmandeln's (and the series') words are keys,
  so Cassino would map its prose to keys when it adopts the crate.
- `brief` decides which bullets there are and in what order; the game supplies the
  words: its strengths (best first, each with the skills it praises), a closure
  from the focus to its bullets (rule, this deal's example; the crate keeps as many
  as fit in three), and the six keys of `Keys`. A mastery is announced for the
  first skill of the ladder that became mastered, after `min_history` earlier
  deals. A focus that yields no bullets is treated as no focus.
- Exact ties in gain go to the earlier skill in the ladder (section 1);
  Cassino's `max_by` took the later.
- A summary of another version or size than the ladder's is stale: `learn` skips
  it and `stale` lists it.

**Test of equivalence** (`kit/learner/tests/equivalence.rs`): with Cassino's
ladder and constants written out as a `Ladder` and `Params`, the crate gives the
same mastery, state, focus and `just_mastered` as Cassino's `learn`, and the same
summary text, on a fixture of 127 cases (the scenarios of Cassino's learner tests, the exact thresholds
and 100 seeded random sequences). The fixture is generated by Cassino's own code
(`tests/fixtures/cassino_learner_gen.rs` says how) and regenerated at each
re-sync. Cases with an exact tie in gain are left out of it. Cassino's generic
learner tests (known beta values, mastered and lost, the state depends on the
state before, the focus is the costliest cleared skill, a few misses among few
chances name no focus, older games count for less, a stale summary is not counted)
are ported as `tests/learn.rs` and `tests/summary.rs`; `tests/brief.rs` holds the
brief's shape.

**JS candidates** (named in `kit/README.md` as "generic in shape"; promoted when
this game's copy has settled): `progress.js` (versioned store, try/catch, newer
never overwritten, caps, export and import, parameterised by key, version, max
and a validator); the dialog layout of `review.js` (bullets with a bold lead and a
folded "how is this worked out"); the Worker harness (`tutorworker*.js`, text
script inlined by `build.py`). Until then Strohmandeln copies them as patterns.

## 14. The table (desktop)

The brief replaces the bullets of the dialog "How the deal went" (`docs/TABLE3D.md`
6.4.1): the header line is the count (kept), then up to three bullets with bold
leads, then the folded "How is this worked out?" (`tutor.how`, `tutor.method`).
The chip, the sitting review and the mute are as in 6.4 and 6.1; the tips shown
by the nudge go in the Kibitzer's tab. `TABLE3D.md` 6.4.4 (what is kept) and 6.4.3
(the Worker) are rewritten to this document when the table work is done. The
progress settings are in section 8. **The phone layout of the dialog and the
nudge is deferred** (maintainer, 15 Oct 2026): nothing here specifies it, and the
open phone items of `PLAN.md` are not touched.

## 15. Open questions for the maintainer

1. **Rules or facts for `behind` and `bid`.** Werner says no general rules can be
   given for the contested suit, and Bermann gives a case where the leader wins
   (`research/16`, 3.1); the book's bid rule is lower than what the strongest
   rung bids. Recommendation: ship a rule only if it passes the measured gate
   (M5), else the fact and the example; do not argue the sources.
2. **The example is one specific move.** The brief's third bullet names a card
   from this deal; the original wish for the review was "trends, not specific
   mistakes". Cassino kept one example in its brief and found the fact alone
   "the same every game and gave nothing to act on". Recommendation: keep one
   example, always as the Kibitzer's, never a count of faults.
3. **The `count` line stays as the dialog's header**, not a bullet.
   Recommendation: keep it; it is the result, not advice.
4. **Promote the learner to `kit/learner` now**, ahead of Cassino adopting it
   back. Recommendation: yes, with the equivalence test; Cassino unchanged.
5. **The compute plan** (section 11, about 2 hours on the VM). Recommendation:
   approve after one timed pair confirms the estimate.
6. **Reword `TABLE3D.md` 6.3.2** ("asking is never counted") to "is not counted
   as evidence". Recommendation: yes; it changes no behaviour.
