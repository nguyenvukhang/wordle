use crate::types::{Outcome, Word};
use crate::util::st;
use std::collections::HashMap;
use std::fmt;

pub struct Node {
    next: HashMap<Outcome, Node>,
    pub guess: Option<Word>,
}

impl Node {
    pub fn new(guess: Option<Word>) -> Self {
        Self {
            guess,
            next: HashMap::new(),
        }
    }

    pub fn push(&mut self, guess: Word, outcome: Outcome) -> &mut Self {
        self.guess = Some(guess);
        if !self.next.contains_key(&outcome) {
            self.next.insert(outcome, Node::new(None));
        }
        self.next.get_mut(&outcome).unwrap()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let g = self.guess.as_ref();
        write!(
            f,
            "Node ({}, {:?})",
            g.map_or(".".to_string(), |v| st(v).to_string()),
            self.next
        )
    }
}
