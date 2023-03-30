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

    pub fn trace(&self, path: &[(Outcome, Option<Word>)]) -> Option<Word> {
        if path.is_empty() {
            return self.guess;
        }
        let (outcome, word) = path.first()?;
        self.next.get(outcome)?.trace(&path[1..])
    }

    pub fn push<'a>(&'a mut self, outcome: Outcome, guess: &Word) -> &'a mut Self {
        let has = self.next.contains_key(&outcome);
        if !has {
            let guess = guess.to_owned();
            let node = Node::new(Some(guess.to_owned()));
            self.next.insert(outcome, node);
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
