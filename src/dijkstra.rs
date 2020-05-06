use crate::graph::Graph;
use crate::NodeId;

use std::collections::BTreeSet;

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
        let mut q: BTreeSet<_> = (0..self.g.node_count()).map(NodeId).collect();

        dist[from.0] = 0;

        while let Some(u) = Self::minimum_vertex(&q, &dist) {
            q.remove(&u);

            for edge in self.g.outgoing_edges_of(u) {
                let alt = dist[u.0] + edge.dist;
                if alt < dist[edge.to.0] {
                    dist[edge.to.0] = alt;
                    prev[edge.to.0] = Some(u);
                }
            }
        }

        dist[*to]
    }

    fn minimum_vertex(q: &BTreeSet<NodeId>, dist: &Vec<u32>) -> Option<NodeId> {
        let mut min_node = (NodeId(usize::MAX), u32::MAX);

        for v in q {
            let v_dist = dist[v.0];
            if v_dist < min_node.1 {
                min_node = (*v, v_dist);
            }
        }

        if min_node.1 < u32::MAX {
            Some(min_node.0)
        } else {
            None
        }
    }
}
