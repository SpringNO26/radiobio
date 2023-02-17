use serde::Deserialize;
use super::traits::ChemicalReaction;

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct AcidBase {
    acid: String,
    base: String,
    pKa: f64,
}

impl ChemicalReaction for AcidBase {
    fn involve(&self, species: &str) -> bool {
        self.acid==species || self.base == species
    }
}

/*
impl IntoIterator for AcidBase {
    type Item = &str;
    type IntoIter = std::array::IntoIter<&str, 2>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([&*self.acid, &*self.base])
    }
}
*/

impl AcidBase {
    pub fn pKa(&self) -> f64 {self.pKa}
}