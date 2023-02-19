use radiobio::reactions::parse_reactions_file;
//use radiobio::reactions::traits::ChemicalReaction;
//use radiobio::reactions::{AcidBase, KReaction};
fn main() {
    let reaction_file = format!(
        "{}/data/reactions.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let reactions = parse_reactions_file(&reaction_file);
    println!("Ron file parsed to {:?}", reactions);
    //Env is {reactions -> {acid_base   -> vec<AcidBase>,
    //                      k_reactions -> vec<KReaction>
    //                     },
    //          species -> HashMap
    //       }


    /* Old Tests
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