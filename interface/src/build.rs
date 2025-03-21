// Outer Crates
extern crate serde;
extern crate toml;

// Inner Crates
use mcpp_core;
use crate::{init::Enviroment, input, output::{self, make_a_file}};
use std::fs;

#[test]
fn pack_meta_test() {
    let mcmeta = vec![
        ("pack_format", "114514"),
        ("description", "description")
    ];
    println!("{}", generate_pack_mcmeta(mcmeta));
}

fn generate_pack_mcmeta(inputs:Vec<(&str, &str)>) -> String {
    format!(
        "{{\n   \"pack\":{{\n{}\n   }}\n}}",
        inputs
            .iter()
            .map(|f| format!("      \"{}\":\"{}\",", f.0, f.1))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn build_datapack(mcpp_toml:&str, file_path:&str, export_to:&str) {
    let env = toml::from_str::<Enviroment>(
        &input::load_a_file_inside(mcpp_toml)
    ).expect("Failed to parse MCPP.toml. MCPP.toml may be invalid.");
    
    // Create a root folder of a datapack
    let pack_root = format!("{}/{}", export_to, env.project_name);

    // Clean up a root folder if the folder exists.
    if fs::exists(&pack_root).unwrap() {
        fs::remove_dir_all(&pack_root).unwrap();
    }
    fs::create_dir(&pack_root).unwrap();
    
    // Create pack.mcmeta
    let mcmeta = vec![
        ("pack_format", "61"),
        ("description", "description")
    ];
    output::make_a_file(
        &pack_root,
        "pack.mcmeta",
        &generate_pack_mcmeta(mcmeta)
    ).unwrap();

    // Create data
    fs::create_dir(format!("{}/data", &pack_root)).unwrap();
    // Create data/<namespace>
    fs::create_dir(format!("{}/data/{}", &pack_root, &env.project_name)).unwrap();
    // Create data/<namespace>/function
    let function_root = format!("{}/data/{}/function", &pack_root, &env.project_name);
    fs::create_dir(&function_root).unwrap();

    make_a_file(
        &function_root,
    "main.mcfunction",
        &mcpp_core::compile_a_file(file_path)
            .unwrap()
            .inside
    ).unwrap();
}