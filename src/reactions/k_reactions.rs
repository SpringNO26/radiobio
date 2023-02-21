use itertools::{chain};
use std::fmt;
use std::rc::Rc;

// use of internal mods.
use super::traits::{
    ChemicalReaction,
    ReactionResult,
    RResult,
};
use super::species::MapSpecies;
use super::errors::RadioBioError;
use super::acid_base::{AcidBase, AcidBaseEquilibrium, Chemical};

#[derive(Debug, Clone)]
pub enum ReactionSpecies {
    Product(String),
    Reactant(String),
}

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
    acid_base: Vec<Rc<AcidBase>>,

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
        /*
        match self.acid_base.len() {
            0 => (),
            x => out.push_str(&format!(" (Linked to {x} \
                acid/base reactions)"))
        }
        */
        write!(f, "{}", out)
    }
}

impl ChemicalReaction for KReaction {
    fn involves(&self, species: &str) -> bool {
     self.reactants.iter().any(|elt| elt==species) ||
     self.products.iter().any(|elt| elt==species)
    }

    fn compute_reaction(&self, species:&MapSpecies)
        -> RResult<ReactionResult>  {
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
        stoichio:Stoichiometry,
        acid_base:  Rc<AcidBase>,
        ) -> Self {

        Self {reactants,
              products,
              k_value,
              stoichio,
              acid_base: vec![acid_base],
            }
    }

    pub fn new_empty(k_val:Option<f64>) -> Self {
        Self {
            reactants: vec![],
            products: vec![],
            k_value: k_val.unwrap_or(0.0),
            stoichio: Stoichiometry { reactants: vec![], products: vec![] },
            acid_base: vec![],
        }
    }

    pub fn k_value(&self) -> f64 {
        self.k_value / self.reactants.len() as f64
    }

    pub fn iter_special(&self) -> impl Iterator<Item = ReactionSpecies> + '_ {
        let it_1 = self.reactants.iter().map(
            |x|ReactionSpecies::Reactant(x.to_string()));
        let it_2 = self.reactants.iter().map(
            |x|ReactionSpecies::Product(x.to_string()));
        return chain(it_1, it_2);
    }

    pub fn iter_species(&self) -> impl Iterator<Item = &String> {
        chain(self.reactants.iter(), self.products.iter())
    }
    pub fn iter_reactants(&self) -> impl Iterator<Item=&String> {
        self.reactants.iter()
    }
    pub fn iter_products(&self) -> impl Iterator<Item=&String> {
        self.products.iter()
    }

    pub fn index_of_reactant(&self, sp:&str) -> RResult<usize> {
        self.reactants.iter().position(|r| r == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }
    pub fn index_of_product(&self, sp:&str) -> RResult<usize> {
        self.products.iter().position(|r| r == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }
    pub fn is_reactant(&self, sp:&str) -> bool {
        self.reactants.iter().any(|elt| elt==sp)
    }
    pub fn is_product(&self, sp:&str) -> bool {
        self.products.iter().any(|elt| elt==sp)
    }
    pub fn get_reactant_stoichio(&self, sp:&str) -> RResult<usize> {
        let idx = self.index_of_reactant(sp)?;
        self.stoichio.reactants.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }

    pub fn get_product_stoichio(&self, sp:&str) -> RResult<usize> {
        let idx = self.index_of_product(sp)?;
        self.stoichio.products.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }

    pub fn has_acidbase_dep(&self) -> bool {
        self.acid_base.len() > 0
    }
    pub fn is_linked_to_acidbase(&self, reaction:&Rc<AcidBase>) -> bool {
        for elt in &self.acid_base {
            // Rc::ptr_eq check for pointer (i.e. address) equality!
            if Rc::ptr_eq(elt, reaction) {
                return true;
            }
        }
        return false;
    }
    pub fn species_linked_to_acidbase(&self, sp:&str) -> Option<Rc<AcidBase>> {
        for reaction in &self.acid_base {
            if reaction.involves(sp) {
                return Some(Rc::clone(reaction));
            }
        }
        None
    }

    pub fn add_reactant(&mut self, sp:&str) {
        match self.index_of_reactant(sp) {
            Ok(idx) => {
                self.stoichio.reactants[idx] += 1;
            } ,
            Err(_) => {
                self.reactants.push(String::from(sp));
                self.stoichio.reactants.push(1);
            }
        }
    }

    pub fn add_acidbase_link(&mut self, reaction:Rc<AcidBase>) {
        self.acid_base.push(reaction);
    }

    pub fn add_product(&mut self, sp:&str) {
        match self.index_of_product(sp) {
            Ok(idx) => {
                self.stoichio.products[idx] += 1;
            } ,
            Err(_) => {
                self.products.push(String::from(sp));
                self.stoichio.products.push(1);
            }
        }
    }
}


