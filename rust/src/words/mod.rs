#![allow(unused)]

use crate::types::Word;

mod answers;
mod guesses;

use answers::ANSWERS;
use guesses::GUESSES;

fn st(w: &Word) -> String {
    String::from_utf8_lossy(w).to_string()
}

fn build(list: &[&[u8; 5]]) -> Vec<[u8; 5]> {
    list.into_iter().map(|v| *v.to_owned()).collect()
}

pub fn answers() -> Vec<Word> {
    build(&answers::ANSWERS)
}

pub fn guesses() -> Vec<Word> {
    build(&guesses::GUESSES)
}

pub fn display_guess(i: usize) -> String {
    GUESSES
        .get(i)
        .map(|v| st(*v))
        .unwrap_or("-----".to_string())
}

pub fn display_answer(i: usize) -> String {
    ANSWERS
        .get(i)
        .map(|v| st(*v))
        .unwrap_or("-----".to_string())
}

pub fn get_guess(i: usize) -> Option<String> {
    GUESSES.get(i).map(|v| st(*v))
}

pub fn get_answer(i: usize) -> Option<String> {
    ANSWERS.get(i).map(|v| st(*v))
}

pub fn find_guess(word: &str) -> Option<usize> {
    let word = word.as_bytes();
    GUESSES.iter().position(|v| **v == word)
}

pub fn find_answer(word: &str) -> Option<usize> {
    let word = word.as_bytes();
    ANSWERS.iter().position(|v| **v == word)
}
