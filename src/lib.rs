#[macro_use]
extern crate rocket;

use std::collections::HashSet;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use semsimian::enums::{DirectionalityEnum, SearchTypeEnum};
use semsimian::termset_pairwise_similarity::{
    TermsetPairwiseSimilarity as Tsps, TermsetPairwiseSimilarity,
};
use semsimian::{RustSemsimian, TermID};

use utils::get_rss_instance;

use crate::utils::DirectionalityEnumWrapper;
use crate::utils::MetricEnumWrapper;

pub mod utils;

lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
    static ref RSS_MUTEX: Mutex<RustSemsimian> = Mutex::new(get_rss_instance());
}

//--- ROUTES ---//
#[get("/")]
pub fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

#[get("/compare/<termset1>/<termset2>/<metric>")]
pub fn compare_termsets(termset1: &str, termset2: &str, metric: Option<&str>) -> Json<Tsps> {
    // split termset1 and termset2 into vectors of TermIDs
    let mut terms1: HashSet<TermID> = HashSet::new();
    for term in termset1.split(',') {
        terms1.insert(term.to_string());
    }
    let mut terms2: HashSet<TermID> = HashSet::new();
    for term in termset2.split(',') {
        terms2.insert(term.to_string());
    }
    info!(
        "\nComparing termsets:\
        \nTermset 1: {:?}\
        \nTermset 2: {:?}\
        \n",
        terms1, terms2
    );
    let result = RSS.termset_pairwise_similarity(
        &terms1,
        &terms2,
        &MetricEnumWrapper::from_param(metric.unwrap()).unwrap(),
    );
    Json(result)
}

#[get("/search/<termset>/<prefix>/<metric>?<limit>&<direction>")]
pub fn search(
    termset: &str,
    prefix: &str,
    metric: Option<&str>,
    limit: Option<usize>,
    direction: Option<&str>,
) -> Json<Vec<(f64, Option<TermsetPairwiseSimilarity>, TermID)>> {
    let assoc_predicate: HashSet<TermID> = HashSet::from(["biolink:has_phenotype".to_string()]);
    let subject_prefixes: Option<Vec<TermID>> = Some(vec![prefix.to_string()]);

    //populate object_terms HashSet by splitting termset string on commas
    let mut object_terms: HashSet<TermID> = HashSet::new();
    for term in termset.split(',') {
        object_terms.insert(term.to_string());
    }
    let search_type: SearchTypeEnum = SearchTypeEnum::Hybrid;
    let limit: usize = limit.unwrap_or(10);

    let direction_enum = direction
        .map(|d| DirectionalityEnumWrapper::from_param(d).unwrap().0)
        .unwrap_or(DirectionalityEnum::Bidirectional);

    // Call the function under test
    let result = RSS_MUTEX.lock().unwrap().associations_search(
        &assoc_predicate,
        &object_terms,
        true,
        &None,
        &subject_prefixes,
        &search_type,
        // convert metric string to MetricEnum using from_string respecting Option expected by the from_string function
        &MetricEnumWrapper::from_param(metric.unwrap()).unwrap(),
        Some(limit),
        &Some(direction_enum),
    );
    println!("Result - {:?}", result);

    // print each entry in the result vector
    for (score, tsps, subject) in result.iter() {
        println!(
            "Score: {:?}\nTsps: {:?}\nSubject: {:?}\n",
            score, tsps, subject
        );
    }

    Json(result)
}
