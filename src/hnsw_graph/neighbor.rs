use crate::hnsw_graph::node::Node;
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Neighbor {
    pub node: Arc<RwLock<Node>>,
    pub distance: f32,
}

impl Neighbor {
    pub fn new(node: Arc<RwLock<Node>>, distance: f32) -> Self {
        Self { node, distance }
    }

    pub fn node(&self) -> Arc<RwLock<Node>> {
        self.node.clone()
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }
}
