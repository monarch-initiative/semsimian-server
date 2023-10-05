// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use semsimian::RustSemsimian;
use semsimian_server::{compare_termsets, say_hello};

#[launch]
pub fn rocket() -> _ {
    compare_termsets("MP:0010771".to_string(), "HP:0004325".to_string());
    rocket::build().mount("/", routes![say_hello, compare_termsets])
}
