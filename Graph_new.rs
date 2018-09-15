use std::cmp::Ordering;
use std::clone::Clone;
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Edge<Cost>
where
    Cost: std::clone::Clone,
{
    to: usize,
    cost: Cost,
}

impl<Cost> Ord for Edge<Cost>
where
    Cost: Ord + std::clone::Clone,
{
    fn cmp(&self, other: &Edge<Cost>) -> Ordering {
        (self.cost).cmp(&(other.cost)).reverse()
    }
}

impl<Cost> PartialOrd for Edge<Cost>
where
    Cost: std::clone::Clone,
    Edge<Cost>: Ord,
{
    fn partial_cmp(&self, other: &Edge<Cost>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Graph
where
    Self::Cost: Clone
{
    type Cost;
    fn len(&self) -> usize;
    fn get<'a>(&'a self, p: usize) -> std::slice::Iter<'a, Edge<Self::Cost>>;
    fn get_order(&self) -> Vec<usize>;
}

#[derive(Clone, Debug, Hash)]
struct AdjList<Cost: Clone> {
    adj: Vec<Vec<Edge<Cost>>>,
}

impl<Cost: Clone> std::ops::Index<usize> for AdjList<Cost> {
    type Output = Vec<Edge<Cost>>;

    fn index(&self, i: usize) -> &Vec<Edge<Cost>> {
        &self.adj[i]
    }
}

impl<Cost: Clone> std::ops::IndexMut<usize> for AdjList<Cost> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<Edge<Cost>> {
        &mut self.adj[i]
    }
}

impl<Cost: Clone> AdjList<Cost> {
    fn new(n: usize) -> AdjList<Cost> {
        AdjList {
            adj: vec![vec![]; n],
        }
    }

    fn add_edge(&mut self, from: usize, to_: usize, cost_: Cost) {
        self.adj[from].push(Edge {
            to: to_,
            cost: cost_,
        });
    }

    fn add_uedge(&mut self, from: usize, to_: usize, cost_: Cost) {
        self.add_edge(from, to_, cost_.clone());
        self.add_edge(to_, from, cost_);
    }
}

impl<Cost> Graph for AdjList<Cost>
where
    Cost: Clone
{
    type Cost = Cost;

    fn len(&self) -> usize {
        self.adj.len()
    }

    fn get<'a>(&'a self, p: usize) -> std::slice::Iter<'a, Edge<Cost>> {
        self.adj[p].iter()
    }

    fn get_order(&self) -> Vec<usize> {
        let mut res = Vec::with_capacity(self.len());
        let mut deg = vec![0; self.len()];
        let mut used = vec![false; self.len()];
        for from in 0..self.len() {
            for e in self.get(from) {
                deg[e.to] += 1;
            }
        }
        for s in 0..self.len() {
            if deg[s] != 0 || used[s] {
                continue;
            }
            let mut stack = Vec::new();
            used[s] = true;
            stack.push(s);
            while let Some(p) = stack.pop() {
                used[p] = true;
                res.push(p);
                for e in self.get(p) {
                    deg[e.to] -= 1;
                    if deg[e.to] == 0 {
                        stack.push(e.to);
                    }
                }
            }

        }
        // debug!(deg);
        res
    }
}

// --------------------

trait Zero {
    fn zero() -> Self;
}

impl Zero for i64 {
    fn zero() -> i64 {
        0
    }
}

trait Max {
    fn max_value() -> Self;
}

impl Max for i64 {
    fn max_value() -> i64 {
        std::i64::MAX
    }
}

trait GraphHavingOrderedCost: Graph
where
    Self::Cost: Ord + Add<Self::Cost, Output = Self::Cost> + Zero + Max + Copy,
{
    fn get_shortest_path(&self, from: usize) -> (Vec<Self::Cost>, Vec<usize>);
    fn bellmanford(&self, from: usize) -> (Vec<Self::Cost>, Vec<bool>);
    fn get_shortest_path_graph(&self, from: usize, dist: &Vec<Self::Cost>) -> AdjList<Self::Cost>;
}

