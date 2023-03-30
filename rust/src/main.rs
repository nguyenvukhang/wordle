mod types;
mod words;

use crate::words::{answers::ANSWERS, guesses::GUESSES};
use types::{Outcome, Word, ALPHABET_HASH, ENTROPY_HASH};

fn entropy(guess: &Word, remaining_ans: &Vec<Word>) -> f64 {
    let mut results = ENTROPY_HASH;
    for answer in remaining_ans {
        let outcome = outcome(guess, &answer) as usize;
        results[outcome] += 1;
    }
    let (mut entropy, len) = (0.0, remaining_ans.len() as f64);
    for i in results {
        if i > 0 {
            let i = i as f64;
            entropy += i / len * (len / i).log2();
        }
    }
    entropy
}

fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let mut outcome = 0;
    let mut d_answer = ALPHABET_HASH;
    answer
        .iter()
        .for_each(|v| d_answer[(v % 32) as usize - 1] += 1);
    let mut g = [false, false, false, false, false];
    // check greens
    for i in 0..5 {
        if guess[i] == answer[i] {
            outcome += 3u8.pow(4 - i as u32) * 2;
            d_answer[(guess[i] % 32) as usize - 1] -= 1;
            g[i] = true;
        }
    }
    // check yellows
    for i in 0..5 {
        let l = (guess[i] % 32) as usize - 1;
        if d_answer[l] > 0 && !g[i] {
            outcome += 3u8.pow(4 - i as u32);
            d_answer[l] -= 1;
        }
    }
    outcome
}

#[test]
fn outcome_test() {
    assert_eq!(outcome(b"zzzzz", b"xxxxx"), 0);
    assert_eq!(outcome(b"zzzzz", b"zzzzz"), 242);
    assert_eq!(outcome(b"eezzz", b"zzzee"), 130);
    assert_eq!(outcome(b"adieu", b"audio"), 199);
}

fn suggest<'a>(possible_guesses: &'a [&Word], remaining_ans: &Vec<Word>) -> &'a Word {
    let mut best = (possible_guesses[0], -1.0);
    for guess in possible_guesses {
        let entropy = entropy(guess, &remaining_ans);
        if entropy > best.1 {
            best = (guess, entropy);
        }
    }
    best.0
}

fn reduce_ans(guess: &Word, answers: &Vec<Word>, desired_outcome: Outcome) -> Vec<Word> {
    answers
        .iter()
        .filter(|answer| outcome(guess, answer) == desired_outcome)
        .map(|v| v.to_owned())
        .collect()
}

fn main() {
    let all_answers = words::build(&ANSWERS);
    let all_guesses = words::build(&GUESSES);
    let mut best = (b"xxxxx", -1.0);

    let remaining_ans = all_answers.clone();

    let fixed_answer = b"frame";
    let path: Vec<Word> = vec![];

    let first_guess = suggest(&GUESSES, &all_answers);
    let outcome = outcome(first_guess, fixed_answer);
    let remaining_ans = reduce_ans(first_guess, &remaining_ans, outcome);

    println!("first guess: {:?}", String::from_utf8_lossy(first_guess));
    println!("outcome: {:?}", outcome);
    println!("possible remained: {:?}", remaining_ans.len());
    println!("{}", entropy(b"soare", &all_answers));
}
