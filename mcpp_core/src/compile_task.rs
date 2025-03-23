use core::fmt;
use std::collections::HashMap;

use evaluater::EvaluateError;
use regex::Regex;
use scoreboard::Calcable;
pub use scoreboard::Scoreboard;
pub use mcfunction::MCFunction;

use crate::{Language, CURRENT_LANGUAGE};

pub mod evaluater;
pub mod scoreboard;
pub mod mcfunction;

#[test]
fn test() {
    let mut compiler = CompileTask::new();
    println!("{}", compiler.compile("{a = 10 * 5 + 7}", "test").unwrap());
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
enum SentenceType {
    IfSentence,
    WhileSentence,
    ForSentence,
    FuncDefinition,
    Sentence
}
#[derive(Clone, Debug)]
pub enum SentenceError {
    UnnamedFunction,
    SentenceHasNoStartIdentifier,
    SentenceDoesntEndWithEndSpecifier,
}
impl fmt::Display for SentenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match CURRENT_LANGUAGE {
            Language::English => match self {
                Self::UnnamedFunction => "A function must has a name.".to_string(),
                Self::SentenceHasNoStartIdentifier => "A sentence must has a {.".to_string(),
                Self::SentenceDoesntEndWithEndSpecifier => "A sentence must ends with }.".to_string()
            },
            Language::Japanese => match self {
                Self::UnnamedFunction => "関数は名前を持たなければなりません。".to_string(),
                Self::SentenceHasNoStartIdentifier => "文は{を持たなければなりません。".to_string(),
                Self::SentenceDoesntEndWithEndSpecifier => "文は}で終了しなければなりません。".to_string()
            }
        })
    }
}
#[derive(Clone)]
enum Error {
    EvaluateError(evaluater::EvaluateError),
    SentenceError(SentenceError)
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::EvaluateError(e) => format!("{}", e),
            Self::SentenceError(e) => format!("{}", e)
        })
    }
}
struct Sentence<'a> {
    pub name : String,
    pub sentence_type : SentenceType,
    pub specifiers : Vec<&'a str>,
    pub parsed_lines : Vec<String>
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

