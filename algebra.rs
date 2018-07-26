
macro_rules! make_modint {
    ($MOD: expr, $name: ident) => {
        #[derive(Ord, Hash, Eq, PartialOrd, PartialEq)]
        struct $name {
            val: i64,
        }

        impl $name {
            fn new(x: i64) -> $name {
                let x = x%$MOD;
                $name{val: if x < 0 { x+$MOD } else { x }}
            }

            fn pow(&self, x: i64) -> $name {
                let mut res = $name::new(1);
                let mut tmp = x;
                let mut p = *self;
                while tmp != 0 {
                    if tmp&1 == 1 {
                        res *= p;
                    }
                    tmp = tmp>>1;
                    p = p*p;
                }
                res
            }

            fn inv(&self) -> $name {
                assert!(self.val != 0);
                let mut a = self.val;
                let mut b = $MOD;
                let mut u = 1;
                let mut v = 0;
                use std::mem::swap;
                while b != 0 {
                    let t = a/b;
                    a -= t*b;
                    swap(&mut a, &mut b);
                    u -= t*v;
                    swap(&mut u, &mut v);
                }
                $name::new(u)
            }
        }

        impl std::clone::Clone for $name {
            fn clone(&self) -> $name {
                $name{ val: self.val }
            }
        }

        impl std::marker::Copy for $name { }

        impl std::ops::Add for $name {
            type Output = $name;
            fn add(self, y: $name) -> $name {
                let tmp = self.val+y.val;
                $name{val: if tmp >= $MOD {tmp-$MOD} else {tmp}}
            }
        }

        impl std::ops::Neg for $name {
            type Output = $name;
            fn neg(self) -> $name {
                $name::new(self.val)
            }
        }

        impl std::ops::Sub for $name {
            type Output = $name;
            fn sub(self, other: $name) -> $name{
                let tmp = self.val-other.val;
                $name{val: if tmp < 0 {tmp+$MOD} else {tmp}}
            }
        }

        impl std::ops::Mul for $name {
            type Output = $name;
            fn mul(self, y: $name) -> $name {
                $name{val: (self.val*y.val)%$MOD}
            }
        }

        impl std::ops::Div for $name {
            type Output = $name;
            fn div(self, other: $name) -> $name {
                self*other.inv()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.val)
            }
        }

        impl std::ops::AddAssign for $name {
        fn add_assign(&mut self, other: $name) {
            *self = *self+other;
        }
        }

        impl std::ops::SubAssign for $name {
            fn sub_assign(&mut self, other: $name) {
                *self = *self-other;
            }
        }

        impl std::ops::MulAssign for $name {
            fn mul_assign(&mut self, other: $name) {
                *self = *self*other;
            }
        }

        impl std::ops::DivAssign for $name {
            fn div_assign(&mut self, other: $name) {
                *self = *self*other.inv();
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.val)
            }
        }
    }
}

make_modint!(1_000_000_000 + 7, ModInt);

fn mint(x: i64) -> ModInt {
    ModInt::new(x)
}

mod algebra {
    use std::ops::{Add, Div, Mul, Neg, Sub};
    use std;

    pub trait Zero {
        fn zero() -> Self;
    }

    pub trait Monoid: Add<Self, Output = Self> + Zero
    where
        Self: std::marker::Sized,
    {}

    pub trait Group: Monoid + Neg<Output = Self> + Sub<Self, Output = Self> {}

    pub trait Ring: Group + One + Mul<Self, Output = Self> {}

    pub trait One {
        fn one() -> Self;
    }

    pub trait Field: Ring + Div<Self, Output = Self> {}
    
    impl<T: Add<T, Output = T> + Zero> Monoid for T {}
    impl<T: Monoid + Neg<Output = T> + Sub<T, Output = T>> Group for T {}
    impl<T: Group + One + Mul<T, Output = T>> Ring for T {}
    impl<T: Ring + Div<T, Output = T>> Field for T {}
}


impl algebra::One for ModInt {
    // const ONE: ModInt = ModInt { val: 1 };
    fn one() -> Self { ModInt{val: 1} }
}

impl algebra::Zero for ModInt {
    fn zero() -> Self { ModInt{val: 0} }
}




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
where
    T: algebra::Field,
{
    fac: Vec<T>,
    facinv: Vec<T>,
}

impl<T> ConvQuery<T>
where
    T: algebra::Field + MapToi64 + std::clone::Clone,
{
    fn new(n: usize) -> Self {
        let mut fac = Vec::with_capacity(n + 1);
        let mut facinv = Vec::with_capacity(n + 1);
        fac.push(T::one());
        for x in 1..n + 1 {
            let num = {
                let last = fac.last().unwrap();
                (*last).clone() * T::map_to_number(x as i64)
            };
            fac.push(num.clone());
        }
        let mut num = T::one() / fac.last().unwrap().clone();
        facinv.push(num.clone());
        for i in (0..n).rev() {
            num = num * T::map_to_number((i + 1) as i64);
            facinv.push(num.clone());
        }
        facinv.reverse();
        Self {
            fac: fac,
            facinv: facinv,
        }
    }

    fn query(&self, n: usize, m: usize) -> T {
        self.fac[n].clone() * self.facinv[n - m].clone() * self.facinv[m].clone()
    }
}