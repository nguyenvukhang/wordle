mod logger;
mod matrix;
mod node;
mod outcome;
mod solver;
mod types;
mod util;
mod words;

use solver::Solver;

fn main() {
    logger::error();
    // Solver::demo_two_up();
    Solver::demo();
}
