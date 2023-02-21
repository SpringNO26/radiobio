use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum RadioBioError {
  #[error("Issue wih ({0}).cc: vec<f64> of a species")]
  UninitializedSpecies(String),

  #[error("Unknown species encountered ({0})")]
  UnknownSpecies(String),

  #[error("Try to push negative cc value of {0} for species {1}")]
  NegativeConcentration(f64, String),

  #[error("Species ({0}) not a reactant of reaction: {1}")]
  SpeciesIsNotReactant(String, String),

  #[error("Species ({0}) not a product of reaction: {1}")]
  SpeciesIsNotProduct(String, String),

  #[error("{0}")]
  UnknownAcidBaseReaction(String),


}