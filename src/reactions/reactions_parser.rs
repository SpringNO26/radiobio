#![allow(dead_code)]

use std::ops::IndexMut;
use std::{fs::File};
use std::collections::HashMap;
use itertools::{chain};

use ron::{de::from_reader};
use serde::Deserialize;
// Intern use
use super::k_reactions::{ReactionSpecies, ReactionRateIndex};
use super::traits::{RawSpecies, IsTrackedSpecies};
use super::{
    KReaction,
    species::SimSpecies,
    acid_base::{AcidBase},
};
use super::errors::RadioBioError;

#[derive(Debug)]
pub struct Env {
    pub reactions: Vec<KReaction>,
    pub species: Vec<SimSpecies>,
    pub bio_param: BioParam,
}

impl Env {
    pub fn list_all_reactants(&self) -> Vec<String>{
        let mut out = vec![];
        for reaction in self.reactions.iter() {
            for sp in reaction.iter_reactants() {
                if !out.contains(sp.as_str()) {
                    out.push(sp.as_owned_str());
                }
            }
        }
        return out;
    }
    pub fn list_all_products(&self) -> Vec<String>{
        let mut out = vec![];
        for reaction in self.reactions.iter() {
            for sp in reaction.iter_products() {
                if !out.contains(sp.as_str()) {
                    out.push(sp.as_owned_str());
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

    pub fn mapped_species(&self) -> HashMap<String, usize> {
        mapped_species(&self.species)
    }

    pub fn iter_tracked_species(&self) -> impl Iterator<Item=&SimSpecies> {
        self.species.iter().filter(|x| x.is_tracked())
    }
}

#[derive(Debug, Deserialize)]
struct RonReactions {
    pub bio_param: BioParam,
    pub fixed_concentrations: HashMap<String, f64>,
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

    // Convert kReactions
    let mut kr_list: Vec<KReaction> = vec![];
    for elt in &config.k_reactions {
        let mut kr =
            KReaction::new_empty(elt.get_k_value());

        for sp in elt.iter_reactants() {
            kr.add_reactant(sp);
        }
        for sp in elt.iter_products() {
            kr.add_product(sp);
        }

        kr_list.push(kr);
    }


    let mut sim_species = make_species_from_config(&config);
    // Link kReactions to Species
    let map_species = mapped_species(&sim_species);

    for (r_idx, reaction) in kr_list.iter().enumerate() {
        for sp in reaction.iter_species() {

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
            match sim_species.index_mut(*idx) {
                SimSpecies::TrackedSpecies(sp) =>
                    {sp.link_kreaction(rrate_idx);},
                SimSpecies::ABCouple(ab) =>
                    {ab.link_kreaction(rrate_idx);},
                _ => {();},
            }
        }
    }

    return Ok(Env {
        reactions: kr_list,
        species: sim_species,
        bio_param: config.bio_param.clone(),
    });

}


// Create a Vec out of the reactions from .ron file
fn make_species_from_config(config: &RonReactions)
    -> Vec<SimSpecies> {

    let mut out:Vec<SimSpecies>= vec![];
    let mut idx:usize=0;
    let mut untracked:Vec<SimSpecies> = vec![];

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
            idx += 1;
        }
    }

    // Finally append the Untracked Species by consuming untracked
    for elt in untracked {
        out.push(elt);
    }
    return out
}

pub fn mapped_species(sp:&Vec<SimSpecies>) -> HashMap<String, usize> {
    let mut out = HashMap::new();
    for (idx, sim_sp) in sp.iter().enumerate() {
        match sim_sp {
            // Nothing to do.
            SimSpecies::ABCouple(_) => (),
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