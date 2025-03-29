use crate::{build, init};

pub fn solve_args(args:Vec<&str>, current_path:&str) {
    match args[0] {
        "build" => {
            let root = match args.get(2) {
                Some(s) => s.clone(),
                None => current_path
            };
            let chest_root = get_chest_root(&root).unwrap();
            build::build(
                format!("{}/MCPP.toml", &chest_root).as_str(),
                format!("{}/src/main.mcpp", &chest_root).as_str(),
                format!("{}/target", &chest_root).as_str()
            );
        },
        "init" => {
            init::init(&current_path);
        },
        "new" => {
            init::new(
                args
                    .get(2)
                    .expect("The new command expects a name of the new project on the secound argument."),
                &current_path
            ); 
        },
        _ => { println!("Invalid subcommand. You can try 'mcpp help' to get information.") }
    }
}