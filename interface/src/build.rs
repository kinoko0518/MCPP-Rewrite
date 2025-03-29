// Outer Crates
extern crate serde;
extern crate toml;

// Inner Crates
use crate::input;
use mcpp_core;
use crate::init::Enviroment;

pub fn build(env_toml:&str, main:&str, target:&str) {
    let env:Enviroment = toml::from_str(&input::load_a_file_inside(env_toml)).unwrap();
    mcpp_core::compile_a_file(main)
        .unwrap()
        .build_datapack(&env.project_name, target)
        .unwrap();
}