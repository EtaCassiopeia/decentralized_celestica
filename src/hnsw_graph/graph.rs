use std::collections::BTreeSet;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;
use std::sync::RwLock;

use rand::Rng;

use crate::hnsw_graph::neighbor::Neighbor;
use crate::hnsw_graph::node::{ComparableNode, Node};

pub struct HNSWGraph {
    max_neighbors: usize,
    max_layer: i32,
    entry_points: HashMap<i32, Arc<RwLock<Node>>>,
    nodes: HashMap<String, Arc<RwLock<Node>>>,
}

impl HNSWGraph {
    pub fn new(max_neighbors: usize, max_layer: i32) -> Self {
        HNSWGraph {
            max_neighbors,
            max_layer,
            entry_points: HashMap::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, cid: String, vector: Vec<f32>) -> Result<(), Box<dyn Error>> {
        if self.nodes.contains_key(&cid) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Node with the given CID already exists",
            )));
        }

        // Determine the layer for the new node
        let new_node_layer = self.random_layer();

        let new_node = Arc::new(RwLock::new(Node::new(cid.clone(), vector, new_node_layer)));

        if new_node_layer > self.max_layer {
            self.entry_points.insert(new_node_layer, new_node.clone());
            self.max_layer = new_node_layer;
        }

        self.connect_new_node(new_node.clone());
        self.nodes.insert(cid, new_node);

        Ok(())
    }

    pub fn layer_growth_probability(&self) -> f64 {
        let p: f64 = 1.0 / self.max_neighbors as f64;
        p
    }

    pub fn random_layer(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let uniform: f64 = rng.gen_range(0.0..1.0);
        let layer = (uniform.ln() / self.layer_growth_probability().ln()).ceil() as i32;

        // Clip the layer number to the range [0, max_layer]
        layer.clamp(0, self.max_layer)
    }

    fn connect_new_node(&mut self, new_node: Arc<RwLock<Node>>) {
        for layer in 0..=new_node.read().unwrap().layer {
            let entry_point = self.entry_points[&layer].clone();
            let new_node_vector = &new_node.read().unwrap().vector;
            //TODO handle Result
            let neighbors = self
                .search_layer_neighbors(new_node_vector, entry_point, layer, usize::MAX)
                .unwrap();

            for neighbor in neighbors {
                self.add_edge(new_node.clone(), neighbor.node.clone(), layer);
                self.add_edge(neighbor.node.clone(), new_node.clone(), layer);
            }
        }
    }

    //TODO move it to a separate module (space or metrics)
    fn euclidean_distance(vec1: &[f32], vec2: &[f32]) -> f32 {
        vec1.iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    fn add_edge(&mut self, node1: Arc<RwLock<Node>>, node2: Arc<RwLock<Node>>, layer: i32) {
        let distance = {
            let node1_read = node1.read().unwrap();
            let node2_read = node2.read().unwrap();
            Self::euclidean_distance(&node1_read.vector, &node2_read.vector)
        };

        {
            let mut node1_write = node1.write().unwrap();
            //TODO handle Result
            let _ = node1_write.add_connection(
                layer,
                Neighbor::new(node2.clone(), distance),
                self.max_neighbors,
            );
        }

        {
            let mut node2_write = node2.write().unwrap();
            //TODO handle Result
            let _ = node2_write.add_connection(
                layer,
                Neighbor::new(node1.clone(), distance),
                self.max_neighbors,
            );
        }
    }

    fn search_layer_neighbors(
        &self,
        query: &[f32],
        entry_point: Arc<RwLock<Node>>,
        layer: i32,
        ef: usize,
    ) -> Result<Vec<Neighbor>, &'static str> {
        let mut visited = HashSet::new();
        let mut candidates = BTreeSet::new();
        let mut result: BTreeSet<ComparableNode> = BTreeSet::new();

        candidates.insert(ComparableNode {
            node: entry_point.clone(),
            distance: entry_point.read().unwrap().distance(query),
        });
        visited.insert(entry_point.read().unwrap().cid.clone());

        while let Some(comparable_node) = candidates.pop_first() {
            //TODO handle Result
            let node = self
                .get_node_by_cid(&comparable_node.node.read().unwrap().cid)
                .unwrap();

            // Terminate the search if the closest candidate is further than the farthest result
            if !result.is_empty() && comparable_node.distance > result.last().unwrap().distance {
                break;
            }

            // Add the current node to the result set
            result.insert(comparable_node.clone());

            // Remove the farthest element from the result set if it exceeds ef
            if result.len() > ef {
                result.pop_last();
            }

            // Add unvisited neighbors to the candidates set
            //TODO handle Result
            let neighbors_read = node.read().unwrap();
            let neighbors = neighbors_read.get_neighbors(layer).unwrap();
            for neighbor in neighbors {
                if !visited.contains(&neighbor.node.read().unwrap().cid) {
                    visited.insert(neighbor.node.read().unwrap().cid.clone());
                    candidates.insert(ComparableNode {
                        node: neighbor.node.clone(),
                        distance: neighbor.distance,
                    });
                }
            }
        }

        // Convert the result set into a Vec<Neighbor>
        let mut neighbors = Vec::new();
        for comparable_node in result.into_iter() {
            let node = self.get_node_by_cid(&comparable_node.node.read().unwrap().cid)?;
            neighbors.push(Neighbor {
                node: node.clone(),
                distance: comparable_node.distance,
            });
        }

        Ok(neighbors)
    }

    fn get_node_by_cid(&self, cid: &str) -> Result<Arc<RwLock<Node>>, &'static str> {
        self.nodes
            .get(cid)
            .map(|node| node.clone())
            .ok_or("Node not found")
    }

    pub fn remove_node(&mut self, node_cid: &str) -> Result<(), &'static str> {
        // Find the node by its CID
        let node = self.get_node_by_cid(node_cid)?;

        // Remove the node from its connected neighbors
        {
            let node_write = node.write().unwrap();
            for (layer, connections) in &node_write.connections {
                for neighbor in connections {
                    let mut neighbor_node_write = neighbor.node.write().unwrap();
                    neighbor_node_write.remove_connection(*layer, node_cid)?;
                }
            }
        }

        // Remove the node from the graph
        self.nodes.remove(node_cid);

        // Remove the node from entry_points if it exists in any layer
        for layer in self.entry_points.clone().keys() {
            if let Some(entry_point) = self.entry_points.get(layer) {
                if Arc::ptr_eq(entry_point, &node) {
                    self.entry_points.remove(layer);
                }
            }
        }

        Ok(())
    }

    fn find_entry_point_in_layer(
        &self,
        query_vector: &[f32],
        layer: i32,
        ef: usize,
    ) -> Result<Arc<RwLock<Node>>, &'static str> {
        // Start with the current entry point in the specified layer
        let mut entry_point = match self.entry_points.get(&layer) {
            Some(entry_point) => entry_point.clone(),
            None => return Err("Entry point not found for the given layer"),
        };

        // Search for the closest neighbor in the given layer
        let neighbors =
            self.search_layer_neighbors(query_vector, entry_point.clone(), layer, ef)?;

        // Check if there is any closer neighbor
        for neighbor in neighbors {
            //if neighbor.distance < distance(&query_vector, &entry_point.read().unwrap().vector) {
            if neighbor.distance < entry_point.read().unwrap().distance(&query_vector) {
                entry_point = neighbor.node.clone();
            }
        }

        Ok(entry_point.clone())
    }

    pub fn search(
        &self,
        query: &[f32],
        k: usize,
        ef: usize,
    ) -> Result<Vec<(String, f32)>, &'static str> {
        if k > ef {
            return Err("k cannot be larger than ef");
        }

        // Start at the top layer and find the entry point in that layer
        let mut entry_point = self.find_entry_point_in_layer(query, 0, ef)?;

        // Search for nearest neighbors in the layers below
        let mut result_set = BTreeSet::new();
        for layer in (0..=self.max_layer).rev() {
            let neighbors = self.search_layer_neighbors(query, entry_point.clone(), layer, ef)?;

            // Insert found neighbors into the result set
            for neighbor in neighbors {
                result_set.insert(ComparableNode {
                    node: neighbor.node.clone(),
                    distance: neighbor.distance,
                });
            }

            // Update the entry point for the next layer
            if layer > 0 {
                entry_point = self.find_entry_point_in_layer(query, layer - 1, ef)?;
            }
        }

        // Return the k nearest neighbors as a Vec<(String, f32)>
        let mut result = Vec::new();
        for (i, comparable_node) in result_set.into_iter().enumerate() {
            if i >= k {
                break;
            }
            result.push((
                comparable_node.node.read().unwrap().cid.clone(),
                comparable_node.distance,
            ));
        }

        Ok(result)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod graph_tests {
    use super::*;

    fn create_test_node(cid: &str, vector: Vec<f32>, layer: i32) -> Node {
        Node {
            cid: cid.to_string(),
            vector,
            layer,
            connections: HashMap::new(),
        }
    }


    #[test]
    fn test_add_node() {
        let mut graph = HNSWGraph::new(16,3);
        let node = create_test_node("node1", vec![1.0, 1.0], 0);
        graph.add_node(node.cid.clone(), node.vector.clone()).unwrap();

        assert_eq!(graph.node_count(), 1);
        assert_eq!(graph.get_node_by_cid(&node.cid).unwrap().read().unwrap().cid, node.cid);
        assert_eq!(graph.get_node_by_cid(&node.cid).unwrap().read().unwrap().vector, node.vector);
        assert_eq!(graph.get_node_by_cid(&node.cid).unwrap().read().unwrap().layer, node.layer);
    }

    #[test]
    fn test_remove_node() {
        let mut graph = HNSWGraph::new(16,3);
        let node = create_test_node("node1", vec![1.0, 1.0], 0);
        graph.add_node(node.cid.clone(), node.vector.clone()).unwrap();
        graph.remove_node(&node.cid);

        assert_eq!(graph.node_count(), 0);
        assert!(graph.get_node_by_cid(&node.cid).is_ok());
    }

    #[test]
    fn test_search() {
        let mut graph = HNSWGraph::new(16,3);
        let node1 = create_test_node("node1", vec![1.0, 1.0], 0);
        let node2 = create_test_node("node2", vec![2.0, 2.0], 0);
        let node3 = create_test_node("node3", vec![3.0, 3.0], 0);

        graph.add_node(node1.cid.clone(), node1.vector.clone()).unwrap();
        graph.add_node(node2.cid.clone(), node2.vector.clone()).unwrap();
        graph.add_node(node3.cid.clone(), node3.vector.clone()).unwrap();

        let query_vector = vec![2.5, 2.5];
        let k = 1;
        let ef = 10;

        let result: Vec<(String, f32)> = graph.search(&query_vector, k, ef).unwrap();
        let similar_cids: Vec<String> = result.into_iter().map(|(cid, _)| cid).collect();

        assert_eq!(similar_cids.len(), k);
        assert_eq!(similar_cids[0], node3.cid);
    }
}
