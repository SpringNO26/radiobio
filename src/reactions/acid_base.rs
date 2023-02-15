use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
}