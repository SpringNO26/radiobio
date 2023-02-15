use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
}