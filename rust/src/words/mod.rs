use crate::types::Word;

mod answers;
mod guesses;

pub use answers::ANSWERS;
pub use guesses::GUESSES;
pub fn build(list: &[&[u8; 5]]) -> Vec<[u8; 5]> {
    list.into_iter()
        .map(|v| {
            let mut r = [0, 0, 0, 0, 0];
            (0..5).for_each(|i| r[i] = v[i]);
            r
        })
        .collect()
}

#[allow(unused)]
pub fn answers() -> Vec<Word> {
    build(&answers::ANSWERS)
}

#[allow(unused)]
pub fn guesses() -> Vec<Word> {
    build(&guesses::GUESSES)
}
