use clap::Parser;
use serde_json::Value;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "jp")]
#[command(about = "A tool to peek into JSON data", long_about = None)]
struct Cli {
    /// Disable array indices in paths
    #[arg(short, long)]
    no_indices: bool,
}

fn list_jq_paths(value: &Value, current_path: &str, separator: &str, use_indices: bool) {
    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let new_path = if current_path.is_empty() {
                    format!(".{}", k)
                } else {
                    format!("{}{}{}", current_path, separator, k)
                };
                list_jq_paths(v, &new_path, separator, use_indices);
            }
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let new_path = if use_indices {
                    format!("{}[{}]", current_path, i)
                } else {
                    format!("{}[]", current_path)
                };
                list_jq_paths(v, &new_path, separator, use_indices);
            }
        }
        _ => println!("{}: {}", current_path, value),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let json: Value = serde_json::from_str(&buffer)?;
    list_jq_paths(&json, "", ".", !cli.no_indices);

    Ok(())
}