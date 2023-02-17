#![allow(dead_code)]

use std::fs::File;
use std::collections::HashMap;

use ron::de::from_reader;
use serde::Deserialize;

// Intern use
use super::{KReaction, AcidBase, Species};

#[derive(Debug, Deserialize)]
pub struct Reactions {
    pub acid_base: Vec<AcidBase>,
    pub k_reactions: Vec<KReaction>,
}

// Read & Parse from .ron file
pub fn parse_reactions_file(path: &str) -> Reactions {
    let file = File::open(&path).expect("Failed Opening
        config reactions file");
    let config: Reactions = match from_reader(file){
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse reactions data file: {}", e);
            std::process::exit(1);
        }
    };
    return config;
}

// Create a HashMap out of the reactions from .ron file
pub fn make_species_from_config(config: &Reactions)
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
fn check_parsed_reactions(config: &Reactions) {

}

impl Reactions {
    pub fn number_of_species(&self) -> i32 {
        let mut v:Vec<String>;
        42
    }
}