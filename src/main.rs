use std::time::Instant;

use outcome::outcome;

mod logger;
mod matrix;
mod node;
mod outcome;
mod solver;
mod types;
mod util;
mod words;

fn outcome_bench() {
    let answers = words::answers();
    let guesses = words::guesses();
    let mut x = 0usize;

    let start = Instant::now();
    for a in &answers {
        for g in &guesses {
            x += outcome(g, a) as usize
            // x += 1;
        }
    }
    println!("{x}");
    println!("{:?}", Instant::elapsed(&start));
}

fn main() {
    // use solver::Solver;
    // logger::info();
    // Solver::demo_two_up();
    // Solver::demo();
    outcome_bench();
}
