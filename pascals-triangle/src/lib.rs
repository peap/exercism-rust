pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = vec![];
        for i in 1..(row_count + 1) {
            if i == 1 {
                rows.push(vec![1]);
            } else {
                let mut row = vec![];
                {
                    // introduce scope for temporary immutable borrow of rows
                    let prev_row = &rows[(i - 2) as usize];
                    for j in 0..(prev_row.len() + 1) {
                        let left = if j == 0 { 0 } else { prev_row[j - 1] };
                        let right = if j >= prev_row.len() { 0 } else { prev_row[j] };
                        row.push(left + right);
                    }
                }
                rows.push(row);
            }
        }
        PascalsTriangle { rows: rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