use std::ops::Add;
impl<T> GraphHavingOrderedCost for T
where
    T: Graph,
    T::Cost: Ord + Add<T::Cost, Output = T::Cost> + Zero + Max + Copy,
    Edge<T::Cost>: Ord,
{
    fn get_shortest_path(&self, from: usize) -> (Vec<Self::Cost>, Vec<usize>) {
        let n = self.len();

        let mut dist = vec![Self::Cost::max_value(); n];
        let mut pre = vec![n as usize; n];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Edge {
            to: from,
            cost: Self::Cost::zero(),
        });
        dist[from] = Self::Cost::zero();
        while let Some(p) = heap.pop() {
            for e in self.get(p.to) {
                if p.cost + e.cost >= dist[e.to] {
                    continue;
                }
                heap.push(Edge {
                    to: e.to,
                    cost: p.cost + e.cost,
                });
                dist[e.to] = p.cost + e.cost;
                pre[e.to] = p.to;
            }
        }
        (dist, pre)
    }

    fn bellmanford(&self, from: usize) -> (Vec<Self::Cost>, Vec<bool>) {
        let n = self.len();
        let mut dist = vec![Self::Cost::max_value(); n];
        let mut updated = vec![true; n];
        dist[from] = Self::Cost::zero();
        for _ in 0..n+1 {
            updated = vec![false; n];
            for from in 0..n {
                if dist[from] == Self::Cost::max_value() {
                    continue;
                }
                for e in self.get(from) {
                    if dist[e.to] > dist[from]+e.cost {
                        dist[e.to] = dist[from]+e.cost;
                        updated[e.to] = true;
                    }
                }
            }
        }
        (dist, updated)
    }

    fn get_shortest_path_graph(&self, from: usize, dist: &Vec<Self::Cost>) -> AdjList<Self::Cost> {
        let n = self.len();

        let mut res = AdjList::new(n);
        for from in 0..n {
            for e in self.get(from) {
                if dist[from] + e.cost == dist[e.to] {
                    res.add_edge(from, e.to, e.cost);
                }
            }
        }
        res
    }
}


impl<Cost: Clone> AdjList<Cost> {
    fn vizualize(&self, name: &str) {
        let path = std::path::Path::new("tmp.dot");
        {
            use std::io::{BufWriter, Write};

            let file = std::fs::File::create(path).unwrap();
            let mut out = BufWriter::new(file);
            let n = self.len();
            writeln!(&mut out, "digraph graph_name {{").unwrap();
            for p in 0..n {
                writeln!(&mut out, "    {}[];",  p);
            }
            for p in 0..n {
                for e in &self[p] {
                    writeln!(&mut out, "    {} -> {}", p, e.to);
                }
            }
            writeln!(&mut out, "}}");
        }
        let mut cmd = std::process::Command::new("dot");
        cmd.arg("-T");
        cmd.arg("png");
        cmd.arg("tmp.dot");
        cmd.arg("-o");
        cmd.arg(name);
        cmd.status().expect("failed to execute dot");
    }
}

impl<Cost: Clone + std::fmt::Display> AdjList<Cost> {
    fn viz_with_cost(&self, name: &str) {
        let path = std::path::Path::new("tmp.dot");
        {
            use std::io::{BufWriter, Write};

            let file = std::fs::File::create(path).unwrap();
            let mut out = BufWriter::new(file);
            let n = self.len();
            writeln!(&mut out, "digraph graph_name {{").unwrap();
            for p in 0..n {
                writeln!(&mut out, "    {}[];",  p);
            }
            for p in 0..n {
                for e in &self[p] {
                    writeln!(&mut out, "    {} -> {} [label={}]", p, e.to, e.cost);
                }
            }
            writeln!(&mut out, "}}");
        }
        let mut cmd = std::process::Command::new("dot");
        cmd.arg("-T");
        cmd.arg("png");
        cmd.arg("tmp.dot");
        cmd.arg("-o");
        cmd.arg(name);
        cmd.status().expect("failed to execute dot");
    }
}