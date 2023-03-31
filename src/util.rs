use crate::types::{Outcome, Word, GREEN, YELLOW};
use std::borrow::Cow;

pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

trait Letter {
    fn letter(&self) -> usize;
}

impl Letter for u8 {
    fn letter(&self) -> usize {
        *self as usize % 32 - 1
    }
}

/// Generate an outcome from scratch (faster than a HashMap, apparently)
pub fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let (mut outcome, mut d, mut g) = (0, [0u8; 26], [false; 5]);
    // check greens
    for i in 0..5 {
        if guess[i] == answer[i] {
            outcome += GREEN[i];
            g[i] = true;
        } else {
            d[answer[i].letter()] += 1;
        }
    }
    // check yellows
    for i in 0..5 {
        if d[guess[i].letter()] > 0 && !g[i] {
            outcome += YELLOW[i];
            d[guess[i].letter()] -= 1;
        }
    }
    outcome
}

#[test]
fn outcome_test() {
    use crate::types::outcome_str;
    macro_rules! test {
        ($guess:expr, $answer:expr, $expected:ident) => {
            let out = outcome($guess, $answer);
            let out = outcome_str(out);
            let expected = stringify!($expected);
            assert_eq!(out, expected)
        };
    }
    test!(b"zzzzz", b"xxxxx", BBBBB);
    test!(b"zzzzz", b"xxxxx", BBBBB);
    test!(b"zzzzz", b"zzzzz", GGGGG);
    test!(b"eezzz", b"zzzee", YYGYY);
    test!(b"adieu", b"audio", GYYBY);
    test!(b"crust", b"rebut", BYYBG);
    test!(b"azzzz", b"zazzz", YYGGG);
    test!(b"azzzz", b"zxxxx", BYBBB);
    // panic!("yes")
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
