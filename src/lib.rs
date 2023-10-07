#[macro_use]
extern crate rocket;

use std::{collections::HashSet};

use lazy_static::lazy_static;
use rocket::serde::json::Json;
use semsimian::{RustSemsimian, TermID};
use structs::Tsps;
use utils::get_rss_instance;

pub mod structs;
pub mod utils;

lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
}

//--- ROUTES ---//
#[get("/")]
pub fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

#[get("/compare/<termset1>/<termset2>")]
pub fn compare_termsets(termset1: String, termset2: String) -> Json<Tsps> {
    // split termset1 and termset2 into vectors of TermIDs
    let mut terms1: HashSet<TermID> = HashSet::new();
    for term in termset1.split(",") {
        terms1.insert(term.to_string());
    }
    let mut terms2: HashSet<TermID> = HashSet::new();
    for term in termset2.split(",") {
        terms2.insert(term.to_string());
    }
    info!(
        "Comparing termsets:\
        \n\tTermset 1: {:?}\
        \n\tTermset 2: {:?}\
        \n",
        terms1, terms2
    );
    let result = RSS.termset_pairwise_similarity(&terms1, &terms2);
    Json(Tsps(result))
}
