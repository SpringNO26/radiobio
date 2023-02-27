#![allow(dead_code)]
/* ---------------------------- External imports ---------------------------- */
use std::ops::IndexMut;
use std::{fs::File};
use std::collections::HashMap;
use itertools::{chain};
use ron::{de::from_reader};
use serde::Deserialize;

/* ---------------------------- Internal imports ---------------------------- */
use super::k_reactions::{
    ReactionRateIndex,
    ChemicalReaction,
    RadiolyticReaction};
use super::traits::{
    RawSpecies,
    IsTrackedSpecies,
    IsChemicalReaction,
    IsChemicalReactionList
};
use super::{
    KReaction,
    species::SimSpecies,
    acid_base::AcidBase,
};
use super::species::ReactionSpecies;
use super::errors::RadioBioError;
use crate::env::Env;
/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */
impl IsChemicalReactionList for Vec<ChemicalReaction> {
    fn push_reaction(&mut self, reaction:ChemicalReaction) {
        self.push(reaction);
    }
}

#[derive(Debug, Deserialize)]
struct RonReactions {
    pub bio_param: BioParam,
    pub fixed_concentrations: HashMap<String, f64>,
    pub initial_concentrations: HashMap<String, f64>,
    pub acid_base: Vec<RonAcidBase>,
    pub k_reactions: Vec<RonKReaction>,
}
//Struct for Ron deserialization
#[derive(Debug, Deserialize, Clone)]
struct RonKReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
struct RonAcidBase {
    acid: String,
    base: String,
    pKa: f64,
}
#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct BioParam {
    pub pH: f64,
    pub radiolytic: HashMap<String, f64>
}

// Read & Parse from .ron file
pub fn parse_reactions_file(path: &str) -> Result<Env, RadioBioError> {
    let file = File::open(&path).expect("Failed Opening
        config reactions file");

    // Get data from file
    let config: RonReactions = match from_reader(file){
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse reactions data file: {}", e);
            std::process::exit(1);
        }
    };

    // Parse kReactions
    let mut reactions_list: Vec<ChemicalReaction> = vec![];
    for elt in &config.k_reactions {
        let mut kr =
            KReaction::new_empty(elt.get_k_value());

        for sp in elt.iter_reactants() {
            kr.add_reactant(sp);
        }
        for sp in elt.iter_products() {
            kr.add_product(sp);
        }

        reactions_list.push_k_reaction(kr);
    }


    let (mut sim_sp, tracked_sp) = make_species_from_config(&config);

    // Parse radiolytic yields
    for (sp, ge) in config.bio_param.radiolytic.iter() {
        if !tracked_sp.contains(sp) { continue; }
        reactions_list.push_radiolytic(
            RadiolyticReaction::new_from_ge(sp.clone(), *ge));
    }

    // Link Species to ChemicalReactions
    let map_species = map_all_species(&sim_sp);
    for (r_idx, reaction) in reactions_list.iter().enumerate() {
        for sp in reaction.species() {

            let idx = match map_species.get(sp.as_str()) {
                Some(x) => x,
                None => continue, // not a tracked species
            };

            let rrate_idx =  match sp {
                ReactionSpecies::Product(_) =>
                    ReactionRateIndex::Production(r_idx),
                ReactionSpecies::Reactant(_) =>
                    ReactionRateIndex::Consumption(r_idx),
            };
            match sim_sp.index_mut(*idx) {
                SimSpecies::TrackedSpecies(sp) =>
                    {sp.link_kreaction(rrate_idx);},
                SimSpecies::ABCouple(ab) =>
                    {ab.link_kreaction(rrate_idx);},
                _ => {();},
            }
        }
    }

    return Ok(Env {
        reactions: reactions_list,
        species: sim_sp,
        bio_param: config.bio_param.clone(),
        initial_cc: config.initial_concentrations,
    });

}


