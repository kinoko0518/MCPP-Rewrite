extern crate serde;
extern crate toml;

use crate::input;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enviroment {
    pub project_name : String,
    pub mc_version : String,
    pub test_world : String,
}
impl Enviroment {
    fn new() -> Enviroment {
        Enviroment {
            project_name: "Untitled".to_string(),
            mc_version: "1.20.1".to_string(),
            test_world: "THE_PATH_OF_TEST_WORLD_HERE".to_string()
        }
    }
}

pub fn init(path:&str) {
    let temp = path
        .split('/')
        .collect::<Vec<&str>>();
    let project_name = temp
        .last()
        .unwrap();
    let create_subdir = |name:&str| {
        fs::create_dir(format!("{}/{}", path, name)).unwrap();
    };

    // Generating src and target
    create_subdir("src");
    create_subdir("target");

    // Generating gitignore
    input::make_a_file(path, ".gitignore", "/target").unwrap();
    
    // Generating MCPP.toml
    let mut new_env = Enviroment::new();
    new_env.project_name = project_name.to_string();
    let toml_inside = toml::to_string(&new_env).unwrap();
    input::make_a_file(path, "MCPP.toml", &toml_inside).unwrap();

    // Generating src/main.mcpp
    let main_mcpp_inside = "fn main() {\n   \n}";
    input::make_a_file(
        format!("{}/src", path).as_str(),
        "main.mcpp",
        main_mcpp_inside
    ).unwrap();
}
pub fn new(project_name:&str, path:&str) {
    let target_path = format!("{}/{}", path, project_name);
    fs::create_dir(&target_path).unwrap();
    init(&target_path);
}