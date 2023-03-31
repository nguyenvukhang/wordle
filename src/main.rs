mod node;
mod types;
mod util;
mod words;

use node::Node;
use std::time::Instant;
use types::Word;
use util::st;
use util::{outcome, suggest};
use words::GUESSES;

fn solve(answers: &[&Word], index: usize, mut graph: &mut Node) -> (u32, Word) {
    let mut remaining_ans = words::build(answers);
    let mut tries = 0;
    let mut guesses = vec![];

    let fixed_answer = answers[index];

    while remaining_ans.len() > 1 {
        let guess = match graph.guess {
            Some(v) => v,
            None => suggest(&GUESSES, &remaining_ans).to_owned(),
        };

        // everytime an outcome is generated, increment the tries by one
        let out = outcome(&guess, &fixed_answer);
        tries += 1;

        // direct hit on correct answer
        if out == 242 {
            return (tries, guess);
        }

        // shrink answer space
        remaining_ans.retain(|answer| outcome(&guess, answer) == out);

        // save past decisions
        graph = graph.push(guess, out);
        guesses.push(st(&guess).to_string());
    }

    // At this point, remaining_ans should have one left inside
    // This takes one last try to get it
    //
    debug_assert!(remaining_ans.len() == 1);
    (tries + 1, remaining_ans[0])
}

fn main() {
    let all_answers = words::ANSWERS;
    let len = all_answers.len();
    let mut graph = Node::new();
    let mut total_tries = 0u32;

    let intervals = 10;
    let printerval = all_answers.len() / intervals;

    let start = Instant::now();
    let mut prev = start;

    for i in 0..len {
        if i % printerval == 0 && i > 0 {
            println!("{i}/{} ({:?})", all_answers.len(), Instant::elapsed(&prev));
            prev = Instant::now();
        }
        let (tries, generated_answer) = solve(&all_answers, i, &mut graph);
        debug_assert_eq!(st(all_answers[i]), st(&generated_answer));
        total_tries += tries;
    }
    let avg = total_tries as f64 / all_answers.len() as f64;

    println!("time elapsed: {:?}", Instant::elapsed(&start));
    println!("avg tries: {avg}",)
}
