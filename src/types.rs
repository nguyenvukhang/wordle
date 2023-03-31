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
