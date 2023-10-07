// import funcions from src/main.rs

use semsimian_server::{compare_termsets, say_hello};

// test say_hello function
#[test]
fn test_run() {
    assert_eq!(say_hello(), "Semsimian Server Online");
}

// test compare_termsets function
#[test]
fn test_compare_termsets() {
    let response = compare_termsets("MP:0010771".to_string(), "HP:0004325".to_string());
    assert_eq!(response.0 .0.subject_termset.len(), 1);
    // assert_eq!(response.0.0.subject_termset[0].id, "MP:0010771");
    // response.0.0.subject_best_matches.values().nth(0).score // no field `score` on type `std::option::Option<&BTreeMap<std::string::String, std::string::String>>`
}
