use std::collections::HashMap;
use std::fmt;

use crate::types::Word;
use crate::util::st;

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
        println!(
            "PUSHING {:?} <- {:?} ({:?})",
            self,
            st(guess),
            path.iter().map(st).collect::<Vec<_>>()
        );
        let front = match path.first() {
            None => {
                self.guessed = Some(guess.to_owned());
                println!("RESULT {:?} <- {:?}", self, st(guess));
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
        println!("RESULT {:?} <- {:?}", self, st(guess));
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Node ({:?}, {:?})",
            self.guessed.as_ref().map(|v| String::from_utf8_lossy(v)),
            self.next
                .iter()
                .map(|v| String::from_utf8_lossy(v.0))
                .collect::<Vec<_>>()
        )
    }
}
