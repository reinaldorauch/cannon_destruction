#[derive(Debug)]
pub struct Matrix {
    pub buf: Vec<usize>,
    pub cols: usize,
    pub rows: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            buf: vec![0; rows * cols],
            cols,
            rows,
        }
    }

    pub fn set(self: &mut Self, x: usize, y: usize, val: usize) {
        self.buf[x * self.rows + y] = val;
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        self.buf[x * self.rows + y]
    }
}

pub fn mochila(capacity: usize, weights: &Vec<usize>, values: &Vec<usize>, total: usize) -> usize {
    let mut results = Matrix::new(capacity + 1, total + 1);

    for i in 1..(total + 1) {
        for c in 1..(capacity + 1) {
            let new_val = if weights[i - 1] <= c {
                let with_item = values[i - 1] + results.get(i - 1, c - weights[i - 1]);
                let without_item = results.get(i - 1, c);
                if with_item >= without_item {
                    with_item
                } else {
                    without_item
                }
            } else {
                results.get(i - 1, c)
            };

            results.set(i, c, new_val);
        }
    }

    results.get(total, capacity)
}
