/* -------------------------------------------------------------------------- */
/*                             MODULE DEFINITIONS                             */
/* -------------------------------------------------------------------------- */
pub mod reactions;
pub mod physics;
pub mod env;

/* -------------------------------------------------------------------------- */
/* ---------------------------- External imports ---------------------------- */
#[macro_use]
extern crate assert_float_eq;


use anyhow::Context;
use ode_solvers as odes;

/* ---------------------------- Internal imports ---------------------------- */
use reactions::{traits::{IsTrackedSpecies}, SimSpecies};
use reactions::k_reactions::ReactionRateIndex;

/* ------------------------------- Re-exports ------------------------------- */

pub use env::{Env, State, Time};
pub use physics::beam::{Beam, IsTimed};

/* -------------------------- Type/func definitions ------------------------- */


pub struct ODESolver {
    pub sim_env: Env,
    pub beam: Beam,
    dim: usize,
}

impl ODESolver {
    pub fn new(env:Env, beam:Beam) -> Self {
        let dim = env.number_of_tracked_species();
        Self { sim_env: env,
               beam: beam,
               dim: dim,
             }
    }
    pub fn dimension(&self) -> usize { self.dim }
}

impl odes::System<State> for ODESolver {
    fn system(&self, t: Time, y: &State, dy: &mut State) {

        // Get the dose_rate for the time t:
        let dr = self.beam.at(t).dose_rate();

        // Create a HashMap<species,f64> with the current cc + /!\ Acid/Base
        let sp_cc = self.sim_env.mapped_cc_species(y);
        // First compute production rate values from reaction list
        let reaction_values: Vec<f64> = self.sim_env
            .compute_chemical_reactions(&sp_cc, dr)
            .with_context(||format!("Failure occurs at t = {t}"))
            .expect("");

        // Compute new concentration values:
        for sim_sp in self.sim_env.iter_tracked_species() {
            let mut kreaction_idx = vec![];
            let mut sp_idx:usize = 0;
            match sim_sp {
                SimSpecies::TrackedSpecies(sp) => {
                    kreaction_idx.extend(sp.iter_kreaction_indexes());
                    sp_idx = sp.index();
                },
                SimSpecies::ABCouple(ab) => {
                    kreaction_idx.extend(ab.iter_kreaction_indexes());
                    sp_idx = ab.index();
                },
                _ => continue,
            }

            dy[sp_idx] = 0_f64;
            for rr_idx in kreaction_idx {
                match rr_idx {
                    ReactionRateIndex::Consumption(idx) => {
                        dy[sp_idx] -= reaction_values[*idx];
                    },
                    ReactionRateIndex::Production(idx) => {
                        dy[sp_idx] += reaction_values[*idx];
                    },
                }
            }
        }
    }

}