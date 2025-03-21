use crate::Language;
use crate::CURRENT_LANGUAGE;

// MC++ Crates
use super::CompileTask;
use super::Scoreboard;
use super::scoreboard::Calcable;
use super::MCFunction;

// Outer Crates
use std::fmt;
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
    let infix_parsed = to_formula_tokens(&task, "1 + b * 3 / 4").unwrap();
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
    let formula = "c = 1 + b * 3 / 4".to_string();
    match evaluate(&mut task, &formula) {
        Ok(o) => {
            assert_eq!(o, vec![
                    "# c = 1 + b * 3 / 4".to_string(),
                    "scoreboard players set #Calc.TEMP MCPP.var 4".to_string(),
                    "scoreboard players set #CONSTANT.3 MCPP.var 3\nscoreboard players operation #Calc.TEMP MCPP.var /= #CONSTANT.3 MCPP.var".to_string(),
                    "scoreboard players operation #Calc.TEMP MCPP.var *= #test.b MCPP.var".to_string(),
                    "scoreboard players add #Calc.TEMP MCPP.var 1".to_string(),
                    "scoreboard players operation #c MCPP.var = #Calc.TEMP MCPP.var".to_string()
                ]
            );
        },
        Err(e) => {
            panic!("{}", e);
        }
    }
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

