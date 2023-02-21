#![allow(dead_code)]

use std::{fs::File};
use std::collections::HashMap;
use itertools::{chain};
use std::rc::Rc;

use ron::{de::from_reader};
use serde::Deserialize;

// Intern use
use super::{
    KReaction,
    AcidBase,
    species::MapSpecies,
    species::SimSpecies,
    species::SimpleSpecies,
};

#[derive(Debug)]
pub struct Env {
    pub reactions: Vec<KReaction>,
    pub species: MapSpecies,
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
}

#[derive(Debug, Deserialize)]
struct RonReactions {
    pub bio_param: BioParam,
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
    pH: f64,
    cc_H2O: f64,
    radiolytic: HashMap<String, f64>
}

// Read & Parse from .ron file
pub fn parse_reactions_file(path: &str) -> Env {
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

    return Env {
        reactions: kr_list,
        species: make_species_from_config(&config),
        bio_param: config.bio_param.clone(),
    };

}

// Create a HashMap out of the reactions from .ron file
fn make_species_from_config(config: &RonReactions)
    -> MapSpecies {

    let mut out = HashMap::new();
    let mut idx:usize=0;
    for reaction in &config.k_reactions {
        for sp in reaction.reactants.iter() {
            if !out.contains_key(sp){
                out.insert(
                    sp.clone(),
                    SimSpecies::RawSpecies(
                            SimpleSpecies::new(
                                sp.clone(), idx))
                );
                idx += 1;
            }
        }
    }
    // Add also the Acid/Base couples with it
    for elt in &config.acid_base {
        out.insert(
            elt.label(),
            SimSpecies::AcidBaseCouple( AcidBase::new(
                elt.acid(),
                elt.base(),
                elt.pKa(),
                idx )));
        idx += 1;
    }

    return out
}

// Check basic rules of chemistry/logic from .ron file
#[allow(unused_variables)]
fn check_parsed_reactions(config: &RonReactions) {
    todo!();
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