#![allow(dead_code)]

use std::{collections::HashMap, fs::File};

use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Reactions {
    acidBase: Vec<AcidBase>,
    k_eactions: Vec<KReaction>,
}

#[derive(Debug, Deserialize)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
}

#[derive(Debug, Deserialize)]
pub struct KReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
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