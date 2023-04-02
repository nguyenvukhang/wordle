use std::{fmt::Display, time::Instant};

use crate::matrix::Matrix;

mod logger;
mod matrix;
mod node;
mod outcome;
mod solver;
mod types;
mod util;
mod words;

struct Timer {
    start: Instant,
    prev: Instant,
}

impl Timer {
    fn new() -> Self {
        Self {
            start: Instant::now(),
            prev: Instant::now(),
        }
    }

    fn mark<T: Display>(&mut self, interval: usize, v: T) {
        if interval % 100 == 0 {
            println!(
                "{v: >16} | {interval: >5} -> {:?}",
                Instant::elapsed(&self.prev)
            );
            self.prev = Instant::now();
        }
    }

    fn end(self) {
        println!("total elapsed: {:?}", Instant::elapsed(&self.start));
    }
}

struct Answers {
    v: Vec<usize>,
}

impl Answers {
    fn new(count: usize) -> Self {
        Self {
            v: (0..count).collect(),
        }
    }

    fn reduce<F: Fn(usize) -> bool>(&self, f: F) -> Self {
        let mut v = Vec::with_capacity(self.v.len());
        self.v.iter().filter(|v| f(**v)).for_each(|a| v.push(*a));
        Self { v }
    }

    fn is_empty(&self) -> bool {
        self.v.is_empty()
    }

    fn len(&self) -> usize {
        self.v.len()
    }
}

fn outcome_bench() {
    let answers = words::answers();
    let guesses = words::guesses();
    let mut mx = Matrix::new(&answers, &guesses);
    let answers = Answers::new(answers.len());

    let mut x = 0u64;
    let mut timer = Timer::new();

    for g1 in 0..guesses.len() {
        for o1 in 0..243 {
            let ans = answers.reduce(|a| mx.is_outcome(g1, a, o1));
            if ans.is_empty() {
                continue;
            }
            // full version
            // let (sug, _) = mx.suggest(&ans.v);
            // x += sug as u64;
            // mock version, which is O(G) time, already is very slow
            for g2 in 0..guesses.len() {
                if g1 == g2 {
                    continue;
                }
                x += ans.len() as u64;
            }
        }
        // debugging stuff ---------------------------------------------
        timer.mark(g1, x);
        if g1 == 3000 {
            break;
        }
    }
    println!("x = {x}");
    assert_eq!(x, 89893925657);
    timer.end();
}

fn main() {
    // logger::info();
    // Solver::demo_two_up();
    solver::Solver::demo();
    // outcome_bench();
}
