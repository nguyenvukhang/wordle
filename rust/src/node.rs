use crate::types::{Outcome, Word};
use std::collections::HashMap;
use std::fmt;

pub struct Node {
    next: HashMap<Outcome, Node>,
    guess: Option<Word>,
}

impl Node {
    pub fn new(guess: Option<Word>) -> Self {
        let next = HashMap::new();
        Self { guess, next }
    }

    pub fn cached(&self) -> Option<Word> {
        self.guess
    }

    pub fn push<'a>(&'a mut self, guess: Word, outcome: Outcome) -> &'a mut Self {
        if let Some(current) = self.guess {
            assert_eq!(current, guess);
        }
        self.guess = Some(guess);
        let has = self.next.contains_key(&outcome);
        if !has {
            self.next.insert(outcome, Node::new(None));
        }
        self.next.get_mut(&outcome).unwrap()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node ({}, {:?})",
            self.guess
                .as_ref()
                .map(|v| String::from_utf8_lossy(v).to_string())
                .unwrap_or(".".to_string()),
            self.next
                .iter()
                // .map(|(outcome, node)| format!("{outcome} -> {node:?}"))
                .collect::<Vec<_>>()
        )
    }
}
