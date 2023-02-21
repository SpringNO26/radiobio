
use std::ops::{Add, Mul, Sub};
use std::{fmt, fmt::Display};
use std::collections::HashMap;

// Internal module use
use super::errors::RadioBioError;

pub type MapSpecies = HashMap<String, Species>;

pub enum ChemicalSpecies {
    TrackedSpecies  (Species),
    UntrackedSpecies(Species),
    AcidBaseCouple  (Species), // Tracked by default
}

#[derive(Debug)]
pub struct Species {
    formula: String,
    cc: Vec<f64>, // History vector containing computed cc values.
}

impl Species {
    pub fn new(formula:String) -> Self {
        Self {formula, cc: vec![0.0]}
    }

    pub fn name(&self) -> &str { &self.formula }

    pub fn last_cc(&self) -> Result<f64, RadioBioError> {
        let n = self.has_history()?;
        Ok(self.cc[n-1])
    }

    pub fn set_last_cc(&mut self, cc:f64) -> Result<(), RadioBioError> {
        let n = self.has_history()?;
        self.cc[n-1] = cc;
        Ok(())
    }

    pub fn push_new_cc(&mut self, cc:f64) -> Result<(), RadioBioError> {
        match cc >= 0.0 {
            true => {
                self.cc.push(cc);
                Ok(())
            },
            false => Err(RadioBioError::NegativeConcentration(
                cc, self.formula.clone())),
        }
    }

    pub fn has_history(&self) -> Result<usize, RadioBioError> {
        match self.cc.len() {
            0 => Err(RadioBioError::UninitializedSpecies(
                self.formula.clone())),
            n => Ok(n)
        }
    }
}

// For use in println!()
impl Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.formula)
    }
}


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