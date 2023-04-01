use crate::outcome::outcome;
use crate::types::Word;

pub struct Entropy {
    /// db[total][freq] -> entropy
    db: Vec<Vec<f64>>,
}

impl Entropy {
    pub fn new(answers_count: usize) -> Self {
        eprintln!("building freq-total-entropy calculator...");
        let mut db = vec![];
        let t = answers_count;
        for total in 0..t {
            let mut row = Vec::with_capacity(total + 1);
            row.push(0.0);
            for freq in 1..total + 1 {
                let (f, n) = (freq as f64, total as f64);
                let e = (f / n) * (n / f).log2();
                row.push(e);
            }
            db.push(row);
        }
        eprintln!("done building freq-total-entropy calculator!");
        Self { db }
    }

    pub fn get(&self, freq: usize, total: usize) -> f64 {
        self.db[total][freq]
    }
}

/// Calculates the entropy (information stood to gain) of a guess
/// against a known list of possible answers
pub fn entropy(guess: &Word, answers: &Vec<Word>) -> f64 {
    let (mut freq, mut entropy, len) = ([0; 243], 0.0, answers.len() as f64);
    for answer in answers {
        freq[outcome(guess, &answer) as usize] += 1;
    }
    for f in freq {
        if f > 0 {
            let f = f as f64;
            entropy += (f / len) * (len / f).log2();
        }
    }
    entropy
}

#[test]
fn entropy_test() {
    use crate::words;
    let answers = words::answers();
    macro_rules! test {
        ($word:expr, $val:expr) => {
            assert_eq!(entropy($word, &answers), $val)
        };
    }
    test!(b"soare", 5.885202744292758);
    test!(b"roate", 5.884856313732008);
    test!(b"raise", 5.878302956493169);
    test!(b"reast", 5.867738020843562);
    test!(b"reast", 5.867738020843562);
}
