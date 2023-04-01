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
