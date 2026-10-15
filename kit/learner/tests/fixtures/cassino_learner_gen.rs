// Not compiled here. To regenerate the fixture, make a scratch crate outside
// this repository whose Cargo.toml has `[workspace]` (empty, so that it is its
// own workspace) and the dependency
//   cassino-core = { path = "<~/cassino>/crates/cassino-core" }
// put this file at src/main.rs and run
//   cargo run --release -q --target-dir <scratch>/target > cassino_learner.txt
// then copy the output to tests/fixtures/cassino_learner.txt. Cassino is only
// read.
//
// Generates tests/fixtures/cassino_learner.txt of the kit/learner crate by
// running Cassino's own learner (cassino-core @ 73f0cd7) on summaries: 27
// hand-made cases (the scenarios of Cassino's learner tests, and the exact
// thresholds `edge-*`), then 100 sequences from a fixed linear congruential
// generator (100 tried, none dropped for a tie): 127 cases in all.
// Cases where two skills tie
// exactly in gain are dropped (kit/learner breaks ties to the earlier skill,
// Cassino to the later; docs/TUTOR.md section 1).
use cassino_core::learner::{learn, Summary, Tally};
use cassino_core::tutor::Skill;

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn idx(s: Skill) -> usize {
    Skill::ALL.iter().position(|&k| k == s).unwrap()
}

fn summary(lines: &[(Skill, u32, u32, f64)]) -> Summary {
    let mut s = Summary::from_chances(&[]);
    for &(skill, chances, met, missed) in lines {
        s.tallies[idx(skill)] = Tally {
            chances,
            met,
            missed,
        };
    }
    s
}

fn game(skill: Skill, chances: u32, met: u32, gap: f64) -> Summary {
    summary(&[(skill, chances, met, f64::from(chances - met) * gap)])
}

fn emit(name: &str, games: &[Summary], out: &mut String) -> bool {
    let l = learn(games);
    let mut gains: Vec<f64> = l
        .skills
        .iter()
        .map(|s| s.gain)
        .filter(|&g| g >= 0.5)
        .collect();
    gains.sort_by(|a, b| a.total_cmp(b));
    if gains.windows(2).any(|w| w[0] == w[1]) {
        return false;
    }
    out.push_str(&format!("case {name}\n"));
    for g in games {
        out.push_str("deal\n");
        out.push_str(&g.to_text());
        out.push('\n');
    }
    let slugs = |v: Vec<Skill>| {
        if v.is_empty() {
            "-".to_string()
        } else {
            v.iter().map(|s| s.slug()).collect::<Vec<_>>().join(",")
        }
    };
    out.push_str(&format!(
        "expect games={} focus={} just={}\n",
        l.games,
        l.focus.map_or("-", |s| s.slug()),
        slugs(l.just_mastered.clone()),
    ));
    for s in Skill::ALL {
        let t = l.state(s);
        out.push_str(&format!(
            "state {} {} {:?} {:?} {:?} {:?}\n",
            s.slug(),
            u8::from(t.mastered),
            t.chances,
            t.met,
            t.cost,
            t.gain
        ));
    }
    out.push_str("end\n");
    true
}