#[derive(Debug, Clone)]
/// The enum of the errors might occurs while evaluating a formula.
/// 
/// **[EvaluateError::OperationOccuredBetweenUnsupportedTypes]**
/// This error occurs when two values are operated by undefined operation.
/// **[EvaluateError::UndefinedFunctionCalled]**
/// This error occurs when a undefined function called. This is a example code of this error happens.
/// ```should_panic
/// let formula = "undefined_function() + 1".to_string();
/// let compiler = CompileTask::new();
/// evaluate(compiler, &formula);
/// ```
pub enum EvaluateError {
    OperationOccuredBetweenUnsupportedTypes,
    UndefinedFunctionCalled(String),
    UndefinedVariableReferenced(String),
    CouldntParseANumber(String),
    UnknownOperatorGiven(String)
}
impl fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match CURRENT_LANGUAGE {
            Language::English => match self {
                Self::OperationOccuredBetweenUnsupportedTypes => String::new(),
                Self::UndefinedFunctionCalled(func_name) => format!("An undefined function, {}(...) called.", func_name),
                Self::UndefinedVariableReferenced(var_name) => format!("An undefined variable, {} referenced.", var_name),
                Self::CouldntParseANumber(invalid_num) => format!("{} couldn't be solved as number.", invalid_num),
                Self::UnknownOperatorGiven(invalid_oper) => format!("{} couldn't be solved as operator.", invalid_oper)
            },
            Language::Japanese => match self {
                Self::OperationOccuredBetweenUnsupportedTypes => String::new(),
                Self::UndefinedFunctionCalled(func_name) => format!("{}(...)は呼び出されましたが、宣言されていません。", func_name),
                Self::UndefinedVariableReferenced(var_name) => format!("{}は参照されましたが、宣言されていません。", var_name),
                Self::CouldntParseANumber(invalid_num) => format!("{}を数字として処理できませんでした。", invalid_num),
                Self::UnknownOperatorGiven(invalid_oper) => format!("{}を算術記号として処理できませんでした。", invalid_oper)
            }
        })
    }
}
/// The function for convert &str type mathmetics operators onto the enum, Operator.
fn to_operator(input:&str) -> Result<Operator, EvaluateError> {
    return match input {
        "+" => Ok(Operator::Add),
        "-" => Ok(Operator::Rem),
        "*" => Ok(Operator::Mul),
        "/" => Ok(Operator::Div),
        "%" => Ok(Operator::Sur),
        "^" => Ok(Operator::Pow),
        _   => Err(EvaluateError::UnknownOperatorGiven(input.to_string()))
    }
}
/// The function to guess that what given token is and convert it a FormulaToken.
fn to_a_formula_token<'a>(compiler:&'a CompileTask, input:&'a str) -> Result<FormulaToken<'a>, EvaluateError> {
    let _input = input.trim();
    if ["+", "-", "*", "/", "%", "^"].contains(&_input) {
        match to_operator(_input) {
            Ok(oper) => Ok(FormulaToken::Operator(oper)),
            Err(e) => Err(e)
        }
    } else if Regex::new("^[0-9]+$").unwrap().is_match(&_input) {
        match _input.parse::<i32>() {
            Ok(i) => Ok(FormulaToken::Int(i)),
            Err(_) => Err(EvaluateError::CouldntParseANumber(_input.to_string()))
        }
    } else if Regex::new("^([a-z]|[A-Z]|[0-9_])+[(].*[)]$").unwrap().is_match(&input) {
        {
            let func_name = input.split_once("(").unwrap().0.to_string();
            match compiler.get_function(&func_name) {
                Some(mf) => Ok(FormulaToken::MCFunction(mf)),
                None => Err(EvaluateError::UndefinedFunctionCalled(func_name))
            }
        }
    } else {
        match compiler.get_variable(&_input.to_string()) {
            Some(var) => Ok(FormulaToken::Scoreboard(var)),
            None => Err(EvaluateError::UndefinedVariableReferenced(input.to_string()))
        }
    }
}
/// The pure function to convert &str type formula onto vector of FormulaToken(s).
fn to_formula_tokens<'a>(compiler:&'a CompileTask, input:&'a str) -> Result<Vec<FormulaToken<'a>>, EvaluateError> {
    let re = Regex::new(r"(\d+|[a-zA-Z]+|[\+\-\*/])").unwrap();
    let splitted:Vec<&'a str> = re
        .find_iter(input)
        .map(|m| m.as_str())
        .collect();
    let tokens = splitted
        .iter()
        .map(|s| to_a_formula_token(compiler,s))
        .collect::<Vec<Result<FormulaToken<'a>, EvaluateError>>>();
    let mut valid_tokens:Vec<FormulaToken<'a>> = Vec::new();
    for token in tokens {
        match token {
            Ok(o) => { valid_tokens.push(o); }
            Err(e) => { return Err(e); }
        }
    }
    Ok(valid_tokens)
}
/// The pure function to reorder a infix notation formula onto a reverse polish notation.
fn to_rpn<'a>(input:Vec<FormulaToken<'a>>) -> Vec<FormulaToken<'a>> {
    let mut out_queue:Vec<FormulaToken> = Vec::new();
    let mut oper_stack:Vec<Operator> = Vec::new();
    for token in input {
        match token {
            FormulaToken::Int(_)
                | FormulaToken::Scoreboard(_)
                | FormulaToken::MCFunction(_) => out_queue.push(token),
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
/// The pure function to calc a reverse polish notation formula.
/// 
/// The calcation commands will be kept in the first element of tuple,
/// and a scoreboard that contains a result will be kept in the secound element of tuple.
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
/// The impure function for evaluate a line.
/// 
/// It returns commands to apply the operations scribed on a formula.
/// This function modify CompileTask because of definition of variables are processed in this function.
pub fn evaluate(compiler:&mut CompileTask, formula:&String) -> Result<Vec<String>, EvaluateError> {
    if !formula.is_empty() {
        let mut lhs = "";
        let rhs = match formula.split_once("=") {
            Some(s) => { lhs = s.0; s.1 }
            None => formula.as_str()
        };
        let calced = calc_rpn(to_rpn(
                match to_formula_tokens(&compiler, rhs) {
                    Ok(o) => o,
                    Err(e) => { return Err(e) }
                }
        ));
        let mut result =if !lhs.is_empty() {
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
        };
        result.insert(0, format!("# {}", formula));
        Ok(result)
    }
    else {
        println!("[WARN] An empty formula calclation occured.");
        Ok(Vec::new())
    }
}