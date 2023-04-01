use std::collections::HashMap;
use std::time::Instant;

use crate::types::{outcome_str, Outcome, Word};
use crate::util::{outcome, st};
use crate::words::GUESSES;

struct EntropyCache {
    db: HashMap<usize, HashMap<Vec<usize>, f64>>,
}

impl EntropyCache {
    fn new() -> Self {
        Self { db: HashMap::new() }
    }

    fn set(&mut self, guess: usize, answers: &Vec<usize>, entropy: f64) {
        let ht = match self.db.get_mut(&guess) {
            Some(v) => v,
            None => {
                self.db.insert(guess, HashMap::new());
                self.db.get_mut(&guess).unwrap()
            }
        };
        ht.insert(answers.clone(), entropy);
    }

    fn get(&self, guess: usize, answers: &Vec<usize>) -> Option<f64> {
        self.db
            .get(&guess)
            .map(|ht| ht.get(answers))
            .flatten()
            .map(|v| *v)
    }
}

pub struct Matrix {
    db: Vec<Vec<Outcome>>,
    entropy_cache: EntropyCache,
}

fn vec<F: Fn(usize) -> T, T>(len: usize, f: F) -> Vec<T> {
    let mut vec = Vec::with_capacity(len);
    (0..len).for_each(|i| vec.push(f(i)));
    vec
}

fn build_entropy(freq: [usize; 243], total: usize) -> f64 {
    let (mut entropy, n) = (0.0, total as f64);
    for f in freq {
        if f > 0 {
            let f = f as f64;
            entropy += (f / n) * (n / f).log2();
        }
    }
    entropy
}

impl Matrix {
    pub fn new(answers: &Vec<Word>, guesses: &Vec<Word>) -> Self {
        Self {
            db: answers
                .iter()
                .map(|a| guesses.iter().map(|g| outcome(g, a)).collect())
                .collect(),
            entropy_cache: EntropyCache::new(),
        }
    }

    pub fn outcome(&self, guess: usize, answer: usize) -> Outcome {
        self.db[answer][guess]
    }

    pub fn entropy_cached(&mut self, guess: usize, answers: &Vec<usize>) -> f64 {
        if let Some(v) = self.entropy_cache.get(guess, answers) {
            return v;
        }
        let entropy = self.entropy(guess, answers);
        self.entropy_cache.set(guess, answers, entropy);
        entropy
    }

    pub fn entropy(&mut self, guess: usize, answers: &Vec<usize>) -> f64 {
        let mut freq = [0; 243];
        for answer in answers {
            freq[self.outcome(guess, *answer) as usize] += 1;
        }
        build_entropy(freq, answers.len())
    }

    fn out_freq(&self, guess: usize, answers: &Vec<usize>) -> Vec<usize> {
        let (mut v, g, a) = (vec(243, |_| 0), guess, answers.iter());
        a.for_each(|a| v[self.outcome(g, *a) as usize] += 1);
        v
    }

    fn shrink(&self, guess: usize, outcome: Outcome, answers: &Vec<usize>) -> Vec<usize> {
        let (g, o, a) = (guess, outcome, answers.clone());
        a.into_iter().filter(|&a| self.outcome(g, a) == o).collect()
    }

    pub fn entropy2(&mut self, guess: usize, answers: &Vec<usize>) -> f64 {
        let start = Instant::now();
        let (g1, n) = (guess, self.guess_count());
        let out_freq = self.out_freq(g1, answers);
        let mut entropy2 = 0.0;
        for o1 in 0..243 {
            // `guess` will never lead to this outcome.
            // example: guess is "iiiii" gives outcome GGGGG.
            if out_freq[o1 as usize] == 0 {
                continue;
            }

            // `guess` + outcome `o1` -> results in this answer list
            let answers = self.shrink(g1, o1, answers);

            for g2 in 0..n {
                if g1 == g2 {
                    continue;
                }
                let mut freq = [0; 243];
                for a in &answers {
                    freq[self.outcome(g2, *a) as usize] += 1;
                }
                let ent = build_entropy(freq, answers.len());
                entropy2 += ent * out_freq[o1 as usize] as f64;
            }

            println!("`{}` -> {}:", st(GUESSES[guess]), outcome_str(o1));
        }
        entropy2 /= (n * n) as f64;
        println!("{g1} -> 2nd: {}", entropy2);
        println!("elapsed: {:?}", Instant::elapsed(&start) * n as u32);
        entropy2
    }

    pub fn suggest(&mut self, answers: &Vec<usize>) -> usize {
        0
    }

    pub fn answer_count(&self) -> usize {
        self.db.len()
    }

    pub fn guess_count(&self) -> usize {
        self.db[0].len()
    }

    pub fn fresh_answer_set(&self) -> Vec<usize> {
        vec(self.answer_count(), |i| i)
    }
}
