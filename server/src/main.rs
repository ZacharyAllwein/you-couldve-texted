use std::fs::File;

fn main() {
    setup();
}

fn setup() -> Result<()>{

    let config_file = File::create("ytc_config.json")?;


}

struct Config<'a>{
    port: &'a str,
    database_path: &'a str,
}

impl Config{
    fn default() -> Config {
        Config {
            port: "7777",
            database_path: "ytc_data.json",
        }
    }
}