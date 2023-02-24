/* ---------------------------- External imports ---------------------------- */
use anyhow::Result;
use std::collections::HashMap;

/* ---------------------------- Internal imports ---------------------------- */
use super::k_reactions::{
    ReactionRateIndex,
    RadiolyticReaction,
    ChemicalReaction,
    KReaction,
};
use super::species::ReactionSpecies;

pub trait IsChemicalReactionList {
    fn push_reaction(&mut self, reaction:ChemicalReaction);
    fn push_radiolytic(&mut self, reaction:RadiolyticReaction) {
        self.push_reaction(ChemicalReaction::Radiolytic(reaction));
    }
    fn push_k_reaction(&mut self, reaction:KReaction) {
        self.push_reaction(ChemicalReaction::KReaction(reaction));
    }
}

pub trait IsChemicalReaction {
    fn compute_reaction(&mut self, current_dose_rate:f64, sp:&HashMap<String, f64>)
    -> Result<()>;
    fn value(&self) -> f64;
    fn species(&self) -> std::slice::Iter<ReactionSpecies>;
    fn reactants(&self) -> ReactantsIter{
        ReactantsIter { inner: self.species() }
    }
    fn products(&self) -> ProductsIter{
        ProductsIter { inner: self.species() }
    }
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

// Struct to enable easy iteration over species.
// it is a bit overkill though...
pub struct ReactantsIter<'a> {
    inner: std::slice::Iter<'a, ReactionSpecies>,
}

impl<'a> Iterator for ReactantsIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let res = match self.inner.nth(0) {
                Some(x) => x,
                None => return None
            };
            match res {
                ReactionSpecies::Product(_) => continue ,
                ReactionSpecies::Reactant(sp) => return Some(sp),
            };
        }
    }
}

pub struct ProductsIter<'a> {
    inner: std::slice::Iter<'a, ReactionSpecies>,
}

impl<'a> Iterator for ProductsIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let res = match self.inner.nth(0) {
                Some(x) => x,
                None => return None
            };
            match res {
                ReactionSpecies::Product(sp) => return Some(sp) ,
                ReactionSpecies::Reactant(_) => continue,
            };
        }
    }
}