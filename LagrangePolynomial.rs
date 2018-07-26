#[derive(Debug, Eq, PartialEq)]
struct LagrangePolynomial<T>
where
    T: std::hash::Hash + std::fmt::Debug + Eq + PartialEq,
{
    c: Vec<T>,
    values: Vec<(T, T)>,
}

impl<T> LagrangePolynomial<T>
where
    T: algebra::Field + std::clone::Clone + Eq + std::fmt::Debug + std::hash::Hash,
{
    fn new(values: Vec<(T, T)>) -> Self {
        let c = values
            .iter()
            .map(|&(ref x, ref y)| {
                let mul = values.iter().fold(T::One(), |mul, &(ref t, _)| {
                    if t != x {
                        mul * (x.clone() - t.clone())
                    } else {
                        mul
                    }
                });
                y.clone() / mul
            })
            .collect();
        LagrangePolynomial {
            c: c,
            values: values,
        }
    }

    fn calc(&self, x: T) -> T {
        let mut get = None;
        for &(ref k, ref v) in &self.values {
            if *k == x {
                get = Some(v);
            }
        }
        if let Some(v) = get {
            v.clone()
        } else {
            let mul = self.values
                .iter()
                .fold(T::One(), |mul, &(ref a, _)| mul * (x.clone() - a.clone()));

            self.c
                .iter()
                .zip(self.values.iter().map(|&(ref a, _)| a))
                .fold(T::Zero(), |sum, (c, a)| {
                    sum + c.clone() * mul.clone() / (x.clone() - a.clone())
                })
        }
    }
}

impl LagrangePolynomial<ModInt> {
    fn fast_new(values: Vec<ModInt>) -> Self {
        use algebra::*;
        let n = values.len();
        let values: Vec<_> = (0..n)
            .map(|x| ModInt::new(x as i64))
            .zip(values.into_iter())
            .collect();
        let c = values
            .iter()
            .scan(
                (1..n)
                    .map(|x| ModInt::new(-(x as i64)))
                    .fold(ModInt::One(), |mul, x| mul * x),
                |mul, &(x, y)| {
                    let ret = y.clone() / *mul;
                    if x.val != n as i64 - 1 {
                        *mul = *mul * (x + ModInt::new(1)) / (ModInt::new(-(n as i64) + 1) + x);
                    }
                    Some(ret)
                },
            )
            .collect();
        LagrangePolynomial {
            values: values,
            c: c,
        }
    }
}
