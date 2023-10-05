// This is a simple web server that uses the semsimian crate for semantic similarity operations.

#[macro_use]
extern crate rocket;

use lazy_static::lazy_static;
use semsimian::RustSemsimian;
use semsimian_server::{compare_termsets, get_rss_instance, say_hello};
use std::collections::HashSet;

// Get a RustSemsimian instance
lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
}

#[launch]
pub fn rocket() -> _ {
    // run a first compare to warm up the RustSemsimian instance
    RSS.termset_pairwise_similarity(
        &HashSet::from(["MP:0010771".to_string()]),
        &HashSet::from(["HP:0004325".to_string()]),
        &None,
    );
    rocket::build().mount("/", routes![say_hello, compare_termsets])
}
