use std::collections::HashMap;


use super::traits::{
    ChemicalReaction,
    RResult,
    ReactionResult,
    ABPartition
};
use super::{Species};



// Main AcidBase struct holding the logic of acid base partitioning.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
    ka: f64,
}

impl ChemicalReaction for AcidBase {
    fn involves(&self, species: &str) -> bool {
        self.acid==species || self.base == species
    }

    #[allow(unused_variables)]
    fn compute_reaction(&self, species:&HashMap<String,Species>)
        -> RResult {
        todo!();
    }
}

#[allow(non_snake_case)]
impl AcidBase {

    pub fn new(acid:String, base: String, pKa: f64) -> Self {
        Self { acid, base, pKa, ka:f64::powf(10.0, -pKa) }
    }

    pub fn pKa(&self) -> f64 {self.pKa}
    pub fn ka(&self)  -> f64 {self.ka}

    pub fn iter(&self) -> AcidBaseIter<'_> {
        AcidBaseIter { inner: self, index: 0 }
    }


    pub fn acid_partition(&self, cc_tot:f64, cc_H_plus:f64) -> RResult {
        let res = ABPartition::new(
            cc_tot / (1.0 + self.ka() / cc_H_plus),
            1.0 / (1.0 + self.ka() / cc_H_plus),
        );
        Ok(ReactionResult::AcidPartition(res))
    }

    pub fn base_partition(&self, cc_tot:f64, cc_H_plus:f64) -> RResult {
        let res = ABPartition::new (
            cc_tot / (1.0 + cc_H_plus / self.ka() ),
            1.0 / (1.0 + cc_H_plus / self.ka() ),
        );
        Ok(ReactionResult::BasePartition(res))
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
