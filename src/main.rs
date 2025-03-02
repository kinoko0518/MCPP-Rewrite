mod compile_task;

// MC++ Crates
use compile_task::{CompileTask, MCFunction};

// Outer Crates
use std::env::args;
use std::fs::File;
use std::io::prelude::*;

fn load_a_file_inside(path:&String) -> String {
    let mut source_code = File::open(path)
        .expect("File not found.");
    let mut contexts = String::new();
    source_code.read_to_string(&mut contexts).expect("An error occured while reading text file.");
    contexts
}
fn compile_a_file(path:&String) -> MCFunction {
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
    println!("{}", compile_a_file(&"C:/Projects/MCPP-Rewrite/test_code.mcpp".to_string()));
}
fn main() {
    let args:Vec<String> = args().collect();
    println!("{}", compile_a_file(args.get(0).expect("File doesn't exist.")).inside);
}