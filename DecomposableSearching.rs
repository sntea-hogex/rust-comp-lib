trait Monoid {
    fn unity() -> Self;
    fn op(self, Self) -> Self;
}

trait Merge {
    fn merge(&mut self, Self);
}

trait DecomposableSet {
    type E;
    type R: Monoid;
    fn query(&self, &Self::E) -> Self::R;
    fn construct(Self::E) -> Self;
    fn len(&self) -> usize;
}

struct EnDynamic<T: DecomposableSet + Merge> {
    sets: Vec<T>,
}

impl<S> EnDynamic<S> where
    S: DecomposableSet+Merge{
    fn new() -> EnDynamic<S> {
        EnDynamic {
            sets: Vec::new(),
        }
    }
    
    fn add(&mut self, e: S::E) {
        let mut t = S::construct(e);
        while let Some(top) = self.sets.pop(){
            if top.len() != t.len() {
                self.sets.push(top);
                break;
            }
            t.merge(top);
        }
        self.sets.push(t);
    }

    fn query(&self, x: &S::E) -> S::R {
        let mut res = S::R::unity();
        for e in &self.sets {
            res = res.op(e.query(x));
        }
        res
    }
}

// impl Monoid for bool {
//     fn unity() -> bool {
//         false
//     }

//     fn op(self, rhs: bool) -> bool {
//         self || rhs
//     }
// }

// #[derive(Debug)]
// struct SortedVec<T>{
//     dat: Vec<T>,
// }

// impl<T> Merge for SortedVec<T>  where 
//     T: std::cmp::PartialOrd+std::clone::Clone {
//     fn merge(&mut self, v: SortedVec<T>) {
//         let mut mv = Vec::new();
//         let mut p = 0;
//         for e in v.dat {
//             while p < self.dat.len() && self.dat[p] <= e {
//                 mv.push(self.dat[p].clone());
//                 p += 1;
//             }
//             mv.push(e);
//         }
//         while p < self.dat.len() {
//             mv.push(self.dat[p].clone());
//             p += 1;
//         }
//         self.dat = mv;
//     }
// }

// impl<T> DecomposableSet for SortedVec<T> where
//     T: std::cmp::PartialOrd  {
//     type E = T;
//     type R = bool;
    
//     fn query(&self, x: &Self::E) -> Self::R {
//         if *x < self.dat[0] {
//             false
//         } else {
//             let mut l = 0;
//             let mut r = self.len();
//             while l+1 < r {
//                 let m = (l+r)/2;
//                 if self.dat[m] <= *x {
//                     l = m;
//                 } else {
//                     r = m;
//                 }
//             }
//             *x == self.dat[l]
//         }
//     }

//     fn construct(x: Self::E) -> Self {
//         SortedVec{ dat: vec![x] }
//     }
    
//     fn len(&self) -> usize {
//         self.dat.len()
//     }
// }
