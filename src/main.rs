mod node;
mod types;
mod util;
mod words;

use node::Node;
use std::time::{Duration, Instant};
use types::Word;
use util::st;
use util::{outcome, suggest};

struct Wordle {
    guesses: Vec<Word>,
    answers: Vec<Word>,
}

impl Wordle {
    pub fn new(guesses: Vec<Word>, answers: Vec<Word>) -> Self {
        Self { guesses, answers }
    }

    pub fn solve_one(&self, answer_index: usize, mut graph: &mut Node) -> (u32, Word) {
        let mut remaining_ans = self.answers.clone();
        let mut tries = 0;

        let answer = self.answers[answer_index];

        while remaining_ans.len() > 1 {
            let guess = match graph.guess {
                Some(v) => v,
                None => suggest(&self.guesses, &remaining_ans),
            };

            // everytime an outcome is generated, increment the tries by one
            let out = outcome(&guess, &answer);
            tries += 1;

            // direct hit on correct answer
            if out == 242 {
                return (tries, guess);
            }

            // shrink answer space
            remaining_ans.retain(|answer| outcome(&guess, answer) == out);

            // save past decisions
            graph = graph.push(guess, out);
        }

        // At this point, remaining_ans should have one left inside
        // This takes one last try to get it
        //
        debug_assert!(remaining_ans.len() == 1);
        (tries + 1, remaining_ans[0])
    }

    pub fn bench(&self, print_count: usize) -> (f64, Duration) {
        let mut total_tries = 0;
        let n = self.answers.len();
        let printerval = n / print_count;
        let mut graph = Node::new();
        let start = Instant::now();
        let mut prev = start;

        for i in 0..1 {
            if i % printerval == 0 && i > 0 {
                println!("{i}/{n} ({:?})", Instant::elapsed(&prev));
                prev = Instant::now();
            }
            let (tries, generated_answer) = self.solve_one(i, &mut graph);
            debug_assert_eq!(st(&self.answers[i]), st(&generated_answer));
            total_tries += tries;
        }
        (total_tries as f64 / n as f64, Instant::elapsed(&start))
    }
}

fn main() {
    let wordle = Wordle::new(words::guesses(), words::answers());
    let (avg_tries, time) = wordle.bench(10);

    println!("time elapsed: {:?}", time);
    println!("avg tries: {avg_tries}")
}
