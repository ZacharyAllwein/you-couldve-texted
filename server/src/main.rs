use std::fs::{self, File};
use std::io::Error;
use serde_json::json;

fn main() {
    setup();
}

fn setup() -> Result<(), Error>{

    let config_json = json!({
        "read":"false",
        "port":"7777",
        "database_path": "ytc_data.json"
    });
    
    fs::write("ytc_config.json", config_json.to_string())?;



    Ok(())
}