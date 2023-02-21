
use std::fmt;

use super::traits::{
    ChemicalReaction,
    RResult,
    IsTrackedSpecies
};
use super::species::MapSpecies;

#[derive(Debug, Clone)]
pub enum Chemical {
    Acid(String),
    Base(String),
}

impl Chemical {
    pub fn str_name(&self) -> String {
        match self {
            Chemical::Acid(name) => name.to_owned(),
            Chemical::Base(name) => name.to_owned()
        }
    }
}

// Main AcidBase struct holding the logic of acid base partitioning.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: Chemical,
    base: Chemical,
    pKa: f64,
    ka: f64,
    index: usize,
    kreaction: Vec<i32>,
    partition: ABPartition
}

impl fmt::Display for AcidBase {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {} pKa = {}",
                  self.acid.str_name(),
                  self.base.str_name(),
                  self.pKa)
    }
}

impl IsTrackedSpecies for AcidBase {
    fn index(&self) -> usize { self.index }
    fn iter_kreaction_indexes(&self) -> std::slice::Iter<i32> {
        self.kreaction.iter()
    }
    fn link_kreaction(&mut self, index:i32) {
        self.kreaction.push(index);
    }
}

impl ChemicalReaction for AcidBase {
    fn involves(&self, species: &str) -> bool {
        self.acid.str_name()==species || self.base.str_name() == species
    }

    #[allow(unused_variables)]
    fn compute_reaction(&self, species:&MapSpecies) {
        todo!();
    }
}

#[allow(non_snake_case)]
impl AcidBase {

    pub fn new(acid:String, base: String, pKa: f64, index:usize) -> Self {
        Self { acid : Chemical::Acid(acid),
               base : Chemical::Base(base),
               pKa  : pKa,
               ka   : f64::powf(10.0, -pKa),
               index: index,
               kreaction: vec![],
               partition: ABPartition::new_empty(),
            }
    }

    pub fn pKa(&self) -> f64 {self.pKa}
    pub fn ka(&self)  -> f64 {self.ka}

    pub fn iter(&self) -> impl Iterator<Item=&Chemical> {
        vec![&self.acid, &self.base].into_iter()
    }

    pub fn update_partition(&mut self, cc_tot:f64, cc_H_plus:f64) {
        self.partition = ABPartition::new(
             cc_tot / ( 1.0 + cc_H_plus / self.ka() ), // Base
            cc_tot / ( 1.0 + self.ka() / cc_H_plus ), // Acid
            1.0    / ( 1.0 + cc_H_plus / self.ka() ), // dBase / dCt
           1.0    / ( 1.0 + self.ka() / cc_H_plus ), // dAcid / dCt
        );
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

    pub fn new_empty() -> Self {
        Self {A:0.0, HA:0.0, dA:0.0, dHA:0.0}
    }
}

/*
// Struct to enable easy iteration over (the 2) reactants.
// it is a bit overkill though...
pub struct AcidBaseIter<'a> {
    inner: &'a AcidBase,
    index: u8,
}

impl<'a> Iterator for AcidBaseIter<'a> {
    type Item = Chemical;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => Chemical::Acid(self.inner.acid.to_string()),
            1 => Chemical::Base(self.inner.base.to_string()),
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
*/