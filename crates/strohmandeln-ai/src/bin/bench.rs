//! How much search costs: nodes and milliseconds for the double-dummy solver
//! at each stage of a deal, to set the budgets (`measurements/README.md`).
//!
//! ```text
//! bench [--deals N] [--cap NODES] [--bits B]
//! ```
//!
//! Plays N random deals to each of several tricks, takes the position with
//! every card known, and solves it exactly (depth to the end) and at fixed
//! depths, reporting the median nodes and time.

use std::time::Instant;
use strohmandeln_ai::sim::Sim;
use strohmandeln_ai::solver::Solver;
use strohmandeln_core::cards::Card;
use strohmandeln_core::deal::Deal;
use strohmandeln_core::rng::Rng;
use strohmandeln_core::types::{Action, Seat, Settings};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut deals = 20u64;
    let mut cap = 2_000_000u64;
    let mut bits = 18u32;
    let mut i = 0;
    while i < args.len() {
        let v = args.get(i + 1).cloned().unwrap_or_default();
        match args[i].as_str() {
            "--deals" => deals = v.parse().expect("--deals N"),
            "--cap" => cap = v.parse().expect("--cap NODES"),
            "--bits" => bits = v.parse().expect("--bits B"),
            other => panic!("unknown {other}"),
        }
        i += 2;
    }
    println!("tricks left | depth | median nodes | median ms | solved of {deals}");
    for stop in [1u8, 5, 10, 14, 18, 21, 24] {
        for depth in [1u32, 2, 3, 4, 6, 99] {
            if depth != 99 && u32::from(28 - stop) < depth {
                continue;
            }
            let mut nodes = Vec::new();
            let mut ms = Vec::new();
            let mut solved = 0;
            for seed in 0..deals {
                let mut rng = Rng::seeded(seed);
                let mut deck: Vec<Card> = Card::all().collect();
                rng.shuffle(&mut deck);
                let mut deal = Deal::new(&deck, Seat::B, 1, Settings::default());
                let mut ev = Vec::new();
                deal.apply(Seat::A, Action::Take, &mut ev).unwrap();
                while deal.trick_number() < u32::from(stop) || deal.led_card().is_some() {
                    let Some(seat) = deal.to_move() else { break };
                    let legal = deal.legal_plays(seat);
                    let (card, _) = legal[rng.below(legal.len() as u64) as usize];
                    deal.apply(seat, Action::play(card), &mut ev).unwrap();
                }
                let Some(seat) = deal.to_move() else { continue };
                let sim = Sim::from_deal(&deal, seat);
                let mut sv = Solver::new(bits);
                sv.set_limit(cap);
                let t = Instant::now();
                let r = sv.solve(&sim, depth);
                ms.push(t.elapsed().as_secs_f64() * 1000.0);
                nodes.push(sv.nodes);
                solved += usize::from(r.is_some());
            }
            nodes.sort_unstable();
            ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
            if nodes.is_empty() {
                continue;
            }
            println!(
                "{:>11} | {:>5} | {:>12} | {:>9.2} | {solved}",
                28 - stop,
                if depth == 99 {
                    "all".to_string()
                } else {
                    depth.to_string()
                },
                nodes[nodes.len() / 2],
                ms[ms.len() / 2]
            );
        }
    }
}
