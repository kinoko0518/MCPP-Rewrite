mod compile_task;

// MC++ Crates
use compile_task::evaluater;
use compile_task::{CompileTask, MCFunction};

// Outer Crates
use std::fs::File;
use std::io::prelude::*;

fn load_a_file_inside(path:String) -> String {
    let mut source_code = File::open(path)
        .expect("File not found.");
    let mut contexts = String::new();
    source_code.read_to_string(&mut contexts).expect("An error occured while reading text file.");
    contexts
}
fn compile_a_file(path:String) -> MCFunction {
    let mut compiler = CompileTask::new();
    compiler.compile(&load_a_file_inside(path), "test_code")
}
fn calc(formula:String) -> String {
    let compiler = CompileTask::new();
    evaluater::calc(&compiler, &formula).join("\n")
}
fn main() {
    println!("{}", compile_a_file("C:/Projects/MCPP-Rewrite/test_code.mcpp".to_string()).inside);
}