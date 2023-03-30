mod ext;
mod types;
mod words;

use ext::String5;
use types::{Arr26, Arr5, Outcome, ALPHABET_HASH};

fn letter_count(arr: &Arr5) -> Arr26 {
    let mut result = ALPHABET_HASH;
    arr.iter().for_each(|v| result[(v % 32) as usize - 1] += 1);
    result
}

fn outcome(guess: &str, answer: &str) -> Outcome {
    let mut outcome = 0;
    let guess = guess.to_arr();
    let answer = answer.to_arr();
    let mut d_answer = letter_count(&answer);
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
    assert_eq!(outcome("zzzzz", "xxxxx"), 0);
    assert_eq!(outcome("zzzzz", "zzzzz"), 242);
    assert_eq!(outcome("eezzz", "zzzee"), 130);
    assert_eq!(outcome("adieu", "audio"), 199);
}

fn main() {
}
