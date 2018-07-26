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
