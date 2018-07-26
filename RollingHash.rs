macro_rules! make_modint {
    ($MOD: expr, $name: ident) => {
        #[derive(Eq, PartialEq, PartialOrd, Ord, Hash)]
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

        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.val == other.val
            }
        }

        impl std::cmp::Eq for $name {}

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.val)
            }
        }
    }
}

make_modint!(1_000_000_000+7, mint0);
make_modint!(1_000_000_000+9, mint1);
make_modint!(999_999_937, mint2);

struct Random {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
    t: i32,
}

impl Random {
    fn new() -> Random {
        Random {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 886751233,
            t: 1,
        }
    }

    fn next(&mut self) -> i32 {
        self.t = self.x^(self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w>>19)) ^ (self.t ^ (self.t>>8));
        self.w&0x7fffffff
    }
}

// construct: O(n), Query: O(1)
struct RolllingHash {
    b0: mint0,
    b1: mint1,
    b2: mint2,
    table: Vec<(mint0, mint1, mint2)>,
    power_inv: Vec<(mint0, mint1, mint2)>
}

impl RolllingHash {
    fn new(s: &str) -> RolllingHash {
        let n = s.len();
        let mut table = Vec::with_capacity(n+1);
        
        let mut rnd = Random::new();
        
        let b0 = mint0::new(rnd.next() as i64);
        let b1 = mint1::new(rnd.next() as i64);
        let b2 = mint2::new(rnd.next() as i64);
        
        let mut t0 = mint0::new(1);
        let mut t1 = mint1::new(1);
        let mut t2 = mint2::new(1);

        table.push((mint0::new(0), mint1::new(0), mint2::new(0)));
        
        let mut val0 = mint0::new(0);
        let mut val1 = mint1::new(0);
        let mut val2 = mint2::new(0);

        for (i, &e) in s.as_bytes().iter().enumerate() {
            val0 += mint0::new(e as i64)*t0;
            val1 += mint1::new(e as i64)*t1;
            val2 += mint2::new(e as i64)*t2;
            table.push((val0, val1, val2));
            t0 *= b0;
            t1 *= b1;
            t2 *= b2;
        }

        let mut powers = Vec::with_capacity(n+1);
        let mut t0 = mint0::new(1);
        let mut t1 = mint1::new(1);
        let mut t2 = mint2::new(1);
        let b0_inv = b0.inv();
        let b1_inv = b1.inv();
        let b2_inv = b2.inv();
        powers.push((t0, t1, t2));
        for i in 0..n {
            t0 *= b0_inv;
            t1 *= b1_inv;
            t2 *= b2_inv;
            powers.push((t0, t1, t2));
        }

        RolllingHash {
            b0: b0,
            b1: b1,
            b2: b2,
            table: table,
            power_inv: powers,
        }
    }

    fn query(&self, l: usize, r: usize) -> (mint0, mint1, mint2) {
        let (lval0, lval1, lval2) = self.table[l];
        let (rval0, rval1, rval2) = self.table[r];
        let (inv0, inv1, inv2) = self.power_inv[l];
        ((rval0-lval0)*inv0, (rval1-lval1)*inv1, (rval2-lval2)*inv2)
    }
}
