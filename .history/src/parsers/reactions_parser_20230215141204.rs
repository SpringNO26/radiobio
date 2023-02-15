#![allow(dead_code)]

use std::{collections::HashMap, fs::File};

use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reactions {
    acidBase: Vec<AcidBase>,
    kReactions: Vec<KReaction>,
}

#[derive(Debug, Deserialize)]
struct AcidBase {
    a: String,
    b: char,
}

#[derive(Debug, Deserialize)]
struct KReaction {
    a: String,
    b: char,
}

fn parse_reactions_file(path: &str) -> Reactions {
    let file = File::open(&path).expect("Failed Opening
        config reactions file");
    let config: Reactions = match from_reader(file){
        Ok(x) => x,
        Err(e) => {
            println!("Failed to parse reactions data file: {}", e);
            std::process::exit(1);
        }
    return config
    }


}