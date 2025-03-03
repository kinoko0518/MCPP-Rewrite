use std::collections::HashMap;

use regex::Regex;
use scoreboard::Calcable;
pub use scoreboard::Scoreboard;
pub use mcfunction::MCFunction;

pub mod evaluater;
pub mod scoreboard;
pub mod mcfunction;

#[test]
fn test() {
    let mut compiler = CompileTask::new();
    println!("{}", compiler.compile("{a = 10 * 5 + 7}", "test"));
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
    pub namespace : String,
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
    fn define_variable(&mut self, name:String, scope:Vec<String>, value:Calcable) -> String {
        self.local_variables.insert(name.clone(),Scoreboard {
            name  : {
                if !Regex::new("[a-zA-Z_]+")
                    .unwrap()
                    .is_match(name.as_str()) {
                        println!("⚠️  {} is not constracted from romantic alphabet nether underbar.", name)
                }
                name.clone()
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
    fn split_sentence(raw:&str) -> (&str, String) {
        let specifier_and_inside:(&str, &str) = raw
            .split_once("{")
            .expect(format!("The sentence, {} have no start identifier.", &raw).as_str());
        let specifier = specifier_and_inside.0.trim();
        if !specifier_and_inside.1.ends_with('}') {
            panic!("The sentence doesn't end with end specifier.")
        }
        let inside:String = specifier_and_inside.1.to_string();
        if inside == String::new() {
            println!("⚠️  A sentence, {}{{...}} is empty.", specifier);
        }
        (specifier, inside)
    }
    fn guess_line_syntax(input:&str) -> SyntaxType {
        let trimed = input.trim();
        if trimed.starts_with('#') { SyntaxType::Comment }
        else if trimed.ends_with('}') { SyntaxType::Sentence }
        else { SyntaxType::Formula }
    }
    /// It receives raw MC++ scripts in &str and returns compiled MCFunction.
    /// 
    /// The argument, "namespace" meant the pack name of compiled datapack.
    pub fn compile(&mut self, raw:&str, namespace:&str) -> MCFunction {
        // Split a given sentence onto a specialiser and a inside.
        let implicated = CompileTask::split_sentence(raw);
        // Split a given specialiser onto tokens.
        let tokenized_specializer:Vec<&str> = implicated.0
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
        
        let formatted = implicated.1.replace("}", ";}");
        let parsed:Vec<&str> = formatted
            .split(";")
            .map(|f| f.trim())
            .filter(|f| !f.is_empty() && f != &"}")
            .collect();
        let mut res:Vec<String> = Vec::new();
        let name:String = match sentence_type {
            SentenceType::FuncDefinition => tokenized_specializer
                .get(1)
                .expect("⛔  Function definement must have a name on secound specializer.")
                .to_string(),
            _ => generate_random_string(32)
        };
        println!("Now compiling {}...", name);

        for line in parsed {
            match CompileTask::guess_line_syntax(line) {
                SyntaxType::Formula => res.push(
                    evaluater::calc(self, &line.to_string())
                        .join("\n")
                ),
                SyntaxType::Sentence => res.push({
                    let mut slave_compiler = self.clone();
                    let compiled = slave_compiler.compile(line, namespace);
                    let compiled_name = compiled.name.clone();
                    self.local_functions.insert(compiled_name.clone(), compiled);
                    let compiled = self.get_function(&compiled_name).unwrap();
                    compiled.save();
                    compiled.call()
                }),
                SyntaxType::Comment => res.push(line.to_string()),
            }
        }
        for var in &self.local_variables { res.push(var.1.free()); }
        println!("Compiling of {} ended successfully!", name);
        MCFunction {
            name : name.to_string(),
            inside : res.join("\n"),
            path : self.scope.join("/"),
            namespace : namespace.to_string(),
            ret_container : Scoreboard {
                name  : format!("TEMP.RETURN_VALUE.{}", name),
                scope : Vec::new()
            }
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
            namespace : String::new(),
            inherited_variables : HashMap::new(),
            local_variables : HashMap::new(),
            inherited_functions : HashMap::new(),
            local_functions : HashMap::new()
        }
    }
}