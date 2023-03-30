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

fn solve(fixed_answer: &Word, graph: &mut Node) {
    let mut remaining_ans = words::build(&ANSWERS);
    let mut path: Vec<Word> = vec![];
    let mut limit = 7;

    while remaining_ans.len() > 1 {
        if limit < 1 {
            break;
        }
        limit -= 1;
        // let guess = match graph.trace(path.as_slice()) {
        //     Some(v) => v.to_owned(),
        //     None => suggest(&GUESSES, &remaining_ans).to_owned(),
        // };
        let guess = suggest(&GUESSES, &remaining_ans).to_owned();
        let out = outcome(&guess, fixed_answer);
        reduce_ans(&mut remaining_ans, &guess, out);
        graph.push(path.as_slice(), &guess);
        path.push(guess);
        println!(
            "{:?}\n{:?}",
            graph,
            path.iter().map(|v| st(v)).collect::<Vec<_>>()
        );
    }
    println!(
        "generated answer: {:?}",
        String::from_utf8_lossy(&remaining_ans[0])
    );
}

fn main() {
    let all_answers = words::build(&ANSWERS);
    let sample = &all_answers[..1];
    let mut graph = Node::new(None);

    for fixed_answer in sample {
        solve(fixed_answer, &mut graph);
        println!(
            "correct answer: {:?}",
            String::from_utf8_lossy(fixed_answer)
        );
    }

    // println!("guess1: {:?}", String::from_utf8_lossy(guess1));
    // println!("guess2: {:?}", String::from_utf8_lossy(guess2));
    // println!(
    //     "graph: {:?}",
    //     graph
    //         .trace(path.as_slice())
    //         .as_ref()
    //         .map(|v| String::from_utf8_lossy(v))
    // );
    // println!("outcome: {:?}", out);
    // println!("possible remained: {:?}", remaining_ans.len());
    // println!("{}", entropy(b"soare", &all_answers));
}
