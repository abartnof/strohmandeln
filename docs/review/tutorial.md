# Review: tutorial pages and review strings

Scope: `web3d/tutorial.md`, `web3d/tutorial.en.md`, `web3d/test/tutorial.test.js`,
the `review.*` keys of `web3d/strings.json`; against TABLE3D 6.2 and RULES.

## Checked, correct

Deal and packets (2.3, 2.4, 4.8), bidding order and values (3.1-3.4, 8.3, 9.1),
the turning chain and the last card (4.3-4.7), following and no obligation to win
(5.2, 5.4, 5.5), honours and announcing (11.1-11.5, 11.7), Pagat ultimo (12.1-12.4),
values and thirds (1.7, 8.1, 8.2), draw and doubling (8.4, 9.2, 14.1, 14.4),
Touren, slate, kreuzer, reconstruction note (15.2, 15.3, 15.8, 15.9), extension
(15.6). The `review.*` strings are accurate, kind, and parallel en/de. German is
formal Sie throughout. Period terms (Troule, Aufnehmer, bedienen, stechen, Deckkarte)
match 03-S12 and 03-S14.

## Findings, fixed

1. TABLE3D 6.2.2 said "less one-third in a group of three". RULES 8.1 (value 5 is
   13 thirds, so each card counts its value less 2/3; a group of three, the sum
   less 2) and research/02 A17 give two-thirds. The pages were right; the spec was
   wrong. Spec corrected; a test holds spec and pages to it.
2. `following`: "the higher tarock, or the higher card of the suit, wins" omitted
   that any tarock beats a suit card (7.1). Reworded.
3. `from-a-packet`: "your opponent, who plays before it is shown" is true only
   when the human leads (6.1, 6.2); and "a packet down to its last card" was
   ambiguous with 4.7. Both reworded.
4. `pagat-ultimo`: the reason "the last card of a packet might turn up higher" is
   uncited and meaningless (a tarock is always pulled, 12.2). Replaced by the
   cited fact (12.1).
5. `packets`: "that card is a cover" before the pull; a turned tarock or king is
   pulled, not a cover (4.5). Reworded in both languages.

## Open

- `bidding` states only the default `both_pass = play`; under `throw_in` the
  deal is thrown in (3.4). TABLE3D 6.2.2 says the page covers both; the
  25-word line cap and the 150-word page cap leave room for one more line.
  Needs the setting passed to the page, so left to the table work package.
