// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use semsimian_server::{compare_termsets, say_hello};

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hello, compare_termsets])
}
