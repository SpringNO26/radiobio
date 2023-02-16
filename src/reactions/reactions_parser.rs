#![allow(dead_code)]

use std::fs::File;

use ron::de::from_reader;
use serde::Deserialize;

// Intern use
use super::{KReaction, AcidBase};

#[derive(Debug, Deserialize)]
pub struct Reactions {
    pub acid_base: Vec<AcidBase>,
    pub k_reactions: Vec<KReaction>,
}

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

impl Reactions {
    pub fn number_of_species(&self) -> i32 {
        let mut v:Vec<String>;
    }
}