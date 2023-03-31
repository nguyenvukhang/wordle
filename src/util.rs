use crate::types::{outcome_str, Outcome, Word, GREEN, YELLOW};
use std::{borrow::Cow, collections::HashSet};

pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

macro_rules! letter {
    ($n:expr) => {
        $n as usize % 32 - 1
    };
}

pub struct CachedOutcome {
    seen: HashSet<(Word, Word)>,
}

impl CachedOutcome {
    pub fn new() -> Self {
        Self {
            seen: HashSet::new(),
        }
    }

    pub fn outcome(&mut self, guess: &Word, answer: &Word) -> Outcome {
        if self.seen.insert((*guess, *answer)) {
            println!("seen! ({}, {})", st(guess), st(answer));
        }
        outcome(guess, answer)
    }
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
    let n = guesses.len();
    let mut entropies = Vec::with_capacity(n);
    let mut cache = CachedOutcome::new();

    let weights = || (0..n).map(|_| 0).collect::<Vec<usize>>();

    for g1 in 0..n {
        let guess1 = &guesses[g1];
        println!("#{} analyzing guess 1: [{}]", g1, st(guess1));
        // save the entropy of the first guess
        entropies.push((entropy(guess1, &answers), 0));

        let mut total_2nd_ent = 0.0;

        let mut outcome1_weights = weights();
        for answer in answers {
            outcome1_weights[cache.outcome(guess1, answer) as usize] += 1;
        }

        for outcome1 in 0..243 {
            // this outcome is not possible given the guess.
            // example: guess is "zzzzz" (which is not a word) and
            // outcome is GGGGG. No such answer exists.
            if outcome1_weights[outcome1 as usize] == 0 {
                continue;
            }

            // suppose the outcome of `out` occurred.
            // this will be the state of the answers list
            //
            // guess1 and outcome1 uniquely define this state.
            let mut answers = answers.clone();
            answers.retain(|ans| cache.outcome(guess1, &ans) == outcome1);

            for guess2 in guesses {
                if guess1 == guess2 {
                    continue;
                }
                let ent2 = entropy(guess2, &answers);
                // if answers.len() == 0 {
                //     println!("\"{}\" -> {}", st(guess1), outcome_str(outcome1));
                //     panic!("no answers possible");
                // }
                total_2nd_ent += ent2 * outcome1_weights[outcome1 as usize] as f64;
            }
            // println!( "{outcome1:>4} {}, guess 2: {}", outcome_str(outcome1), st(best_next.0));
        }
        total_2nd_ent /= (n * n) as f64;
        println!("total 2nd: {}", total_2nd_ent);
    }

    println!("entropies -> {:?}", entropies.len());
    guesses[0]
}
