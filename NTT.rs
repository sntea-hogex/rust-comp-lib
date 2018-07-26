// const MOD: i64 = 1012924417;

struct Polynomial {
    coef: Vec<ModInt>,
    root: ModInt,
}

impl Polynomial {
    fn new(n: usize, root_: ModInt) -> Polynomial {
        Polynomial {
            root: root_,
            coef: vec![ModInt::new(0); n],
        }
    }
    
    fn len(&self) -> usize {
        self.coef.len()
    }


    fn dft(&self, n: usize) -> Polynomial {
        fn bit_reverse(a: u32) -> u32 {
            let mut x = a;
            x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
            x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
            x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
            x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
            (x >> 16) | (x << 16)
        }

        let mut iv: Vec<_> = (0..n).map(|x| bit_reverse(x as u32)).collect();
        iv.sort();
        let mut res = Polynomial::new(n, self.root);
        for i in 0..n {
            let ind = bit_reverse(iv[i]) as usize;
            res.coef[i] = if ind < self.coef.len(){
                self.coef[ind]
            } else {
                ModInt::new(0)
            };
        }
        let mut l = 2;
        while l <= n {
            let theta = self.root.pow((MOD-1)/l as i64);
            let mut s = 0;
            while s < n {
                let mut t = ModInt::new(1);
                for i in 0..l/2 {
                    let t1 = res.coef[s+i]+t*res.coef[s+l/2+i];
                    let t2 = res.coef[s+i]-t*res.coef[s+l/2+i];
                    res.coef[s+i] = t1;
                    res.coef[s+l/2+i] = t2;
                    t *= theta;
                }
                s += l;
            }
            l <<= 1;
        }

        res
    }

    fn inverse_dft(&self) -> Polynomial {
        let n = self.len();
        let mut res = Polynomial::new(n, self.root.inv());
        for i in 0..self.len() {
            res.coef[i] = self.coef[i];
        }
        res = res.dft(n);
        for i in 0..res.len() {
            res.coef[i] /= ModInt::new(res.len() as i64);
        }
        res
    }

}

impl std::ops::Mul for Polynomial {
    type Output = Polynomial;
    fn mul(self, other: Polynomial) -> Polynomial {
        let m = self.len()+other.len();
        let mut n = 1;
        while n < m {
            n <<= 1;
        }
        let gg = self.dft(n);
        let hh = other.dft(n);
        let mut res = Polynomial::new(n, self.root);
        for i in 0..n {
            res.coef[i] = gg.coef[i]*hh.coef[i];
        }
        res.inverse_dft()
    }
}
