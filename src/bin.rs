// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]

use axum::{ routing::get, Router };
use clap::Parser;

use semsimian_server::{ compare_termsets, say_hello, search };

#[derive(Parser)]
#[command(version, about)]
struct Cli {}

#[tokio::main]
pub fn main() -> _ {
    //  Initialize the CLI parser
    let _cli = Cli::parse();

    // Run a compare and search to initialize Closure and IC maps
    compare_termsets(
        "HP:0000001,HP:0000002",
        "HP:0000003,HP:0000004",
        Some(std::path::PathBuf::from("ancestor_information_content"))
    );
    search(
        "HP:0000001,HP:0000002",
        "ZFIN",
        Some(std::path::PathBuf::from("ancestor_information_content")),
        Some(1),
        Some("bidirectional")
    );

    // Start the Rocket web server
    let app = Router::new()
        .route("/", get(say_hello ))
        .route("compare", get(compare_termsets )

    rocket::build().mount("/", routes![say_hello, compare_termsets, search])
}
