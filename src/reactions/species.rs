
//Internal module use:
use super::{KReaction, AcidBase};

pub struct Species {
    formula: String,
    //kreaction_idx: Vec<usize>,
}

impl Species {
    pub fn new(formula:String) -> Self {
        Self {formula}
    }
}

pub trait TrackedSpecies {

}


impl TrackedSpecies for Species {

}

