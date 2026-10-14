//! The double-dummy solver: alpha-beta over the tricks of a deal with every
//! card known (a sampled world), for card points in thirds.
//!
//! Values are **margins in thirds of the player to move**, counted from the
//! node on (what is already taken is not in them, so that positions reached
//! by different routes share a table entry). A node is the start of a trick;
//! its two plies (lead, reply) are searched in turn, and the trick's thirds
//! go to its winner, who leads the next node. The Pagat ultimo is worth
//! `sim::ULTIMO_THIRDS` to its winner.
//!
//! - **Move ordering.** The table's best move first; then, for a lead, high
//!   cards first (they win, and a win narrows the window); for a reply, the
//!   cheapest winner first, then the cheapest losers.
//! - **Transposition table.** Direct-mapped on the position's Zobrist hash at
//!   trick starts, with exact, lower and upper bounds, and the depth searched.
//! - **Depth.** `depth` tricks are searched; a node at depth 0 is valued by a
//!   rollout of the cheap policy to the end of the deal. A node whose depth
//!   covers the rest of the deal is exact.
//! - **Budget.** Every node counts against a limit; when it runs out the
//!   search stops and says so (`exhausted`), and its values are not to be
//!   used. Nodes, not seconds, so the result is deterministic.

use crate::cardset::leader_wins;
use crate::sim::{Sim, ULTIMO_THIRDS};
use strohmandeln_core::cards::Card;

const INF: i32 = 1_000_000;
/// What a rollout leaf costs against the node budget: it plays out the rest
/// of the deal.
const LEAF_COST: u64 = 24;
/// Depth stored for a node searched to the end of the deal.
const EXACT_DEPTH: u8 = 255;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Bound {
    Exact,
    Lower,
    Upper,
}

#[derive(Copy, Clone)]
struct Entry {
    key: u64,
    value: i32,
    depth: u8,
    bound: Bound,
    best: u8,
}

const EMPTY: Entry = Entry {
    key: 0,
    value: 0,
    depth: 0,
    bound: Bound::Exact,
    best: 255,
};

pub struct Solver {
    table: Vec<Entry>,
    mask: usize,
    /// Nodes searched since the limit was set.
    pub nodes: u64,
    limit: u64,
    /// The limit was reached: values from this search are not to be used.
    pub exhausted: bool,
}

impl Solver {
    /// A solver with a table of `2^bits` entries.
    pub fn new(bits: u32) -> Solver {
        Solver {
            table: vec![EMPTY; 1 << bits],
            mask: (1 << bits) - 1,
            nodes: 0,
            limit: u64::MAX,
            exhausted: false,
        }
    }

    /// Starts a new budget of `limit` nodes.
    pub fn set_limit(&mut self, limit: u64) {
        self.nodes = 0;
        self.limit = limit;
        self.exhausted = false;
    }

    /// Forgets the table (a new world).
    pub fn clear(&mut self) {
        self.table.iter_mut().for_each(|e| *e = EMPTY);
    }

    /// The margin in thirds, for the player to move, of the rest of the deal
    /// under best play by both, searched `depth` tricks deep. `None` when the
    /// budget ran out first. The position must be at the start of a trick.
    pub fn solve(&mut self, s: &Sim, depth: u32) -> Option<i32> {
        let v = self.node(s, -INF, INF, depth);
        (!self.exhausted).then_some(v)
    }

    /// The value of each legal move of the mover, to the mover, at `depth`
    /// tricks (counting the one in progress). `None` if the budget ran out.
    pub fn move_values(&mut self, s: &Sim, depth: u32) -> Option<Vec<(Card, i32)>> {
        let mut out = Vec::new();
        for c in self.ordered(s, None) {
            let v = self.child(s, c, -INF, INF, depth);
            if self.exhausted {
                return None;
            }
            out.push((c, v));
        }
        Some(out)
    }

    /// Moves in search order.
    fn ordered(&self, s: &Sim, first: Option<Card>) -> Vec<Card> {
        let mut moves: Vec<Card> = s.legal().to_vec();
        match s.led() {
            None => {
                moves.sort_by_key(|c| std::cmp::Reverse((c.thirds(), c.strength(), c.is_tarock())))
            }
            Some(led) => moves.sort_by_key(|&c| {
                let wins = !leader_wins(led, c);
                (
                    u8::from(!wins),
                    if wins { c.strength() } else { 0 },
                    c.thirds(),
                    c.strength(),
                )
            }),
        }
        if let Some(f) = first {
            if let Some(i) = moves.iter().position(|&c| c == f) {
                moves[..=i].rotate_right(1);
            }
        }
        moves
    }

