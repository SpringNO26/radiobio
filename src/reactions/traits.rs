use super::species::MapSpecies;
use super::errors::RadioBioError;
use super::k_reactions::ReactionRateIndex;

pub type RResult<T> = Result<T, RadioBioError>;


pub trait ChemicalReaction {
    fn involves(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&MapSpecies);
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
    /*
    fn created_by_kreaction(&mut self, index:usize) {
        self.link_kreaction(index as i32);
    }
    fn removed_by_kreaction(&mut self, index:usize) {
        self.link_kreaction(-(index as i32));
    }
    */
}