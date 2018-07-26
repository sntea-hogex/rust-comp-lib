trait BinarySearch {
    type E;
    fn lower_bound(&self, &(Self::E)) -> usize;
    fn upper_bound(&self, &(Self::E)) -> usize;
}

impl<T> BinarySearch for Vec<T>
        where T: std::cmp::Ord {
    type E = T;

    fn lower_bound(&self, e: &(Self::E)) -> usize {
         if self[0] >= *e {
             0
         } else {
            let mut ng = 0;
            let mut ok = self.len();
            while ok-ng > 1 {
                let m = (ok+ng)/2;
                if self[m] >= *e {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            ok
         }
    }
    
    fn upper_bound(&self, e: &(Self::E)) -> usize {
        if self[0] > *e {
             0
         } else {
            let mut ng = 0;
            let mut ok = self.len();
            while ok-ng > 1 {
                let m = (ok+ng)/2;
                if self[m] > *e {
                    ok = m;
                } else {
                    ng = m;
                }
            }
            ok
         }
    }
}