use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RadioBioError {
  #[error("Issue wih ({0}).cc: vec<f64> of a species")]
  UninitializedSpecies(String),

  #[error("Unknown species encountered ({0})")]
  UnknownSpecies(String),

  #[error("Try to push negative cc value of {0} for species {1}")]
  NegativeConcentration(f64, String),
}