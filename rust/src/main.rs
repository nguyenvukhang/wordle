mod node;
mod types;
mod util;
mod words;

use crate::node::Node;
use crate::util::st;
use crate::words::GUESSES;
use std::time::Instant;
use types::Word;
use util::{entropy, outcome};

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
    let (mut remaining_ans, mut tries) = (words::answers(), 0);

    while remaining_ans.len() > 1 {
        let guess = match graph.guess {
            Some(v) => v,
            None => suggest(&GUESSES, &remaining_ans).to_owned(),
        };

        // everytime an outcome is generated, increment the tries by one
        let out = outcome(&guess, fixed_answer);
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
    // assert!(remaining_ans.len() == 1);
    (tries + 1, remaining_ans[0])
}

fn main() {
    let all_answers = words::answers();
    let mut graph = Node::new(None);
    let mut total_tries = 0u32;

    let start = Instant::now();
    for fixed_answer in &all_answers {
        let (tries, generated_answer) = solve(fixed_answer, &mut graph);
        assert_eq!(st(fixed_answer), st(&generated_answer));
        total_tries += tries;
    }
    let avg = total_tries as f64 / all_answers.len() as f64;

    println!("time elapsed: {:?}", Instant::elapsed(&start));
    println!("avg tries: {avg}",)
}
