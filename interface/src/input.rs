// Outer Crates
use std::io::prelude::*;
use std::fs;

/// Get user input as String.
/// 
/// The first argument, message is shown alike
/// 
/// Message : (User input area)
pub fn get_input(message: &str) -> String {
    print!("{} : ", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
pub fn wait_for_any_input() {
    print!("Waiting for any input...");
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}
/// Read a file from given path, and return the inside.
pub fn load_a_file_inside(path:&str) -> String {
    let mut source_code = fs::File::open(path)
        .expect("File not found.");
    let mut contexts = String::new();
    source_code.read_to_string(&mut contexts).expect("An error occured while reading text file.");
    contexts
}
pub fn make_a_file(path:&str, file_name:&str, content:&str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/{}", path, file_name))?;
    file.write(content.as_bytes())?;
    Ok(())
}