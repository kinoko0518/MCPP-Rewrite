use core::fmt;
use std::collections::HashMap;

use evaluater::EvaluateError;
pub use scoreboard::Scoreboard;
pub use mcfunction::MCFunction;

use crate::{Language, CURRENT_LANGUAGE};

pub mod evaluater;
pub mod scoreboard;
pub mod mcfunction;

#[test]
fn test() {
    let mut compiler = CompileTask::new();
    println!("\n{}", (compiler.compile("if (1 - 1) * 0 {a = (0.1 + 2) * 5}", "test").unwrap().inside));
}

#[derive(Clone)]
/// The struct compiles a sentense, the code areas between { and } in MC++.
/// 
/// # Namespace
/// This is corresponding to a name of objective in Minecraft.
/// 
/// It is MCPP.var in default.
/// 
/// # Variables
/// CompileTasks have two type variables,
/// 
/// **inherited variables** and **local variables**.
/// 
/// Inherited variables are given by master compiler.
/// 
/// for example, in the case of
/// ```ignore
/// { // name the inside of this parentheses C
///     a = 1;
///     { // name the inside of this parentheses D
///         b = 2;
///     }
/// }
/// ```
/// D is a slave compiler of C, and as D, a is a inherited variable and b is a local variable.
/// 
/// local variables will be released at the end of sentence.
/// 
/// # Functions
/// CompileTasks have two type of functions as well as variables.
/// 
/// **inherited functions** and **local functions**.
/// 
/// Inherited functions are given by master compiler.
/// 
/// for example, in the case of
/// ```ignore
/// { // name the inside of this parentheses C
///     fn a {...}
///     { // name the inside of this parentheses D
///         fn b {...}
///     }
/// }
/// ```
/// D is a slave compiler of C, and as D, a is a inherited function and b is a local function.
/// 
/// local functions will be released at the end of sentence.
/// 
/// # Scope
/// Variables have true name used in Minecraft.
/// 
/// To specialise scoreboards used in other projects and have same name,
/// 
/// they have scope separeted by dots(.).
/// 
/// For example,
/// 
/// Scope : \["foo", "bar"\]
/// 
/// Name  : "baz"
/// 
/// In that case, the true name will be #foo.bar.baz
/// 
/// And it meant \["foo", "bar"\] part.
pub struct CompileTask {
    pub inherited_variables : HashMap<String, Scoreboard>,
    pub local_variables : HashMap<String, Scoreboard>,
    pub inherited_functions : HashMap<String, MCFunction>,
    pub local_functions : HashMap<String, MCFunction>,
    pub scope : Vec<String>,
}

#[derive(Debug)]
enum SyntaxType {
    Comment,
    Sentence,
    Formula,
}
enum Line {
    Comment(String),
    Formula(String),
    Sentence(Sentence)
}
#[derive(Clone, Debug)]
pub enum SentenceError {
    UnnamedFunction,
    SentenceHasNoStartIdentifier,
    SentenceDoesntEndWithEndSpecifier,
    InvalidFormula(EvaluateError)
}
impl fmt::Display for SentenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match CURRENT_LANGUAGE {
            Language::English => match self {
                Self::UnnamedFunction => "A function must has a name.".to_string(),
                Self::SentenceHasNoStartIdentifier => "A sentence must has a {.".to_string(),
                Self::SentenceDoesntEndWithEndSpecifier => "A sentence must ends with }.".to_string(),
                Self::InvalidFormula(e) => format!("Error(s) occured while evaluating a formula. Detail => {}", e)
            },
            Language::Japanese => match self {
                Self::UnnamedFunction => "関数は名前を持たなければなりません。".to_string(),
                Self::SentenceHasNoStartIdentifier => "文は{を持たなければなりません。".to_string(),
                Self::SentenceDoesntEndWithEndSpecifier => "文は}で終了しなければなりません。".to_string(),
                Self::InvalidFormula(e) => format!("式の評価中にエラーが発生しました。詳細 => {}", e)
            }
        })
    }
}
struct Sentence {
    pub name : String,
    pub specifiers : Vec<String>,
    pub parsed_lines : Vec<Line>
}

/// This is a function to generate expected length random charactors.
/// 
/// The length will be defined by the argument, length.
fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                            ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            0123456789";
    let mut rng = rand::thread_rng();

    let random_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    random_string
}

