use crate::graph::Graph;
use crate::NodeId;

use std::collections::BinaryHeap;

pub struct Dijkstra<'a> {
    g: &'a Graph,
}

impl<'a> Dijkstra<'a> {
    pub fn new(g: &'a Graph) -> Self {
        Dijkstra { g }
    }

    pub fn dist(&mut self, from: NodeId, to: NodeId) -> u32 {
        let mut dist = vec![u32::MAX; self.g.node_count()];
        let mut prev: Vec<Option<NodeId>> = vec![None; self.g.node_count()];
        let mut heap = BinaryHeap::new();

        heap.push(HeapElement {
            dist: 0,
            node: from,
            prev_node: from,
        });

        while let Some(HeapElement {
            dist: u_dist,
            node: u,
            prev_node,
        }) = heap.pop()
        {
            // If your heap does not support a decrease key operation, you can
            // include nodes multiple times and with the following condition
            // ensure, that each is only processed once. (This is also said to
            // perform better than decrease key, but I never benchmarked it)
            if u_dist >= dist[u.0] {
                continue;
            }

            dist[u.0] = u_dist;
            prev[u.0] = Some(prev_node);

            if u == to {
                return u_dist;
            }

            for edge in self.g.outgoing_edges_of(u) {
                let alt = u_dist + edge.dist;
                if alt < dist[edge.to.0] {
                    heap.push(HeapElement {
                        dist: alt,
                        node: edge.to,
                        prev_node: u,
                    });
                }
            }
        }

        dist[*to]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct HeapElement {
    dist: u32,
    node: NodeId,
    prev_node: NodeId,
}

// The binary heap we are using is a max-heap. Therefore, we need to define a
// custom ordering which reverses the sorting.
impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
