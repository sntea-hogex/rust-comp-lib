#[allow(dead_code)]
fn getline() -> String {
    let mut res = String::new();
    std::io::stdin().read_line(&mut res).ok();
    res
}

#[allow(unused_macros)]
macro_rules! readl {
    ($t: ty) => {
        {
            let s = getline();
            s.trim().parse::<$t>().unwrap()
        }
    };
    ($( $t: ty),+ ) => {
        {
            let s = getline();
            let mut iter = s.trim().split(' ');
            ($(iter.next().unwrap().parse::<$t>().unwrap(),)*)
        }
    };
}

#[allow(unused_macros)]
macro_rules! readlvec {
    ($t: ty) => {
        {
            let s = getline();
            let iter = s.trim().split(' ');
            iter.map(|x| x.parse().unwrap()).collect::<Vec<$t>>()
        }
    }
}

#[allow(unused_macros)]
macro_rules! debug { ($x: expr) => { println!("{}: {:?}", stringify!($x), $x) } }
// macro_rules! debug { ($x: expr) => () }

#[allow(dead_code)]
fn show<T>(iter: T) -> String
where
    T: Iterator,
    T::Item: std::fmt::Display
{
    let mut res = iter.fold(String::new(), |sum, e| sum + &format!("{} ", e));
    res.pop();
    res
}

#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)]
use std::collections::btree_map::Entry::{Occupied, Vacant};

fn main() {
    use std::io::Write;
    let out = std::io::stdout();
    let mut out = std::io::BufWriter::new(out.lock());
    macro_rules! printf { ($($arg:tt)*) => (write!(out, $($arg)*).unwrap()); }
    macro_rules! printfln { () => (writeln!(out).unwrap()); ($($arg:tt)*) => (writeln!(out, $($arg)*).unwrap()); }
    
    
}
