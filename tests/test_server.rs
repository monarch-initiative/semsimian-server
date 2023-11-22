// import funcions from src/main.rs

use semsimian_server::{compare_termsets, say_hello};

// test say_hello function
#[test]
fn test_run() {
    assert_eq!(say_hello(), "Semsimian Server Online");
}

// test compare_termsets function
#[test]
fn test_compare() {
    let response = compare_termsets("MP:0010771".to_string(), "HP:0004325".to_string());
    let tsps = &response.0;
    // dbg!(&tsps);
    assert_eq!(
        tsps.subject_termset[0].keys().next().unwrap(),
        "MP:0010771",
        "First key in subject_termset is not MP:0010771"
    );
    let first_subject_termset_key = &tsps.subject_termset[0].keys().next().unwrap(); // Assuming there's only one key
    let subj_best_matches = &tsps.subject_best_matches[*first_subject_termset_key];
    // dbg!(&subj_best_matches)
    let first_match = &subj_best_matches.get_key_value("MP:0010771");//.get_key_value("MP:0010771");
    dbg!(&first_match); // for some reason this is returning None
}

// #[test]
// fn test_search() {}
