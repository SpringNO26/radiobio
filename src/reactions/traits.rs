use std::collections::HashMap;
use super::Species;

pub trait ChemicalReaction {
    fn involve(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&HashMap<String, Species>) -> f64;
}