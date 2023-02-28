
#[allow(unused_imports)]
use std::{fs::File, io::BufWriter, io::Write, path::Path};

use radiobio::ode_solver::rk4::Rk4;
use radiobio::reactions::parse_reactions_file;
use radiobio::physics::beam::Beam;
use radiobio::{ODESolver, Time, State};


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

    let beam = Beam::new_constant(String::from("e"), 2.0).expect("");
    let beam = Beam::new_pulsed(String::from("e"), 1e6, 250e-6, 1e-6).expect("");

    let sim = ODESolver::new( sim_env, beam );
    let labels = sim.sim_env.species_label();
    let y0 = sim.sim_env.get_initial_values();

    // Debug of Sim:
    for elt in sim.sim_env.species.iter() {
        println!("{:?}", elt);
    }

    //std::process::exit(0);
    /* ---------------------------------------------------------------------- */
    //let mut stepper = Dopri5::new(sim, 0.0, 1e-3, 1e-10, y0, 1e-6, 1e-8);
    //let mut stepper = Dop853::new(sim, 1e-9, 10e-6, 0.0, y0, 1e-16, 1e-16);
    let mut stepper = Rk4::new(sim, 1e-9, y0, 0.1, 0.5e-6);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {
            println!("{}", stats);
            let file = format!(
                "{}/output/rk4.dat",
                env!("CARGO_MANIFEST_DIR")
            );
            let path = Path::new(&file);
            save(labels,
            stepper.x_out(),
            stepper.y_out(),
            path);
            println!("Results saved in: {:?}", path);
        }
        Err(e) => println!("An error occured: {}", e),
    }
}


pub fn save(labels: Vec<String>, times: &Vec<Time>, states: &Vec<State>, filename: &Path) {
    // Create or open file
    let file = match File::create(filename) {
        Err(e) => {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buf) => buf,
    };
    let mut buf = BufWriter::new(file);

    // Write labels
    write!(&mut buf, "{}", &labels[0]).unwrap();
    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }

    for label in labels[1..].iter() {
        write!(&mut buf, ", {}", label).unwrap();
    }
    write!(&mut buf, "\n").unwrap();

    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }

    // Write time and state vector in a csv format
    for (i, state) in states.iter().enumerate() {
        buf.write_fmt(format_args!("{}", times[i])).unwrap();
        for val in state.iter() {
            buf.write_fmt(format_args!(", {}", val)).unwrap();
        }
        buf.write_fmt(format_args!("\n")).unwrap();
    }
    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }
}