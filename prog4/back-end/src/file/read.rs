use std::fs;

pub fn read(path_to_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path_to_file)?;
    Ok(content)
}