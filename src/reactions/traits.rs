use super::species::MapSpecies;
use super::errors::RadioBioError;


pub type RResult<T> = Result<T, RadioBioError>;


pub trait ChemicalReaction {
    fn involves(&self, species: &str) -> bool;
    fn compute_reaction(&self, species:&MapSpecies);
}

pub trait IsTrackedSpecies {
    fn index(&self) -> usize;
    fn iter_kreaction_indexes(&self) -> std::slice::Iter<i32>;
    fn link_kreaction(&mut self, index:i32);
    fn created_by_kreaction(&mut self, index:usize) {
        self.link_kreaction(index as i32);
    }
    fn removed_by_kreaction(&mut self, index:usize) {
        self.link_kreaction(-(index as i32));
    }
}