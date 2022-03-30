use serde_json::{json, to_string_pretty};
use std::error::{self, Error};
use std::fs::{self, File};
use std::io;
use std::process;

fn main() {
    
    let config = match get_config(){
        Ok(conf) => conf,
        Err(e) => {
            eprintln!("Config fetch error: {}", e);
            process::exit(1);
        }
    };
}

fn get_config() -> Result<serde_json::Value, Box<dyn Error>> {

    //attempt to read in config file if it doesn't exist, create a new one and exit
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

    //read in config file as serde Value
    let config: serde_json::Value = serde_json::from_str(&file)?;

    Ok(config)
}
