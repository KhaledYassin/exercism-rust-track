pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: (0..row_count).map(|row| compute_row(row)).collect(),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn compute_row(n: u32) -> Vec<u32> {
    (0..=n)
        .rev()
        .map(|k| factorial(n) / (factorial(k) * factorial(n - k)))
        .collect()
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
