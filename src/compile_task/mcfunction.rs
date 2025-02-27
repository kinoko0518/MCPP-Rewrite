use super::{CompileTask, Scoreboard};

#[derive(Clone, Debug)]
pub struct MCFunction {
    pub name      : String,
    pub inside    : String,
    pub path      : String,
    pub namespace : String,

    pub ret_container : Scoreboard
}
impl std::fmt::Display for MCFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}\n{}]", self.get_fullpath(), self.inside)
    }
}
impl MCFunction {
    pub fn new(name:&str, inside:&str, path:&str, compiler:&CompileTask) -> MCFunction {
        MCFunction {
            name      : name.to_string(),
            inside    : inside.to_string(),
            path      : path.to_string(),
            namespace : compiler.namespace.clone(),

            ret_container : Scoreboard {
                name  : format!("TEMP.RETURN_VALUE.{}", name),
                scope : Vec::new()
            }
        }
    }
    pub fn get_fullpath(&self) -> String { format!("{}:{}{}{}", self.namespace, self.path, if !self.path.is_empty() {"/"} else {""}, self.name ) }
    pub fn save(&self) {
        
    }
    pub fn call(&self) -> String {
        format!("function {}", self.get_fullpath())
    }
}