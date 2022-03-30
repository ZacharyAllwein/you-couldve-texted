use serde_json::{json, to_string_pretty};
use std::error::{self, Error};
use std::fs::{self, File};
use std::io;
use std::process;

fn main() {
    if let Err(e) = check_config() {
        eprintln!("Server start Error: {}", e)
    }
}

fn check_config() -> Result<(), Box<dyn Error>> {
    let file = fs::read_to_string("ytc_config.json").or_else(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            let config_json = json!({
                "read": false,
                "port": 7777,
            });
            fs::write("ytc_config.json", to_string_pretty(&config_json).unwrap())?;
            process::exit(1);
        } else {
            return Err(err);
        }
    })?;

    let config: serde_json::Value = serde_json::from_str(&file)?;

    if !config["read"].as_bool().unwrap() {
        eprintln!("Please set 'read' in config to true, before continuing");
        process::exit(1);
    }

    Ok(())
}
