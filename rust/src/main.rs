mod types;
mod util;
mod words;

use crate::words::{answers::ANSWERS, guesses::GUESSES};
use std::collections::HashMap;
use types::{Outcome, Word};
use util::{entropy, outcome};

/// Reduce the answer list based on a guess and the received outcome
pub fn reduce_ans(answers: &mut Vec<Word>, guess: &Word, out: Outcome) {
    answers.retain(|answer| outcome(guess, answer) == out);
}

struct Node {
    next: HashMap<Word, Node>,
    guessed: Option<Word>,
}

impl Node {
    pub fn new(guess: Option<Word>) -> Self {
        Self {
            next: HashMap::new(),
            guessed: guess,
        }
    }

    pub fn trace(&self, path: &[Word]) -> Option<Word> {
        if path.is_empty() {
            return self.guessed;
        }
        let front = path.first()?;
        self.next.get(front)?.trace(&path[1..])
    }

    pub fn push(&mut self, path: &[Word], guess: &Word) {
        let front = match path.first() {
            None => {
                self.guessed = Some(guess.to_owned());
                return;
            }
            Some(v) => v,
        };
        match self.next.get_mut(front) {
            Some(v) => v.push(&path[1..], guess),
            None => {
                self.next.insert(*front, Node::new(Some(guess.to_owned())));
            }
        }
    }
}

/// suggest a next word to play
pub fn suggest<'a>(guesses: &'a [&Word], answers: &Vec<Word>, path: &Vec<Word>) -> &'a Word {
    let mut best = (guesses[0], -1.0);
    for guess in guesses {
        let entropy = entropy(guess, &answers);
        if entropy > best.1 {
            best = (guess, entropy);
        }
    }
    best.0
}

fn main() {
    let all_answers = words::build(&ANSWERS);
    let all_guesses = words::build(&GUESSES);
    let mut graph = Node::new(None);

    let mut remaining_ans = all_answers.clone();

    let fixed = b"frame";
    let path: Vec<Word> = vec![];

    let guess1 = suggest(&GUESSES, &remaining_ans, &path);
    let out = outcome(guess1, fixed);
    reduce_ans(&mut remaining_ans, guess1, out);
    graph.push(path.as_slice(), guess1);

    let guess2 = suggest(&GUESSES, &remaining_ans, &path);
    let out = outcome(guess2, fixed);
    reduce_ans(&mut remaining_ans, guess2, out);
    graph.push(path.as_slice(), guess2);


    println!("guess1: {:?}", String::from_utf8_lossy(guess1));
    println!("guess2: {:?}", String::from_utf8_lossy(guess2));
    println!(
        "graph: {:?}",
        graph
            .trace(path.as_slice())
            .as_ref()
            .map(|v| String::from_utf8_lossy(v))
    );
    println!("outcome: {:?}", out);
    println!("possible remained: {:?}", remaining_ans.len());
    println!("{}", entropy(b"soare", &all_answers));
}
