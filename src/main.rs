
use radiobio::reactions::parse_reactions_file;
use radiobio::physics::beam::Beam;
use radiobio::{ODESolver, State};

use ode_solvers::dop853::*;
use ode_solvers::*;

use nalgebra::{DVector, dvector};

fn main() {
    let reaction_file = format!(
        "{}/data/reactions.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    // Env is {reactions -> {acid_base   -> vec<AcidBase>,
    //                      k_reactions -> vec<KReaction>
    //                     },
    //          species -> HashMap
    //       }
    let sim_env = parse_reactions_file(&reaction_file).unwrap();
    let beam = Beam::new_constant(String::from("e"), 1e4).expect("");
    let sim = ODESolver::new( sim_env, beam );

    let y0 = sim.sim_env.get_initial_values();
    let mut stepper = Dop853::new(sim, 0.0, 0.1, 1e-6, y0, 1e-14, 1e-14);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("{}", stats);
            //et path = Path::new("./outputs/three_body_dop853_dvector.dat");
            //save(stepper.x_out(), stepper.y_out(), path);
            //println!("Results saved in: {:?}", path);
        }
        Err(e) => println!("An error occured: {}", e),
    }



/*
    println!("\n\nBiologic parameters from RON file: ");
    println!("{:?}\n\n", sim_env.bio_param);

    println!("\n Chemical Reactions: ");
    for (idx, elt) in sim_env.reactions.iter().enumerate(){
        println!("{idx}) {elt:?}");
    }

    let x = sim_env.list_all_reactants();
    println!("\n\n There are {} species involved as reactants:", x.len());
    println!("{:?}", x);

    let x = sim_env.list_all_products();
    println!("\n\n There are {} species involved as products:", x.len());
    println!("{:?}", x);


    println!("\n\nSpecies Vec structure");
    for elt in &sim_env.species {
        println!("{:?}", elt);
    }

    let x = sim_env.number_of_tracked_species();
    println!("\n\n==> Number of tracked species: {}", x);

    let map_sp = sim_env.map_all_species();
    println!("\n\nHere is the map of Species:\n{:?}", map_sp);

    println!("\n\nTest Ge conversion: {:.4e}", ge_to_kr(2.8).unwrap());

    let (x,y) = (15.0_f64, 0.0);
    let c = x/y;
    println!("\n\nTest 0.0 division: {}", f64::is_nan(c));
    println!("Test 0.0 division: {}", c.is_nan());
    println!("Test 0.0 division: {}", c.is_finite());
    println!("Test 0.0 division: {}", c.is_infinite());

    println!("\n\nTest env capacities:");
    for elt in sim_env.iter_ABCouples() {
        println!("ABCouple: {:?}", elt);
    }
    let y = dvector![1e-6, 2e-4, 1e-5, 0.0, 0.0, 0.0, 0.0];
    let sp_cc = sim_env.mapped_cc_species(&y);
    println!("Mapped Species: {:?}", sp_cc);

    println!("\n\n Test of reaction computing: ");
    let r = sim_env.compute_chemical_reactions(&sp_cc, 1e3)
        .expect("Unable to compute chemical reactions results");
*/

/*     println!("\n\n Testing Beam: ");
    let mut beam = Beam::new_pulsed(String::from("p"),
                                         2.0,
                                         4.0,
                                         0.001).unwrap();
    println!("{:?}", beam);

    for elt in (0..20).map(|x| x as f64 * 0.5) {
        println!("Time is {:.1} s -> Dr: {:.2} Gy/s",elt, beam.at(elt).current_dose_rate());
    } */



/* -------------------------------- Old Tests ------------------------------- */
    /*
    let x = reactions.k_reactions[5].clone();
    println!("Reaction is: {:?}", &x);
    println!("\tcontains e_aq? {}", x.involve("e_aq"));
    println!("\tcontains H_r? {}", x.involve("H_r"));
    println!("\tcontains H2O2? {}", x.involve("H2O2"));
    println!("\tcontains h_r? {}", x.involve("h_r"));
    println!("\tk-value = {}", x.k_value());

    let mut hash = make_species_from_config(&reactions);
    let species = "H2O2".to_string();
    match hash.get(&species) {
        Some(sp) => println!("Found: {:?}", sp),
        None => println!("No species named: {:?}", species)
    }
    println!("There are {} species involved", hash.len());
    for (key, val) in &hash {
        println!("\t {:?}", val);
    }

    hash.entry(species).and_modify(
        |sp| sp.set_last_cc(55.0).unwrap()
    );
    println!("After modif: {:?}", hash.get("H2O2"));
    let sp1 = hash.get("H2O2").unwrap();
    let sp2 = hash.get("e_aq").unwrap();

    println!("Trying Math operation of species: ");
    println!(" -> Addition: {}", sp1+sp2);
    println!(" -> Multiplication: {}", sp1*sp2);
    println!(" -> Substraction: {}", sp2-sp1);

    let x = reactions.k_reactions[6].clone();
    println!("Another Reaction is: {:?}", &x);


    //Acid Base reactions test
    let x = reactions.acid_base[0].clone();
    for elt in x.iter() {
        println!("Acid Base species: {elt}");
    }
    */
}