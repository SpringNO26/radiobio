use radiobio::parsers::reactions_parser::parse_reactions_file;

fn main() {
    let reaction_file = format!(
        "{}/data/reactions.ron",
        env!("CARGO_MANIFEST_DIR")
    );
    let reactions = parse_reactions_file(&reaction_file);

    println!("Reaction Config: {:?}", &reactions);
}