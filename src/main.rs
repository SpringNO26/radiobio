use radiobio::reactions::parse_reactions_file;
use radiobio::reactions::traits::ChemicalReaction;
use radiobio::reactions::{AcidBase, KReaction};
fn main() {
    let reaction_file = format!(
        "{}/data/reactions.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let reactions = parse_reactions_file(&reaction_file);

    //println!("Reaction Config: {:?}", &reactions);

    let x = reactions.k_reactions[5].clone();
    println!("Reaction is: {:?}", &x);
    println!("\tcontains e_aq? {}", x.involve("e_aq"));
    println!("\tcontains H_r? {}", x.involve("H_r"));
    println!("\tcontains H2O2? {}", x.involve("H2O2"));
    println!("\tcontains h_r? {}", x.involve("h_r"));
    println!("\tk-value = {}", x.k_value());

}