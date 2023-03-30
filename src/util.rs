use std::borrow::Cow;

use crate::types::{Outcome, Word};

pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

/// Generate an outcome from scratch (faster than a HashMap, apparently)
pub fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let (mut outcome, mut d, mut g) = (0, [0u8; 26], [false; 5]);
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

#[test]
fn outcome_test() {
    assert_eq!(outcome(b"zzzzz", b"xxxxx"), 0);
    assert_eq!(outcome(b"zzzzz", b"zzzzz"), 242);
    assert_eq!(outcome(b"eezzz", b"zzzee"), 130);
    assert_eq!(outcome(b"adieu", b"audio"), 199);
    assert_eq!(outcome(b"crust", b"rebut"), 38);
}

/// Calculates the entropy (information stood to gain) of a guess
/// against a known list of possible answers
pub fn entropy(guess: &Word, answers: &Vec<Word>) -> f64 {
    let (mut freq, mut entropy, len) = ([0; 243], 0.0, answers.len() as f64);
    for answer in answers {
        freq[outcome(guess, &answer) as usize] += 1;
    }
    for f in freq {
        if f > 0 {
            let f = f as f64;
            entropy += f / len * (len / f).log2();
        }
    }
    entropy
}

#[test]
fn entropy_test() {
    use crate::words;
    use crate::words::ANSWERS;
    let answers = words::build(&ANSWERS);
    macro_rules! test {
        ($word:expr, $val:expr) => {
            assert_eq!(entropy($word, &answers), $val)
        };
    }
    test!(b"soare", 5.885202744292758);
    test!(b"roate", 5.884856313732008);
    test!(b"raise", 5.878302956493169);
    test!(b"reast", 5.867738020843562);
    test!(b"reast", 5.867738020843562);
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
