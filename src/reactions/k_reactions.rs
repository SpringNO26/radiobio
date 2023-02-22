use std::fmt;

// use of internal mods.
use super::traits::{
    ChemicalReaction,
    RResult,
};
use super::species::MapSpecies;
use super::errors::RadioBioError;

#[derive(Debug, Clone)]
pub enum ReactionSpecies {
    Product(String),
    Reactant(String),
}

impl ReactionSpecies {
    pub fn as_str(&self) -> &String {
        match self {
            ReactionSpecies::Reactant(sp) => sp,
            ReactionSpecies::Product(sp) => sp
        }
    }
    pub fn as_owned_str(&self) -> String {
        match self {
            ReactionSpecies::Reactant(sp) => sp.to_string(),
            ReactionSpecies::Product(sp) => sp.to_string()
        }
    }
    pub fn is_reactant(&self) -> bool {
        match self {
            ReactionSpecies::Reactant(_) => true,
            ReactionSpecies::Product(_) => false
        }
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KReaction{
    species: Vec<ReactionSpecies>,
    k_value: f64,
    stoichio: Vec<usize>,
    value: f64,
}

impl ChemicalReaction for KReaction {
    fn involves(&self, species: &str) -> bool {
     self.species.iter().any(|elt| elt.as_str()==species)
    }

    #[allow(unused_variables)]
    fn compute_reaction(&self, species:&MapSpecies) {
        todo!();
        /*/
        let mut res = self.k_value;
        for elt in &self.reactants {
            match species.get(elt) {
                Some(sp) => {
                    let val = sp.last_cc()?;
                    res *= val;
                },
                None => {
                    return Err(RadioBioError::UnknownSpecies(
                        elt.to_string() ));
                },
            }
        }
        Ok(ReactionResult::ProductionRate(res))
    */
    }
}

impl KReaction {
    pub fn new(
        species:Vec<ReactionSpecies>,
        k_value:f64,
        stoichio:Vec<usize>,
        ) -> Self {

        Self {species,
              k_value,
              stoichio,
              value: 0.0,
            }
    }

    pub fn new_empty(k_val:Option<f64>) -> Self {
        Self {
            species: vec![],
            k_value: k_val.unwrap_or(0.0),
            stoichio: vec![],
            value:0.0,
        }
    }

    pub fn number_of_reactants(&self) -> usize {
        self.species.iter()
                    .filter(|sp| sp.is_reactant())
                    .count()
    }

    pub fn k_value(&self) -> f64 {
        self.k_value / (self.number_of_reactants() as f64)
    }

    pub fn iter_species(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
    }
    pub fn iter_reactants(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
                    .filter(|sp| sp.is_reactant())
    }
    pub fn iter_products(&self) -> impl Iterator<Item=&ReactionSpecies> {
        self.species.iter()
                    .filter(|sp| !sp.is_reactant())
    }
    pub fn iter_reactants_indexed(&self)
        -> impl Iterator<Item=(usize, &ReactionSpecies)>
    {
        self.species.iter()
                    .enumerate()
                    .filter(|(_, sp)| sp.is_reactant())
    }
    pub fn iter_products_indexed(&self)
        -> impl Iterator<Item=(usize, &ReactionSpecies)>
    {
        self.species.iter()
                    .enumerate()
                    .filter(|(_, sp)| !sp.is_reactant())
    }

    pub fn index_of_reactant(&self, sp:&str) -> RResult<usize> {
        self.iter_reactants()
            .position(|r| r.as_str() == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }
    pub fn index_of_product(&self, sp:&str) -> RResult<usize> {
        self.iter_products()
            .position(|r| r.as_str() == sp).ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }
    pub fn is_reactant(&self, sp:&str) -> bool {
        self.iter_reactants().any(|elt| elt.as_str()==sp)
    }
    pub fn is_product(&self, sp:&str) -> bool {
        self.iter_products().any(|elt| elt.as_str()==sp)
    }
    pub fn get_stoichio(&self, sp:&str) -> RResult<usize> {
        let idx = self.index_of_reactant(sp)?;
        self.stoichio.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotReactant(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }

    pub fn get_product_stoichio(&self, sp:&str) -> RResult<usize> {
        let idx = self.index_of_product(sp)?;
        self.stoichio.get(idx).cloned().ok_or(
            RadioBioError::SpeciesIsNotProduct(
                sp.to_string(),
                format!("{}", self)
            )
        )
    }

    pub fn add_reactant(&mut self, sp:&str) {
        match self.index_of_reactant(sp) {
            Ok(idx) => {
                self.stoichio[idx] += 1;
            } ,
            Err(_) => {
                self.species.push(
                    ReactionSpecies::Reactant(String::from(sp)));
                self.stoichio.push(1);
            }
        }
    }

    pub fn add_product(&mut self, sp:&str) {
        match self.index_of_product(sp) {
            Ok(idx) => {
                self.stoichio[idx] += 1;
            },
            Err(_) => {
                self.species.push(
                    ReactionSpecies::Product(String::from(sp)));
                self.stoichio.push(1);
            }
        }
    }
}

impl fmt::Display for KReaction {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        let mut out: String = String::from("");
        for (idx, sp) in self.iter_reactants_indexed(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio[idx];
            if stoichio == 1 {
                out.push_str(sp.as_str());
            } else {
                out.push_str(&format!("{stoichio} {}", sp.as_str()));
            }
        }
        out.push_str(" -> ");
        for (idx, sp) in self.iter_products_indexed(){
            if idx > 0 {
                out.push_str(" + ");
            }
            let stoichio = self.stoichio[idx];
            if stoichio == 1 {
                out.push_str(sp.as_str());
            } else {
                out.push_str(&format!("{stoichio} {}", sp.as_str()));
            }
        }
        /*
        match self.acid_base.len() {
            0 => (),
            x => out.push_str(&format!(" (Linked to {x} \
                acid/base reactions)"))
        }
        */
        write!(f, "{}", out)
    }
}



//Obsolete implementations
/*
    pub fn iter_special(&self) -> impl Iterator<Item = ReactionSpecies> + '_ {
        let it_1 = self.reactants.iter().map(
            |x|ReactionSpecies::Reactant(x.to_string()));
        let it_2 = self.reactants.iter().map(
            |x|ReactionSpecies::Product(x.to_string()));
        return chain(it_1, it_2);
    }
 */