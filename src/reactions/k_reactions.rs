use itertools::{chain};

// use of internal mods.
use super::traits::ChemicalReaction;
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

impl ChemicalReaction for KReaction {
    fn involve(&self, species: &str) -> bool {
     self.reactants.iter().any(|elt| elt==species) ||
     self.products.iter().any(|elt| elt==species)
    }

    fn compute_reaction(&self, species:&MapSpecies)
        -> Result<f64, RadioBioError>  {
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
        Ok(res)
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

    pub fn k_value(&self) -> f64 {self.k_value}
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        chain(self.reactants.iter(), self.products.iter())
    }

    pub fn index_of_reactant(&self, sp:&str) -> Option<usize> {
        self.reactants.iter().position(|r| r == sp)
    }
    pub fn index_of_product(&self, sp:&str) -> Option<usize> {
        self.products.iter().position(|r| r == sp)
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
}


