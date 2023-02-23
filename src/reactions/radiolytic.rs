
/* ---------------------------- External imports ---------------------------- */
use anyhow::Result;
use std::collections::HashMap;

/* ---------------------------- Internal imports ---------------------------- */
use crate::physics::utils::ge_to_kr;
use super::traits::IsChemicalReaction;

/* -------------------------------------------------------------------------- */
/*                         FUNCTION/STRUCT DEFINITIONS                        */
/* -------------------------------------------------------------------------- */

#[derive(Debug)]
pub struct RadiolyticReaction {
    species: String,
    reaction_cst : f64, //Kr => concentration yield (mol/l/Gy)
    value: f64,
}

impl RadiolyticReaction {
    pub fn new_from_ge(species:String, ge: f64) -> Self {
        Self { species, reaction_cst: ge_to_kr(ge).unwrap(), value:0_f64 }
    }
    pub fn kr(&self) -> f64 {self.reaction_cst}
}

impl IsChemicalReaction for RadiolyticReaction {
    fn compute_reaction(&mut self, current_dose_rate:f64, sp:&HashMap<String, f64>)
    -> Result<()> {
        self.value = self.kr() * current_dose_rate;
        Ok(())
    }
    fn value(&self) -> f64 {self.value}
}

/* -------------------------------------------------------------------------- */
/*                                   TESTING                                  */
/* -------------------------------------------------------------------------- */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2+2, 4);
    }
}