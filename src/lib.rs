use std::collections::HashSet;
use std::sync::Mutex;

use axum::extract::{ Path, Query, Json };
use lazy_static::lazy_static;
use semsimian::enums::{ DirectionalityEnum, SearchTypeEnum };
use semsimian::termset_pairwise_similarity::{
    TermsetPairwiseSimilarity as Tsps,
    TermsetPairwiseSimilarity,
};
use semsimian::{ RustSemsimian, TermID };
use serde::Deserialize;
use tracing::info;

use crate::utils::get_rss_instance;
use crate::utils::DirectionalityEnumWrapper;
use crate::utils::MetricEnumWrapper;

pub mod utils;

lazy_static! {
    static ref RSS: RustSemsimian = get_rss_instance();
    static ref RSS_MUTEX: Mutex<RustSemsimian> = Mutex::new(get_rss_instance());
}

#[derive(Debug, Deserialize)]
pub struct CompareParams {
    pub termset1: String,
    pub termset2: String,
    pub metric: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub termset: String,
    pub prefix: String,
    pub metric: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub limit: Option<usize>,
    pub direction: Option<String>,
}

//--- ROUTES ---//

pub async fn say_hello() -> &'static str {
    "Semsimian Server Online"
}

pub async fn compare_termsets(Path(params): Path<CompareParams>) -> Json<Tsps> {
    // destructure params to get termset1, termset2, and metric
    let CompareParams { termset1, termset2, metric } = params;

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
        terms1,
        terms2
    );
    let metric = metric.unwrap_or("ancestor_information_content".to_string());
    let result = RSS.termset_pairwise_similarity(
        &terms1,
        &terms2,
        &MetricEnumWrapper::from_param(&metric).unwrap()
    );
    Json(result)
}

pub async fn search(
    Path(params): Path<SearchParams>,
    Query(query_params): Query<QueryParams>
) -> Json<Vec<(f64, Option<TermsetPairwiseSimilarity>, TermID)>> {
    // print path and query params
    println!(
        "\nSearch called with params:\
        \nPath Params: {:?}\
        \nQuery Params: {:?}\
        \n",
        params,
        query_params
    );
    let SearchParams { termset, prefix, metric } = params;
    let QueryParams { limit, direction } = query_params;
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
        .map(|d| DirectionalityEnumWrapper::from_param(&d).unwrap().0)
        .unwrap_or(DirectionalityEnum::Bidirectional);

    // Call the function under test
    let metric = metric.unwrap_or("ancestor_information_content".to_string());
    let result = RSS_MUTEX.lock()
        .unwrap()
        .associations_search(
            &assoc_predicate,
            &object_terms,
            true,
            &None,
            &subject_prefixes,
            &search_type,
            &MetricEnumWrapper::from_param(&metric).unwrap(),
            Some(limit),
            &Some(direction_enum)
        );
    println!("Result - {result:?}");

    // print each entry in the result vector
    for (score, tsps, subject) in result.iter() {
        println!("Score: {score:?}\nTsps: {tsps:?}\nSubject: {subject:?}\n");
    }

    Json(result)
}