// Create a Vec out of the reactions from .ron file
fn make_species_from_config(config: &RonReactions)
    -> (Vec<SimSpecies>, Vec<String>) {

    let mut out:Vec<SimSpecies>= vec![];
    let mut idx:usize=0;
    let mut untracked:Vec<SimSpecies> = vec![];
    let mut tracked_species = vec![];

    // Manually add H_plus & OH_minus as constant A/B partners (pH related)
    untracked.push(SimSpecies::new_cst_species(
        String::from("H_plus"),
        f64::powf(10.0, -config.bio_param.pH)));

    untracked.push(SimSpecies::new_cst_species(
        String::from("OH_minus"),
        f64::powf(10.0, -14.0+config.bio_param.pH)));

    // Add also the Acid/Base couples with it
    for elt in &config.acid_base {
        out.push(
            SimSpecies::ABCouple(
                AcidBase::new(
                    elt.acid(),
                    elt.base(),
                    elt.pKa(),
                    idx,
                )));
        // Create both Acid and Base "RawSpecies" which will later be
        // appended to the final vector with all species
        tracked_species.push(elt.acid());
        tracked_species.push(elt.base());
        untracked.push(SimSpecies::new_acid_partner(
            elt.acid(),
            idx));
        untracked.push(SimSpecies::new_base_partner(
            elt.base(),
            idx));
        idx += 1;
    }

    // Loop over all k reactions to add all their reactants (not products)
    for reaction in &config.k_reactions {
        for sp in reaction.reactants.iter() {
            // First check if involved in a A/B reaction => skipped
            if untracked.iter()
                          .any(|elt| elt.as_owned_str()==*sp){
                continue;
            }
            // Second check if Species is declared as constant
            if config.fixed_concentrations.contains_key(sp) {
                untracked.push(
                    SimSpecies::new_cst_species(
                        sp.to_string(),
                        config.fixed_concentrations[sp]));
                continue;
            }
            // Third check if already added in final vector
            if out.iter()
                  .any(|elt| elt.as_owned_str()==*sp) {
                continue;
            }
            // Then create the new species and append it to the final vector
            out.push(SimSpecies::new_tracked_species(
                            sp.clone(),
                            idx));
            tracked_species.push(sp.clone());
            idx += 1;
        }
    }

    // Finally append the Untracked Species by consuming untracked
    for elt in untracked {
        out.push(elt);
    }
    (out, tracked_species)
}

pub fn map_all_species(sp:&Vec<SimSpecies>) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    for (idx, sim_sp) in sp.iter().enumerate() {
        match sim_sp {
            // Nothing to do.
            SimSpecies::ABCouple(ab) => {
                out.insert(ab.as_owned_str(), ab.index());
            },
            // Add to map
            SimSpecies::ABPartner(ab) => {
                out.insert(ab.as_owned_str(), ab.index());
            },
            SimSpecies::CstSpecies(sp) => {
                out.insert(sp.as_owned_str(), idx);
            },
            SimSpecies::TrackedSpecies(sp) => {
                out.insert(sp.as_owned_str(), idx);
            },
        }
    }
    return out;
}

// Check basic rules of chemistry/logic from .ron file
#[allow(unused_variables)]
fn check_parsed_reactions(config: &RonReactions) {
    todo!();
    // A Constant species cannot be involved in an Acid Base reaction /!\
}

impl RonReactions {
    pub fn number_of_species(&self) -> usize {
        let mut v:Vec<String> = vec![];
        for reaction in &self.k_reactions{
            for species in chain(reaction.reactants.iter(), reaction.products.iter()) {
                if !v.contains(species){
                    v.push(species.clone());
                }
            }
        }
        return v.len();
    }
}

#[allow(non_snake_case)]
impl RonAcidBase {
    pub fn acid(&self) -> String {self.acid.clone()}
    pub fn base(&self) -> String {self.base.clone()}
    pub fn pKa(&self) -> f64   {self.pKa  }
    pub fn label(&self) -> String {
        format!("{}/{}", self.acid, self.base)
    }
}

impl RonKReaction {
    pub fn iter_reactants(&self) -> impl Iterator<Item = &String> {
        self.reactants.iter()
    }
    pub fn iter_products(&self) -> impl Iterator<Item = &String> {
        self.products.iter()
    }
    pub fn get_k_value(&self) -> Option<f64> {
        Some(self.k_value)
    }
}