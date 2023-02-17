use serde::Deserialize;
use itertools::{chain};
use std::collections::HashMap;

// use of internal mods.
use super::traits::ChemicalReaction;
use super::Species;

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

    fn compute_reaction(&self, species:&HashMap<String, Species>) -> f64 {
        let mut res = self.k_value;
        for sp in &self.reactants {
            res *= species.get(sp).unwrap().cc();
        }
        return res;
    }
}

impl KReaction {
    pub fn k_value(&self) -> f64 {self.k_value}
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        chain(self.reactants.iter(), self.products.iter())
    }
}