use std::process;
mod config;
mod server;

use server::Server;
fn main() {
    let config = config::get_or_create_config();

    if let Err(e) = config::verify(&config) {
        eprintln!("{}", e);
        process::exit(0);
    }


    server::run(config);
}
