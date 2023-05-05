use crate::hnsw_graph::dist;
use crate::hnsw_graph::hnsw::{Hnsw, Neighbour};

pub struct VectorAPI {
    hnsw: Hnsw<f32, dist::DistCosine>,
}

impl VectorAPI {
    pub fn new(hnsw: Hnsw<f32, dist::DistCosine>) -> Self {
        VectorAPI { hnsw }
    }

    pub fn parallel_insert(&self, data: &Vec<(&Vec<f32>, usize)>) {
        self.hnsw.parallel_insert(data);
    }

    pub fn parallel_search(
        &self,
        data: &Vec<Vec<f32>>,
        knbn: usize,
        ef: usize,
    ) -> Vec<Vec<Neighbour>> {
        self.hnsw.parallel_search(data, knbn, ef)
    }
}
