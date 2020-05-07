use crate::graph::Graph;
use crate::NodeId;

use std::collections::BinaryHeap;

pub struct Dijkstra<'a> {
    g: &'a Graph,
    dist: Vec<u32>,
    prev: Vec<Option<NodeId>>,
    heap: BinaryHeap<HeapElement>,
    touched: Vec<NodeId>,
    last_from: NodeId,
}

impl<'a> Dijkstra<'a> {
    pub fn new(g: &'a Graph) -> Self {
        let dist = vec![u32::MAX; g.node_count()];
        let prev = vec![None; g.node_count()];
        let heap = BinaryHeap::new();
        let touched = Vec::new();
        Dijkstra {
            g,
            dist,
            prev,
            heap,
            touched,
            last_from: NodeId(usize::MAX),
        }
    }

    pub fn reset_state(&mut self) {
        for t in &self.touched {
            self.dist[t.0] = u32::MAX;
            self.prev[t.0] = None;
        }
        self.heap.clear();
        self.touched.clear();
    }

    pub fn dist(&mut self, from: NodeId, to: NodeId) -> u32 {
        // If the query starts from the same node as before we can reuse it
        if self.last_from == from {
            if self.dist[to.0] < u32::MAX {
                return self.dist[to.0];
            }
        } else {
            // If not we initialize it normally
            self.last_from = from;
            self.reset_state();

            self.heap.push(HeapElement {
                dist: 0,
                node: from,
                prev_node: from,
            });
        }

        while let Some(HeapElement {
            dist: u_dist,
            node: u,
            prev_node,
        }) = self.heap.pop()
        {
            // If your heap does not support a decrease key operation, you can
            // include nodes multiple times and with the following condition
            // ensure, that each is only processed once. (This is also said to
            // perform better than decrease key, but I never benchmarked it)
            if u_dist >= self.dist[u.0] {
                continue;
            }

            self.dist[u.0] = u_dist;
            self.prev[u.0] = Some(prev_node);
            self.touched.push(u);

            for edge in self.g.outgoing_edges_of(u) {
                let alt = u_dist + edge.dist;
                if alt < self.dist[edge.to.0] {
                    self.heap.push(HeapElement {
                        dist: alt,
                        node: edge.to,
                        prev_node: u,
                    });
                }
            }
            // We moved this down here to have the heap in a consistent state
            // (all outgoing neighbors of `to` are in the heap)
            if u == to {
                return u_dist;
            }
        }

        self.dist[*to]
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
