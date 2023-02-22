
use std::{fmt, fmt::Display};
use std::collections::HashMap;

use super::acid_base::{AcidBase, ABPartner};
use super::traits::{IsTrackedSpecies, RawSpecies};

// Internal module use
//use super::errors::RadioBioError;

pub type MapSpecies = HashMap<String, SimSpecies>;

#[derive(Debug)]
pub enum SimSpecies {
    TrackedSpecies(SimpleSpecies),
    CstSpecies(CstSpecies), //No need to track it.
    ABCouple(AcidBase), // Also a Tracked Species
    ABPartner(ABPartner), // Not tracked in sim
}

impl SimSpecies {
    pub fn as_owned_str(&self) -> String {
        match self{
            Self::TrackedSpecies(sp) => sp.as_owned_str(),
            Self::CstSpecies(sp) => sp.as_owned_str(),
            Self::ABPartner(sp) => sp.as_owned_str(),
            // Cannot be borrowed as it is created on the fly
            Self::ABCouple(ab) => ab.as_owned_str(),
        }
    }
    pub fn new_tracked_species(label:String, index:usize) -> Self {
        Self::TrackedSpecies(SimpleSpecies::new(label, index))
    }
    pub fn new_cst_species(label:String, cc:f64) -> Self {
        Self::CstSpecies(CstSpecies::new(label, cc))
    }
    pub fn new_acid_partner(label:String, index:usize) -> Self {
        Self::ABPartner(ABPartner::new_acid(label, index))
    }
    pub fn new_base_partner(label:String, index:usize) -> Self {
        Self::ABPartner(ABPartner::new_base(label, index))
    }
    pub fn is_tracked(&self) -> bool {
        match self {
            Self::TrackedSpecies(_)  => true,
            Self::ABCouple(_)  => true,
            _ => false,
        }
    }

}


// For use in println!()
impl Display for SimSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimSpecies::TrackedSpecies(sp) => sp.fmt(f),
            SimSpecies::CstSpecies(sp) => sp.fmt(f),
            SimSpecies::ABCouple(ab) => ab.fmt(f),
            SimSpecies::ABPartner(sp) => sp.fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct SimpleSpecies {
    label: String,
    cc_value: f64,
    index: usize,
    kreaction: Vec<i32>,
}

impl RawSpecies for SimpleSpecies {
    fn as_str(&self) -> &String { &self.label }
    fn cc_value(&self) -> f64 { self.cc_value }
    fn set_cc_value(&mut self, cc:f64) {
        self.cc_value = cc;
    }
}

impl IsTrackedSpecies for SimpleSpecies {
    fn index(&self) -> usize { self.index }
    fn iter_kreaction_indexes(&self) -> std::slice::Iter<i32>{
        self.kreaction.iter()
    }
    fn link_kreaction(&mut self, index:i32) {
        self.kreaction.push(index);
    }
}

impl SimpleSpecies {
    pub fn new(label:String, index:usize) -> Self {
        Self {label, cc_value:0.0, index, kreaction:vec![]}
    }

    pub fn new_cst(label:String, cc:f64) -> Self {
        Self {label, cc_value:cc, index:usize::MAX, kreaction:vec![]}
    }

    pub fn name(&self) -> &str { &self.label }
    pub fn set_index(&mut self, index:usize) {
        self.index = index;
    }
}

// For use in println!()
impl Display for SimpleSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.label)
    }
}


#[derive(Debug)]
pub struct CstSpecies {
    label: String,
    cc_value: f64,
}

impl RawSpecies for CstSpecies {
    fn as_str(&self) -> &String { &self.label }
    fn cc_value(&self) -> f64 { self.cc_value }
    fn set_cc_value(&mut self, cc:f64) {
        self.cc_value = cc;
    }
}

impl CstSpecies {
    pub fn new(label:String, cc:f64) -> Self {
        Self {label, cc_value:cc}
    }

    pub fn name(&self) -> &str { &self.label }
}

// For use in println!()
impl Display for CstSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.label)
    }
}

/*
// Implements Basic maths operations between Species.
// --> Act on last element of cc vector
impl<'a, 'b> Add<&'b Species> for &'a Species {
    type Output = f64;
    fn add(self, rhs:&'b Species) -> f64 {
        let sp1 =  self.last_cc().unwrap_or(0.0);
        let sp2 = rhs.last_cc().unwrap_or(0.0);
        sp1 + sp2
    }
}

impl<'a, 'b> Mul<&'b Species> for &'a Species {
    type Output = f64;
    fn mul(self, rhs:&'b Species) -> f64 {
        let sp1 =  self.last_cc().unwrap_or(0.0);
        let sp2 = rhs.last_cc().unwrap_or(0.0);
        sp1 * sp2
    }
}

impl Mul<f64> for &Species {
    type Output = f64;
    fn mul(self, rhs:f64) -> f64 {
        let sp1 =  self.last_cc().unwrap_or(0.0);
        sp1 * rhs
    }
}

impl<'a, 'b> Sub<&'b Species> for &'a Species {
    type Output = f64;
    fn sub(self, rhs:&'b Species) -> f64 {
        let sp1 =  self.last_cc().unwrap_or(0.0);
        let sp2 = rhs.last_cc().unwrap_or(0.0);
        sp1 - sp2
    }
}
*/