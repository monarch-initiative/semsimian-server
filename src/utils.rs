use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Mutex;

use semsimian::enums::{ DirectionalityEnum, MetricEnum, SearchTypeEnum };
use semsimian::{ Predicate, RustSemsimian, TermID };
use flate2::read::GzDecoder;

const PHENIO_DB_URL: &str = "https://data.monarchinitiative.org/monarch-kg-dev/latest/phenio.db.gz";

/// Resolve the phenio.db path: PHENIO_PATH env var takes priority,
/// otherwise fall back to $HOME/.data/oaklib/phenio.db.
fn resolve_phenio_path() -> PathBuf {
    if let Some(phenio_path) = std::env::var_os("PHENIO_PATH") {
        PathBuf::from(phenio_path)
    } else if let Some(home) = std::env::var_os("HOME") {
        let mut p = PathBuf::from(home);
        p.push(".data/oaklib/phenio.db");
        p
    } else {
        panic!("Neither PHENIO_PATH nor HOME is set");
    }
}

/// Check for phenio.db at the resolved path, download if not present.
pub fn check_for_phenio() {
    let db_path = resolve_phenio_path();

    if db_path.exists() {
        println!("phenio.db found at {}, launching server", db_path.display());
    } else {
        println!("phenio.db not found at {}, downloading from {}", db_path.display(), PHENIO_DB_URL);
        let response = reqwest::blocking
            ::get(PHENIO_DB_URL)
            .expect("Failed to download phenio.db.gz");

        std::fs::create_dir_all(db_path.parent().unwrap()).expect("Failed to create directory");

        let mut gz = GzDecoder::new(response);
        let mut out_file = std::fs::File
            ::create(&db_path)
            .expect("Failed to create phenio.db file");
        std::io::copy(&mut gz, &mut out_file).expect("Failed to write phenio.db file");
    }
}

pub fn get_rss_instance() -> RustSemsimian {
    let db_path = resolve_phenio_path();
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

/**
 * Implement parsing for MetricEnumWrapper and DirectionalityEnumWrapper
 */

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
