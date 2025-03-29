pub mod build;
pub mod init;
pub mod input;
pub mod cui;

use std::env;
use std::fs;
use std::io::Write;

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

pub fn make_a_file(path:&str, file_name:&str, content:&str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/{}", path, file_name))?;
    file.write(content.as_bytes())?;
    Ok(())
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

    match args.get(1) {
        Some(_) => cui::solve_args(
            args[1..]
                .iter()
                .map(|f| f.as_str())
                .collect::<Vec<&str>>()
                .to_vec(),
            get_current_path().as_str()
        ),
        None => {
            println!("Hello, you can try these subcommands.\nbuild (path)\n   The subcommand for compile a file.\nhelp\n  You can receive more detailed information.")
        }
    }
}