    /// The value for the mover of playing `c`, with the window (alpha, beta).
    fn child(&mut self, s: &Sim, c: Card, alpha: i32, beta: i32, depth: u32) -> i32 {
        let mover = s.mover();
        let mut s2 = s.clone();
        match s2.play(c) {
            // The opponent replies: his value is the negative of mine.
            None => -self.reply(&s2, -beta, -alpha, depth),
            Some((winner, thirds)) => {
                let t = thirds as i32;
                let (sign, lo, hi) = if winner == mover {
                    (1, alpha - t, beta - t)
                } else {
                    (-1, -beta - t, -alpha - t)
                };
                let x = self.after_trick(&s2, lo, hi, depth.saturating_sub(1));
                sign * (t + x)
            }
        }
    }

    /// The reply ply: the same search over the mover's moves, mid-trick.
    fn reply(&mut self, s: &Sim, alpha: i32, beta: i32, depth: u32) -> i32 {
        self.count();
        if self.exhausted {
            return 0;
        }
        let mut alpha = alpha;
        let mut best = -INF;
        for c in self.ordered(s, None) {
            let v = self.child(s, c, alpha, beta, depth);
            if self.exhausted {
                return 0;
            }
            if v > best {
                best = v;
                if v > alpha {
                    alpha = v;
                }
                if alpha >= beta {
                    break;
                }
            }
        }
        best
    }

    /// The value, for the winner who leads it, of the position after a trick.
    fn after_trick(&mut self, s: &Sim, alpha: i32, beta: i32, depth: u32) -> i32 {
        if s.is_over() {
            let u = i32::from(s.ultimo) * ULTIMO_THIRDS;
            return if s.leader == 0 { u } else { -u };
        }
        self.node(s, alpha, beta, depth)
    }

    /// A trick-start node (the leader to move).
    fn node(&mut self, s: &Sim, alpha: i32, beta: i32, depth: u32) -> i32 {
        self.count();
        if self.exhausted {
            return 0;
        }
        let remaining = 28 - u32::from(s.trick);
        if depth == 0 {
            // The horizon: the cheap policy plays the deal out.
            self.charge(LEAF_COST);
            let l = s.leader;
            let here = s.take[usize::from(l)] as i32 - s.take[usize::from(1 - l)] as i32;
            return s.clone().rollout_for(l) - here;
        }
        let exact = depth >= remaining;
        let stored_depth = if exact { EXACT_DEPTH } else { depth as u8 };
        let key = s.key();
        let slot = (key as usize) & self.mask;
        let e = self.table[slot];
        let mut first = None;
        if e.key == key {
            if e.best != 255 {
                first = Card::from_index(e.best);
            }
            if e.depth == stored_depth {
                match e.bound {
                    Bound::Exact => return e.value,
                    Bound::Lower if e.value >= beta => return e.value,
                    Bound::Upper if e.value <= alpha => return e.value,
                    _ => {}
                }
            }
        }
        let orig_alpha = alpha;
        let mut alpha = alpha;
        let mut best = -INF;
        let mut best_move = 255u8;
        for c in self.ordered(s, first) {
            let v = self.child(s, c, alpha, beta, depth);
            if self.exhausted {
                return 0;
            }
            if v > best {
                best = v;
                best_move = c.index() as u8;
                if v > alpha {
                    alpha = v;
                }
                if alpha >= beta {
                    break;
                }
            }
        }
        let bound = if best <= orig_alpha {
            Bound::Upper
        } else if best >= beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        self.table[slot] = Entry {
            key,
            value: best,
            depth: stored_depth,
            bound,
            best: best_move,
        };
        best
    }

    fn count(&mut self) {
        self.charge(1);
    }

    fn charge(&mut self, n: u64) {
        self.nodes += n;
        if self.nodes > self.limit {
            self.exhausted = true;
        }
    }
}

#[cfg(test)]
#[path = "solver_tests.rs"]
mod tests;
