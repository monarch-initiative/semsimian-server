use semsimian::{Predicate, RustSemsimian};
use std::path::{Path, PathBuf};

// Check for phenio.db in ~/.data/oaklib, download if not present
// const PHENIO_DB_URL: &str = "https://data.monarchinitiative.org/monarch-kg-dev/latest/phenio.db.gz";

// pub fn check_for_phenio() {
//     let mut db_path = PathBuf::new();
//     if let Some(home) = std::env::var_os("HOME") {
//         db_path.push(home);
//         db_path.push(".data/oaklib/phenio.db.gz");
//     } else {
//         panic!("Failed to get home directory");
//     }
//     if !db_path.exists() {
//         println!("Downloading phenio.db");
//         let mut resp = reqwest::blocking::get(PHENIO_DB_URL).expect("Failed to download phenio.db");
//         let mut out = std::fs::File::create(&db_path).expect("Failed to create phenio.db");
//         std::io::copy(&mut resp, &mut out).expect("Failed to write phenio.db");
//     }
// }

// Get a RustSemsimian instance, ensure phenio.db 
pub fn get_rss_instance() -> RustSemsimian {
    let mut db_path = PathBuf::new();
    if let Some(home) = std::env::var_os("HOME") {
        db_path.push(home);
        db_path.push(".data/oaklib/phenio.db");
    } else {
        panic!("Failed to get home directory");
    }
    // check_for_phenio();
    let db = Some(db_path.to_str().expect("Failed to convert path to string"));

    let predicates: Option<Vec<Predicate>> = Some(vec![
        "rdfs:subClassOf".to_string(),
        // "BFO:0000050".to_string(),
        // "UPHENO:0000001".to_string(),
    ]);

    let mut rss = RustSemsimian::new(None, predicates, None, db);
    rss.update_closure_and_ic_map();
    rss
}
