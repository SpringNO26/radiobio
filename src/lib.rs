#[macro_use]
extern crate assert_float_eq;
extern crate nalgebra as na;

use std::collections::HashMap;

pub mod reactions;
pub mod physics;

pub use reactions::reactions_parser::Env;
pub use physics::beam::{Beam, IsTimed};

// ODE Solver
pub use ode_solvers as odes;
use reactions::{traits::{IsChemicalReaction, RawSpecies}, SimSpecies};

type State = na::DVector<f64>;
type Time = f64;

pub struct ODESolver {
    sim_env: Env,
    beam: Beam,

}

impl odes::System<State> for ODESolver {
    fn system(&self, t: Time, y: &State, dy: &mut State) {

        // Get the dose_rate for the time t:
        let dr = self.beam.at(t).dose_rate();

        // Create a HashMap<species,f64> with the current cc + /!\ Acid/Base
        let sp_idx = self.sim_env.mapped_species();
        let mut sp_val:HashMap<String, f64> = HashMap::new();
        for (species, idx) in sp_idx.iter() {
            sp_val.insert(species.clone(), y[*idx]);
        }
        //Still need to add untracked species & AcidBasePartners
        for sp_sim in self.sim_env.species.iter() {
            match sp_sim {
                SimSpecies::TrackedSpecies(_) => continue,
                SimSpecies::ABCouple(_) => continue,
                SimSpecies::CstSpecies(sp) => {
                    sp_val.insert(sp.as_owned_str(), sp.cc_value());
                },
                SimSpecies::ABPartner(sp) => {
                    todo!();
                },
            }
        }
        // First compute production rate values from reaction list
        let mut reaction_values: Vec<f64> = vec![];
        /*
        for reaction in self.sim_env.reactions.iter() {
            reaction_values.push(reaction.compute_reaction(dr, sp));
        }
        */

        // For each tracked species, compute new concentration
        //  Basically for each tracked species, loop over involved reactions.

    }

}