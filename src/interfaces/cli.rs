use std::sync::Arc;

use clap::{App, Arg, SubCommand};
use prettytable::{format, row, Table};
use tokio::runtime::Runtime;

use crate::hnsw_graph::hnsw::Neighbour;
use crate::interfaces::api::VectorAPI;

pub struct CLIInterface {
    api: Arc<VectorAPI>,
}

impl CLIInterface {
    pub fn new(api: Arc<VectorAPI>) -> Self {
        Self { api }
    }

    pub fn run(&self) {
        let rt = Runtime::new().unwrap();

        loop {
            let app = App::new("VectorAPI CLI Interface")
                .version("1.0")
                .author("Your Name <your.email@example.com>")
                .about("Vector search using a CLI interface")
                .subcommand(
                    SubCommand::with_name("insert")
                        .about("Insert data points")
                        .arg(
                            Arg::with_name("data")
                                .short('d')
                                .long("data")
                                .value_name("DATA")
                                .help("Data points to insert (comma-separated list of floats)")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("cid")
                                .short('i')
                                .long("cid")
                                .value_name("CID")
                                .help("CID of document to insert")
                                .takes_value(true),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("search")
                        .about("Search for nearest neighbors")
                        .arg(
                            Arg::with_name("query")
                                .short('q')
                                .long("query")
                                .value_name("QUERY")
                                .help("Query point (comma-separated list of floats)")
                                .takes_value(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("knbn")
                                .short('k')
                                .long("knbn")
                                .value_name("K")
                                .help("Number of nearest neighbors to search")
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("ef")
                                .short('e')
                                .long("ef")
                                .value_name("EF")
                                .help("Size of the dynamic search list")
                                .takes_value(true),
                        ),
                )
                .subcommand(SubCommand::with_name("exit").about("Exit the CLI"));

            let matches = app.get_matches();

            if let Some(insert_matches) = matches.subcommand_matches("insert") {
                let data_str = insert_matches.value_of("data").unwrap();
                let data: Vec<f32> = data_str
                    .split(',')
                    .map(|s| s.parse::<f32>().unwrap())
                    .collect();

                //TODO change the id type to String
                let cid = insert_matches
                    .value_of("cid")
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();

                let insert_task = async move {
                    self.api.parallel_insert(&vec![(&data, cid)]);
                };

                rt.block_on(insert_task);
            } else if let Some(search_matches) = matches.subcommand_matches("search") {
                let query_str = search_matches.value_of("query").unwrap();
                let query: Vec<f32> = query_str
                    .split(',')
                    .map(|s| s.parse::<f32>().unwrap())
                    .collect();

                let knbn = search_matches
                    .value_of("knbn")
                    .unwrap_or("10")
                    .parse::<usize>()
                    .unwrap();
                let ef = search_matches
                    .value_of("ef")
                    .unwrap_or("50")
                    .parse::<usize>()
                    .unwrap();

                let search_task = async move {
                    //self.api.parallel_search(&query, knbn, ef)
                    self.api.parallel_search(&vec![query], knbn, ef)
                };

                let search_results: Vec<Vec<Neighbour>> = rt.block_on(search_task);
                let flattened_search_results: Vec<Neighbour> = search_results
                    .into_iter()
                    .flat_map(|v| v.into_iter())
                    .collect();

                let mut table = Table::new();
                table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
                table.set_titles(row!["Index", "Distance"]);

                for Neighbour {
                    d_id,
                    distance,
                    p_id,
                } in flattened_search_results
                {
                    table.add_row(row![d_id, distance]);
                }

                table.printstd();
            } else if let Some(_) = matches.subcommand_matches("exit") {
                println!("Exiting the CLI.");
                break;
            } else {
                println!("Please provide either 'insert', 'search', or 'exit' subcommand.");
            }
        }
    }
}