impl Sentence {
    fn guess_line_syntax(input:&str) -> SyntaxType {
        let trimed = input.trim();
        if trimed.starts_with('#') { SyntaxType::Comment }
        else if trimed.ends_with('}') { SyntaxType::Sentence }
        else { SyntaxType::Formula }
    }
    fn split_sentence(raw:&str) -> Result<(&str, String), SentenceError> {
        let specifier_and_inside:(&str, &str) = match raw.split_once("{") {
            Some(s) => s,
            None => return Err(SentenceError::SentenceHasNoStartIdentifier)
        };
        let specifier = specifier_and_inside.0.trim();
        if !specifier_and_inside.1.ends_with('}') {
            return Err(SentenceError::SentenceDoesntEndWithEndSpecifier)
        }
        let inside:String = specifier_and_inside.1[..specifier_and_inside.1.len() - 1].to_string();
        if inside == String::new() {
            println!("⚠️  A sentence, {}{{...}} is empty.", specifier);
        }
        Ok((specifier, inside))
    }
    pub fn onto_sentence(raw:&str) -> Result<Sentence, SentenceError> {
        // Split a given sentence onto a specialiser and a inside.
        let splitted = match Sentence::split_sentence(raw) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        // Split a given specialiser onto tokens.
        let tokenized_specializer:Vec<&str> = splitted
            .0
            .split_whitespace()
            .filter(|f| !f.is_empty())
            .collect();
        let parsed = splitted
            .1
            .replace("}", "};")
            .split(";")
            .map(|f| f.trim())
            .filter(|f| !f.is_empty() )
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        let name:String = match tokenized_specializer.get(0) {
            None => generate_random_string(30),
            Some(s) => match *s {
                "fn" => match tokenized_specializer.get(1) {
                    Some(s) => s.to_string(),
                    None => return Err(SentenceError::UnnamedFunction)
                },
                _ => generate_random_string(32)
            }
        };
        let mut lines:Vec<Line> = Vec::new();
        for line in &parsed {
            let line = match Self::guess_line_syntax(&line) {
                SyntaxType::Comment => Line::Comment(line.clone()),
                SyntaxType::Formula => Line::Formula(line.clone()),
                SyntaxType::Sentence => Line::Sentence(Self::onto_sentence(&line)?)
            };
            lines.push(line);
        }
        Ok(
            Sentence {
                name : name,
                specifiers : tokenized_specializer
                    .iter()
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>(),
                parsed_lines : lines
            }
        )
    }
    fn compile_then_call(&self, compiler:&mut CompileTask, namespace:&str) -> Result<String, SentenceError> {
        let mut slave_compiler = compiler.clone();
        match slave_compiler.compile_sentence(self, namespace) {
            Ok(o) => {
                let compiled_name = o.name.clone();
                compiler.local_functions.insert(compiled_name.clone(), o);
                Ok(compiler.get_function(&compiled_name).unwrap().callment.clone())
            },
            Err(e) => Ok(
                format!(
                    "### {} ###\n### {} : {} ###",
                    match CURRENT_LANGUAGE {
                        Language::English => "Because of failture of compiling a sentence, a callment of the sentence was skipped.",
                        Language::Japanese => "文はコンパイルに失敗したため、スキップされました。"
                    },
                    match CURRENT_LANGUAGE {
                        Language::English => "The Error",
                        Language::Japanese => "エラー内容"
                    },
                    e
                )
            )
        }
    }
}

impl CompileTask {
    fn compile_sentence(&mut self, sentence:&Sentence, namespace:&str) -> Result<MCFunction, SentenceError> {
        let mut res:Vec<String> = Vec::new();
        println!("Now compiling {}...", sentence.name);

        for line in &sentence.parsed_lines {
            let compiled  =match line {
                Line::Formula(f) => self.eval_line(&f),
                Line::Sentence(s) => s.compile_then_call(self, namespace)?,
                Line::Comment(c) => c.clone()
            };
            res.push(compiled);
        }
        let pure_callment = format!(
            "function {}:{}{}{}",
            namespace,
            self.scope.clone().join("/"),
            if self.scope.is_empty() {"/"} else {""},
            sentence.name
        );
        let callment = match sentence.specifiers.get(0) {
            Some(s) => match s.as_str() {
                "if" => Ok(
                    match evaluater::add_execution_condition(
                        &self,
                        sentence.name.as_str(),
                        &pure_callment,
                        &sentence.specifiers[1..].join(" "),
                    ) {
                        Ok(o) => o,
                        Err(e) => return Err(SentenceError::InvalidFormula(e))
                    }
                ),
                _ => Ok(pure_callment)
            },
            None => Ok(pure_callment)
        }?;
        // Free variables
        if !&self.local_variables.is_empty() {
            res.push("\n# Free all of local variables".to_string());
            for var in &self.local_variables {
                res.push(var.1.free());
            }
        }
        println!("Compiling of {} ended successfully!", sentence.name);
        Ok(
            MCFunction {
                name : sentence.name.to_string(),
                inside : res.join("\n"),
                namespace : namespace.to_string(),
                callment : callment,
                child_func : self.local_functions
                    .to_owned()
                    .into_iter()
                    .map(|f| f.1)
                    .collect::<Vec<MCFunction>>(),
                scope : self.scope.clone(),

                ret_container : Scoreboard {
                    name  : format!("TEMP.RETURN_VALUE.{}", sentence.name),
                    data_type : scoreboard::Types::Non,
                    scope : Vec::new()
                }
            }
        )
    }
    pub fn compile(&mut self, raw:&str, namespace:&str) -> Result<MCFunction, SentenceError> {
        self.compile_sentence(&Sentence::onto_sentence(raw)?, namespace)
    }
    fn eval_line(&mut self, formula:&str) -> String {
        match evaluater::evaluate(self, formula) {
            Ok(compiled) => compiled.join("\n"),
            Err(e) => format!(
                "### {} ###\n### {} -> {} ###",
                match CURRENT_LANGUAGE {
                    Language::English => "Because of evaluation error(s), evaluation of this line was skipped.",
                    Language::Japanese => "評価エラーのため、式の評価はスキップされました。"
                },
                match CURRENT_LANGUAGE {
                    Language::English => "The Error",
                    Language::Japanese => "エラー内容"
                },
                e
            )
        }
    }
    fn get_function(&self, name:&String) -> Option<&MCFunction> {
        if self.inherited_functions.contains_key(name) { Some(self.inherited_functions.get(name).unwrap()) }
        else if self.local_functions.contains_key(name) { Some(self.local_functions.get(name).unwrap()) }
        else { None }
    }
    fn get_variable(&self, name:&String) -> Option<&Scoreboard> {
        if self.inherited_variables.contains_key(name) { Some(self.inherited_variables.get(name).unwrap()) }
        else if self.local_variables.contains_key(name) { Some(self.local_variables.get(name).unwrap()) }
        else { None }
    }
    pub fn new() -> CompileTask {
        CompileTask {
            scope : Vec::new(),
            inherited_variables : HashMap::new(),
            local_variables : HashMap::new(),
            inherited_functions : HashMap::new(),
            local_functions : HashMap::new()
        }
    }
}