mod f32_reference;
mod mcx_parser;
mod sweep;

fn main() {
    let config_path = std::path::PathBuf::from("../scenes/configs/cube60g1.json");
    let scene = mcx_parser::parse_mcx_json(&config_path);

    println!("{:#?}", scene);
}