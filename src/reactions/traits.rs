/* ---------------------------- External imports ---------------------------- */
use anyhow::Result;
use std::collections::HashMap;

/* ---------------------------- Internal imports ---------------------------- */
use super::k_reactions::ReactionRateIndex;


pub trait IsChemicalReaction {
    fn compute_reaction(&mut self, current_dose_rate:f64, sp:&HashMap<String, f64>)
    -> Result<()>;
    fn value(&self) -> f64;
}

pub trait RawSpecies {
    fn as_str(&self) -> &String;
    fn as_owned_str(&self) -> String {self.as_str().to_string()}
    fn cc_value(&self) -> f64;
    fn set_cc_value(&mut self, cc:f64);
}

pub trait IsTrackedSpecies {
    fn index(&self) -> usize;
    fn iter_kreaction_indexes(&self) -> std::slice::Iter<ReactionRateIndex>;
    fn link_kreaction(&mut self, index:ReactionRateIndex);
}