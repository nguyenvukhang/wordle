mod entropy;
mod matrix;
mod node;
mod solver;
mod types;
mod util;
mod words;

use crate::solver::Solver;

fn main() {
    let mut wordle = Solver::new(&words::guesses(), &words::answers());
    let (avg_tries, time) = wordle.bench(10);
    println!("time elapsed: {:?}", time);
    println!("avg tries: {avg_tries}")
}
