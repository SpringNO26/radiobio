use serde::Deserialize;
use itertools::{chain};
use std::collections::HashMap;

// use of internal mods.
use super::traits::ChemicalReaction;
use super::Species;
use super::errors::RadioBioError;

#[derive(Debug)]
pub struct Stoichiometry {

}

#[derive(Debug, Clone)]
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

    fn compute_reaction(&self, species:&HashMap<String, Species>)
        -> Result<f64, RadioBioError>  {
        let mut res = self.k_value;
        for elt in &self.reactants {
            match species.get(elt) {
                Some(sp) => {
                    let val = sp.last_cc()?;
                    res *= val;
                },
                None => {
                    return Err(RadioBioError::UnknownSpecies(
                        elt.to_string() ));
                },
            }
        }
        Ok(res)
    }
}

impl KReaction {
    pub fn k_value(&self) -> f64 {self.k_value}
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        chain(self.reactants.iter(), self.products.iter())
    }
}


