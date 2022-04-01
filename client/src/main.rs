use client::Client;
use std::env;
use std::process;

mod client;

fn main() {
    //program takes in a socket address to connect to, if nothing in program exits
    let socket_address = match env::args().nth(1) {
        None => {
            eprintln!("No socket address");
            process::exit(0);
        }
        Some(address) => address,
    };

    //turns the socket address into a client instance and exits if the socket address doesn't exist,
    let mut client = match Client::new(socket_address) {
        Err(_) => {
            eprintln!("Invalid socket address");
            process::exit(0);
        }
        Ok(conn) => conn,
    };

    client.login();
}
