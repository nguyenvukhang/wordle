/// Ternary/Base-3 counting system
///
/// 0: Black
/// 1: Yellow
/// 2: Green
///
/// 0 means all 5 are Black: guess is completely wrong.
/// 242 means all 5 are Green: guess is correct.
pub type Outcome = u8;

pub type Word = [u8; 5];
