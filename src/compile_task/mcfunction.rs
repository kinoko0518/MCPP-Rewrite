use std::fs::File;
use std::io::Write;

pub struct MCFunction {
    child_functions : Vec<MCFunction>,

    root   : super::CompileTask,

    name   : String,
    path   : Vec<String>,

    header : Vec<String>,
    main   : Vec<String>,
    footer : Vec<String>
}

impl MCFunction {
    pub fn get(&self) -> String {
        return format!("{}\n\n{}\n\n{}", &self.header.join("\n"), &self.main.join("\n"), &self.footer.join("\n"));
    }

    fn get_path() {
        return 
    }

    pub fn save(&self, save_path:String) {
        let path = format!("{}/{}.mcfunction", save_path, &self.name);
        let mut file = File::create(path).expect("Could not create a file.");
        writeln!(file, "{}", self.get()).expect("Could not write onto a file.");
        for child_func in &self.child_functions {
            child_func.save(format!("{}/{}", save_path, &self.name));
        }
    }
}