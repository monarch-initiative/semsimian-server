use semsimian::termset_pairwise_similarity::TermsetPairwiseSimilarity;
use serde::{
    ser::{SerializeStruct, Serializer},
    Serialize,
};

#[derive(Debug)]
pub struct Tsps(pub TermsetPairwiseSimilarity);

impl Serialize for Tsps {
    // Semsimian doesn't have a Serialize implementation for TermsetPairwiseSimilarity
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Tsps", 9)?;
        state.serialize_field("subject_termset", &self.0.subject_termset)?;
        state.serialize_field("subject_best_matches", &self.0.subject_best_matches)?;
        state.serialize_field(
            "subject_best_matches_similarity_map",
            &self.0.subject_best_matches_similarity_map,
        )?;
        state.serialize_field("object_termset", &self.0.object_termset)?;
        state.serialize_field("object_best_matches", &self.0.object_best_matches)?;
        state.serialize_field(
            "object_best_matches_similarity_map",
            &self.0.object_best_matches_similarity_map,
        )?;
        state.serialize_field("average_score", &self.0.average_score)?;
        state.serialize_field("best_score", &self.0.best_score)?;
        state.serialize_field("metric", &self.0.metric)?;
        state.end()
    }
}
