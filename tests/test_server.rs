use axum::extract::{ Path, Query };
use semsimian_server::{ compare_termsets, search, CompareParams, QueryParams, SearchParams };

// test compare_termsets function
#[tokio::test]
async fn test_compare() {
    let response = compare_termsets(
        Path(CompareParams {
            termset1: "MP:0010771".to_string(),
            termset2: "HP:0004325".to_string(),
            metric: Some("jaccard_similarity".to_string()),
        })
    ).await;
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
        Path(SearchParams {
            termset: "HP:0000001,HP:0000002".to_string(),
            prefix: "ZFIN".to_string(),
            metric: Some("ancestor_information_content".to_string()),
        }),
        Query(QueryParams { limit: Some(1), direction: Some("bidirectional".to_string()) })
    );
}
