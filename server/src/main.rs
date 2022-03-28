use std::fs::File; 
use std::io::Error;

fn main() {
    setup();
}

fn setup() -> Result<(), Error>{

    let config_file = File::create("ytc_config.json")?;

    Ok(())


}

struct Config<'a>{
    port: &'a str,
    database_path: &'a str,
}

impl<'a> Config<'a>{
    fn default() -> Config<'a>{
        Config {
            port: "7777",
            database_path: "ytc_data.json",
        }
    }
}