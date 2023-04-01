use std::time::{Duration, Instant};

use crate::{
    matrix::Matrix,
    node::Node,
    types::Word,
    words::{self, GUESSES},
};

pub struct Solver {
    matrix: Matrix,
}

impl Solver {
    pub fn new(guesses: &Vec<Word>, answers: &Vec<Word>) -> Self {
        Self {
            matrix: Matrix::new(answers, guesses),
        }
    }

    pub fn solve_one(&mut self, answer: usize, mut graph: &mut Node) -> (u32, usize) {
        let mut remaining_ans = self.matrix.fresh_answer_set();
        let mut tries = 0;

        while remaining_ans.len() > 1 {
            let guess = match graph.guess {
                Some(v) => v,
                None => self.matrix.suggest(&remaining_ans),
            };

            // everytime an outcome is generated, increment the tries by one
            let out = self.matrix.outcome(guess, answer);
            tries += 1;

            // direct hit on correct answer
            if out == 242 {
                return (tries, guess);
            }

            // shrink answer space
            remaining_ans.retain(|answer| self.matrix.outcome(guess, *answer) == out);

            // save past decisions
            graph = graph.push(guess, out);
        }

        // At this point, remaining_ans should have one left inside
        // This takes one last try to get it
        //
        debug_assert!(remaining_ans.len() == 1);
        (tries + 1, remaining_ans[0])
    }

    pub fn bench(&mut self, print_count: usize) -> (f64, Duration) {
        let mut total_tries = 0;
        let n = self.matrix.answer_count();
        let printerval = n / print_count;
        let mut graph = Node::new();
        let start = Instant::now();
        let mut prev = start;

        for i in 0..n {
            if i % printerval == 0 && i > 0 {
                println!("{i}/{n} ({:?})", Instant::elapsed(&prev));
                prev = Instant::now();
            }
            let (tries, generated_answer) = self.solve_one(i, &mut graph);
            debug_assert_eq!(i, generated_answer);
            total_tries += tries;
        }
        (total_tries as f64 / n as f64, Instant::elapsed(&start))
    }

    pub fn demo() {
        let mut wordle = Solver::new(&words::guesses(), &words::answers());
        let (avg_tries, time) = wordle.bench(10);
        println!("time elapsed: {:?}", time);
        println!("avg tries: {avg_tries}")
    }
}
