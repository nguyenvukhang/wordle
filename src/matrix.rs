use crate::types::{Outcome, Word};
use crate::util::outcome;

pub struct Matrix {
    db: Vec<Vec<Outcome>>,
}

impl Matrix {
    pub fn new(answers: &Vec<Word>, guesses: &Vec<Word>) -> Self {
        Self {
            db: answers
                .iter()
                .map(|a| guesses.iter().map(|g| outcome(g, a)).collect())
                .collect(),
        }
    }

    pub fn outcome(&self, guess: usize, answer: usize) -> Outcome {
        self.db[answer][guess]
    }

    pub fn entropy(&self, guess: usize, answers: &Vec<usize>) -> f64 {
        let (mut freq, mut entropy) = ([0; 243], 0.0);
        let n = answers.len() as f64;
        for answer in answers {
            freq[self.outcome(guess, *answer) as usize] += 1;
        }
        for f in freq {
            if f > 0 {
                let f = f as f64;
                entropy += (f / n) * (n / f).log2();
            }
        }
        entropy
    }

    pub fn suggest(&self, answers: &Vec<usize>) -> usize {
        let mut best = (0, -1.0);
        let guess_count = self.guess_count();
        for guess in 0..guess_count {
            let entropy = self.entropy(guess, answers);
            if entropy > best.1 {
                best = (guess, entropy);
            }
        }
        best.0
    }

    pub fn answer_count(&self) -> usize {
        self.db.len()
    }

    pub fn guess_count(&self) -> usize {
        self.db[0].len()
    }

    pub fn fresh_answer_set(&self) -> Vec<usize> {
        let n = self.answer_count();
        let mut res = Vec::with_capacity(n);
        (0..n).for_each(|i| res.push(i));
        res
    }
}
