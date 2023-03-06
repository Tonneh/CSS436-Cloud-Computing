use std::fs::File;
use std::io::Write;

pub fn write(path_to_file: &str, content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path_to_file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}