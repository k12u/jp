use serde_json::{Value};
use std::io::{self, Read};

fn list_jq_paths(value: &Value, current_path: &str, separator: &str) {
    match value {
        Value::Object(map) => {
            for (k, v) in map.iter() {
                let new_path = if current_path.is_empty() {
                    format!(".{}", k)
                } else {
                    format!("{}{}{}", current_path, separator, k)
                };
                list_jq_paths(v, &new_path, separator);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_path = format!("{}[{}]", current_path, i);
                list_jq_paths(v, &new_path, separator);
            }
        }
        _ => println!("{}: {}", current_path, value),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let json: Value = serde_json::from_str(&buffer)?;
    list_jq_paths(&json, "", ".");

    Ok(())
}
