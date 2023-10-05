// import funcions from src/main.rs

use semsimian_server::{say_hello, compare_termsets};

// test say_hello function
#[test]
fn test_run() {
    assert_eq!(say_hello(), "Semsimian Server Online");
}

// test compare_termsets function
// #[test]
// fn test_compare_termsets() {
//     let tsps = compare_termsets("MP:0010771".to_string(), "HP:0004325".to_string());
//     assert_eq!(tsps.average_score, 0.0);
//     assert_eq!(tsps.best_score, 0.0);
//     assert_eq!(tsps.metric, "resnik");
//     assert_eq!(tsps.subject_termset, "MP:0010771");
//     assert_eq!(tsps.subject_best_matches, vec!["MP:0010771"]);
//     assert_eq!(tsps.subject_best_matches_similarity_map, vec![0.0]);
//     assert_eq!(tsps.object_termset, "HP:0004325");
//     assert_eq!(tsps.object_best_matches, vec!["HP:0004325"]);
//     assert_eq!(tsps.object_best_matches_similarity_map, vec![0.0]);
// }