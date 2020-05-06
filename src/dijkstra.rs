use crate::graph::Graph;
use crate::NodeId;

pub struct Dijkstra<'a> {
    g: &'a Graph,
}

impl<'a> Dijkstra<'a> {
    pub fn new(g: &'a Graph) -> Self {
        Dijkstra { g }
    }

    pub fn dist(&mut self, from: NodeId, to: NodeId) -> u32 {
        0
    }
}
