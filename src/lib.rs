//--- IMPORTS ---//
use std::{collections::HashSet, path::PathBuf};

// this lets us use the #[get] macro etc.
#[macro_use]
extern crate rocket;

// this lets us return JSON from our routes
use lazy_static::lazy_static;
use rocket::serde::json::Json;
use semsimian::termset_pairwise_similarity::TermsetPairwiseSimilarity;
use semsimian::{Predicate, RustSemsimian, TermID};
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

//--- STRUCTS ---//
pub struct Tsps(TermsetPairwiseSimilarity);

// Semsimian doesn't have a Serialize implementation for TermsetPairwiseSimilarity
impl Serialize for Tsps {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tsps", 9)?;
        state.serialize_field("subject_termset", &self.0.subject_termset)?;
        state.serialize_field("subject_best_matches", &self.0.subject_best_matches)?;
        state.serialize_field(
            "subject_best_matches_similarity_map",
            &self.0.subject_best_matches_similarity_map,
        )?;
        state.serialize_field("object_termset", &self.0.object_termset)?;
        state.serialize_field("object_best_matches", &self.0.object_best_matches)?;
        state.serialize_field(
            "object_best_matches_similarity_map",
            &self.0.object_best_matches_similarity_map,
        )?;
        state.serialize_field("average_score", &self.0.average_score)?;
        state.serialize_field("best_score", &self.0.best_score)?;
        state.serialize_field("metric", &self.0.metric)?;
        state.end()
    }
}

// Get a RustSemsimian instance
pub fn get_rss_instance() -> RustSemsimian {
    let mut db_path = PathBuf::new();
    if let Some(home) = std::env::var_os("HOME") {
        db_path.push(home);
        db_path.push(".data/oaklib/phenio.db");
    } else {
        panic!("Failed to get home directory");
    }
    let db = Some(db_path.to_str().expect("Failed to convert path to string"));

    let predicates: Option<Vec<Predicate>> = Some(vec![
        "rdfs:subClassOf".to_string(),
        "BFO:0000050".to_string(),
        "UPHENO:0000001".to_string(),
    ]);

    let mut rss = RustSemsimian::new(None, predicates, None, db);
    rss.update_closure_and_ic_map();
    rss
}

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
    println!("Termset 1: {:?}", terms1);
    println!("Termset 2: {:?}", terms2);
    let result = RSS.termset_pairwise_similarity(&terms1, &terms2);
    Json(Tsps(result))
}

//--- Run Server ---//
#[launch]
fn rocket() -> _ {
    // run a first compare to warm up the RustSemsimian instance
    RSS.termset_pairwise_similarity(
        &HashSet::from(["MP:0010771".to_string()]),
        &HashSet::from(["HP:0004325".to_string()]),
    );
    rocket::build().mount("/", routes![say_hello, compare_termsets])
}
