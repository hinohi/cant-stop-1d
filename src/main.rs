mod expr;

use std::collections::HashMap;

use clap::Clap;

use crate::expr::Expr;

#[derive(Debug, Copy, Clone, Clap)]
struct Opts {
    #[clap(short, long, default_value = "6")]
    dice: u32,
    #[clap(short, long, default_value = "20")]
    goal: u32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct State {
    original_pos: u32,
    pos: u32,
    dice: Option<u32>,
}

impl State {
    pub const fn new(original_pos: u32) -> State {
        State {
            original_pos,
            pos: original_pos,
            dice: None,
        }
    }

    pub const fn dice(&self, dice: u32) -> State {
        State {
            original_pos: self.original_pos,
            pos: self.original_pos + dice,
            dice: Some(dice),
        }
    }

    pub fn go_success(&self) -> State {
        State {
            original_pos: self.original_pos,
            pos: self.pos + self.dice.unwrap(),
            dice: self.dice,
        }
    }

    pub const fn stop(&self) -> State {
        State {
            original_pos: self.pos,
            pos: self.pos,
            dice: None,
        }
    }
}

#[derive(Debug)]
struct Solver {
    opts: Opts,
    mem: HashMap<u32, f64>,
}

impl Solver {
    pub fn new(opts: Opts) -> Solver {
        let n = (opts.goal + 1) * (opts.goal + 1) * opts.dice;
        Solver {
            opts,
            mem: HashMap::with_capacity(n as usize),
        }
    }

    pub fn solve(&mut self, n: u32) -> f64 {
        self.dfs_a(State::new(n))
    }

    pub fn strategy(&mut self, n: u32, dice: u32) -> (f64, f64) {
        let state = State::new(n).dice(dice);
        let stop_s = self.dfs_a(state.stop());
        let total_s = self.dfs_b(state).bisect();
        (total_s, stop_s)
    }

    const fn dice_n(&self) -> f64 {
        self.opts.dice as f64
    }

    fn dfs_a(&mut self, state: State) -> f64 {
        assert!(state.dice.is_none());
        if state.pos >= self.opts.goal {
            return 0.0;
        }
        if let Some(result) = self.mem.get(&state.pos) {
            return result.clone();
        }
        let mut s = Expr::constant(1.0);
        for dice in 1..=self.opts.dice {
            s = s + self.dfs_b(state.dice(dice)) / self.dice_n();
        }
        let r = s.bisect();
        self.mem.insert(state.pos, r.clone());
        r
    }

    fn dfs_b(&mut self, state: State) -> Expr {
        assert!(state.dice.is_some());
        if state.pos >= self.opts.goal {
            return Expr::constant(0.0);
        }
        let stop = self.dfs_a(state.stop());
        let go_success = self.dfs_b(state.go_success());
        let go_fail = Expr::self_consistent(1.0) + Expr::constant(1.0);
        let go = go_success / self.dice_n() + go_fail * ((self.dice_n() - 1.0) / self.dice_n());
        go.min(Expr::constant(stop))
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let goal = opts.goal;
    let mut solver = Solver::new(opts);
    for i in 0..goal {
        println!("{} {}", i, solver.solve(i));
    }
    for n in 0..goal {
        print!("{}", n);
        for d in 1..=opts.dice {
            print!(" {:?}", solver.strategy(n, d));
        }
        println!();
    }
}
