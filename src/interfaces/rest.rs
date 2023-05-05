use std::net::SocketAddr;
use std::sync::Arc;

use actix_web::rt as actix_rt;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tokio_compat_02::FutureExt;

use crate::hnsw_graph::hnsw::Neighbour;
use crate::interfaces::api::VectorAPI;

// Define request and response types
#[derive(Serialize, Deserialize)]
pub struct InsertRequest {
    pub data: Vec<(Vec<f32>, usize)>,
}

#[derive(Serialize, Deserialize)]
pub struct SearchRequest {
    pub data: Vec<Vec<f32>>,
    pub knbn: usize,
    pub ef: usize,
}

#[derive(Serialize, Deserialize)]
pub struct SearchResult {
    pub results: Vec<Vec<Neighbour>>,
}

// Request handlers
async fn handle_insert(
    api: web::Data<Arc<VectorAPI>>,
    req: web::Json<InsertRequest>,
) -> impl Responder {
    //TODO double check this
    api.parallel_insert(
        &req.data
            .iter()
            .map(|(data, idx)| (data as &Vec<f32>, *idx))
            .collect::<Vec<_>>(),
    );
    HttpResponse::Ok().json("Insert successful")
}

async fn handle_search(
    api: web::Data<Arc<VectorAPI>>,
    req: web::Json<SearchRequest>,
) -> impl Responder {
    let results = api.parallel_search(&req.data, req.knbn, req.ef);
    HttpResponse::Ok().json(SearchResult { results })
}

pub async fn start_rest_api(api: Arc<VectorAPI>, address: SocketAddr) -> std::io::Result<()> {
    let api = web::Data::new(api);
    HttpServer::new(move || {
        App::new()
            .app_data(api.clone())
            .route("/insert", web::post().to(handle_insert))
            .route("/search", web::post().to(handle_search))
    })
    .bind(address)?
    .run()
    .await
}

// pub async fn start_rest_api(api: Arc<VectorAPI>, address: SocketAddr) -> std::io::Result<()> {
//     let sys = actix_rt::System::new();
//     sys.block_on(async {
//         HttpServer::new(move || {
//             App::new()
//                 .app_data(web::Data::new(api.clone()))
//                 .route("/insert", web::post().to(handle_insert))
//                 .route("/search", web::post().to(handle_search))
//         })
//         .bind(address)?
//         .run()
//         .compat() // Use the compat extension trait to convert the Actix-web future to a Tokio future
//         .await
//     })
// }
