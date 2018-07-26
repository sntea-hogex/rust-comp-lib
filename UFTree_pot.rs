#[derive(Clone, Eq, PartialEq)]
struct UnionFind<Cost> 
where
    Cost: Copy + algebra::Group + Eq
{
    par: Vec<usize>,
    rank: Vec<usize>,
    pot: Vec<Cost>,
}

impl<Cost> UnionFind<Cost>
where
    Cost: Copy + algebra::Group + Eq + std::fmt::Debug
{
    fn new(n: usize) -> Self {
        UnionFind {
            par : (0..n).collect(),
            rank : vec![0;n],
            pot : vec![Cost::zero(); n],
        }
    }

    fn find(&mut self, x: usize) -> (usize, Cost) {
        if x == self.par[x] {
            (x, Cost::zero())
        } else {
            let par = self.par[x];
            let dis = self.pot[x];
            let (ind, val) = self.find(par);
            self.par[x] = ind;
            self.pot[x] = val+dis;
            (ind, val+dis)
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn unite(&mut self, a: usize, b: usize, c: Cost) -> bool {
        let (apar, apot) = self.find(a);
        let (bpar, bpot) = self.find(b);
        if apar == bpar {
            apot-bpot == c
        } else {
            if self.rank[apar] > self.rank[bpar] {
                self.par[bpar] = apar;
                self.pot[bpar] = apot-bpot-c;
            } else {
                self.par[apar] = bpar;
                self.pot[apar] = c+bpot-apot;
                if self.rank[apar] == self.rank[bpar] {
                    self.rank[bpar] += 1;
                }
            }
            true
        }
    }
}
impl algebra::Zero for i64 {
    fn zero() -> Self { 0 }
}
impl algebra::Zero for i32 {
    fn zero() -> Self { 0 }
}
