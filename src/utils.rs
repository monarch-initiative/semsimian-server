use semsimian::enums::{ DirectionalityEnum, MetricEnum, SearchTypeEnum };
use semsimian::{ Predicate, RustSemsimian, TermID };
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

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
//     let db = Some(db_path.to_str().expect("Failed to convert path to string"));
//     return db;
// }

// Get a RustSemsimian instance, ensure phenio.db
pub fn get_rss_instance() -> RustSemsimian {
    let mut db_path = PathBuf::new();
    //if PHENIO_PATH is in the environment, use that as db_path, else use ~/.data/oaklib/phenio.db
    if let Some(phenio_path) = std::env::var_os("PHENIO_PATH") {
        db_path.push(phenio_path);
    } else if let Some(home) = std::env::var_os("HOME") {
        db_path.push(home);
        db_path.push(".data/oaklib/phenio.db");
    } else {
        panic!("Failed to get home directory");
    }
    let db = Some(db_path.to_str().expect("Failed to convert path to string"));
    // let db = check_for_phenio();

    let predicates: Option<Vec<Predicate>> = Some(vec!["rdfs:subClassOf".to_string()]);

    let assoc_predicate: HashSet<TermID> = HashSet::from(["biolink:has_phenotype".to_string()]);
    let rss = Mutex::new(Some(RustSemsimian::new(None, predicates, None, db, None)));

    {
        let mut locked_rss = rss.lock().unwrap();
        if let Some(rss_instance) = locked_rss.as_mut() {
            rss_instance.pregenerate_cache(&assoc_predicate, &SearchTypeEnum::Flat);
        }
    }

    // Take the RustSemsimian instance out of the Mutex
    let mut guard = rss.lock().unwrap();
    let rss_instance: Option<RustSemsimian> = guard.take();

    // Now rss_instance is an Option<RustSemsimian>
    rss_instance.unwrap()
}

// Define a wrapper type in your own crate
pub struct MetricEnumWrapper(pub MetricEnum);
pub struct DirectionalityEnumWrapper(pub DirectionalityEnum);

//
// Implement parsing for MetricEnumWrapper and DirectionalityEnumWrapper without Rocket
//

impl MetricEnumWrapper {
    pub fn from_param(param: &str) -> Option<Self> {
        match param {
            "jaccard_similarity" => Some(MetricEnumWrapper(MetricEnum::JaccardSimilarity)),
            "phenodigm_score" => Some(MetricEnumWrapper(MetricEnum::PhenodigmScore)),
            "cosine_similarity" => Some(MetricEnumWrapper(MetricEnum::CosineSimilarity)),
            _ => Some(MetricEnumWrapper(MetricEnum::AncestorInformationContent)),
        }
    }
}

impl DirectionalityEnumWrapper {
    pub fn from_param(param: &str) -> Option<Self> {
        match param {
            "bidirectional" => Some(DirectionalityEnumWrapper(DirectionalityEnum::Bidirectional)),
            "subject_to_object" =>
                Some(DirectionalityEnumWrapper(DirectionalityEnum::SubjectToObject)),
            "object_to_subject" =>
                Some(DirectionalityEnumWrapper(DirectionalityEnum::ObjectToSubject)),
            _ => Some(DirectionalityEnumWrapper(DirectionalityEnum::Bidirectional)),
        }
    }
}
// Implement Deref so you can use the wrapper type like the original enum
use std::ops::Deref;

impl Deref for MetricEnumWrapper {
    type Target = MetricEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DirectionalityEnumWrapper {
    type Target = DirectionalityEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
