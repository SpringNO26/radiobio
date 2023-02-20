use itertools::{chain};
use std::fmt;

// use of internal mods.
use super::traits::{
    ChemicalReaction,
    ReactionResult,
    RResult,
};
use super::species::MapSpecies;
use super::errors::RadioBioError;

#[derive(Debug, Clone)]
pub struct Stoichiometry {
    pub reactants: Vec<usize>,
    pub products: Vec<usize>,
}

impl Stoichiometry {
    pub fn new() -> Self {Self { reactants: vec![], products: vec![] }}
}

#[derive(Debug, Clone)]
pub struct KReaction{
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
    stoichio: Stoichiometry,
}

impl fmt::Display for KReaction {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut out: String = String::from("");
        for (idx, sp) in self.reactants.iter().enumerate(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio.reactants[idx];
            if stoichio == 1 {
                out.push_str(sp);
            } else {
                out.push_str(&format!("{stoichio} {sp}"));
            }
        }
        out.push_str(" -> ");
        for (idx, sp) in self.products.iter().enumerate(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio.products[idx];
            if stoichio == 1 {
                out.push_str(sp);
            } else {
                out.push_str(&format!("{stoichio} {sp}"));
            }
        }
        write!(f, "{}", out)
    }
}

impl ChemicalReaction for KReaction {
    fn involves(&self, species: &str) -> bool {
     self.reactants.iter().any(|elt| elt==species) ||
     self.products.iter().any(|elt| elt==species)
    }

    fn compute_reaction(&self, species:&MapSpecies)
        -> RResult  {
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
        Ok(ReactionResult::ProductionRate(res))
    }
}

impl KReaction {
    pub fn new(
        reactants:Vec<String>,
        products:Vec<String>,
        k_value:f64,
        stoichio:Stoichiometry) -> Self {

        Self {reactants, products, k_value, stoichio}
    }

    pub fn new_empty(k_val:Option<f64>) -> Self {
        Self {
            reactants: vec![],
            products: vec![],
            k_value: k_val.unwrap_or(0.0),
            stoichio: Stoichiometry { reactants: vec![], products: vec![] }
        }
    }

    pub fn k_value(&self) -> f64 {
        self.k_value / self.reactants.len() as f64
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        chain(self.reactants.iter(), self.products.iter())
    }

    pub fn index_of_reactant(&self, sp:&str) -> Option<usize> {
        self.reactants.iter().position(|r| r == sp)
    }
    pub fn index_of_product(&self, sp:&str) -> Option<usize> {
        self.products.iter().position(|r| r == sp)
    }
    pub fn is_reactant(&self, sp:&str) -> bool {
        self.reactants.iter().any(|elt| elt==sp)
    }
    pub fn is_product(&self, sp:&str) -> bool {
        self.products.iter().any(|elt| elt==sp)
    }

    pub fn add_reactant(&mut self, sp:&str) {
        match self.index_of_reactant(sp) {
            Some(idx) => {
                self.stoichio.reactants[idx] += 1;
            } ,
            None => {
                self.reactants.push(String::from(sp));
                self.stoichio.reactants.push(1);
            }
        }
    }

    pub fn add_product(&mut self, sp:&str) {
        match self.index_of_product(sp) {
            Some(idx) => {
                self.stoichio.products[idx] += 1;
            } ,
            None => {
                self.products.push(String::from(sp));
                self.stoichio.products.push(1);
            }
        }
    }

    pub fn compute_derivative(&self, sp:&str) -> RResult {
        if !self.is_reactant(sp) {
            return Err(RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                String::from("")
            ));
        }
        let mut res = 0.0;
        return Ok(ReactionResult::DerivateRate(res));


    }
}


