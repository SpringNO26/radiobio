
pub trait ChemicalReaction {
    fn involve(&self, species: &str) -> bool;
}