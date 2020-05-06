pub mod dijkstra;
pub mod graph;
pub mod pbf;

use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub const BENCH_FILE: &'static str = "resources/bench.data";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Clone, Copy, Hash)]
pub struct NodeId(usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: i64,
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    id: usize,
    from: NodeId,
    to: NodeId,
    dist: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchData {
    pub graph: graph::Graph,
    pub sources: Vec<NodeId>,
    pub targets: Vec<NodeId>,
}

impl Ord for NodeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl Deref for NodeId {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<usize> for NodeId {
    fn from(source: usize) -> Self {
        Self(source)
    }
}

impl From<i64> for NodeId {
    fn from(source: i64) -> Self {
        Self(source as usize)
    }
}
