use serde::Deserialize;

// use of internal mods.
use super::traits::ChemicalReaction;

#[derive(Debug, Deserialize, Clone)]
pub struct KReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
}

impl ChemicalReaction for KReaction {
    fn involve(&self, species: &str) -> bool {
     self.reactants.iter().any(|elt| elt==species) ||
     self.products.iter().any(|elt| elt==species)
    }
}

impl KReaction {
    pub fn k_value(&self) -> f64 {self.k_value}
}