use crate::types::{Outcome, Word};
use crate::util::st;
use std::fmt;

pub struct Node {
    next: Vec<(Outcome, Node)>,
    pub guess: Option<Word>,
}

impl Node {
    pub fn new(guess: Option<Word>) -> Self {
        Self {
            guess,
            next: Vec::new(),
        }
    }

    pub fn push(&mut self, guess: Word, outcome: Outcome) -> &mut Self {
        self.guess = Some(guess);
        if self.next.iter().find(|v| v.0 == outcome).is_none() {
            self.next.push((outcome, Node::new(None)));
        }
        &mut self.next.iter_mut().find(|v| v.0 == outcome).unwrap().1
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
