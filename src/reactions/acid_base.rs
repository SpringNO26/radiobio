use serde::Deserialize;
use std::collections::HashMap;


use super::traits::ChemicalReaction;
use super::Species;

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

    fn compute_reaction(&self, species:&HashMap<String,Species>) -> f64 {
        todo!();
    }
}

impl AcidBase {
    pub fn pKa(&self) -> f64 {self.pKa}
    pub fn iter(&self) -> AcidBaseIter<'_> {
        AcidBaseIter { inner: self, index: 0 }
    }
}

// Struct to enable easy iteration over (the 2) reactants.
// it is a bit overkill though...
pub struct AcidBaseIter<'a> {
    inner: &'a AcidBase,
    index: u8,
}

impl<'a> Iterator for AcidBaseIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => &(self.inner.acid),
            1 => &(self.inner.base),
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}