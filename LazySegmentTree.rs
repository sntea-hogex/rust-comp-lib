trait Monoid {
    fn unity() -> Self;
    fn op(&self, &Self) -> Self;
}

#[derive(Clone, Debug)]
struct LazySegmentTree<T, U, F> {
    dat: Vec<T>,
    lazy: Vec<U>,
    change: F,
    length: usize,
}

impl<T, U, F> LazySegmentTree<T, U, F>
where
    T: Monoid + std::clone::Clone + std::fmt::Debug,
    U: Monoid + std::clone::Clone + std::fmt::Debug,
    F: FnMut(&mut T, U, usize, usize),
{
    // change: lazyからdatへ変更を伝播させる関数
    fn new(n: usize, init: T, change: F) -> LazySegmentTree<T, U, F> {
        let mut len = 1;
        while len < n {
            len <<= 1;
        }
        LazySegmentTree {
            dat: vec![init; len * 2],
            lazy: vec![U::unity(); len * 2],
            change: change,
            length: len,
        }
    }

    fn propagate(&mut self, v: usize, l: usize, r: usize) {
        let lv = 2 * v + 1;
        let rv = 2 * v + 2;
        let mut lval = U::unity();
        std::mem::swap(&mut lval, &mut self.lazy[v]);
        let m = (l + r) / 2;
        if lv < self.lazy.len() {
            (self.change)(&mut self.dat[lv], lval.clone(), l, m);
            if lv < self.lazy.len() / 2 - 1 {
                self.lazy[lv] = self.lazy[lv].op(&lval);
            }
        }
        if rv < self.lazy.len() {
            (self.change)(&mut self.dat[rv], lval.clone(), m, r);
            if rv < self.lazy.len() / 2 - 1 {
                self.lazy[rv] = self.lazy[rv].op(&lval);
            }
            self.dat[v] = self.dat[lv].op(&self.dat[rv]);
        }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn range_update(&mut self, l: usize, r: usize, x: U) {
        let len = self.len();
        self.range_update_(0, 0, len, l, r, x);
    }

    fn range_update_(&mut self, v: usize, a: usize, b: usize, l: usize, r: usize, x: U) {
        self.propagate(v, a, b);
        let intersect = |a, b, l, r| !(r <= a || b <= l);
        let contain = |a, b, l, r| l <= a && b <= r;

        if !intersect(a, b, l, r) {
            return;
        }

        if contain(a, b, l, r) {
            if v < self.len() - 1 {
                self.lazy[v] = self.lazy[v].op(&x);
            }
            (self.change)(&mut self.dat[v], x.clone(), a, b);
        } else {
            let m = (a + b) / 2;
            let lv = 2 * v + 1;
            let rv = 2 * v + 2;
            self.range_update_(lv, a, m, l, r, x.clone());
            self.range_update_(rv, m, b, l, r, x);

            if rv < 2 * self.len() {
                self.dat[v] = self.dat[lv].op(&self.dat[rv]);
            }
        }
    }

    fn update(&mut self, i: usize, val: T) {
        assert!(i < self.len());
        let mut p = self.dat.len() / 2 - 1 + i;
        self.dat[p] = val;
        while p != 0 {
            p = (p - 1) / 2;
            self.dat[p] = self.dat[2 * p + 1].op(&self.dat[2 * p + 2]);
        }
    }

    fn update_(&mut self, v: usize, l: usize, r: usize, i: usize, val: T) {
        self.propagate(v, l, r);
        self.dat[v] = self.dat[v].op(&val);

        let m = (l + r) / 2;
        if l <= i && i < m {
            self.update_(2 * v + 1, l, m, i, val);
        } else if m <= i && i < r {
            self.update_(2 * v + 2, m, r, i, val);
        }
    }

    fn query(&mut self, l: usize, r: usize) -> T {
        let len = self.len();
        self.query_(l, r, 0, 0, len)
    }

    fn query_(&mut self, l: usize, r: usize, i: usize, a: usize, b: usize) -> T {
        self.propagate(i, a, b);

        if l <= a && b <= r {
            self.dat[i].clone()
        } else if !(r <= a || b <= l) {
            let retl = self.query_(l, r, i * 2 + 1, a, (a + b) / 2);
            let retr = self.query_(l, r, i * 2 + 2, (a + b) / 2, b);
            retl.op(&retr)
        } else {
            T::unity()
        }
    }

    // fn get(&self, ind: usize) -> &T {
    //     &self.dat[self.dat.len() / 2 + ind - 1]
    // }

    // fn prop_all(&mut self) {
    //     let len = self.len();
    //     self.prop_all_(0, 0, len);
    // }

    // fn prop_all_(&mut self, v: usize, l: usize, r: usize) {
    //     if v >= 2 * self.len() - 1 {
    //         return;
    //     }
    //     self.propagate(v, l, r);
    //     let m = (l+r)/2;
    //     self.prop_all_(2*v+1, l, m);
    //     self.prop_all_(2*v+2, m, r);
    // }
}