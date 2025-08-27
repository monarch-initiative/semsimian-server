use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

use rocket::request::FromParam;
use semsimian::enums::{ DirectionalityEnum, MetricEnum, SearchTypeEnum };
use semsimian::{ Predicate, RustSemsimian, TermID };
use flate2::read::GzDecoder;

// Check for phenio.db in ~/.data/oaklib, download if not present
const PHENIO_DB_URL: &str = "https://data.monarchinitiative.org/monarch-kg-dev/latest/phenio.db.gz";

pub fn check_for_phenio() {
    let mut db_path = PathBuf::new();
    if let Some(home) = std::env::var_os("HOME") {
        db_path.push(home);
        db_path.push(".data/oaklib/phenio.db");
    } else {
        panic!("Failed to get home directory");
    }

    if db_path.exists() {
        println!("phenio.db found, launching server");
    } else {
        println!("phenio.db not found, downloading from {}", PHENIO_DB_URL);
        // Download the phenio.db.gz file
        let response = reqwest::blocking
            ::get(PHENIO_DB_URL)
            .expect("Failed to download phenio.db.gz");

        // Create the directory if it doesn't exist
        std::fs::create_dir_all(db_path.parent().unwrap()).expect("Failed to create directory");

        // Unpack the .gz
        let mut gz = GzDecoder::new(response);
        let mut out_file = std::fs::File
            ::create(&db_path)
            .expect("Failed to create phenio.db file");
        std::io::copy(&mut gz, &mut out_file).expect("Failed to write phenio.db file");
    }
}

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

// Define a wrapper type for Semsimian MetricEnum and DirectionalityEnum
pub struct MetricEnumWrapper(pub MetricEnum);
pub struct DirectionalityEnumWrapper(pub DirectionalityEnum);

// Implement FromParam for our wrappers
impl<'a> FromParam<'a> for MetricEnumWrapper {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "jaccard_similarity" => Ok(MetricEnumWrapper(MetricEnum::JaccardSimilarity)),
            "phenodigm_score" => Ok(MetricEnumWrapper(MetricEnum::PhenodigmScore)),
            "cosine_similarity" => Ok(MetricEnumWrapper(MetricEnum::CosineSimilarity)),
            _ => Ok(MetricEnumWrapper(MetricEnum::AncestorInformationContent)),
        }
    }
}

impl<'a> FromParam<'a> for DirectionalityEnumWrapper {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        match param {
            "bidirectional" => Ok(DirectionalityEnumWrapper(DirectionalityEnum::Bidirectional)),
            "subject_to_object" =>
                Ok(DirectionalityEnumWrapper(DirectionalityEnum::SubjectToObject)),
            "object_to_subject" =>
                Ok(DirectionalityEnumWrapper(DirectionalityEnum::ObjectToSubject)),
            _ => Ok(DirectionalityEnumWrapper(DirectionalityEnum::Bidirectional)),
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
