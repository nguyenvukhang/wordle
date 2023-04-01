use std::time::Instant;

use crate::types::{Outcome, Word};
use crate::util::outcome;
use crate::words::{display_guess, find_guess, get_guess};

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

    pub fn entropy(&mut self, guess: usize, answers: &Vec<usize>) -> f64 {
        let (mut freq, mut ent, n) = ([0; 243], 0.0, answers.len() as f64);
        for answer in answers {
            freq[self.outcome(guess, *answer) as usize] += 1;
        }
        for f in freq {
            if f > 0 {
                let f = f as f64;
                ent += (f / n) * (n / f).log2();
            }
        }
        ent
    }

    pub fn suggest(&mut self, answers: &Vec<usize>) -> (usize, f64) {
        if answers.len() == 0 {
            return (answers[0], 0.0);
        }
        if answers.len() == 1 {
            return (answers[0], 1.0);
        }
        let mut best = (0, -1.0);
        let guess_count = self.guess_count();
        for guess in 0..guess_count {
            let entropy = self.entropy(guess, answers);
            if entropy > best.1 {
                best = (guess, entropy);
            }
        }
        log::debug!("{} @ {:.8}", display_guess(best.0), best.1);
        best
    }

    pub fn answer_count(&self) -> usize {
        self.db.len()
    }

    pub fn guess_count(&self) -> usize {
        self.db[0].len()
    }

    pub fn fresh_answer_set(&self) -> Vec<usize> {
        (0..self.answer_count()).collect()
    }

    fn shrink(&self, guess: usize, outcome: Outcome, answers: &Vec<usize>) -> Vec<usize> {
        let (g, o, a) = (guess, outcome, answers.clone());
        a.into_iter().filter(|&a| self.outcome(g, a) == o).collect()
    }

    /// Fixing a guess and answer list, calculate the average entropy
    /// gained from picking the best next guess given any outcome.
    ///
    /// Incoming state:
    ///   history: (start) -> "soare"
    ///   answer list: full
    ///
    /// Processing:
    ///   if outcome is BBBBY, guess 'denet'
    ///   if outcome is BYYBG, guess 'bundt'
    ///   if outcome is BBBYG, guess 'pudic'
    ///   ... (243 outcomes)
    ///
    /// Note that 'denet' is uniquely determined by "soare" & BBBBY
    /// and that 'bundt' is uniquely determined by "soare" & BYYBG
    /// and so on.
    ///
    /// Each of these 243 uniquely determined next-guesses will have
    /// their expected information gain. We take that average and add
    /// it to "soare"'s information gain.
    ///
    /// This will be "soare"'s 2-up look-ahead expected information
    /// gain.
    pub fn entropy2(&mut self, guess: usize, answers: &Vec<usize>) -> f64 {
        let mut entropy2 = 0.0;

        for o1 in 0..243 {
            // `guess` + outcome `o1` -> results in this answer list
            let answers = self.shrink(guess, o1, answers);

            // not possible to reach an outcome of o1.
            if answers.is_empty() {
                continue;
            }

            entropy2 += self.suggest(&answers).1;
        }
        entropy2 / 243 as f64
    }

    pub fn suggest2(&mut self, answers: &Vec<usize>) -> (usize, f64) {
        let mut best = (0, -1.0);
        let guess_count = self.guess_count();
        let start = Instant::now();

        let print_count = 50;
        let printerval = guess_count / print_count;

        for g1 in 0..guess_count {
            let e1 = self.entropy(g1, answers);
            let e2 = self.entropy2(g1, answers);
            let entropy = e1 + e2;
            if entropy > best.1 {
                best = (g1, entropy);
                log::info!("{} @ {:.8}", display_guess(best.0), best.1);
            }
            if g1 % printerval == 0 {
                let elapsed = Instant::elapsed(&start);
                let avg = elapsed / (g1 + 1) as u32;
                let rem = avg * (guess_count - g1 + 1) as u32;
                println!("{g1}/{guess_count}");
                println!("{:?} | {:?}", avg, rem);
            }
        }
        log::info!("{} @ {:.8}", display_guess(best.0), best.1);
        best
    }
}
