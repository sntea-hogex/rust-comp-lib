#[derive(Eq, PartialEq, Clone)]
struct PartiallyPersistentArray<T>
where
    T: std::clone::Clone,
{
    data: Vec<Vec<(usize, T)>>,
    time: usize,
}

impl<T> PartiallyPersistentArray<T>
where
    T: std::clone::Clone,
{
    fn new(n: usize, init: T) -> Self {
        Self {
            data: vec![vec![(0, init)]; n],
            time: 1,
        }
    }

    fn get_time(&self) -> usize {
        self.time
    }

    fn update(&mut self, i: usize, v: T) -> usize {
        self.data[i].push((self.time, v));
        self.time += 1;
        self.time
    }

    fn get(&self, t: usize, i: usize) -> Option<&T> {
        let mut l = 0;
        let mut r = self.data[i].len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.data[i][m].0 < t {
                l = m;
            } else {
                r = m;
            }
        }
        Some(&self.data[i][l].1)
    }
}
