fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn extgcd(a: i64, b: i64) -> (i64, i64) {
    if b == 1 {
        (0, 1)
    } else {
        let (x, y) = extgcd(b, a % b);
        (y, x - a / b * y)
    }
}

fn eratosthenes(n: usize) -> Vec<bool> {
    let n = n + 1;
    let mut res = vec![true; n];
    res[0] = false;
    res[1] = false;
    for i in (2..).take_while(|i| i * i <= n) {
        if !res[i] {
            continue;
        }
        let mut j = i * 2;
        while j < n {
            res[j] = false;
            j += i;
        }
    }
    res
}
