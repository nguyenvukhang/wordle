mod ext;
mod types;
mod words;

use ext::String5;
use types::{Outcome, ALPHABET_HASH};

fn outcome(guess: &[u8; 5], answer: &[u8; 5]) -> Outcome {
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

fn main() {}
