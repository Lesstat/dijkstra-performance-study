use serde::{Deserialize, Serialize};

use crate::{Edge, Node};
use std::convert::TryInto;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    offset: Vec<usize>,
    half_edges: Vec<HalfEdge>,
}

impl Graph {
    pub fn new(mut nodes: Vec<Node>, mut edges: Vec<Edge>) -> Graph {
        Graph::rename(&mut nodes, &mut edges);
        let mut offset = vec![0; nodes.len() + 1];

        edges.sort_by_key(|e| e.from);
        edges.iter_mut().enumerate().for_each(|(i, e)| e.id = i);
        let half_edges = edges
            .iter()
            .map(|e| HalfEdge {
                to: e.to,
                dist: e.dist,
            })
            .collect();

        edges.iter().for_each(|e| offset[e.from as usize + 1] += 1);
        for i in 1..offset.len() {
            offset[i] += offset[i - 1]
        }

        Graph {
            nodes,
            edges,
            offset,
            half_edges,
        }
    }
    fn rename(nodes: &mut [Node], edges: &mut [Edge]) {
        use std::collections::HashMap;
        nodes.sort_by_key(|n| n.id);

        let old_id_to_new_id: HashMap<_, _> =
            nodes.iter().enumerate().map(|(i, n)| (n.id, i)).collect();

        edges.iter_mut().for_each(|e| {
            e.from = old_id_to_new_id[&e.from].try_into().unwrap();
            e.to = old_id_to_new_id[&e.to].try_into().unwrap();
        })
    }

    pub fn outgoing_edges_of(&self, node_id: i64) -> &[HalfEdge] {
        let node_id = node_id as usize;
        &self.half_edges[self.offset[node_id]..self.offset[node_id + 1]]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn node(&self, node_id: i64) -> &Node {
        &self.nodes[node_id as usize]
    }
    pub fn edge(&self, edge_id: i64) -> &Edge {
        &self.edges[edge_id as usize]
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct HalfEdge {
    pub to: i64,
    pub dist: u32,
}
