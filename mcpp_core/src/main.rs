mod compile_task;

// MC++ Crates
use compile_task::{CompileTask, MCFunction, SentenceError};

// Outer Crates
use std::fs::File;
use std::io::prelude::*;

enum Language {
    English,
    Japanese
}
const CURRENT_LANGUAGE:Language = Language::Japanese;

/// Read a file from given path, and return the inside.
fn load_a_file_inside(path:&str) -> String {
    let mut source_code = File::open(path)
        .expect("File not found.");
    let mut contexts = String::new();
    source_code.read_to_string(&mut contexts).expect("An error occured while reading text file.");
    contexts
}
/// Compile given text file and return result as MCFunction.
/// 
/// This is a wrapper of [`compile_task::CompileTask::compile()`]
pub fn compile_a_file(path:&str) -> Result<MCFunction, SentenceError> {
    let mut compiler = CompileTask::new();
    compiler.compile(
        &load_a_file_inside(path),
        {
            let splitted:Vec<&str> = path
                .split("/")
                .collect();
            splitted
                .get(splitted.len()-1)
                .unwrap()
        }
    )
}
#[test]
fn compile_test() {
    println!("{}", compile_a_file(&"C:/Projects/MCPP-Rewrite/test_code.mcpp".to_string()).unwrap());
}
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
fn main() {
    println!("MC++ / Author : Kinokov Shotaskovich / Date : 2025/03/03");
    let file_path = get_input("File path here");
    println!("\nNow compiling...");
    let compiled = compile_a_file(file_path.as_str());
    match compiled {
        Ok(mcf) => println!("Compile ended successfully!\n\nResult:\n{}\n", mcf.inside),
        Err(e) => println!("Compile failed.\n\nError:\n{}\n", e)
    }
    wait_for_any_input();
}