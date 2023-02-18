use std::collections::HashMap;
use super::Species;
use super::errors::RadioBioError;

pub trait ChemicalReaction {
    fn involve(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&HashMap<String, Species>)
        -> Result<f64, RadioBioError>;
}