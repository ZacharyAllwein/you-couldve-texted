use serde_json;
use std::fs;
use std::process;
use std::path::Path;

fn main() {
    
    let config: serde_json::Value = {
        if Path::new("ytc_config").exists() {
            let file = fs::read_to_string("ytc_config").unwrap();
            serde_json::from_str(&file).expect("Malformed Json")
        }
        else {
            let config_json = serde_json::json!({
                "read": false,
                "port": 7777,
            });

            fs::write("ytc_config.json", serde_json::to_string_pretty(&config_json).unwrap()).unwrap();
            config_json
        }
    };

    if !&config["read"].as_bool().unwrap(){
        eprintln!("Please set 'read' in config to true");
        process::exit(1);
    }
}