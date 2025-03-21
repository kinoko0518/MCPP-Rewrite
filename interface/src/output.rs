use std::{fs, io::Write};

pub fn make_a_file(path:&str, file_name:&str, content:&str) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/{}", path, file_name))?;
    file.write(content.as_bytes())?;
    Ok(())
}