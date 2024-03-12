use semsimian_server::{compare_termsets, search};

// test compare_termsets function
#[test]
fn test_compare() {
    let response = compare_termsets(&*"MP:0010771".to_string(), &*"HP:0004325".to_string(), "AncestorInformationContent");
    let tsps = &response.0;
    let integument_phenotype = "MP:0010771";
    let expected_first_match = String::from("match_source");
    // dbg!(&tsps);
    assert_eq!(
        tsps.subject_termset[0].keys().next().unwrap(),
        integument_phenotype,
        "First key in subject_termset is not MP:0010771"
    );
    let first_subject_termset_key = &tsps.subject_termset[0].keys().next().unwrap(); // Assuming there's only one key
    let subj_best_matches = &tsps.subject_best_matches[*first_subject_termset_key];
    // dbg!(&subj_best_matches)
    // let first_match = &subj_best_matches.get_key_value("MP:0010771");//.get_key_value("MP:0010771");
    let first_match = subj_best_matches
        .iter()
        .find(|&(_, v)| v == integument_phenotype)
        .map(|(k, __)| k)
        .unwrap();
    assert_eq!(first_match, &expected_first_match);
}

#[test]
fn test_search() {
    let _response = search(
        &*"HP:0000001,HP:0000002".to_string(),
        &*"ZFIN".to_string(),
        "AncestorInformationContent",
        Some(1),
    );
}