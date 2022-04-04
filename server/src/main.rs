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

    let server = match Server::new(config){
        Ok(server) => server,
        Err(_) => {
            eprintln!("Failed to bind to given address");
            process::exit(0);
        }
    };

    server.start();
}
