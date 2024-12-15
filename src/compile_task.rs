use std::{collections::HashMap, fmt::format, fs::File, hash::Hash, os::raw, path::Display, result, vec};
use rand::Rng;
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
    Operator(Operator),
}
#[derive(PartialEq)]
enum Operator { Add, Rem, Mul, Div, Sur, Pow, Inv }

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Add => "+",
            Operator::Rem => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Sur => "%",
            Operator::Pow => "^",
            Operator::Inv => panic!("Invalid Operator!")
        })
    }
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

impl PartialEq for FormulaToken<'_> {
    fn eq(&self, other: &Self) -> bool {
        return match (self, other) {
            (FormulaToken::Int(i), FormulaToken::Int(ii)) => i == ii,
            (FormulaToken::Operator(o), FormulaToken::Operator(oo)) => *o == *oo,
            (FormulaToken::Scoreboard(scr), FormulaToken::Scoreboard(scrscr)) => scr == scrscr,
            _ => panic!("Invalid comparement!")
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
        let res = task.to_formula_tokens("1 + b * 4 % 5");
        for i in res { print!("{} ", i); }
    }
}

fn random_name(longness:i32) -> String {
    let mut res = String::new();
    let availables = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
        "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
    ];
    for i in 0..longness {
        let target:usize = rand::thread_rng().gen_range(0..availables.len());
        res += availables[target];
    }
    return res;
}

impl CompileTask {
    fn to_operator(&self, input:&str) -> Operator {
        return if input == "+" { Operator::Add }
        else if input == "-" { Operator::Rem }
        else if input == "*" { Operator::Mul }
        else if input == "/" { Operator::Div }
        else if input == "%" { Operator::Sur }
        else if input == "^" { Operator::Pow } 
        else { panic!("Invalid operator!") }
    }
    fn to_a_formula_token(&self, input:&str) -> FormulaToken {
        let _input = input.trim();
        if Regex::new("[0-9]+").unwrap().is_match(&_input) {
            FormulaToken::Int(_input.parse::<i32>().expect("Invalid number."))
        } else {
            return FormulaToken::Scoreboard(self.variables.get(_input).expect("A referenced variable is undefined"))
        }
    }
    fn to_formula_tokens(&self, raw:&str) -> Vec<FormulaToken> {
        let mut res:Vec<FormulaToken> = vec!();
        let mut last = 0;
        for i in 1..raw.len() {
            let operartor = raw.get(i-1..i).unwrap().to_string();
            if ["+", "-", "*", "/", "%"].contains(&operartor.as_str()) {
                let value = raw.get(last..i-1).unwrap();
                res.push(self.to_a_formula_token(value));
                res.push(FormulaToken::Operator(self.to_operator(operartor.as_str())));
                last = i+1;
            }
        }
        res.push(self.to_a_formula_token(raw.get(last..).unwrap()));
        return res;
    }
    fn formula_parser(self, raw:&str) {
        let tokens = self.to_formula_tokens(raw);
        let definements:Vec<String> = vec![];
        let freements:Vec<String> = vec![];
        
        let highest_priority = [
            FormulaToken::Operator(Operator::Pow)
        ];

        let high_priority = [
            FormulaToken::Operator(Operator::Mul),
            FormulaToken::Operator(Operator::Div),
            FormulaToken::Operator(Operator::Sur)
        ];

        let low_priority = [
            FormulaToken::Operator(Operator::Add),
            FormulaToken::Operator(Operator::Rem)
        ];

        for i in 0..tokens.len() {
            if highest_priority.contains(&tokens[i]) {
                let solved = match (&tokens[i-1..i], &tokens[i..i+1]) {
                    (FormulaToken::Int(&i), FormulaToken::Int(&ii)) if tokens[..i] == Operator::Pow =>
                        FormulaToken::Int(i.pow(*ii as u32)),
                    (FormulaToken::Int(i), FormulaToken::Scoreboard(scr)) => {
                        let temp_scr = scoreboard::Scoreboard { name  : random_name(32), scope : vec!["TEMP".to_string()] };
                        definements.push(temp_scr.assign(&scoreboard::Calcable::Scr(**scr)).to_string());
                        definements.push(temp_scr.calc(&tokens[i], &scoreboard::Calcable::Int(i)));
                        FormulaToken::Scoreboard(&temp_scr)
                    }
                };
            }
        }
    }
}


impl std::fmt::Display for CompileTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Compiling {}... █▬▬]", &self.pack_name)
    }
}