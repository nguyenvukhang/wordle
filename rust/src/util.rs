use crate::types::{Outcome, Word, GREEN, YELLOW};
use std::{borrow::Cow, time::Instant};

#[allow(unused)]
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

pub fn bench<F: Fn() -> ()>(title: &str, f: F) {
    let start = Instant::now();
    f();
    let elapsed = Instant::elapsed(&start);
    println!("[{title}] runtime: {:?}", elapsed);
}
