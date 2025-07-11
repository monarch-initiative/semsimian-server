// This is a simple web server that uses the semsimian crate for semantic similarity operations.

use axum::{ extract::Path, extract::Query, routing::get, Router };
use clap::Parser;

use semsimian_server::{
    say_hello,
    compare_termsets,
    search,
    CompareParams,
    SearchParams,
    QueryParams,
};

#[derive(Parser)]
#[command(version, about)]
struct Cli {}

#[tokio::main]
pub async fn main() -> () {
    //  Initialize the CLI parser
    let _cli = Cli::parse();

    // Run a compare and search to initialize Closure and IC maps
    let _ = compare_termsets(
        Path(CompareParams {
            termset1: "HP:0000001,HP:0000002".to_string(),
            termset2: "HP:0000003,HP:0000004".to_string(),
            metric: Some("ancestor_information_content".to_string()),
        })
    ).await;
    let _ = search(
        Path(SearchParams {
            termset: "HP:0000001,HP:0000002".to_string(),
            prefix: "ZFIN".to_string(),
            metric: Some("ancestor_information_content".to_string()),
        }),
        Query(QueryParams {
            limit: Some(10),
            direction: Some("bidirectional".to_string()),
        })
    ).await;

    let app = Router::new()
        .route("/", get(say_hello))
        .route("/compare/{termset1}/{termset2}/{metric}", get(compare_termsets))
        .route("/search/{termset}/{prefix}", get(search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await.unwrap();
    println!("Semsimian Server is live at: https://0.0.0.0:9999");
    axum::serve(listener, app).await.unwrap();
}
