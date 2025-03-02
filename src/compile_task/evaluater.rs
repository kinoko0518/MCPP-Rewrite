// MC++ Crates
use super::CompileTask;
use super::Scoreboard;
use super::scoreboard::Calcable;
use super::MCFunction;

// Outer Crates
use std::vec;
use regex::Regex;

#[test]
fn calc_test() {
    let mut task = CompileTask::new();
    task.define_variable("b".to_string(), vec!["test".to_string()], Calcable::Int(12));
    let correct_infix = vec![
        FormulaToken::Int(1),
        FormulaToken::Operator(Operator::Add),
        FormulaToken::Scoreboard(&task.get_variable(&"b".to_string()).unwrap()),
        FormulaToken::Operator(Operator::Mul),
        FormulaToken::Int(3),
        FormulaToken::Operator(Operator::Div),
        FormulaToken::Int(4)
    ];
    let infix_parsed = to_formula_tokens(&task, "1 + b * 3 / 4");
    assert_eq!(infix_parsed, correct_infix);
    let correct_reverse_polish = vec![
        FormulaToken::Int(1),
        FormulaToken::Scoreboard(&task.local_variables["b"]),
        FormulaToken::Int(3),
        FormulaToken::Int(4),
        FormulaToken::Operator(Operator::Div),
        FormulaToken::Operator(Operator::Mul),
        FormulaToken::Operator(Operator::Add)
    ];
    assert_eq!(to_rpn(infix_parsed), correct_reverse_polish);
    let formula = "let c = 1 + b * 3 / 4".to_string();
    assert_eq!(calc(&mut task, &formula), vec![
            "scoreboard players set #Calc.TEMP MCPP.var 4".to_string(),
            "scoreboard players set #CONSTANT.3 MCPP.var 3\nscoreboard players operation #Calc.TEMP MCPP.var /= #CONSTANT.3 MCPP.var".to_string(),
            "scoreboard players operation #Calc.TEMP MCPP.var *= #test.b MCPP.var".to_string(),
            "scoreboard add #Calc.TEMP MCPP.var 1".to_string(),
            "scoreboard players operation #c MCPP.var = #Calc.TEMP MCPP.var".to_string()
        ]
    );
}

#[derive(Clone, Copy, Debug)]
enum FormulaToken<'a> {
    Int(i32),
    Scoreboard(&'a Scoreboard),
    Operator(Operator),
    MCFunction(&'a MCFunction)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Operator { Add, Rem, Mul, Div, Sur, Pow, LPt, RPt }

impl Operator {
    fn get_priority(&self) -> i32 {
        match self {
            Operator::Add | Operator::Rem => 0,
            Operator::Mul | Operator::Div | Operator::Sur => 1,
            Operator::Pow => 2,
            Operator::LPt | Operator::RPt => 3,
        }
    }
}
impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Operator::Add => "+",
            Operator::Rem => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
            Operator::Sur => "%",
            Operator::Pow => "^",
            Operator::LPt => "(",
            Operator::RPt => ")",
        })
    }
}
impl std::fmt::Display for FormulaToken<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormulaToken::Int(i) => write!(f, "{}", i.to_string()),
            FormulaToken::Operator(o) => write!(f, "{}", o),
            FormulaToken::Scoreboard(s) => write!(f,"{}", s),
            FormulaToken::MCFunction(mcf) => write!(f, "{}(...)", mcf.name)
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

