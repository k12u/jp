use serde_json::{Value};
use std::io::{self, Read};

fn list_jq_paths(value: &Value, current_path: &str, separator: &str) {
    match value {
        Value::Object(map) => {
            for (k, v) in map.iter() {
                let new_path = if current_path.is_empty() {
                    k.to_string()
                } else {
                    format!("{}{}{}", current_path, separator, k)
                };
                list_jq_paths(v, &new_path, separator);
            }
        }
        Value::Array(arr) => {
            let is_nested = arr.iter().any(|v| v.is_object() || v.is_array());
            let new_path = format!("{}[]", current_path);

            if is_nested {
                println!("{}", new_path);
                for v in arr.iter() {
                    list_jq_paths(v, &new_path, separator);
                }
            } else {
                let values_str: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                println!("{}: {}", new_path, values_str.join(", "));
            }
        }
        _ => println!("{}: {}", current_path, value),
    }
}

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let json: Value = serde_json::from_str(&buffer)?;
    list_jq_paths(&json, "", ".");

    Ok(())
}