impl CompileTask {
    /// It is the function for define a variable with bondaging a given value.
    /// # Example
    /// ```
    /// // This compile will end with successful although foo isn't defined in MC++ script.
    /// let mut compiler = CompileTask::new();
    /// compiler.define_variable("foo".to_string(), vec!["test".to_string], Calcable::Int(10));
    /// compiler.compile("{bar = foo * 7}") // The result is equal to the result of compiling {bar = 10 * 7}
    /// ```
    fn define_variable(&mut self, name:String, scope:Vec<String>, value:Calcable) -> Result<String, EvaluateError> {
        self.local_variables.insert(name.clone(),Scoreboard {
            name  : {
                if !Regex::new("[a-zA-Z_]+")
                    .unwrap()
                    .is_match(name.as_str()) {
                        println!("⚠️  {} is not constracted from romantic alphabet nether underbar.", name)
                }
                name.clone()
            },
            data_type : match value {
                Calcable::Int(_) => scoreboard::Types::Int,
                Calcable::Flt(_) => scoreboard::Types::Flt,
                Calcable::Bln(_) => scoreboard::Types::Bln,
                Calcable::Scr(s) => s.data_type.clone(),
                Calcable::Mcf(mcf) => mcf.ret_container.data_type.clone(),
            },
            scope : scope
        });
        self.get_variable(&name).unwrap().assign(&value)
    }
    /// It will return a specifier and a inside of given sentence.
    /// # Example
    /// ```
    /// // It will end with successful
    /// assert_eq!(
    ///     CompileTask::split_sentence("fn foo { c = 10 }"),
    ///     ("fn foo", "c = 10".to_string())
    /// );
    /// ```
    fn split_sentence(raw:&str) -> Result<(&str, String), SentenceError> {
        let specifier_and_inside:(&str, &str) = match raw.split_once("{") {
            Some(s) => s,
            None => return Err(SentenceError::SentenceHasNoStartIdentifier)
        };
        let specifier = specifier_and_inside.0.trim();
        if !specifier_and_inside.1.ends_with('}') {
            return Err(SentenceError::SentenceDoesntEndWithEndSpecifier)
        }
        let inside:String = specifier_and_inside.1.to_string();
        if inside == String::new() {
            println!("⚠️  A sentence, {}{{...}} is empty.", specifier);
        }
        Ok((specifier, inside))
    }
    fn guess_line_syntax(input:&str) -> SyntaxType {
        let trimed = input.trim();
        if trimed.starts_with('#') { SyntaxType::Comment }
        else if trimed.ends_with('}') { SyntaxType::Sentence }
        else { SyntaxType::Formula }
    }
    fn implicate_sentence(raw:&str) -> Result<Sentence, SentenceError> {
        // Split a given sentence onto a specialiser and a inside.
        let implicated = match CompileTask::split_sentence(raw) {
            Ok(o) => o,
            Err(e) => return Err(e)
        };
        // Split a given specialiser onto tokens.
        let tokenized_specializer:Vec<&str> = implicated
            .0
            .split_whitespace()
            .filter(|f| !f.is_empty())
            .collect();
        let sentence_type = if tokenized_specializer.is_empty() {
            SentenceType::Sentence
        } else {
            match *tokenized_specializer.get(0).unwrap() {
                "if"    => SentenceType::IfSentence,
                "for"   => SentenceType::ForSentence,
                "while" => SentenceType::WhileSentence,
                "fn"    => SentenceType::FuncDefinition,
                _       => SentenceType::Sentence
            }
        };
        let parsed = implicated
            .1
            .replace("}", ";}")
            .split(";")
            .map(|f| f.trim())
            .filter(|f| !f.is_empty() && f != &"}")
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        let name:String = match sentence_type {
            SentenceType::FuncDefinition => match tokenized_specializer.get(1) {
                Some(s) => s.to_string(),
                None => return Err(SentenceError::UnnamedFunction)
            },
            _ => generate_random_string(32)
        };
        Ok(
            Sentence {
                name : name,
                sentence_type : sentence_type,
                specifiers : tokenized_specializer,
                parsed_lines : parsed
            }
        )
    }
    /// It receives raw MC++ scripts in &str and returns compiled MCFunction.
    /// 
    /// The argument, "namespace" meant the pack name of compiled datapack.
    pub fn compile(&mut self, raw:&str, namespace:&str) -> Result<MCFunction, SentenceError> {
        let mut res:Vec<String> = Vec::new();
        let mut occured_errors:Vec<Error> = Vec::new();

        let impl_sentence = match Self::implicate_sentence(raw) {
            Ok(s) => s,
            Err(e) => return Err(e)
        };
        println!("Now compiling {}...", impl_sentence.name);

        for line in impl_sentence.parsed_lines {
            match CompileTask::guess_line_syntax(line.as_str()) {
                SyntaxType::Formula => res.push(
                    match evaluater::evaluate(self, &line.to_string()) {
                        Ok(o) => o.join("\n"),
                        Err(e) => {
                            occured_errors.push(Error::EvaluateError(e.clone()));
                            format!(
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
                ),
                SyntaxType::Sentence => res.push(
                    {
                        let mut slave_compiler = self.clone();
                        match slave_compiler.compile(line.as_str(), namespace) {
                            Ok(o) => {
                                let compiled_name = o.name.clone();
                                self.local_functions.insert(compiled_name.clone(), o);
                                let compiled = self.get_function(&compiled_name).unwrap();
                                compiled.save();
                                compiled.call()
                            },
                            Err(e) => {
                                occured_errors.push(Error::SentenceError(e.clone()));
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
                            }
                        }
                    }
                ),
                SyntaxType::Comment => res.push(line.to_string()),
            }
        }
        // Free variables
        res.push("# Free all of local variables".to_string());
        for var in &self.local_variables {
            res.push(var.1.free());
        }
        println!("Compiling of {} ended successfully!", impl_sentence.name);
        Ok(
            MCFunction {
                name : impl_sentence.name.to_string(),
                inside : res.join("\n"),
                path : self.scope.join("/"),
                namespace : namespace.to_string(),
                ret_container : Scoreboard {
                    name  : format!("TEMP.RETURN_VALUE.{}", impl_sentence.name),
                    data_type : scoreboard::Types::Non,
                    scope : Vec::new()
                }
            }
        )
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