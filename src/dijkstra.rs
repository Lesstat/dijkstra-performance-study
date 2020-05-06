use crate::graph::Graph;

pub struct Dijkstra<'a> {
    g: &'a Graph,
}

impl<'a> Dijkstra<'a> {
    pub fn new(g: &'a Graph) -> Self {
        Dijkstra { g }
    }

    pub fn dist(&mut self, from: i64, to: i64) -> u32 {
        0
    }
}
