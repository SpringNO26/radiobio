
/* ---------------------------- External imports ---------------------------- */
use std::collections::HashMap;
use anyhow::{Result, Context};
use ode_solvers as odes;

/* ---------------------------- Internal imports ---------------------------- */
use super::reactions::SimSpecies;
use super::reactions::k_reactions::ChemicalReaction;
use super::reactions::traits::{
    IsChemicalReaction,
    RawSpecies,
};
use super::reactions::reactions_parser::{
    BioParam,
    map_all_species,
};
use super::reactions::acid_base::AcidBase;

/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */

macro_rules! extract {
    ($e:expr, $p:path) => {
        match $e {
            $p(value) => Some(value),
            _ => None,
        }
    };
}

/* ------------------------- Type def for ODE solver ------------------------ */
pub type State = odes::DVector<f64>;
pub type Time = f64;
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct Env {
    pub reactions: Vec<ChemicalReaction>,
    pub species: Vec<SimSpecies>,
    pub bio_param: BioParam,
    pub initial_cc: HashMap<String, f64>,
}

impl Env {
    pub fn list_all_reactants(&self) -> Vec<String>{
        let mut out = vec![];
        for reaction in self.reactions.iter() {
            for sp in reaction.reactants() {
                if !out.contains(sp) {
                    out.push(String::from(sp));
                }
            }
        }
        return out;
    }
    pub fn list_all_products(&self) -> Vec<String>{
        let mut out = vec![];
        for reaction in self.reactions.iter() {
            for sp in reaction.products() {
                if !out.contains(sp) {
                    out.push(String::from(sp));
                }
            }
        }
        return out;
    }
    pub fn number_of_tracked_species(&self) -> usize {
        self.species.iter()
                    .filter(|x| x.is_tracked())
                    .count()
    }

    pub fn map_all_species(&self) -> HashMap<String, usize> {
        map_all_species(&self.species)
    }

    pub fn iter_tracked_species(&self) -> impl Iterator<Item=&SimSpecies> {
        self.species.iter()
            .filter(|x| x.is_tracked())
    }

    #[allow(non_snake_case)]
    pub fn iter_ABCouples(&self) -> impl Iterator<Item=&AcidBase> {
        self.species.iter()
                    .filter(|x| x.is_ABCouple())
                    .map(|x| extract!(x, SimSpecies::ABCouple).unwrap())
    }

    pub fn mapped_cc_species(&self, y:&State) -> HashMap<String, f64> {
        let mut out: HashMap<String, f64> = HashMap::new();
        let sp_idx = self.map_all_species();

        // Copy cc of TrackedSpecies from ODE Solver
        for (species, idx) in sp_idx.iter() {
            match y.get(*idx) {
                Some(&value) if value>=0_f64 => {
                    out.insert(species.clone(), value);
                }
                // Negative cc value case:
                Some(_) => {
                    out.insert(species.clone(), 0_f64);
                }
                None => {();}
            }

        }
        // Still need to add untracked species & AcidBasePartners
        for sp_sim in self.species.iter() {
            match sp_sim {
                SimSpecies::CstSpecies(sp) => {
                    out.insert(sp.as_owned_str(), sp.cc_value());
                },
                _ => continue
            }
        }
        self.compute_acid_base(&mut out);
        return out;
    }

    #[allow(non_snake_case)]
    pub fn compute_acid_base(&self, cc:&mut HashMap<String, f64>) {
        let cc_H_plus = *cc.get("H_plus").unwrap();
        for couple in self.iter_ABCouples() {
            let label = couple.as_owned_str();
            let cc_tot = *cc.get(&label).unwrap();
            let partition = couple.compute_partition( cc_tot, cc_H_plus);
            // Update Acid entry
            cc.entry(couple.acid_str().clone())
              .and_modify(|val| {*val=partition.acid();})
              .or_insert(partition.acid());
            // Update Base entry
            cc.entry(couple.base_str().clone())
              .and_modify(|val| {*val=partition.acid();})
              .or_insert(partition.acid());
        }
    }

    pub fn compute_chemical_reactions(&self, cc:&HashMap<String, f64>, dose_rate:f64)
    -> Result<Vec<f64>> {
        let mut out = vec![];
        for reaction in self.reactions.iter() {
            match reaction {
                ChemicalReaction::Radiolytic(r) => {
                    let val = r
                        .compute_reaction(dose_rate, cc)
                        .with_context(||format!("While computing reaction: {:?}", r))?;
                    out.push(val);
                },
                ChemicalReaction::KReaction(r) => {
                    let val = r
                        .compute_reaction(dose_rate, cc)
                        .with_context(||format!("While computing reaction: {:?}", r))?;
                    out.push(val);
                },
            }
        }
        return Ok(out);
    }

    // Create vector with cc's at t = 0
    pub fn get_initial_values(&self) -> State {
        let mut out = State::zeros(self.number_of_tracked_species());
        if self.initial_cc.len() <= 0 {
            return out;
        }
        let sp_idx = self.map_all_species();
        for (sp, value) in self.initial_cc.iter() {
            match sp_idx.get(sp) {
                Some(idx) => out[*idx] = *value,
                None => continue
            }
        }
        return out;
    }
}


/* -------------------------------------------------------------------------- */
/*                                   TESTING                                  */
/* -------------------------------------------------------------------------- */
//#[cfg(test)]
//mod tests {
//    //use super::*;
//}