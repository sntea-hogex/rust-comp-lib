use std::ops::{Add, Sub, AddAssign, SubAssign, Neg};
struct CumSum2D<T>
where
    T: algebra::Group,
{
    table: Vec<Vec<T>>,
}

impl<T> CumSum2D<T>
where
    T: algebra::Group+std::marker::Copy,
{
    fn new(vals: &Vec<Vec<T>>) -> Self {
        let n = vals.len();
        let m = vals[0].len();
        let mut table = vec![vec![T::ZERO; m+1]; n+1];
        for i in 0..n {
            for j in 0..m {
                table[i+1][j+1] = vals[i][j]+table[i][j+1]+table[i+1][j]-table[i][j];
            }
        }
        Self {
            table: table,
        }
    }

    fn query(&self, lx: usize, ly: usize, rx: usize, ry: usize) -> T {
        self.table[rx][ry]-self.table[rx][ly]-self.table[lx][ry]+self.table[lx][ly]
    }
}
