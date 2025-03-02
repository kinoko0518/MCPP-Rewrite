use std::collections::HashMap;

use regex::Regex;
use scoreboard::Calcable;
pub use scoreboard::Scoreboard;
pub use mcfunction::MCFunction;

pub mod evaluater;
pub mod scoreboard;
pub mod mcfunction;

#[derive(Clone)]
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
    pub fn compile(&mut self, raw:&str, namespace:&str) -> MCFunction {
        let implicated = CompileTask::split_sentence(raw);
        let specializer = implicated.0;
        let tokenized_specializer:Vec<&str> = specializer
            .split_whitespace()
            .filter(|f| !f.is_empty())
            .collect();
        let sentence_type = if tokenized_specializer.is_empty() { SentenceType::Sentence }
        else {
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