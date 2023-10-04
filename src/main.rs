// This is a simple web server that uses the semsimian crate to compare two sets of terms
// TODO:
// - Initialize RustSemsimian object once and pass it to the compare_termsets function
// - Implement Serialize for RustSemsimian structs so that we can return it from the compare_termsets function
// - it's only grabbing the first term from each termset right now, need to fix that

//--- IMPORTS ---//
use std::{collections::HashSet, path::PathBuf};

// this lets us use the #[get] macro etc.
#[macro_use]
extern crate rocket;

// this lets us return JSON from our routes
use rocket::serde::json::Json; 
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};
use semsimian::termset_pairwise_similarity::TermsetPairwiseSimilarity;
use semsimian::{Predicate, RustSemsimian, TermID};

//--- STRUCTS ---//
struct TSPS(TermsetPairwiseSimilarity);

// Semsimian doesn't have a Serialize implementation for TermsetPairwiseSimilarity
impl Serialize for TSPS {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("TSPS", 9)?;
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


#[get("/compare/<termset1>/<termset2>")]
// #[get("/compare")]
fn compare_termsets(termset1: String, termset2: String) -> Json<TSPS> {
    // Compare two termsets, each represented as a comma-separated set of term IDs
    // Return a TermsetPairwiseSimilarity object

    // get path to phenio.db
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

    // split termset1 and termset2 into vectors of TermIDs

    let mut terms1: HashSet<TermID> = HashSet::new();
    for term in termset1.split(",") {
        terms1.insert(term.to_string());
    };
    
    let mut terms2: HashSet<TermID> = HashSet::new();
    for term in termset2.split(",") {
        terms2.insert(term.to_string());
    };
    println!("Termset 1: {:?}", terms1);
    println!("Termset 2: {:?}", terms2);
    let result = rss.termset_pairwise_similarity(&terms1, &terms2, &None);
    Json(TSPS(result))
}

// this is our get route which will be requested at the "/" location wherever it is mounted
#[get("/")]
fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

// start the web server and mount our get route at "/api". Can replace /api with anything
// or just leave it as "/" as the default location
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![say_hello, compare_termsets])
}
