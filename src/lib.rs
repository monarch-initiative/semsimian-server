#[macro_use]
extern crate rocket;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use lazy_static::lazy_static;
use rocket::request::FromParam;
use rocket::serde::json::Json;
use semsimian::enums::{DirectionalityEnum, SearchTypeEnum};
use semsimian::termset_pairwise_similarity::{
    TermsetPairwiseSimilarity as Tsps, TermsetPairwiseSimilarity,
};
use semsimian::{RustSemsimian, TermID};

use crate::utils::{get_rss_instance, get_association_cache};
use crate::utils::DirectionalityEnumWrapper;
use crate::utils::MetricEnumWrapper;

pub mod utils;

lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
    static ref RSS_POOL: Vec<Mutex<RustSemsimian>> = {
        let pool_size = num_cpus::get();
        println!("Creating RSS pool with {} instances", pool_size);
        (0..pool_size)
            .map(|_| Mutex::new(get_rss_instance()))
            .collect()
    };
    static ref ASSOCIATION_CACHE: HashMap<String, HashSet<String>> = get_association_cache();
    static ref POOL_COUNTER: AtomicUsize = AtomicUsize::new(0);
}

fn get_rss_from_pool() -> &'static Mutex<RustSemsimian> {
    let index = POOL_COUNTER.fetch_add(1, Ordering::Relaxed) % RSS_POOL.len();
    &RSS_POOL[index]
}

//--- ROUTES ---//
#[get("/")]
pub fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

#[get("/compare/<termset1>/<termset2>/<metric..>")]
pub fn compare_termsets(termset1: &str, termset2: &str, metric: Option<PathBuf>) -> Json<Tsps> {
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
    let default_metric = PathBuf::from("ancestor_information_content");
    let metric_path = metric.unwrap_or(default_metric);
    let metric_str = metric_path.to_str().unwrap();
    let result = RSS.termset_pairwise_similarity(
        &terms1,
        &terms2,
        &MetricEnumWrapper::from_param(metric_str).unwrap(),
    );
    Json(result)
}

#[get("/search/<termset>/<prefix>/<metric..>?<limit>&<direction>")]
pub fn search(
    termset: &str,
    prefix: &str,
    metric: Option<PathBuf>,
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
    let default_metric = PathBuf::from("ancestor_information_content");
    let metric_path = metric.unwrap_or(default_metric);
    let metric_str = metric_path.to_str().unwrap();
    let result = get_rss_from_pool().lock().unwrap().associations_search_with_cache(
        &assoc_predicate,                                           // object_closure_predicates
        &object_terms,                                              // object_set
        true,                                                       // include_similarity_object
        &None,                                                      // subject_set
        &subject_prefixes,                                          // subject_prefixes
        &search_type,                                               // search_type
        &MetricEnumWrapper::from_param(metric_str).unwrap(),        // score_metric
        Some(limit),                                                // limit
        &Some(direction_enum),                                      // direction
        &ASSOCIATION_CACHE,                                         // prefix_expansion_cache
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
