use std::cmp::Ordering;
use std::clone::Clone;
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Edge<Cost>
where
    Cost: std::clone::Clone
{
    to: usize,
    cost: Cost,
}

impl<Cost> Ord for Edge<Cost>
where
    Cost: Ord + std::clone::Clone
{
    fn cmp(&self, other: &Edge<Cost>) -> Ordering {
        (self.cost).cmp(&(other.cost)).reverse()
    }
}

impl<Cost> PartialOrd for Edge<Cost>
where
    Cost: std::clone::Clone,
    Edge<Cost>: Ord
{
    fn partial_cmp(&self, other: &Edge<Cost>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Hash)]
struct Graph<Cost: Clone> {
    adj: Vec<Vec<Edge<Cost>>>,
}

impl<Cost: Clone> std::ops::Index<usize> for Graph<Cost> {
    type Output = Vec<Edge<Cost>>;

    fn index(&self, i: usize) -> &Vec<Edge<Cost>> {
        &self.adj[i]
    }
}

impl<Cost: Clone> std::ops::IndexMut<usize> for Graph<Cost> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Vec<Edge<Cost>> {
        &mut self.adj[i]
    }
}

impl<Cost: Clone> Graph<Cost> {

    fn new(n: usize) -> Graph<Cost> {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    fn len(&self) -> usize{
        self.adj.len()
    }

    fn add_edge(&mut self, from: usize, to_: usize, cost_: Cost) {
        self.adj[from].push(Edge{to: to_, cost: cost_});
    }

    fn add_uedge(&mut self, from: usize, to_: usize, cost_: Cost) {
        self.add_edge(from, to_, cost_.clone());
        self.add_edge(to_, from, cost_);
    }

    fn get<'a>(&'a self, p: usize) -> std::slice::Iter<'a, Edge<Cost>> {
        self.adj[p].iter()
    }
}

trait Zero {
    fn zero() -> Self;
}

impl Zero for i64 {
    fn zero() -> i64 { 0 }
}

trait Max {
    fn max_value() -> Self;
}

impl Max for i64 {
    fn max_value() -> i64 { std::i64::MAX }
}

use std::ops::Add;
impl<Cost> Graph<Cost>
where
    Cost: Ord + Clone + Add<Cost, Output=Cost> + Zero + Max + Copy,
    Edge<Cost>: Ord
{
    fn get_shortest_path(&self, from: usize) -> (Vec<Cost>, Vec<usize>) {
        let n = self.len();
        
        let mut dist = vec![Cost::max_value(); n];
        let mut pre = vec![n as usize; n];
        let mut heap = std::collections::BinaryHeap::new();
        heap.push(Edge{to: from, cost: Cost::zero()});
        dist[from] = Cost::zero();
        while let Some(p) = heap.pop() {
            for e in &(*self)[p.to] {
                if p.cost+e.cost >= dist[e.to] {
                    continue;
                }
                heap.push(Edge{to: e.to, cost: p.cost+e.cost});
                dist[e.to] = p.cost+e.cost;
                pre[e.to] = p.to;
            }
        }
        (dist, pre)
    }
}


use std::io::prelude::*;
impl<Cost: Clone> Graph<Cost> {
    fn vizualize(&self, name: &str) {
        let path = std::path::Path::new("tmp.dot");
        {
            let mut file = std::fs::File::create(path).unwrap();
            let n = self.len();
            file.write_all("digraph graph_name {\n".as_bytes()).unwrap();
            for p in 0..n {
                let s = ["    ".to_string(), p.to_string(),  "[];\n".to_string()].connect("");
                file.write_all(s.as_bytes()).unwrap();
            }
            for p in 0..n {
                for e in &self[p] {
                    file.write_all(b"    ").unwrap();
                    file.write_all(p.to_string().as_bytes()).unwrap();
                    file.write_all(b" -> ").unwrap();
                    file.write_all(e.to.to_string().as_bytes()).unwrap();
                    file.write_all(b"\n").unwrap();
                }
            }
            file.write(b"}\n").unwrap();
        }
        let mut cmd = std::process::Command::new("dot");
        cmd.arg("-T");
        cmd.arg("png");
        cmd.arg("tmp.dot");
        cmd.arg("-o");
        cmd.arg(name);
        cmd.status().expect("commend failed to start");
    }
}

