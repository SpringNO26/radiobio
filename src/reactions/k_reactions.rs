/* ---------------------------- External imports ---------------------------- */
use std::fmt;
use anyhow::{Result, bail};
use std::collections::HashMap;

/* ---------------------------- Internal imports ---------------------------- */
use super::traits::{IsChemicalReaction};
use super::errors::RadioBioError;
use super::species::ReactionSpecies;
use crate::physics::utils::ge_to_kr;

/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */

#[derive(Debug, Clone)]
pub enum ChemicalReaction {
    KReaction(KReaction),
    Radiolytic(RadiolyticReaction)
}

impl IsChemicalReaction for ChemicalReaction {
    fn compute_reaction(&self, current_dose_rate:f64, sp:&HashMap<String, f64>)
    -> Result<f64> {
        match self {
            ChemicalReaction::KReaction(r) =>
                r.compute_reaction(current_dose_rate, sp),
            ChemicalReaction::Radiolytic(r) =>
                r.compute_reaction(current_dose_rate, sp),
        }
    }

    fn species(&self) -> std::slice::Iter<ReactionSpecies> {
        match self {
            ChemicalReaction::KReaction(r) => r.species(),
            ChemicalReaction::Radiolytic(r) => r.species(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReactionRateIndex {
    Production(usize),
    Consumption(usize)
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KReaction{
    species: Vec<ReactionSpecies>,
    k_value: f64,
    stoichio: Vec<usize>,
    value: f64,
}

impl IsChemicalReaction for KReaction {
    fn compute_reaction(&self, _:f64, sp:&HashMap<String, f64>)
    -> Result<f64>{
        let mut res = self.k_value;
        for elt in self.iter_reactants().map(|x|x.as_str()) {
            match sp.get(elt) {
                Some(cc) => {
                    res *= cc;
                },
                None => {
                    bail!(RadioBioError::UnknownSpecies(elt.to_string()));
                },
            }
        }
        Ok(res)
    }
    fn species(&self) -> std::slice::Iter<ReactionSpecies> {
        self.species.iter()
    }


}

impl KReaction {
    pub fn new(
        species:Vec<ReactionSpecies>,
        k_value:f64,
        stoichio:Vec<usize>,
        ) -> Self {

        Self {species,
              k_value,
              stoichio,
              value: 0.0,
            }
    }

    pub fn new_empty(k_val:Option<f64>) -> Self {
        Self {
            species: vec![],
            k_value: k_val.unwrap_or(0.0),
            stoichio: vec![],
            value:0.0,
        }
    }

    pub fn number_of_reactants(&self) -> usize {
        self.species.iter()
                    .filter(|sp| sp.is_reactant())
                    .count()
    }

    pub fn k_value(&self) -> f64 {
        self.k_value / (self.number_of_reactants() as f64)
    }

    pub fn iter_species(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
    }

    pub fn iter_reactants(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
                    .filter(|sp| sp.is_reactant())
    }
    pub fn iter_products(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
                    .filter(|sp| !sp.is_reactant())
    }

    pub fn iter_reactants_indexed(&self)
        -> impl Iterator<Item=(usize, &ReactionSpecies)>
    {
        self.species.iter()
                    .enumerate()
                    .filter(|(_, sp)| sp.is_reactant())
    }
    pub fn iter_products_indexed(&self)
        -> impl Iterator<Item=(usize, &ReactionSpecies)>
    {
        self.species.iter()
                    .enumerate()
                    .filter(|(_, sp)| !sp.is_reactant())
    }

    pub fn index_of_reactant(&self, sp:&str) -> Result<usize> {
        self.iter_reactants()
            .position(|r| r.as_str() == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)).into()
        )
    }
    pub fn index_of_product(&self, sp:&str) -> Result<usize> {
        self.iter_products()
            .position(|r| r.as_str() == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)).into()
        )
    }
    pub fn is_reactant(&self, sp:&str) -> bool {
        self.iter_reactants().any(|elt| elt.as_str()==sp)
    }
    pub fn is_product(&self, sp:&str) -> bool {
        self.iter_products().any(|elt| elt.as_str()==sp)
    }
    pub fn get_stoichio(&self, sp:&str) -> Result<usize> {
        let idx = self.index_of_reactant(sp)?;
        self.stoichio.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)).into()
        )
    }

    pub fn get_product_stoichio(&self, sp:&str) -> Result<usize> {
        let idx = self.index_of_product(sp)?;
        self.stoichio.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotProduct(
                sp.to_string(),
                format!("{}", self)).into()
        )
    }

    pub fn add_reactant(&mut self, sp:&str) {
        match self.index_of_reactant(sp) {
            Ok(idx) => {
                self.stoichio[idx] += 1;
            } ,
            Err(_) => {
                self.species.push(
                    ReactionSpecies::Reactant(String::from(sp)));
                self.stoichio.push(1);
            }
        }
    }

    pub fn add_product(&mut self, sp:&str) {
        match self.index_of_product(sp) {
            Ok(idx) => {
                self.stoichio[idx] += 1;
            },
            Err(_) => {
                self.species.push(
                    ReactionSpecies::Product(String::from(sp)));
                self.stoichio.push(1);
            }
        }
    }
}

impl fmt::Display for KReaction {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut out: String = String::from("");
        for (idx, sp) in self.iter_reactants_indexed(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio[idx];
            if stoichio == 1 {
                out.push_str(sp.as_str());
            } else {
                out.push_str(&format!("{stoichio} {}", sp.as_str()));
            }
        }
        out.push_str(" -> ");
        for (idx, sp) in self.iter_products_indexed(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio[idx];
            if stoichio == 1 {
                out.push_str(sp.as_str());
            } else {
                out.push_str(&format!("{stoichio} {}", sp.as_str()));
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

/* -------------------------------------------------------------------------- */
/*                       RADIOLYTIC REACTIONS DEFINITION                      */
/* -------------------------------------------------------------------------- */
#[derive(Debug, Clone)]
pub struct RadiolyticReaction {
    species: Vec<ReactionSpecies>,
    reaction_cst : f64, //Kr => concentration yield (mol/l/Gy)
}

impl RadiolyticReaction {
    pub fn new_from_ge(species:String, ge: f64) -> Self {
        Self { species: vec![ReactionSpecies::Product(species),],
               reaction_cst: ge_to_kr(ge).unwrap() }
    }
    pub fn kr(&self) -> f64 {self.reaction_cst}
}

impl IsChemicalReaction for RadiolyticReaction {
    fn compute_reaction(&self, current_dose_rate:f64, _:&HashMap<String, f64>)
    -> Result<f64> {
        Ok(self.kr() * current_dose_rate)

    }

    fn species(&self) -> std::slice::Iter<ReactionSpecies> {
        self.species.iter()
    }

}

//Obsolete implementations
/*
    pub fn iter_special(&self) -> impl Iterator<Item = ReactionSpecies> + '_ {
        let it_1 = self.reactants.iter().map(
            |x|ReactionSpecies::Reactant(x.to_string()));
        let it_2 = self.reactants.iter().map(
            |x|ReactionSpecies::Product(x.to_string()));
        return chain(it_1, it_2);
    }
 */