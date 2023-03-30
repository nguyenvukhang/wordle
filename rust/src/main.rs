mod types;
mod util;
mod words;

use crate::words::{answers::ANSWERS, guesses::GUESSES};
use types::{Outcome, Word};
use util::{entropy, outcome};

/// Reduce the answer list based on a guess and the received outcome
pub fn reduce_ans(answers: &mut Vec<Word>, guess: &Word, out: Outcome) {
    answers.retain(|answer| outcome(guess, answer) == out);
}

/// suggest a next word to play
pub fn suggest<'a>(guesses: &'a [&Word], answers: &Vec<Word>, path: &Vec<Word>) -> &'a Word {
    let mut best = (guesses[0], -1.0);
    for guess in guesses {
        let entropy = entropy(guess, &answers);
        if entropy > best.1 {
            best = (guess, entropy);
        }
    }
    best.0
}

fn main() {
    let all_answers = words::build(&ANSWERS);
    let all_guesses = words::build(&GUESSES);

    let mut remaining_ans = all_answers.clone();

    let fixed = b"frame";
    let path: Vec<Word> = vec![];

    let first_guess = suggest(&GUESSES, &all_answers, &path);
    let outcome = outcome(first_guess, fixed);
    reduce_ans(&mut remaining_ans, first_guess, outcome);

    println!("first guess: {:?}", String::from_utf8_lossy(first_guess));
    println!("outcome: {:?}", outcome);
    println!("possible remained: {:?}", remaining_ans.len());
    println!("{}", entropy(b"soare", &all_answers));
}
