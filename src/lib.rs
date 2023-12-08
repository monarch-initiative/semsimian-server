#[macro_use]
extern crate rocket;

use std::collections::HashSet;
use std::sync::Mutex;

use lazy_static::lazy_static;
use rocket::serde::json::Json;
use semsimian::enums::SearchTypeEnum;
use semsimian::termset_pairwise_similarity::TermsetPairwiseSimilarity as Tsps;
use semsimian::{RustSemsimian, TermID};

use utils::get_rss_instance;
pub mod utils;

lazy_static! {
    static ref RSS: Mutex<RustSemsimian> = Mutex::new(get_rss_instance());
}

//--- ROUTES ---//
#[get("/")]
pub fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

#[get("/compare/<termset1>/<termset2>")]
pub fn compare_termsets(termset1: &str, termset2: &str) -> Json<Tsps> {
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
        "\nComparing termsets:\
        \nTermset 1: {:?}\
        \nTermset 2: {:?}\
        \n",
        terms1, terms2
    );
    let result = RSS
        .lock()
        .unwrap()
        .termset_pairwise_similarity(&terms1, &terms2);
    Json(result)
}

// #[get("/search")]
pub fn full_search(termset: &str, taxon: &str, taxon_termset: &str) -> Json<Vec<(f64, Option<Tsps>, String)>> {
    // ARGS:
    // profile_entities --                      termset for comparison
    // all_associated_objects_for_subjects --   termset from target taxon
    // flatten_result --                        i think this will be None
    // limit --                                 maybe None? or 20?
    // include_similarity_object --             true?

    // split termset into vector of TermIDs
    let mut object_set: HashSet<TermID> = HashSet::new();
    for term in termset.split(",") {
        object_set.insert(term.to_string());
    }
    info!("\nSearching for termset: {:?}\n", object_set);

    let mut expanded_subject_map: HashSet<TermID> = HashSet::new();
    for term in taxon_termset.split(",") {
        expanded_subject_map.insert(term.to_string());
    };
    let limit: Option<usize> = None; // Some(20);
    let include_similarity_object = true;

    let result: Vec<(f64, Option<Tsps>, String)> = RSS.lock().unwrap().full_search(
        &object_set,
        &expanded_subject_map,
        None,
        &limit,
        include_similarity_object,
    );
    Json(result)
}

#[get("/test_search")]
// pub fn full_search() {
pub fn test_associations_search_phenio_mondo() {
    let assoc_predicate: HashSet<TermID> = HashSet::from(["biolink:has_phenotype".to_string()]);
    let subject_prefixes: Option<Vec<TermID>> = Some(vec!["MONDO:".to_string()]);
    let object_terms: HashSet<TermID> = HashSet::from([
        "HP:0008132".to_string(),
        "HP:0000189".to_string(),
        "HP:0000275".to_string(),
        "HP:0000276".to_string(),
        "HP:0000278".to_string(),
        "HP:0000347".to_string(),
        "HP:0001371".to_string(),
        "HP:0000501".to_string(),
        "HP:0000541".to_string(),
        "HP:0000098".to_string(),
    ]);
    let search_type: SearchTypeEnum = SearchTypeEnum::Full;
    let limit: Option<usize> = Some(20);

    // Call the function under test
    let result = RSS.lock().unwrap().associations_search(
        &assoc_predicate,
        &object_terms,
        true,
        &None,
        &subject_prefixes,
        &search_type,
        limit,
    );
    println!("Result - {:?}", result);
    let unique_scores: HashSet<_> = result.iter().map(|(score, _, _)| score.to_bits()).collect();
    let count = unique_scores.len();
    assert!(count <= limit.unwrap());
}
