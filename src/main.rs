mod entropy;
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
    logger::init().unwrap();
    Solver::demo_two_up();
    // Solver::demo();
}
