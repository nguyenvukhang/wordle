use crate::types::{outcome_str, Outcome, Word, GREEN, YELLOW};
use std::borrow::Cow;

pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

macro_rules! letter {
    ($n:expr) => {
        $n as usize % 32 - 1
    };
}

/// Generate an outcome from scratch (faster than a HashMap, apparently)
pub fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let (mut outcome, mut d, mut mask) = (0, [0u8; 26], 0u8);
    // check greens
    for i in 0..5 {
        if guess[i] == answer[i] {
            outcome += GREEN[i];
            mask |= 1 << i;
        } else {
            d[letter!(answer[i])] += 1;
        }
    }
    // check yellows
    for i in 0..5 {
        if d[letter!(guess[i])] > 0 && mask & 1 << i == 0 {
            outcome += YELLOW[i];
            d[letter!(guess[i])] -= 1;
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
            entropy += (f / len) * (len / f).log2();
        }
    }
    entropy
}

#[test]
fn entropy_test() {
    use crate::words;
    let answers = words::answers();
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
pub fn suggest(guesses: &[Word], answers: &Vec<Word>) -> Word {
    let mut best = (&guesses[0], -1.0);
    let n = guesses.len();
    let mut entropies = Vec::with_capacity(n);
    for guess in guesses {
        let entropy = entropy(guess, &answers);
        entropies.push(entropy);
        if entropy > best.1 {
            best = (guess, entropy);
        }
    }

    for out in 0..243 {
        let mut best_next = (&guesses[0], -1.0);

        // supposed and outcome of `out` occurred.
        // this will be the state of the answers list
        let mut answers = answers.clone();
        answers.retain(|ans| outcome(best.0, &ans) == out);

        for g in 0..n {
            let ent2 = entropy(&guesses[g], &answers);
            if ent2 > best_next.1 {
                best_next = (&guesses[g], ent2);
            }
        }
        println!("{out:>4} {}, next: {}", outcome_str(out), st(best_next.0));
    }
    println!("entropies -> {:?}", entropies.len());
    *best.0
}
