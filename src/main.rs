use std::net::AddrParseError;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::thread;
#[cfg(test)]
use std::{println as info, println as warn, println as debug, println as trace};

use clap::{App, Arg};
use log::{info, warn};
use tokio::select;
use tokio::signal;
use tokio::try_join;

use d_celestica::hnsw_graph::dist;
use d_celestica::hnsw_graph::hnsw::Hnsw;
use d_celestica::interfaces::api::VectorAPI;
use d_celestica::interfaces::cli_grpc::GrpcCli;
use d_celestica::interfaces::grpc::*;
use d_celestica::interfaces::rest::*;

#[actix_rt::main]
async fn main() {
    info!("Starting Celestica");
    // async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use clap::{App, Arg};

    env_logger::init();

    let matches = App::new("Celestica")
        .version("1.0")
        .author("Mohsen Zainalpour <zainalpour@celestica.dev>")
        .about("A decentralized vector database")
        .arg(
            Arg::with_name("grpc_cli")
                .long("grpc_cli")
                .value_name("GRPC_CLI")
                .help("Start GrpcCli")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("rest_port")
                .long("rest_port")
                .value_name("REST_PORT")
                .help("Port number for the REST interface")
                .env("REST_PORT")
                .takes_value(true)
                .default_value("8080"),
        )
        .arg(
            Arg::with_name("grpc_port")
                .long("grpc_port")
                .value_name("GRPC_PORT")
                .help("Port number for the GRPC interface")
                .takes_value(true)
                .env("GRPC_PORT")
                .default_value("50051"),
        )
        .arg(
            Arg::with_name("grpc_server_host")
                .long("grpc_server_host")
                .value_name("GRPC_SERVER_HOST")
                .help("Host name for the GRPC server")
                .takes_value(true)
                .env("GRPC_SERVER_HOST")
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::with_name("max_nb_connection")
                .long("max_nb_connection")
                .value_name("MAX_NB_CONNECTION")
                .help("Maximum number of connections per element")
                .takes_value(true)
                .env("MAX_NB_CONNECTION")
                .default_value("16"),
        )
        .arg(
            Arg::with_name("max_elements")
                .long("max_elements")
                .value_name("MAX_ELEMENTS")
                .help("Maximum number of elements to store")
                .takes_value(true)
                .env("MAX_ELEMENTS")
                .default_value("10000"),
        )
        .arg(
            Arg::with_name("max_layer")
                .long("max_layer")
                .value_name("MAX_LAYER")
                .help("Maximum number of layers")
                .takes_value(true)
                .env("MAX_LAYER")
                .default_value("16"),
        )
        .arg(
            Arg::with_name("ef_construction")
                .long("ef_construction")
                .value_name("EF_CONSTRUCTION")
                .help("Size of the dynamic candidate list during construction")
                .takes_value(true)
                .env("EF_CONSTRUCTION")
                .default_value("200"),
        )
        .get_matches();

    let grpc_port = matches
        .value_of("grpc_port")
        .unwrap()
        .parse::<u16>()
        .unwrap();

    if matches.is_present("grpc_cli") {
        // Start the gRPC client
        info!("Starting gRPC client");

        let grpc_server_host = matches.value_of("grpc_server_host").unwrap();

        let address = format!("http://{}:{}", grpc_server_host, grpc_port);
        let mut grpc_cli = GrpcCli::new(&address).await.unwrap();

        grpc_cli.start().await;
    } else {
        info!("Starting server with REST and gRPC interfaces");

        let host = "0.0.0.0";
        let rest_port = matches
            .value_of("rest_port")
            .unwrap()
            .parse::<u16>()
            .unwrap();

        let max_nb_connection = matches
            .value_of("max_nb_connection")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let max_elements = matches
            .value_of("max_elements")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let max_layer = matches
            .value_of("max_layer")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let ef_construction = matches
            .value_of("ef_construction")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        //TODO get Distance from command line
        // Create an instance of Hnsw with your desired parameters
        let hnsw = Hnsw::new(
            max_nb_connection,
            max_elements,
            max_layer,
            ef_construction,
            dist::DistCosine,
        );

        // Initialize the unified VectorAPI with the Hnsw instance
        let vector_api = Arc::new(VectorAPI::new(hnsw));

        let rest_addr = create_socket_addr(host, rest_port).unwrap();
        let grpc_addr = create_socket_addr(host, grpc_port).unwrap();

        let rest_api = Arc::clone(&vector_api);
        let grpc_api = Arc::clone(&vector_api);

        info!("Starting REST API on {}", rest_addr);
        let rest_server = actix_web::rt::spawn(async move {
            start_rest_api(rest_api, rest_addr).await.unwrap();
        });

        info!("Starting gRPC server on {}", grpc_addr);
        let grpc_server = actix_web::rt::spawn(async move {
            start_grpc(grpc_api, grpc_addr).await.unwrap();
        });

        let ctrl_c = signal::ctrl_c();

        select! {
            _ = rest_server => {
                warn!("REST API server stopped");
            },
            _ = grpc_server => {
                warn!("gRPC server stopped");
            },
            _ = ctrl_c => {
                warn!("Ctrl+C received, shutting down...");
            }
        }
    }

    fn create_socket_addr(host: &str, port: u16) -> Result<SocketAddr, AddrParseError> {
        let address_str = format!("{}:{}", host, port);
        SocketAddr::from_str(&address_str)
    }
}
