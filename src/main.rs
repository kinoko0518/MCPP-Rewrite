mod compile_task;

// MC++ Crates
use compile_task::{CompileTask, MCFunction};

// Outer Crates
use std::fs::File;
use std::io::prelude::*;

fn load_a_file_inside(path:&str) -> String {
    let mut source_code = File::open(path)
        .expect("File not found.");
    let mut contexts = String::new();
    source_code.read_to_string(&mut contexts).expect("An error occured while reading text file.");
    contexts
}
fn compile_a_file(path:&str) -> MCFunction {
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
fn get_input(message: &str) -> String {
    print!("{} : ", message);
    std::io::stdout().flush().unwrap();
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}
fn main() {
    println!("MC++ / Author : Kinokov Shotaskovich / Date : 2025/03/03");
    let file_path = get_input("File path here");
    println!("\nNow compiling...");
    println!("Compile ended successfully!\n\nResult:\n{}", compile_a_file(file_path.as_str()).inside);
}