fn to_operator(input:&str) -> Operator {
    return if input == "+" { Operator::Add }
    else if input == "-" { Operator::Rem }
    else if input == "*" { Operator::Mul }
    else if input == "/" { Operator::Div }
    else if input == "%" { Operator::Sur }
    else if input == "^" { Operator::Pow } 
    else { panic!("Invalid operator!") }
}
fn to_a_formula_token<'a>(compiler:&'a CompileTask, input:&'a str) -> FormulaToken<'a> {
    let _input = input.trim();
    if ["+", "-", "*", "/", "%", "^"].contains(&_input) {
        FormulaToken::Operator(to_operator(_input))
    } else if Regex::new("^[0-9]+$").unwrap().is_match(&_input) {
        FormulaToken::Int(_input.parse::<i32>().expect(format!("{} is invalid as number.", &_input).as_str()))
    } else if Regex::new("^([a-z]|[A-Z]|[0-9_])+[(].*[)]$").unwrap().is_match(&input) {
        return FormulaToken::MCFunction({
            let func_name = input.split_once("(").unwrap().0.to_string();
            compiler.get_function(&func_name).expect(format!("The called function, {}(...) is undefined.", &func_name).as_str())
        })
    } else { return FormulaToken::Scoreboard(
        compiler.get_variable(&_input.to_string())
            .expect(format!("A referenced variable, {} is undefined.", _input)
            .as_str()
            )
        )
    }
}    
fn to_formula_tokens<'a>(compiler:&'a CompileTask, input:&'a str) -> Vec<FormulaToken<'a>> {
    let re = Regex::new(r"(\d+|[a-zA-Z]+|[\+\-\*/])").unwrap();
    let splitted:Vec<&'a str> = re
        .find_iter(input)
        .map(|m| m.as_str())
        .collect();
    let tokens:Vec<FormulaToken<'a>> = splitted
        .iter()
        .map(|s| to_a_formula_token(compiler,s))
        .collect();
    tokens
}
fn to_rpn<'a>(input:Vec<FormulaToken<'a>>) -> Vec<FormulaToken<'a>> {
    let mut out_queue:Vec<FormulaToken> = Vec::new();
    let mut oper_stack:Vec<Operator> = Vec::new();
    for token in input {
        match token {
            FormulaToken::Int(_) | FormulaToken::Scoreboard(_)|FormulaToken::MCFunction(_) => out_queue.push(token),
            FormulaToken::Operator(o) => {
                let higher_priority_than_top = !oper_stack.is_empty() && oper_stack.last().unwrap().get_priority() <= o.get_priority();
                if oper_stack.is_empty() || higher_priority_than_top {
                    oper_stack.push(o)
                } else {
                    match o {
                        Operator::LPt => { oper_stack.push(o); }
                        Operator::RPt => {
                            for i in 0..oper_stack.len() {
                                if oper_stack[i] == Operator::LPt { oper_stack.remove(i); break; }
                                else { out_queue.push(FormulaToken::Operator(oper_stack[i])); }
                            }
                        }
                        _ => {
                            while !oper_stack.is_empty() && higher_priority_than_top {
                                let temp = oper_stack[0];
                                oper_stack.remove(0);
                                out_queue.push(FormulaToken::Operator(temp));
                            }
                            out_queue.push(FormulaToken::Operator(o));
                        }
                    }
                }
            }
        }
    }
    for oper in oper_stack.iter().rev() {
        out_queue.push(FormulaToken::Operator(*oper));
    }
    out_queue
}
fn calc_rpn(formula:Vec<FormulaToken>) -> (Vec<String>, Scoreboard) {
    let temp = Scoreboard { name : "TEMP".to_string(), scope : vec!["Calc".to_string()] };
    let mut responce:Vec<String> = Vec::new();
    let mut stack:Vec<Calcable> = Vec::new();
    for token in &formula {
        match token {
            FormulaToken::Int(i) => stack.push(Calcable::Int(*i)),
            FormulaToken::Scoreboard(s) => stack.push(Calcable::Scr(s)),
            FormulaToken::MCFunction(f) => if &formula.len() <= &(1 as usize) { responce.push(f.call()); }
            else { stack.push(Calcable::Mcf(f)) },
            FormulaToken::Operator(o) => {
                let lhs = stack.pop().unwrap();
                let rhs = stack.pop().unwrap();
                let target = match lhs {
                    Calcable::Scr(s) => s,
                    Calcable::Int(_) => { responce.push(temp.assign(&lhs)); &temp }
                    Calcable::Mcf(f) => { responce.push(f.call()); &f.ret_container }
                };
                responce.push(target.calc(format!("{}", &o).as_str(), &rhs));
                stack.push(Calcable::Scr(target));
            }
        }
    }
    (responce, temp)
}
pub fn calc(compiler:&mut CompileTask, formula:&String) -> Vec<String> {
    if !formula.is_empty() {
        let mut lhs = "";
        let rhs = match formula.split_once("=") {
            Some(s) => { lhs = s.0; s.1 }
            None => formula.as_str()
        };
        let calced = calc_rpn(
            to_rpn(
                to_formula_tokens(
                    &compiler, rhs)
                )
        );
        if !lhs.is_empty() {
            let mut temp = calced.0;
            temp.push(
                compiler.define_variable(
                    lhs.trim().to_string(),
                    compiler.scope.clone(),
                    Calcable::Scr(&calced.1)
                )
            );
            temp
        } else {
            calced.0
        }
    }
    else {
        println!("[WARN] An empty formula calclation occured."); Vec::new()
    }
}