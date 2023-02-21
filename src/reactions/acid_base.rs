use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use super::errors::RadioBioError;
use super::traits::{
    ChemicalReaction,
    RResult,
    ReactionResult,
    ABPartition
};
use super::{Species};

#[derive(Debug, Clone)]
pub enum Chemical {
    Acid(String),
    Base(String),
}

//Structures Holding info about current acid/base equilibrium
#[derive(Debug, Clone)]
#[allow(non_snake_case, dead_code)]
pub struct AcidBaseEquilibrium {
    time_index: usize       ,
    cc_H_plus : f64         ,
    partitions: Vec<(Rc<AcidBase>, ABPartition)> ,
}
impl AcidBaseEquilibrium {
    pub fn new(time_index:usize, cc_H_plus:f64) -> Self {
        Self { time_index,
               cc_H_plus,
               partitions: vec![],
            }
    }

    pub fn add_partition(&mut self, reaction: Rc<AcidBase>, partition:ABPartition) {
        self.partitions.push((reaction, partition));
    }

    pub fn get_partition(&self, reaction: Rc<AcidBase>) -> RResult<&ABPartition> {
        for (ab, partition) in &self.partitions {
            if Rc::ptr_eq(ab, &reaction) {
                return Ok(partition);
            }
        }
        Err(RadioBioError::UnknownAcidBaseReaction(format!("{}", reaction)))
    }
}


// Main AcidBase struct holding the logic of acid base partitioning.
#[derive(Debug, Clone)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
    ka: f64,
}

impl fmt::Display for AcidBase {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {} pKa = {}", self.acid, self.base, self.pKa)
    }
}

impl ChemicalReaction for AcidBase {
    fn involves(&self, species: &str) -> bool {
        self.acid==species || self.base == species
    }

    #[allow(unused_variables)]
    fn compute_reaction(&self, species:&HashMap<String,Species>)
        -> RResult<ReactionResult> {
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

    pub fn acid_partition(&self, cc_tot:f64, cc_H_plus:f64) -> RResult<ReactionResult> {
        let res = ABPartition::new(
             cc_tot / ( 1.0 + cc_H_plus / self.ka() ), // Base
            cc_tot / ( 1.0 + self.ka() / cc_H_plus ), // Acid
            1.0    / ( 1.0 + cc_H_plus / self.ka() ), // dBase / dCt
           1.0    / ( 1.0 + self.ka() / cc_H_plus ), // dAcid / dCt
        );
        Ok(ReactionResult::AcidBasePartition(res))
    }

    pub fn identify_partner(&self, sp:&str) -> RResult<Chemical> {
        if self.acid == sp { return Ok(Chemical::Acid(sp.to_string()));}
        if self.base == sp { return Ok(Chemical::Base(sp.to_string()));}
        Err(RadioBioError::SpeciesIsNotReactant(
            sp.to_string(),
            format!("{}", self)))
    }
}

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
