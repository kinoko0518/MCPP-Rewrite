use crate::Language;
use crate::CURRENT_LANGUAGE;

use super::scoreboard::Types;
// MC++ Crates
use super::CompileTask;
use super::Scoreboard;
use super::scoreboard::Calcable;
use super::MCFunction;

use core::f32;
// Outer Crates
use std::fmt;
use std::vec;
use regex::Regex;

#[test]
fn float_calc_test() {
    let mut task = CompileTask::new();
    task.define_variable(
        "b".to_string(),
        vec!["test".to_string()],
        Calcable::Int(12)
    ).unwrap();
    println!("{}", evaluate(&mut task, "d:float = (0.03 * 0.2) + 0.05").unwrap().join("\n"));
}

#[derive(Clone, Copy, Debug)]
enum FormulaToken<'a> {
    Int(i32),
    Float(f32),
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
            FormulaToken::Float(fl) => write!(f, "{}", fl.to_string()),
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
            _ => panic!("Invalid comparement! lhs => {} | rhs => {}", self, other)
        }
    }
}

#[derive(Debug, Clone)]
/// The enum of the errors might occurs while evaluating a formula.
/// 
/// **[EvaluateError::OperationOccuredBetweenUnsupportedTypes]**
/// 
/// This error occurs when two values are operated by undefined operation.
/// 
/// **[EvaluateError::UndefinedFunctionCalled]**
/// 
/// This error occurs when a undefined function called. This is a example code of this error happens.
/// 
/// ```should_panic
/// use mcpp_core::compile_task::evaluater::evaluate;
/// use mcpp_core::compile_task::CompileTask;
/// 
/// let formula = "undefined_function()".to_string();
/// let mut compiler = CompileTask::new();
/// evaluate(&mut compiler, &formula).unwrap();
/// ```
pub enum EvaluateError {
    OperationOccuredBetweenUnsupportedTypes(Types, Types),
    AssignOccuredBetweenUnsupportedTypes(Types, Types),
    ComparementOccuredBetweenUnsupportedTypes(Types, Types),
    UndefinedFunctionCalled(String),
    UndefinedVariableReferenced(String),
    CouldntParseANumber(String),
    UnknownOperatorGiven(String),
    UnknownTypeAnnotation(String),
    UnbalancedBrackets
}
impl fmt::Display for EvaluateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match CURRENT_LANGUAGE {
            Language::English => match self {
                Self::AssignOccuredBetweenUnsupportedTypes(value, onto) => format!(
                    "Assigning {} type value onto {} type variable is undefined operation.", value, onto
                ),
                Self::OperationOccuredBetweenUnsupportedTypes(left, right) => format!(
                    "Calclation between {} and {} is undefined operation.", left, right
                ),
                Self::ComparementOccuredBetweenUnsupportedTypes(left, right) => format!(
                    "Comparement between {} and {} is undefined operation.", left, right
                ),
                Self::UndefinedFunctionCalled(func_name) => format!(
                    "An undefined function, {}(...) called.", func_name
                ),
                Self::UndefinedVariableReferenced(var_name) => format!(
                    "An undefined variable, {} referenced.", var_name
                ),
                Self::CouldntParseANumber(invalid_num) => format!(
                    "{} couldn't be solved as number.", invalid_num
                ),
                Self::UnknownOperatorGiven(invalid_oper) => format!(
                    "{} couldn't be solved as operator.", invalid_oper
                ),
                Self::UnknownTypeAnnotation(_type) => format!(
                    "The variable was annotated as {}, But {} is unknown", _type, _type
                ),
                Self::UnbalancedBrackets => "Amount of right parenthese(s) and left parenthese(s) must be equal.".to_string()
            },
            Language::Japanese => match self {
                Self::AssignOccuredBetweenUnsupportedTypes(value, onto) => format!(
                    "{}型の変数に{}型の値を代入する操作は未定義です。", onto, value
                ),
                Self::OperationOccuredBetweenUnsupportedTypes(left, right) => format!(
                    "{}型と{}型の計算は未定義操作です。", left, right
                ),
                Self::ComparementOccuredBetweenUnsupportedTypes(left, right) => format!(
                    "{}型と{}型の比較は未定義操作です。", left, right
                ),
                Self::UndefinedFunctionCalled(func_name) => format!(
                    "{}(...)は呼び出されましたが、宣言されていません。", func_name
                ),
                Self::UndefinedVariableReferenced(var_name) => format!(
                    "{}は参照されましたが、宣言されていません。", var_name
                ),
                Self::CouldntParseANumber(invalid_num) => format!(
                    "{}を数字として処理できませんでした。", invalid_num
                ),
                Self::UnknownOperatorGiven(invalid_oper) => format!(
                    "{}を算術記号として処理できませんでした。", invalid_oper
                ),
                Self::UnknownTypeAnnotation(_type) => format!(
                    "変数は{}として型注釈されていますが、{}は有効な型ではありません。", _type, _type
                ),
                Self::UnbalancedBrackets => "右かっこの数と左かっこの数が一致しません。".to_string()
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
        "(" => Ok(Operator::LPt),
        ")" => Ok(Operator::RPt),
        _   => Err(EvaluateError::UnknownOperatorGiven(input.to_string()))
    }
}
/// The function to guess that what given token is and convert it a FormulaToken.
fn to_a_formula_token<'a>(compiler:&'a CompileTask, input:&'a str) -> Result<FormulaToken<'a>, EvaluateError> {
    let _input = input.trim();
    let float_regex = Regex::new("^[0-9]*[.][0-9]+$").unwrap();
    let int_regex = Regex::new("^[0-9]+$").unwrap();
    let func_regex = Regex::new("^([a-z]|[A-Z]|[0-9_])+[(].*[)]$").unwrap();

    if ["+", "-", "*", "/", "%", "^", "(", ")"].contains(&_input) {
        // Operator
        match to_operator(_input) {
            Ok(oper) => Ok(FormulaToken::Operator(oper)),
            Err(e) => Err(e)
        }
    } else if float_regex.is_match(_input) {
        // Float
        match _input.parse::<f32>() {
            Ok(f) => Ok(FormulaToken::Float(f)),
            Err(_) => Err(EvaluateError::CouldntParseANumber(_input.to_string()))
        }
    } else if int_regex.is_match(_input) {
        // Int
        match _input.parse::<i32>() {
            Ok(i) => Ok(FormulaToken::Int(i)),
            Err(_) => Err(EvaluateError::CouldntParseANumber(_input.to_string()))
        }
    } else if func_regex.is_match(input) {
        // Function
        {
            let func_name = input.split_once("(").unwrap().0.to_string();
            match compiler.get_function(&func_name) {
                Some(mf) => Ok(FormulaToken::MCFunction(mf)),
                None => Err(EvaluateError::UndefinedFunctionCalled(func_name))
            }
        }
    } else {
        // Variable
        match compiler.get_variable(&_input.to_string()) {
            Some(var) => Ok(FormulaToken::Scoreboard(var)),
            None => Err(EvaluateError::UndefinedVariableReferenced(input.to_string()))
        }
    }
}
/// The pure function to convert &str type formula onto vector of FormulaToken(s).
fn to_formula_tokens<'a>(compiler:&'a CompileTask, input:&'a str) -> Result<Vec<FormulaToken<'a>>, EvaluateError> {
    let re = Regex::new(r"(\d*[.]\d+|\d+|[a-zA-Z]+|[\+\-\*\(\)/])").unwrap();
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
        valid_tokens.push(token?)
    }
    Ok(valid_tokens)
}
/// The pure function to reorder a infix notation formula onto a reverse polish notation.
fn to_rpn<'a>(input:Vec<FormulaToken<'a>>) -> Result<Vec<FormulaToken<'a>>, EvaluateError> {
    let mut output_queue: Vec<FormulaToken> = Vec::new();
    let mut operator_stack: Vec<Operator> = Vec::new();
    
    for token in input {
        match token {
            FormulaToken::Int(_)
                | FormulaToken::Float(_)
                | FormulaToken::Scoreboard(_)
                | FormulaToken::MCFunction(_) => {
                output_queue.push(token);
            },
            FormulaToken::Operator(op) => match op {
                Operator::LPt => {
                    operator_stack.push(op);
                },
                Operator::RPt => {
                    while !operator_stack.is_empty() {
                        let top_op = operator_stack.pop().unwrap();
                        if top_op == Operator::LPt {
                            break;
                        }
                        output_queue.push(FormulaToken::Operator(top_op));
                    }
                },
                _ => {
                    while !operator_stack.is_empty() {
                        let top_op = *operator_stack.last().unwrap();
                        if top_op == Operator::LPt || top_op.get_priority() < op.get_priority() {
                            break;
                        }
                        output_queue.push(FormulaToken::Operator(operator_stack.pop().unwrap()));
                    }
                    operator_stack.push(op);
                }
            }
        }
    }
    while !operator_stack.is_empty() {
        let op = operator_stack.pop().unwrap();
        if op == Operator::LPt && op == Operator::RPt {
            return Err(EvaluateError::UnbalancedBrackets)
        }
        output_queue.push(FormulaToken::Operator(op));
    }
    Ok(output_queue)
}
/// The pure function to calc a reverse polish notation formula.
/// 
/// The calcation commands will be kept in the first element of tuple,
/// and a scoreboard that contains a result will be kept in the secound element of tuple.
fn calc_rpn(formula:Vec<FormulaToken>, temp_score_data_type:Types) -> Result<(Vec<String>, Scoreboard), EvaluateError> {
    let temp = Scoreboard {
        name : "TEMP".to_string(),
        data_type : temp_score_data_type,
        scope : vec!["Calc".to_string()]
    };
    let mut responce:Vec<String> = Vec::new();
    let mut stack:Vec<Calcable> = Vec::new();
    for token in &formula {
        match token {
            FormulaToken::Int(i) => stack.push(Calcable::Int(*i)),
            FormulaToken::Float(f) => stack.push(Calcable::Flt(*f)),
            FormulaToken::Scoreboard(s) => stack.push(Calcable::Scr(s)),
            FormulaToken::MCFunction(f) => if &formula.len() <= &(1 as usize) {
                responce.push(f.call()); 
            } else {
                stack.push(Calcable::Mcf(f))
            },
            FormulaToken::Operator(o) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();

                let target = match lhs {
                    Calcable::Scr(s) => s,
                    Calcable::Mcf(f) => { responce.push(f.call()); &f.ret_container },
                    _ => { responce.push(temp.assign(&lhs)?); &temp }
                };
                responce.push(target.calc(format!("{}", &o).as_str(), &rhs)?);
                stack.push(Calcable::Scr(target));
            }
        }
    }
    Ok((responce, temp))
}
fn implicate_lhs(lhs:&str) -> Result<(String, Types), EvaluateError> {
    if lhs.contains(":") {
        let splitted = lhs.split_once(":").unwrap();
        Ok((
            splitted.0.to_string(),
            match splitted.1 {
                "int" => Types::Int,
                "float" => Types::Flt,
                _ => {
                    return Err(EvaluateError::UnknownTypeAnnotation(splitted.1.to_string()))
                }
            }
        ))
    } else {
        Ok((
            lhs.to_string(),
            Types::Int
        ))
    }
}

/// The impure function for evaluate a line.
/// 
/// It returns commands to apply the operations scribed on a formula.
/// This function modify CompileTask because of definition of variables are processed in this function.
pub fn evaluate(compiler:&mut CompileTask, formula:&str) -> Result<Vec<String>, EvaluateError> {
    if !formula.is_empty() {
        let mut lhs = String::new();
        let mut data_type = Types::Int;
        let rhs = match formula.split_once("=") {
            Some(s) => {
                let implicated = implicate_lhs(s.0.trim())?;
                lhs = implicated.0;
                data_type = implicated.1;
                s.1
            }
            None => formula
        };
        let calced = calc_rpn(
            to_rpn(
                match to_formula_tokens(&compiler, rhs) {
                    Ok(o) => o,
                    Err(e) => { return Err(e) }
                }
            )?,
            data_type
        )?;
        let mut result =if !lhs.is_empty() {
            let mut temp = calced.0;
            temp.push(
                compiler.define_variable(
                    lhs.trim().to_string(),
                    compiler.scope.clone(),
                    Calcable::Scr(&calced.1)
                )?
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