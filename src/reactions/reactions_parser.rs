#![allow(dead_code)]

use std::{fs::File};
use std::collections::HashMap;
use itertools::{chain};

use ron::{de::from_reader};
use serde::Deserialize;

// Intern use
use super::{
    KReaction,
    AcidBase,
    Species,
    species::MapSpecies,
};

#[derive(Debug)]
pub struct Env {
    pub reactions: Reactions,
    pub species: MapSpecies,
}

#[derive(Debug)]
pub struct Reactions {
    pub acid_base: Vec<AcidBase>,
    pub k_reactions: Vec<KReaction>
}

#[derive(Debug, Deserialize)]
struct RonReactions {
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

    // Convert AcidBase
    let mut ab = vec![];
    for elt in &config.acid_base {
        ab.push(AcidBase::new( elt.acid(), elt.base(), elt.pKa() ));
    }
    // Convert kReactions
    let mut kr_list: Vec<KReaction> = vec![];
    for elt in &config.k_reactions {
        let mut kr = KReaction::new_empty(elt.get_k_value());

        for sp in elt.iter_reactants() {
            kr.add_reactant(sp);
        }
        for sp in elt.iter_products() {
            kr.add_product(sp);
        }

        kr_list.push(kr);
    }

    return Env {
        reactions: Reactions {acid_base:ab, k_reactions: kr_list},
        species: make_species_from_config(&config),
    };

}

// Create a HashMap out of the reactions from .ron file
fn make_species_from_config(config: &RonReactions)
    -> MapSpecies {

    let mut out = HashMap::new();
    for reaction in &config.k_reactions {
        for species in chain(reaction.reactants.iter(), reaction.products.iter()) {
            if !out.contains_key(species){
                out.insert(
                    species.clone(),
                    Species::new(species.clone()));
            }
        }
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