use std::collections::HashMap;
use crate::file;
use crate::file::read::read;

pub fn parse_input_file(filename: &str) -> Result<HashMap<String, HashMap<String, String>>, Box<dyn std::error::Error>> {
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
