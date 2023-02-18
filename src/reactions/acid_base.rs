use std::collections::HashMap;


use super::traits::ChemicalReaction;
use super::{Species, errors::RadioBioError};

// Struct storing the results of the Acid Partition compute
#[allow(dead_code)]
pub struct AcidPartition {
    acid: f64,
    base: f64,
    acid_derive: f64,
    base_derive: f64,
}


// Main AcidBase struct holding the logic of acid base partitioning.
#[derive(Debug, Clone)]
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

    #[allow(unused_variables)]
    fn compute_reaction(&self, species:&HashMap<String,Species>)
        -> Result<f64, RadioBioError> {
        todo!();
    }
}

#[allow(non_snake_case)]
impl AcidBase {

    pub fn new(acid:String, base: String, pKa: f64) -> Self {
        Self { acid, base, pKa }
    }

    pub fn pKa(&self) -> f64 {self.pKa}

    pub fn iter(&self) -> AcidBaseIter<'_> {
        AcidBaseIter { inner: self, index: 0 }
    }

    pub fn acid_partition(&self, cc_tot:f64, cc_H_plus:f64) -> AcidPartition {
        let ka = f64::powf(10.0, -self.pKa);
        AcidPartition {
            acid:     cc_tot / (1.0 + ka        / cc_H_plus),
            base:     cc_tot / (1.0 + cc_H_plus / ka       ),
            acid_derive: 1.0 / (1.0 + ka        / cc_H_plus),
            base_derive: 1.0 / (1.0 + cc_H_plus / ka       ),
        }
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
