// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use semsimian_server::{compare_termsets, search, say_hello};

#[launch]
pub fn rocket() -> _ {
    compare_termsets("HP:0000001,HP:0000002", "HP:0000003,HP:0000004", "");
    search("HP:0000001,HP:0000002", "ZFIN", "", Some(1));
    rocket::build().mount("/", routes![
        say_hello,
        compare_termsets,
        search
    ])
}
