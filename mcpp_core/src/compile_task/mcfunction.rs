use super::Scoreboard;
use std::{fs, io::Write};

#[derive(Clone, Debug)]
pub struct MCFunction {
    pub name       : String,
    pub inside     : String,
    pub callment   : String,
    pub namespace  : String,

    pub child_func : Vec<MCFunction>,
    pub scope      : Vec<String>,

    pub ret_container : Scoreboard
}
impl std::fmt::Display for MCFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{\n{}\n}}", self.name, self.inside)
    }
}

fn generate_pack_mcmeta(inputs:Vec<(&str, &str)>) -> String {
    format!(
        "{{\n   \"pack\":{{\n{}\n   }}\n}}",
        inputs
            .iter()
            .map(|f| format!("      \"{}\":\"{}\",", f.0, f.1))
            .collect::<Vec<String>>()
            .join("\n")
    )
}

pub fn make_a_file(path:&str, file_name:&str, content:&str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/{}", path, file_name))?;
    file.write(content.as_bytes())?;
    Ok(())
}

impl MCFunction {
    fn save(&self, funcs_path:&str) -> std::io::Result<()> {
        let path = format!("{}/{}", funcs_path, self.scope.join("/"));
        make_a_file(&path, &format!("{}.mcfunction", self.name), &self.inside)?;
        for f in &self.child_func {
            f.save(funcs_path)?
        }
        Ok(())
    }
    pub fn build_datapack(&self, pack_name:&str, root_path:&str) -> std::io::Result<()> {
        let pack_root = format!("{}/{}", root_path, pack_name);
        // Clean up a root folder if the folder exists.
        if fs::exists(&pack_root).unwrap() {
            fs::remove_dir_all(&pack_root).unwrap();
        }
        fs::create_dir(&pack_root).unwrap();
        
        // Create pack.mcmeta
        let mcmeta = vec![
            ("pack_format", "61"),
            ("description", "description")
        ];
        make_a_file(
            &pack_root,
            "pack.mcmeta",
            &generate_pack_mcmeta(mcmeta)
        ).unwrap();

        // Create data
        fs::create_dir(format!("{}/data", &pack_root)).unwrap();
        // Create data/<namespace>
        fs::create_dir(format!("{}/data/{}", &pack_root, pack_name)).unwrap();
        // Create data/<namespace>/function
        let function_root = format!("{}/data/{}/function", &pack_root, pack_name);
        fs::create_dir(&function_root).unwrap();
        
        self.save(&function_root)?;
        Ok(())
    }
}