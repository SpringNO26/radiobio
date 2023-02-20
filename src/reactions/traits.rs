use std::collections::HashMap;
use super::Species;
use super::errors::RadioBioError;


pub type RResult<T> = Result<T, RadioBioError>;
pub enum ReactionResult {
    ProductionRate(f64),
    DerivateRate(f64),
    AcidBasePartition(ABPartition),
}

impl ReactionResult {
    pub fn from_prod_rate() -> ReactionResult {
        ReactionResult::ProductionRate(4.5)
    }
}

// Struct storing the results of the Acid Partition compute
#[derive(Debug, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct ABPartition {
    pub A:f64,
    pub HA:f64,
    pub dA:f64,
    pub dHA:f64
}

#[allow(dead_code, non_snake_case)]
impl ABPartition {
    pub fn new(A:f64, HA:f64, dA:f64, dHA:f64) -> Self {
        Self { A, HA, dA, dHA}
    }
}

pub trait ChemicalReaction {
    fn involves(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&HashMap<String, Species>)
        -> RResult<ReactionResult>;
    //fn derivative(&self, );
}