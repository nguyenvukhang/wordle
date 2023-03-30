use std::collections::HashMap;

use crate::types::Word;

pub struct Node {
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
