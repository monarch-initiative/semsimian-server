use semsimian::{Predicate, RustSemsimian};
use std::path::PathBuf;


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