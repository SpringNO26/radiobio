use serde::Deserialize;
use super::traits::ChemicalReaction;

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
}

impl ChemicalReaction for AcidBase {
    fn involve(&self, species: &str) -> bool {
        self.acid==species || self.base == species
    }
}