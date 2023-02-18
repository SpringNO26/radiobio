#![allow(dead_code)]

use std::fs::File;
use std::collections::HashMap;

use ron::de::from_reader;
use serde::Deserialize;

// Intern use
use super::{
    KReaction,
    AcidBase,
    Species,
};

type MapSpecies = HashMap<String, Species>;

pub struct Env {
    reactions: Reactions,
    species: MapSpecies,
}

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
pub fn parse_reactions_file(path: &str) -> RonReactions {
    let file = File::open(&path).expect("Failed Opening
        config reactions file");
    let config: RonReactions = match from_reader(file){
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse reactions data file: {}", e);
            std::process::exit(1);
        }
    };
    return config;
}

// Create a HashMap out of the reactions from .ron file
pub fn make_species_from_config(config: &RonReactions)
    -> HashMap<String, Species> {

    let mut out = HashMap::new();
    for reaction in &config.k_reactions{
        for species in reaction.iter() {
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
            for species in reaction.iter() {
                if !v.contains(species){
                    v.push(species.clone());
                }
            }
        }
        return v.len();
    }
}