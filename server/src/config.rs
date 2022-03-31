use std::fs;
use std::path::Path;

pub fn get_or_create_config() -> serde_json::Value {
    if Path::new("ytc_config.json").exists() {
        let file = fs::read_to_string("ytc_config.json").unwrap();
        serde_json::from_str(&file).expect("Malformed Json")
    } else {
        let config_json = serde_json::json!({
            "read": false,
            "ip": "127.0.0.1",
            "port": "7777",
        });

        fs::write(
            "ytc_config.json",
            serde_json::to_string_pretty(&config_json).unwrap(),
        )
        .unwrap();
        config_json
    }
}

pub fn verify(config: &serde_json::Value) -> Result<(), &'static str> {
    if !&config["read"].as_bool().unwrap() {
        Err("Please read config file and set value of 'read' to true")
    } else {
        Ok(())
    }
}
