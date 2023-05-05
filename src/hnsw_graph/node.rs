use crate::hnsw_graph::neighbor::Neighbor;
use std::cmp::Ordering;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Clone)]
pub struct Node {
    //The cid of the image, which can also be used as the key of the node
    pub cid: String,
    pub vector: Vec<f32>,
    pub layer: i32,
    pub connections: HashMap<i32, Vec<Neighbor>>,
}

impl Node {
    pub fn new(cid: String, vector: Vec<f32>, layer: i32) -> Self {
        Node {
            cid,
            vector,
            layer,
            connections: HashMap::new(),
        }
    }

    pub fn distance(&self, vec: &[f32]) -> f32 {
        let mut distance = 0.0;
        for i in 0..self.vector.len() {
            distance += (self.vector[i] - vec[i]).powi(2);
        }
        distance.sqrt()
    }

    pub fn add_connection(
        &mut self,
        layer: i32,
        neighbor: Neighbor,
        max_neighbors: usize,
    ) -> Result<(), &'static str> {
        match self.connections.get_mut(&layer) {
            Some(neighbors) => {
                if neighbors.len() < max_neighbors {
                    neighbors.push(neighbor);
                    Ok(())
                } else {
                    Err("Maximum number of neighbors reached for this node at this layer.")
                }
            }
            None => {
                let mut new_neighbors = Vec::with_capacity(max_neighbors);
                new_neighbors.push(neighbor);
                self.connections.insert(layer, new_neighbors);
                Ok(())
            }
        }
    }

    pub fn remove_connection(
        &mut self,
        layer: i32,
        neighbor_cid: &str,
    ) -> Result<(), &'static str> {
        match self.connections.get_mut(&layer) {
            Some(neighbors) => {
                if let Some(index) = neighbors
                    .iter()
                    .position(|n| n.node.read().unwrap().cid == neighbor_cid)
                {
                    neighbors.remove(index);
                    Ok(())
                } else {
                    Err("Neighbor not found in the specified layer.")
                }
            }
            None => Err("Layer not found in the connections."),
        }
    }

    pub fn get_neighbors(&self, layer: i32) -> Option<&Vec<Neighbor>> {
        self.connections.get(&layer)
    }

    pub fn get_cid(&self) -> &str {
        &self.cid
    }

    pub fn get_layer(&self) -> i32 {
        self.layer
    }

    pub fn get_vector(&self) -> &[f32] {
        &self.vector
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cid.eq(&other.cid)
    }
}

#[derive(Clone)]
pub struct ComparableNode {
    pub node: Arc<RwLock<Node>>,
    pub distance: f32,
}

impl ComparableNode {
    pub fn new(node: Arc<RwLock<Node>>, query_vector: &[f32]) -> Self {
        let distance = node.read().unwrap().distance(query_vector);
        ComparableNode { node, distance }
    }
}

impl Eq for ComparableNode {}

impl PartialEq for ComparableNode {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl PartialOrd for ComparableNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}

impl Ord for ComparableNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_node(cid: &str, vector: Vec<f32>, layer: i32) -> Node {
        Node {
            cid: String::from(cid),
            vector,
            layer,
            connections: HashMap::new(),
        }
    }

    #[test]
    fn test_add_connection() {
        let mut node1 = create_test_node("node1", vec![1.0, 1.0], 0);
        let node2 = create_test_node("node2", vec![2.0, 2.0], 0);
        let distance = node1.distance(&node2.vector);
        let neighbor = Neighbor::new(Arc::new(RwLock::new(node2.clone())), distance);

        node1.add_connection(0, neighbor, 1).unwrap();

        assert_eq!(node1.connections.len(), 1);
        assert_eq!(node1.connections.get(&0).unwrap().len(), 1);
        assert_eq!(node1.connections.get(&0).unwrap()[0].node.read().unwrap().cid, "node2");
        assert_eq!(node1.connections.get(&0).unwrap()[0].distance, distance);
    }

    #[test]
    fn test_remove_connection() {
        let mut node1 = create_test_node("node1", vec![1.0, 1.0], 0);
        let node2 = create_test_node("node2", vec![2.0, 2.0], 0);

        let distance = node1.distance(&node2.vector);
        let neighbor = Neighbor::new(Arc::new(RwLock::new(node2.clone())), distance);

        node1.add_connection(0, neighbor, 1).unwrap();

        node1.remove_connection(0, node2.get_cid()).unwrap();

        assert_eq!(node1.connections.len(), 1);
        assert_eq!(node1.connections.get(&0).unwrap().len(), 0);
    }
}

