#[derive(Clone)]
struct ConvexHullTrick<T> {
    lines: Vec<(T, T)>,
}

impl<T> ConvexHullTrick<T>
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Sub<Output = T>
        + Copy
        + PartialOrd,
{
    fn new() -> Self {
        ConvexHullTrick { lines: Vec::new() }
    }

    fn check<'a>(mut l1: &'a (T, T), l2: &(T, T), mut l3: &'a (T, T)) -> bool {
        if l1 < l3 {
            std::mem::swap(&mut l1, &mut l3);
        }
        (l3.1 - l2.1) * (l2.0 - l1.0) >= (l2.1 - l1.1) * (l3.0 - l2.0)
    }

    fn add(&mut self, a: T, b: T) {
        while self.lines.len() >= 2
            && Self::check(
                &self.lines[self.lines.len() - 2],
                self.lines.last().expect("cht Error!!"),
                &(a, b),
            ) {
            self.lines.pop();
        }
        self.lines.push((a, b));
    }

    fn calc(&self, i: usize, x: T) -> T {
        self.lines[i].0 * x + self.lines[i].1
    }

    fn query(&self, x: T) -> T {
        if self.lines.len() == 1 {
            return self.calc(0, x);
        }

        let mut ng = 0;
        let mut ok = self.lines.len();
        let check = |i| {
            if i + 1 == self.lines.len() {
                true
            } else {
                self.calc(i, x) < self.calc(i + 1, x)
            }
        };
        if check(ng) {
            return self.calc(ng, x);
        }
        while ok-ng > 1 {
            let m = (ok + ng) / 2;
            if check(m) {
                ok = m;
            } else {
                ng = m;
            }
        }
        self.calc(ok, x)
    }
}