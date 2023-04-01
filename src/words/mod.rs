use crate::types::Word;

mod answers;
mod guesses;

use answers::ANSWERS;
use guesses::GUESSES;

fn build(list: &[&[u8; 5]]) -> Vec<[u8; 5]> {
    list.into_iter().map(|v| *v.to_owned()).collect()
}

#[allow(unused)]
pub fn answers() -> Vec<Word> {
    build(&answers::ANSWERS)
}

#[allow(unused)]
pub fn guesses() -> Vec<Word> {
    build(&guesses::GUESSES)
}

#[allow(unused)]
pub fn get_guess(i: usize) -> Option<Word> {
    GUESSES.get(i).map(|v| **v)
}

#[allow(unused)]
pub fn get_answer(i: usize) -> Option<Word> {
    ANSWERS.get(i).map(|v| **v)
}

#[allow(unused)]
pub fn find_guess(word: &str) -> Option<usize> {
    let word = word.as_bytes();
    GUESSES.iter().position(|v| **v == word)
}

#[allow(unused)]
pub fn find_answer(word: &str) -> Option<usize> {
    let word = word.as_bytes();
    ANSWERS.iter().position(|v| **v == word)
}
