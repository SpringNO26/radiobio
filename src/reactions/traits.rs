use std::collections::HashMap;
use super::Species;
use super::errors::RadioBioError;


pub type RResult = Result<ReactionResult, RadioBioError>;
pub enum ReactionResult {
    ProductionRate(f64),
    DerivateRate(f64),
    AcidPartition(ABPartition),
    BasePartition(ABPartition),
}

impl ReactionResult {
    pub fn from_prod_rate() -> ReactionResult {
        ReactionResult::ProductionRate(4.5)
    }
}

// Struct storing the results of the Acid Partition compute
#[allow(dead_code)]
pub struct ABPartition {
    cc: f64,
    derivative: f64,
}
impl ABPartition {
    pub fn new(cc:f64, derivative:f64) -> Self {
        Self { cc, derivative }
    }
}

pub trait ChemicalReaction {
    fn involves(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&HashMap<String, Species>)
        -> RResult;
    //fn derivative(&self, );
}