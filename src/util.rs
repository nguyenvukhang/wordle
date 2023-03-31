use crate::types::{moutcome, Outcome, Word, GREEN, YELLOW};
use std::borrow::Cow;

pub fn st(w: &Word) -> Cow<'_, str> {
    String::from_utf8_lossy(w)
}

/// Generate an outcome from scratch (faster than a HashMap, apparently)
pub fn outcome(guess: &Word, answer: &Word) -> Outcome {
    let mut outcome = 0;
    let mut mask = [false; 5];
    // check greens
    for i in 0..5 {
        if guess[i] == answer[i] {
            mask[i] = true;
            outcome += GREEN[i];
        }
    }
    println!("a -> {}", st(answer));
    // check yellows
    let r = (0..5).filter(|i| !mask[*i]).collect::<Vec<_>>();
    for i in &r {
        for j in &r {
            if guess[*i] == answer[*j] {
                outcome += YELLOW[*i];
                break;
            }
        }
    }
    outcome
}

#[test]
fn outcome_test() {
    assert_eq!(outcome(b"zzzzz", b"xxxxx"), moutcome("BBBBB"));
    assert_eq!(outcome(b"zzzzz", b"zzzzz"), moutcome("GGGGG"));
    assert_eq!(outcome(b"eezzz", b"zzzee"), moutcome("YYGYY"));
    assert_eq!(outcome(b"adieu", b"audio"), moutcome("GYYBY"));
    assert_eq!(outcome(b"crust", b"rebut"), moutcome("BYYBG"));
    assert_eq!(outcome(b"azzzz", b"zazzz"), moutcome("YYGGG"));
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
