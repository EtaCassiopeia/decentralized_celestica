use clap::{App, Arg, SubCommand};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use tonic::transport::Channel;
use tonic::Response;

use colored::*;

use vector_service::{
    vector_service_client::VectorServiceClient, FloatArray, InsertRequest, Neighbours,
    SearchRequest,
};

use crate::interfaces::cli_grpc::vector_service::SearchResult;

pub mod vector_service {
    tonic::include_proto!("vector_service");
}

pub struct GrpcCli {
    client: VectorServiceClient<Channel>,
}

impl GrpcCli {
    pub async fn new(address: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = tonic::transport::Endpoint::from_shared(address.to_string())?
            .connect()
            .await?;

        let client = VectorServiceClient::new(channel);

        Ok(Self { client })
    }

    pub async fn insert(
        &mut self,
        key: usize,
        vector: Vec<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let float_array = FloatArray { values: vector };
        let request = tonic::Request::new(InsertRequest {
            ids: vec![key as u32],
            data: vec![float_array],
        });

        let _response = self.client.insert(request).await?;

        Ok(())
    }

    pub async fn search(
        &mut self,
        query: Vec<f32>,
        knbn: usize,
        ef: usize,
    ) -> Result<Vec<Neighbours>, Box<dyn std::error::Error>> {
        let float_array = FloatArray { values: query };
        let request = tonic::Request::new(SearchRequest {
            data: vec![float_array],
            knbn: knbn as u32,
            ef: ef as u32,
        });

        let response: Response<SearchResult> = self.client.search(request).await?;
        let neighbours = response.into_inner().neighbours;

        Ok(neighbours)
    }

    pub async fn start(&mut self) {
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());

                    let matches = App::new("GrpcCli")
                        .version("1.0")
                        .author("Mohsen Zainalpour <zainalpour@celestica.dev>")
                        .about("gRPC client for Celestica")
                        .subcommand(
                            SubCommand::with_name("insert")
                                .about("Insert a vector")
                                .arg(
                                    Arg::with_name("key")
                                        .short('k')
                                        .long("key")
                                        .takes_value(true)
                                        .required(true),
                                )
                                .arg(
                                    Arg::with_name("vector")
                                        .short('v')
                                        .long("vector")
                                        .takes_value(true)
                                        .required(true),
                                ),
                        )
                        .subcommand(
                            SubCommand::with_name("search")
                                .about("Search for neighbours")
                                .arg(
                                    Arg::with_name("vector")
                                        .short('v')
                                        .long("vector")
                                        .takes_value(true)
                                        .required(true),
                                )
                                .arg(
                                    Arg::with_name("k")
                                        .short('k')
                                        .long("knbn")
                                        .takes_value(true)
                                        .required(true),
                                )
                                .arg(
                                    Arg::with_name("ef")
                                        .short('e')
                                        .long("ef")
                                        .takes_value(true)
                                        .required(true),
                                ),
                        )
                        .subcommand(SubCommand::with_name("exit").about("Exit the application"))
                        .setting(clap::AppSettings::NoBinaryName)
                        .try_get_matches_from(line.split_whitespace());

                    match matches {
                        Ok(matches) => {
                            if let Some(matches) = matches.subcommand_matches("insert") {
                                let key =
                                    matches.value_of("key").unwrap().parse::<usize>().unwrap();
                                let vector_str = matches.value_of("vector").unwrap();
                                let vector: Vec<f32> = vector_str
                                    .split(',')
                                    .map(|s| s.parse::<f32>().unwrap())
                                    .collect();

                                match self.insert(key, vector).await {
                                    Ok(_) => {
                                        println!("{}", "Vector inserted successfully.".green())
                                    }
                                    Err(err) => println!("Error inserting vector: {:?}", err),
                                }
                            } else if let Some(matches) = matches.subcommand_matches("search") {
                                let vector_str = matches.value_of("vector").unwrap();
                                let vector: Vec<f32> = vector_str
                                    .split(',')
                                    .map(|s| s.parse::<f32>().unwrap())
                                    .collect();
                                let k = matches.value_of("k").unwrap().parse::<usize>().unwrap();
                                let ef = matches.value_of("ef").unwrap().parse::<usize>().unwrap();

                                match self.search(vector, k, ef).await {
                                    Ok(neighbours) => {
                                        println!("{}", "Neighbours found:".green());
                                        for neighbour in
                                            neighbours.into_iter().flat_map(|n| n.neighbour)
                                        {
                                            println!(
                                                "ID: {}, Distance: {}",
                                                format!("{}", neighbour.d_id).blue(),
                                                format!("{:.2}", neighbour.distance).blue()
                                            );
                                        }
                                    }
                                    Err(err) => {
                                        println!("Error searching for neighbours: {:?}", err)
                                    }
                                }
                            } else if matches.subcommand_matches("exit").is_some() {
                                println!("{}", "Exiting...".red());
                                break;
                            } else {
                                println!("Invalid command. Please enter a valid command.");
                            }
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err);
                            eprintln!("Type 'help' for more information.");
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        rl.save_history("history.txt").unwrap();
    }
}
