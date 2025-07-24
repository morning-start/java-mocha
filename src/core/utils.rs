use serde_json::Value;
use std::fs::File;
use std::io::Write;


pub fn save_json(json: &Value, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(serde_json::to_string_pretty(json)?.as_bytes())?;
    Ok(())
}

pub fn load_json(file_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let json = serde_json::from_str(&std::fs::read_to_string(file_path)?)?;
    Ok(json)
}
