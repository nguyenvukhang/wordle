mod types;
mod words;

use crate::words::{answers::ANSWERS, guesses::GUESSES};
use types::{Outcome, Word, ALPHABET_HASH, ENTROPY_HASH};

/// Generate an outcome from scratch (faster than a HashMap, apparently)
fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let (mut outcome, mut d, mut g) = (0, ALPHABET_HASH, [false; 5]);
    answer.iter().for_each(|v| d[(v % 32) as usize - 1] += 1);
    // check greens
    for i in 0..5 {
        if guess[i] == answer[i] {
            outcome += 3u8.pow(4 - i as u32) * 2;
            d[(guess[i] % 32) as usize - 1] -= 1;
            g[i] = true;
        }
    }
    // check yellows
    for i in 0..5 {
        let l = (guess[i] % 32) as usize - 1;
        if d[l] > 0 && !g[i] {
            outcome += 3u8.pow(4 - i as u32);
            d[l] -= 1;
        }
    }
    outcome
}

/// Calculates the entropy (information stood to gain) of a guess
/// against a known list of possible answers
pub fn entropy(guess: &Word, answers: &Vec<Word>) -> f64 {
    let (mut results, mut entropy, len) = (ENTROPY_HASH, 0.0, answers.len() as f64);
    for answer in answers {
        let outcome = outcome(guess, &answer) as usize;
        results[outcome] += 1;
    }
    for i in results {
        if i > 0 {
            let i = i as f64;
            entropy += i / len * (len / i).log2();
        }
    }
    entropy
}

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

#[test]
fn outcome_test() {
    assert_eq!(outcome(b"zzzzz", b"xxxxx"), 0);
    assert_eq!(outcome(b"zzzzz", b"zzzzz"), 242);
    assert_eq!(outcome(b"eezzz", b"zzzee"), 130);
    assert_eq!(outcome(b"adieu", b"audio"), 199);
}

fn main() {
    let all_answers = words::build(&ANSWERS);
    let all_guesses = words::build(&GUESSES);
    let mut best = (b"xxxxx", -1.0);

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
