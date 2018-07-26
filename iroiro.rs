mod algebra {
    use std::ops::{Add, Mul, Neg, Sub, Div};
    use std;

    pub trait Zero {
        const ZERO: Self;
    }
    
    pub trait Monoid: Add<Self, Output=Self>+Zero 
        where Self : std::marker::Sized{
    }

    pub trait Group: Monoid+Neg+Sub<Self, Output=Self> {
    }

    pub trait Ring: Group+Mul<Self, Output=Self> {
    }

    pub trait One {
        const ONE: Self;
    }

    pub trait Field: Ring+One+Div<Self, Output=Self> {
    }
}

impl algebra::One for ModInt {
    const ONE: ModInt = ModInt{val: 1};
}

impl algebra::Zero for ModInt {
    const ZERO: ModInt = ModInt{val: 0};
}

impl algebra::Monoid for ModInt {}

impl algebra::Group for ModInt {}

impl algebra::Ring for ModInt {}

impl algebra::Field for ModInt {}

trait MapToi64 {
    fn map_to_number(i64) -> Self;
}

impl MapToi64 for ModInt {
    fn map_to_number(x: i64) -> ModInt {
        ModInt::new(x)
    }
}

#[derive(Debug)]
struct ConvQuery<T> 
    where T: algebra::Field  {
    fac: Vec<T>,
    facinv: Vec<T>,
}

impl<T> ConvQuery<T>
    where T: algebra::Field+MapToi64+std::clone::Clone {
    
    fn new(n: usize) -> Self {
        let mut fac = Vec::with_capacity(n+1);
        let mut facinv = Vec::with_capacity(n+1);
        fac.push(T::ONE);
        for x in 1..n+1 {
            let num = {
                let last = fac.last().unwrap();
                (*last).clone() * T::map_to_number(x as i64)
            };
            fac.push(num.clone());
        }
        let mut num = T::ONE/fac.last().unwrap().clone();
        facinv.push(num.clone());
        for i in (0..n).rev() {
            num = num * T::map_to_number((i+1) as i64);
            facinv.push(num.clone());
        }
        facinv.reverse();
        Self {
            fac: fac,
            facinv: facinv,
        }
    }

    fn query(&self, n: usize, m: usize) -> T {
        self.fac[n].clone() * self.facinv[n-m].clone() * self.facinv[m].clone()
    }
}
