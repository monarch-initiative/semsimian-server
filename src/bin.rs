// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use semsimian_server::{ compare_termsets, say_hello, search };

#[launch]
pub fn rocket() -> _ {
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
    rocket::build().mount("/", routes![say_hello, compare_termsets, search])
}
