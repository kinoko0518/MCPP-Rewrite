use super::Scoreboard;

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
        write!(f, "{} {{\n{}\n}}", self.get_fullpath(), self.inside)
    }
}
impl MCFunction {
    /// The pure function to get the path of the function in Minecraft.
    pub fn get_fullpath(&self) -> String {
        format!("{}:{}{}{}",
            self.namespace,
            self.path,
            if !self.path.is_empty() {"/"} else {""},
            self.name
        )
    }
    pub fn save(&self) {
        
    }
    /// This is a pure function to get callment of MCFunctions.
    pub fn call(&self) -> String {
        format!("function {}", self.get_fullpath())
    }
}