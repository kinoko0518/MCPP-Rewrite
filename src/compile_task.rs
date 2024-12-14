use std::{collections::HashMap, fmt::format, hash::Hash, os::raw, path::Display, result, vec};
use mcfunction::MCFunction;
use regex::Regex;
use std::fmt::Debug;

use scoreboard::Scoreboard;

pub mod mcfunction;
pub mod scoreboard;

struct CompileTask {
    pack_name : String,
    root_path : String,

    current_scope : Vec<String>,

    variables : HashMap<String, Scoreboard>,
    functions : HashMap<String, MCFunction>
}

enum FormulaToken<'a> {
    Int(i32),
    Scoreboard(&'a Scoreboard),
    Operator(String),
}

impl std::fmt::Display for FormulaToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaToken::Int(i) => write!(f, "{}", i.to_string()),
            FormulaToken::Operator(o) => write!(f, "{}", o),
            FormulaToken::Scoreboard(s) => write!(f,"{}", s)
        }
    }
}

#[test]
fn valid_check() {
    let mut task = CompileTask {
        pack_name : "Test".to_string(),
        root_path : "TEST_DIR".to_string(),
        current_scope : vec![],
        variables : HashMap::new(),
        functions : HashMap::new()
    };
    if true {
        task.variables.insert("b".to_string(),Scoreboard {
            name  : "b".to_string(),
            scope : vec!["test".to_string()]
        });
        print!("[ ");
        let res = task.to_formula_tokens("1 + b * 4 % 5".to_string());
        for i in res { print!("{} ", i); }
        print!("]");
    }
}

impl CompileTask {
    fn to_a_formula_token(&self, input:&str) -> FormulaToken {
        let _input = input.trim();
        if Regex::new("[0-9]+").unwrap().is_match(&_input) {
            FormulaToken::Int(_input.parse::<i32>().expect("Invalid number."))
        } else {
            return FormulaToken::Scoreboard(self.variables.get(_input).expect("A referenced variable is undefined"))
        }
    }
    fn to_formula_tokens(&self, raw:String) -> Vec<FormulaToken> {
        let mut res:Vec<FormulaToken> = vec!();
        let mut last = 0;
        for i in 1..raw.len() {
            let operartor = raw.get(i-1..i).unwrap().to_string();
            if ["+", "-", "*", "/", "%"].contains(&operartor.as_str()) {
                let value = raw.get(last..i-1).unwrap();
                res.push(self.to_a_formula_token(value));
                res.push(FormulaToken::Operator(operartor));
                last = i+1;
            }
        }
        res.push(self.to_a_formula_token(raw.get(last..).unwrap()));
        return res;
    }
}


impl std::fmt::Display for CompileTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Compiling {}... █▬▬]", &self.pack_name)
    }
}