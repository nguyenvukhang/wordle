use crate::types::{Outcome, Word};
use crate::util::outcome;
use crate::words::{find_guess, get_guess};

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

    pub fn suggest(&mut self, answers: &Vec<usize>) -> usize {
        let mut best = (0, -1.0);
        let guess_count = self.guess_count();
        for guess in 0..guess_count {
            let entropy = self.entropy(guess, answers);
            if entropy > best.1 {
                best = (guess, entropy);
            }
        }
        log::info!("{} @ {:.8}", get_guess(best.0).unwrap_or_default(), best.1);
        best.0
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

    fn out_freq(&self, guess: usize, answers: &Vec<usize>) -> Vec<usize> {
        let (mut v, g, a) = (
            (0..243).map(|_| 0).collect::<Vec<_>>(),
            guess,
            answers.iter(),
        );
        a.for_each(|a| v[self.outcome(g, *a) as usize] += 1);
        v
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
        let (g1, n) = (guess, self.guess_count());
        let out_freq = self.out_freq(g1, answers);
        let mut entropy2 = 0.0;

        for o1 in 0..1 {
            // `guess` will never lead to this outcome.
            // example: guess is "iiiii" gives outcome GGGGG.
            if out_freq[o1] == 0 {
                continue;
            }

            // `guess` + outcome `o1` -> results in this answer list
            let _ = self.shrink(g1, o1 as Outcome, answers);

            for g2 in 0..n {
                if g1 == g2 {
                    continue;
                }
            }
        }
        entropy2 /= (n * n) as f64;
        entropy2
    }
}
