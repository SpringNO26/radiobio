
use std::{fmt, fmt::Display};
use std::collections::HashMap;

use super::AcidBase;
use super::traits::IsTrackedSpecies;

// Internal module use
//use super::errors::RadioBioError;

pub type MapSpecies = HashMap<String, SimSpecies>;

#[derive(Debug)]
pub enum SimSpecies {
    RawSpecies(SimpleSpecies),
    AcidBaseCouple(AcidBase)
}
/*
impl SimSpecies {
    pub fn
}
*/
impl IsTrackedSpecies for SimSpecies {
    fn index(&self) -> usize {
        match self {
            SimSpecies::AcidBaseCouple(ab) => ab.index(),
            SimSpecies::RawSpecies(sp) => sp.index()
        }
    }

    fn iter_kreaction_indexes(&self) -> std::slice::Iter<i32> {
        match self {
            SimSpecies::AcidBaseCouple(ab) =>
                ab.iter_kreaction_indexes(),
            SimSpecies::RawSpecies(sp) =>
                sp.iter_kreaction_indexes()
        }
    }

    fn link_kreaction(&mut self, index:i32) {
        match self {
            SimSpecies::AcidBaseCouple(ab) =>
                ab.link_kreaction(index),
            SimSpecies::RawSpecies(sp) =>
                sp.link_kreaction(index)
        }
    }
}

// For use in println!()
impl Display for SimSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SimSpecies::RawSpecies(sp) => sp.fmt(f),
            SimSpecies::AcidBaseCouple(ab) => ab.fmt(f)
        }
    }
}


#[derive(Debug)]
pub struct SimpleSpecies {
    formula: String,
    index: usize,
    kreaction: Vec<i32>,
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
    pub fn new(formula:String, index:usize) -> Self {
        Self {formula, index, kreaction:vec![]}
    }

    pub fn name(&self) -> &str { &self.formula }
    pub fn set_index(&mut self, index:usize) {
        self.index = index;
    }
}

// For use in println!()
impl Display for SimpleSpecies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.formula)
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