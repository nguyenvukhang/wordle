mod node;
mod types;
mod util;
mod words;

use crate::node::Node;
use crate::util::st;
use crate::words::{answers::ANSWERS, guesses::GUESSES};
use types::{Outcome, Word};
use util::{entropy, outcome};

/// Reduce the answer list based on a guess and the received outcome
pub fn reduce_ans(answers: &mut Vec<Word>, guess: &Word, out: Outcome) {
    answers.retain(|answer| outcome(guess, answer) == out);
}

/// suggest a next word to play
pub fn suggest<'a>(guesses: &'a [&Word], answers: &Vec<Word>) -> &'a Word {
    let mut best = (guesses[0], -1.0);
    for guess in guesses {
        let entropy = entropy(guess, &answers);
        if entropy > best.1 {
            best = (guess, entropy);
        }
    }
    best.0
}

fn solve(fixed_answer: &Word, mut graph: &mut Node) -> (u32, Word) {
    let mut remaining_ans = words::build(&ANSWERS);
    let mut tries = 0;

    while remaining_ans.len() > 1 {
        let guess = match graph.cached() {
            Some(v) => v.to_owned(),
            None => suggest(&GUESSES, &remaining_ans).to_owned(),
        };
        let out = outcome(&guess, fixed_answer);
        reduce_ans(&mut remaining_ans, &guess, out);
        graph = graph.push(guess, out);
        tries += 1;
    }
    (tries, remaining_ans[0])
}

fn main() {
    let runs = Some(1000);

    let all_answers = words::build(&ANSWERS);
    let runs = runs.unwrap_or(all_answers.len());
    let sample = &all_answers[..runs];

    let mut graph = Node::new(None);

    let mut total_tries = 0u32;
    let mut counter = 1;

    for fixed_answer in sample {
        println!("{counter}");
        counter += 1;
        let (tries, generated_answer) = solve(fixed_answer, &mut graph);
        assert_eq!(st(fixed_answer), st(&generated_answer));
        total_tries += tries;
    }
    println!("avg tries: {}", total_tries as f64 / runs as f64)
}
