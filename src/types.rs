/// Ternary/Base-3 counting system
///
/// 0: Black
/// 1: Yellow
/// 2: Green
///
/// 0 means all 5 are Black: guess is completely wrong.
/// 242 means all 5 are Green: guess is correct.
/// 182 means the first letter is Green.
pub type Outcome = u8;
pub const YELLOW: [u8; 5] = [81, 27, 9, 3, 1];
pub const GREEN: [u8; 5] = [162, 54, 18, 6, 2];

pub type Word = [u8; 5];

#[cfg(test)]
pub fn moutcome(text: &str) -> Outcome {
    let text = text.as_bytes();
    (0..5).fold(0, |a, i| match text[i] {
        b'Y' => a + YELLOW[i],
        b'G' => a + GREEN[i],
        _ => a,
    })
}

#[cfg(test)]
pub fn outcome_str(num: Outcome) -> String {
    let mut x = num;
    let mut res = [b' '; 5];
    for i in 0..5 {
        let c = x % 3;
        match c {
            2 => res[i] = b'G',
            1 => res[i] = b'Y',
            _ => res[i] = b'B',
        }
        x /= 3;
    }
    res.reverse();
    String::from_utf8_lossy(&res).to_string()
}

#[test]
fn outcome_str_test() {
    macro_rules! test {
        ($num:expr, $str:ident) => {
            assert_eq!(outcome_str($num), stringify!($str));
        };
    }

    test!(0, BBBBB);

    test!(1, BBBBY);
    test!(3, BBBYB);
    test!(9, BBYBB);
    test!(27, BYBBB);
    test!(81, YBBBB);

    test!(2, BBBBG);
    test!(6, BBBGB);
    test!(18, BBGBB);
    test!(54, BGBBB);
    test!(162, GBBBB);

    test!(69, BGYGB); // 54 + 9 + 6
}