fn main() {
    use Skill::*;
    let mut out = String::new();
    let mut n = 0;
    let mut hand = |name: &str, games: Vec<Summary>, out: &mut String| {
        assert!(emit(name, &games, out), "{name} ties");
        n += 1;
    };
    // The scenarios of Cassino's learner tests.
    let mut games = Vec::new();
    for i in 0..3 {
        games.push(game(Pairs, 6, 6, 1.0));
        hand(&format!("mastered-{i}"), games.clone(), &mut out);
    }
    games.push(game(Pairs, 6, 3, 1.0));
    hand("hysteresis", games.clone(), &mut out);
    for _ in 0..4 {
        games.push(game(Pairs, 6, 1, 1.0));
    }
    hand("lost", games.clone(), &mut out);
    let middling = game(Pairs, 30, 25, 0.5);
    let mut up: Vec<Summary> = (0..3).map(|_| game(Pairs, 30, 30, 0.5)).collect();
    let mut down: Vec<Summary> = (0..3).map(|_| game(Pairs, 30, 15, 0.5)).collect();
    for _ in 0..30 {
        up.push(middling.clone());
        down.push(middling.clone());
    }
    hand("past-up", up, &mut out);
    hand("past-down", down, &mut out);
    hand(
        "pairs-first",
        vec![summary(&[
            (Pairs, 10, 4, 3.0),
            (Sums, 10, 2, 16.0),
            (Valuables, 5, 3, 1.0),
        ])],
        &mut out,
    );
    hand(
        "sums-next",
        (0..3)
            .map(|_| {
                summary(&[
                    (Pairs, 20, 20, 0.0),
                    (Sums, 10, 2, 16.0),
                    (Valuables, 5, 3, 1.0),
                ])
            })
            .collect(),
        &mut out,
    );
    hand("sums-alone", vec![summary(&[(Sums, 10, 2, 16.0)])], &mut out);
    hand(
        "few-misses-few-chances",
        vec![summary(&[(Valuables, 5, 4, 3.0)])],
        &mut out,
    );
    hand(
        "surely-short-valuables",
        vec![summary(&[(Valuables, 5, 2, 3.0)]); 6],
        &mut out,
    );
    hand(
        "one-missed-chance",
        vec![summary(&[(Valuables, 1, 0, 3.0)])],
        &mut out,
    );
    hand(
        "prerequisite-not-short",
        vec![summary(&[(Pairs, 8, 7, 2.0), (Sums, 6, 1, 25.0)]); 20],
        &mut out,
    );
    hand(
        "costly-middling",
        vec![summary(&[(Valuables, 12, 10, 14.0)]); 8],
        &mut out,
    );
    hand(
        "costly-not-surely-short",
        vec![summary(&[(Valuables, 6, 5, 12.0)]); 10],
        &mut out,
    );
    hand(
        "nothing-costs-enough",
        vec![summary(&[(Pairs, 10, 9, 0.2), (Valuables, 5, 4, 0.1)])],
        &mut out,
    );
    hand("no-games", vec![], &mut out);
    hand(
        "older-count-less",
        vec![game(Valuables, 10, 10, 1.0), game(Valuables, 10, 0, 1.0)],
        &mut out,
    );
    hand(
        "older-count-less-reversed",
        vec![game(Valuables, 10, 0, 1.0), game(Valuables, 10, 10, 1.0)],
        &mut out,
    );
    // The thresholds, exactly: answering-builds has top_cost 0 and top_rate 1,
    // so its gain is the miss itself (tests/thresholds.rs holds the same).
    // min_chances 4, min_gain 0.5, costly_chances 8, costly_gain 2.
    for (name, chances, met, missed) in [
        ("edge-min-gain-at", 4, 0, 0.5),
        ("edge-min-gain-below", 4, 0, 0.499),
        ("edge-min-chances-below", 3, 0, 0.5),
        ("edge-costly-at", 8, 7, 2.0),
        ("edge-costly-chances-below", 7, 6, 2.0),
        ("edge-costly-gain-below", 8, 7, 1.999),
    ] {
        hand(
            name,
            vec![summary(&[(AnsweringBuilds, chances, met, missed)])],
            &mut out,
        );
    }
    let mut stale = summary(&[(Pairs, 20, 0, 40.0)]);
    stale.version += 1;
    hand("stale", vec![stale], &mut out);

    // Random sittings: each skill has a rate and a gap for the whole
    // sequence, and each deal a number of chances.
    let mut rng = Lcg(0x5eed_cafe);
    let mut made = 0;
    let mut tries = 0;
    while made < 100 {
        tries += 1;
        let deals = 1 + rng.below(10) as usize;
        // Half the skills of a sequence are played well, so that mastery,
        // loss and "no focus" are well represented.
        let rates: Vec<u64> = Skill::ALL
            .iter()
            .map(|_| {
                if rng.below(2) == 0 {
                    85 + rng.below(16)
                } else {
                    rng.below(101)
                }
            })
            .collect();
        let gaps: Vec<u64> = Skill::ALL.iter().map(|_| rng.below(25)).collect(); // quarter points
        let rare: Vec<bool> = Skill::ALL.iter().map(|_| rng.below(3) == 0).collect();
        let games: Vec<Summary> = (0..deals)
            .map(|_| {
                let mut s = Summary::from_chances(&[]);
                for i in 0..8 {
                    let chances = if rare[i] { rng.below(4) } else { rng.below(16) } as u32;
                    let met = (0..chances).filter(|_| rng.below(100) < rates[i]).count() as u32;
                    let missed = f64::from(chances - met) * gaps[i] as f64 * 0.25;
                    s.tallies[i] = Tally {
                        chances,
                        met,
                        missed,
                    };
                }
                s
            })
            .collect();
        if emit(&format!("random-{made}"), &games, &mut out) {
            made += 1;
        }
    }
    eprintln!("{n} hand-made, {made} random ({tries} tried)");
    print!("{out}");
}
