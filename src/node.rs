use crate::types::{Outcome, Word};
use crate::util::st;
use std::fmt;

pub struct Node {
    next: Vec<(Outcome, Node)>,
    pub guess: Option<Word>,
}

impl Node {
    pub fn new() -> Self {
        Self {
            guess: None,
            next: Vec::new(),
        }
    }

    pub fn push(&mut self, guess: Word, outcome: Outcome) -> &mut Self {
        self.guess = Some(guess);
        let pos = match self.next.iter().position(|v| v.0 == outcome) {
            None => {
                self.next.push((outcome, Node::new()));
                self.next.len() - 1
            }
            Some(v) => v,
        };
        &mut self.next[pos].1
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
