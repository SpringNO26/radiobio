use serde::Deserialize;
use itertools::{chain};

// use of internal mods.
use super::traits::ChemicalReaction;

#[derive(Debug, Deserialize, Clone)]
pub struct KReaction {
    reactants: Vec<String>,
    products: Vec<String>,
    k_value: f64,
}

impl ChemicalReaction for KReaction {
    fn involve(&self, species: &str) -> bool {
     self.reactants.iter().any(|elt| elt==species) ||
     self.products.iter().any(|elt| elt==species)
    }
}

impl KReaction {
    pub fn k_value(&self) -> f64 {self.k_value}
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.reactants.iter().chain(self.products.iter())
    }
}

/*
impl IntoIterator for KReaction {
    type Item = &str;
    type IntoIter = std::array::IntoIter<&str, 2>;

    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([&*self.acid, &*self.base])
    }
}

fn iter(&self) -> impl Iterator<Item = u8> {
    once(self.r).chain(once(self.g)).chain(once(self.b))
}

*/