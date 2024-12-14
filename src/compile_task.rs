pub mod mcfunction;
pub mod scoreboard;

struct CompileTask {
    pack_name : String,
    root_path : String,

    variables : Vec<scoreboard::Scoreboard>,
    functions : Vec<mcfunction::MCFunction>
}

impl CompileTask {
    fn parse_formula(raw:String) {
        
    }
}
