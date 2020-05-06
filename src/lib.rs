pub mod dijkstra;
pub mod graph;
pub mod pbf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    id: i64,
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    id: usize,
    from: i64,
    to: i64,
    dist: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BenchData {
    pub graph: graph::Graph,
    pub sources: Vec<i64>,
    pub targets: Vec<i64>,
}
