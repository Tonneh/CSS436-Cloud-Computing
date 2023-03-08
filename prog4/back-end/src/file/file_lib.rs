use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn parse_input_file(
    filename: &str,
) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
    let file = read(filename)?;

    let mut map = HashMap::new();

    for line in file.lines() {
        let mut parts: Vec<&str> = line.split([' ', '\t']).collect();
        parts.retain(|&i| i != "");
        let name = parts[0..2].join(" ");
        if name.contains('=') {
            continue;
        }
        parts.retain(|&i| i.contains("="));
        let mut values = HashMap::new();
        for part in parts.iter() {
            let kv: Vec<&str> = part.split('=').collect();
            let key = kv[0];
            let value = kv[1];
            values.insert(key.to_string(), value.to_string());
        }
        map.insert(name, values);
    }
    Ok(map)
}

pub fn read(path_to_file: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path_to_file)?;
    Ok(content)
}

pub fn write(path_to_file: &str, content: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path_to_file)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
