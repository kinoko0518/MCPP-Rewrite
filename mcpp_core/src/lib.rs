pub mod compile_task;

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