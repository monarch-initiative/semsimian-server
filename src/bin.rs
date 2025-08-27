// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use clap::Parser;

use semsimian_server::{ compare_termsets, say_hello, search, utils::check_for_phenio };

#[derive(Parser)]
#[command(version, about)]
struct Cli {}

#[launch]
pub fn rocket() -> _ {
    //  Initialize the CLI parser
    let _cli = Cli::parse();

    // Check for phenio.db, download if missing
    check_for_phenio();

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
    rocket::build().mount("/", routes![say_hello, compare_termsets, search])
}
