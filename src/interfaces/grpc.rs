use std::net::SocketAddr;
use std::sync::Arc;

use tonic::{transport::Server, Request, Response, Status};

use vector_service::{
    vector_service_server::{VectorService, VectorServiceServer},
    InsertRequest, Neighbour as PbNeighbour, Neighbours, PointId, SearchRequest, SearchResult,
};

use crate::hnsw_graph::hnsw::Neighbour;
use crate::interfaces::api::VectorAPI;

// Import the generated Rust code
pub mod vector_service {
    tonic::include_proto!("vector_service");
}

pub struct GRPCServer {
    api: Arc<VectorAPI>,
}

impl GRPCServer {
    pub fn new(api: Arc<VectorAPI>) -> Self {
        GRPCServer { api }
    }
}

#[tonic::async_trait]
impl VectorService for GRPCServer {
    async fn insert(&self, request: Request<InsertRequest>) -> Result<Response<()>, Status> {
        let request_data = request.into_inner();
        let data: Vec<(Vec<f32>, usize)> = request_data
            .data
            .into_iter()
            .map(|float_array| float_array.values)
            .zip(request_data.ids)
            .map(|(data, id)| (data, id as usize))
            .collect();

        self.api.parallel_insert(
            &data
                .iter()
                .map(|(vec, idx)| (vec as &Vec<f32>, *idx))
                .collect::<Vec<_>>(),
        );

        Ok(Response::new(()))
    }

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<SearchResult>, Status> {
        let request_data = request.into_inner();
        let data: Vec<Vec<f32>> = request_data
            .data
            .into_iter()
            .map(|float_array| float_array.values)
            .collect();

        let results: Vec<Vec<Neighbour>> =
            self.api
                .parallel_search(&data, request_data.knbn as usize, request_data.ef as usize);

        let neighbours_message: Vec<Neighbours> = results
            .into_iter()
            .map(|neighbours| {
                let neighbour_message: Vec<PbNeighbour> = neighbours
                    .into_iter()
                    .map(|neighbour| PbNeighbour {
                        d_id: neighbour.d_id as u32,
                        distance: neighbour.distance,
                        point_id: Some(PointId {
                            layer: neighbour.p_id.0 as u32,
                            index: neighbour.p_id.1,
                        }),
                    })
                    .collect();
                Neighbours {
                    neighbour: neighbour_message,
                }
            })
            .collect();

        Ok(Response::new(SearchResult {
            neighbours: neighbours_message,
        }))
    }
}

pub async fn start_grpc(
    api: Arc<VectorAPI>,
    address: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let vector_service = GRPCServer::new(api);
    let svc = VectorServiceServer::new(vector_service);
    Server::builder().add_service(svc).serve(address).await?;

    Ok(())
}
