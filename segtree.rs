trait Monoid {
    fn unity() -> Self;
    fn op(&self, &Self) -> Self;
}

#[derive(Clone)]
struct SegmentTree<T: Monoid + std::clone::Clone> {
    dat: Vec<T>,
    length: usize,
}

impl<T: Monoid + std::clone::Clone> SegmentTree<T> {
    fn new(n: usize) -> SegmentTree<T> {
        let mut len = 1;
        while len < n {
            len <<= 1;
        }
        SegmentTree {
            dat: vec![T::unity(); len*2],
            length: n,
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn update(&mut self, i: usize, val: T) {
        assert!(i < self.len());
        let mut p = self.dat.len()/2-1+i;
        self.dat[p] = val;
        while p != 0 {
            p = (p-1)/2;
            self.dat[p] = self.dat[2*p+1].op(&self.dat[2*p+2]);
        }
    }

    fn query(&self, l: usize, r: usize) -> T {
        self.query_(l, r, 0, 0, self.dat.len()/2)
    }

    fn get(&self, ind: usize) -> T {
        self.dat[self.dat.len()/2+ind-1]
    }

    fn query_(&self, l: usize, r: usize, i: usize, a: usize, b: usize) -> T {
        if l <= a && b <= r {
            self.dat[i].clone()
        } else if !(r <= a || b <= l) {
            let retl = self.query_(l, r, i*2+1, a, (a+b)/2);
            let retr = self.query_(l, r, i*2+2, (a+b)/2, b);
            retl.op(&retr)
        } else {
            T::unity()
        }
    }

    // fn debug(&self) {
    //     printvec(&self.dat);
    // }
}