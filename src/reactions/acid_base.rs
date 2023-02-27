/* ---------------------------- External imports ---------------------------- */
use std::fmt::{self, Display};
/* ---------------------------- Internal imports ---------------------------- */
use super::traits::{
    RawSpecies,
    IsTrackedSpecies
};
use super::k_reactions::ReactionRateIndex;

/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
pub enum Chemical {
    Acid(String),
    Base(String),
}

impl Chemical {
    pub fn as_owned_str(&self) -> String {
        match self {
            Chemical::Acid(name) => name.to_owned(),
            Chemical::Base(name) => name.to_owned()
        }
    }
    pub fn as_str(&self) -> &String {
        match self {
            Chemical::Acid(name) => name,
            Chemical::Base(name) => name
        }
    }
}

#[derive(Debug)]
pub struct ABPartner {
    label: Chemical,
    // Index of the related Acid/Base Reaction. Better than a Rc cell to
    // insure mutability of the linked AcidBase structure.
    reaction_index: usize,
}
impl ABPartner {
    fn new(label:Chemical, index:usize) -> Self {
        Self { label: label, reaction_index: index }
    }
    pub fn new_acid(label:String, index:usize) -> Self {
        ABPartner::new(Chemical::Acid(label), index)
    }
    pub fn new_base(label:String, index:usize) -> Self {
        ABPartner::new(Chemical::Base(label), index)
    }
    pub fn index(&self) -> usize { self.reaction_index }
}
impl RawSpecies for ABPartner {
    fn as_str(&self) -> &String { self.label.as_str() }
}
// For use in println!()
impl Display for ABPartner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.label.as_str())
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
    kreaction: Vec<ReactionRateIndex>,
}
impl IsTrackedSpecies for AcidBase {
    fn index(&self) -> usize { self.index }
    fn iter_kreaction_indexes(&self) -> std::slice::Iter<ReactionRateIndex> {
        self.kreaction.iter()
    }
    fn link_kreaction(&mut self, index:ReactionRateIndex) {
        self.kreaction.push(index);
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
            }
    }
    pub fn pKa(&self) -> f64 {self.pKa}
    pub fn ka(&self)  -> f64 {self.ka}
    pub fn iter(&self) -> impl Iterator<Item=&Chemical> {
        vec![&self.acid, &self.base].into_iter()
    }
    pub fn compute_partition(&self, cc_tot:f64, cc_H_plus:f64)
    -> ABPartition {
        ABPartition::new(
             cc_tot / ( 1.0 + cc_H_plus / self.ka() ), // Base
            cc_tot / ( 1.0 + self.ka() / cc_H_plus ), // Acid
            1.0    / ( 1.0 + cc_H_plus / self.ka() ), // dBase / dCt
           1.0    / ( 1.0 + self.ka() / cc_H_plus ), // dAcid / dCt
        )
    }
    pub fn as_owned_str(&self) -> String {
        format!("{}/{}", self.acid.as_str(), self.base.as_str())
    }
    pub fn acid_str(&self) -> &String {
        self.acid.as_str()
    }
    pub fn base_str(&self) -> &String {
        self.base.as_str()
    }
}
impl fmt::Display for AcidBase {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} / {} pKa = {}",
                  self.acid.as_owned_str(),
                  self.base.as_owned_str(),
                  self.pKa)
    }
}
// Struct storing the results of the Acid Partition compute
#[derive(Debug, Clone)]
#[allow(dead_code, non_snake_case)]
pub struct ABPartition {
    A:f64,
    HA:f64,
    dA:f64,
    dHA:f64
}

#[allow(dead_code, non_snake_case)]
impl ABPartition {
    pub fn new(A:f64, HA:f64, dA:f64, dHA:f64) -> Self {
        Self { A, HA, dA, dHA}
    }
    pub fn acid(&self) -> f64 { self.HA }
    pub fn base(&self) -> f64 { self.A  }
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