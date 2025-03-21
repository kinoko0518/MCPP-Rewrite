pub mod build;
pub mod init;
pub mod input;
pub mod output;

use std::env;
use std::fs;

fn get_current_path() -> String {
    env::current_dir()
        .expect("Something went wrong. Couldn't get the current path.")
        .into_os_string()
        .into_string()
        .expect("Couldn't parse the current path onto String.")
}

fn is_chest_root(path:&str) -> bool {
    match fs::exists(format!("{}/{}", path, "MCPP.toml")) {
        Ok(o) => o,
        Err(_) => false
    }
}

#[test]
fn chest_test() {
    println!("{}", get_chest_root("C:\\Projects\\MCPP-Rewrite\\test_code\\src").unwrap());
}

fn get_chest_root(from:&str) -> Result<String, ()> {
    let path_elements = from
        .split("\\")
        .collect::<Vec<&str>>();
    for i in 0..path_elements.len() {
        let temp_path = path_elements[0..path_elements.len() - i].join("/");
        if is_chest_root(&temp_path) { return Ok(temp_path); }
    }
    Err(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let current_path = get_current_path();

    match args.get(1) {
        Some(s) => match s.as_str() {
            "build" => {
                let root = match args.get(2) {
                    Some(s) => s.clone(),
                    None => get_current_path()
                };
                let chest_root = get_chest_root(&root).unwrap();
                build::build_datapack(
                    format!("{}/MCPP.toml", &chest_root).as_str(),
                    format!("{}/src/main.mcpp", &chest_root).as_str(),
                    format!("{}/target", &chest_root).as_str()
                );
            },
            "init" => { init::init(&current_path); },
            "new" => {
                init::new(
                    args
                        .get(2)
                        .expect("The new command expects a name of the new project on the secound argument."),
                    &current_path
                ); 
            },
            _ => { println!("Invalid subcommand. You can try 'mcpp help' to get information.") }
        },
        None => {
            println!("Hello, you can try these subcommands.\nbuild (path)\n   The subcommand for compile a file.\nhelp\n  You can receive more detailed information.")
        }
    }
}