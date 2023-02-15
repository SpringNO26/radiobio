#![allow(dead_code)]

use std::{collections::HashMap, fs::File};

use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Reactions {
    boolean: bool,
    float: f32,
    map: HashMap<u8, char>,
    nested: Nested,
    tuple: (u32, u32),
    vec: Vec<Nested>,
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